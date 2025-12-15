//! BLE 中心端数据发送集成测试
//!
//! 这些测试验证 BLE 中心端数据发送的完整流程。
//!
//! 主要测试:
//! - CentralDataConfig 配置验证
//! - CentralDataSender 创建和状态管理
//! - 数据发送流程
//! - 回调触发

use nearclip_ble::{
    BleError, CentralDataConfig, CentralDataSender, ChunkHeader, Chunker, DataSenderCallback,
    SendState, DEFAULT_ACK_TIMEOUT_MS, DEFAULT_MTU, DEFAULT_RETRY_COUNT, DEFAULT_SEND_TIMEOUT_SECS,
    CHUNK_HEADER_SIZE,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    complete_messages: Arc<Mutex<Vec<u16>>>,
    error_messages: Arc<Mutex<Vec<(u16, String)>>>,
    ack_messages: Arc<Mutex<Vec<u16>>>,
    complete_count: AtomicUsize,
    error_count: AtomicUsize,
    ack_count: AtomicUsize,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            complete_messages: Arc::new(Mutex::new(Vec::new())),
            error_messages: Arc::new(Mutex::new(Vec::new())),
            ack_messages: Arc::new(Mutex::new(Vec::new())),
            complete_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
            ack_count: AtomicUsize::new(0),
        }
    }

    fn complete_count(&self) -> usize {
        self.complete_count.load(Ordering::Relaxed)
    }

    fn error_count(&self) -> usize {
        self.error_count.load(Ordering::Relaxed)
    }

    fn ack_count(&self) -> usize {
        self.ack_count.load(Ordering::Relaxed)
    }

    fn get_complete_messages(&self) -> Vec<u16> {
        self.complete_messages.lock().unwrap().clone()
    }
}

impl DataSenderCallback for TestCallback {
    fn on_send_complete(&self, message_id: u16) {
        self.complete_count.fetch_add(1, Ordering::Relaxed);
        self.complete_messages.lock().unwrap().push(message_id);
    }

    fn on_send_error(&self, message_id: u16, error: BleError) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
        self.error_messages
            .lock()
            .unwrap()
            .push((message_id, error.to_string()));
    }

    fn on_ack_received(&self, message_id: u16) {
        self.ack_count.fetch_add(1, Ordering::Relaxed);
        self.ack_messages.lock().unwrap().push(message_id);
    }
}

// ============================================================
// CentralDataConfig 测试
// ============================================================

#[test]
fn test_central_data_config_default() {
    let config = CentralDataConfig::new();

    assert_eq!(config.mtu, DEFAULT_MTU);
    assert_eq!(
        config.send_timeout,
        Duration::from_secs(DEFAULT_SEND_TIMEOUT_SECS)
    );
    assert_eq!(config.retry_count, DEFAULT_RETRY_COUNT);
    assert_eq!(
        config.ack_timeout,
        Duration::from_millis(DEFAULT_ACK_TIMEOUT_MS)
    );
}

#[test]
fn test_central_data_config_builder() {
    let config = CentralDataConfig::new()
        .with_mtu(256)
        .with_send_timeout(Duration::from_secs(60))
        .with_retry_count(5)
        .with_ack_timeout(Duration::from_secs(10));

    assert_eq!(config.mtu, 256);
    assert_eq!(config.send_timeout, Duration::from_secs(60));
    assert_eq!(config.retry_count, 5);
    assert_eq!(config.ack_timeout, Duration::from_secs(10));
}

#[test]
fn test_central_data_config_validation() {
    // 有效配置
    let valid_config = CentralDataConfig::new();
    assert!(valid_config.validate().is_ok());

    // MTU 太小
    let small_mtu = CentralDataConfig::new().with_mtu(5);
    assert!(small_mtu.validate().is_err());

    // 边界值：刚好有效的 MTU
    let min_mtu = CHUNK_HEADER_SIZE + 3 + 1; // ATT_HEADER + CHUNK_HEADER + 1
    let boundary_mtu = CentralDataConfig::new().with_mtu(min_mtu);
    assert!(boundary_mtu.validate().is_ok());

    // 发送超时为零
    let zero_send_timeout = CentralDataConfig::new().with_send_timeout(Duration::ZERO);
    assert!(zero_send_timeout.validate().is_err());

    // ACK 超时为零
    let zero_ack_timeout = CentralDataConfig::new().with_ack_timeout(Duration::ZERO);
    assert!(zero_ack_timeout.validate().is_err());
}

#[test]
fn test_central_data_config_clone() {
    let config = CentralDataConfig::new()
        .with_mtu(128)
        .with_retry_count(10);

    let cloned = config.clone();
    assert_eq!(config.mtu, cloned.mtu);
    assert_eq!(config.retry_count, cloned.retry_count);
}

// ============================================================
// CentralDataSender 测试
// ============================================================

#[tokio::test]
async fn test_sender_creation() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let sender = CentralDataSender::new(config, callback).await;
    assert!(sender.is_ok());
}

#[tokio::test]
async fn test_sender_invalid_config() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new().with_mtu(5);

    let sender = CentralDataSender::new(config, callback).await;
    assert!(sender.is_err());
    assert!(matches!(sender.unwrap_err(), BleError::Configuration(_)));
}

#[tokio::test]
async fn test_sender_initial_state() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let sender = CentralDataSender::new(config, callback).await.unwrap();

    assert_eq!(sender.state().await, SendState::Idle);
    assert!(!sender.is_connected().await);
    assert!(sender.connected_device().await.is_none());

    let (sent, total) = sender.send_progress().await;
    assert_eq!(sent, 0);
    assert_eq!(total, 0);
}

#[tokio::test]
async fn test_sender_connect_platform_not_supported() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let mut sender = CentralDataSender::new(config, callback).await.unwrap();

    let result = sender.connect("device-001").await;

    // 当前所有平台返回 PlatformNotSupported
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BleError::PlatformNotSupported));
}

#[tokio::test]
async fn test_sender_send_without_connection() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let mut sender = CentralDataSender::new(config, callback).await.unwrap();

    let result = sender.send(b"Hello, World!").await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BleError::DataTransfer(_)));
}

#[tokio::test]
async fn test_sender_disconnect_when_idle() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let mut sender = CentralDataSender::new(config, callback).await.unwrap();

    // 断开未连接的发送器应该成功
    let result = sender.disconnect().await;
    assert!(result.is_ok());
    assert_eq!(sender.state().await, SendState::Idle);
}

#[tokio::test]
async fn test_sender_config_accessor() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new().with_mtu(256).with_retry_count(7);

    let sender = CentralDataSender::new(config, callback).await.unwrap();

    assert_eq!(sender.config().mtu, 256);
    assert_eq!(sender.config().retry_count, 7);
}

// ============================================================
// ACK 处理测试
// ============================================================

#[tokio::test]
async fn test_sender_handle_ack_no_active_message() {
    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let sender = CentralDataSender::new(config, callback.clone()).await.unwrap();

    // 在没有活跃消息时发送 ACK，不应触发回调
    sender.handle_ack(42).await;

    // 由于没有匹配的消息 ID，不应触发回调
    assert_eq!(callback.ack_count(), 0);
}

// ============================================================
// SendState 测试
// ============================================================

#[test]
fn test_send_state_equality() {
    assert_eq!(SendState::Idle, SendState::Idle);
    assert_eq!(SendState::Connected, SendState::Connected);
    assert_eq!(SendState::Sending, SendState::Sending);
    assert_eq!(SendState::WaitingAck, SendState::WaitingAck);

    assert_ne!(SendState::Idle, SendState::Connected);
    assert_ne!(SendState::Connected, SendState::Sending);
    assert_ne!(SendState::Sending, SendState::WaitingAck);
}

#[test]
fn test_send_state_debug() {
    // 验证 Debug trait 实现
    let state = SendState::Idle;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Idle"));
}

// ============================================================
// DataSenderCallback 测试
// ============================================================

#[test]
fn test_callback_on_send_complete() {
    let callback = TestCallback::new();

    callback.on_send_complete(1);
    callback.on_send_complete(2);
    callback.on_send_complete(3);

    assert_eq!(callback.complete_count(), 3);

    let messages = callback.get_complete_messages();
    assert_eq!(messages, vec![1, 2, 3]);
}

#[test]
fn test_callback_on_send_error() {
    let callback = TestCallback::new();

    callback.on_send_error(1, BleError::Timeout("test".into()));
    callback.on_send_error(2, BleError::DataTransfer("failed".into()));

    assert_eq!(callback.error_count(), 2);
}

#[test]
fn test_callback_on_ack_received() {
    let callback = TestCallback::new();

    callback.on_ack_received(1);
    callback.on_ack_received(2);

    assert_eq!(callback.ack_count(), 2);
}

// ============================================================
// 与 Chunker 集成测试
// ============================================================

#[test]
fn test_chunker_integration_small_data() {
    // 验证小数据分片
    let data = b"Hello";
    let chunks = Chunker::chunk(data, 1, DEFAULT_MTU).unwrap();

    assert_eq!(chunks.len(), 1);

    let header = ChunkHeader::from_bytes(&chunks[0][..CHUNK_HEADER_SIZE]).unwrap();
    assert_eq!(header.message_id, 1);
    assert_eq!(header.total_chunks, 1);
}

#[test]
fn test_chunker_integration_large_data() {
    // 验证大数据分片
    let data: Vec<u8> = (0..100).collect();
    let chunks = Chunker::chunk(&data, 42, DEFAULT_MTU).unwrap();

    // 每个分片 payload 最大 12 字节 (23 - 3 - 8)
    // 100 字节需要 ceil(100/12) = 9 个分片
    assert!(chunks.len() > 1);

    // 验证所有分片属于同一消息
    for chunk in &chunks {
        let header = ChunkHeader::from_bytes(&chunk[..CHUNK_HEADER_SIZE]).unwrap();
        assert_eq!(header.message_id, 42);
        assert_eq!(header.total_chunks, chunks.len() as u16);
    }
}

// ============================================================
// BleError 相关测试
// ============================================================

#[test]
fn test_ble_error_connection_failed() {
    let err = BleError::ConnectionFailed("device not found".to_string());
    assert!(err.to_string().contains("Connection failed"));
    assert!(err.to_string().contains("device not found"));
}

#[test]
fn test_ble_error_data_transfer() {
    let err = BleError::DataTransfer("not connected".to_string());
    assert!(err.to_string().contains("Data transfer error"));
    assert!(err.to_string().contains("not connected"));
}

#[test]
fn test_ble_error_timeout() {
    let err = BleError::Timeout("operation timed out".to_string());
    assert!(err.to_string().contains("Timeout"));
}

#[test]
fn test_ble_error_platform_not_supported() {
    let err = BleError::PlatformNotSupported;
    assert!(err.to_string().contains("Platform not supported"));
}

// ============================================================
// 并发访问测试
// ============================================================

#[tokio::test]
async fn test_sender_concurrent_state_access() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let sender = Arc::new(Mutex::new(
        CentralDataSender::new(config, callback).await.unwrap(),
    ));

    // 启动多个并发任务读取状态
    let mut handles = vec![];

    for _ in 0..10 {
        let s = Arc::clone(&sender);
        handles.push(tokio::spawn(async move {
            let guard = s.lock().await;
            guard.state().await
        }));
    }

    // 所有任务应该成功完成且返回 Idle
    for handle in handles {
        let state = handle.await.unwrap();
        assert_eq!(state, SendState::Idle);
    }
}

#[tokio::test]
async fn test_sender_concurrent_disconnect() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let callback = Arc::new(TestCallback::new());
    let config = CentralDataConfig::new();

    let sender = Arc::new(Mutex::new(
        CentralDataSender::new(config, callback).await.unwrap(),
    ));

    // 并发调用 disconnect（幂等操作）
    let mut handles = vec![];

    for _ in 0..5 {
        let s = Arc::clone(&sender);
        handles.push(tokio::spawn(async move {
            let mut guard = s.lock().await;
            guard.disconnect().await
        }));
    }

    // 所有 disconnect 调用应该成功
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // 最终状态应该是 Idle
    let guard = sender.lock().await;
    assert_eq!(guard.state().await, SendState::Idle);
}
