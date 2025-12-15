//! 剪贴板发送集成测试
//!
//! 这些测试验证剪贴板发送的完整流程。
//!
//! 主要测试:
//! - 发送器创建和配置
//! - 通道选择逻辑
//! - 回调触发
//! - 状态管理

use nearclip_sync::{
    BleOnlyChannelSelector, Channel, ChannelInfo, ChannelSelector, ChannelStatus,
    ClipboardSendCallback, ClipboardSender, ClipboardSenderConfig, Message, MessageType,
    PriorityChannelSelector, SendStatus, SyncError, WifiOnlyChannelSelector,
    DEFAULT_ACK_TIMEOUT_SECS, DEFAULT_RETRY_COUNT,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    success_devices: Arc<Mutex<Vec<String>>>,
    failure_devices: Arc<Mutex<Vec<(String, String)>>>,
    ack_devices: Arc<Mutex<Vec<String>>>,
    success_count: AtomicUsize,
    failure_count: AtomicUsize,
    ack_count: AtomicUsize,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            success_devices: Arc::new(Mutex::new(Vec::new())),
            failure_devices: Arc::new(Mutex::new(Vec::new())),
            ack_devices: Arc::new(Mutex::new(Vec::new())),
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

    #[allow(dead_code)]
    fn get_success_devices(&self) -> Vec<String> {
        self.success_devices.lock().unwrap().clone()
    }

    fn get_failure_devices(&self) -> Vec<(String, String)> {
        self.failure_devices.lock().unwrap().clone()
    }
}

impl ClipboardSendCallback for TestCallback {
    fn on_send_success(&self, device_id: &str) {
        self.success_count.fetch_add(1, Ordering::Relaxed);
        self.success_devices
            .lock()
            .unwrap()
            .push(device_id.to_string());
    }

    fn on_send_failure(&self, device_id: &str, error: SyncError) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        self.failure_devices
            .lock()
            .unwrap()
            .push((device_id.to_string(), error.to_string()));
    }

    fn on_ack_received(&self, device_id: &str) {
        self.ack_count.fetch_add(1, Ordering::Relaxed);
        self.ack_devices.lock().unwrap().push(device_id.to_string());
    }
}

// ============================================================
// 配置测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = ClipboardSenderConfig::new();

    assert_eq!(
        config.ack_timeout,
        Duration::from_secs(DEFAULT_ACK_TIMEOUT_SECS)
    );
    assert_eq!(config.retry_count, DEFAULT_RETRY_COUNT);
}

#[test]
fn test_config_builder_chain() {
    let config = ClipboardSenderConfig::new()
        .with_device_id("my-device-123")
        .with_ack_timeout(Duration::from_secs(30))
        .with_retry_count(10);

    assert_eq!(config.device_id, "my-device-123");
    assert_eq!(config.ack_timeout, Duration::from_secs(30));
    assert_eq!(config.retry_count, 10);
}

#[test]
fn test_config_validation_success() {
    let config = ClipboardSenderConfig::new().with_device_id("valid-device");
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_empty_device_id() {
    let config = ClipboardSenderConfig::new();
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(SyncError::Configuration(_))));
}

#[test]
fn test_config_validation_zero_timeout() {
    let config = ClipboardSenderConfig::new()
        .with_device_id("device")
        .with_ack_timeout(Duration::ZERO);
    let result = config.validate();
    assert!(result.is_err());
}

// ============================================================
// 通道选择测试
// ============================================================

#[test]
fn test_priority_selector_prefers_wifi() {
    let selector = PriorityChannelSelector;
    let channels = vec![
        ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
    ];

    let selected = selector.select(&channels);
    assert_eq!(selected, Some(Channel::Wifi));
}

#[test]
fn test_priority_selector_fallback_to_ble() {
    let selector = PriorityChannelSelector;
    let channels = vec![
        ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable),
    ];

    let selected = selector.select(&channels);
    assert_eq!(selected, Some(Channel::Ble));
}

#[test]
fn test_priority_selector_no_available_channels() {
    let selector = PriorityChannelSelector;
    let channels = vec![
        ChannelInfo::new(Channel::Ble, ChannelStatus::Busy),
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Connecting),
    ];

    let selected = selector.select(&channels);
    assert_eq!(selected, None);
}

#[test]
fn test_wifi_only_selector() {
    let selector = WifiOnlyChannelSelector;

    // WiFi available - should select WiFi
    let channels = vec![
        ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
    ];
    assert_eq!(selector.select(&channels), Some(Channel::Wifi));

    // Only BLE available - should select None
    let channels = vec![
        ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable),
    ];
    assert_eq!(selector.select(&channels), None);
}

#[test]
fn test_ble_only_selector() {
    let selector = BleOnlyChannelSelector;

    // BLE available - should select BLE
    let channels = vec![
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
        ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
    ];
    assert_eq!(selector.select(&channels), Some(Channel::Ble));

    // Only WiFi available - should select None
    let channels = vec![
        ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
        ChannelInfo::new(Channel::Ble, ChannelStatus::Unavailable),
    ];
    assert_eq!(selector.select(&channels), None);
}

// ============================================================
// 发送器测试
// ============================================================

#[tokio::test]
async fn test_sender_creation_success() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback);
    assert!(sender.is_ok());
}

#[tokio::test]
async fn test_sender_creation_failure_invalid_config() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new(); // Missing device_id

    let sender = ClipboardSender::new(config, callback);
    assert!(sender.is_err());
}

#[tokio::test]
async fn test_sender_initial_state() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback).unwrap();

    assert_eq!(sender.status().await, SendStatus::Idle);
    assert_eq!(
        sender.channel_status(Channel::Wifi).await,
        ChannelStatus::Unavailable
    );
    assert_eq!(
        sender.channel_status(Channel::Ble).await,
        ChannelStatus::Unavailable
    );
}

#[tokio::test]
async fn test_sender_update_channel_status() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback).unwrap();

    // Update WiFi to available
    sender
        .update_channel_status(Channel::Wifi, ChannelStatus::Available)
        .await;
    assert_eq!(
        sender.channel_status(Channel::Wifi).await,
        ChannelStatus::Available
    );

    // Update BLE to busy
    sender
        .update_channel_status(Channel::Ble, ChannelStatus::Busy)
        .await;
    assert_eq!(
        sender.channel_status(Channel::Ble).await,
        ChannelStatus::Busy
    );
}

#[tokio::test]
async fn test_sender_send_no_channel_available() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback.clone()).unwrap();

    // No channels available
    let result = sender.send(b"Hello, World!", "target-device").await;

    assert!(matches!(result, Err(SyncError::ChannelUnavailable)));
    assert_eq!(callback.failure_count(), 1);

    let failures = callback.get_failure_devices();
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].0, "target-device");
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

    // Should fail with PlatformNotSupported (current stub implementation)
    let result = sender.send(b"Hello, World!", "target-device").await;

    assert!(matches!(result, Err(SyncError::PlatformNotSupported)));
    assert_eq!(callback.failure_count(), 1);
}

#[tokio::test]
async fn test_sender_handle_ack_when_waiting() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback.clone()).unwrap();

    // Simulate sending state by making WiFi available and attempting send
    // Then manually set to WaitingAck
    sender
        .update_channel_status(Channel::Wifi, ChannelStatus::Available)
        .await;

    // Reset after failed send
    sender.reset().await;

    // Manually simulate waiting state (in real scenario, this would be set after successful send)
    // For now, we can't truly test this without mocking the channel

    // Handle ACK when idle should not trigger callback
    sender.handle_ack("target-device").await;
    assert_eq!(callback.ack_count(), 0);
}

#[tokio::test]
async fn test_sender_reset() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = ClipboardSender::new(config, callback).unwrap();

    // Make WiFi available and try to send (will fail but change state)
    sender
        .update_channel_status(Channel::Wifi, ChannelStatus::Available)
        .await;
    let _ = sender.send(b"test", "target").await;

    // Reset should return to Idle
    sender.reset().await;
    assert_eq!(sender.status().await, SendStatus::Idle);
}

#[tokio::test]
async fn test_sender_config_accessor() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new()
        .with_device_id("my-device")
        .with_retry_count(7)
        .with_ack_timeout(Duration::from_secs(15));

    let sender = ClipboardSender::new(config, callback).unwrap();

    assert_eq!(sender.config().device_id, "my-device");
    assert_eq!(sender.config().retry_count, 7);
    assert_eq!(sender.config().ack_timeout, Duration::from_secs(15));
}

// ============================================================
// 消息构建测试
// ============================================================

#[test]
fn test_clipboard_sync_message_construction() {
    let content = b"Hello, World!";
    let device_id = "device-123";

    let msg = Message::clipboard_sync(content, device_id.to_string());

    assert_eq!(msg.msg_type, MessageType::ClipboardSync);
    assert_eq!(msg.payload, content.to_vec());
    assert_eq!(msg.device_id, device_id);
    assert!(msg.timestamp > 0);
}

#[test]
fn test_message_serialization_roundtrip() {
    let content = b"Test clipboard content";
    let msg = Message::clipboard_sync(content, "device-456".to_string());

    let serialized = msg.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert_eq!(msg.msg_type, deserialized.msg_type);
    assert_eq!(msg.payload, deserialized.payload);
    assert_eq!(msg.device_id, deserialized.device_id);
    assert_eq!(msg.timestamp, deserialized.timestamp);
}

#[test]
fn test_message_large_payload() {
    let large_content: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let msg = Message::clipboard_sync(&large_content, "device".to_string());

    let serialized = msg.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.payload.len(), 10000);
    assert_eq!(deserialized.payload, large_content);
}

#[test]
fn test_message_unicode_content() {
    let content = "你好，世界！🎉 Hello, World!".as_bytes();
    let msg = Message::clipboard_sync(content, "设备-123".to_string());

    let serialized = msg.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert_eq!(deserialized.payload, content);
    assert_eq!(deserialized.device_id, "设备-123");
}

// ============================================================
// Channel 枚举测试
// ============================================================

#[test]
fn test_channel_properties() {
    // WiFi properties
    assert_eq!(Channel::Wifi.as_str(), "wifi");
    assert!(Channel::Wifi.is_high_speed());
    assert!(Channel::Wifi.priority() > Channel::Ble.priority());

    // BLE properties
    assert_eq!(Channel::Ble.as_str(), "ble");
    assert!(!Channel::Ble.is_high_speed());
}

#[test]
fn test_channel_default() {
    assert_eq!(Channel::default(), Channel::Wifi);
}

#[test]
fn test_channel_display() {
    assert_eq!(format!("{}", Channel::Wifi), "wifi");
    assert_eq!(format!("{}", Channel::Ble), "ble");
}

// ============================================================
// ChannelStatus 枚举测试
// ============================================================

#[test]
fn test_channel_status_can_send() {
    assert!(ChannelStatus::Available.can_send());
    assert!(!ChannelStatus::Unavailable.can_send());
    assert!(!ChannelStatus::Busy.can_send());
    assert!(!ChannelStatus::Connecting.can_send());
}

#[test]
fn test_channel_status_display() {
    assert_eq!(format!("{}", ChannelStatus::Available), "available");
    assert_eq!(format!("{}", ChannelStatus::Unavailable), "unavailable");
    assert_eq!(format!("{}", ChannelStatus::Busy), "busy");
    assert_eq!(format!("{}", ChannelStatus::Connecting), "connecting");
}

// ============================================================
// SyncError 测试
// ============================================================

#[test]
fn test_sync_error_variants() {
    let errors = vec![
        SyncError::Configuration("test".into()),
        SyncError::ChannelUnavailable,
        SyncError::Timeout("test".into()),
        SyncError::SendFailed("test".into()),
        SyncError::AckTimeout("test".into()),
        SyncError::DeviceNotConnected("test".into()),
        SyncError::PlatformNotSupported,
        SyncError::Serialization("test".into()),
    ];

    for error in errors {
        // All errors should have a non-empty display
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn test_sync_error_clone_eq() {
    let err1 = SyncError::ChannelUnavailable;
    let err2 = err1.clone();
    assert_eq!(err1, err2);

    let err3 = SyncError::Timeout("test".into());
    let err4 = SyncError::Timeout("test".into());
    assert_eq!(err3, err4);
}

// ============================================================
// SendStatus 测试
// ============================================================

#[test]
fn test_send_status_variants() {
    assert_eq!(SendStatus::default(), SendStatus::Idle);
    assert_ne!(SendStatus::Idle, SendStatus::Sending);
    assert_ne!(SendStatus::Sending, SendStatus::WaitingAck);
}

// ============================================================
// 并发测试
// ============================================================

#[tokio::test]
async fn test_concurrent_status_reads() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = Arc::new(ClipboardSender::new(config, callback).unwrap());

    let mut handles = vec![];

    for _ in 0..10 {
        let s = Arc::clone(&sender);
        handles.push(tokio::spawn(async move { s.status().await }));
    }

    for handle in handles {
        let status = handle.await.unwrap();
        assert_eq!(status, SendStatus::Idle);
    }
}

#[tokio::test]
async fn test_concurrent_channel_updates() {
    let callback = Arc::new(TestCallback::new());
    let config = ClipboardSenderConfig::new().with_device_id("test-device");

    let sender = Arc::new(ClipboardSender::new(config, callback).unwrap());

    let mut handles = vec![];

    // Concurrent updates to different channels
    for i in 0..5 {
        let s = Arc::clone(&sender);
        let channel = if i % 2 == 0 {
            Channel::Wifi
        } else {
            Channel::Ble
        };
        handles.push(tokio::spawn(async move {
            s.update_channel_status(channel, ChannelStatus::Available)
                .await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // At least one of the channels should be available
    let wifi_status = sender.channel_status(Channel::Wifi).await;
    let ble_status = sender.channel_status(Channel::Ble).await;
    assert!(
        wifi_status == ChannelStatus::Available || ble_status == ChannelStatus::Available
    );
}
