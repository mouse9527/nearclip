# Task 0301: 定义传输方向枚举 (TDD版本)

## 任务描述

按照TDD原则定义传输方向枚举，用于标识文本传输是发送还是接收。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_direction_tests.rs
#[cfg(test)]
mod transfer_direction_tests {
    use super::*;
    
    #[test]
    fn test_direction_display() {
        // RED: 测试传输方向显示
        let direction = TransferDirection::Send;
        assert_eq!(format!("{}", direction), "Send");
        
        let direction = TransferDirection::Receive;
        assert_eq!(format!("{}", direction), "Receive");
    }
    
    #[test]
    fn test_direction_is_send() {
        // RED: 测试发送方向判断
        assert!(TransferDirection::Send.is_send());
        assert!(!TransferDirection::Receive.is_send());
    }
    
    #[test]
    fn test_direction_is_receive() {
        // RED: 测试接收方向判断
        assert!(TransferDirection::Receive.is_receive());
        assert!(!TransferDirection::Send.is_receive());
    }
    
    #[test]
    fn test_direction_opposite() {
        // RED: 测试方向反转
        assert_eq!(TransferDirection::Send.opposite(), TransferDirection::Receive);
        assert_eq!(TransferDirection::Receive.opposite(), TransferDirection::Send);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransferDirection {
    Send,
    Receive,
}

use std::fmt;

impl fmt::Display for TransferDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferDirection::Send => write!(f, "Send"),
            TransferDirection::Receive => write!(f, "Receive"),
        }
    }
}

impl TransferDirection {
    pub fn is_send(&self) -> bool {
        matches!(self, TransferDirection::Send)
    }
    
    pub fn is_receive(&self) -> bool {
        matches!(self, TransferDirection::Receive)
    }
    
    pub fn opposite(&self) -> TransferDirection {
        match self {
            TransferDirection::Send => TransferDirection::Receive,
            TransferDirection::Receive => TransferDirection::Send,
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TransferDirection {
    Send,
    Receive,
}

impl TransferDirection {
    pub fn is_send(&self) -> bool {
        matches!(self, TransferDirection::Send)
    }
    
    pub fn is_receive(&self) -> bool {
        matches!(self, TransferDirection::Receive)
    }
    
    pub fn opposite(&self) -> TransferDirection {
        match self {
            TransferDirection::Send => TransferDirection::Receive,
            TransferDirection::Receive => TransferDirection::Send,
        }
    }
    
    pub fn emoji(&self) -> &'static str {
        match self {
            TransferDirection::Send => "📤",
            TransferDirection::Receive => "📥",
        }
    }
    
    pub fn action_verb(&self) -> &'static str {
        match self {
            TransferDirection::Send => "sending",
            TransferDirection::Receive => "receiving",
        }
    }
    
    pub fn for_device(&self, device_id: &str, other_device_id: &str) -> (&str, &str) {
        match self {
            TransferDirection::Send => (device_id, other_device_id),
            TransferDirection::Receive => (other_device_id, device_id),
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的基础枚举：

```rust
// rust-core/domain/entities/transfer.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TransferDirection {
    // 传输方向定义
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- 无

## 后续任务

- [Task 0302: 定义传输状态枚举](0302-transfer-status-enum.md)
- [Task 0303: 定义传输优先级枚举](0303-transfer-priority-enum.md)