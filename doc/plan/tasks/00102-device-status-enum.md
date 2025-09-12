# Task 00102: 定义设备状态枚举

## 任务描述

按照TDD原则定义设备状态枚举，表示设备的不同连接和发现状态。

## TDD开发要求

### RED阶段 - 编写失败的测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_status_display() {
        // RED: 测试设备状态显示
        let status = DeviceStatus::Online;
        assert_eq!(status.display(), "Online");
        
        let error_status = DeviceStatus::Error("Network failed".to_string());
        assert_eq!(error_status.display(), "Error: Network failed");
    }

    #[test]
    fn test_device_status_properties() {
        // RED: 测试设备状态属性
        assert!(DeviceStatus::Online.is_connected());
        assert!(DeviceStatus::Connecting.is_connecting());
        assert!(DeviceStatus::Offline.is_disconnected());
        assert!(DeviceStatus::Error("test".to_string()).is_error());
    }
}
```

### GREEN阶段 - 最小实现
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStatus {
    Offline,
    Online,
    Connecting,
    Connected,
    Error(String),
}

impl DeviceStatus {
    pub fn display(&self) -> String {
        match self {
            DeviceStatus::Offline => "Offline".to_string(),
            DeviceStatus::Online => "Online".to_string(),
            DeviceStatus::Connecting => "Connecting".to_string(),
            DeviceStatus::Connected => "Connected".to_string(),
            DeviceStatus::Error(msg) => format!("Error: {}", msg),
        }
    }
    
    pub fn is_connected(&self) -> bool {
        matches!(self, DeviceStatus::Online | DeviceStatus::Connected)
    }
    
    pub fn is_connecting(&self) -> bool {
        matches!(self, DeviceStatus::Connecting)
    }
    
    pub fn is_disconnected(&self) -> bool {
        matches!(self, DeviceStatus::Offline | DeviceStatus::Error(_))
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, DeviceStatus::Error(_))
    }
    
    pub fn error_message(&self) -> Option<&str> {
        match self {
            DeviceStatus::Error(msg) => Some(msg),
            _ => None,
        }
    }
}
```

### REFACTOR阶段
```rust
// 代码已经简洁，无需重构
```

## 验收标准
- [ ] 所有测试通过
- [ ] 状态定义完整
- [ ] 状态方法工作正常

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00103: 定义设备能力枚举](00103-device-capability-enum.md)