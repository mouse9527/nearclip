# Task 0303: 实现传输事件系统 (TDD版本)

## 任务描述

按照TDD原则实现文本传输的事件系统，为传输过程中的状态变化提供事件通知机制。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_event_tests.rs
#[cfg(test)]
mod transfer_event_tests {
    use super::*;
    
    #[test]
    fn test_transfer_started_event() {
        // RED: 测试传输开始事件
        let event = TransferEvent::transfer_started("session-123", "sender-001", "receiver-001");
        
        assert_eq!(event.session_id(), "session-123");
        assert_eq!(event.sender_id(), "sender-001");
        assert_eq!(event.receiver_id(), "receiver-001");
        assert!(matches!(event.event_type(), TransferEventType::TransferStarted));
    }
    
    #[test]
    fn test_progress_updated_event() {
        // RED: 测试进度更新事件
        let event = TransferEvent::progress_updated("session-123", 0.5, 1024);
        
        assert_eq!(event.session_id(), "session-123");
        assert_eq!(event.progress(), 0.5);
        assert_eq!(event.transferred_bytes(), 1024);
        assert!(matches!(event.event_type(), TransferEventType::ProgressUpdated));
    }
    
    #[test]
    fn test_transfer_completed_event() {
        // RED: 测试传输完成事件
        let event = TransferEvent::transfer_completed("session-123", 2048, std::time::Duration::from_secs(5));
        
        assert_eq!(event.session_id(), "session-123");
        assert_eq!(event.total_bytes(), 2048);
        assert_eq!(event.duration(), std::time::Duration::from_secs(5));
        assert!(matches!(event.event_type(), TransferEventType::TransferCompleted));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum TransferEventType {
    TransferStarted,
    ProgressUpdated,
    TransferCompleted,
    TransferFailed,
}

#[derive(Debug)]
pub struct TransferEvent {
    event_type: TransferEventType,
    session_id: String,
    progress: f32,
    transferred_bytes: u64,
}

impl TransferEvent {
    pub fn transfer_started(session_id: &str, sender_id: &str, receiver_id: &str) -> Self {
        Self {
            event_type: TransferEventType::TransferStarted,
            session_id: session_id.to_string(),
            progress: 0.0,
            transferred_bytes: 0,
        }
    }
    
    pub fn progress_updated(session_id: &str, progress: f32, transferred_bytes: u64) -> Self {
        Self {
            event_type: TransferEventType::ProgressUpdated,
            session_id: session_id.to_string(),
            progress,
            transferred_bytes,
        }
    }
    
    pub fn transfer_completed(session_id: &str, total_bytes: u64, duration: std::time::Duration) -> Self {
        Self {
            event_type: TransferEventType::TransferCompleted,
            session_id: session_id.to_string(),
            progress: 1.0,
            transferred_bytes: total_bytes,
        }
    }
    
    pub fn event_type(&self) -> &TransferEventType {
        &self.event_type
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    pub fn transferred_bytes(&self) -> u64 {
        self.transferred_bytes
    }
    
    pub fn total_bytes(&self) -> u64 {
        self.transferred_bytes // 简化实现
    }
    
    pub fn duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(0) // 简化实现
    }
    
    pub fn sender_id(&self) -> &str {
        "" // 简化实现
    }
    
    pub fn receiver_id(&self) -> &str {
        "" // 简化实现
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum TransferEventType {
    TransferStarted,
    ProgressUpdated,
    TransferCompleted,
    TransferFailed,
    TransferCancelled,
    SessionCreated,
    SessionDestroyed,
}

#[derive(Debug)]
pub struct TransferEvent {
    event_type: TransferEventType,
    session_id: String,
    payload: EventPayload,
    timestamp: SystemTime,
    metadata: EventMetadata,
}

#[derive(Debug)]
pub enum EventPayload {
    TransferStarted {
        sender_id: String,
        receiver_id: String,
        content_size: u64,
    },
    ProgressUpdated {
        progress: f32,
        transferred_bytes: u64,
        total_bytes: u64,
    },
    TransferCompleted {
        total_bytes: u64,
        duration: std::time::Duration,
        throughput: f64,
    },
    TransferFailed {
        error_code: String,
        error_message: String,
    },
    TransferCancelled {
        reason: String,
    },
    SessionCreated {
        initiator_id: String,
    },
    SessionDestroyed {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub sequence_number: u64,
    pub correlation_id: Option<String>,
    pub tags: Vec<String>,
}

impl TransferEvent {
    // 重构后的代码，保持测试绿色
    pub fn transfer_started(session_id: &str, sender_id: &str, receiver_id: &str) -> Self {
        Self::new(
            TransferEventType::TransferStarted,
            session_id,
            EventPayload::TransferStarted {
                sender_id: sender_id.to_string(),
                receiver_id: receiver_id.to_string(),
                content_size: 0, // 简化实现
            },
        )
    }
    
    pub fn progress_updated(session_id: &str, progress: f32, transferred_bytes: u64) -> Self {
        Self::new(
            TransferEventType::ProgressUpdated,
            session_id,
            EventPayload::ProgressUpdated {
                progress,
                transferred_bytes,
                total_bytes: transferred_bytes, // 简化实现
            },
        )
    }
    
    pub fn transfer_completed(session_id: &str, total_bytes: u64, duration: std::time::Duration) -> Self {
        Self::new(
            TransferEventType::TransferCompleted,
            session_id,
            EventPayload::TransferCompleted {
                total_bytes,
                duration,
                throughput: 0.0, // 简化实现
            },
        )
    }
    
    fn new(event_type: TransferEventType, session_id: &str, payload: EventPayload) -> Self {
        Self {
            event_type,
            session_id: session_id.to_string(),
            payload,
            timestamp: SystemTime::now(),
            metadata: EventMetadata {
                sequence_number: 0, // 简化实现
                correlation_id: None,
                tags: Vec::new(),
            },
        }
    }
    
    pub fn event_type(&self) -> &TransferEventType {
        &self.event_type
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    
    pub fn timestamp(&self) -> &SystemTime {
        &self.timestamp
    }
    
    pub fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    pub fn progress(&self) -> f32 {
        match &self.payload {
            EventPayload::ProgressUpdated { progress, .. } => *progress,
            EventPayload::TransferCompleted { .. } => 1.0,
            EventPayload::TransferStarted { .. } => 0.0,
            _ => 0.0,
        }
    }
    
    pub fn transferred_bytes(&self) -> u64 {
        match &self.payload {
            EventPayload::ProgressUpdated { transferred_bytes, .. } => *transferred_bytes,
            EventPayload::TransferCompleted { total_bytes, .. } => *total_bytes,
            _ => 0,
        }
    }
    
    pub fn total_bytes(&self) -> u64 {
        match &self.payload {
            EventPayload::ProgressUpdated { total_bytes, .. } => *total_bytes,
            EventPayload::TransferCompleted { total_bytes, .. } => *total_bytes,
            EventPayload::TransferStarted { content_size, .. } => *content_size,
            _ => 0,
        }
    }
    
    pub fn duration(&self) -> std::time::Duration {
        match &self.payload {
            EventPayload::TransferCompleted { duration, .. } => *duration,
            _ => std::time::Duration::from_secs(0),
        }
    }
    
    pub fn sender_id(&self) -> &str {
        match &self.payload {
            EventPayload::TransferStarted { sender_id, .. } => sender_id,
            _ => "",
        }
    }
    
    pub fn receiver_id(&self) -> &str {
        match &self.payload {
            EventPayload::TransferStarted { receiver_id, .. } => receiver_id,
            _ => "",
        }
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.event_type,
            TransferEventType::TransferCompleted
                | TransferEventType::TransferFailed
                | TransferEventType::TransferCancelled
        )
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的事件结构：

```rust
// rust-core/domain/events/transfer.rs
#[derive(Debug)]
pub struct TransferEvent {
    // 传输事件核心属性
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [传输错误处理](0302-transfer-error-handling.md)

## 后续任务

- [Task 0304: 实现传输队列管理](0304-transfer-queue-management.md)
- [Task 0305: 实现传输协议处理](0305-transfer-protocol-handling.md)
- [Task 0306: 实现传输进度跟踪](0306-transfer-progress-tracking.md)