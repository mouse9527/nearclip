//! 通信通道抽象
//!
//! 定义 WiFi 和 BLE 通道的抽象接口和选择策略。
//!
//! # 通道类型
//!
//! | 通道 | 优先级 | 特点 |
//! |------|--------|------|
//! | WiFi | 高 | 高速、低延迟、需要同一局域网 |
//! | BLE | 低 | 低速、高延迟、不依赖网络 |
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{Channel, ChannelStatus};
//!
//! let wifi_status = ChannelStatus::Available;
//! let ble_status = ChannelStatus::Unavailable;
//!
//! // WiFi 优先
//! if wifi_status == ChannelStatus::Available {
//!     println!("Using WiFi channel");
//! } else if ble_status == ChannelStatus::Available {
//!     println!("Falling back to BLE channel");
//! }
//! ```

use std::fmt;

/// 通信通道类型
///
/// NearClip 支持两种通信通道，WiFi 优先，BLE 备选。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Channel {
    /// WiFi 通道 (TCP/TLS)
    ///
    /// 高速、低延迟，适合大数据量传输。
    /// 需要两台设备在同一局域网内。
    #[default]
    Wifi,

    /// BLE 通道 (Bluetooth Low Energy)
    ///
    /// 低速、高延迟，适合小数据量传输。
    /// 不依赖 WiFi 网络，适合户外场景。
    Ble,
}

impl Channel {
    /// 获取通道名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Wifi => "wifi",
            Channel::Ble => "ble",
        }
    }

    /// 获取通道优先级（数值越大优先级越高）
    pub fn priority(&self) -> u8 {
        match self {
            Channel::Wifi => 10,
            Channel::Ble => 5,
        }
    }

    /// 是否是高速通道
    pub fn is_high_speed(&self) -> bool {
        matches!(self, Channel::Wifi)
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 通道状态
///
/// 表示通道的当前可用性。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ChannelStatus {
    /// 可用
    ///
    /// 通道已连接，可以发送数据。
    #[default]
    Available,

    /// 不可用
    ///
    /// 通道未连接或连接已断开。
    Unavailable,

    /// 忙碌
    ///
    /// 通道正在传输数据，暂时无法处理新请求。
    Busy,

    /// 连接中
    ///
    /// 正在建立连接。
    Connecting,
}

impl ChannelStatus {
    /// 获取状态名称
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelStatus::Available => "available",
            ChannelStatus::Unavailable => "unavailable",
            ChannelStatus::Busy => "busy",
            ChannelStatus::Connecting => "connecting",
        }
    }

    /// 是否可以发送数据
    pub fn can_send(&self) -> bool {
        matches!(self, ChannelStatus::Available)
    }
}

impl fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 通道信息
///
/// 包含通道类型和状态的组合信息。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelInfo {
    /// 通道类型
    pub channel: Channel,
    /// 通道状态
    pub status: ChannelStatus,
}

impl ChannelInfo {
    /// 创建通道信息
    pub fn new(channel: Channel, status: ChannelStatus) -> Self {
        Self { channel, status }
    }

    /// 是否可以发送数据
    pub fn can_send(&self) -> bool {
        self.status.can_send()
    }
}

/// 通道选择策略
///
/// 定义如何从多个可用通道中选择最佳通道。
pub trait ChannelSelector: Send + Sync {
    /// 选择最佳可用通道
    ///
    /// # Arguments
    ///
    /// * `channels` - 可用通道列表及其状态
    ///
    /// # Returns
    ///
    /// 选中的通道，如果没有可用通道则返回 None
    fn select(&self, channels: &[ChannelInfo]) -> Option<Channel>;
}

/// 默认通道选择器
///
/// 按优先级选择：WiFi > BLE。
/// 只选择状态为 `Available` 的通道。
#[derive(Debug, Clone, Copy, Default)]
pub struct PriorityChannelSelector;

impl ChannelSelector for PriorityChannelSelector {
    fn select(&self, channels: &[ChannelInfo]) -> Option<Channel> {
        channels
            .iter()
            .filter(|info| info.can_send())
            .max_by_key(|info| info.channel.priority())
            .map(|info| info.channel)
    }
}

/// 仅 WiFi 通道选择器
///
/// 只选择 WiFi 通道，忽略 BLE。
#[derive(Debug, Clone, Copy, Default)]
pub struct WifiOnlyChannelSelector;

impl ChannelSelector for WifiOnlyChannelSelector {
    fn select(&self, channels: &[ChannelInfo]) -> Option<Channel> {
        channels
            .iter()
            .find(|info| info.channel == Channel::Wifi && info.can_send())
            .map(|info| info.channel)
    }
}

/// 仅 BLE 通道选择器
///
/// 只选择 BLE 通道，忽略 WiFi。
#[derive(Debug, Clone, Copy, Default)]
pub struct BleOnlyChannelSelector;

impl ChannelSelector for BleOnlyChannelSelector {
    fn select(&self, channels: &[ChannelInfo]) -> Option<Channel> {
        channels
            .iter()
            .find(|info| info.channel == Channel::Ble && info.can_send())
            .map(|info| info.channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_as_str() {
        assert_eq!(Channel::Wifi.as_str(), "wifi");
        assert_eq!(Channel::Ble.as_str(), "ble");
    }

    #[test]
    fn test_channel_priority() {
        assert!(Channel::Wifi.priority() > Channel::Ble.priority());
    }

    #[test]
    fn test_channel_is_high_speed() {
        assert!(Channel::Wifi.is_high_speed());
        assert!(!Channel::Ble.is_high_speed());
    }

    #[test]
    fn test_channel_display() {
        assert_eq!(format!("{}", Channel::Wifi), "wifi");
        assert_eq!(format!("{}", Channel::Ble), "ble");
    }

    #[test]
    fn test_channel_default() {
        assert_eq!(Channel::default(), Channel::Wifi);
    }

    #[test]
    fn test_channel_status_as_str() {
        assert_eq!(ChannelStatus::Available.as_str(), "available");
        assert_eq!(ChannelStatus::Unavailable.as_str(), "unavailable");
        assert_eq!(ChannelStatus::Busy.as_str(), "busy");
        assert_eq!(ChannelStatus::Connecting.as_str(), "connecting");
    }

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
        assert_eq!(format!("{}", ChannelStatus::Busy), "busy");
    }

    #[test]
    fn test_channel_status_default() {
        assert_eq!(ChannelStatus::default(), ChannelStatus::Available);
    }

    #[test]
    fn test_channel_info_new() {
        let info = ChannelInfo::new(Channel::Wifi, ChannelStatus::Available);
        assert_eq!(info.channel, Channel::Wifi);
        assert_eq!(info.status, ChannelStatus::Available);
    }

    #[test]
    fn test_channel_info_can_send() {
        let available = ChannelInfo::new(Channel::Wifi, ChannelStatus::Available);
        let unavailable = ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable);

        assert!(available.can_send());
        assert!(!unavailable.can_send());
    }

    #[test]
    fn test_priority_channel_selector_wifi_first() {
        let selector = PriorityChannelSelector;
        let channels = vec![
            ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
        ];

        let selected = selector.select(&channels);
        assert_eq!(selected, Some(Channel::Wifi));
    }

    #[test]
    fn test_priority_channel_selector_fallback_to_ble() {
        let selector = PriorityChannelSelector;
        let channels = vec![
            ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable),
        ];

        let selected = selector.select(&channels);
        assert_eq!(selected, Some(Channel::Ble));
    }

    #[test]
    fn test_priority_channel_selector_none_available() {
        let selector = PriorityChannelSelector;
        let channels = vec![
            ChannelInfo::new(Channel::Ble, ChannelStatus::Unavailable),
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Busy),
        ];

        let selected = selector.select(&channels);
        assert_eq!(selected, None);
    }

    #[test]
    fn test_priority_channel_selector_empty() {
        let selector = PriorityChannelSelector;
        let channels: Vec<ChannelInfo> = vec![];

        let selected = selector.select(&channels);
        assert_eq!(selected, None);
    }

    #[test]
    fn test_wifi_only_channel_selector() {
        let selector = WifiOnlyChannelSelector;

        // WiFi available
        let channels = vec![
            ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
        ];
        assert_eq!(selector.select(&channels), Some(Channel::Wifi));

        // WiFi unavailable
        let channels = vec![
            ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Unavailable),
        ];
        assert_eq!(selector.select(&channels), None);
    }

    #[test]
    fn test_ble_only_channel_selector() {
        let selector = BleOnlyChannelSelector;

        // BLE available
        let channels = vec![
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
            ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
        ];
        assert_eq!(selector.select(&channels), Some(Channel::Ble));

        // BLE unavailable
        let channels = vec![
            ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
            ChannelInfo::new(Channel::Ble, ChannelStatus::Unavailable),
        ];
        assert_eq!(selector.select(&channels), None);
    }

    #[test]
    fn test_channel_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Channel::Wifi);
        set.insert(Channel::Ble);

        assert!(set.contains(&Channel::Wifi));
        assert!(set.contains(&Channel::Ble));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_channel_status_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ChannelStatus::Available);
        set.insert(ChannelStatus::Unavailable);

        assert!(set.contains(&ChannelStatus::Available));
        assert!(set.contains(&ChannelStatus::Unavailable));
    }

    #[test]
    fn test_channel_clone() {
        let ch = Channel::Wifi;
        let cloned = ch;
        assert_eq!(ch, cloned);
    }

    #[test]
    fn test_channel_info_clone() {
        let info = ChannelInfo::new(Channel::Wifi, ChannelStatus::Available);
        let cloned = info;
        assert_eq!(info, cloned);
    }

    #[test]
    fn test_priority_selector_debug() {
        let selector = PriorityChannelSelector;
        let debug_str = format!("{:?}", selector);
        assert!(debug_str.contains("PriorityChannelSelector"));
    }
}
