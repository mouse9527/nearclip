//! 通道自动切换集成测试
//!
//! 这些测试验证通道自动切换的完整流程。
//!
//! 主要测试:
//! - 切换器创建和配置
//! - WiFi -> BLE 自动降级
//! - BLE -> WiFi 自动恢复
//! - 手动切换
//! - 切换回调触发

use nearclip_sync::{
    Channel, ChannelStatus, ChannelSwitchCallback, ChannelSwitcher, ChannelSwitcherConfig,
    PrioritySwitchStrategy, StickySwitchStrategy, SwitchReason, SwitchStrategy, SyncError,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    switches: Arc<Mutex<Vec<(Option<Channel>, Option<Channel>, SwitchReason)>>>,
    switch_count: AtomicUsize,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            switches: Arc::new(Mutex::new(Vec::new())),
            switch_count: AtomicUsize::new(0),
        }
    }

    fn switch_count(&self) -> usize {
        self.switch_count.load(Ordering::Relaxed)
    }

    fn get_switches(&self) -> Vec<(Option<Channel>, Option<Channel>, SwitchReason)> {
        self.switches.lock().unwrap().clone()
    }

    fn clear(&self) {
        self.switches.lock().unwrap().clear();
        self.switch_count.store(0, Ordering::Relaxed);
    }
}

impl ChannelSwitchCallback for TestCallback {
    fn on_channel_switched(
        &self,
        from: Option<Channel>,
        to: Option<Channel>,
        reason: SwitchReason,
    ) {
        self.switches.lock().unwrap().push((from, to, reason));
        self.switch_count.fetch_add(1, Ordering::Relaxed);
    }
}

// ============================================================
// 配置测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = ChannelSwitcherConfig::new();

    assert_eq!(config.preferred_channel, Channel::Wifi);
    assert!(config.auto_fallback);
    assert!(config.auto_recovery);
}

#[test]
fn test_config_builder_chain() {
    let config = ChannelSwitcherConfig::new()
        .with_preferred_channel(Channel::Ble)
        .with_auto_fallback(false)
        .with_auto_recovery(false);

    assert_eq!(config.preferred_channel, Channel::Ble);
    assert!(!config.auto_fallback);
    assert!(!config.auto_recovery);
}

#[test]
fn test_config_fallback_channel() {
    let config_wifi = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Wifi);
    assert_eq!(config_wifi.fallback_channel(), Channel::Ble);

    let config_ble = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);
    assert_eq!(config_ble.fallback_channel(), Channel::Wifi);
}

#[test]
fn test_config_validation_success() {
    let config = ChannelSwitcherConfig::new();
    assert!(config.validate().is_ok());
}

// ============================================================
// 切换器创建测试
// ============================================================

#[test]
fn test_switcher_creation_success() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();

    let switcher = ChannelSwitcher::new(config, callback);
    assert!(switcher.is_ok());
}

#[test]
fn test_switcher_initial_state() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback).unwrap();

    assert_eq!(switcher.current_channel(), None);
    assert_eq!(switcher.get_status(Channel::Wifi), ChannelStatus::Unavailable);
    assert_eq!(switcher.get_status(Channel::Ble), ChannelStatus::Unavailable);
    assert!(!switcher.has_available_channel());
}

#[test]
fn test_switcher_config_accessor() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);

    let switcher = ChannelSwitcher::new(config, callback).unwrap();
    assert_eq!(switcher.config().preferred_channel, Channel::Ble);
}

// ============================================================
// WiFi -> BLE 自动降级测试
// ============================================================

#[test]
fn test_wifi_to_ble_fallback() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_auto_fallback(true);
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // WiFi 先变为可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    // BLE 也变为可用
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
    // 仍然使用 WiFi (首选)
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    callback.clear();

    // WiFi 变为不可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

    // 应该自动切换到 BLE
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));
    assert_eq!(callback.switch_count(), 1);

    let switches = callback.get_switches();
    assert_eq!(switches[0].0, Some(Channel::Wifi));
    assert_eq!(switches[0].1, Some(Channel::Ble));
    assert_eq!(switches[0].2, SwitchReason::Unavailable);
}

#[test]
fn test_no_fallback_when_disabled() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_auto_fallback(false);
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 两个通道都可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

    callback.clear();

    // WiFi 变为不可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

    // 应该切换到 None (不自动降级)
    assert_eq!(switcher.current_channel(), None);
    assert_eq!(callback.switch_count(), 1);

    let switches = callback.get_switches();
    assert_eq!(switches[0].1, None);
}

// ============================================================
// BLE -> WiFi 自动恢复测试
// ============================================================

#[test]
fn test_ble_to_wifi_recovery() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_auto_recovery(true);
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 只有 BLE 可用
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));

    callback.clear();

    // WiFi 变为可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

    // 应该自动恢复到 WiFi (更高优先级)
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
    assert_eq!(callback.switch_count(), 1);

    let switches = callback.get_switches();
    assert_eq!(switches[0].0, Some(Channel::Ble));
    assert_eq!(switches[0].1, Some(Channel::Wifi));
    assert_eq!(switches[0].2, SwitchReason::HigherPriority);
}

#[test]
fn test_no_recovery_when_disabled() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_auto_recovery(false);
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 只有 BLE 可用
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));

    callback.clear();

    // WiFi 变为可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

    // 不应该自动恢复，保持 BLE
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));
    assert_eq!(callback.switch_count(), 0);
}

// ============================================================
// 手动切换测试
// ============================================================

#[test]
fn test_manual_switch_success() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 两个通道都可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

    callback.clear();

    // 手动切换到 BLE
    let result = switcher.switch_to(Channel::Ble);
    assert!(result.is_ok());
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));
    assert_eq!(callback.switch_count(), 1);

    let switches = callback.get_switches();
    assert_eq!(switches[0].2, SwitchReason::Manual);
}

#[test]
fn test_manual_switch_to_unavailable_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback).unwrap();

    // 只有 WiFi 可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

    // 尝试手动切换到不可用的 BLE
    let result = switcher.switch_to(Channel::Ble);
    assert!(result.is_err());
    assert!(matches!(result, Err(SyncError::ChannelUnavailable)));
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
}

#[test]
fn test_manual_switch_to_same_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // WiFi 可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    callback.clear();

    // 手动切换到已经是当前通道的 WiFi
    let result = switcher.switch_to(Channel::Wifi);
    assert!(result.is_ok());
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
    // 不应该触发回调 (没有变化)
    assert_eq!(callback.switch_count(), 0);
}

// ============================================================
// 所有通道不可用测试
// ============================================================

#[test]
fn test_all_channels_unavailable() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // WiFi 可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

    callback.clear();

    // WiFi 变为不可用 (BLE 从未可用)
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

    // 应该切换到 None
    assert_eq!(switcher.current_channel(), None);
    assert!(!switcher.has_available_channel());
    assert_eq!(callback.switch_count(), 1);

    let switches = callback.get_switches();
    assert_eq!(switches[0].1, None);
    assert_eq!(switches[0].2, SwitchReason::Unavailable);
}

// ============================================================
// 重置测试
// ============================================================

#[test]
fn test_reset() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // WiFi 可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    callback.clear();

    // 重置
    switcher.reset();

    assert_eq!(switcher.current_channel(), None);
    assert_eq!(callback.switch_count(), 1);
}

#[test]
fn test_reset_when_no_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 初始状态没有通道
    switcher.reset();

    // 不应该触发回调
    assert_eq!(callback.switch_count(), 0);
}

// ============================================================
// select_best_channel 测试
// ============================================================

#[test]
fn test_select_best_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = ChannelSwitcher::new(config, callback).unwrap();

    // 都不可用
    assert_eq!(switcher.select_best_channel(), None);

    // 只有 BLE 可用
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.select_best_channel(), Some(Channel::Ble));

    // 两个都可用 - WiFi 优先
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.select_best_channel(), Some(Channel::Wifi));
}

// ============================================================
// 切换策略测试
// ============================================================

#[test]
fn test_priority_switch_strategy() {
    let strategy = PrioritySwitchStrategy;

    // 两个都可用 - 选择 WiFi
    assert_eq!(strategy.select(true, true, None), Some(Channel::Wifi));
    assert_eq!(strategy.select(true, true, Some(Channel::Ble)), Some(Channel::Wifi));

    // 只有 BLE 可用
    assert_eq!(strategy.select(false, true, None), Some(Channel::Ble));

    // 只有 WiFi 可用
    assert_eq!(strategy.select(true, false, None), Some(Channel::Wifi));

    // 都不可用
    assert_eq!(strategy.select(false, false, None), None);
}

#[test]
fn test_sticky_switch_strategy() {
    let strategy = StickySwitchStrategy;

    // 当前是 WiFi，两个都可用 - 保持 WiFi
    assert_eq!(strategy.select(true, true, Some(Channel::Wifi)), Some(Channel::Wifi));

    // 当前是 BLE，两个都可用 - 保持 BLE
    assert_eq!(strategy.select(true, true, Some(Channel::Ble)), Some(Channel::Ble));

    // 当前是 WiFi，只有 BLE 可用 - 切换到 BLE
    assert_eq!(strategy.select(false, true, Some(Channel::Wifi)), Some(Channel::Ble));

    // 没有当前通道，两个都可用 - 选择 WiFi
    assert_eq!(strategy.select(true, true, None), Some(Channel::Wifi));
}

// ============================================================
// 完整切换流程测试
// ============================================================

#[test]
fn test_complete_switching_flow() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new()
        .with_auto_fallback(true)
        .with_auto_recovery(true);
    let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

    // 1. 初始状态：无通道
    assert_eq!(switcher.current_channel(), None);

    // 2. WiFi 变为可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    // 3. BLE 也变为可用 (不切换)
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    // 4. WiFi 不可用 -> 切换到 BLE
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);
    assert_eq!(switcher.current_channel(), Some(Channel::Ble));

    // 5. WiFi 恢复 -> 自动切换回 WiFi
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    assert_eq!(switcher.current_channel(), Some(Channel::Wifi));

    // 6. 两个都不可用 -> None
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Available, ChannelStatus::Unavailable);
    assert_eq!(switcher.current_channel(), None);

    // 验证回调次数
    // Initial(WiFi), Initial(BLE不触发), Unavailable(->BLE), HigherPriority(->WiFi),
    // Unavailable(->BLE因为BLE还可用), Unavailable(->None)
    // 实际：1(WiFi) + 0 + 1(->BLE) + 1(->WiFi) + 1(->BLE) + 1(->None) = 5
    let switches = callback.get_switches();
    assert!(switches.len() >= 4);
}

// ============================================================
// 并发测试
// ============================================================

#[test]
fn test_concurrent_status_updates() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = Arc::new(ChannelSwitcher::new(config, callback).unwrap());

    let mut handles = vec![];

    // 多个线程更新不同通道状态
    for i in 0..5 {
        let s = Arc::clone(&switcher);
        let channel = if i % 2 == 0 { Channel::Wifi } else { Channel::Ble };
        handles.push(std::thread::spawn(move || {
            s.handle_status_change(channel, ChannelStatus::Unavailable, ChannelStatus::Available);
            std::thread::sleep(std::time::Duration::from_millis(1));
            s.handle_status_change(channel, ChannelStatus::Available, ChannelStatus::Unavailable);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 验证没有 panic
}

#[test]
fn test_concurrent_reads() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new();
    let switcher = Arc::new(ChannelSwitcher::new(config, callback).unwrap());

    // 设置初始状态
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

    let mut handles = vec![];

    for _ in 0..10 {
        let s = Arc::clone(&switcher);
        handles.push(std::thread::spawn(move || {
            let _ = s.current_channel();
            let _ = s.get_status(Channel::Wifi);
            let _ = s.has_available_channel();
            let _ = s.select_best_channel();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ============================================================
// SwitchReason 测试
// ============================================================

#[test]
fn test_switch_reason_as_str() {
    assert_eq!(SwitchReason::Unavailable.as_str(), "unavailable");
    assert_eq!(SwitchReason::HigherPriority.as_str(), "higher_priority");
    assert_eq!(SwitchReason::Manual.as_str(), "manual");
    assert_eq!(SwitchReason::Initial.as_str(), "initial");
}

#[test]
fn test_switch_reason_display() {
    assert_eq!(format!("{}", SwitchReason::Unavailable), "unavailable");
    assert_eq!(format!("{}", SwitchReason::HigherPriority), "higher_priority");
}

// ============================================================
// BLE 优先配置测试
// ============================================================

#[test]
fn test_ble_preferred_channel() {
    let callback = Arc::new(TestCallback::new());
    let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);
    let switcher = ChannelSwitcher::new(config, callback).unwrap();

    // 两个都可用
    switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
    switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

    // 应该选择 BLE (因为配置为首选)
    assert_eq!(switcher.select_best_channel(), Some(Channel::Ble));
}
