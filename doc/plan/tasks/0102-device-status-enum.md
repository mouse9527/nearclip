# Task 0102: 定义设备状态枚举 (TDD版本)

## 任务描述

按照TDD原则定义设备状态枚举，用于跟踪设备的连接和运行状态。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_status_tests.rs
#[cfg(test)]
mod device_status_tests {
    use super::*;
    
    #[test]
    fn test_device_status_creation() {
        // RED: 测试设备状态创建
        let status = DeviceStatus::Online;
        assert_eq!(format!("{}", status), "Online");
        
        let status = DeviceStatus::Offline;
        assert_eq!(format!("{}", status), "Offline");
    }
    
    #[test]
    fn test_device_status_transitions() {
        // RED: 测试状态转换验证
        let current = DeviceStatus::Offline;
        let target = DeviceStatus::Online;
        
        assert!(current.can_transition_to(&target));
        
        let current = DeviceStatus::Online;
        let target = DeviceStatus::Offline;
        assert!(current.can_transition_to(&target));
    }
    
    #[test]
    fn test_invalid_transitions() {
        // RED: 测试无效状态转换
        let current = DeviceStatus::Connecting;
        let target = DeviceStatus::Connecting;
        assert!(!current.can_transition_to(&target)); // 不能从连接中状态转换到连接中状态
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    Online,
    Offline,
    Connecting,
    Disconnecting,
}

use std::fmt;

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Online => write!(f, "Online"),
            DeviceStatus::Offline => write!(f, "Offline"),
            DeviceStatus::Connecting => write!(f, "Connecting"),
            DeviceStatus::Disconnecting => write!(f, "Disconnecting"),
        }
    }
}

impl DeviceStatus {
    pub fn can_transition_to(&self, target: &DeviceStatus) -> bool {
        use DeviceStatus::*;
        match (self, target) {
            (Offline, Connecting) => true,
            (Connecting, Online) => true,
            (Online, Disconnecting) => true,
            (Disconnecting, Offline) => true,
            (Online, Offline) => true,
            (Offline, Online) => true,
            _ => false,
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceStatus {
    Online,
    Offline,
    Connecting,
    Disconnecting,
    Busy,
    Error(String),
    Maintenance,
}

impl DeviceStatus {
    pub fn is_connected(&self) -> bool {
        matches!(self, DeviceStatus::Online | DeviceStatus::Busy)
    }
    
    pub fn is_transitional(&self) -> bool {
        matches!(self, DeviceStatus::Connecting | DeviceStatus::Disconnecting)
    }
    
    pub fn is_available(&self) -> bool {
        matches!(self, DeviceStatus::Online | DeviceStatus::Offline)
    }
    
    pub fn can_transition_to(&self, target: &DeviceStatus) -> bool {
        use DeviceStatus::*;
        match (self, target) {
            // 基本连接状态转换
            (Offline, Connecting) => true,
            (Connecting, Online) => true,
            (Connecting, Error(_)) => true,
            (Online, Disconnecting) => true,
            (Disconnecting, Offline) => true,
            (Disconnecting, Error(_)) => true,
            
            // 直接状态转换
            (Online, Offline) => true,
            (Offline, Online) => true,
            (Online, Busy) => true,
            (Busy, Online) => true,
            
            // 错误状态恢复
            (Error(_), Offline) => true,
            (Error(_), Maintenance) => true,
            
            // 维护状态
            (Maintenance, Offline) => true,
            
            _ => false,
        }
    }
    
    pub fn is_stable(&self) -> bool {
        !self.is_transitional() && !matches!(self, DeviceStatus::Error(_))
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的基础枚举：

```rust
// rust-core/domain/entities/device_status.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceStatus {
    // 设备状态定义
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [Task 0101: 定义设备类型枚举](0101-device-enum-definition.md)

## 后续任务

- [Task 0103: 定义设备能力枚举](0103-device-capability-enum.md)
- [Task 0104: 定义设备错误类型](0104-device-error-types.md)