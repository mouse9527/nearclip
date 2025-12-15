//! 通道监测集成测试
//!
//! 这些测试验证通道状态监测的完整流程。
//!
//! 主要测试:
//! - 监测器创建和配置
//! - 状态变更检测
//! - 回调触发
//! - 并发状态更新

use nearclip_sync::{
    Channel, ChannelMonitor, ChannelMonitorConfig, ChannelSnapshot, ChannelStatus,
    ChannelStatusCallback, SyncError, DEFAULT_CHECK_INTERVAL_SECS, DEFAULT_STATUS_TIMEOUT_SECS,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    changes: Arc<Mutex<Vec<(Channel, ChannelStatus, ChannelStatus)>>>,
    all_unavailable_count: AtomicUsize,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            changes: Arc::new(Mutex::new(Vec::new())),
            all_unavailable_count: AtomicUsize::new(0),
        }
    }

    fn change_count(&self) -> usize {
        self.changes.lock().unwrap().len()
    }

    fn all_unavailable_count(&self) -> usize {
        self.all_unavailable_count.load(Ordering::Relaxed)
    }

    fn get_changes(&self) -> Vec<(Channel, ChannelStatus, ChannelStatus)> {
        self.changes.lock().unwrap().clone()
    }

    fn clear(&self) {
        self.changes.lock().unwrap().clear();
        self.all_unavailable_count.store(0, Ordering::Relaxed);
    }
}

impl ChannelStatusCallback for TestCallback {
    fn on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus) {
        self.changes.lock().unwrap().push((channel, old, new));
    }

    fn on_all_channels_unavailable(&self) {
        self.all_unavailable_count.fetch_add(1, Ordering::Relaxed);
    }
}

// ============================================================
// 配置测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = ChannelMonitorConfig::new();

    assert_eq!(
        config.check_interval,
        Duration::from_secs(DEFAULT_CHECK_INTERVAL_SECS)
    );
    assert_eq!(
        config.timeout,
        Duration::from_secs(DEFAULT_STATUS_TIMEOUT_SECS)
    );
}

#[test]
fn test_config_builder_chain() {
    let config = ChannelMonitorConfig::new()
        .with_check_interval(Duration::from_secs(10))
        .with_timeout(Duration::from_secs(60));

    assert_eq!(config.check_interval, Duration::from_secs(10));
    assert_eq!(config.timeout, Duration::from_secs(60));
}

#[test]
fn test_config_validation_success() {
    let config = ChannelMonitorConfig::new();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_zero_interval() {
    let config = ChannelMonitorConfig::new().with_check_interval(Duration::ZERO);
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(SyncError::Configuration(_))));
}

#[test]
fn test_config_validation_zero_timeout() {
    let config = ChannelMonitorConfig::new().with_timeout(Duration::ZERO);
    let result = config.validate();
    assert!(result.is_err());
}

// ============================================================
// 监测器创建测试
// ============================================================

#[test]
fn test_monitor_creation_success() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();

    let monitor = ChannelMonitor::new(config, callback);
    assert!(monitor.is_ok());
}

#[test]
fn test_monitor_creation_failure_invalid_config() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new().with_check_interval(Duration::ZERO);

    let monitor = ChannelMonitor::new(config, callback);
    assert!(monitor.is_err());
}

#[test]
fn test_monitor_config_accessor() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new().with_check_interval(Duration::from_secs(15));

    let monitor = ChannelMonitor::new(config, callback).unwrap();

    assert_eq!(monitor.config().check_interval, Duration::from_secs(15));
}

// ============================================================
// 状态变更测试
// ============================================================

#[test]
fn test_initial_state_unavailable() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    // Both channels should be Unavailable initially
    assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Unavailable);
    assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Unavailable);
}

#[test]
fn test_status_update_triggers_callback() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Update WiFi to Available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);

    // Verify callback was triggered
    assert_eq!(callback.change_count(), 1);

    let changes = callback.get_changes();
    assert_eq!(changes[0].0, Channel::Wifi);
    assert_eq!(changes[0].1, ChannelStatus::Unavailable);
    assert_eq!(changes[0].2, ChannelStatus::Available);
}

#[test]
fn test_same_status_no_callback() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Update to same status (Unavailable -> Unavailable)
    monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);

    // No callback should be triggered
    assert_eq!(callback.change_count(), 0);
}

#[test]
fn test_multiple_status_changes() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Multiple state transitions
    monitor.update_status(Channel::Wifi, ChannelStatus::Connecting);
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    monitor.update_status(Channel::Wifi, ChannelStatus::Busy);
    monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);

    assert_eq!(callback.change_count(), 4);

    let changes = callback.get_changes();
    assert_eq!(
        changes[0],
        (
            Channel::Wifi,
            ChannelStatus::Unavailable,
            ChannelStatus::Connecting
        )
    );
    assert_eq!(
        changes[1],
        (
            Channel::Wifi,
            ChannelStatus::Connecting,
            ChannelStatus::Available
        )
    );
    assert_eq!(
        changes[2],
        (
            Channel::Wifi,
            ChannelStatus::Available,
            ChannelStatus::Busy
        )
    );
    assert_eq!(
        changes[3],
        (
            Channel::Wifi,
            ChannelStatus::Busy,
            ChannelStatus::Unavailable
        )
    );
}

// ============================================================
// 全部不可用回调测试
// ============================================================

#[test]
fn test_all_channels_unavailable_callback() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Make WiFi available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    assert_eq!(callback.all_unavailable_count(), 0);

    // Make WiFi unavailable -> all channels unavailable
    monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);
    assert_eq!(callback.all_unavailable_count(), 1);
}

#[test]
fn test_all_unavailable_not_triggered_when_one_available() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Make both available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    monitor.update_status(Channel::Ble, ChannelStatus::Available);

    // Make WiFi unavailable, but BLE still available
    monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);
    assert_eq!(callback.all_unavailable_count(), 0);

    // Now make BLE unavailable too
    monitor.update_status(Channel::Ble, ChannelStatus::Unavailable);
    assert_eq!(callback.all_unavailable_count(), 1);
}

// ============================================================
// 快照测试
// ============================================================

#[test]
fn test_snapshot_initial() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    let snapshot = monitor.snapshot();
    assert_eq!(snapshot.wifi_status, ChannelStatus::Unavailable);
    assert_eq!(snapshot.ble_status, ChannelStatus::Unavailable);
    assert!(snapshot.timestamp > 0);
}

#[test]
fn test_snapshot_after_updates() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    monitor.update_status(Channel::Ble, ChannelStatus::Connecting);

    let snapshot = monitor.snapshot();
    assert_eq!(snapshot.wifi_status, ChannelStatus::Available);
    assert_eq!(snapshot.ble_status, ChannelStatus::Connecting);
}

#[test]
fn test_snapshot_has_available_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    // Initially no available channel
    let snapshot = monitor.snapshot();
    assert!(!snapshot.has_available_channel());

    // Make WiFi available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    let snapshot = monitor.snapshot();
    assert!(snapshot.has_available_channel());
}

#[test]
fn test_snapshot_preferred_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    // No available channel
    let snapshot = monitor.snapshot();
    assert_eq!(snapshot.preferred_channel(), None);

    // Only BLE available
    monitor.update_status(Channel::Ble, ChannelStatus::Available);
    let snapshot = monitor.snapshot();
    assert_eq!(snapshot.preferred_channel(), Some(Channel::Ble));

    // Both available - prefers WiFi
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    let snapshot = monitor.snapshot();
    assert_eq!(snapshot.preferred_channel(), Some(Channel::Wifi));
}

// ============================================================
// 批量更新测试
// ============================================================

#[test]
fn test_batch_update() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    monitor.update_statuses(&[
        (Channel::Wifi, ChannelStatus::Available),
        (Channel::Ble, ChannelStatus::Connecting),
    ]);

    assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Available);
    assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Connecting);
    assert_eq!(callback.change_count(), 2);
}

// ============================================================
// 重置测试
// ============================================================

#[test]
fn test_reset() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Make both available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    monitor.update_status(Channel::Ble, ChannelStatus::Available);
    callback.clear();

    // Reset
    monitor.reset();

    // Both should be unavailable
    assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Unavailable);
    assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Unavailable);

    // Callbacks should have been triggered
    assert_eq!(callback.change_count(), 2);
    assert_eq!(callback.all_unavailable_count(), 1);
}

#[test]
fn test_reset_when_already_unavailable() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Initial state is already unavailable
    monitor.reset();

    // No callbacks should be triggered (status unchanged)
    assert_eq!(callback.change_count(), 0);
    assert_eq!(callback.all_unavailable_count(), 0);
}

// ============================================================
// 启动/停止测试
// ============================================================

#[test]
fn test_start_stop() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    assert!(!monitor.is_running());

    monitor.start();
    assert!(monitor.is_running());

    monitor.stop();
    assert!(!monitor.is_running());
}

#[test]
fn test_status_updates_work_when_not_running() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

    // Not running, but updates should still work
    assert!(!monitor.is_running());
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);

    assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Available);
    assert_eq!(callback.change_count(), 1);
}

// ============================================================
// 并发测试
// ============================================================

#[test]
fn test_concurrent_status_reads() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = Arc::new(ChannelMonitor::new(config, callback).unwrap());

    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&monitor);
        handles.push(std::thread::spawn(move || {
            m.get_status(Channel::Wifi);
            m.get_status(Channel::Ble);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_status_updates() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = Arc::new(ChannelMonitor::new(config, callback).unwrap());

    let mut handles = vec![];

    // Multiple threads updating different channels
    for i in 0..5 {
        let m = Arc::clone(&monitor);
        let channel = if i % 2 == 0 {
            Channel::Wifi
        } else {
            Channel::Ble
        };
        handles.push(std::thread::spawn(move || {
            m.update_status(channel, ChannelStatus::Available);
            std::thread::sleep(Duration::from_millis(1));
            m.update_status(channel, ChannelStatus::Unavailable);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final state should be consistent (both unavailable due to final updates)
    // Note: exact state depends on thread scheduling, just verify no panic
}

#[test]
fn test_concurrent_snapshot() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = Arc::new(ChannelMonitor::new(config, callback).unwrap());

    let mut handles = vec![];

    // One thread updating, others reading snapshots
    let m = Arc::clone(&monitor);
    handles.push(std::thread::spawn(move || {
        for _ in 0..10 {
            m.update_status(Channel::Wifi, ChannelStatus::Available);
            m.update_status(Channel::Wifi, ChannelStatus::Unavailable);
        }
    }));

    for _ in 0..5 {
        let m = Arc::clone(&monitor);
        handles.push(std::thread::spawn(move || {
            for _ in 0..10 {
                let _snapshot = m.snapshot();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ============================================================
// has_available_channel 测试
// ============================================================

#[test]
fn test_has_available_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelMonitorConfig::new();
    let monitor = ChannelMonitor::new(config, callback).unwrap();

    // Initially no available channel
    assert!(!monitor.has_available_channel());

    // Make WiFi available
    monitor.update_status(Channel::Wifi, ChannelStatus::Available);
    assert!(monitor.has_available_channel());

    // Make WiFi unavailable but BLE available
    monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);
    monitor.update_status(Channel::Ble, ChannelStatus::Available);
    assert!(monitor.has_available_channel());

    // Make both unavailable
    monitor.update_status(Channel::Ble, ChannelStatus::Unavailable);
    assert!(!monitor.has_available_channel());
}

// ============================================================
// ChannelSnapshot 测试
// ============================================================

#[test]
fn test_channel_snapshot_equality() {
    let s1 = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);
    let s2 = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);

    // Timestamps will differ, but we can compare fields
    assert_eq!(s1.wifi_status, s2.wifi_status);
    assert_eq!(s1.ble_status, s2.ble_status);
}
