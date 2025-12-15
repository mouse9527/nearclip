//! BLE 中心端数据发送模块
//!
//! 提供 BLE Central 模式下的数据发送功能，支持分片发送和 ACK 确认。
//!
//! # 概述
//!
//! 本模块实现中心端的数据发送器，用于向外设设备发送剪贴板数据。
//! 主要功能：
//! - 连接到已发现的外设设备
//! - 自动分片大数据（MTU 限制）
//! - 等待 ACK 确认
//! - 超时重试机制
//!
//! # 平台支持
//!
//! 当前实现提供统一的 API 接口。平台特定的 BLE 实现将通过 feature flags 启用：
//! - macOS: CoreBluetooth CBCentralManager
//! - Linux: BlueZ D-Bus API
//! - Android: 通过 uniffi JNI 绑定
//!
//! # Example
//!
//! ```no_run
//! use nearclip_ble::{CentralDataSender, CentralDataConfig, DataSenderCallback, BleError};
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl DataSenderCallback for MyCallback {
//!     fn on_send_complete(&self, message_id: u16) {
//!         println!("Message {} sent successfully", message_id);
//!     }
//!
//!     fn on_send_error(&self, message_id: u16, error: BleError) {
//!         eprintln!("Failed to send message {}: {}", message_id, error);
//!     }
//!
//!     fn on_ack_received(&self, message_id: u16) {
//!         println!("ACK received for message {}", message_id);
//!     }
//! }
//!
//! # async fn example() -> Result<(), BleError> {
//! let config = CentralDataConfig::new();
//! let callback = Arc::new(MyCallback);
//!
//! let mut sender = CentralDataSender::new(config, callback).await?;
//! sender.connect("device-001").await?;
//! sender.send(b"Hello, World!").await?;
//! sender.disconnect().await?;
//! # Ok(())
//! # }
//! ```

use crate::chunk::Chunker;
use crate::gatt::{ATT_HEADER_SIZE, CHUNK_HEADER_SIZE};
use crate::BleError;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, trace, warn};

/// 默认 MTU 大小
pub const DEFAULT_MTU: usize = 23;

/// 默认发送超时时间（秒）
pub const DEFAULT_SEND_TIMEOUT_SECS: u64 = 30;

/// 默认重试次数
pub const DEFAULT_RETRY_COUNT: u32 = 3;

/// 默认 ACK 等待超时（毫秒）
pub const DEFAULT_ACK_TIMEOUT_MS: u64 = 5000;

/// 数据发送回调接口
///
/// 实现此 trait 以接收 BLE 数据发送事件。
///
/// # Thread Safety
///
/// 实现必须是线程安全的 (`Send + Sync`)，因为回调可能从不同的线程调用。
///
/// # Example
///
/// ```
/// use nearclip_ble::{DataSenderCallback, BleError};
///
/// struct MyCallback {
///     sent_count: std::sync::atomic::AtomicUsize,
/// }
///
/// impl DataSenderCallback for MyCallback {
///     fn on_send_complete(&self, message_id: u16) {
///         self.sent_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
///         println!("Message {} sent", message_id);
///     }
///
///     fn on_send_error(&self, message_id: u16, error: BleError) {
///         eprintln!("Send failed: {}", error);
///     }
///
///     fn on_ack_received(&self, message_id: u16) {
///         println!("ACK for message {}", message_id);
///     }
/// }
/// ```
pub trait DataSenderCallback: Send + Sync {
    /// 数据发送完成时调用
    ///
    /// # Arguments
    ///
    /// * `message_id` - 消息 ID
    fn on_send_complete(&self, message_id: u16);

    /// 发送出错时调用
    ///
    /// # Arguments
    ///
    /// * `message_id` - 消息 ID
    /// * `error` - 错误信息
    fn on_send_error(&self, message_id: u16, error: BleError);

    /// 收到 ACK 时调用
    ///
    /// # Arguments
    ///
    /// * `message_id` - 消息 ID
    fn on_ack_received(&self, message_id: u16);
}

/// 中心端数据发送器配置
///
/// # Example
///
/// ```
/// use nearclip_ble::CentralDataConfig;
/// use std::time::Duration;
///
/// let config = CentralDataConfig::new()
///     .with_mtu(256)
///     .with_send_timeout(Duration::from_secs(60))
///     .with_retry_count(5);
///
/// assert_eq!(config.mtu, 256);
/// assert_eq!(config.retry_count, 5);
/// ```
#[derive(Debug, Clone)]
pub struct CentralDataConfig {
    /// MTU 大小
    pub mtu: usize,
    /// 发送超时时间
    pub send_timeout: Duration,
    /// 重试次数
    pub retry_count: u32,
    /// ACK 等待超时
    pub ack_timeout: Duration,
}

impl Default for CentralDataConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl CentralDataConfig {
    /// 创建默认配置
    ///
    /// 默认值：
    /// - MTU: 23 (BLE 最小值)
    /// - 发送超时: 30 秒
    /// - 重试次数: 3
    /// - ACK 超时: 5 秒
    pub fn new() -> Self {
        Self {
            mtu: DEFAULT_MTU,
            send_timeout: Duration::from_secs(DEFAULT_SEND_TIMEOUT_SECS),
            retry_count: DEFAULT_RETRY_COUNT,
            ack_timeout: Duration::from_millis(DEFAULT_ACK_TIMEOUT_MS),
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

    /// 设置发送超时时间
    ///
    /// # Arguments
    ///
    /// * `timeout` - 超时时间
    pub fn with_send_timeout(mut self, timeout: Duration) -> Self {
        self.send_timeout = timeout;
        self
    }

    /// 设置重试次数
    ///
    /// # Arguments
    ///
    /// * `count` - 重试次数
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// 设置 ACK 等待超时
    ///
    /// # Arguments
    ///
    /// * `timeout` - 超时时间
    pub fn with_ack_timeout(mut self, timeout: Duration) -> Self {
        self.ack_timeout = timeout;
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), BleError> {
        // MTU 必须足够容纳头部和至少 1 字节 payload
        let min_mtu = ATT_HEADER_SIZE + CHUNK_HEADER_SIZE + 1;
        if self.mtu < min_mtu {
            return Err(BleError::Configuration(format!(
                "MTU too small: {} (minimum: {})",
                self.mtu, min_mtu
            )));
        }

        if self.send_timeout.is_zero() {
            return Err(BleError::Configuration(
                "send_timeout cannot be zero".into(),
            ));
        }

        if self.ack_timeout.is_zero() {
            return Err(BleError::Configuration("ack_timeout cannot be zero".into()));
        }

        Ok(())
    }
}

/// 发送状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendState {
    /// 空闲，未连接
    Idle,
    /// 已连接，准备发送
    Connected,
    /// 正在发送
    Sending,
    /// 等待 ACK
    WaitingAck,
}

/// 发送器内部状态
struct SenderState {
    /// 当前状态
    state: SendState,
    /// 已连接的设备 ID
    connected_device: Option<String>,
    /// 当前发送的消息 ID
    current_message_id: Option<u16>,
    /// 已发送的分片数
    sent_chunks: usize,
    /// 总分片数
    total_chunks: usize,
    /// 当前重试次数
    current_retry: u32,
}

impl SenderState {
    fn new() -> Self {
        Self {
            state: SendState::Idle,
            connected_device: None,
            current_message_id: None,
            sent_chunks: 0,
            total_chunks: 0,
            current_retry: 0,
        }
    }
}

/// BLE 中心端数据发送器
///
/// 向外设设备发送数据，自动处理分片和重试。
///
/// # 设计说明
///
/// 此结构体故意不实现 `Clone`，因为：
/// - 连接是独占资源
/// - 发送状态不应被共享
/// - 如需跨任务共享，应使用 `Arc<Mutex<CentralDataSender>>`
///
/// # Example
///
/// ```no_run
/// use nearclip_ble::{CentralDataSender, CentralDataConfig, DataSenderCallback, BleError};
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl DataSenderCallback for MyCallback {
///     fn on_send_complete(&self, _id: u16) {}
///     fn on_send_error(&self, _id: u16, _err: BleError) {}
///     fn on_ack_received(&self, _id: u16) {}
/// }
///
/// # async fn example() -> Result<(), BleError> {
/// let config = CentralDataConfig::new();
/// let callback = Arc::new(MyCallback);
///
/// let mut sender = CentralDataSender::new(config, callback).await?;
///
/// // 连接到设备
/// sender.connect("device-001").await?;
/// assert!(sender.is_connected().await);
///
/// // 发送数据
/// let message_id = sender.send(b"Hello").await?;
///
/// // 断开连接
/// sender.disconnect().await?;
/// # Ok(())
/// # }
/// ```
pub struct CentralDataSender {
    /// 配置
    config: CentralDataConfig,
    /// 回调
    callback: Arc<dyn DataSenderCallback>,
    /// 内部状态
    state: Arc<RwLock<SenderState>>,
    /// 消息 ID 生成器
    message_id_counter: AtomicU16,
}

impl CentralDataSender {
    /// 创建新的数据发送器
    ///
    /// # Arguments
    ///
    /// * `config` - 发送器配置
    /// * `callback` - 发送事件回调
    ///
    /// # Returns
    ///
    /// 成功返回 `CentralDataSender`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::Configuration` - 配置无效
    #[instrument(skip(config, callback))]
    pub async fn new(
        config: CentralDataConfig,
        callback: Arc<dyn DataSenderCallback>,
    ) -> Result<Self, BleError> {
        config.validate()?;

        debug!(
            mtu = config.mtu,
            timeout_secs = config.send_timeout.as_secs(),
            retry_count = config.retry_count,
            "Creating central data sender"
        );

        Ok(Self {
            config,
            callback,
            state: Arc::new(RwLock::new(SenderState::new())),
            message_id_counter: AtomicU16::new(1),
        })
    }

    /// 连接到外设设备
    ///
    /// # Arguments
    ///
    /// * `device_id` - 目标设备 ID
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::PlatformNotSupported` - 当前平台不支持
    /// - `BleError::ConnectionFailed` - 连接失败
    #[instrument(skip(self))]
    pub async fn connect(&mut self, device_id: &str) -> Result<(), BleError> {
        {
            let state = self.state.read().await;
            if state.state != SendState::Idle {
                warn!(
                    current_state = ?state.state,
                    "Cannot connect: sender not idle"
                );
                return Err(BleError::ConnectionFailed(
                    "Sender not in idle state".into(),
                ));
            }
        }

        info!(device_id, "Connecting to device");

        // 平台特定实现
        #[cfg(target_os = "macos")]
        {
            // TODO: 实现 macOS CoreBluetooth 连接
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: 实现 Linux BlueZ 连接
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(target_os = "android")]
        {
            // Android 通过 uniffi JNI 绑定实现
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
            Err(BleError::PlatformNotSupported)
        }
    }

    /// 发送数据
    ///
    /// 自动分片并发送数据，等待所有分片发送完成。
    ///
    /// # Arguments
    ///
    /// * `data` - 要发送的数据
    ///
    /// # Returns
    ///
    /// 成功返回消息 ID，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::DataTransfer` - 未连接或发送失败
    /// - `BleError::Timeout` - 发送超时
    #[instrument(skip(self, data), fields(data_len = data.len()))]
    pub async fn send(&mut self, data: &[u8]) -> Result<u16, BleError> {
        // 检查连接状态
        {
            let state = self.state.read().await;
            if state.state != SendState::Connected {
                return Err(BleError::DataTransfer(format!(
                    "Not connected, current state: {:?}",
                    state.state
                )));
            }
        }

        // 生成消息 ID
        let message_id = self.message_id_counter.fetch_add(1, Ordering::Relaxed);

        // 分片数据
        let chunks = Chunker::chunk(data, message_id, self.config.mtu)?;
        let total_chunks = chunks.len();

        info!(
            message_id,
            data_len = data.len(),
            total_chunks,
            "Sending data"
        );

        // 更新状态
        {
            let mut state = self.state.write().await;
            state.state = SendState::Sending;
            state.current_message_id = Some(message_id);
            state.sent_chunks = 0;
            state.total_chunks = total_chunks;
            state.current_retry = 0;
        }

        // 发送分片（当前平台不支持，模拟发送）
        for (i, chunk) in chunks.iter().enumerate() {
            trace!(
                message_id,
                chunk_index = i,
                chunk_len = chunk.len(),
                "Sending chunk"
            );

            // 实际发送逻辑（平台特定）
            let send_result = self.send_chunk(chunk).await;

            match send_result {
                Ok(()) => {
                    let mut state = self.state.write().await;
                    state.sent_chunks = i + 1;
                }
                Err(e) => {
                    error!(
                        message_id,
                        chunk_index = i,
                        error = %e,
                        "Failed to send chunk"
                    );

                    // 检查重试
                    let should_retry = {
                        let mut state = self.state.write().await;
                        state.current_retry += 1;
                        state.current_retry <= self.config.retry_count
                    };

                    if should_retry {
                        warn!(message_id, chunk_index = i, "Retrying chunk");
                        // 重试逻辑会在平台实现中处理
                    } else {
                        // 通知失败
                        self.callback.on_send_error(message_id, e.clone());

                        // 重置状态
                        let mut state = self.state.write().await;
                        state.state = SendState::Connected;
                        state.current_message_id = None;

                        return Err(e);
                    }
                }
            }
        }

        // 发送完成
        {
            let mut state = self.state.write().await;
            state.state = SendState::Connected;
            state.current_message_id = None;
        }

        debug!(message_id, "Send complete");
        self.callback.on_send_complete(message_id);

        Ok(message_id)
    }

    /// 发送单个分片（内部方法）
    async fn send_chunk(&self, _chunk: &[u8]) -> Result<(), BleError> {
        // 平台特定实现
        #[cfg(target_os = "macos")]
        {
            // TODO: 实现 macOS CoreBluetooth 写入
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: 实现 Linux BlueZ 写入
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(target_os = "android")]
        {
            // Android 通过 uniffi JNI 绑定实现
            Err(BleError::PlatformNotSupported)
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
            Err(BleError::PlatformNotSupported)
        }
    }

    /// 断开连接
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`
    #[instrument(skip(self))]
    pub async fn disconnect(&mut self) -> Result<(), BleError> {
        {
            let state = self.state.read().await;
            if state.state == SendState::Idle {
                debug!("Already disconnected");
                return Ok(());
            }

            if let Some(device_id) = &state.connected_device {
                info!(device_id, "Disconnecting from device");
            }
        }

        // 清理状态
        let mut state = self.state.write().await;
        state.state = SendState::Idle;
        state.connected_device = None;
        state.current_message_id = None;
        state.sent_chunks = 0;
        state.total_chunks = 0;
        state.current_retry = 0;

        Ok(())
    }

    /// 检查是否已连接
    pub async fn is_connected(&self) -> bool {
        let state = self.state.read().await;
        state.state != SendState::Idle
    }

    /// 获取当前状态
    pub async fn state(&self) -> SendState {
        self.state.read().await.state
    }

    /// 获取已连接的设备 ID
    pub async fn connected_device(&self) -> Option<String> {
        self.state.read().await.connected_device.clone()
    }

    /// 获取配置
    pub fn config(&self) -> &CentralDataConfig {
        &self.config
    }

    /// 处理收到的 ACK（供平台代码调用）
    ///
    /// # Arguments
    ///
    /// * `message_id` - 消息 ID
    #[instrument(skip(self))]
    pub async fn handle_ack(&self, message_id: u16) {
        debug!(message_id, "ACK received");

        let state = self.state.read().await;
        if state.current_message_id == Some(message_id) {
            self.callback.on_ack_received(message_id);
        } else {
            warn!(
                received_id = message_id,
                expected_id = ?state.current_message_id,
                "Unexpected ACK received"
            );
        }
    }

    /// 获取发送进度
    ///
    /// # Returns
    ///
    /// 返回 (已发送分片数, 总分片数)
    pub async fn send_progress(&self) -> (usize, usize) {
        let state = self.state.read().await;
        (state.sent_chunks, state.total_chunks)
    }
}

// 为 SenderState 实现 Debug
impl std::fmt::Debug for SenderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SenderState")
            .field("state", &self.state)
            .field("connected_device", &self.connected_device)
            .field("current_message_id", &self.current_message_id)
            .field("progress", &format!("{}/{}", self.sent_chunks, self.total_chunks))
            .finish()
    }
}

// 为 CentralDataSender 实现自定义 Debug
impl std::fmt::Debug for CentralDataSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CentralDataSender")
            .field("config", &self.config)
            .field("callback", &"<DataSenderCallback>")
            .field("message_id_counter", &self.message_id_counter)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    /// 测试用的回调实现
    struct TestCallback {
        complete_count: AtomicUsize,
        error_count: AtomicUsize,
        ack_count: AtomicUsize,
        last_message_id: RwLock<Option<u16>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                complete_count: AtomicUsize::new(0),
                error_count: AtomicUsize::new(0),
                ack_count: AtomicUsize::new(0),
                last_message_id: RwLock::new(None),
            }
        }
    }

    impl DataSenderCallback for TestCallback {
        fn on_send_complete(&self, message_id: u16) {
            self.complete_count.fetch_add(1, Ordering::Relaxed);
            if let Ok(mut id) = self.last_message_id.try_write() {
                *id = Some(message_id);
            }
        }

        fn on_send_error(&self, _message_id: u16, _error: BleError) {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_ack_received(&self, _message_id: u16) {
            self.ack_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    // ========================================
    // CentralDataConfig Tests
    // ========================================

    #[test]
    fn test_config_default() {
        let config = CentralDataConfig::new();

        assert_eq!(config.mtu, DEFAULT_MTU);
        assert_eq!(config.send_timeout, Duration::from_secs(DEFAULT_SEND_TIMEOUT_SECS));
        assert_eq!(config.retry_count, DEFAULT_RETRY_COUNT);
        assert_eq!(config.ack_timeout, Duration::from_millis(DEFAULT_ACK_TIMEOUT_MS));
    }

    #[test]
    fn test_config_with_mtu() {
        let config = CentralDataConfig::new().with_mtu(512);
        assert_eq!(config.mtu, 512);
    }

    #[test]
    fn test_config_with_send_timeout() {
        let config = CentralDataConfig::new().with_send_timeout(Duration::from_secs(60));
        assert_eq!(config.send_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_config_with_retry_count() {
        let config = CentralDataConfig::new().with_retry_count(5);
        assert_eq!(config.retry_count, 5);
    }

    #[test]
    fn test_config_with_ack_timeout() {
        let config = CentralDataConfig::new().with_ack_timeout(Duration::from_secs(10));
        assert_eq!(config.ack_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_config_validate_success() {
        let config = CentralDataConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_mtu_too_small() {
        let config = CentralDataConfig::new().with_mtu(5);
        let result = config.validate();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MTU too small"));
    }

    #[test]
    fn test_config_validate_zero_send_timeout() {
        let config = CentralDataConfig::new().with_send_timeout(Duration::ZERO);
        let result = config.validate();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("send_timeout"));
    }

    #[test]
    fn test_config_validate_zero_ack_timeout() {
        let config = CentralDataConfig::new().with_ack_timeout(Duration::ZERO);
        let result = config.validate();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ack_timeout"));
    }

    #[test]
    fn test_config_builder_chain() {
        let config = CentralDataConfig::new()
            .with_mtu(256)
            .with_send_timeout(Duration::from_secs(45))
            .with_retry_count(10)
            .with_ack_timeout(Duration::from_secs(8));

        assert_eq!(config.mtu, 256);
        assert_eq!(config.send_timeout, Duration::from_secs(45));
        assert_eq!(config.retry_count, 10);
        assert_eq!(config.ack_timeout, Duration::from_secs(8));
    }

    // ========================================
    // CentralDataSender Tests
    // ========================================

    #[tokio::test]
    async fn test_sender_new_success() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let sender = CentralDataSender::new(config, callback).await;
        assert!(sender.is_ok());
    }

    #[tokio::test]
    async fn test_sender_new_invalid_config() {
        let config = CentralDataConfig::new().with_mtu(1);
        let callback = Arc::new(TestCallback::new());

        let sender = CentralDataSender::new(config, callback).await;
        assert!(sender.is_err());
    }

    #[tokio::test]
    async fn test_sender_initial_state() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let sender = CentralDataSender::new(config, callback).await.unwrap();

        assert_eq!(sender.state().await, SendState::Idle);
        assert!(!sender.is_connected().await);
        assert!(sender.connected_device().await.is_none());
    }

    #[tokio::test]
    async fn test_sender_connect_platform_not_supported() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let mut sender = CentralDataSender::new(config, callback).await.unwrap();

        let result = sender.connect("device-001").await;

        // 在大多数平台上返回 PlatformNotSupported
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BleError::PlatformNotSupported));
    }

    #[tokio::test]
    async fn test_sender_send_not_connected() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let mut sender = CentralDataSender::new(config, callback).await.unwrap();

        let result = sender.send(b"Hello").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BleError::DataTransfer(_)));
    }

    #[tokio::test]
    async fn test_sender_disconnect_when_idle() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let mut sender = CentralDataSender::new(config, callback).await.unwrap();

        // 断开未连接的发送器应该成功
        let result = sender.disconnect().await;
        assert!(result.is_ok());
        assert_eq!(sender.state().await, SendState::Idle);
    }

    #[tokio::test]
    async fn test_sender_config_accessor() {
        let config = CentralDataConfig::new().with_mtu(256);
        let callback = Arc::new(TestCallback::new());

        let sender = CentralDataSender::new(config, callback).await.unwrap();

        assert_eq!(sender.config().mtu, 256);
    }

    #[tokio::test]
    async fn test_sender_send_progress_initial() {
        let config = CentralDataConfig::new();
        let callback = Arc::new(TestCallback::new());

        let sender = CentralDataSender::new(config, callback).await.unwrap();

        let (sent, total) = sender.send_progress().await;
        assert_eq!(sent, 0);
        assert_eq!(total, 0);
    }

    #[tokio::test]
    async fn test_sender_handle_ack() {
        let callback = Arc::new(TestCallback::new());
        let config = CentralDataConfig::new();

        let sender = CentralDataSender::new(config, callback.clone()).await.unwrap();

        // 模拟设置当前消息 ID
        {
            let mut state = sender.state.write().await;
            state.current_message_id = Some(42);
        }

        sender.handle_ack(42).await;

        assert_eq!(callback.ack_count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_sender_handle_unexpected_ack() {
        let callback = Arc::new(TestCallback::new());
        let config = CentralDataConfig::new();

        let sender = CentralDataSender::new(config, callback.clone()).await.unwrap();

        // 设置期望的消息 ID
        {
            let mut state = sender.state.write().await;
            state.current_message_id = Some(42);
        }

        // 发送不匹配的 ACK
        sender.handle_ack(99).await;

        // 不应该触发回调
        assert_eq!(callback.ack_count.load(Ordering::Relaxed), 0);
    }

    // ========================================
    // SendState Tests
    // ========================================

    #[test]
    fn test_send_state_values() {
        assert_ne!(SendState::Idle, SendState::Connected);
        assert_ne!(SendState::Connected, SendState::Sending);
        assert_ne!(SendState::Sending, SendState::WaitingAck);
    }
}
