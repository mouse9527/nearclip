# Task 0306: 实现传输进度跟踪 (TDD版本)

## 任务描述

按照TDD原则实现传输进度跟踪，为用户实时显示传输进度和剩余时间。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_progress_tests.rs
#[cfg(test)]
mod transfer_progress_tests {
    use super::*;
    
    #[test]
    fn test_progress_tracker_creation() {
        // RED: 测试进度跟踪器创建
        let tracker = ProgressTracker::new(1000); // 1000字节总大小
        
        assert_eq!(tracker.total_size(), 1000);
        assert_eq!(tracker.transferred(), 0);
        assert_eq!(tracker.progress(), 0.0);
        assert_eq!(tracker.remaining_bytes(), 1000);
    }
    
    #[test]
    fn test_progress_update() {
        // RED: 测试进度更新
        let mut tracker = ProgressTracker::new(1000);
        
        // 传输500字节
        tracker.update_progress(500).unwrap();
        
        assert_eq!(tracker.transferred(), 500);
        assert_eq!(tracker.progress(), 0.5);
        assert_eq!(tracker.remaining_bytes(), 500);
        assert!(tracker.is_in_progress());
    }
    
    #[test]
    fn test_transfer_completion() {
        // RED: 测试传输完成
        let mut tracker = ProgressTracker::new(1000);
        
        // 完成传输
        tracker.update_progress(1000).unwrap();
        
        assert_eq!(tracker.transferred(), 1000);
        assert_eq!(tracker.progress(), 1.0);
        assert_eq!(tracker.remaining_bytes(), 0);
        assert!(tracker.is_completed());
        assert!(tracker.end_time().is_some());
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::SystemTime;

#[derive(Debug)]
pub struct ProgressTracker {
    total_size: u64,
    transferred: u64,
    start_time: SystemTime,
    end_time: Option<SystemTime>,
}

impl ProgressTracker {
    pub fn new(total_size: u64) -> Self {
        Self {
            total_size,
            transferred: 0,
            start_time: SystemTime::now(),
            end_time: None,
        }
    }
    
    pub fn total_size(&self) -> u64 {
        self.total_size
    }
    
    pub fn transferred(&self) -> u64 {
        self.transferred
    }
    
    pub fn progress(&self) -> f32 {
        if self.total_size == 0 {
            1.0
        } else {
            self.transferred as f32 / self.total_size as f32
        }
    }
    
    pub fn remaining_bytes(&self) -> u64 {
        self.total_size - self.transferred
    }
    
    pub fn update_progress(&mut self, transferred: u64) -> Result<(), ProgressError> {
        if transferred > self.total_size {
            return Err(ProgressError::InvalidProgress);
        }
        
        self.transferred = transferred;
        
        if transferred == self.total_size {
            self.end_time = Some(SystemTime::now());
        }
        
        Ok(())
    }
    
    pub fn is_in_progress(&self) -> bool {
        self.transferred > 0 && self.transferred < self.total_size
    }
    
    pub fn is_completed(&self) -> bool {
        self.transferred == self.total_size
    }
    
    pub fn end_time(&self) -> Option<&SystemTime> {
        self.end_time.as_ref()
    }
}

#[derive(Debug)]
pub enum ProgressError {
    InvalidProgress,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ProgressTracker {
    total_size: u64,
    transferred: u64,
    chunks_completed: u32,
    total_chunks: u32,
    timing: ProgressTiming,
    speed_calculator: SpeedCalculator,
    milestones: Vec<ProgressMilestone>,
    state: TransferState,
}

#[derive(Debug, Clone)]
pub struct ProgressTiming {
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub last_update: SystemTime,
    pub estimated_completion: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct SpeedCalculator {
    pub recent_samples: VecDeque<SpeedSample>,
    pub current_speed: f64, // bytes per second
    pub average_speed: f64,
    pub peak_speed: f64,
}

#[derive(Debug, Clone)]
pub struct SpeedSample {
    pub timestamp: SystemTime,
    pub bytes_transferred: u64,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct ProgressMilestone {
    pub progress: f32,
    pub timestamp: SystemTime,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferState {
    NotStarted,
    InProgress,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub enum ProgressError {
    InvalidProgress,
    InvalidChunkCount,
    AlreadyCompleted,
    NotStarted,
    AlreadyPaused,
    NotPaused,
}

impl ProgressTracker {
    // 重构后的代码，保持测试绿色
    pub fn new(total_size: u64) -> Self {
        let total_chunks = Self::calculate_chunks(total_size);
        Self {
            total_size,
            transferred: 0,
            chunks_completed: 0,
            total_chunks,
            timing: ProgressTiming::new(),
            speed_calculator: SpeedCalculator::new(),
            milestones: Vec::new(),
            state: TransferState::NotStarted,
        }
    }
    
    pub fn with_chunk_size(total_size: u64, chunk_size: usize) -> Self {
        let total_chunks = ((total_size as usize + chunk_size - 1) / chunk_size) as u32;
        Self {
            total_size,
            transferred: 0,
            chunks_completed: 0,
            total_chunks,
            timing: ProgressTiming::new(),
            speed_calculator: SpeedCalculator::new(),
            milestones: Vec::new(),
            state: TransferState::NotStarted,
        }
    }
    
    pub fn total_size(&self) -> u64 {
        self.total_size
    }
    
    pub fn transferred(&self) -> u64 {
        self.transferred
    }
    
    pub fn progress(&self) -> f32 {
        if self.total_size == 0 {
            1.0
        } else {
            (self.transferred as f32 / self.total_size as f32).min(1.0)
        }
    }
    
    pub fn chunk_progress(&self) -> f32 {
        if self.total_chunks == 0 {
            1.0
        } else {
            (self.chunks_completed as f32 / self.total_chunks as f32).min(1.0)
        }
    }
    
    pub fn remaining_bytes(&self) -> u64 {
        self.total_size - self.transferred
    }
    
    pub fn remaining_chunks(&self) -> u32 {
        self.total_chunks - self.chunks_completed
    }
    
    pub fn current_speed(&self) -> f64 {
        self.speed_calculator.current_speed
    }
    
    pub fn average_speed(&self) -> f64 {
        self.speed_calculator.average_speed
    }
    
    pub fn peak_speed(&self) -> f64 {
        self.speed_calculator.peak_speed
    }
    
    pub fn estimated_time_remaining(&self) -> Option<Duration> {
        if self.current_speed() > 0.0 && !self.is_completed() {
            let remaining_bytes = self.remaining_bytes() as f64;
            Some(Duration::from_secs_f64(remaining_bytes / self.current_speed()))
        } else {
            None
        }
    }
    
    pub fn estimated_completion_time(&self) -> Option<SystemTime> {
        self.timing.estimated_completion
    }
    
    pub fn state(&self) -> &TransferState {
        &self.state
    }
    
    pub fn is_in_progress(&self) -> bool {
        matches!(self.state, TransferState::InProgress)
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.state, TransferState::Completed)
    }
    
    pub fn is_paused(&self) -> bool {
        matches!(self.state, TransferState::Paused)
    }
    
    pub fn elapsed_time(&self) -> Duration {
        let end = self.timing.end_time.unwrap_or(SystemTime::now());
        end.duration_since(self.timing.start_time).unwrap_or(Duration::ZERO)
    }
    
    pub fn milestones(&self) -> &[ProgressMilestone] {
        &self.milestones
    }
    
    pub fn update_progress(&mut self, transferred: u64) -> Result<(), ProgressError> {
        if self.is_completed() {
            return Err(ProgressError::AlreadyCompleted);
        }
        
        if transferred > self.total_size {
            return Err(ProgressError::InvalidProgress);
        }
        
        let old_transferred = self.transferred;
        self.transferred = transferred;
        
        // 更新速度计算
        if transferred > old_transferred {
            let now = SystemTime::now();
            let duration = now.duration_since(self.timing.last_update).unwrap_or(Duration::ZERO);
            let bytes_delta = transferred - old_transferred;
            
            if duration > Duration::ZERO {
                self.speed_calculator.add_sample(SpeedSample {
                    timestamp: now,
                    bytes_transferred: bytes_delta,
                    duration,
                });
            }
        }
        
        self.timing.last_update = SystemTime::now();
        
        // 更新预计完成时间
        self.update_estimated_completion();
        
        // 检查里程碑
        self.check_milestones();
        
        // 检查是否完成
        if transferred == self.total_size {
            self.complete();
        }
        
        Ok(())
    }
    
    pub fn update_chunk_progress(&mut self, chunks_completed: u32) -> Result<(), ProgressError> {
        if chunks_completed > self.total_chunks {
            return Err(ProgressError::InvalidChunkCount);
        }
        
        self.chunks_completed = chunks_completed;
        
        // 估算已传输字节数
        let estimated_transferred = (chunks_completed as u64 * 4096).min(self.total_size);
        self.update_progress(estimated_transferred)?;
        
        Ok(())
    }
    
    pub fn start(&mut self) -> Result<(), ProgressError> {
        if matches!(self.state, TransferState::InProgress) {
            return Err(ProgressError::AlreadyCompleted);
        }
        
        self.state = TransferState::InProgress;
        self.timing.start_time = SystemTime::now();
        self.timing.last_update = SystemTime::now();
        
        self.add_milestone(0.0, "Transfer started".to_string());
        
        Ok(())
    }
    
    pub fn pause(&mut self) -> Result<(), ProgressError> {
        if !matches!(self.state, TransferState::InProgress) {
            return Err(ProgressError::NotPaused);
        }
        
        self.state = TransferState::Paused;
        self.add_milestone(self.progress(), "Transfer paused".to_string());
        
        Ok(())
    }
    
    pub fn resume(&mut self) -> Result<(), ProgressError> {
        if !matches!(self.state, TransferState::Paused) {
            return Err(ProgressError::NotPaused);
        }
        
        self.state = TransferState::InProgress;
        self.timing.last_update = SystemTime::now();
        self.add_milestone(self.progress(), "Transfer resumed".to_string());
        
        Ok(())
    }
    
    pub fn complete(&mut self) {
        self.state = TransferState::Completed;
        self.timing.end_time = Some(SystemTime::now());
        self.timing.estimated_completion = Some(self.timing.end_time.unwrap());
        
        self.add_milestone(1.0, "Transfer completed".to_string());
    }
    
    pub fn cancel(&mut self) {
        self.state = TransferState::Cancelled;
        self.add_milestone(self.progress(), "Transfer cancelled".to_string());
    }
    
    pub fn fail(&mut self, reason: String) {
        self.state = TransferState::Failed;
        self.add_milestone(self.progress(), format!("Transfer failed: {}", reason));
    }
    
    fn update_estimated_completion(&mut self) {
        if let Some(remaining) = self.estimated_time_remaining() {
            self.timing.estimated_completion = Some(SystemTime::now() + remaining);
        }
    }
    
    fn check_milestones(&mut self) {
        let progress = self.progress();
        
        // 检查25%里程碑
        if progress >= 0.25 && !self.milestones.iter().any(|m| (m.progress - 0.25).abs() < 0.01) {
            self.add_milestone(0.25, "25% complete".to_string());
        }
        
        // 检查50%里程碑
        if progress >= 0.5 && !self.milestones.iter().any(|m| (m.progress - 0.5).abs() < 0.01) {
            self.add_milestone(0.5, "50% complete".to_string());
        }
        
        // 检查75%里程碑
        if progress >= 0.75 && !self.milestones.iter().any(|m| (m.progress - 0.75).abs() < 0.01) {
            self.add_milestone(0.75, "75% complete".to_string());
        }
    }
    
    fn add_milestone(&mut self, progress: f32, message: String) {
        self.milestones.push(ProgressMilestone {
            progress,
            timestamp: SystemTime::now(),
            message,
        });
    }
    
    fn calculate_chunks(total_size: u64) -> u32 {
        const CHUNK_SIZE: usize = 4096;
        ((total_size as usize + CHUNK_SIZE - 1) / CHUNK_SIZE) as u32
    }
}

impl ProgressTiming {
    fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
            end_time: None,
            last_update: SystemTime::now(),
            estimated_completion: None,
        }
    }
}

impl SpeedCalculator {
    fn new() -> Self {
        Self {
            recent_samples: VecDeque::new(),
            current_speed: 0.0,
            average_speed: 0.0,
            peak_speed: 0.0,
        }
    }
    
    fn add_sample(&mut self, sample: SpeedSample) {
        // 添加新样本
        self.recent_samples.push_back(sample);
        
        // 保持最近30秒的样本
        let thirty_seconds_ago = SystemTime::now() - Duration::from_secs(30);
        while let Some(front) = self.recent_samples.front() {
            if front.timestamp < thirty_seconds_ago {
                self.recent_samples.pop_front();
            } else {
                break;
            }
        }
        
        // 计算当前速度（最近5秒）
        let five_seconds_ago = SystemTime::now() - Duration::from_secs(5);
        let recent_bytes: u64 = self.recent_samples
            .iter()
            .filter(|s| s.timestamp >= five_seconds_ago)
            .map(|s| s.bytes_transferred)
            .sum();
        
        let recent_duration: Duration = self.recent_samples
            .iter()
            .filter(|s| s.timestamp >= five_seconds_ago)
            .map(|s| s.duration)
            .sum();
        
        self.current_speed = if recent_duration > Duration::ZERO {
            recent_bytes as f64 / recent_duration.as_secs_f64()
        } else {
            0.0
        };
        
        // 更新平均速度
        if !self.recent_samples.is_empty() {
            let total_bytes: u64 = self.recent_samples.iter().map(|s| s.bytes_transferred).sum();
            let total_duration: Duration = self.recent_samples.iter().map(|s| s.duration).sum();
            self.average_speed = total_bytes as f64 / total_duration.as_secs_f64();
        }
        
        // 更新峰值速度
        self.peak_speed = self.peak_speed.max(self.current_speed);
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的进度跟踪：

```rust
// rust-core/domain/transfer/progress.rs
pub struct ProgressTracker {
    // 传输进度跟踪实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [传输协议处理](0305-transfer-protocol-handling.md)

## 后续任务

- [Task 0307: 实现传输状态监控](0307-transfer-status-monitor.md)
- [Task 0308: 实现传输数据压缩](0308-transfer-data-compression.md)
- [Task 0309: 实现传输重试机制](0309-transfer-retry-mechanism.md)