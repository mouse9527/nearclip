# Task 0301: 定义文本传输会话 (TDD版本)

## 任务描述

按照TDD原则定义文本传输会话的基础结构，为文本传输建立会话管理基础。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_session_tests.rs
#[cfg(test)]
mod transfer_session_tests {
    use super::*;
    
    #[test]
    fn test_transfer_session_creation() {
        // RED: 测试传输会话创建
        let session_id = "test-session-123";
        let sender_id = "sender-device";
        let receiver_id = "receiver-device";
        let content = "Hello, World!";
        
        let session = TransferSession::new(session_id, sender_id, receiver_id, content);
        
        assert_eq!(session.session_id(), session_id);
        assert_eq!(session.sender_id(), sender_id);
        assert_eq!(session.receiver_id(), receiver_id);
        assert_eq!(session.content(), content);
        assert_eq!(session.status(), &TransferStatus::Created);
        assert_eq!(session.progress(), 0.0);
    }
    
    #[test]
    fn test_session_status_transitions() {
        // RED: 测试会话状态转换
        let mut session = TransferSession::new("session-123", "sender", "receiver", "test content");
        
        // 初始状态
        assert_eq!(session.status(), &TransferStatus::Created);
        
        // 开始传输
        session.start_transfer().unwrap();
        assert_eq!(session.status(), &TransferStatus::InProgress);
        
        // 完成传输
        session.complete_transfer().unwrap();
        assert_eq!(session.status(), &TransferStatus::Completed);
        assert!(session.completion_time().is_some());
    }
    
    #[test]
    fn test_progress_tracking() {
        // RED: 测试进度跟踪
        let mut session = TransferSession::new("session-123", "sender", "receiver", "test content");
        
        // 初始进度
        assert_eq!(session.progress(), 0.0);
        assert_eq!(session.transferred_bytes(), 0);
        
        // 更新进度
        session.update_progress(0.5, 50).unwrap();
        assert_eq!(session.progress(), 0.5);
        assert_eq!(session.transferred_bytes(), 50);
        
        // 完成进度
        session.update_progress(1.0, 100).unwrap();
        assert_eq!(session.progress(), 1.0);
        assert!(session.is_completed());
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Created,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug)]
pub struct TransferSession {
    session_id: String,
    sender_id: String,
    receiver_id: String,
    content: String,
    status: TransferStatus,
    progress: f32,
    transferred_bytes: u64,
    created_at: SystemTime,
    completion_time: Option<SystemTime>,
}

impl TransferSession {
    pub fn new(session_id: &str, sender_id: &str, receiver_id: &str, content: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
            content: content.to_string(),
            status: TransferStatus::Created,
            progress: 0.0,
            transferred_bytes: 0,
            created_at: SystemTime::now(),
            completion_time: None,
        }
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    
    pub fn sender_id(&self) -> &str {
        &self.sender_id
    }
    
    pub fn receiver_id(&self) -> &str {
        &self.receiver_id
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
    
    pub fn status(&self) -> &TransferStatus {
        &self.status
    }
    
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    pub fn transferred_bytes(&self) -> u64 {
        self.transferred_bytes
    }
    
    pub fn completion_time(&self) -> Option<&SystemTime> {
        self.completion_time.as_ref()
    }
    
    pub fn start_transfer(&mut self) -> Result<(), TransferError> {
        if self.status != TransferStatus::Created {
            return Err(TransferError::InvalidStateTransition);
        }
        
        self.status = TransferStatus::InProgress;
        Ok(())
    }
    
    pub fn complete_transfer(&mut self) -> Result<(), TransferError> {
        if self.status != TransferStatus::InProgress {
            return Err(TransferError::InvalidStateTransition);
        }
        
        self.status = TransferStatus::Completed;
        self.completion_time = Some(SystemTime::now());
        Ok(())
    }
    
    pub fn update_progress(&mut self, progress: f32, transferred_bytes: u64) -> Result<(), TransferError> {
        if !(0.0..=1.0).contains(&progress) {
            return Err(TransferError::InvalidProgress);
        }
        
        self.progress = progress;
        self.transferred_bytes = transferred_bytes;
        Ok(())
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TransferStatus::Completed)
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct TransferSession {
    session_id: String,
    sender_id: String,
    receiver_id: String,
    content: String,
    status: TransferStatus,
    progress: TransferProgress,
    timing: TransferTiming,
    metadata: TransferMetadata,
}

#[derive(Debug, Clone)]
pub struct TransferProgress {
    pub percentage: f32,
    pub transferred_bytes: u64,
    pub total_bytes: u64,
    pub chunks_completed: u32,
    pub total_chunks: u32,
}

#[derive(Debug)]
pub struct TransferTiming {
    pub created_at: SystemTime,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub last_progress_update: SystemTime,
}

#[derive(Debug, Clone)]
pub struct TransferMetadata {
    pub content_type: String,
    pub content_hash: Option<String>,
    pub retry_count: u32,
    pub max_retries: u32,
}

impl TransferSession {
    // 重构后的代码，保持测试绿色
    pub fn new(session_id: &str, sender_id: &str, receiver_id: &str, content: &str) -> Self {
        let content_bytes = content.as_bytes();
        
        Self {
            session_id: session_id.to_string(),
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
            content: content.to_string(),
            status: TransferStatus::Created,
            progress: TransferProgress {
                percentage: 0.0,
                transferred_bytes: 0,
                total_bytes: content_bytes.len() as u64,
                chunks_completed: 0,
                total_chunks: Self::calculate_chunks(content_bytes.len()),
            },
            timing: TransferTiming {
                created_at: SystemTime::now(),
                started_at: None,
                completed_at: None,
                last_progress_update: SystemTime::now(),
            },
            metadata: TransferMetadata {
                content_type: "text/plain".to_string(),
                content_hash: Some(Self::calculate_hash(content_bytes)),
                retry_count: 0,
                max_retries: 3,
            },
        }
    }
    
    fn calculate_chunks(content_size: usize) -> u32 {
        const CHUNK_SIZE: usize = 4096;
        ((content_size + CHUNK_SIZE - 1) / CHUNK_SIZE) as u32
    }
    
    fn calculate_hash(content: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
    
    pub fn start_transfer(&mut self) -> Result<(), TransferError> {
        self.validate_state_transition(&TransferStatus::InProgress)?;
        
        self.status = TransferStatus::InProgress;
        self.timing.started_at = Some(SystemTime::now());
        Ok(())
    }
    
    pub fn complete_transfer(&mut self) -> Result<(), TransferError> {
        self.validate_state_transition(&TransferStatus::Completed)?;
        
        self.status = TransferStatus::Completed;
        self.timing.completed_at = Some(SystemTime::now());
        self.progress.percentage = 1.0;
        self.progress.transferred_bytes = self.progress.total_bytes;
        self.progress.chunks_completed = self.progress.total_chunks;
        Ok(())
    }
    
    pub fn update_progress(&mut self, percentage: f32, transferred_bytes: u64) -> Result<(), TransferError> {
        if !(0.0..=1.0).contains(&percentage) {
            return Err(TransferError::InvalidProgress);
        }
        
        if transferred_bytes > self.progress.total_bytes {
            return Err(TransferError::InvalidByteCount);
        }
        
        self.progress.percentage = percentage;
        self.progress.transferred_bytes = transferred_bytes;
        self.timing.last_progress_update = SystemTime::now();
        Ok(())
    }
    
    pub fn update_chunk_progress(&mut self, chunks_completed: u32) -> Result<(), TransferError> {
        if chunks_completed > self.progress.total_chunks {
            return Err(TransferError::InvalidChunkCount);
        }
        
        self.progress.chunks_completed = chunks_completed;
        self.progress.percentage = chunks_completed as f32 / self.progress.total_chunks as f32;
        self.progress.transferred_bytes = (chunks_completed as u64 * 4096).min(self.progress.total_bytes);
        
        Ok(())
    }
    
    fn validate_state_transition(&self, new_status: &TransferStatus) -> Result<(), TransferError> {
        use TransferStatus::*;
        
        let is_valid = match (&self.status, new_status) {
            (Created, InProgress) => true,
            (InProgress, Completed) => true,
            (InProgress, Failed(_)) => true,
            (InProgress, Cancelled) => true,
            (_, Failed(_)) => true, // 任何状态都可以转到失败
            (_, Cancelled) => true, // 任何状态都可以被取消
            _ => false,
        };
        
        if !is_valid {
            return Err(TransferError::InvalidStateTransition);
        }
        
        Ok(())
    }
    
    pub fn duration(&self) -> Option<std::time::Duration> {
        self.timing.started_at.map(|started| {
            let end = self.timing.completed_at.unwrap_or(SystemTime::now());
            end.duration_since(started).unwrap_or(std::time::Duration::from_secs(0))
        })
    }
    
    pub fn throughput(&self) -> Option<f64> {
        self.duration().map(|duration| {
            let seconds = duration.as_secs_f64();
            if seconds > 0.0 {
                self.progress.transferred_bytes as f64 / seconds
            } else {
                0.0
            }
        })
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的核心实体：

```rust
// rust-core/domain/entities/transfer.rs
#[derive(Debug)]
pub struct TransferSession {
    // 传输会话核心属性
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [设备抽象层](../tasks/0101-device-abstraction-layer.md)

## 后续任务

- [Task 0302: 实现传输错误处理](0302-transfer-error-handling.md)
- [Task 0303: 实现传输事件系统](0303-transfer-event-system.md)
- [Task 0304: 实现传输队列管理](0304-transfer-queue-management.md)