//! 设备信息管理模块
//!
//! 定义设备相关的数据结构，包括设备信息、状态和平台类型。
//!
//! # 示例
//!
//! ```
//! use nearclip_core::{DeviceInfo, DeviceStatus, DevicePlatform};
//!
//! // 创建设备信息
//! let device = DeviceInfo::new("device-123", "MacBook Pro")
//!     .with_platform(DevicePlatform::MacOS);
//!
//! assert_eq!(device.id(), "device-123");
//! assert_eq!(device.name(), "MacBook Pro");
//! assert_eq!(device.platform(), DevicePlatform::MacOS);
//! assert_eq!(device.status(), DeviceStatus::Disconnected);
//! ```

use std::time::Instant;

// ============================================================
// DevicePlatform - 设备平台
// ============================================================

/// 设备平台类型
///
/// 标识设备运行的操作系统平台。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DevicePlatform {
    /// macOS 平台
    MacOS,
    /// Android 平台
    Android,
    /// 未知平台
    #[default]
    Unknown,
}

impl DevicePlatform {
    /// 返回平台名称字符串
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::DevicePlatform;
    ///
    /// assert_eq!(DevicePlatform::MacOS.as_str(), "macOS");
    /// assert_eq!(DevicePlatform::Android.as_str(), "Android");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            DevicePlatform::MacOS => "macOS",
            DevicePlatform::Android => "Android",
            DevicePlatform::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for DevicePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// DeviceStatus - 设备状态
// ============================================================

/// 设备连接状态
///
/// 表示设备当前的连接状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DeviceStatus {
    /// 已连接
    Connected,
    /// 未连接
    #[default]
    Disconnected,
    /// 连接中
    Connecting,
    /// 连接失败
    Failed,
}

impl DeviceStatus {
    /// 返回状态名称字符串
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::DeviceStatus;
    ///
    /// assert_eq!(DeviceStatus::Connected.as_str(), "Connected");
    /// assert_eq!(DeviceStatus::Disconnected.as_str(), "Disconnected");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceStatus::Connected => "Connected",
            DeviceStatus::Disconnected => "Disconnected",
            DeviceStatus::Connecting => "Connecting",
            DeviceStatus::Failed => "Failed",
        }
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        matches!(self, DeviceStatus::Connected)
    }

    /// 检查是否可以尝试连接
    pub fn can_connect(&self) -> bool {
        matches!(self, DeviceStatus::Disconnected | DeviceStatus::Failed)
    }
}

impl std::fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// DeviceInfo - 设备信息
// ============================================================

/// 设备信息
///
/// 存储设备的基本信息，包括 ID、名称、平台和连接状态。
///
/// # 示例
///
/// ```
/// use nearclip_core::{DeviceInfo, DeviceStatus, DevicePlatform};
///
/// let device = DeviceInfo::new("abc123", "My MacBook")
///     .with_platform(DevicePlatform::MacOS);
///
/// assert_eq!(device.id(), "abc123");
/// assert_eq!(device.name(), "My MacBook");
/// assert!(device.status().can_connect());
/// ```
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// 设备唯一标识符
    id: String,
    /// 设备显示名称
    name: String,
    /// 设备平台
    platform: DevicePlatform,
    /// 连接状态
    status: DeviceStatus,
    /// 最后活动时间
    last_seen: Option<Instant>,
}

impl DeviceInfo {
    /// 创建新的设备信息
    ///
    /// # 参数
    ///
    /// * `id` - 设备唯一标识符
    /// * `name` - 设备显示名称
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::DeviceInfo;
    ///
    /// let device = DeviceInfo::new("device-1", "Test Device");
    /// assert_eq!(device.id(), "device-1");
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            platform: DevicePlatform::Unknown,
            status: DeviceStatus::Disconnected,
            last_seen: None,
        }
    }

    /// 设置设备平台
    pub fn with_platform(mut self, platform: DevicePlatform) -> Self {
        self.platform = platform;
        self
    }

    /// 设置设备状态
    pub fn with_status(mut self, status: DeviceStatus) -> Self {
        self.status = status;
        self
    }

    /// 获取设备 ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取设备名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取设备平台
    pub fn platform(&self) -> DevicePlatform {
        self.platform
    }

    /// 获取设备状态
    pub fn status(&self) -> DeviceStatus {
        self.status
    }

    /// 获取最后活动时间
    pub fn last_seen(&self) -> Option<Instant> {
        self.last_seen
    }

    /// 更新状态
    pub fn set_status(&mut self, status: DeviceStatus) {
        self.status = status;
        if status == DeviceStatus::Connected {
            self.last_seen = Some(Instant::now());
        }
    }

    /// 更新最后活动时间
    pub fn touch(&mut self) {
        self.last_seen = Some(Instant::now());
    }

    /// 设置设备名称
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// 设置设备平台
    pub fn set_platform(&mut self, platform: DevicePlatform) {
        self.platform = platform;
    }
}

impl PartialEq for DeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DeviceInfo {}

impl std::hash::Hash for DeviceInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------
    // DevicePlatform 测试
    // --------------------------------------------------------

    #[test]
    fn test_platform_as_str() {
        assert_eq!(DevicePlatform::MacOS.as_str(), "macOS");
        assert_eq!(DevicePlatform::Android.as_str(), "Android");
        assert_eq!(DevicePlatform::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(format!("{}", DevicePlatform::MacOS), "macOS");
        assert_eq!(format!("{}", DevicePlatform::Android), "Android");
    }

    #[test]
    fn test_platform_default() {
        let platform: DevicePlatform = Default::default();
        assert_eq!(platform, DevicePlatform::Unknown);
    }

    #[test]
    fn test_platform_clone() {
        let p1 = DevicePlatform::MacOS;
        let p2 = p1;
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_platform_debug() {
        let debug = format!("{:?}", DevicePlatform::Android);
        assert!(debug.contains("Android"));
    }

    // --------------------------------------------------------
    // DeviceStatus 测试
    // --------------------------------------------------------

    #[test]
    fn test_status_as_str() {
        assert_eq!(DeviceStatus::Connected.as_str(), "Connected");
        assert_eq!(DeviceStatus::Disconnected.as_str(), "Disconnected");
        assert_eq!(DeviceStatus::Connecting.as_str(), "Connecting");
        assert_eq!(DeviceStatus::Failed.as_str(), "Failed");
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", DeviceStatus::Connected), "Connected");
        assert_eq!(format!("{}", DeviceStatus::Connecting), "Connecting");
    }

    #[test]
    fn test_status_is_connected() {
        assert!(DeviceStatus::Connected.is_connected());
        assert!(!DeviceStatus::Disconnected.is_connected());
        assert!(!DeviceStatus::Connecting.is_connected());
        assert!(!DeviceStatus::Failed.is_connected());
    }

    #[test]
    fn test_status_can_connect() {
        assert!(DeviceStatus::Disconnected.can_connect());
        assert!(DeviceStatus::Failed.can_connect());
        assert!(!DeviceStatus::Connected.can_connect());
        assert!(!DeviceStatus::Connecting.can_connect());
    }

    #[test]
    fn test_status_default() {
        let status: DeviceStatus = Default::default();
        assert_eq!(status, DeviceStatus::Disconnected);
    }

    // --------------------------------------------------------
    // DeviceInfo 测试
    // --------------------------------------------------------

    #[test]
    fn test_device_new() {
        let device = DeviceInfo::new("id-123", "Test Device");
        assert_eq!(device.id(), "id-123");
        assert_eq!(device.name(), "Test Device");
        assert_eq!(device.platform(), DevicePlatform::Unknown);
        assert_eq!(device.status(), DeviceStatus::Disconnected);
        assert!(device.last_seen().is_none());
    }

    #[test]
    fn test_device_with_platform() {
        let device = DeviceInfo::new("id", "name").with_platform(DevicePlatform::MacOS);
        assert_eq!(device.platform(), DevicePlatform::MacOS);
    }

    #[test]
    fn test_device_with_status() {
        let device = DeviceInfo::new("id", "name").with_status(DeviceStatus::Connected);
        assert_eq!(device.status(), DeviceStatus::Connected);
    }

    #[test]
    fn test_device_builder_chain() {
        let device = DeviceInfo::new("id", "name")
            .with_platform(DevicePlatform::Android)
            .with_status(DeviceStatus::Connecting);

        assert_eq!(device.platform(), DevicePlatform::Android);
        assert_eq!(device.status(), DeviceStatus::Connecting);
    }

    #[test]
    fn test_device_set_status() {
        let mut device = DeviceInfo::new("id", "name");
        assert!(device.last_seen().is_none());

        device.set_status(DeviceStatus::Connected);
        assert_eq!(device.status(), DeviceStatus::Connected);
        assert!(device.last_seen().is_some());
    }

    #[test]
    fn test_device_touch() {
        let mut device = DeviceInfo::new("id", "name");
        assert!(device.last_seen().is_none());

        device.touch();
        assert!(device.last_seen().is_some());
    }

    #[test]
    fn test_device_set_name() {
        let mut device = DeviceInfo::new("id", "old name");
        device.set_name("new name");
        assert_eq!(device.name(), "new name");
    }

    #[test]
    fn test_device_set_platform() {
        let mut device = DeviceInfo::new("id", "name");
        device.set_platform(DevicePlatform::MacOS);
        assert_eq!(device.platform(), DevicePlatform::MacOS);
    }

    #[test]
    fn test_device_equality() {
        let d1 = DeviceInfo::new("same-id", "Device 1");
        let d2 = DeviceInfo::new("same-id", "Device 2");
        let d3 = DeviceInfo::new("different-id", "Device 1");

        assert_eq!(d1, d2); // 相同 ID
        assert_ne!(d1, d3); // 不同 ID
    }

    #[test]
    fn test_device_hash() {
        use std::collections::HashSet;

        let d1 = DeviceInfo::new("id-1", "Device 1");
        let d2 = DeviceInfo::new("id-1", "Device 1 copy");
        let d3 = DeviceInfo::new("id-2", "Device 2");

        let mut set = HashSet::new();
        set.insert(d1);
        set.insert(d2); // 相同 ID，不会增加
        set.insert(d3);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_device_clone() {
        let d1 = DeviceInfo::new("id", "name")
            .with_platform(DevicePlatform::MacOS)
            .with_status(DeviceStatus::Connected);
        let d2 = d1.clone();

        assert_eq!(d1.id(), d2.id());
        assert_eq!(d1.name(), d2.name());
        assert_eq!(d1.platform(), d2.platform());
        assert_eq!(d1.status(), d2.status());
    }

    #[test]
    fn test_device_debug() {
        let device = DeviceInfo::new("id-123", "Test");
        let debug = format!("{:?}", device);
        assert!(debug.contains("DeviceInfo"));
        assert!(debug.contains("id-123"));
    }
}
