//! 通道状态监测
//!
//! 实现通道状态的监测和变更通知逻辑。
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{
//!     ChannelMonitor, ChannelMonitorConfig, ChannelStatusCallback,
//!     Channel, ChannelStatus,
//! };
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl ChannelStatusCallback for MyCallback {
//!     fn on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus) {
//!         println!("{} changed from {} to {}", channel, old, new);
//!     }
//!     fn on_all_channels_unavailable(&self) {
//!         println!("All channels are unavailable!");
//!     }
//! }
//!
//! let callback = Arc::new(MyCallback);
//! let config = ChannelMonitorConfig::new();
//! let monitor = ChannelMonitor::new(config, callback).unwrap();
//!
//! // Update channel status
//! monitor.update_status(Channel::Wifi, ChannelStatus::Available);
//! ```

use crate::channel::{Channel, ChannelStatus};
use crate::sender::SyncError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// 默认检查间隔（5 秒）
pub const DEFAULT_CHECK_INTERVAL_SECS: u64 = 5;

/// 默认状态超时时间（30 秒）
pub const DEFAULT_STATUS_TIMEOUT_SECS: u64 = 30;

/// 通道状态变更回调
///
/// 接收通道状态变更的通知。
pub trait ChannelStatusCallback: Send + Sync {
    /// 状态变更通知
    ///
    /// 当通道状态发生变化时调用。
    ///
    /// # Arguments
    ///
    /// * `channel` - 发生变化的通道
    /// * `old_status` - 旧状态
    /// * `new_status` - 新状态
    fn on_status_changed(&self, channel: Channel, old_status: ChannelStatus, new_status: ChannelStatus);

    /// 所有通道不可用通知
    ///
    /// 当所有通道都变为不可用时调用。
    fn on_all_channels_unavailable(&self);
}

/// 通道监测器配置
///
/// 配置监测器的检查间隔和超时参数。
///
/// # Example
///
/// ```
/// use nearclip_sync::ChannelMonitorConfig;
/// use std::time::Duration;
///
/// let config = ChannelMonitorConfig::new()
///     .with_check_interval(Duration::from_secs(10))
///     .with_timeout(Duration::from_secs(60));
/// ```
#[derive(Debug, Clone)]
pub struct ChannelMonitorConfig {
    /// 检查间隔（供未来主动轮询使用）
    pub check_interval: Duration,
    /// 状态超时时间
    pub timeout: Duration,
}

impl ChannelMonitorConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            check_interval: Duration::from_secs(DEFAULT_CHECK_INTERVAL_SECS),
            timeout: Duration::from_secs(DEFAULT_STATUS_TIMEOUT_SECS),
        }
    }

    /// 设置检查间隔
    pub fn with_check_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }

    /// 设置状态超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), SyncError> {
        if self.check_interval.is_zero() {
            return Err(SyncError::Configuration(
                "Check interval must be greater than zero".to_string(),
            ));
        }

        if self.timeout.is_zero() {
            return Err(SyncError::Configuration(
                "Timeout must be greater than zero".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ChannelMonitorConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 通道状态快照
///
/// 包含所有通道在某一时刻的状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelSnapshot {
    /// WiFi 通道状态
    pub wifi_status: ChannelStatus,
    /// BLE 通道状态
    pub ble_status: ChannelStatus,
    /// 快照时间戳（毫秒）
    pub timestamp: u64,
}

impl ChannelSnapshot {
    /// 创建新的快照
    pub fn new(wifi_status: ChannelStatus, ble_status: ChannelStatus) -> Self {
        Self {
            wifi_status,
            ble_status,
            timestamp: Self::now_ms(),
        }
    }

    /// 获取当前时间戳（毫秒）
    fn now_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    /// 检查是否有任何可用通道
    pub fn has_available_channel(&self) -> bool {
        self.wifi_status == ChannelStatus::Available || self.ble_status == ChannelStatus::Available
    }

    /// 获取优先通道
    ///
    /// 如果 WiFi 可用则返回 WiFi，否则如果 BLE 可用则返回 BLE，否则返回 None。
    pub fn preferred_channel(&self) -> Option<Channel> {
        if self.wifi_status == ChannelStatus::Available {
            Some(Channel::Wifi)
        } else if self.ble_status == ChannelStatus::Available {
            Some(Channel::Ble)
        } else {
            None
        }
    }
}

/// 内部状态条目
#[derive(Debug, Clone)]
struct StatusEntry {
    status: ChannelStatus,
    #[allow(dead_code)]
    last_updated: u64,
}

impl StatusEntry {
    fn new(status: ChannelStatus) -> Self {
        Self {
            status,
            last_updated: ChannelSnapshot::now_ms(),
        }
    }
}

/// 通道监测器
///
/// 负责监测和管理通道状态，并在状态变更时触发回调。
///
/// # Example
///
/// ```
/// use nearclip_sync::{
///     ChannelMonitor, ChannelMonitorConfig, ChannelStatusCallback,
///     Channel, ChannelStatus,
/// };
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl ChannelStatusCallback for MyCallback {
///     fn on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus) {
///         println!("{}: {} -> {}", channel, old, new);
///     }
///     fn on_all_channels_unavailable(&self) {
///         println!("No channels available");
///     }
/// }
///
/// let config = ChannelMonitorConfig::new();
/// let callback = Arc::new(MyCallback);
/// let monitor = ChannelMonitor::new(config, callback).unwrap();
///
/// // 初始状态都是 Unavailable
/// let snapshot = monitor.snapshot();
/// assert_eq!(snapshot.wifi_status, ChannelStatus::Unavailable);
///
/// // 更新 WiFi 状态
/// monitor.update_status(Channel::Wifi, ChannelStatus::Available);
/// let snapshot = monitor.snapshot();
/// assert_eq!(snapshot.wifi_status, ChannelStatus::Available);
/// ```
pub struct ChannelMonitor {
    config: ChannelMonitorConfig,
    callback: Arc<dyn ChannelStatusCallback>,
    states: RwLock<HashMap<Channel, StatusEntry>>,
    running: RwLock<bool>,
}

impl ChannelMonitor {
    /// 创建新的监测器
    ///
    /// # Arguments
    ///
    /// * `config` - 监测器配置
    /// * `callback` - 状态变更回调
    ///
    /// # Returns
    ///
    /// 新的监测器实例，或配置错误
    pub fn new(
        config: ChannelMonitorConfig,
        callback: Arc<dyn ChannelStatusCallback>,
    ) -> Result<Self, SyncError> {
        config.validate()?;

        let mut initial_states = HashMap::new();
        initial_states.insert(Channel::Wifi, StatusEntry::new(ChannelStatus::Unavailable));
        initial_states.insert(Channel::Ble, StatusEntry::new(ChannelStatus::Unavailable));

        Ok(Self {
            config,
            callback,
            states: RwLock::new(initial_states),
            running: RwLock::new(false),
        })
    }

    /// 获取配置引用
    pub fn config(&self) -> &ChannelMonitorConfig {
        &self.config
    }

    /// 启动监测
    ///
    /// 标记监测器为运行状态。当前实现为被动模式，
    /// 实际状态更新由外部调用 `update_status()` 完成。
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }

    /// 停止监测
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        *self.running.read().unwrap()
    }

    /// 获取通道当前状态
    pub fn get_status(&self, channel: Channel) -> ChannelStatus {
        self.states
            .read()
            .unwrap()
            .get(&channel)
            .map(|e| e.status)
            .unwrap_or(ChannelStatus::Unavailable)
    }

    /// 更新通道状态
    ///
    /// 如果状态发生变化，会触发回调通知。
    ///
    /// # Arguments
    ///
    /// * `channel` - 要更新的通道
    /// * `new_status` - 新状态
    pub fn update_status(&self, channel: Channel, new_status: ChannelStatus) {
        let old_status = {
            let mut states = self.states.write().unwrap();
            let entry = states.entry(channel).or_insert_with(|| StatusEntry::new(ChannelStatus::Unavailable));
            let old = entry.status;
            entry.status = new_status;
            entry.last_updated = ChannelSnapshot::now_ms();
            old
        };

        // 状态变更时触发回调
        if old_status != new_status {
            self.callback.on_status_changed(channel, old_status, new_status);

            // 检查是否所有通道都不可用
            if !self.has_available_channel() {
                self.callback.on_all_channels_unavailable();
            }
        }
    }

    /// 批量更新通道状态
    ///
    /// # Arguments
    ///
    /// * `updates` - 通道和新状态的列表
    pub fn update_statuses(&self, updates: &[(Channel, ChannelStatus)]) {
        for (channel, status) in updates {
            self.update_status(*channel, *status);
        }
    }

    /// 获取状态快照
    ///
    /// 返回所有通道的当前状态。
    pub fn snapshot(&self) -> ChannelSnapshot {
        let states = self.states.read().unwrap();
        ChannelSnapshot::new(
            states.get(&Channel::Wifi).map(|e| e.status).unwrap_or(ChannelStatus::Unavailable),
            states.get(&Channel::Ble).map(|e| e.status).unwrap_or(ChannelStatus::Unavailable),
        )
    }

    /// 检查是否有任何可用通道
    pub fn has_available_channel(&self) -> bool {
        let states = self.states.read().unwrap();
        states.values().any(|e| e.status == ChannelStatus::Available)
    }

    /// 重置所有通道状态为不可用
    pub fn reset(&self) {
        let old_snapshot = self.snapshot();

        {
            let mut states = self.states.write().unwrap();
            for entry in states.values_mut() {
                entry.status = ChannelStatus::Unavailable;
                entry.last_updated = ChannelSnapshot::now_ms();
            }
        }

        // 触发状态变更回调
        if old_snapshot.wifi_status != ChannelStatus::Unavailable {
            self.callback.on_status_changed(Channel::Wifi, old_snapshot.wifi_status, ChannelStatus::Unavailable);
        }
        if old_snapshot.ble_status != ChannelStatus::Unavailable {
            self.callback.on_status_changed(Channel::Ble, old_snapshot.ble_status, ChannelStatus::Unavailable);
        }

        // 如果之前有可用通道，现在触发全部不可用回调
        if old_snapshot.has_available_channel() {
            self.callback.on_all_channels_unavailable();
        }
    }
}

impl std::fmt::Debug for ChannelMonitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelMonitor")
            .field("config", &self.config)
            .field("running", &*self.running.read().unwrap())
            .field("snapshot", &self.snapshot())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    struct TestCallback {
        changes: Mutex<Vec<(Channel, ChannelStatus, ChannelStatus)>>,
        all_unavailable_count: AtomicUsize,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                changes: Mutex::new(Vec::new()),
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
    }

    impl ChannelStatusCallback for TestCallback {
        fn on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus) {
            self.changes.lock().unwrap().push((channel, old, new));
        }

        fn on_all_channels_unavailable(&self) {
            self.all_unavailable_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Config tests
    #[test]
    fn test_config_new() {
        let config = ChannelMonitorConfig::new();
        assert_eq!(config.check_interval, Duration::from_secs(DEFAULT_CHECK_INTERVAL_SECS));
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_STATUS_TIMEOUT_SECS));
    }

    #[test]
    fn test_config_builder() {
        let config = ChannelMonitorConfig::new()
            .with_check_interval(Duration::from_secs(10))
            .with_timeout(Duration::from_secs(60));

        assert_eq!(config.check_interval, Duration::from_secs(10));
        assert_eq!(config.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_config_validate_ok() {
        let config = ChannelMonitorConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_interval() {
        let config = ChannelMonitorConfig::new().with_check_interval(Duration::ZERO);
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::Configuration(_))));
    }

    #[test]
    fn test_config_validate_zero_timeout() {
        let config = ChannelMonitorConfig::new().with_timeout(Duration::ZERO);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_default() {
        let config = ChannelMonitorConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(DEFAULT_CHECK_INTERVAL_SECS));
    }

    #[test]
    fn test_config_clone() {
        let config = ChannelMonitorConfig::new().with_check_interval(Duration::from_secs(15));
        let cloned = config.clone();
        assert_eq!(config.check_interval, cloned.check_interval);
    }

    #[test]
    fn test_config_debug() {
        let config = ChannelMonitorConfig::new();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ChannelMonitorConfig"));
    }

    // Snapshot tests
    #[test]
    fn test_snapshot_new() {
        let snapshot = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);
        assert_eq!(snapshot.wifi_status, ChannelStatus::Available);
        assert_eq!(snapshot.ble_status, ChannelStatus::Unavailable);
        assert!(snapshot.timestamp > 0);
    }

    #[test]
    fn test_snapshot_has_available_channel() {
        let both_unavailable = ChannelSnapshot::new(ChannelStatus::Unavailable, ChannelStatus::Unavailable);
        assert!(!both_unavailable.has_available_channel());

        let wifi_available = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);
        assert!(wifi_available.has_available_channel());

        let ble_available = ChannelSnapshot::new(ChannelStatus::Unavailable, ChannelStatus::Available);
        assert!(ble_available.has_available_channel());

        let both_available = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Available);
        assert!(both_available.has_available_channel());
    }

    #[test]
    fn test_snapshot_preferred_channel() {
        // Both unavailable
        let snapshot = ChannelSnapshot::new(ChannelStatus::Unavailable, ChannelStatus::Unavailable);
        assert_eq!(snapshot.preferred_channel(), None);

        // WiFi available - prefers WiFi
        let snapshot = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Available);
        assert_eq!(snapshot.preferred_channel(), Some(Channel::Wifi));

        // Only BLE available
        let snapshot = ChannelSnapshot::new(ChannelStatus::Unavailable, ChannelStatus::Available);
        assert_eq!(snapshot.preferred_channel(), Some(Channel::Ble));
    }

    #[test]
    fn test_snapshot_clone_eq() {
        let snapshot1 = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);
        let snapshot2 = snapshot1.clone();
        // Note: timestamps will differ slightly, so we compare fields
        assert_eq!(snapshot1.wifi_status, snapshot2.wifi_status);
        assert_eq!(snapshot1.ble_status, snapshot2.ble_status);
    }

    #[test]
    fn test_snapshot_debug() {
        let snapshot = ChannelSnapshot::new(ChannelStatus::Available, ChannelStatus::Unavailable);
        let debug_str = format!("{:?}", snapshot);
        assert!(debug_str.contains("ChannelSnapshot"));
    }

    // Monitor tests
    #[test]
    fn test_monitor_new_valid() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback);
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_monitor_new_invalid_config() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new().with_check_interval(Duration::ZERO);
        let monitor = ChannelMonitor::new(config, callback);
        assert!(monitor.is_err());
    }

    #[test]
    fn test_monitor_initial_state() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback).unwrap();

        // Both channels should be Unavailable initially
        assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Unavailable);
        assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Unavailable);
    }

    #[test]
    fn test_monitor_start_stop() {
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
    fn test_monitor_update_status() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Update WiFi to Available
        monitor.update_status(Channel::Wifi, ChannelStatus::Available);
        assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Available);

        // Callback should have been triggered
        assert_eq!(callback.change_count(), 1);

        let changes = callback.get_changes();
        assert_eq!(changes[0].0, Channel::Wifi);
        assert_eq!(changes[0].1, ChannelStatus::Unavailable);
        assert_eq!(changes[0].2, ChannelStatus::Available);
    }

    #[test]
    fn test_monitor_update_same_status_no_callback() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Initial status is Unavailable, update to Unavailable
        monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);

        // No callback should be triggered (status unchanged)
        assert_eq!(callback.change_count(), 0);
    }

    #[test]
    fn test_monitor_all_channels_unavailable_callback() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Make WiFi available
        monitor.update_status(Channel::Wifi, ChannelStatus::Available);
        assert_eq!(callback.all_unavailable_count(), 0);

        // Make WiFi unavailable again
        monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);
        assert_eq!(callback.all_unavailable_count(), 1);
    }

    #[test]
    fn test_monitor_snapshot() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback).unwrap();

        // Initial snapshot
        let snapshot = monitor.snapshot();
        assert_eq!(snapshot.wifi_status, ChannelStatus::Unavailable);
        assert_eq!(snapshot.ble_status, ChannelStatus::Unavailable);

        // Update and check
        monitor.update_status(Channel::Wifi, ChannelStatus::Available);
        monitor.update_status(Channel::Ble, ChannelStatus::Connecting);

        let snapshot = monitor.snapshot();
        assert_eq!(snapshot.wifi_status, ChannelStatus::Available);
        assert_eq!(snapshot.ble_status, ChannelStatus::Connecting);
    }

    #[test]
    fn test_monitor_has_available_channel() {
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
    }

    #[test]
    fn test_monitor_update_statuses_batch() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Batch update
        monitor.update_statuses(&[
            (Channel::Wifi, ChannelStatus::Available),
            (Channel::Ble, ChannelStatus::Connecting),
        ]);

        assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Available);
        assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Connecting);
        assert_eq!(callback.change_count(), 2);
    }

    #[test]
    fn test_monitor_reset() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Make both available
        monitor.update_status(Channel::Wifi, ChannelStatus::Available);
        monitor.update_status(Channel::Ble, ChannelStatus::Available);
        let initial_changes = callback.change_count();

        // Reset
        monitor.reset();

        // Both should be unavailable
        assert_eq!(monitor.get_status(Channel::Wifi), ChannelStatus::Unavailable);
        assert_eq!(monitor.get_status(Channel::Ble), ChannelStatus::Unavailable);

        // Callbacks should have been triggered for both
        assert_eq!(callback.change_count(), initial_changes + 2);

        // All unavailable callback should have been triggered
        assert!(callback.all_unavailable_count() > 0);
    }

    #[test]
    fn test_monitor_config_accessor() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new().with_check_interval(Duration::from_secs(15));
        let monitor = ChannelMonitor::new(config, callback).unwrap();

        assert_eq!(monitor.config().check_interval, Duration::from_secs(15));
    }

    #[test]
    fn test_monitor_debug() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback).unwrap();

        let debug_str = format!("{:?}", monitor);
        assert!(debug_str.contains("ChannelMonitor"));
    }

    #[test]
    fn test_monitor_status_transitions() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelMonitorConfig::new();
        let monitor = ChannelMonitor::new(config, callback.clone()).unwrap();

        // Unavailable -> Connecting -> Available -> Busy -> Unavailable
        monitor.update_status(Channel::Wifi, ChannelStatus::Connecting);
        monitor.update_status(Channel::Wifi, ChannelStatus::Available);
        monitor.update_status(Channel::Wifi, ChannelStatus::Busy);
        monitor.update_status(Channel::Wifi, ChannelStatus::Unavailable);

        let changes = callback.get_changes();
        assert_eq!(changes.len(), 4);

        assert_eq!(changes[0], (Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Connecting));
        assert_eq!(changes[1], (Channel::Wifi, ChannelStatus::Connecting, ChannelStatus::Available));
        assert_eq!(changes[2], (Channel::Wifi, ChannelStatus::Available, ChannelStatus::Busy));
        assert_eq!(changes[3], (Channel::Wifi, ChannelStatus::Busy, ChannelStatus::Unavailable));
    }
}
