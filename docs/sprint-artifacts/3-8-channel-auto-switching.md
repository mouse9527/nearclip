# Story 3.8: 实现通道自动切换

Status: done

## Story

As a 用户,
I want WiFi 断开时自动切换到蓝牙,
So that 同步不中断.

## Acceptance Criteria

1. **Given** WiFi 通道不可用 **When** 需要发送剪贴板 **Then** 自动选择 BLE 通道
2. **And** WiFi 恢复后切回
3. **And** 切换过程对用户透明
4. **And** 测试验证切换逻辑

## Tasks / Subtasks

- [x] Task 1: 定义自动切换回调接口 (AC: 3)
  - [x] 1.1 创建 `ChannelSwitchCallback` trait
  - [x] 1.2 定义 `on_channel_switched(&self, from: Channel, to: Channel, reason: SwitchReason)` 方法
  - [x] 1.3 定义 `SwitchReason` 枚举 (Unavailable, HigherPriority, Manual, Initial)

- [x] Task 2: 实现自动切换器配置 (AC: 1, 2)
  - [x] 2.1 创建 `crates/nearclip-sync/src/switcher.rs`
  - [x] 2.2 定义 `ChannelSwitcherConfig` 结构体
  - [x] 2.3 实现 builder pattern: `with_preferred_channel()`, `with_auto_fallback()`, `with_auto_recovery()`
  - [x] 2.4 实现 `validate()` 方法

- [x] Task 3: 实现自动切换器核心 (AC: 1, 2, 3)
  - [x] 3.1 创建 `ChannelSwitcher` 结构体
  - [x] 3.2 实现 `ChannelSwitcher::new(config, callback)`
  - [x] 3.3 实现 `current_channel(&self)` 获取当前活跃通道
  - [x] 3.4 实现 `select_best_channel(&self)` 选择最佳可用通道
  - [x] 3.5 实现 `handle_status_change(&self, channel, old, new)` 处理状态变更

- [x] Task 4: 实现通道切换逻辑 (AC: 1, 2)
  - [x] 4.1 WiFi 不可用时自动切换到 BLE
  - [x] 4.2 WiFi 恢复时自动切换回 WiFi (如果配置了 auto_recovery)
  - [x] 4.3 BLE 不可用且 WiFi 可用时切换到 WiFi
  - [x] 4.4 所有通道不可用时设置为 None

- [x] Task 5: 实现切换策略 (AC: 1, 2, 3)
  - [x] 5.1 创建 `SwitchStrategy` trait
  - [x] 5.2 实现 `PrioritySwitchStrategy` (WiFi 优先)
  - [x] 5.3 实现 `StickySwitchStrategy` (尽量保持当前通道)

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 添加 `mod switcher;`
  - [x] 6.2 添加 re-exports

- [x] Task 7: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 测试 `ChannelSwitcherConfig` 配置验证
  - [x] 7.2 测试 `ChannelSwitcher` 创建
  - [x] 7.3 测试 WiFi -> BLE 切换
  - [x] 7.4 测试 BLE -> WiFi 自动恢复
  - [x] 7.5 测试切换回调触发

- [x] Task 8: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 创建 `tests/channel_switcher_integration.rs`
  - [x] 8.2 测试：完整切换流程
  - [x] 8.3 测试：回调正确触发
  - [x] 8.4 测试：多次切换场景

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-sync` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-sync` 确保测试通过
  - [x] 9.3 运行 `cargo clippy -p nearclip-sync` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **通道优先级**: WiFi 优先，BLE 备选
2. **异步模式**: 使用 tokio async/await
3. **错误处理**: 复用 `SyncError` 枚举

### 与其他 Story 的关系

- Story 3-7: 通道状态监测 (提供状态变更通知)
- Story 3-5: 剪贴板发送 (使用切换后的通道)
- Story 3-9: 发送重试机制 (切换失败后的重试)

### 设计决策

1. **被动切换模式**: `ChannelSwitcher` 响应 `ChannelMonitor` 的状态变更
2. **回调通知**: 切换时触发回调，上层可记录日志或通知用户
3. **可配置策略**: 支持不同的切换策略 (优先级/粘性)
4. **自动恢复**: 可配置是否在高优先级通道恢复时自动切回

### 数据结构

```rust
// 切换原因
enum SwitchReason {
    Unavailable,      // 当前通道不可用
    HigherPriority,   // 更高优先级通道可用
    Manual,           // 手动切换
}

// 切换回调
trait ChannelSwitchCallback {
    fn on_channel_switched(&self, from: Option<Channel>, to: Option<Channel>, reason: SwitchReason);
}

// 切换器配置
struct ChannelSwitcherConfig {
    preferred_channel: Channel,    // 首选通道 (默认 WiFi)
    auto_fallback: bool,           // 自动降级到备用通道 (默认 true)
    auto_recovery: bool,           // 自动恢复到首选通道 (默认 true)
}

// 切换器
struct ChannelSwitcher {
    config: ChannelSwitcherConfig,
    current: RwLock<Option<Channel>>,
    callback: Arc<dyn ChannelSwitchCallback>,
}
```

### 切换流程

```
WiFi Available     WiFi Unavailable    WiFi Recovered
     |                   |                   |
     v                   v                   v
[Use WiFi] -------> [Use BLE] -------> [Use WiFi]
                  (auto_fallback)    (auto_recovery)
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing (26 switcher unit tests)
- [x] Integration tests passing (26 integration tests)
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
