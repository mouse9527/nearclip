//! 通道自动切换
//!
//! 实现通道的自动切换逻辑，当首选通道不可用时自动切换到备用通道。
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{
//!     ChannelSwitcher, ChannelSwitcherConfig, ChannelSwitchCallback,
//!     Channel, ChannelStatus, SwitchReason,
//! };
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl ChannelSwitchCallback for MyCallback {
//!     fn on_channel_switched(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason) {
//!         println!("Switched from {:?} to {:?} due to {:?}", from, to, reason);
//!     }
//! }
//!
//! let callback = Arc::new(MyCallback);
//! let config = ChannelSwitcherConfig::new();
//! let switcher = ChannelSwitcher::new(config, callback).unwrap();
//!
//! // Handle status changes
//! switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
//! assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
//! ```

use crate::channel::{Channel, ChannelStatus};
use crate::sender::SyncError;
use std::fmt;
use std::sync::{Arc, RwLock};

/// 切换原因
///
/// 描述通道切换发生的原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchReason {
    /// 当前通道不可用
    Unavailable,
    /// 更高优先级通道可用
    HigherPriority,
    /// 手动切换
    Manual,
    /// 初始选择
    Initial,
}

impl SwitchReason {
    /// 获取原因名称
    pub fn as_str(&self) -> &'static str {
        match self {
            SwitchReason::Unavailable => "unavailable",
            SwitchReason::HigherPriority => "higher_priority",
            SwitchReason::Manual => "manual",
            SwitchReason::Initial => "initial",
        }
    }
}

impl fmt::Display for SwitchReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 通道切换回调
///
/// 接收通道切换的通知。
pub trait ChannelSwitchCallback: Send + Sync {
    /// 通道切换通知
    ///
    /// 当活跃通道发生切换时调用。
    ///
    /// # Arguments
    ///
    /// * `from` - 切换前的通道 (None 表示无活跃通道)
    /// * `to` - 切换后的通道 (None 表示无可用通道)
    /// * `reason` - 切换原因
    fn on_channel_switched(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason);
}

/// 通道切换器配置
///
/// 配置切换器的行为。
///
/// # Example
///
/// ```
/// use nearclip_sync::{ChannelSwitcherConfig, Channel};
///
/// let config = ChannelSwitcherConfig::new()
///     .with_preferred_channel(Channel::Wifi)
///     .with_auto_fallback(true)
///     .with_auto_recovery(true);
/// ```
#[derive(Debug, Clone)]
pub struct ChannelSwitcherConfig {
    /// 首选通道 (默认 WiFi)
    pub preferred_channel: Channel,
    /// 是否自动降级到备用通道 (默认 true)
    pub auto_fallback: bool,
    /// 是否自动恢复到首选通道 (默认 true)
    pub auto_recovery: bool,
}

impl ChannelSwitcherConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            preferred_channel: Channel::Wifi,
            auto_fallback: true,
            auto_recovery: true,
        }
    }

    /// 设置首选通道
    pub fn with_preferred_channel(mut self, channel: Channel) -> Self {
        self.preferred_channel = channel;
        self
    }

    /// 设置是否自动降级
    pub fn with_auto_fallback(mut self, enabled: bool) -> Self {
        self.auto_fallback = enabled;
        self
    }

    /// 设置是否自动恢复
    pub fn with_auto_recovery(mut self, enabled: bool) -> Self {
        self.auto_recovery = enabled;
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), SyncError> {
        // 目前配置都是有效的
        Ok(())
    }

    /// 获取备用通道
    pub fn fallback_channel(&self) -> Channel {
        match self.preferred_channel {
            Channel::Wifi => Channel::Ble,
            Channel::Ble => Channel::Wifi,
        }
    }
}

impl Default for ChannelSwitcherConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 内部状态
#[derive(Debug, Clone)]
struct SwitcherState {
    current_channel: Option<Channel>,
    wifi_status: ChannelStatus,
    ble_status: ChannelStatus,
}

impl SwitcherState {
    fn new() -> Self {
        Self {
            current_channel: None,
            wifi_status: ChannelStatus::Unavailable,
            ble_status: ChannelStatus::Unavailable,
        }
    }

    fn get_status(&self, channel: Channel) -> ChannelStatus {
        match channel {
            Channel::Wifi => self.wifi_status,
            Channel::Ble => self.ble_status,
        }
    }

    fn set_status(&mut self, channel: Channel, status: ChannelStatus) {
        match channel {
            Channel::Wifi => self.wifi_status = status,
            Channel::Ble => self.ble_status = status,
        }
    }

    fn is_available(&self, channel: Channel) -> bool {
        self.get_status(channel) == ChannelStatus::Available
    }

    fn has_any_available(&self) -> bool {
        self.wifi_status == ChannelStatus::Available || self.ble_status == ChannelStatus::Available
    }
}

/// 通道切换器
///
/// 负责根据通道状态自动选择最佳通道。
///
/// # Example
///
/// ```
/// use nearclip_sync::{
///     ChannelSwitcher, ChannelSwitcherConfig, ChannelSwitchCallback,
///     Channel, ChannelStatus, SwitchReason,
/// };
/// use std::sync::Arc;
///
/// struct MyCallback;
/// impl ChannelSwitchCallback for MyCallback {
///     fn on_channel_switched(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason) {
///         println!("{:?} -> {:?}", from, to);
///     }
/// }
///
/// let config = ChannelSwitcherConfig::new();
/// let callback = Arc::new(MyCallback);
/// let switcher = ChannelSwitcher::new(config, callback).unwrap();
///
/// // Initially no channel available
/// assert_eq!(switcher.current_channel(), None);
///
/// // WiFi becomes available
/// switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
/// assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
///
/// // WiFi becomes unavailable, BLE available
/// switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
/// switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);
/// assert_eq!(switcher.current_channel(), Some(Channel::Ble));
/// ```
pub struct ChannelSwitcher {
    config: ChannelSwitcherConfig,
    state: RwLock<SwitcherState>,
    callback: Arc<dyn ChannelSwitchCallback>,
}

impl ChannelSwitcher {
    /// 创建新的切换器
    ///
    /// # Arguments
    ///
    /// * `config` - 切换器配置
    /// * `callback` - 切换回调
    ///
    /// # Returns
    ///
    /// 新的切换器实例，或配置错误
    pub fn new(
        config: ChannelSwitcherConfig,
        callback: Arc<dyn ChannelSwitchCallback>,
    ) -> Result<Self, SyncError> {
        config.validate()?;

        Ok(Self {
            config,
            state: RwLock::new(SwitcherState::new()),
            callback,
        })
    }

    /// 获取配置引用
    pub fn config(&self) -> &ChannelSwitcherConfig {
        &self.config
    }

    /// 获取当前活跃通道
    pub fn current_channel(&self) -> Option<Channel> {
        self.state.read().unwrap().current_channel
    }

    /// 获取通道状态
    pub fn get_status(&self, channel: Channel) -> ChannelStatus {
        self.state.read().unwrap().get_status(channel)
    }

    /// 检查是否有可用通道
    pub fn has_available_channel(&self) -> bool {
        self.state.read().unwrap().has_any_available()
    }

    /// 处理通道状态变更
    ///
    /// 根据状态变更决定是否需要切换通道。
    ///
    /// # Arguments
    ///
    /// * `channel` - 状态变更的通道
    /// * `old_status` - 旧状态
    /// * `new_status` - 新状态
    pub fn handle_status_change(
        &self,
        channel: Channel,
        _old_status: ChannelStatus,
        new_status: ChannelStatus,
    ) {
        let (should_switch, from, to, reason) = {
            let mut state = self.state.write().unwrap();
            state.set_status(channel, new_status);
            self.evaluate_switch(&state)
        };

        if should_switch {
            self.do_switch(from, to, reason);
        }
    }

    /// 手动切换到指定通道
    ///
    /// # Arguments
    ///
    /// * `channel` - 目标通道
    ///
    /// # Returns
    ///
    /// 如果通道可用则返回 Ok，否则返回错误
    pub fn switch_to(&self, channel: Channel) -> Result<(), SyncError> {
        let (from, is_available) = {
            let state = self.state.read().unwrap();
            (state.current_channel, state.is_available(channel))
        };

        if !is_available {
            return Err(SyncError::ChannelUnavailable);
        }

        if from != Some(channel) {
            self.do_switch(from, Some(channel), SwitchReason::Manual);
        }

        Ok(())
    }

    /// 选择最佳可用通道
    ///
    /// 根据配置的优先级选择最佳通道。
    pub fn select_best_channel(&self) -> Option<Channel> {
        let state = self.state.read().unwrap();
        self.find_best_channel(&state)
    }

    /// 重置切换器状态
    pub fn reset(&self) {
        let from = {
            let mut state = self.state.write().unwrap();
            let from = state.current_channel;
            *state = SwitcherState::new();
            from
        };

        if from.is_some() {
            self.callback.on_channel_switched(from, None, SwitchReason::Manual);
        }
    }

    /// 评估是否需要切换
    fn evaluate_switch(&self, state: &SwitcherState) -> (bool, Option<Channel>, Option<Channel>, SwitchReason) {
        let current = state.current_channel;
        let best = self.find_best_channel(state);

        // 没有变化
        if current == best {
            return (false, current, best, SwitchReason::Initial);
        }

        // 当前无通道，选择最佳
        if current.is_none() && best.is_some() {
            return (true, None, best, SwitchReason::Initial);
        }

        // 当前通道仍然可用
        if let Some(curr) = current {
            if state.is_available(curr) {
                // 检查是否需要自动恢复到首选通道
                if self.config.auto_recovery
                    && curr != self.config.preferred_channel
                    && state.is_available(self.config.preferred_channel)
                {
                    return (true, current, Some(self.config.preferred_channel), SwitchReason::HigherPriority);
                }
                // 当前通道仍可用，不切换
                return (false, current, current, SwitchReason::Initial);
            }
        }

        // 当前通道不可用，需要切换
        if current.is_some() {
            if self.config.auto_fallback && best.is_some() {
                return (true, current, best, SwitchReason::Unavailable);
            } else {
                // auto_fallback=false 或没有可用通道时，切换到 None
                return (true, current, None, SwitchReason::Unavailable);
            }
        }

        (false, current, best, SwitchReason::Initial)
    }

    /// 查找最佳可用通道
    fn find_best_channel(&self, state: &SwitcherState) -> Option<Channel> {
        // 首选通道可用
        if state.is_available(self.config.preferred_channel) {
            return Some(self.config.preferred_channel);
        }

        // 备用通道可用
        let fallback = self.config.fallback_channel();
        if state.is_available(fallback) {
            return Some(fallback);
        }

        None
    }

    /// 执行切换
    fn do_switch(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason) {
        {
            let mut state = self.state.write().unwrap();
            state.current_channel = to;
        }

        self.callback.on_channel_switched(from, to, reason);
    }
}

impl fmt::Debug for ChannelSwitcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChannelSwitcher")
            .field("config", &self.config)
            .field("current_channel", &self.current_channel())
            .finish()
    }
}

/// 切换策略 trait
///
/// 定义通道选择策略。
pub trait SwitchStrategy: Send + Sync {
    /// 选择最佳通道
    fn select(&self, wifi_available: bool, ble_available: bool, current: Option<Channel>) -> Option<Channel>;
}

/// 优先级切换策略
///
/// 始终选择最高优先级的可用通道（WiFi > BLE）。
#[derive(Debug, Clone, Copy, Default)]
pub struct PrioritySwitchStrategy;

impl SwitchStrategy for PrioritySwitchStrategy {
    fn select(&self, wifi_available: bool, ble_available: bool, _current: Option<Channel>) -> Option<Channel> {
        if wifi_available {
            Some(Channel::Wifi)
        } else if ble_available {
            Some(Channel::Ble)
        } else {
            None
        }
    }
}

/// 粘性切换策略
///
/// 尽量保持当前通道，只在当前通道不可用时才切换。
#[derive(Debug, Clone, Copy, Default)]
pub struct StickySwitchStrategy;

impl SwitchStrategy for StickySwitchStrategy {
    fn select(&self, wifi_available: bool, ble_available: bool, current: Option<Channel>) -> Option<Channel> {
        // 如果当前通道仍然可用，保持不变
        if let Some(curr) = current {
            match curr {
                Channel::Wifi if wifi_available => return Some(Channel::Wifi),
                Channel::Ble if ble_available => return Some(Channel::Ble),
                _ => {}
            }
        }

        // 否则选择任何可用通道
        if wifi_available {
            Some(Channel::Wifi)
        } else if ble_available {
            Some(Channel::Ble)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct TestCallback {
        switches: Mutex<Vec<(Option<Channel>, Option<Channel>, SwitchReason)>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                switches: Mutex::new(Vec::new()),
            }
        }

        fn switch_count(&self) -> usize {
            self.switches.lock().unwrap().len()
        }

        fn get_switches(&self) -> Vec<(Option<Channel>, Option<Channel>, SwitchReason)> {
            self.switches.lock().unwrap().clone()
        }

        fn clear(&self) {
            self.switches.lock().unwrap().clear();
        }
    }

    impl ChannelSwitchCallback for TestCallback {
        fn on_channel_switched(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason) {
            self.switches.lock().unwrap().push((from, to, reason));
        }
    }

    // Config tests
    #[test]
    fn test_config_new() {
        let config = ChannelSwitcherConfig::new();
        assert_eq!(config.preferred_channel, Channel::Wifi);
        assert!(config.auto_fallback);
        assert!(config.auto_recovery);
    }

    #[test]
    fn test_config_builder() {
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
        let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Wifi);
        assert_eq!(config.fallback_channel(), Channel::Ble);

        let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);
        assert_eq!(config.fallback_channel(), Channel::Wifi);
    }

    #[test]
    fn test_config_validate() {
        let config = ChannelSwitcherConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = ChannelSwitcherConfig::default();
        assert_eq!(config.preferred_channel, Channel::Wifi);
    }

    #[test]
    fn test_config_clone() {
        let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);
        let cloned = config.clone();
        assert_eq!(config.preferred_channel, cloned.preferred_channel);
    }

    #[test]
    fn test_config_debug() {
        let config = ChannelSwitcherConfig::new();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ChannelSwitcherConfig"));
    }

    // SwitchReason tests
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
    }

    #[test]
    fn test_switch_reason_clone_eq() {
        let r1 = SwitchReason::Unavailable;
        let r2 = r1;
        assert_eq!(r1, r2);
    }

    // Switcher tests
    #[test]
    fn test_switcher_new() {
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
    }

    #[test]
    fn test_switcher_wifi_available() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // WiFi becomes available
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

        assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
        assert_eq!(callback.switch_count(), 1);

        let switches = callback.get_switches();
        assert_eq!(switches[0].0, None);
        assert_eq!(switches[0].1, Some(Channel::Wifi));
        assert_eq!(switches[0].2, SwitchReason::Initial);
    }

    #[test]
    fn test_switcher_fallback_to_ble() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // WiFi available first
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        // BLE also available
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

        callback.clear();

        // WiFi becomes unavailable
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

        assert_eq!(switcher.current_channel(), Some(Channel::Ble));
        assert_eq!(callback.switch_count(), 1);

        let switches = callback.get_switches();
        assert_eq!(switches[0].0, Some(Channel::Wifi));
        assert_eq!(switches[0].1, Some(Channel::Ble));
        assert_eq!(switches[0].2, SwitchReason::Unavailable);
    }

    #[test]
    fn test_switcher_auto_recovery() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new().with_auto_recovery(true);
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // Only BLE available initially
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
        assert_eq!(switcher.current_channel(), Some(Channel::Ble));

        callback.clear();

        // WiFi becomes available - should auto recover
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

        assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
        assert_eq!(callback.switch_count(), 1);

        let switches = callback.get_switches();
        assert_eq!(switches[0].2, SwitchReason::HigherPriority);
    }

    #[test]
    fn test_switcher_no_auto_recovery() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new().with_auto_recovery(false);
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // Only BLE available initially
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
        assert_eq!(switcher.current_channel(), Some(Channel::Ble));

        callback.clear();

        // WiFi becomes available - should NOT auto recover
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

        assert_eq!(switcher.current_channel(), Some(Channel::Ble));
        assert_eq!(callback.switch_count(), 0);
    }

    #[test]
    fn test_switcher_no_auto_fallback() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new().with_auto_fallback(false);
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // WiFi available first
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        // BLE also available
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

        callback.clear();

        // WiFi becomes unavailable - should NOT auto fallback
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

        // Should switch to None since auto_fallback is disabled
        assert_eq!(switcher.current_channel(), None);
    }

    #[test]
    fn test_switcher_all_unavailable() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // WiFi available first
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

        callback.clear();

        // WiFi becomes unavailable (BLE never was available)
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Available, ChannelStatus::Unavailable);

        assert_eq!(switcher.current_channel(), None);
        assert_eq!(callback.switch_count(), 1);

        let switches = callback.get_switches();
        assert_eq!(switches[0].1, None);
        assert_eq!(switches[0].2, SwitchReason::Unavailable);
    }

    #[test]
    fn test_switcher_manual_switch() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        // Both available
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);

        callback.clear();

        // Manual switch to BLE
        let result = switcher.switch_to(Channel::Ble);
        assert!(result.is_ok());
        assert_eq!(switcher.current_channel(), Some(Channel::Ble));
        assert_eq!(callback.switch_count(), 1);

        let switches = callback.get_switches();
        assert_eq!(switches[0].2, SwitchReason::Manual);
    }

    #[test]
    fn test_switcher_manual_switch_unavailable() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback).unwrap();

        // Only WiFi available
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);

        // Try to switch to unavailable BLE
        let result = switcher.switch_to(Channel::Ble);
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::ChannelUnavailable)));
        assert_eq!(switcher.current_channel(), Some(Channel::Wifi));
    }

    #[test]
    fn test_switcher_reset() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback.clone()).unwrap();

        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        callback.clear();

        switcher.reset();

        assert_eq!(switcher.current_channel(), None);
        assert_eq!(callback.switch_count(), 1);
    }

    #[test]
    fn test_switcher_select_best_channel() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback).unwrap();

        // Nothing available
        assert_eq!(switcher.select_best_channel(), None);

        // Only BLE available
        switcher.handle_status_change(Channel::Ble, ChannelStatus::Unavailable, ChannelStatus::Available);
        assert_eq!(switcher.select_best_channel(), Some(Channel::Ble));

        // Both available - WiFi preferred
        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        assert_eq!(switcher.select_best_channel(), Some(Channel::Wifi));
    }

    #[test]
    fn test_switcher_has_available_channel() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback).unwrap();

        assert!(!switcher.has_available_channel());

        switcher.handle_status_change(Channel::Wifi, ChannelStatus::Unavailable, ChannelStatus::Available);
        assert!(switcher.has_available_channel());
    }

    #[test]
    fn test_switcher_config_accessor() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new().with_preferred_channel(Channel::Ble);
        let switcher = ChannelSwitcher::new(config, callback).unwrap();

        assert_eq!(switcher.config().preferred_channel, Channel::Ble);
    }

    #[test]
    fn test_switcher_debug() {
        let callback = Arc::new(TestCallback::new());
        let config = ChannelSwitcherConfig::new();
        let switcher = ChannelSwitcher::new(config, callback).unwrap();

        let debug_str = format!("{:?}", switcher);
        assert!(debug_str.contains("ChannelSwitcher"));
    }

    // Strategy tests
    #[test]
    fn test_priority_strategy() {
        let strategy = PrioritySwitchStrategy;

        // Both available - prefer WiFi
        assert_eq!(strategy.select(true, true, None), Some(Channel::Wifi));
        assert_eq!(strategy.select(true, true, Some(Channel::Ble)), Some(Channel::Wifi));

        // Only BLE
        assert_eq!(strategy.select(false, true, None), Some(Channel::Ble));

        // Only WiFi
        assert_eq!(strategy.select(true, false, None), Some(Channel::Wifi));

        // None
        assert_eq!(strategy.select(false, false, None), None);
    }

    #[test]
    fn test_sticky_strategy() {
        let strategy = StickySwitchStrategy;

        // Current WiFi, both available - stay on WiFi
        assert_eq!(strategy.select(true, true, Some(Channel::Wifi)), Some(Channel::Wifi));

        // Current BLE, both available - stay on BLE
        assert_eq!(strategy.select(true, true, Some(Channel::Ble)), Some(Channel::Ble));

        // Current WiFi, only BLE available - switch to BLE
        assert_eq!(strategy.select(false, true, Some(Channel::Wifi)), Some(Channel::Ble));

        // No current, both available - select WiFi
        assert_eq!(strategy.select(true, true, None), Some(Channel::Wifi));
    }

    #[test]
    fn test_priority_strategy_debug() {
        let strategy = PrioritySwitchStrategy;
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("PrioritySwitchStrategy"));
    }

    #[test]
    fn test_sticky_strategy_debug() {
        let strategy = StickySwitchStrategy;
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("StickySwitchStrategy"));
    }
}
