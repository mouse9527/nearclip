# Task 00106: 定义设备发现trait接口

## 任务描述

按照TDD原则定义设备发现的trait接口，为不同发现方式提供统一抽象。

## TDD开发要求

### RED阶段 - 编写失败的测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{Device, DeviceType};

    #[test]
    fn test_mock_discovery_start_stop() {
        // RED: 测试模拟发现器启动停止
        let mut discovery = MockDiscovery::new();
        
        assert!(!discovery.is_active());
        
        let result = discovery.start();
        assert!(result.is_ok());
        assert!(discovery.is_active());
        
        let result = discovery.stop();
        assert!(result.is_ok());
        assert!(!discovery.is_active());
    }

    #[test]
    fn test_mock_discovery_get_devices() {
        // RED: 测试获取发现的设备
        let mut discovery = MockDiscovery::new();
        discovery.start().unwrap();
        
        let devices = discovery.get_discovered_devices();
        assert!(devices.is_empty());
        
        // 添加模拟设备
        let device = Device::new("test-id".to_string(), "Test Device".to_string(), DeviceType::Phone);
        discovery.add_mock_device(device.clone());
        
        let devices = discovery.get_discovered_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id(), "test-id");
    }
}
```

### GREEN阶段 - 最小实现
```rust
#[derive(Debug)]
pub enum DiscoveryError {
    AlreadyStarted,
    NotStarted,
    StartFailed(String),
    StopFailed(String),
    DeviceError(String),
}

pub trait DeviceDiscovery: Send + Sync {
    fn start(&mut self) -> Result<(), DiscoveryError>;
    fn stop(&mut self) -> Result<(), DiscoveryError>;
    fn is_active(&self) -> bool;
    fn get_discovered_devices(&self) -> Vec<Device>;
}

// Mock implementation for testing
#[derive(Debug)]
pub struct MockDiscovery {
    is_active: bool,
    devices: Vec<Device>,
}

impl MockDiscovery {
    pub fn new() -> Self {
        Self {
            is_active: false,
            devices: Vec::new(),
        }
    }
    
    pub fn add_mock_device(&mut self, device: Device) {
        self.devices.push(device);
    }
}

impl DeviceDiscovery for MockDiscovery {
    fn start(&mut self) -> Result<(), DiscoveryError> {
        if self.is_active {
            return Err(DiscoveryError::AlreadyStarted);
        }
        self.is_active = true;
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), DiscoveryError> {
        if !self.is_active {
            return Err(DiscoveryError::NotStarted);
        }
        self.is_active = false;
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        self.is_active
    }
    
    fn get_discovered_devices(&self) -> Vec<Device> {
        self.devices.clone()
    }
}
```

### REFACTOR阶段
```rust
// 添加更多方法到trait
pub trait DeviceDiscovery: Send + Sync {
    fn start(&mut self) -> Result<(), DiscoveryError>;
    fn stop(&mut self) -> Result<(), DiscoveryError>;
    fn is_active(&self) -> bool;
    fn get_discovered_devices(&self) -> Vec<Device>;
    fn get_device_count(&self) -> usize {
        self.get_discovered_devices().len()
    }
    fn has_device(&self, device_id: &str) -> bool {
        self.get_discovered_devices()
            .iter()
            .any(|d| d.id() == device_id)
    }
    fn clear_devices(&mut self) {
        // Default implementation does nothing
    }
}

impl DeviceDiscovery for MockDiscovery {
    fn clear_devices(&mut self) {
        self.devices.clear();
    }
}
```

## 验收标准
- [ ] 所有测试通过
- [ ] trait接口定义完整
- [ ] Mock实现工作正常

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00107: 定义发现服务配置结构](00107-discovery-config-structure.md)