//! 剪贴板内容发送
//!
//! 实现剪贴板内容的发送逻辑，包括通道选择和 ACK 等待。
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{ClipboardSender, ClipboardSenderConfig, ClipboardSendCallback, SyncError};
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl ClipboardSendCallback for MyCallback {
//!     fn on_send_success(&self, device_id: &str) {
//!         println!("Sent to {}", device_id);
//!     }
//!     fn on_send_failure(&self, device_id: &str, error: SyncError) {
//!         println!("Failed to send to {}: {}", device_id, error);
//!     }
//!     fn on_ack_received(&self, device_id: &str) {
//!         println!("ACK from {}", device_id);
//!     }
//! }
//!
//! # async fn example() -> Result<(), SyncError> {
//! let callback = Arc::new(MyCallback);
//! let config = ClipboardSenderConfig::new();
//! let sender = ClipboardSender::new(config, callback)?;
//! # Ok(())
//! # }
//! ```

use crate::channel::{Channel, ChannelInfo, ChannelSelector, ChannelStatus, PriorityChannelSelector};
use crate::protocol::Message;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;

/// 默认 ACK 超时时间（5 秒）
pub const DEFAULT_ACK_TIMEOUT_SECS: u64 = 5;

/// 默认重试次数
pub const DEFAULT_RETRY_COUNT: u32 = 3;

/// 同步错误类型
#[derive(Debug, Clone, PartialEq, Error)]
pub enum SyncError {
    /// 配置错误
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// 没有可用通道
    #[error("No channel available")]
    ChannelUnavailable,

    /// 发送超时
    #[error("Send timeout: {0}")]
    Timeout(String),

    /// 发送失败
    #[error("Send failed: {0}")]
    SendFailed(String),

    /// ACK 超时
    #[error("ACK timeout: {0}")]
    AckTimeout(String),

    /// 设备未连接
    #[error("Device not connected: {0}")]
    DeviceNotConnected(String),

    /// 平台不支持
    #[error("Platform not supported")]
    PlatformNotSupported,

    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// 剪贴板发送回调
///
/// 接收发送状态的通知。
pub trait ClipboardSendCallback: Send + Sync {
    /// 发送成功
    ///
    /// 当消息成功发送到目标设备时调用。
    /// 注意：此时可能尚未收到 ACK。
    fn on_send_success(&self, device_id: &str);

    /// 发送失败
    ///
    /// 当发送失败时调用。
    fn on_send_failure(&self, device_id: &str, error: SyncError);

    /// 收到 ACK
    ///
    /// 当收到目标设备的确认响应时调用。
    fn on_ack_received(&self, device_id: &str);
}

/// 剪贴板发送器配置
///
/// 配置发送器的超时和重试参数。
///
/// # Example
///
/// ```
/// use nearclip_sync::ClipboardSenderConfig;
/// use std::time::Duration;
///
/// let config = ClipboardSenderConfig::new()
///     .with_ack_timeout(Duration::from_secs(10))
///     .with_retry_count(5);
/// ```
#[derive(Debug, Clone)]
pub struct ClipboardSenderConfig {
    /// ACK 等待超时
    pub ack_timeout: Duration,
    /// 重试次数
    pub retry_count: u32,
    /// 本地设备 ID
    pub device_id: String,
}

impl ClipboardSenderConfig {
    /// 创建默认配置
    ///
    /// 使用默认值：
    /// - ACK 超时：5 秒
    /// - 重试次数：3 次
    /// - 设备 ID：空字符串（需要设置）
    pub fn new() -> Self {
        Self {
            ack_timeout: Duration::from_secs(DEFAULT_ACK_TIMEOUT_SECS),
            retry_count: DEFAULT_RETRY_COUNT,
            device_id: String::new(),
        }
    }

    /// 设置 ACK 超时时间
    pub fn with_ack_timeout(mut self, timeout: Duration) -> Self {
        self.ack_timeout = timeout;
        self
    }

    /// 设置重试次数
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// 设置本地设备 ID
    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = device_id.into();
        self
    }

    /// 验证配置
    ///
    /// # Returns
    ///
    /// 如果配置有效则返回 Ok，否则返回配置错误
    pub fn validate(&self) -> Result<(), SyncError> {
        if self.ack_timeout.is_zero() {
            return Err(SyncError::Configuration(
                "ACK timeout must be greater than zero".to_string(),
            ));
        }

        if self.device_id.is_empty() {
            return Err(SyncError::Configuration(
                "Device ID must not be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ClipboardSenderConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 发送状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SendStatus {
    /// 空闲
    #[default]
    Idle,
    /// 发送中
    Sending,
    /// 等待 ACK
    WaitingAck,
}

/// 内部发送器状态
struct SenderState {
    /// 当前发送状态
    status: SendStatus,
    /// 通道状态
    channels: Vec<ChannelInfo>,
}

impl SenderState {
    fn new() -> Self {
        Self {
            status: SendStatus::Idle,
            channels: vec![
                ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable),
                ChannelInfo::new(Channel::Ble, ChannelStatus::Unavailable),
            ],
        }
    }
}

/// 剪贴板发送器
///
/// 负责将剪贴板内容发送到目标设备。
/// 自动选择可用通道（WiFi 优先），并等待 ACK 确认。
///
/// # Example
///
/// ```
/// use nearclip_sync::{ClipboardSender, ClipboardSenderConfig, ClipboardSendCallback, SyncError};
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl ClipboardSendCallback for MyCallback {
///     fn on_send_success(&self, device_id: &str) {}
///     fn on_send_failure(&self, device_id: &str, error: SyncError) {}
///     fn on_ack_received(&self, device_id: &str) {}
/// }
///
/// # async fn example() -> Result<(), SyncError> {
/// let config = ClipboardSenderConfig::new()
///     .with_device_id("my-device-123");
/// let callback = Arc::new(MyCallback);
/// let sender = ClipboardSender::new(config, callback)?;
/// # Ok(())
/// # }
/// ```
pub struct ClipboardSender {
    config: ClipboardSenderConfig,
    callback: Arc<dyn ClipboardSendCallback>,
    state: Arc<RwLock<SenderState>>,
    channel_selector: Box<dyn ChannelSelector>,
}

impl ClipboardSender {
    /// 创建新的发送器
    ///
    /// # Arguments
    ///
    /// * `config` - 发送器配置
    /// * `callback` - 发送状态回调
    ///
    /// # Returns
    ///
    /// 新的发送器实例，或配置错误
    pub fn new(
        config: ClipboardSenderConfig,
        callback: Arc<dyn ClipboardSendCallback>,
    ) -> Result<Self, SyncError> {
        config.validate()?;

        Ok(Self {
            config,
            callback,
            state: Arc::new(RwLock::new(SenderState::new())),
            channel_selector: Box::new(PriorityChannelSelector),
        })
    }

    /// 使用自定义通道选择器创建发送器
    pub fn with_channel_selector(
        config: ClipboardSenderConfig,
        callback: Arc<dyn ClipboardSendCallback>,
        selector: Box<dyn ChannelSelector>,
    ) -> Result<Self, SyncError> {
        config.validate()?;

        Ok(Self {
            config,
            callback,
            state: Arc::new(RwLock::new(SenderState::new())),
            channel_selector: selector,
        })
    }

    /// 获取配置引用
    pub fn config(&self) -> &ClipboardSenderConfig {
        &self.config
    }

    /// 获取当前发送状态
    pub async fn status(&self) -> SendStatus {
        self.state.read().await.status
    }

    /// 更新通道状态
    ///
    /// # Arguments
    ///
    /// * `channel` - 要更新的通道
    /// * `status` - 新状态
    pub async fn update_channel_status(&self, channel: Channel, status: ChannelStatus) {
        let mut state = self.state.write().await;
        for info in &mut state.channels {
            if info.channel == channel {
                info.status = status;
                break;
            }
        }
    }

    /// 获取通道状态
    pub async fn channel_status(&self, channel: Channel) -> ChannelStatus {
        let state = self.state.read().await;
        state
            .channels
            .iter()
            .find(|info| info.channel == channel)
            .map(|info| info.status)
            .unwrap_or(ChannelStatus::Unavailable)
    }

    /// 选择可用通道
    async fn select_channel(&self) -> Option<Channel> {
        let state = self.state.read().await;
        self.channel_selector.select(&state.channels)
    }

    /// 发送剪贴板内容
    ///
    /// 构建 ClipboardSync 消息并通过可用通道发送。
    ///
    /// # Arguments
    ///
    /// * `content` - 剪贴板内容字节
    /// * `target_device_id` - 目标设备 ID
    ///
    /// # Returns
    ///
    /// 成功返回使用的通道，失败返回错误
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nearclip_sync::{ClipboardSender, ClipboardSenderConfig, ClipboardSendCallback, SyncError};
    /// # use std::sync::Arc;
    /// # struct MyCallback;
    /// # impl ClipboardSendCallback for MyCallback {
    /// #     fn on_send_success(&self, _: &str) {}
    /// #     fn on_send_failure(&self, _: &str, _: SyncError) {}
    /// #     fn on_ack_received(&self, _: &str) {}
    /// # }
    /// # async fn example() -> Result<(), SyncError> {
    /// # let config = ClipboardSenderConfig::new().with_device_id("dev");
    /// # let callback = Arc::new(MyCallback);
    /// # let sender = ClipboardSender::new(config, callback)?;
    /// let result = sender.send(b"Hello, World!", "target-device").await;
    /// match result {
    ///     Ok(channel) => println!("Sent via {:?}", channel),
    ///     Err(e) => println!("Failed: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, content: &[u8], target_device_id: &str) -> Result<Channel, SyncError> {
        // 1. 选择通道
        let channel = match self.select_channel().await {
            Some(ch) => ch,
            None => {
                let error = SyncError::ChannelUnavailable;
                self.callback.on_send_failure(target_device_id, error.clone());
                return Err(error);
            }
        };

        // 2. 更新状态为发送中
        {
            let mut state = self.state.write().await;
            state.status = SendStatus::Sending;
        }

        // 3. 构建消息
        let message = Message::clipboard_sync(content, self.config.device_id.clone());
        let _serialized = message
            .serialize()
            .map_err(|e| SyncError::Serialization(e.to_string()))?;

        // 4. 通过选定通道发送（当前返回 PlatformNotSupported）
        let send_result = self.send_via_channel(channel, target_device_id).await;

        // 5. 根据结果更新状态和触发回调
        match send_result {
            Ok(()) => {
                {
                    let mut state = self.state.write().await;
                    state.status = SendStatus::WaitingAck;
                }
                self.callback.on_send_success(target_device_id);
                Ok(channel)
            }
            Err(e) => {
                {
                    let mut state = self.state.write().await;
                    state.status = SendStatus::Idle;
                }
                self.callback.on_send_failure(target_device_id, e.clone());
                Err(e)
            }
        }
    }

    /// 通过指定通道发送数据
    ///
    /// 当前阶段返回 PlatformNotSupported，实际发送逻辑将在协调层中实现。
    async fn send_via_channel(
        &self,
        _channel: Channel,
        _target_device_id: &str,
    ) -> Result<(), SyncError> {
        // 当前阶段：所有平台返回 PlatformNotSupported
        // 实际发送逻辑将在 Story 3-11 (核心协调层) 中实现
        //
        // WiFi 通道需要：
        // - 已建立的 TcpConnection
        // - 调用 connection.write_all() 发送消息
        //
        // BLE 通道需要：
        // - 已连接的 CentralDataSender
        // - 调用 sender.send() 发送数据
        Err(SyncError::PlatformNotSupported)
    }

    /// 处理收到的 ACK
    ///
    /// 当收到目标设备的确认响应时调用。
    ///
    /// # Arguments
    ///
    /// * `device_id` - 发送 ACK 的设备 ID
    pub async fn handle_ack(&self, device_id: &str) {
        let current_status = {
            let state = self.state.read().await;
            state.status
        };

        if current_status == SendStatus::WaitingAck {
            {
                let mut state = self.state.write().await;
                state.status = SendStatus::Idle;
            }
            self.callback.on_ack_received(device_id);
        }
    }

    /// 重置状态
    ///
    /// 将发送器状态重置为空闲。
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        state.status = SendStatus::Idle;
    }
}

impl std::fmt::Debug for ClipboardSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipboardSender")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestCallback {
        success_count: AtomicUsize,
        failure_count: AtomicUsize,
        ack_count: AtomicUsize,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                success_count: AtomicUsize::new(0),
                failure_count: AtomicUsize::new(0),
                ack_count: AtomicUsize::new(0),
            }
        }

        #[allow(dead_code)]
        fn success_count(&self) -> usize {
            self.success_count.load(Ordering::Relaxed)
        }

        fn failure_count(&self) -> usize {
            self.failure_count.load(Ordering::Relaxed)
        }

        fn ack_count(&self) -> usize {
            self.ack_count.load(Ordering::Relaxed)
        }
    }

    impl ClipboardSendCallback for TestCallback {
        fn on_send_success(&self, _device_id: &str) {
            self.success_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_send_failure(&self, _device_id: &str, _error: SyncError) {
            self.failure_count.fetch_add(1, Ordering::Relaxed);
        }

        fn on_ack_received(&self, _device_id: &str) {
            self.ack_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn test_config_new() {
        let config = ClipboardSenderConfig::new();
        assert_eq!(
            config.ack_timeout,
            Duration::from_secs(DEFAULT_ACK_TIMEOUT_SECS)
        );
        assert_eq!(config.retry_count, DEFAULT_RETRY_COUNT);
        assert!(config.device_id.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = ClipboardSenderConfig::new()
            .with_ack_timeout(Duration::from_secs(10))
            .with_retry_count(5)
            .with_device_id("my-device");

        assert_eq!(config.ack_timeout, Duration::from_secs(10));
        assert_eq!(config.retry_count, 5);
        assert_eq!(config.device_id, "my-device");
    }

    #[test]
    fn test_config_validate_ok() {
        let config = ClipboardSenderConfig::new().with_device_id("dev-123");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_timeout() {
        let config = ClipboardSenderConfig::new()
            .with_device_id("dev")
            .with_ack_timeout(Duration::ZERO);

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::Configuration(_))));
    }

    #[test]
    fn test_config_validate_empty_device_id() {
        let config = ClipboardSenderConfig::new();

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::Configuration(_))));
    }

    #[test]
    fn test_config_default() {
        let config = ClipboardSenderConfig::default();
        assert_eq!(
            config.ack_timeout,
            Duration::from_secs(DEFAULT_ACK_TIMEOUT_SECS)
        );
    }

    #[test]
    fn test_config_clone() {
        let config = ClipboardSenderConfig::new()
            .with_device_id("dev")
            .with_retry_count(7);
        let cloned = config.clone();
        assert_eq!(config.retry_count, cloned.retry_count);
        assert_eq!(config.device_id, cloned.device_id);
    }

    #[test]
    fn test_sender_new_invalid_config() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new(); // Empty device_id

        let result = ClipboardSender::new(config, callback);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sender_new_valid() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback);
        assert!(sender.is_ok());
    }

    #[tokio::test]
    async fn test_sender_initial_status() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback).unwrap();
        assert_eq!(sender.status().await, SendStatus::Idle);
    }

    #[tokio::test]
    async fn test_sender_update_channel_status() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback).unwrap();

        // Initially unavailable
        assert_eq!(
            sender.channel_status(Channel::Wifi).await,
            ChannelStatus::Unavailable
        );

        // Update to available
        sender
            .update_channel_status(Channel::Wifi, ChannelStatus::Available)
            .await;
        assert_eq!(
            sender.channel_status(Channel::Wifi).await,
            ChannelStatus::Available
        );
    }

    #[tokio::test]
    async fn test_sender_send_no_channel() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback.clone()).unwrap();

        // No channels available
        let result = sender.send(b"test content", "target-device").await;
        assert!(matches!(result, Err(SyncError::ChannelUnavailable)));
        assert_eq!(callback.failure_count(), 1);
    }

    #[tokio::test]
    async fn test_sender_send_platform_not_supported() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback.clone()).unwrap();

        // Make WiFi available
        sender
            .update_channel_status(Channel::Wifi, ChannelStatus::Available)
            .await;

        // Should fail with PlatformNotSupported
        let result = sender.send(b"test content", "target-device").await;
        assert!(matches!(result, Err(SyncError::PlatformNotSupported)));
        assert_eq!(callback.failure_count(), 1);
    }

    #[tokio::test]
    async fn test_sender_handle_ack_when_waiting() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback.clone()).unwrap();

        // Manually set state to WaitingAck
        {
            let mut state = sender.state.write().await;
            state.status = SendStatus::WaitingAck;
        }

        sender.handle_ack("target-device").await;

        assert_eq!(sender.status().await, SendStatus::Idle);
        assert_eq!(callback.ack_count(), 1);
    }

    #[tokio::test]
    async fn test_sender_handle_ack_when_idle() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback.clone()).unwrap();

        // Already idle
        sender.handle_ack("target-device").await;

        // Should not trigger callback
        assert_eq!(callback.ack_count(), 0);
    }

    #[tokio::test]
    async fn test_sender_reset() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback).unwrap();

        // Set to sending
        {
            let mut state = sender.state.write().await;
            state.status = SendStatus::Sending;
        }

        sender.reset().await;
        assert_eq!(sender.status().await, SendStatus::Idle);
    }

    #[tokio::test]
    async fn test_sender_config_accessor() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new()
            .with_device_id("test-device")
            .with_retry_count(7);

        let sender = ClipboardSender::new(config, callback).unwrap();
        assert_eq!(sender.config().retry_count, 7);
        assert_eq!(sender.config().device_id, "test-device");
    }

    #[test]
    fn test_send_status_default() {
        assert_eq!(SendStatus::default(), SendStatus::Idle);
    }

    #[test]
    fn test_send_status_eq() {
        assert_eq!(SendStatus::Idle, SendStatus::Idle);
        assert_ne!(SendStatus::Idle, SendStatus::Sending);
    }

    #[test]
    fn test_sync_error_display() {
        let err = SyncError::ChannelUnavailable;
        assert!(err.to_string().contains("No channel available"));

        let err = SyncError::Timeout("test".into());
        assert!(err.to_string().contains("Send timeout"));

        let err = SyncError::PlatformNotSupported;
        assert!(err.to_string().contains("Platform not supported"));
    }

    #[test]
    fn test_sync_error_clone_eq() {
        let err1 = SyncError::ChannelUnavailable;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[tokio::test]
    async fn test_sender_debug() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardSenderConfig::new().with_device_id("test-device");

        let sender = ClipboardSender::new(config, callback).unwrap();
        let debug_str = format!("{:?}", sender);
        assert!(debug_str.contains("ClipboardSender"));
    }

    #[test]
    fn test_config_debug() {
        let config = ClipboardSenderConfig::new().with_device_id("dev");
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ClipboardSenderConfig"));
    }
}
