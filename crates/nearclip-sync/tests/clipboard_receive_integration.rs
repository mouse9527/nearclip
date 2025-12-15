//! 剪贴板接收集成测试
//!
//! 这些测试验证剪贴板接收的完整流程。
//!
//! 主要测试:
//! - 接收器创建和配置
//! - 消息解析和验证
//! - 回调触发
//! - ACK 生成

use nearclip_sync::{
    ClipboardReceiveCallback, ClipboardReceiver, ClipboardReceiverConfig, Message, MessageType,
    SyncError, DEFAULT_MAX_MESSAGE_SIZE, DEFAULT_MESSAGE_TIMEOUT_SECS,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    received_contents: Arc<Mutex<Vec<(Vec<u8>, String)>>>,
    errors: Arc<Mutex<Vec<String>>>,
    received_count: AtomicUsize,
    error_count: AtomicUsize,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            received_contents: Arc::new(Mutex::new(Vec::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
            received_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
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

    #[allow(dead_code)]
    fn get_errors(&self) -> Vec<String> {
        self.errors.lock().unwrap().clone()
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

// ============================================================
// 配置测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = ClipboardReceiverConfig::new();

    assert_eq!(config.max_message_size, DEFAULT_MAX_MESSAGE_SIZE);
    assert_eq!(
        config.message_timeout,
        Duration::from_secs(DEFAULT_MESSAGE_TIMEOUT_SECS)
    );
    assert!(config.device_id.is_empty());
}

#[test]
fn test_config_builder_chain() {
    let config = ClipboardReceiverConfig::new()
        .with_device_id("my-device-123")
        .with_max_message_size(2 * 1024 * 1024)
        .with_message_timeout(Duration::from_secs(60));

    assert_eq!(config.device_id, "my-device-123");
    assert_eq!(config.max_message_size, 2 * 1024 * 1024);
    assert_eq!(config.message_timeout, Duration::from_secs(60));
}

#[test]
fn test_config_validation_success() {
    let config = ClipboardReceiverConfig::new().with_device_id("valid-device");
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_empty_device_id() {
    let config = ClipboardReceiverConfig::new();
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(SyncError::Configuration(_))));
}

#[test]
fn test_config_validation_zero_message_size() {
    let config = ClipboardReceiverConfig::new()
        .with_device_id("device")
        .with_max_message_size(0);
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_config_validation_zero_timeout() {
    let config = ClipboardReceiverConfig::new()
        .with_device_id("device")
        .with_message_timeout(Duration::ZERO);
    let result = config.validate();
    assert!(result.is_err());
}

// ============================================================
// 接收器测试
// ============================================================

#[test]
fn test_receiver_creation_success() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("test-device");

    let receiver = ClipboardReceiver::new(config, callback);
    assert!(receiver.is_ok());
}

#[test]
fn test_receiver_creation_failure_invalid_config() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new(); // Missing device_id

    let receiver = ClipboardReceiver::new(config, callback);
    assert!(receiver.is_err());
}

#[test]
fn test_receiver_config_accessor() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new()
        .with_device_id("my-device")
        .with_max_message_size(500_000);

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    assert_eq!(receiver.config().device_id, "my-device");
    assert_eq!(receiver.config().max_message_size, 500_000);
}

// ============================================================
// 消息处理测试
// ============================================================

#[test]
fn test_handle_valid_clipboard_sync() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // Create a valid ClipboardSync message
    let content = b"Hello, World! This is clipboard content.";
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
}

#[test]
fn test_handle_message_returns_ack() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let msg = Message::clipboard_sync(b"Test", "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data).unwrap();

    // Verify ACK is valid
    let ack_msg = Message::deserialize(&result.ack_data).unwrap();
    assert_eq!(ack_msg.msg_type, MessageType::Ack);
    assert_eq!(ack_msg.device_id, "receiver-device");
}

#[test]
fn test_handle_invalid_data() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // Invalid MessagePack data
    let result = receiver.handle_message(b"not valid messagepack data");

    assert!(result.is_err());
    assert_eq!(callback.error_count(), 1);
    assert_eq!(callback.received_count(), 0);
}

#[test]
fn test_handle_wrong_message_type_heartbeat() {
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
fn test_handle_wrong_message_type_ack() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // ACK message instead of ClipboardSync
    let msg = Message::ack("sender-device".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data);

    assert!(result.is_err());
    assert_eq!(callback.error_count(), 1);
}

#[test]
fn test_handle_too_large_message() {
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

// ============================================================
// ACK 生成测试
// ============================================================

#[test]
fn test_create_ack() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let msg = Message::clipboard_sync(b"test content", "sender".to_string());
    let ack_data = receiver.create_ack(&msg);

    assert!(ack_data.is_ok());

    // Verify the ACK message
    let ack_data = ack_data.unwrap();
    let ack_msg = Message::deserialize(&ack_data).unwrap();
    assert_eq!(ack_msg.msg_type, MessageType::Ack);
    assert_eq!(ack_msg.device_id, "receiver-device");
}

#[test]
fn test_ack_roundtrip() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    // Original message
    let original = Message::clipboard_sync(b"Hello!", "sender".to_string());

    // Create ACK
    let ack_data = receiver.create_ack(&original).unwrap();

    // Deserialize and verify
    let ack_msg = Message::deserialize(&ack_data).unwrap();
    assert!(ack_msg.timestamp > 0);
    assert!(!ack_msg.is_expired(30000)); // ACK should be fresh
}

// ============================================================
// 消息验证测试
// ============================================================

#[test]
fn test_validate_message_valid() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let msg = Message::clipboard_sync(b"test", "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.validate_message(&data);
    assert!(result.is_ok());

    let validated = result.unwrap();
    assert_eq!(validated.msg_type, MessageType::ClipboardSync);
}

#[test]
fn test_validate_message_invalid_type() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-device");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let msg = Message::heartbeat("sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.validate_message(&data);
    assert!(result.is_err());
}

#[test]
fn test_validate_message_too_large() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new()
        .with_device_id("receiver-device")
        .with_max_message_size(50);

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let large_content: Vec<u8> = vec![0u8; 100];
    let msg = Message::clipboard_sync(&large_content, "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.validate_message(&data);
    assert!(result.is_err());
}

// ============================================================
// 多消息处理测试
// ============================================================

#[test]
fn test_handle_multiple_messages_sequentially() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // Handle multiple messages
    for i in 0..10 {
        let content = format!("Message content {}", i);
        let msg = Message::clipboard_sync(content.as_bytes(), format!("sender-{}", i));
        let data = msg.serialize().unwrap();

        let result = receiver.handle_message(&data);
        assert!(result.is_ok());
    }

    assert_eq!(callback.received_count(), 10);
    assert_eq!(callback.error_count(), 0);

    let received = callback.get_received();
    assert_eq!(received.len(), 10);

    // Verify order
    for (i, (content, device_id)) in received.iter().enumerate() {
        let expected_content = format!("Message content {}", i);
        assert_eq!(content, expected_content.as_bytes());
        assert_eq!(device_id, &format!("sender-{}", i));
    }
}

#[test]
fn test_handle_mixed_valid_and_invalid_messages() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // Valid message
    let msg1 = Message::clipboard_sync(b"Valid 1", "sender-1".to_string());
    assert!(receiver.handle_message(&msg1.serialize().unwrap()).is_ok());

    // Invalid message (wrong type)
    let msg2 = Message::heartbeat("sender-2".to_string());
    assert!(receiver.handle_message(&msg2.serialize().unwrap()).is_err());

    // Valid message
    let msg3 = Message::clipboard_sync(b"Valid 2", "sender-3".to_string());
    assert!(receiver.handle_message(&msg3.serialize().unwrap()).is_ok());

    // Invalid data
    assert!(receiver.handle_message(b"invalid data").is_err());

    // Valid message
    let msg4 = Message::clipboard_sync(b"Valid 3", "sender-4".to_string());
    assert!(receiver.handle_message(&msg4.serialize().unwrap()).is_ok());

    assert_eq!(callback.received_count(), 3);
    assert_eq!(callback.error_count(), 2);
}

// ============================================================
// 特殊内容测试
// ============================================================

#[test]
fn test_handle_unicode_content() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    let content = "你好，世界！🎉 Hello, 世界! Привет мир!".as_bytes();
    let msg = Message::clipboard_sync(content, "sender-中文".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data);
    assert!(result.is_ok());

    let received = callback.get_received();
    assert_eq!(received[0].0, content);
    assert_eq!(received[0].1, "sender-中文");
}

#[test]
fn test_handle_binary_content() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // Binary content with all byte values
    let content: Vec<u8> = (0..=255).collect();
    let msg = Message::clipboard_sync(&content, "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data);
    assert!(result.is_ok());

    let received = callback.get_received();
    assert_eq!(received[0].0, content);
}

#[test]
fn test_handle_empty_content() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    let msg = Message::clipboard_sync(b"", "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data);
    assert!(result.is_ok());

    let received = callback.get_received();
    assert_eq!(received[0].0.len(), 0);
}

#[test]
fn test_handle_large_content() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new()
        .with_device_id("receiver")
        .with_max_message_size(10 * 1024 * 1024); // 10 MB

    let receiver = ClipboardReceiver::new(config, callback.clone()).unwrap();

    // 1 MB content
    let large_content: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let msg = Message::clipboard_sync(&large_content, "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data);
    assert!(result.is_ok());

    let received = callback.get_received();
    assert_eq!(received[0].0.len(), 1_000_000);
}

// ============================================================
// ReceiveResult 测试
// ============================================================

#[test]
fn test_receive_result_contains_original_message() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let content = b"Original content";
    let msg = Message::clipboard_sync(content, "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data).unwrap();

    // ReceiveResult contains the parsed message
    assert_eq!(result.message.msg_type, MessageType::ClipboardSync);
    assert_eq!(result.message.payload, content.to_vec());
    assert_eq!(result.message.device_id, "sender");
}

#[test]
fn test_receive_result_ack_data_is_valid() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardReceiverConfig::new().with_device_id("receiver-42");

    let receiver = ClipboardReceiver::new(config, callback).unwrap();

    let msg = Message::clipboard_sync(b"test", "sender".to_string());
    let data = msg.serialize().unwrap();

    let result = receiver.handle_message(&data).unwrap();

    // ACK data should be deserializable
    let ack = Message::deserialize(&result.ack_data).unwrap();
    assert_eq!(ack.msg_type, MessageType::Ack);
    assert_eq!(ack.device_id, "receiver-42");
}

// ============================================================
// 错误类型测试
// ============================================================

#[test]
fn test_sync_error_variants_display() {
    let errors = vec![
        SyncError::Configuration("test config".into()),
        SyncError::ChannelUnavailable,
        SyncError::Timeout("test timeout".into()),
        SyncError::SendFailed("test send".into()),
        SyncError::AckTimeout("test ack".into()),
        SyncError::DeviceNotConnected("test device".into()),
        SyncError::PlatformNotSupported,
        SyncError::Serialization("test serial".into()),
    ];

    for error in errors {
        // All errors should have a non-empty display
        let display = error.to_string();
        assert!(!display.is_empty());
    }
}

#[test]
fn test_sync_error_clone_eq() {
    let err1 = SyncError::ChannelUnavailable;
    let err2 = err1.clone();
    assert_eq!(err1, err2);
}
