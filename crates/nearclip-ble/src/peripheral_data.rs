//! BLE 外设端数据接收模块
//!
//! 提供 BLE Peripheral 模式下的数据接收功能，支持分片数据重组。
//!
//! # 概述
//!
//! 本模块实现外设端的数据接收器，用于接收来自中心设备的剪贴板数据。
//! 主要功能：
//! - 监听 GATT 特征的写入事件
//! - 自动重组分片数据
//! - 通过回调通知上层数据接收完成
//!
//! # 平台支持
//!
//! 当前实现提供统一的 API 接口。平台特定的 BLE 实现将通过 feature flags 启用：
//! - macOS: CoreBluetooth CBPeripheralManager
//! - Linux: BlueZ D-Bus API
//! - Android: 通过 uniffi JNI 绑定
//!
//! # Example
//!
//! ```no_run
//! use nearclip_ble::{PeripheralDataReceiver, PeripheralDataConfig, DataReceiverCallback, BleError};
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl DataReceiverCallback for MyCallback {
//!     fn on_data_received(&self, data: Vec<u8>, from_device: &str) {
//!         println!("Received {} bytes from {}", data.len(), from_device);
//!     }
//!
//!     fn on_receive_error(&self, error: BleError) {
//!         eprintln!("Receive error: {}", error);
//!     }
//! }
//!
//! # async fn example() -> Result<(), BleError> {
//! let config = PeripheralDataConfig::new();
//! let callback = Arc::new(MyCallback);
//!
//! let mut receiver = PeripheralDataReceiver::new(config, callback).await?;
//! receiver.start().await?;
//!
//! // ... 数据接收在后台进行 ...
//!
//! receiver.stop().await?;
//! # Ok(())
//! # }
//! ```

use crate::chunk::{ChunkHeader, Reassembler};
use crate::gatt::CHUNK_HEADER_SIZE;
use crate::BleError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, trace, warn};

/// 默认 MTU 大小
pub const DEFAULT_MTU: usize = 23;

/// 默认重组超时时间（秒）
pub const DEFAULT_REASSEMBLE_TIMEOUT_SECS: u64 = 30;

/// 最大并发消息数
pub const MAX_CONCURRENT_MESSAGES: usize = 16;

/// 数据接收回调接口
///
/// 实现此 trait 以接收 BLE 数据接收事件。
///
/// # Thread Safety
///
/// 实现必须是线程安全的 (`Send + Sync`)，因为回调可能从不同的线程调用。
///
/// # Example
///
/// ```
/// use nearclip_ble::{DataReceiverCallback, BleError};
///
/// struct MyCallback {
///     received_count: std::sync::atomic::AtomicUsize,
/// }
///
/// impl DataReceiverCallback for MyCallback {
///     fn on_data_received(&self, data: Vec<u8>, from_device: &str) {
///         self.received_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
///         println!("Received {} bytes from {}", data.len(), from_device);
///     }
///
///     fn on_receive_error(&self, error: BleError) {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
pub trait DataReceiverCallback: Send + Sync {
    /// 接收到完整数据时调用
    ///
    /// # Arguments
    ///
    /// * `data` - 完整的接收数据
    /// * `from_device` - 发送方设备标识符
    fn on_data_received(&self, data: Vec<u8>, from_device: &str);

    /// 接收出错时调用
    ///
    /// # Arguments
    ///
    /// * `error` - 错误信息
    fn on_receive_error(&self, error: BleError);
}

/// 外设数据接收器配置
///
/// # Example
///
/// ```
/// use nearclip_ble::PeripheralDataConfig;
/// use std::time::Duration;
///
/// let config = PeripheralDataConfig::new()
///     .with_mtu(512)
///     .with_reassemble_timeout(Duration::from_secs(60));
///
/// assert_eq!(config.mtu, 512);
/// ```
#[derive(Debug, Clone)]
pub struct PeripheralDataConfig {
    /// MTU 大小
    pub mtu: usize,
    /// 分片重组超时时间
    pub reassemble_timeout: Duration,
    /// 最大并发消息数
    pub max_concurrent_messages: usize,
}

impl Default for PeripheralDataConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PeripheralDataConfig {
    /// 创建默认配置
    ///
    /// 默认值：
    /// - MTU: 23 (BLE 最小值)
    /// - 重组超时: 30 秒
    /// - 最大并发消息: 16
    pub fn new() -> Self {
        Self {
            mtu: DEFAULT_MTU,
            reassemble_timeout: Duration::from_secs(DEFAULT_REASSEMBLE_TIMEOUT_SECS),
            max_concurrent_messages: MAX_CONCURRENT_MESSAGES,
        }
    }

    /// 设置 MTU 大小
    ///
    /// # Arguments
    ///
    /// * `mtu` - MTU 大小（字节）
    pub fn with_mtu(mut self, mtu: usize) -> Self {
        self.mtu = mtu;
        self
    }

    /// 设置重组超时时间
    ///
    /// # Arguments
    ///
    /// * `timeout` - 超时时间
    pub fn with_reassemble_timeout(mut self, timeout: Duration) -> Self {
        self.reassemble_timeout = timeout;
        self
    }

    /// 设置最大并发消息数
    ///
    /// # Arguments
    ///
    /// * `max` - 最大并发数
    pub fn with_max_concurrent_messages(mut self, max: usize) -> Self {
        self.max_concurrent_messages = max;
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), BleError> {
        if self.mtu < CHUNK_HEADER_SIZE + 1 {
            return Err(BleError::Configuration(format!(
                "MTU too small: {} (minimum: {})",
                self.mtu,
                CHUNK_HEADER_SIZE + 1
            )));
        }

        if self.max_concurrent_messages == 0 {
            return Err(BleError::Configuration(
                "max_concurrent_messages cannot be 0".into(),
            ));
        }

        Ok(())
    }
}

/// 外设数据接收器内部状态
struct ReceiverState {
    /// 是否正在运行
    is_running: bool,
    /// 当前活跃的消息重组器 (message_id -> Reassembler)
    reassemblers: HashMap<u16, Reassembler>,
}

/// BLE 外设端数据接收器
///
/// 接收来自中心设备的数据，自动处理分片重组。
///
/// # 设计说明
///
/// 此结构体故意不实现 `Clone`，因为：
/// - 数据接收是独占操作
/// - 内部状态（重组器）不应被共享
/// - 如需跨任务共享，应使用 `Arc<Mutex<PeripheralDataReceiver>>`
///
/// # Example
///
/// ```no_run
/// use nearclip_ble::{PeripheralDataReceiver, PeripheralDataConfig, DataReceiverCallback, BleError};
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl DataReceiverCallback for MyCallback {
///     fn on_data_received(&self, _data: Vec<u8>, _from: &str) {}
///     fn on_receive_error(&self, _error: BleError) {}
/// }
///
/// # async fn example() -> Result<(), BleError> {
/// let config = PeripheralDataConfig::new();
/// let callback = Arc::new(MyCallback);
///
/// let mut receiver = PeripheralDataReceiver::new(config, callback).await?;
///
/// // 开始接收
/// receiver.start().await?;
/// assert!(receiver.is_running().await);
///
/// // 停止接收
/// receiver.stop().await?;
/// # Ok(())
/// # }
/// ```
pub struct PeripheralDataReceiver {
    /// 配置
    config: PeripheralDataConfig,
    /// 回调
    callback: Arc<dyn DataReceiverCallback>,
    /// 内部状态
    state: Arc<RwLock<ReceiverState>>,
}

impl PeripheralDataReceiver {
    /// 创建新的数据接收器
    ///
    /// # Arguments
    ///
    /// * `config` - 接收器配置
    /// * `callback` - 数据接收回调
    ///
    /// # Returns
    ///
    /// 成功返回 `PeripheralDataReceiver`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::Configuration` - 配置无效
    #[instrument(skip(config, callback))]
    pub async fn new(
        config: PeripheralDataConfig,
        callback: Arc<dyn DataReceiverCallback>,
    ) -> Result<Self, BleError> {
        config.validate()?;

        debug!(
            mtu = config.mtu,
            timeout_secs = config.reassemble_timeout.as_secs(),
            max_concurrent = config.max_concurrent_messages,
            "Creating peripheral data receiver"
        );

        Ok(Self {
            config,
            callback,
            state: Arc::new(RwLock::new(ReceiverState {
                is_running: false,
                reassemblers: HashMap::new(),
            })),
        })
    }

    /// 开始接收数据
    ///
    /// 启动 GATT 特征监听，准备接收数据。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::PlatformNotSupported` - 当前平台不支持
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<(), BleError> {
        let state = self.state.write().await;

        if state.is_running {
            warn!("Peripheral data receiver already running");
            return Ok(());
        }

        info!("Starting peripheral data receiver");

        // 平台特定实现
        #[cfg(target_os = "macos")]
        {
            // TODO: 实现 macOS CoreBluetooth GATT 服务
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: 实现 Linux BlueZ GATT 服务
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "android")]
        {
            // Android 通过 uniffi JNI 绑定实现
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
            // 其他平台暂不支持
            return Err(BleError::PlatformNotSupported);
        }

        #[allow(unreachable_code)]
        {
            state.is_running = true;
            Ok(())
        }
    }

    /// 停止接收数据
    ///
    /// 停止 GATT 特征监听，清理资源。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<(), BleError> {
        let mut state = self.state.write().await;

        if !state.is_running {
            debug!("Peripheral data receiver not running");
            return Ok(());
        }

        info!("Stopping peripheral data receiver");

        // 清理所有未完成的重组器
        let incomplete_count = state.reassemblers.len();
        if incomplete_count > 0 {
            warn!(
                incomplete_count,
                "Dropping incomplete message reassemblers"
            );
        }
        state.reassemblers.clear();
        state.is_running = false;

        Ok(())
    }

    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        self.state.read().await.is_running
    }

    /// 获取配置
    pub fn config(&self) -> &PeripheralDataConfig {
        &self.config
    }

    /// 处理接收到的数据分片
    ///
    /// 此方法由平台特定代码调用，当 GATT 特征收到写入时触发。
    ///
    /// # Arguments
    ///
    /// * `data` - 接收到的原始数据（包含分片头部）
    /// * `from_device` - 发送方设备标识符
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    #[instrument(skip(self, data))]
    pub async fn handle_chunk(&self, data: &[u8], from_device: &str) -> Result<(), BleError> {
        if data.len() < CHUNK_HEADER_SIZE {
            let err = BleError::ChunkError(format!(
                "Data too short: {} bytes (minimum: {})",
                data.len(),
                CHUNK_HEADER_SIZE
            ));
            self.callback.on_receive_error(err.clone());
            return Err(err);
        }

        let header = ChunkHeader::from_bytes(data)?;
        let payload = data[CHUNK_HEADER_SIZE..].to_vec();

        trace!(
            message_id = header.message_id,
            sequence = header.sequence_number,
            total = header.total_chunks,
            payload_len = payload.len(),
            from_device,
            "Received chunk"
        );

        // 获取或创建重组器
        let mut state = self.state.write().await;

        // 清理过期的重组器
        self.cleanup_expired_reassemblers(&mut state).await;

        // 检查并发限制
        if !state.reassemblers.contains_key(&header.message_id)
            && state.reassemblers.len() >= self.config.max_concurrent_messages
        {
            let err = BleError::DataTransfer(format!(
                "Too many concurrent messages: {} (max: {})",
                state.reassemblers.len(),
                self.config.max_concurrent_messages
            ));
            self.callback.on_receive_error(err.clone());
            return Err(err);
        }

        // 获取或创建重组器
        let reassembler = state
            .reassemblers
            .entry(header.message_id)
            .or_insert_with(|| {
                debug!(
                    message_id = header.message_id,
                    total_chunks = header.total_chunks,
                    from_device,
                    "Creating new reassembler"
                );
                Reassembler::new(
                    header.message_id,
                    header.total_chunks,
                    self.config.reassemble_timeout,
                )
            });

        // 添加分片
        reassembler.add_chunk(header, payload)?;

        // 检查��否完成
        if reassembler.is_complete() {
            debug!(
                message_id = header.message_id,
                from_device,
                "Message complete, assembling"
            );

            // 从 HashMap 中取出重组器
            if let Some(reassembler) = state.reassemblers.remove(&header.message_id) {
                match reassembler.assemble() {
                    Ok(complete_data) => {
                        info!(
                            message_id = header.message_id,
                            size = complete_data.len(),
                            from_device,
                            "Data received successfully"
                        );
                        self.callback.on_data_received(complete_data, from_device);
                    }
                    Err(e) => {
                        error!(
                            message_id = header.message_id,
                            error = %e,
                            "Failed to assemble message"
                        );
                        self.callback.on_receive_error(e);
                    }
                }
            }
        }

        Ok(())
    }

    /// 清理过期的重组器
    async fn cleanup_expired_reassemblers(&self, state: &mut ReceiverState) {
        let expired_ids: Vec<u16> = state
            .reassemblers
            .iter()
            .filter(|(_, r)| r.is_expired())
            .map(|(id, _)| *id)
            .collect();

        for id in expired_ids {
            warn!(message_id = id, "Removing expired reassembler");
            state.reassemblers.remove(&id);
        }
    }

    /// 获取当前活跃的消息数
    pub async fn active_message_count(&self) -> usize {
        self.state.read().await.reassemblers.len()
    }
}

// 为 ReceiverState 实现 Debug
impl std::fmt::Debug for ReceiverState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReceiverState")
            .field("is_running", &self.is_running)
            .field("active_reassemblers", &self.reassemblers.len())
            .finish()
    }
}

// 为 PeripheralDataReceiver 实现自定义 Debug（跳过 callback trait object）
impl std::fmt::Debug for PeripheralDataReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeripheralDataReceiver")
            .field("config", &self.config)
            .field("callback", &"<DataReceiverCallback>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunker;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// 测试用的回调实现
    struct TestCallback {
        received_data: Arc<RwLock<Vec<Vec<u8>>>>,
        received_from: Arc<RwLock<Vec<String>>>,
        error_count: AtomicUsize,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                received_data: Arc::new(RwLock::new(Vec::new())),
                received_from: Arc::new(RwLock::new(Vec::new())),
                error_count: AtomicUsize::new(0),
            }
        }
    }

    impl DataReceiverCallback for TestCallback {
        fn on_data_received(&self, data: Vec<u8>, from_device: &str) {
            // 使用 try_write 避免在测试中死锁
            if let Ok(mut received) = self.received_data.try_write() {
                received.push(data);
            }
            if let Ok(mut from) = self.received_from.try_write() {
                from.push(from_device.to_string());
            }
        }

        fn on_receive_error(&self, _error: BleError) {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    // ========================================
    // PeripheralDataConfig Tests
    // ========================================

    #[test]
    fn test_config_default() {
        let config = PeripheralDataConfig::new();

        assert_eq!(config.mtu, DEFAULT_MTU);
        assert_eq!(
            config.reassemble_timeout,
            Duration::from_secs(DEFAULT_REASSEMBLE_TIMEOUT_SECS)
        );
        assert_eq!(config.max_concurrent_messages, MAX_CONCURRENT_MESSAGES);
    }

    #[test]
    fn test_config_with_mtu() {
        let config = PeripheralDataConfig::new().with_mtu(512);

        assert_eq!(config.mtu, 512);
    }

    #[test]
    fn test_config_with_timeout() {
        let config =
            PeripheralDataConfig::new().with_reassemble_timeout(Duration::from_secs(60));

        assert_eq!(config.reassemble_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_config_with_max_concurrent() {
        let config = PeripheralDataConfig::new().with_max_concurrent_messages(32);

        assert_eq!(config.max_concurrent_messages, 32);
    }

    #[test]
    fn test_config_validate_success() {
        let config = PeripheralDataConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_mtu_too_small() {
        let config = PeripheralDataConfig::new().with_mtu(5);
        let result = config.validate();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MTU too small"));
    }

    #[test]
    fn test_config_validate_zero_concurrent() {
        let config = PeripheralDataConfig::new().with_max_concurrent_messages(0);
        let result = config.validate();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot be 0"));
    }

    #[test]
    fn test_config_builder_chain() {
        let config = PeripheralDataConfig::new()
            .with_mtu(256)
            .with_reassemble_timeout(Duration::from_secs(45))
            .with_max_concurrent_messages(8);

        assert_eq!(config.mtu, 256);
        assert_eq!(config.reassemble_timeout, Duration::from_secs(45));
        assert_eq!(config.max_concurrent_messages, 8);
    }

    // ========================================
    // PeripheralDataReceiver Tests
    // ========================================

    #[tokio::test]
    async fn test_receiver_new_success() {
        let config = PeripheralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let receiver = PeripheralDataReceiver::new(config, callback).await;
        assert!(receiver.is_ok());
    }

    #[tokio::test]
    async fn test_receiver_new_invalid_config() {
        let config = PeripheralDataConfig::new().with_mtu(1);
        let callback = Arc::new(TestCallback::new());

        let receiver = PeripheralDataReceiver::new(config, callback).await;
        assert!(receiver.is_err());
    }

    #[tokio::test]
    async fn test_receiver_stop_without_start() {
        let config = PeripheralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let mut receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 停止未启动的接收器应该成功
        let result = receiver.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_receiver_handle_single_chunk() {
        let config = PeripheralDataConfig::new().with_mtu(100);
        let callback = Arc::new(TestCallback::new());
        let callback_clone = callback.clone();

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 创建单个分片的数据
        let data = b"Hello";
        let chunks = Chunker::chunk(data, 1, 100).unwrap();
        assert_eq!(chunks.len(), 1);

        // 处理分片
        receiver
            .handle_chunk(&chunks[0], "device-1")
            .await
            .unwrap();

        // 验证回调被调用
        let received = callback_clone.received_data.read().await;
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], data);

        let from = callback_clone.received_from.read().await;
        assert_eq!(from[0], "device-1");
    }

    #[tokio::test]
    async fn test_receiver_handle_multiple_chunks() {
        let config = PeripheralDataConfig::new().with_mtu(23);
        let callback = Arc::new(TestCallback::new());
        let callback_clone = callback.clone();

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 创建多个分片的数据
        let data = b"Hello, World! This is a longer message.";
        let chunks = Chunker::chunk(data, 1, 23).unwrap();
        assert!(chunks.len() > 1);

        // 处理所有分片
        for chunk in &chunks {
            receiver.handle_chunk(chunk, "device-2").await.unwrap();
        }

        // 验证回调被调用
        let received = callback_clone.received_data.read().await;
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], data);
    }

    #[tokio::test]
    async fn test_receiver_handle_out_of_order_chunks() {
        let config = PeripheralDataConfig::new().with_mtu(23);
        let callback = Arc::new(TestCallback::new());
        let callback_clone = callback.clone();

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 创建多个分片的数据
        let data = b"The quick brown fox jumps over the lazy dog";
        let chunks = Chunker::chunk(data, 42, 23).unwrap();
        assert!(chunks.len() > 1);

        // 乱序处理分片
        for chunk in chunks.iter().rev() {
            receiver.handle_chunk(chunk, "device-3").await.unwrap();
        }

        // 验证回调被调用
        let received = callback_clone.received_data.read().await;
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], data);
    }

    #[tokio::test]
    async fn test_receiver_handle_invalid_chunk() {
        let config = PeripheralDataConfig::new();
        let callback = Arc::new(TestCallback::new());
        let callback_clone = callback.clone();

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 发送无效的数据（太短）
        let result = receiver.handle_chunk(&[1, 2, 3], "device-4").await;

        assert!(result.is_err());
        assert_eq!(callback_clone.error_count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_receiver_active_message_count() {
        let config = PeripheralDataConfig::new().with_mtu(23);
        let callback = Arc::new(TestCallback::new());

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 初始时没有活跃消息
        assert_eq!(receiver.active_message_count().await, 0);

        // 发送第一个消息的第一个分片
        let data1 = b"Message 1 is longer than MTU";
        let chunks1 = Chunker::chunk(data1, 1, 23).unwrap();
        receiver.handle_chunk(&chunks1[0], "device").await.unwrap();

        // 现在有一个活跃消息
        assert_eq!(receiver.active_message_count().await, 1);

        // 发送第二个消息的第一个分片
        let data2 = b"Message 2 is also long enough";
        let chunks2 = Chunker::chunk(data2, 2, 23).unwrap();
        receiver.handle_chunk(&chunks2[0], "device").await.unwrap();

        // 现在有两个活跃消息
        assert_eq!(receiver.active_message_count().await, 2);
    }

    #[tokio::test]
    async fn test_receiver_config_accessor() {
        let config = PeripheralDataConfig::new().with_mtu(256);
        let callback = Arc::new(TestCallback::new());

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        assert_eq!(receiver.config().mtu, 256);
    }

    #[tokio::test]
    async fn test_receiver_multiple_complete_messages() {
        let config = PeripheralDataConfig::new().with_mtu(100);
        let callback = Arc::new(TestCallback::new());
        let callback_clone = callback.clone();

        let receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

        // 发送多个完整消息
        for i in 0..3u16 {
            let data = format!("Message {}", i);
            let chunks = Chunker::chunk(data.as_bytes(), i, 100).unwrap();
            for chunk in &chunks {
                receiver.handle_chunk(chunk, "device").await.unwrap();
            }
        }

        // 验证所有消息都被接收
        let received = callback_clone.received_data.read().await;
        assert_eq!(received.len(), 3);
    }
}
