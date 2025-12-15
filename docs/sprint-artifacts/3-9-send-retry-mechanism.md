# Story 3.9: 实现发送重试机制

Status: done

## Story

As a 用户,
I want 发送失败时自动重试,
So that 提高同步成功率.

## Acceptance Criteria

1. **Given** 发送失败（超时或错误） **When** 触发重试逻辑 **Then** 最多重试 3 次
2. **And** 重试间隔 2 秒
3. **And** 3 次失败后返回错误
4. **And** 测试验证重试行为

## Tasks / Subtasks

- [x] Task 1: 定义重试配置 (AC: 1, 2)
  - [x] 1.1 创建 `crates/nearclip-sync/src/retry.rs`
  - [x] 1.2 定义 `RetryConfig` 结构体 (max_retries, retry_delay)
  - [x] 1.3 实现 builder pattern
  - [x] 1.4 实现 `validate()` 方法

- [x] Task 2: 定义重试策略接口 (AC: 1, 2)
  - [x] 2.1 创建 `RetryStrategy` trait
  - [x] 2.2 定义 `next_delay(&self, attempt: u32) -> Option<Duration>` 方法
  - [x] 2.3 定义 `should_retry(&self, attempt: u32, error: &SyncError) -> bool` 方法

- [x] Task 3: 实现固定延迟策略 (AC: 2)
  - [x] 3.1 创建 `FixedDelayStrategy` 结构体
  - [x] 3.2 实现 `RetryStrategy` trait
  - [x] 3.3 默认延迟 2 秒

- [x] Task 4: 实现指数退避策略 (AC: 2)
  - [x] 4.1 创建 `ExponentialBackoffStrategy` 结构体
  - [x] 4.2 实现 `RetryStrategy` trait
  - [x] 4.3 支持 base_delay, max_delay, multiplier 配置

- [x] Task 5: 实现重试回调接口 (AC: 1, 3)
  - [x] 5.1 创建 `RetryCallback` trait
  - [x] 5.2 定义 `on_retry(&self, attempt: u32, error: &SyncError, delay: Duration)` 方法
  - [x] 5.3 定义 `on_exhausted(&self, total_attempts: u32, final_error: &SyncError)` 方法

- [x] Task 6: 实现重试执行器 (AC: 1, 2, 3)
  - [x] 6.1 创建 `RetryExecutor` 结构体
  - [x] 6.2 实现 `execute<F, T, E>(&self, operation: F) -> Result<T, E>` 方法
  - [x] 6.3 实现重试循环逻辑

- [x] Task 7: 导出模块 (AC: 1)
  - [x] 7.1 在 `lib.rs` 添加 `pub mod retry;`
  - [x] 7.2 添加 re-exports

- [x] Task 8: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 测试 `RetryConfig` 配置验证
  - [x] 8.2 测试 `FixedDelayStrategy` 重试计算
  - [x] 8.3 测试 `ExponentialBackoffStrategy` 退避计算
  - [x] 8.4 测试 `RetryExecutor` 重试逻辑
  - [x] 8.5 测试 3 次失败后停止

- [x] Task 9: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 9.1 创建 `tests/retry_integration.rs`
  - [x] 9.2 测试：成功无需重试
  - [x] 9.3 测试：重试后成功
  - [x] 9.4 测试：重试耗尽失败
  - [x] 9.5 测试：回调正确触发

- [x] Task 10: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 10.1 运行 `cargo build -p nearclip-sync` 确保无错误
  - [x] 10.2 运行 `cargo test -p nearclip-sync` 确保测试通过
  - [x] 10.3 运行 `cargo clippy -p nearclip-sync` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **重试次数**: 默认 3 次
2. **重试间隔**: 默认 2 秒
3. **异步模式**: 使用 tokio async/await
4. **错误处理**: 复用 `SyncError` 枚举

### 与其他 Story 的关系

- Story 3-5: 剪贴板发送 (使用重试机制)
- Story 3-8: 通道自动切换 (切换失败后可能触发重试)
- Story 3-11: 核心协调层 (整合重试逻辑)

### 设计决策

1. **策略模式**: 使用 `RetryStrategy` trait 支持不同重试策略
2. **回调通知**: 每次重试和最终失败时触发回调
3. **泛型执行器**: `RetryExecutor` 可用于任何异步操作
4. **可中断**: 支持提前取消重试

### 数据结构

```rust
// 重试配置
struct RetryConfig {
    max_retries: u32,      // 最大重试次数 (默认 3)
    retry_delay: Duration, // 重试间隔 (默认 2s)
}

// 重试策略
trait RetryStrategy: Send + Sync {
    fn next_delay(&self, attempt: u32) -> Option<Duration>;
    fn should_retry(&self, attempt: u32, error: &SyncError) -> bool;
}

// 固定延迟策略
struct FixedDelayStrategy {
    delay: Duration,
    max_retries: u32,
}

// 指数退避策略
struct ExponentialBackoffStrategy {
    base_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
    max_retries: u32,
}

// 重试回调
trait RetryCallback: Send + Sync {
    fn on_retry(&self, attempt: u32, error: &SyncError, delay: Duration);
    fn on_exhausted(&self, total_attempts: u32, final_error: &SyncError);
}

// 重试执行器
struct RetryExecutor<S: RetryStrategy> {
    strategy: S,
    callback: Option<Arc<dyn RetryCallback>>,
}
```

### 重试流程

```
[执行操作]
     |
     v
[成功?]--Yes--> [返回结果]
     |
    No
     v
[可重试?]--No--> [返回错误]
     |
    Yes
     v
[触发 on_retry 回调]
     |
     v
[等待 delay]
     |
     v
[重试次数 < max?]--No--> [触发 on_exhausted] --> [返回错误]
     |
    Yes
     |
     v
[执行操作] (循环)
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing (32 retry unit tests)
- [x] Integration tests passing (28 integration tests)
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
