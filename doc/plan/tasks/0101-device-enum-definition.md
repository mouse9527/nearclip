# Task 0101: 定义设备类型枚举 (TDD版本)

## 任务描述

按照TDD原则定义设备的基本类型枚举，这是设备抽象层的基础。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_enum_tests.rs
#[cfg(test)]
mod device_enum_tests {
    use super::*;
    
    #[test]
    fn test_device_type_display() {
        // RED: 测试设备类型显示
        let device_type = DeviceType::Mobile;
        assert_eq!(format!("{}", device_type), "Mobile");
        
        let device_type = DeviceType::Desktop;
        assert_eq!(format!("{}", device_type), "Desktop");
    }
    
    #[test]
    fn test_device_type_from_str() {
        // RED: 测试从字符串解析设备类型
        assert_eq!("mobile".parse::<DeviceType>().unwrap(), DeviceType::Mobile);
        assert_eq!("desktop".parse::<DeviceType>().unwrap(), DeviceType::Desktop);
        assert!("unknown".parse::<DeviceType>().is_err());
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

use std::fmt;
use std::str::FromStr;

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Mobile => write!(f, "Mobile"),
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Tablet => write!(f, "Tablet"),
        }
    }
}

impl FromStr for DeviceType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mobile" => Ok(DeviceType::Mobile),
            "desktop" => Ok(DeviceType::Desktop),
            "tablet" => Ok(DeviceType::Tablet),
            _ => Err(format!("Unknown device type: {}", s)),
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tablet,
    Wearable,
    TV,
    Embedded,
    Unknown,
}

impl DeviceType {
    pub fn is_mobile(&self) -> bool {
        matches!(self, DeviceType::Mobile | DeviceType::Tablet | DeviceType::Wearable)
    }
    
    pub fn is_fixed(&self) -> bool {
        matches!(self, DeviceType::Desktop | DeviceType::TV | DeviceType::Embedded)
    }
    
    pub fn supports_ble(&self) -> bool {
        matches!(self, DeviceType::Mobile | DeviceType::Tablet | DeviceType::Wearable)
    }
    
    pub fn supports_wifi(&self) -> bool {
        !matches!(self, DeviceType::Embedded) // 嵌入式设备可能不支持WiFi
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的基础枚举：

```rust
// rust-core/domain/entities/device_types.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceType {
    // 设备类型定义
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- 无

## 后续任务

- [Task 0102: 定义设备状态枚举](0102-device-status-enum.md)
- [Task 0103: 定义设备能力枚举](0103-device-capability-enum.md)