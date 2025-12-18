//! NearClipManager 集成测试
//!
//! 这些测试验证核心管理器的完整流程。
//!
//! 主要测试:
//! - 完整生命周期 (create -> start -> stop)
//! - 设备连接流程
//! - 剪贴板同步流程

use nearclip_core::{
    DeviceInfo, DevicePlatform, DeviceStatus, NearClipCallback, NearClipConfig, NearClipError,
    NearClipManager, NoOpCallback, DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_DEVICE_NAME,
    DEFAULT_HEARTBEAT_INTERVAL_SECS, DEFAULT_MAX_RETRIES,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    connected: Mutex<Vec<String>>,
    disconnected: Mutex<Vec<String>>,
    clipboard_received: Mutex<Vec<(Vec<u8>, String)>>,
    errors: Mutex<Vec<String>>,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            connected: Mutex::new(Vec::new()),
            disconnected: Mutex::new(Vec::new()),
            clipboard_received: Mutex::new(Vec::new()),
            errors: Mutex::new(Vec::new()),
        }
    }

    fn connected_ids(&self) -> Vec<String> {
        self.connected.lock().unwrap().clone()
    }

    fn disconnected_ids(&self) -> Vec<String> {
        self.disconnected.lock().unwrap().clone()
    }

    fn received_clipboard(&self) -> Vec<(Vec<u8>, String)> {
        self.clipboard_received.lock().unwrap().clone()
    }

    fn error_messages(&self) -> Vec<String> {
        self.errors.lock().unwrap().clone()
    }
}

impl NearClipCallback for TestCallback {
    fn on_device_connected(&self, device: &DeviceInfo) {
        self.connected.lock().unwrap().push(device.id().to_string());
    }

    fn on_device_disconnected(&self, device_id: &str) {
        self.disconnected
            .lock()
            .unwrap()
            .push(device_id.to_string());
    }

    fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
        self.clipboard_received
            .lock()
            .unwrap()
            .push((content.to_vec(), from_device.to_string()));
    }

    fn on_sync_error(&self, error: &NearClipError) {
        self.errors.lock().unwrap().push(error.to_string());
    }
}

// ============================================================
// 常量测试
// ============================================================

#[test]
fn test_default_constants() {
    assert_eq!(DEFAULT_DEVICE_NAME, "NearClip Device");
    assert_eq!(DEFAULT_CONNECTION_TIMEOUT_SECS, 30);
    assert_eq!(DEFAULT_HEARTBEAT_INTERVAL_SECS, 10);
    assert_eq!(DEFAULT_MAX_RETRIES, 3);
}

// ============================================================
// 配置测试
// ============================================================

#[test]
fn test_config_creation() {
    let config = NearClipConfig::new("Test Device");
    assert_eq!(config.device_name(), "Test Device");
    assert!(config.wifi_enabled());
    assert!(config.ble_enabled());
    assert!(config.auto_connect());
}

#[test]
fn test_config_builder_full() {
    let config = NearClipConfig::new("Device")
        .with_wifi_enabled(false)
        .with_ble_enabled(true)
        .with_auto_connect(false)
        .with_connection_timeout(Duration::from_secs(60))
        .with_heartbeat_interval(Duration::from_secs(5))
        .with_max_retries(5)
        .with_mdns_service_name("_custom._tcp.local.");

    assert!(!config.wifi_enabled());
    assert!(config.ble_enabled());
    assert!(!config.auto_connect());
    assert_eq!(config.connection_timeout(), Duration::from_secs(60));
    assert_eq!(config.heartbeat_interval(), Duration::from_secs(5));
    assert_eq!(config.max_retries(), 5);
    assert_eq!(config.mdns_service_name(), "_custom._tcp.local.");
}

#[test]
fn test_config_validation() {
    // 有效配置
    let valid = NearClipConfig::new("Device");
    assert!(valid.validate().is_ok());

    // 空名称
    let empty_name = NearClipConfig::new("");
    assert!(empty_name.validate().is_err());

    // 没有通道
    let no_channels = NearClipConfig::new("Device")
        .with_wifi_enabled(false)
        .with_ble_enabled(false);
    assert!(no_channels.validate().is_err());
}

// ============================================================
// 设备信息测试
// ============================================================

#[test]
fn test_device_info_creation() {
    let device = DeviceInfo::new("device-123", "Test Device")
        .with_platform(DevicePlatform::MacOS)
        .with_status(DeviceStatus::Connected);

    assert_eq!(device.id(), "device-123");
    assert_eq!(device.name(), "Test Device");
    assert_eq!(device.platform(), DevicePlatform::MacOS);
    assert_eq!(device.status(), DeviceStatus::Connected);
}

#[test]
fn test_device_status_transitions() {
    let mut device = DeviceInfo::new("d1", "Device");

    assert_eq!(device.status(), DeviceStatus::Disconnected);
    assert!(device.status().can_connect());

    device.set_status(DeviceStatus::Connecting);
    assert_eq!(device.status(), DeviceStatus::Connecting);
    assert!(!device.status().can_connect());

    device.set_status(DeviceStatus::Connected);
    assert_eq!(device.status(), DeviceStatus::Connected);
    assert!(device.status().is_connected());

    device.set_status(DeviceStatus::Failed);
    assert_eq!(device.status(), DeviceStatus::Failed);
    assert!(device.status().can_connect());
}

#[test]
fn test_device_platform_display() {
    assert_eq!(DevicePlatform::MacOS.to_string(), "macOS");
    assert_eq!(DevicePlatform::Android.to_string(), "Android");
    assert_eq!(DevicePlatform::Unknown.to_string(), "Unknown");
}

// ============================================================
// 管理器创建测试
// ============================================================

#[test]
fn test_manager_creation_success() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let result = NearClipManager::new(config, callback);
    assert!(result.is_ok());
}

#[test]
fn test_manager_creation_invalid_config() {
    let config = NearClipConfig::new(""); // 无效
    let callback = Arc::new(NoOpCallback);
    let result = NearClipManager::new(config, callback);
    assert!(result.is_err());
}

#[test]
fn test_manager_initial_state() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    assert!(!manager.is_running());
    assert_eq!(manager.current_channel(), None);
    assert!(manager.get_paired_devices().is_empty());
    assert!(manager.get_connected_devices().is_empty());
}

// ============================================================
// 管理器生命周期测试
// ============================================================

#[tokio::test]
async fn test_manager_start_stop() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    assert!(!manager.is_running());

    manager.start().await.unwrap();
    assert!(manager.is_running());
    assert!(manager.current_channel().is_some());

    manager.stop().await;
    assert!(!manager.is_running());
    assert!(manager.current_channel().is_none());
}

#[tokio::test]
async fn test_manager_double_start() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();
    assert!(manager.is_running());

    // 第二次启动不应该失败
    manager.start().await.unwrap();
    assert!(manager.is_running());
}

#[tokio::test]
async fn test_manager_double_stop() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();
    manager.stop().await;
    assert!(!manager.is_running());

    // 第二次停止不应该失败
    manager.stop().await;
    assert!(!manager.is_running());
}

// ============================================================
// 通道选择测试
// ============================================================

#[tokio::test]
async fn test_manager_wifi_channel() {
    let config = NearClipConfig::new("Test")
        .with_wifi_enabled(true)
        .with_ble_enabled(false);
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();
    assert_eq!(
        manager.current_channel(),
        Some(nearclip_sync::Channel::Wifi)
    );
}

#[tokio::test]
async fn test_manager_ble_channel() {
    let config = NearClipConfig::new("Test")
        .with_wifi_enabled(false)
        .with_ble_enabled(true);
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();
    assert_eq!(
        manager.current_channel(),
        Some(nearclip_sync::Channel::Ble)
    );
}

// ============================================================
// 设备管理测试
// ============================================================

#[test]
fn test_manager_add_remove_devices() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let d1 = DeviceInfo::new("d1", "Device 1").with_platform(DevicePlatform::MacOS);
    let d2 = DeviceInfo::new("d2", "Device 2").with_platform(DevicePlatform::Android);

    manager.add_paired_device(d1);
    manager.add_paired_device(d2);
    assert_eq!(manager.get_paired_devices().len(), 2);

    let removed = manager.remove_paired_device("d1");
    assert!(removed.is_some());
    assert_eq!(manager.get_paired_devices().len(), 1);

    let not_found = manager.remove_paired_device("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_manager_get_device_status() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let device = DeviceInfo::new("d1", "Device 1").with_status(DeviceStatus::Connected);
    manager.add_paired_device(device);

    assert_eq!(
        manager.get_device_status("d1"),
        Some(DeviceStatus::Connected)
    );
    assert_eq!(manager.get_device_status("nonexistent"), None);
}

#[test]
fn test_manager_connected_devices_filter() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let d1 = DeviceInfo::new("d1", "D1").with_status(DeviceStatus::Connected);
    let d2 = DeviceInfo::new("d2", "D2").with_status(DeviceStatus::Disconnected);
    let d3 = DeviceInfo::new("d3", "D3").with_status(DeviceStatus::Connected);
    let d4 = DeviceInfo::new("d4", "D4").with_status(DeviceStatus::Connecting);

    manager.add_paired_device(d1);
    manager.add_paired_device(d2);
    manager.add_paired_device(d3);
    manager.add_paired_device(d4);

    assert_eq!(manager.get_paired_devices().len(), 4);
    assert_eq!(manager.get_connected_devices().len(), 2);
}

// ============================================================
// 设备连接/断开测试
// ============================================================

#[tokio::test]
async fn test_manager_connect_device_not_discovered() {
    // connect_device() 需要设备先通过 mDNS 发现才能连接
    // 如果设备已配对但未在网络上发现，应返回 Network 错误
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    let device = DeviceInfo::new("d1", "Device 1");
    manager.add_paired_device(device);
    manager.start().await.unwrap();

    // 设备已配对但未在网络上发现
    let result = manager.connect_device("d1").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(NearClipError::Network(_))));

    // 设备状态应保持不变
    assert_eq!(
        manager.get_device_status("d1"),
        Some(DeviceStatus::Disconnected)
    );
    // 不应触发连接回调
    assert!(callback.connected_ids().is_empty());
}

#[tokio::test]
async fn test_manager_connect_device_not_found() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();
    let result = manager.connect_device("nonexistent").await;

    assert!(result.is_err());
    assert!(matches!(result, Err(NearClipError::DeviceNotFound(_))));
}

#[tokio::test]
async fn test_manager_connect_device_not_running() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let device = DeviceInfo::new("d1", "Device 1");
    manager.add_paired_device(device);

    let result = manager.connect_device("d1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_manager_disconnect_device() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    let device = DeviceInfo::new("d1", "Device 1").with_status(DeviceStatus::Connected);
    manager.add_paired_device(device);

    manager.disconnect_device("d1").await.unwrap();

    assert_eq!(
        manager.get_device_status("d1"),
        Some(DeviceStatus::Disconnected)
    );
    assert_eq!(callback.disconnected_ids(), vec!["d1"]);
}

// ============================================================
// 剪贴板同步测试
// ============================================================

#[tokio::test]
async fn test_manager_sync_clipboard_not_running() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let result = manager.sync_clipboard(b"test content").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_manager_sync_clipboard_no_devices() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    manager.start().await.unwrap();

    // 没有设备时应该成功但不发送
    let result = manager.sync_clipboard(b"test content").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_manager_sync_clipboard_with_connected_device() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = NearClipManager::new(config, callback).unwrap();

    let device = DeviceInfo::new("d1", "Device 1").with_status(DeviceStatus::Connected);
    manager.add_paired_device(device);

    manager.start().await.unwrap();

    let result = manager.sync_clipboard(b"test content").await;
    assert!(result.is_ok());
}

// ============================================================
// 回调测试
// ============================================================

#[test]
fn test_callback_clipboard_received() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    let device = DeviceInfo::new("d1", "Device 1");
    manager.add_paired_device(device);

    manager.handle_clipboard_received(b"hello world", "d1");

    let received = callback.received_clipboard();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].0, b"hello world");
    assert_eq!(received[0].1, "d1");
}

#[test]
fn test_callback_sync_error() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    manager.handle_sync_error(NearClipError::Network("connection failed".to_string()));

    let errors = callback.error_messages();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("connection failed"));
}

#[test]
fn test_callback_device_connected() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    let device = DeviceInfo::new("d1", "Device 1").with_status(DeviceStatus::Connected);
    manager.handle_device_connected(device);

    assert_eq!(callback.connected_ids(), vec!["d1"]);
    assert_eq!(manager.get_paired_devices().len(), 1);
}

#[test]
fn test_callback_device_disconnected() {
    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    let device = DeviceInfo::new("d1", "Device 1").with_status(DeviceStatus::Connected);
    manager.add_paired_device(device);

    manager.handle_device_disconnected("d1");

    assert_eq!(callback.disconnected_ids(), vec!["d1"]);
    assert_eq!(
        manager.get_device_status("d1"),
        Some(DeviceStatus::Disconnected)
    );
}

// ============================================================
// NoOpCallback 测试
// ============================================================

#[test]
fn test_noop_callback_no_panic() {
    let callback = NoOpCallback;
    let device = DeviceInfo::new("d1", "Device 1");
    let error = NearClipError::Network("test".to_string());

    // 这些都不应该 panic
    callback.on_device_connected(&device);
    callback.on_device_disconnected("d1");
    callback.on_clipboard_received(b"content", "d1");
    callback.on_sync_error(&error);
}

// ============================================================
// 并发测试
// ============================================================

#[test]
fn test_manager_concurrent_device_access() {
    use std::thread;

    let config = NearClipConfig::new("Test Device");
    let callback = Arc::new(NoOpCallback);
    let manager = Arc::new(NearClipManager::new(config, callback).unwrap());

    // 添加一些设备
    for i in 0..10 {
        let device = DeviceInfo::new(format!("d{}", i), format!("Device {}", i));
        manager.add_paired_device(device);
    }

    let mut handles = vec![];

    // 并发读取
    for _ in 0..5 {
        let m = Arc::clone(&manager);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _ = m.get_paired_devices();
                let _ = m.get_connected_devices();
                let _ = m.is_running();
            }
        }));
    }

    // 并发写入
    for i in 0..5 {
        let m = Arc::clone(&manager);
        handles.push(thread::spawn(move || {
            for j in 0..20 {
                let device = DeviceInfo::new(format!("new-{}-{}", i, j), format!("New Device {}", j));
                m.add_paired_device(device);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // 验证没有崩溃，数据一致
    assert!(manager.get_paired_devices().len() >= 10);
}

// ============================================================
// 完整工作流测试
// ============================================================

#[tokio::test]
async fn test_complete_workflow() {
    // 完整工作流测试（不包括实际网络连接，因为需要 mDNS 发现）
    let config = NearClipConfig::new("MacBook Pro")
        .with_wifi_enabled(true)
        .with_ble_enabled(true);
    let callback = Arc::new(TestCallback::new());
    let manager = NearClipManager::new(config, callback.clone()).unwrap();

    // 1. 初始状态
    assert!(!manager.is_running());
    assert!(manager.get_paired_devices().is_empty());

    // 2. 启动服务
    manager.start().await.unwrap();
    assert!(manager.is_running());

    // 3. 添加配对设备
    let device = DeviceInfo::new("iphone-123", "iPhone 15")
        .with_platform(DevicePlatform::Unknown); // iOS 不在枚举中
    manager.add_paired_device(device);
    assert_eq!(manager.get_paired_devices().len(), 1);

    // 4. 连接设备 - 未在网络上发现，应返回错误
    let connect_result = manager.connect_device("iphone-123").await;
    assert!(connect_result.is_err());
    assert!(matches!(connect_result, Err(NearClipError::Network(_))));

    // 5. 模拟设备已连接（通过 handle_device_connected）
    let connected_device = DeviceInfo::new("iphone-123", "iPhone 15")
        .with_platform(DevicePlatform::Unknown)
        .with_status(DeviceStatus::Connected);
    manager.handle_device_connected(connected_device);
    assert_eq!(manager.get_connected_devices().len(), 1);
    assert_eq!(callback.connected_ids().len(), 1);

    // 6. 同步剪贴板
    manager.sync_clipboard(b"Hello from MacBook").await.unwrap();

    // 7. 接收远程剪贴板
    manager.handle_clipboard_received(b"Hello from iPhone", "iphone-123");
    assert_eq!(callback.received_clipboard().len(), 1);

    // 8. 断开设备
    manager.disconnect_device("iphone-123").await.unwrap();
    assert_eq!(manager.get_connected_devices().len(), 0);
    assert_eq!(callback.disconnected_ids().len(), 1);

    // 9. 停止服务
    manager.stop().await;
    assert!(!manager.is_running());
}
