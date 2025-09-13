# Task 00107: 定义发现服务配置结构

## 任务描述

按照TDD原则定义发现服务的配置结构，包含超时、重试等参数。

## TDD开发要求

### RED阶段 - 编写失败的测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_config_default() {
        // RED: 测试默认配置
        let config = DiscoveryConfig::default();
        
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retry_count, 3);
        assert_eq!(config.scan_interval_ms, 5000);
        assert!(config.enable_auto_restart);
    }

    #[test]
    fn test_discovery_config_builder() {
        // RED: 测试配置构建器
        let config = DiscoveryConfig::builder()
            .timeout_seconds(60)
            .max_retry_count(5)
            .scan_interval_ms(2000)
            .enable_auto_restart(false)
            .build();
        
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.max_retry_count, 5);
        assert_eq!(config.scan_interval_ms, 2000);
        assert!(!config.enable_auto_restart);
    }

    #[test]
    fn test_discovery_config_validation() {
        // RED: 测试配置验证
        let valid_config = DiscoveryConfig::default();
        assert!(valid_config.validate().is_ok());
        
        let invalid_config = DiscoveryConfig {
            timeout_seconds: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
```

### GREEN阶段 - 最小实现
```rust
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub timeout_seconds: u64,
    pub max_retry_count: u32,
    pub scan_interval_ms: u64,
    pub enable_auto_restart: bool,
    pub enable_wifi_discovery: bool,
    pub enable_ble_discovery: bool,
    pub max_device_count: usize,
    pub discovery_name: String,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retry_count: 3,
            scan_interval_ms: 5000,
            enable_auto_restart: true,
            enable_wifi_discovery: true,
            enable_ble_discovery: true,
            max_device_count: 50,
            discovery_name: "NearClip Discovery".to_string(),
        }
    }
}

impl DiscoveryConfig {
    pub fn builder() -> DiscoveryConfigBuilder {
        DiscoveryConfigBuilder::new()
    }
    
    pub fn validate(&self) -> Result<(), DiscoveryConfigError> {
        if self.timeout_seconds == 0 {
            return Err(DiscoveryConfigError::InvalidTimeout("Timeout must be positive".to_string()));
        }
        
        if self.max_retry_count == 0 {
            return Err(DiscoveryConfigError::InvalidRetryCount("Retry count must be positive".to_string()));
        }
        
        if self.scan_interval_ms == 0 {
            return Err(DiscoveryConfigError::InvalidScanInterval("Scan interval must be positive".to_string()));
        }
        
        if self.max_device_count == 0 {
            return Err(DiscoveryConfigError::InvalidMaxDevices("Max device count must be positive".to_string()));
        }
        
        if self.discovery_name.is_empty() {
            return Err(DiscoveryConfigError::InvalidName("Discovery name cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct DiscoveryConfigBuilder {
    config: DiscoveryConfig,
}

impl DiscoveryConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: DiscoveryConfig::default(),
        }
    }
    
    pub fn timeout_seconds(mut self, timeout: u64) -> Self {
        self.config.timeout_seconds = timeout;
        self
    }
    
    pub fn max_retry_count(mut self, count: u32) -> Self {
        self.config.max_retry_count = count;
        self
    }
    
    pub fn scan_interval_ms(mut self, interval: u64) -> Self {
        self.config.scan_interval_ms = interval;
        self
    }
    
    pub fn enable_auto_restart(mut self, enable: bool) -> Self {
        self.config.enable_auto_restart = enable;
        self
    }
    
    pub fn enable_wifi_discovery(mut self, enable: bool) -> Self {
        self.config.enable_wifi_discovery = enable;
        self
    }
    
    pub fn enable_ble_discovery(mut self, enable: bool) -> Self {
        self.config.enable_ble_discovery = enable;
        self
    }
    
    pub fn max_device_count(mut self, count: usize) -> Self {
        self.config.max_device_count = count;
        self
    }
    
    pub fn discovery_name(mut self, name: String) -> Self {
        self.config.discovery_name = name;
        self
    }
    
    pub fn build(self) -> DiscoveryConfig {
        self.config
    }
}

#[derive(Debug, PartialEq)]
pub enum DiscoveryConfigError {
    InvalidTimeout(String),
    InvalidRetryCount(String),
    InvalidScanInterval(String),
    InvalidMaxDevices(String),
    InvalidName(String),
}
```

### REFACTOR阶段
```rust
impl DiscoveryConfig {
    pub fn for_testing() -> Self {
        Self {
            timeout_seconds: 5,
            max_retry_count: 1,
            scan_interval_ms: 1000,
            enable_auto_restart: false,
            enable_wifi_discovery: false,
            enable_ble_discovery: false,
            max_device_count: 10,
            discovery_name: "Test Discovery".to_string(),
        }
    }
    
    pub fn for_high_performance() -> Self {
        Self {
            timeout_seconds: 15,
            max_retry_count: 5,
            scan_interval_ms: 2000,
            enable_auto_restart: true,
            enable_wifi_discovery: true,
            enable_ble_discovery: true,
            max_device_count: 100,
            discovery_name: "High Performance Discovery".to_string(),
        }
    }
    
    pub fn for_battery_saving() -> Self {
        Self {
            timeout_seconds: 60,
            max_retry_count: 2,
            scan_interval_ms: 10000,
            enable_auto_restart: false,
            enable_wifi_discovery: true,
            enable_ble_discovery: false,
            max_device_count: 20,
            discovery_name: "Battery Saving Discovery".to_string(),
        }
    }
}
```

## 验收标准
- [ ] 所有测试通过
- [ ] 配置结构定义完整
- [ ] 构建器和验证正常工作

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00108a: Android 应用构建](00108a-android-app-build-verification.md)
- [Task 00108b: Android BLE 设备发现核心](00108b-android-ble-discovery-core.md)
