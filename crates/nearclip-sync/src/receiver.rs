//! 剪贴板内容接收
//!
//! 实现剪贴板内容的接收和解析逻辑，包括消息验证和 ACK 生成。
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{ClipboardReceiver, ClipboardReceiverConfig, ClipboardReceiveCallback, SyncError};
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl ClipboardReceiveCallback for MyCallback {
//!     fn on_clipboard_received(&self, content: &[u8], device_id: &str) {
//!         println!("Received {} bytes from {}", content.len(), device_id);
//!     }
//!     fn on_receive_error(&self, error: SyncError) {
//!         println!("Error: {}", error);
//!     }
//! }
//!
//! let callback = Arc::new(MyCallback);
//! let config = ClipboardReceiverConfig::new().with_device_id("my-device");
//! let receiver = ClipboardReceiver::new(config, callback).unwrap();
//! ```

use crate::protocol::{Message, MessageType, ProtocolError};
use crate::sender::SyncError;
use std::sync::Arc;
use std::time::Duration;

/// 默认最大消息大小（1 MB）
pub const DEFAULT_MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// 默认消息超时时间（30 秒）
pub const DEFAULT_MESSAGE_TIMEOUT_SECS: u64 = 30;

/// 剪贴板接收回调
///
/// 接收剪贴板内容的通知。
pub trait ClipboardReceiveCallback: Send + Sync {
    /// 收到剪贴板内容
    ///
    /// 当成功解析并验证 ClipboardSync 消息后调用。
    ///
    /// # Arguments
    ///
    /// * `content` - 剪贴板内容字节
    /// * `device_id` - 发送方设备 ID
    fn on_clipboard_received(&self, content: &[u8], device_id: &str);

    /// 接收错误
    ///
    /// 当接收或解析消息失败时调用。
    fn on_receive_error(&self, error: SyncError);
}

/// 剪贴板接收器配置
///
/// 配置接收器的消息大小限制和超时参数。
///
/// # Example
///
/// ```
/// use nearclip_sync::ClipboardReceiverConfig;
/// use std::time::Duration;
///
/// let config = ClipboardReceiverConfig::new()
///     .with_device_id("my-device")
///     .with_max_message_size(2 * 1024 * 1024)  // 2 MB
///     .with_message_timeout(Duration::from_secs(60));
/// ```
#[derive(Debug, Clone)]
pub struct ClipboardReceiverConfig {
    /// 最大消息大小（字节）
    pub max_message_size: usize,
    /// 消息超时时间（过期消息将被拒绝）
    pub message_timeout: Duration,
    /// 本地设备 ID（用于生成 ACK）
    pub device_id: String,
}

impl ClipboardReceiverConfig {
    /// 创建默认配置
    ///
    /// 使用默认值：
    /// - 最大消息大小：1 MB
    /// - 消息超时：30 秒
    /// - 设备 ID：空字符串（需要设置）
    pub fn new() -> Self {
        Self {
            max_message_size: DEFAULT_MAX_MESSAGE_SIZE,
            message_timeout: Duration::from_secs(DEFAULT_MESSAGE_TIMEOUT_SECS),
            device_id: String::new(),
        }
    }

    /// 设置最大消息大小
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    /// 设置消息超时时间
    pub fn with_message_timeout(mut self, timeout: Duration) -> Self {
        self.message_timeout = timeout;
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
        if self.max_message_size == 0 {
            return Err(SyncError::Configuration(
                "Max message size must be greater than zero".to_string(),
            ));
        }

        if self.message_timeout.is_zero() {
            return Err(SyncError::Configuration(
                "Message timeout must be greater than zero".to_string(),
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

impl Default for ClipboardReceiverConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 接收结果
///
/// 包含接收到的消息和生成的 ACK。
#[derive(Debug)]
pub struct ReceiveResult {
    /// 原始消息
    pub message: Message,
    /// 序列化后的 ACK 数据（可直接发送）
    pub ack_data: Vec<u8>,
}

/// 剪贴板接收器
///
/// 负责接收和解析剪贴板内容消息。
/// 被动接收模式：上层传入接收到的数据，接收器解析并触发回调。
///
/// # Example
///
/// ```
/// use nearclip_sync::{ClipboardReceiver, ClipboardReceiverConfig, ClipboardReceiveCallback, SyncError, Message};
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl ClipboardReceiveCallback for MyCallback {
///     fn on_clipboard_received(&self, content: &[u8], device_id: &str) {}
///     fn on_receive_error(&self, error: SyncError) {}
/// }
///
/// let config = ClipboardReceiverConfig::new().with_device_id("my-device");
/// let callback = Arc::new(MyCallback);
/// let receiver = ClipboardReceiver::new(config, callback).unwrap();
///
/// // 模拟接收到的消息
/// let msg = Message::clipboard_sync(b"Hello!", "sender-device".to_string());
/// let data = msg.serialize().unwrap();
///
/// // 处理接收到的数据
/// let result = receiver.handle_message(&data);
/// assert!(result.is_ok());
/// ```
pub struct ClipboardReceiver {
    config: ClipboardReceiverConfig,
    callback: Arc<dyn ClipboardReceiveCallback>,
}

impl ClipboardReceiver {
    /// 创建新的接收器
    ///
    /// # Arguments
    ///
    /// * `config` - 接收器配置
    /// * `callback` - 接收回调
    ///
    /// # Returns
    ///
    /// 新的接收器实例，或配置错误
    pub fn new(
        config: ClipboardReceiverConfig,
        callback: Arc<dyn ClipboardReceiveCallback>,
    ) -> Result<Self, SyncError> {
        config.validate()?;

        Ok(Self { config, callback })
    }

    /// 获取配置引用
    pub fn config(&self) -> &ClipboardReceiverConfig {
        &self.config
    }

    /// 处理接收到的消息数据
    ///
    /// 解析消息、验证类型和大小、触发回调、生成 ACK。
    ///
    /// # Arguments
    ///
    /// * `data` - 接收到的原始消息数据（MessagePack 序列化）
    ///
    /// # Returns
    ///
    /// 成功返回 `ReceiveResult`（包含消息和 ACK），失败返回错误
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::{ClipboardReceiver, ClipboardReceiverConfig, ClipboardReceiveCallback, SyncError, Message};
    /// use std::sync::Arc;
    ///
    /// struct MyCallback;
    /// impl ClipboardReceiveCallback for MyCallback {
    ///     fn on_clipboard_received(&self, content: &[u8], device_id: &str) {
    ///         println!("Got: {:?} from {}", content, device_id);
    ///     }
    ///     fn on_receive_error(&self, error: SyncError) {}
    /// }
    ///
    /// let config = ClipboardReceiverConfig::new().with_device_id("my-device");
    /// let callback = Arc::new(MyCallback);
    /// let receiver = ClipboardReceiver::new(config, callback).unwrap();
    ///
    /// let msg = Message::clipboard_sync(b"Test", "sender".to_string());
    /// let data = msg.serialize().unwrap();
    /// let result = receiver.handle_message(&data);
    /// ```
    pub fn handle_message(&self, data: &[u8]) -> Result<ReceiveResult, SyncError> {
        // 1. 检查数据大小
        if data.len() > self.config.max_message_size {
            let error = SyncError::SendFailed(format!(
                "Message too large: {} bytes (max: {})",
                data.len(),
                self.config.max_message_size
            ));
            self.callback.on_receive_error(error.clone());
            return Err(error);
        }

        // 2. 反序列化消息
        let message = Message::deserialize(data).map_err(|e| {
            let error = self.protocol_error_to_sync_error(e);
            self.callback.on_receive_error(error.clone());
            error
        })?;

        // 3. 验证消息类型
        if message.msg_type != MessageType::ClipboardSync {
            let error = SyncError::SendFailed(format!(
                "Unexpected message type: {:?}, expected ClipboardSync",
                message.msg_type
            ));
            self.callback.on_receive_error(error.clone());
            return Err(error);
        }

        // 4. 验证消息是否过期
        let timeout_ms = self.config.message_timeout.as_millis() as u64;
        if message.is_expired(timeout_ms) {
            let error = SyncError::Timeout(format!(
                "Message expired (age: {}ms, max: {}ms)",
                message.age_ms(),
                timeout_ms
            ));
            self.callback.on_receive_error(error.clone());
            return Err(error);
        }

        // 5. 触发回调
        self.callback
            .on_clipboard_received(&message.payload, &message.device_id);

        // 6. 生成 ACK
        let ack_data = self.create_ack(&message)?;

        Ok(ReceiveResult {
            message,
            ack_data,
        })
    }

    /// 创建 ACK 响应
    ///
    /// 根据原始消息生成 ACK 消息。
    ///
    /// # Arguments
    ///
    /// * `original_message` - 收到的原始消息
    ///
    /// # Returns
    ///
    /// 序列化后的 ACK 数据，可直接发送给对方
    pub fn create_ack(&self, _original_message: &Message) -> Result<Vec<u8>, SyncError> {
        let ack = Message::ack(self.config.device_id.clone());
        ack.serialize()
            .map_err(|e| self.protocol_error_to_sync_error(e))
    }

    /// 验证数据是否为有效的 ClipboardSync 消息
    ///
    /// 不触发回调，仅验证消息格式和类型。
    ///
    /// # Arguments
    ///
    /// * `data` - 待验证的数据
    ///
    /// # Returns
    ///
    /// 有效返回 Ok(Message)，无效返回错误
    pub fn validate_message(&self, data: &[u8]) -> Result<Message, SyncError> {
        // 检查大小
        if data.len() > self.config.max_message_size {
            return Err(SyncError::SendFailed(format!(
                "Message too large: {} bytes",
                data.len()
            )));
        }

        // 反序列化
        let message = Message::deserialize(data)
            .map_err(|e| self.protocol_error_to_sync_error(e))?;

        // 验证类型
        if message.msg_type != MessageType::ClipboardSync {
            return Err(SyncError::SendFailed(format!(
                "Unexpected message type: {:?}",
                message.msg_type
            )));
        }

        Ok(message)
    }

    /// 将 ProtocolError 转换为 SyncError
    fn protocol_error_to_sync_error(&self, error: ProtocolError) -> SyncError {
        match error {
            ProtocolError::Serialization(msg) => SyncError::Serialization(msg),
            ProtocolError::Deserialization(msg) => SyncError::Serialization(msg),
        }
    }
}

impl std::fmt::Debug for ClipboardReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipboardReceiver")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    struct TestCallback {
        received_count: AtomicUsize,
        error_count: AtomicUsize,
        received_contents: Mutex<Vec<(Vec<u8>, String)>>,
        errors: Mutex<Vec<String>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                received_count: AtomicUsize::new(0),
                error_count: AtomicUsize::new(0),
                received_contents: Mutex::new(Vec::new()),
                errors: Mutex::new(Vec::new()),
            }
        }

        fn received_count(&self) -> usize {
            self.received_count.load(Ordering::Relaxed)
        }

        fn error_count(&self) -> usize {
            self.error_count.load(Ordering::Relaxed)
        }

        fn get_received(&self) -> Vec<(Vec<u8>, String)> {
            self.received_contents.lock().unwrap().clone()
        }
    }

    impl ClipboardReceiveCallback for TestCallback {
        fn on_clipboard_received(&self, content: &[u8], device_id: &str) {
            self.received_count.fetch_add(1, Ordering::Relaxed);
            self.received_contents
                .lock()
                .unwrap()
                .push((content.to_vec(), device_id.to_string()));
        }

        fn on_receive_error(&self, error: SyncError) {
            self.error_count.fetch_add(1, Ordering::Relaxed);
            self.errors.lock().unwrap().push(error.to_string());
        }
    }

    #[test]
    fn test_config_new() {
        let config = ClipboardReceiverConfig::new();
        assert_eq!(config.max_message_size, DEFAULT_MAX_MESSAGE_SIZE);
        assert_eq!(
            config.message_timeout,
            Duration::from_secs(DEFAULT_MESSAGE_TIMEOUT_SECS)
        );
        assert!(config.device_id.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = ClipboardReceiverConfig::new()
            .with_max_message_size(2 * 1024 * 1024)
            .with_message_timeout(Duration::from_secs(60))
            .with_device_id("my-device");

        assert_eq!(config.max_message_size, 2 * 1024 * 1024);
        assert_eq!(config.message_timeout, Duration::from_secs(60));
        assert_eq!(config.device_id, "my-device");
    }

    #[test]
    fn test_config_validate_ok() {
        let config = ClipboardReceiverConfig::new().with_device_id("dev-123");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_size() {
        let config = ClipboardReceiverConfig::new()
            .with_device_id("dev")
            .with_max_message_size(0);

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::Configuration(_))));
    }

    #[test]
    fn test_config_validate_zero_timeout() {
        let config = ClipboardReceiverConfig::new()
            .with_device_id("dev")
            .with_message_timeout(Duration::ZERO);

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_empty_device_id() {
        let config = ClipboardReceiverConfig::new();

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_default() {
        let config = ClipboardReceiverConfig::default();
        assert_eq!(config.max_message_size, DEFAULT_MAX_MESSAGE_SIZE);
    }

    #[test]
    fn test_config_clone() {
        let config = ClipboardReceiverConfig::new()
            .with_device_id("dev")
            .with_max_message_size(500);
        let cloned = config.clone();
        assert_eq!(config.max_message_size, cloned.max_message_size);
    }

    #[test]
    fn test_receiver_new_valid() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("test-device");

        let receiver = ClipboardReceiver::new(config, callback);
        assert!(receiver.is_ok());
    }

    #[test]
    fn test_receiver_new_invalid_config() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new(); // Missing device_id

        let receiver = ClipboardReceiver::new(config, callback);
        assert!(receiver.is_err());
    }

    #[test]
    fn test_receiver_handle_valid_message() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

        // Create a valid ClipboardSync message
        let content = b"Hello, World!";
        let msg = Message::clipboard_sync(content, "sender-device".to_string());
        let data = msg.serialize().unwrap();

        // Handle the message
        let result = receiver.handle_message(&data);
        assert!(result.is_ok());

        // Verify callback was triggered
        assert_eq!(callback.received_count(), 1);
        assert_eq!(callback.error_count(), 0);

        let received = callback.get_received();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].0, content.to_vec());
        assert_eq!(received[0].1, "sender-device");

        // Verify ACK was generated
        let result = result.unwrap();
        assert!(!result.ack_data.is_empty());
    }

    #[test]
    fn test_receiver_handle_invalid_data() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

        // Invalid data
        let result = receiver.handle_message(b"not valid messagepack");

        assert!(result.is_err());
        assert_eq!(callback.error_count(), 1);
        assert_eq!(callback.received_count(), 0);
    }

    #[test]
    fn test_receiver_handle_wrong_message_type() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

        // Heartbeat message instead of ClipboardSync
        let msg = Message::heartbeat("sender-device".to_string());
        let data = msg.serialize().unwrap();

        let result = receiver.handle_message(&data);

        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::SendFailed(_))));
        assert_eq!(callback.error_count(), 1);
    }

    #[test]
    fn test_receiver_handle_too_large_message() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new()
            .with_device_id("receiver-device")
            .with_max_message_size(100); // Very small limit

        let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

        // Create a message that exceeds the limit
        let large_content: Vec<u8> = vec![0u8; 200];
        let msg = Message::clipboard_sync(&large_content, "sender".to_string());
        let data = msg.serialize().unwrap();

        let result = receiver.handle_message(&data);

        assert!(result.is_err());
        assert_eq!(callback.error_count(), 1);
    }

    #[test]
    fn test_receiver_create_ack() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback).unwrap();

        let msg = Message::clipboard_sync(b"test", "sender".to_string());
        let ack_data = receiver.create_ack(&msg);

        assert!(ack_data.is_ok());

        // Verify the ACK is a valid message
        let ack_data = ack_data.unwrap();
        let ack_msg = Message::deserialize(&ack_data).unwrap();
        assert_eq!(ack_msg.msg_type, MessageType::Ack);
        assert_eq!(ack_msg.device_id, "receiver-device");
    }

    #[test]
    fn test_receiver_validate_message_valid() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback).unwrap();

        let msg = Message::clipboard_sync(b"test", "sender".to_string());
        let data = msg.serialize().unwrap();

        let result = receiver.validate_message(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_receiver_validate_message_invalid_type() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

        let receiver = ClipboardReceiver::new(config, callback).unwrap();

        let msg = Message::ack("sender".to_string());
        let data = msg.serialize().unwrap();

        let result = receiver.validate_message(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_receiver_config_accessor() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new()
            .with_device_id("test-device")
            .with_max_message_size(500);

        let receiver = ClipboardReceiver::new(config, callback).unwrap();

        assert_eq!(receiver.config().device_id, "test-device");
        assert_eq!(receiver.config().max_message_size, 500);
    }

    #[test]
    fn test_receiver_debug() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("test-device");

        let receiver = ClipboardReceiver::new(config, callback).unwrap();
        let debug_str = format!("{:?}", receiver);
        assert!(debug_str.contains("ClipboardReceiver"));
    }

    #[test]
    fn test_config_debug() {
        let config = ClipboardReceiverConfig::new().with_device_id("dev");
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ClipboardReceiverConfig"));
    }

    #[test]
    fn test_receive_result_debug() {
        let msg = Message::clipboard_sync(b"test", "sender".to_string());
        let result = ReceiveResult {
            message: msg,
            ack_data: vec![1, 2, 3],
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ReceiveResult"));
    }

    #[test]
    fn test_multiple_messages() {
        let callback = Arc::new(TestCallback::new());
        let config = ClipboardReceiverConfig::new().with_device_id("receiver");

        let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

        // Handle multiple messages
        for i in 0..5 {
            let content = format!("Message {}", i);
            let msg = Message::clipboard_sync(content.as_bytes(), format!("sender-{}", i));
            let data = msg.serialize().unwrap();

            let result = receiver.handle_message(&data);
            assert!(result.is_ok());
        }

        assert_eq!(callback.received_count(), 5);
        assert_eq!(callback.error_count(), 0);

        let received = callback.get_received();
        assert_eq!(received.len(), 5);
    }
}
