# Task 0101: 实现设备抽象层 (TDD版本)

## 任务描述

按照TDD原则实现设备抽象层，为不同类型的设备提供统一的接口。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_abstraction_tests.rs
#[cfg(test)]
mod device_abstraction_tests {
    use super::*;
    
    #[test]
    fn test_device_creation() {
        // RED: 测试设备创建
        let device = Device::new("device-001", "Test Device", DeviceType::Mobile);
        
        assert_eq!(device.id(), "device-001");
        assert_eq!(device.name(), "Test Device");
        assert_eq!(device.device_type(), &DeviceType::Mobile);
        assert_eq!(device.status(), &DeviceStatus::Offline);
    }
    
    #[test]
    fn test_device_status_transitions() {
        // RED: 测试设备状态转换
        let mut device = Device::new("device-001", "Test Device", DeviceType::Mobile);
        
        // 初始状态
        assert_eq!(device.status(), &DeviceStatus::Offline);
        
        // 连接设备
        device.connect().unwrap();
        assert_eq!(device.status(), &DeviceStatus::Online);
        
        // 断开连接
        device.disconnect().unwrap();
        assert_eq!(device.status(), &DeviceStatus::Offline);
    }
    
    #[test]
    fn test_device_capabilities() {
        // RED: 测试设备能力
        let mut device = Device::new("device-001", "Test Device", DeviceType::Mobile);
        
        // 初始无能力
        assert!(!device.has_capability(&DeviceCapability::WiFi));
        
        // 添加能力
        device.add_capability(DeviceCapability::WiFi);
        assert!(device.has_capability(&DeviceCapability::WiFi));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tablet,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    Online,
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    WiFi,
    Bluetooth,
    Cellular,
}

#[derive(Debug)]
pub struct Device {
    id: String,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
    capabilities: std::collections::HashSet<DeviceCapability>,
}

impl Device {
    pub fn new(id: &str, name: &str, device_type: DeviceType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            device_type,
            status: DeviceStatus::Offline,
            capabilities: std::collections::HashSet::new(),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }
    
    pub fn status(&self) -> &DeviceStatus {
        &self.status
    }
    
    pub fn connect(&mut self) -> Result<(), DeviceError> {
        self.status = DeviceStatus::Online;
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), DeviceError> {
        self.status = DeviceStatus::Offline;
        Ok(())
    }
    
    pub fn add_capability(&mut self, capability: DeviceCapability) {
        self.capabilities.insert(capability);
    }
    
    pub fn has_capability(&self, capability: &DeviceCapability) -> bool {
        self.capabilities.contains(capability)
    }
}

#[derive(Debug)]
pub enum DeviceError {
    ConnectionFailed,
    AlreadyConnected,
    AlreadyDisconnected,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::HashSet;
use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tablet,
    Wearable,
    TV,
    Embedded,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    Online,
    Offline,
    Connecting,
    Disconnecting,
    Busy,
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    WiFi,
    Bluetooth,
    Cellular,
    Ethernet,
    TextTransfer,
    FileTransfer,
    Encryption,
    Compression,
    MultiHop,
    CrossPlatform,
}

#[derive(Debug)]
pub struct Device {
    id: String,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
    capabilities: HashSet<DeviceCapability>,
    address: DeviceAddress,
    metrics: DeviceMetrics,
    metadata: DeviceMetadata,
    created_at: SystemTime,
    last_updated: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DeviceAddress {
    pub ip_address: Option<IpAddr>,
    pub mac_address: Option<String>,
    pub bluetooth_address: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct DeviceMetrics {
    pub signal_strength: Option<f32>,
    pub battery_level: Option<f32>,
    pub last_seen: SystemTime,
    pub connection_uptime: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Debug, Clone)]
pub struct DeviceMetadata {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub custom_attributes: std::collections::HashMap<String, String>,
}

impl Device {
    // 重构后的代码，保持测试绿色
    pub fn new(id: &str, name: &str, device_type: DeviceType) -> Self {
        let now = SystemTime::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            device_type,
            status: DeviceStatus::Offline,
            capabilities: HashSet::new(),
            address: DeviceAddress::new(),
            metrics: DeviceMetrics::new(),
            metadata: DeviceMetadata::new(),
            created_at: now,
            last_updated: now,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }
    
    pub fn status(&self) -> &DeviceStatus {
        &self.status
    }
    
    pub fn capabilities(&self) -> &HashSet<DeviceCapability> {
        &self.capabilities
    }
    
    pub fn address(&self) -> &DeviceAddress {
        &self.address
    }
    
    pub fn metrics(&self) -> &DeviceMetrics {
        &self.metrics
    }
    
    pub fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
    
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }
    
    pub fn last_updated(&self) -> SystemTime {
        self.last_updated
    }
    
    pub fn connect(&mut self) -> Result<(), DeviceError> {
        match self.status {
            DeviceStatus::Offline => {
                self.status = DeviceStatus::Connecting;
                self.touch();
                // 模拟连接过程
                self.status = DeviceStatus::Online;
                self.metrics.last_seen = SystemTime::now();
                Ok(())
            }
            DeviceStatus::Online => Err(DeviceError::AlreadyConnected),
            DeviceStatus::Connecting => Err(DeviceError::ConnectionInProgress),
            _ => Err(DeviceError::InvalidStateTransition),
        }
    }
    
    pub fn disconnect(&mut self) -> Result<(), DeviceError> {
        match self.status {
            DeviceStatus::Online => {
                self.status = DeviceStatus::Disconnecting;
                self.touch();
                self.status = DeviceStatus::Offline;
                Ok(())
            }
            DeviceStatus::Offline => Err(DeviceError::AlreadyDisconnected),
            DeviceStatus::Disconnecting => Err(DeviceError::DisconnectionInProgress),
            _ => Err(DeviceError::InvalidStateTransition),
        }
    }
    
    pub fn add_capability(&mut self, capability: DeviceCapability) {
        if self.capabilities.insert(capability) {
            self.touch();
        }
    }
    
    pub fn remove_capability(&mut self, capability: &DeviceCapability) {
        if self.capabilities.remove(capability) {
            self.touch();
        }
    }
    
    pub fn has_capability(&self, capability: &DeviceCapability) -> bool {
        self.capabilities.contains(capability)
    }
    
    fn touch(&mut self) {
        self.last_updated = SystemTime::now();
    }
}

impl DeviceAddress {
    pub fn new() -> Self {
        Self {
            ip_address: None,
            mac_address: None,
            bluetooth_address: None,
            port: None,
        }
    }
}

impl DeviceMetrics {
    pub fn new() -> Self {
        Self {
            signal_strength: None,
            battery_level: None,
            last_seen: SystemTime::now(),
            connection_uptime: Duration::ZERO,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
}

impl DeviceMetadata {
    pub fn new() -> Self {
        Self {
            manufacturer: None,
            model: None,
            os_name: None,
            os_version: None,
            app_version: None,
            custom_attributes: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum DeviceError {
    ConnectionFailed(String),
    AlreadyConnected,
    AlreadyDisconnected,
    ConnectionInProgress,
    DisconnectionInProgress,
    InvalidStateTransition,
    DeviceNotFound,
    AuthenticationFailed,
    NetworkError(String),
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的核心实体：

```rust
// rust-core/domain/entities/device.rs
pub struct Device {
    // 设备抽象实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- 无

## 后续任务

- [Task 0102: 实现设备发现服务](../tasks/0102-device-discovery-service.md)
- [Task 0103: 实现传输抽象](../tasks/0103-transport-abstraction.md)  
- [Task 0104: 实现mDNS发现](../tasks/0104-mdns-discovery.md)
- [Task 0105: 实现BLE发现](../tasks/0105-ble-discovery.md)