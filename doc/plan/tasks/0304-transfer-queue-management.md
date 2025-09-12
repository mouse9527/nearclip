# Task 0304: 实现传输队列管理 (TDD版本)

## 任务描述

按照TDD原则实现文本传输的队列管理，处理多个传输任务的排队和调度。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_queue_tests.rs
#[cfg(test)]
mod transfer_queue_tests {
    use super::*;
    
    #[test]
    fn test_queue_enqueue_dequeue() {
        // RED: 测试队列入队出队
        let mut queue = TransferQueue::new();
        let session = create_test_session("session-1");
        
        let result = queue.enqueue(session);
        assert!(result.is_ok());
        
        let dequeued = queue.dequeue().unwrap();
        assert_eq!(dequeued.session_id(), "session-1");
    }
    
    #[test]
    fn test_queue_priority_ordering() {
        // RED: 测试优先级排序
        let mut queue = TransferQueue::new();
        let low_priority = create_test_session("low").with_priority(TransferPriority::Low);
        let high_priority = create_test_session("high").with_priority(TransferPriority::High);
        
        queue.enqueue(low_priority).unwrap();
        queue.enqueue(high_priority).unwrap();
        
        // 高优先级应该先出队
        let first = queue.dequeue().unwrap();
        assert_eq!(first.session_id(), "high");
        
        let second = queue.dequeue().unwrap();
        assert_eq!(second.session_id(), "low");
    }
    
    #[test]
    fn test_queue_capacity_limit() {
        // RED: 测试队列容量限制
        let mut queue = TransferQueue::with_capacity(2);
        
        // 填满队列
        queue.enqueue(create_test_session("1")).unwrap();
        queue.enqueue(create_test_session("2")).unwrap();
        
        // 尝试超过容量
        let result = queue.enqueue(create_test_session("3"));
        assert!(matches!(result, Err(TransferQueueError::QueueFull)));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug)]
pub struct TransferSession {
    session_id: String,
    priority: TransferPriority,
}

impl TransferSession {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            priority: TransferPriority::Normal,
        }
    }
    
    pub fn with_priority(mut self, priority: TransferPriority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

#[derive(Debug)]
pub enum TransferQueueError {
    QueueFull,
    QueueEmpty,
}

#[derive(Debug)]
pub struct TransferQueue {
    heap: BinaryHeap<QueueItem>,
    capacity: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct QueueItem {
    session: TransferSession,
    priority: TransferPriority,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl TransferQueue {
    pub fn new() -> Self {
        Self::with_capacity(100)
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::new(),
            capacity,
        }
    }
    
    pub fn enqueue(&mut self, session: TransferSession) -> Result<(), TransferQueueError> {
        if self.heap.len() >= self.capacity {
            return Err(TransferQueueError::QueueFull);
        }
        
        self.heap.push(QueueItem {
            session,
            priority: TransferPriority::Normal,
        });
        Ok(())
    }
    
    pub fn dequeue(&mut self) -> Result<TransferSession, TransferQueueError> {
        self.heap.pop()
            .map(|item| item.session)
            .ok_or(TransferQueueError::QueueEmpty)
    }
}

// 测试辅助函数
fn create_test_session(session_id: &str) -> TransferSession {
    TransferSession::new(session_id)
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::collections::BinaryHeap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransferPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Urgent = 4,
}

#[derive(Debug)]
pub struct TransferQueue {
    heap: BinaryHeap<QueueItem>,
    capacity: usize,
    metrics: QueueMetrics,
    config: QueueConfig,
}

#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub max_capacity: usize,
    pub max_retry_count: u32,
    pub timeout_duration: Duration,
    pub priority_boost_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct QueueMetrics {
    pub enqueued_count: u64,
    pub dequeued_count: u64,
    pub failed_count: u64,
    pub average_wait_time: Duration,
}

#[derive(Debug, Eq, PartialEq)]
struct QueueItem {
    session: TransferSession,
    priority: TransferPriority,
    enqueued_at: SystemTime,
    retry_count: u32,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 优先级为主，等待时间为辅
        self.priority
            .cmp(&other.priority)
            .reverse()
            .then(self.enqueued_at.cmp(&other.enqueued_at))
    }
}

impl TransferQueue {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_config(QueueConfig::default())
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        let mut config = QueueConfig::default();
        config.max_capacity = capacity;
        Self::with_config(config)
    }
    
    pub fn with_config(config: QueueConfig) -> Self {
        Self {
            heap: BinaryHeap::new(),
            capacity: config.max_capacity,
            metrics: QueueMetrics::new(),
            config,
        }
    }
    
    pub fn enqueue(&mut self, session: TransferSession) -> Result<(), TransferQueueError> {
        if self.heap.len() >= self.capacity {
            return Err(TransferQueueError::QueueFull);
        }
        
        self.heap.push(QueueItem {
            session,
            priority: TransferPriority::Normal,
            enqueued_at: SystemTime::now(),
            retry_count: 0,
        });
        
        self.metrics.enqueued_count += 1;
        Ok(())
    }
    
    pub fn enqueue_with_priority(&mut self, session: TransferSession, priority: TransferPriority) -> Result<(), TransferQueueError> {
        if self.heap.len() >= self.capacity {
            return Err(TransferQueueError::QueueFull);
        }
        
        self.heap.push(QueueItem {
            session,
            priority,
            enqueued_at: SystemTime::now(),
            retry_count: 0,
        });
        
        self.metrics.enqueued_count += 1;
        Ok(())
    }
    
    pub fn dequeue(&mut self) -> Result<TransferSession, TransferQueueError> {
        let item = self.heap.pop().ok_or(TransferQueueError::QueueEmpty)?;
        self.metrics.dequeued_count += 1;
        
        // 更新平均等待时间
        if let Ok(wait_time) = item.enqueued_at.elapsed() {
            self.metrics.update_average_wait_time(wait_time);
        }
        
        Ok(item.session)
    }
    
    pub fn peek(&self) -> Option<&TransferSession> {
        self.heap.peek().map(|item| &item.session)
    }
    
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    
    pub fn metrics(&self) -> &QueueMetrics {
        &self.metrics
    }
    
    pub fn retry_failed(&mut self, session: TransferSession) -> Result<(), TransferQueueError> {
        if self.heap.len() >= self.capacity {
            return Err(TransferQueueError::QueueFull);
        }
        
        let mut retry_count = 1;
        if let Some(item) = self.heap.iter().find(|item| item.session.session_id() == session.session_id()) {
            retry_count = item.retry_count + 1;
        }
        
        if retry_count > self.config.max_retry_count {
            return Err(TransferQueueError::MaxRetriesExceeded);
        }
        
        self.heap.push(QueueItem {
            session,
            priority: TransferPriority::High, // 重试任务提升优先级
            enqueued_at: SystemTime::now(),
            retry_count,
        });
        
        self.metrics.enqueued_count += 1;
        Ok(())
    }
    
    pub fn remove_timed_out(&mut self) -> Vec<TransferSession> {
        let now = SystemTime::now();
        let mut removed = Vec::new();
        
        self.heap.retain(|item| {
            if now.duration_since(item.enqueued_at) > self.config.timeout_duration {
                removed.push(item.session.clone());
                false
            } else {
                true
            }
        });
        
        self.metrics.failed_count += removed.len() as u64;
        removed
    }
}

impl QueueMetrics {
    fn new() -> Self {
        Self {
            enqueued_count: 0,
            dequeued_count: 0,
            failed_count: 0,
            average_wait_time: Duration::from_secs(0),
        }
    }
    
    fn update_average_wait_time(&mut self, new_wait_time: Duration) {
        let total_wait = self.average_wait_time * self.dequeued_count as u32 + new_wait_time;
        self.average_wait_time = total_wait / (self.dequeued_count as u32 + 1);
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_capacity: 100,
            max_retry_count: 3,
            timeout_duration: Duration::from_secs(300), // 5分钟
            priority_boost_enabled: true,
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为infrastructure层的队列管理：

```rust
// rust-core/infrastructure/queue/transfer_queue.rs
pub struct TransferQueue {
    // 传输队列实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [传输事件系统](0303-transfer-event-system.md)

## 后续任务

- Task 0305: 实现传输协议处理
- Task 0306: 实现传输进度跟踪
- Task 0307: 实现传输状态监控