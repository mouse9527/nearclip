//! NearClip 配置模块
//!
//! 定义 NearClip 的配置结构和验证逻辑。
//!
//! # 示例
//!
//! ```
//! use nearclip_core::NearClipConfig;
//!
//! let config = NearClipConfig::new("My Device")
//!     .with_wifi_enabled(true)
//!     .with_ble_enabled(true)
//!     .with_auto_connect(true);
//!
//! assert!(config.validate().is_ok());
//! ```

use crate::error::NearClipError;
use std::time::Duration;

/// 默认设备名称
pub const DEFAULT_DEVICE_NAME: &str = "NearClip Device";

/// 默认连接超时（秒）
pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30;

/// 默认心跳间隔（秒）
pub const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 10;

/// 默认重试次数
pub const DEFAULT_MAX_RETRIES: u32 = 3;

// ============================================================
// NearClipConfig - 配置结构
// ============================================================

/// NearClip 配置
///
/// 控制 NearClip 的行为，包括设备名称、启用的通道、自动连接等。
///
/// # 示例
///
/// ```
/// use nearclip_core::NearClipConfig;
/// use std::time::Duration;
///
/// let config = NearClipConfig::new("MacBook Pro")
///     .with_wifi_enabled(true)
///     .with_ble_enabled(true)
///     .with_auto_connect(true)
///     .with_connection_timeout(Duration::from_secs(60));
///
/// assert_eq!(config.device_name(), "MacBook Pro");
/// assert!(config.wifi_enabled());
/// assert!(config.ble_enabled());
/// ```
#[derive(Debug, Clone)]
pub struct NearClipConfig {
    /// 本设备名称
    device_name: String,
    /// 启用 WiFi 通道
    wifi_enabled: bool,
    /// 启用 BLE 通道
    ble_enabled: bool,
    /// 自动连接已配对设备
    auto_connect: bool,
    /// 连接超时
    connection_timeout: Duration,
    /// 心跳间隔
    heartbeat_interval: Duration,
    /// 最大重试次数
    max_retries: u32,
    /// mDNS 服务名称
    mdns_service_name: String,
}

impl Default for NearClipConfig {
    fn default() -> Self {
        Self::new(DEFAULT_DEVICE_NAME)
    }
}

impl NearClipConfig {
    /// 创建新配置
    ///
    /// # 参数
    ///
    /// * `device_name` - 本设备的显示名称
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::NearClipConfig;
    ///
    /// let config = NearClipConfig::new("My Device");
    /// assert_eq!(config.device_name(), "My Device");
    /// ```
    pub fn new(device_name: impl Into<String>) -> Self {
        Self {
            device_name: device_name.into(),
            wifi_enabled: true,
            ble_enabled: true,
            auto_connect: true,
            connection_timeout: Duration::from_secs(DEFAULT_CONNECTION_TIMEOUT_SECS),
            heartbeat_interval: Duration::from_secs(DEFAULT_HEARTBEAT_INTERVAL_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
            mdns_service_name: "_nearclip._tcp.local.".to_string(),
        }
    }

    /// 设置 WiFi 通道启用状态
    pub fn with_wifi_enabled(mut self, enabled: bool) -> Self {
        self.wifi_enabled = enabled;
        self
    }

    /// 设置 BLE 通道启用状态
    pub fn with_ble_enabled(mut self, enabled: bool) -> Self {
        self.ble_enabled = enabled;
        self
    }

    /// 设置自动连接
    pub fn with_auto_connect(mut self, enabled: bool) -> Self {
        self.auto_connect = enabled;
        self
    }

    /// 设置连接超时
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// 设置心跳间隔
    pub fn with_heartbeat_interval(mut self, interval: Duration) -> Self {
        self.heartbeat_interval = interval;
        self
    }

    /// 设置最大重试次数
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// 设置 mDNS 服务名称
    pub fn with_mdns_service_name(mut self, name: impl Into<String>) -> Self {
        self.mdns_service_name = name.into();
        self
    }

    /// 获取设备名称
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// 检查 WiFi 是否启用
    pub fn wifi_enabled(&self) -> bool {
        self.wifi_enabled
    }

    /// 检查 BLE 是否启用
    pub fn ble_enabled(&self) -> bool {
        self.ble_enabled
    }

    /// 检查自动连接是否启用
    pub fn auto_connect(&self) -> bool {
        self.auto_connect
    }

    /// 获取连接超时
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    /// 获取心跳间隔
    pub fn heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }

    /// 获取最大重试次数
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// 获取 mDNS 服务名称
    pub fn mdns_service_name(&self) -> &str {
        &self.mdns_service_name
    }

    /// 检查是否有任何通道启用
    pub fn has_any_channel(&self) -> bool {
        self.wifi_enabled || self.ble_enabled
    }

    /// 验证配置
    ///
    /// # 错误
    ///
    /// - 设备名称为空
    /// - 没有启用任何通道
    /// - 连接超时为 0
    /// - 心跳间隔为 0
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::NearClipConfig;
    ///
    /// let config = NearClipConfig::new("Valid Device");
    /// assert!(config.validate().is_ok());
    ///
    /// let invalid = NearClipConfig::new("");
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), NearClipError> {
        if self.device_name.trim().is_empty() {
            return Err(NearClipError::Config(
                "device_name cannot be empty".to_string(),
            ));
        }

        if !self.has_any_channel() {
            return Err(NearClipError::Config(
                "at least one channel (WiFi or BLE) must be enabled".to_string(),
            ));
        }

        if self.connection_timeout.is_zero() {
            return Err(NearClipError::Config(
                "connection_timeout must be greater than 0".to_string(),
            ));
        }

        if self.heartbeat_interval.is_zero() {
            return Err(NearClipError::Config(
                "heartbeat_interval must be greater than 0".to_string(),
            ));
        }

        if self.max_retries == 0 {
            return Err(NearClipError::Config(
                "max_retries must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = NearClipConfig::new("Test Device");
        assert_eq!(config.device_name(), "Test Device");
        assert!(config.wifi_enabled());
        assert!(config.ble_enabled());
        assert!(config.auto_connect());
    }

    #[test]
    fn test_config_default() {
        let config = NearClipConfig::default();
        assert_eq!(config.device_name(), DEFAULT_DEVICE_NAME);
    }

    #[test]
    fn test_config_builder() {
        let config = NearClipConfig::new("Device")
            .with_wifi_enabled(false)
            .with_ble_enabled(true)
            .with_auto_connect(false)
            .with_connection_timeout(Duration::from_secs(60))
            .with_heartbeat_interval(Duration::from_secs(5))
            .with_max_retries(5);

        assert!(!config.wifi_enabled());
        assert!(config.ble_enabled());
        assert!(!config.auto_connect());
        assert_eq!(config.connection_timeout(), Duration::from_secs(60));
        assert_eq!(config.heartbeat_interval(), Duration::from_secs(5));
        assert_eq!(config.max_retries(), 5);
    }

    #[test]
    fn test_config_validate_success() {
        let config = NearClipConfig::new("Valid Device");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_empty_name() {
        let config = NearClipConfig::new("");
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(NearClipError::Config(_))));
    }

    #[test]
    fn test_config_validate_whitespace_name() {
        let config = NearClipConfig::new("   ");
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_no_channels() {
        let config = NearClipConfig::new("Device")
            .with_wifi_enabled(false)
            .with_ble_enabled(false);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_wifi_only() {
        let config = NearClipConfig::new("Device")
            .with_wifi_enabled(true)
            .with_ble_enabled(false);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_ble_only() {
        let config = NearClipConfig::new("Device")
            .with_wifi_enabled(false)
            .with_ble_enabled(true);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_timeout() {
        let config = NearClipConfig::new("Device")
            .with_connection_timeout(Duration::ZERO);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_zero_heartbeat() {
        let config = NearClipConfig::new("Device")
            .with_heartbeat_interval(Duration::ZERO);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_zero_retries() {
        let config = NearClipConfig::new("Device")
            .with_max_retries(0);
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_has_any_channel() {
        let config1 = NearClipConfig::new("D")
            .with_wifi_enabled(true)
            .with_ble_enabled(false);
        assert!(config1.has_any_channel());

        let config2 = NearClipConfig::new("D")
            .with_wifi_enabled(false)
            .with_ble_enabled(true);
        assert!(config2.has_any_channel());

        let config3 = NearClipConfig::new("D")
            .with_wifi_enabled(false)
            .with_ble_enabled(false);
        assert!(!config3.has_any_channel());
    }

    #[test]
    fn test_config_mdns_service_name() {
        let config = NearClipConfig::new("Device")
            .with_mdns_service_name("_custom._tcp.local.");
        assert_eq!(config.mdns_service_name(), "_custom._tcp.local.");
    }

    #[test]
    fn test_config_clone() {
        let c1 = NearClipConfig::new("Device")
            .with_wifi_enabled(false)
            .with_max_retries(10);
        let c2 = c1.clone();

        assert_eq!(c1.device_name(), c2.device_name());
        assert_eq!(c1.wifi_enabled(), c2.wifi_enabled());
        assert_eq!(c1.max_retries(), c2.max_retries());
    }

    #[test]
    fn test_config_debug() {
        let config = NearClipConfig::new("Test");
        let debug = format!("{:?}", config);
        assert!(debug.contains("NearClipConfig"));
        assert!(debug.contains("Test"));
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_CONNECTION_TIMEOUT_SECS, 30);
        assert_eq!(DEFAULT_HEARTBEAT_INTERVAL_SECS, 10);
        assert_eq!(DEFAULT_MAX_RETRIES, 3);
    }
}
