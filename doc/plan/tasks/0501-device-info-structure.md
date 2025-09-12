# Task 0501: 实现设备信息结构 (TDD版本)

## 任务描述

按照TDD原则实现设备信息结构，定义设备的基本属性和元数据。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_info_tests.rs
#[cfg(test)]
mod device_info_tests {
    use super::*;
    
    #[test]
    fn test_device_info_creation() {
        // RED: 测试设备信息创建
        let device = DeviceInfo::new("device-001", "My iPhone", DeviceType::Mobile);
        
        assert_eq!(device.id(), "device-001");
        assert_eq!(device.name(), "My iPhone");
        assert_eq!(device.device_type(), &DeviceType::Mobile);
        assert!(device.created_at() <= SystemTime::now());
    }
    
    #[test]
    fn test_device_info_update() {
        // RED: 测试设备信息更新
        let mut device = DeviceInfo::new("device-001", "Old Name", DeviceType::Mobile);
        
        device.update_name("New Name");
        assert_eq!(device.name(), "New Name");
        assert!(device.last_updated() > device.created_at());
    }
    
    #[test]
    fn test_device_capabilities() {
        // RED: 测试设备能力
        let mut device = DeviceInfo::new("device-001", "Test Device", DeviceType::Mobile);
        
        device.add_capability(DeviceCapability::TextTransfer);
        device.add_capability(DeviceCapability::Encryption);
        
        assert!(device.has_capability(&DeviceCapability::TextTransfer));
        assert!(device.has_capability(&DeviceCapability::Encryption));
        assert!(!device.has_capability(&DeviceCapability::FileTransfer));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::SystemTime;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tablet,
    Wearable,
    TV,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    TextTransfer,
    FileTransfer,
    Encryption,
    Compression,
    NetworkDiscovery,
    Bluetooth,
    WiFi,
}

#[derive(Debug)]
pub struct DeviceInfo {
    id: String,
    name: String,
    device_type: DeviceType,
    capabilities: HashSet<DeviceCapability>,
    created_at: SystemTime,
    last_updated: SystemTime,
}

impl DeviceInfo {
    pub fn new(id: &str, name: &str, device_type: DeviceType) -> Self {
        let now = SystemTime::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            device_type,
            capabilities: HashSet::new(),
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
    
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }
    
    pub fn last_updated(&self) -> SystemTime {
        self.last_updated
    }
    
    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.last_updated = SystemTime::now();
    }
    
    pub fn add_capability(&mut self, capability: DeviceCapability) {
        self.capabilities.insert(capability);
        self.last_updated = SystemTime::now();
    }
    
    pub fn has_capability(&self, capability: &DeviceCapability) -> bool {
        self.capabilities.contains(capability)
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::{HashSet, HashMap};
use std::net::IpAddr;
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    TextTransfer,
    FileTransfer,
    ImageTransfer,
    Encryption,
    Compression,
    NetworkDiscovery,
    Bluetooth,
    WiFi,
    Ethernet,
    MultiHop,
    CrossPlatform,
    AutoSync,
    BatteryOptimization,
}

#[derive(Debug, Clone)]
pub enum DeviceStatus {
    Online,
    Offline,
    Busy,
    Idle,
    Maintenance,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct DeviceLocation {
    pub network_name: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub mac_address: Option<String>,
    pub bluetooth_address: Option<String>,
    pub last_seen: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DevicePerformance {
    pub battery_level: Option<f32>,
    pub signal_strength: Option<f32>,
    pub bandwidth: Option<u64>,
    pub latency: Option<Duration>,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct DeviceMetadata {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub hardware_id: Option<String>,
    pub custom_attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct DeviceInfo {
    id: String,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
    capabilities: HashSet<DeviceCapability>,
    location: DeviceLocation,
    performance: DevicePerformance,
    metadata: DeviceMetadata,
    created_at: SystemTime,
    last_updated: SystemTime,
    last_seen: SystemTime,
    connection_count: u32,
    trust_level: f32,
}

impl DeviceInfo {
    // 重构后的代码，保持测试绿色
    pub fn new(id: &str, name: &str, device_type: DeviceType) -> Self {
        let now = SystemTime::now();
        Self {
            id: id.to_string(),
            name: name.to_string(),
            device_type,
            status: DeviceStatus::Offline,
            capabilities: HashSet::new(),
            location: DeviceLocation::new(),
            performance: DevicePerformance::new(),
            metadata: DeviceMetadata::new(),
            created_at: now,
            last_updated: now,
            last_seen: now,
            connection_count: 0,
            trust_level: 0.5, // 默认中等信任度
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
    
    pub fn location(&self) -> &DeviceLocation {
        &self.location
    }
    
    pub fn performance(&self) -> &DevicePerformance {
        &self.performance
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
    
    pub fn last_seen(&self) -> SystemTime {
        self.last_seen
    }
    
    pub fn connection_count(&self) -> u32 {
        self.connection_count
    }
    
    pub fn trust_level(&self) -> f32 {
        self.trust_level
    }
    
    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.touch();
    }
    
    pub fn set_status(&mut self, status: DeviceStatus) {
        self.status = status;
        self.touch();
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
    
    pub fn has_capabilities(&self, required: &[DeviceCapability]) -> bool {
        required.iter().all(|cap| self.capabilities.contains(cap))
    }
    
    pub fn update_location(&mut self, location: DeviceLocation) {
        self.location = location;
        self.last_seen = SystemTime::now();
        self.touch();
    }
    
    pub fn update_performance(&mut self, performance: DevicePerformance) {
        self.performance = performance;
        self.touch();
    }
    
    pub fn update_metadata(&mut self, metadata: DeviceMetadata) {
        self.metadata = metadata;
        self.touch();
    }
    
    pub fn set_trust_level(&mut self, level: f32) {
        self.trust_level = level.clamp(0.0, 1.0);
        self.touch();
    }
    
    pub fn increment_connection_count(&mut self) {
        self.connection_count += 1;
        self.last_seen = SystemTime::now();
        self.touch();
    }
    
    pub fn is_online(&self) -> bool {
        matches!(self.status, DeviceStatus::Online) || 
        matches!(self.status, DeviceStatus::Idle) ||
        matches!(self.status, DeviceStatus::Busy)
    }
    
    pub fn is_reachable(&self) -> bool {
        self.is_online() && 
        (self.location.ip_address.is_some() || self.location.bluetooth_address.is_some())
    }
    
    pub fn is_trusted(&self, min_trust_level: f32) -> bool {
        self.trust_level >= min_trust_level
    }
    
    pub fn time_since_last_seen(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.last_seen)
            .unwrap_or(Duration::ZERO)
    }
    
    pub fn touch(&mut self) {
        self.last_updated = SystemTime::now();
    }
    
    pub fn generate_new_id() -> String {
        Uuid::new_v4().to_string()
    }
    
    pub fn calculate_compatibility_score(&self, other: &DeviceInfo) -> f32 {
        let mut score = 0.0;
        let mut max_score = 0.0;
        
        // 设备类型兼容性
        max_score += 1.0;
        if self.device_type == other.device_type {
            score += 1.0;
        } else if matches!(self.device_type, DeviceType::Mobile) && matches!(other.device_type, DeviceType::Mobile) {
            score += 0.8;
        }
        
        // 能力匹配度
        let common_capabilities: usize = self.capabilities.intersection(&other.capabilities).count();
        let total_capabilities = self.capabilities.len().max(other.capabilities.len());
        if total_capabilities > 0 {
            let capability_score = common_capabilities as f32 / total_capabilities as f32;
            score += capability_score * 2.0;
            max_score += 2.0;
        }
        
        // 状态兼容性
        max_score += 1.0;
        if self.is_online() && other.is_online() {
            score += 1.0;
        }
        
        // 信任度匹配
        let trust_diff = (self.trust_level - other.trust_level).abs();
        let trust_score = 1.0 - trust_diff;
        score += trust_score * 1.0;
        max_score += 1.0;
        
        if max_score > 0.0 {
            score / max_score
        } else {
            0.0
        }
    }
}

impl DeviceLocation {
    pub fn new() -> Self {
        Self {
            network_name: None,
            ip_address: None,
            mac_address: None,
            bluetooth_address: None,
            last_seen: SystemTime::now(),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.network_name.is_none() &&
        self.ip_address.is_none() &&
        self.mac_address.is_none() &&
        self.bluetooth_address.is_none()
    }
}

impl DevicePerformance {
    pub fn new() -> Self {
        Self {
            battery_level: None,
            signal_strength: None,
            bandwidth: None,
            latency: None,
            cpu_usage: None,
            memory_usage: None,
        }
    }
    
    pub fn is_healthy(&self) -> bool {
        self.battery_level.map_or(true, |level| level > 0.2) &&
        self.cpu_usage.map_or(true, |usage| usage < 0.9) &&
        self.memory_usage.map_or(true, |usage| usage < 0.9)
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
            hardware_id: None,
            custom_attributes: HashMap::new(),
        }
    }
    
    pub fn get_display_name(&self) -> String {
        if let (Some(manufacturer), Some(model)) = (&self.manufacturer, &self.model) {
            format!("{} {}", manufacturer, model)
        } else if let Some(model) = &self.model {
            model.clone()
        } else if let Some(manufacturer) = &self.manufacturer {
            manufacturer.clone()
        } else {
            "Unknown Device".to_string()
        }
    }
    
    pub fn add_custom_attribute(&mut self, key: String, value: String) {
        self.custom_attributes.insert(key, value);
    }
    
    pub fn get_custom_attribute(&self, key: &str) -> Option<&String> {
        self.custom_attributes.get(key)
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的设备实体：

```rust
// rust-core/domain/entities/device.rs
pub struct DeviceInfo {
    // 设备信息实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [内容变化检测](0403-content-change-detection.md)

## 后续任务

- [Task 0502: 实现设备存储管理](0502-device-storage-management.md)
- [Task 0503: 实现设备状态管理](0503-device-state-management.md)
- [Task 0504: 实现设备信任机制](0504-device-trust.md)