# Story 3.7: 实现通道状态监测

Status: done

## Story

As a 用户,
I want 系统自动检测通道可用性,
So that 选择最佳通道同步.

## Acceptance Criteria

1. **Given** WiFi 和 BLE 通道均已初始化 **When** 持续监测通道状态 **Then** 检测 WiFi 连接断开
2. **And** 检测 BLE 连接断开
3. **And** 触发状态变更回调
4. **And** 测试验证状态检测准确

## Tasks / Subtasks

- [x] Task 1: 定义状态变更回调接口 (AC: 3)
  - [x] 1.1 创建 `ChannelStatusCallback` trait
  - [x] 1.2 定义 `on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus)` 方法
  - [x] 1.3 定义 `on_all_channels_unavailable(&self)` 方法

- [x] Task 2: 实现通道监测器配置 (AC: 1, 2)
  - [x] 2.1 创建 `crates/nearclip-sync/src/monitor.rs`
  - [x] 2.2 定义 `ChannelMonitorConfig` 结构体
  - [x] 2.3 实现 builder pattern: `with_check_interval()`, `with_timeout()`
  - [x] 2.4 实现 `validate()` 方法

- [x] Task 3: 实现通道监测器核心 (AC: 1, 2, 3)
  - [x] 3.1 创建 `ChannelMonitor` 结构体
  - [x] 3.2 实现 `ChannelMonitor::new(config, callback)`
  - [x] 3.3 实现 `start(&self)` 方法开始监测
  - [x] 3.4 实现 `stop(&self)` 方法停止监测
  - [x] 3.5 实现 `get_status(&self, channel: Channel)` 获取当前状态
  - [x] 3.6 实现 `update_status(&self, channel: Channel, status: ChannelStatus)` 更新状态

- [x] Task 4: 实现状态变更检测逻辑 (AC: 1, 2, 3)
  - [x] 4.1 实现内部状态存储 (RwLock<HashMap<Channel, ChannelStatus>>)
  - [x] 4.2 状态变更时触发回调
  - [x] 4.3 检测所有通道不可用时触发特殊回调

- [x] Task 5: 实现状态快照 (AC: 1, 2)
  - [x] 5.1 创建 `ChannelSnapshot` 结构体
  - [x] 5.2 实现 `snapshot(&self)` 获取所有通道当前状态
  - [x] 5.3 包含时间戳信息

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 添加 `mod monitor;`
  - [x] 6.2 添加 re-exports

- [x] Task 7: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 测试 `ChannelMonitorConfig` 配置验证
  - [x] 7.2 测试 `ChannelMonitor` 创建
  - [x] 7.3 测试状态更新和回调触发
  - [x] 7.4 测试快照功能

- [x] Task 8: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 创建 `tests/channel_monitor_integration.rs`
  - [x] 8.2 测试：状态变更检测
  - [x] 8.3 测试：回调正确触发
  - [x] 8.4 测试：并发状态更新

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

- Story 3-5: 剪贴板发送 (使用 ChannelStatus)
- Story 3-6: 剪贴板接收 (使用 ChannelStatus)
- Story 3-8: 通道自动切换 (基于本 Story 的监测结果)

### 设计决策

1. **被动更新模式**: `ChannelMonitor` 不主动轮询网络状态，由上层调用 `update_status()` 更新
2. **回调通知**: 状态变更时立即触发回调
3. **线程安全**: 使用 `RwLock` 保护内部状态
4. **快照机制**: 提供 `snapshot()` 方法获取一致的状态视图

### 数据结构

```rust
// 状态回调
trait ChannelStatusCallback {
    fn on_status_changed(&self, channel: Channel, old: ChannelStatus, new: ChannelStatus);
    fn on_all_channels_unavailable(&self);
}

// 监测器配置
struct ChannelMonitorConfig {
    check_interval: Duration,  // 检查间隔 (供未来主动轮询使用)
    timeout: Duration,         // 状态超时时间
}

// 监测器
struct ChannelMonitor {
    config: ChannelMonitorConfig,
    callback: Arc<dyn ChannelStatusCallback>,
    states: RwLock<HashMap<Channel, ChannelStatus>>,
}

// 状态快照
struct ChannelSnapshot {
    wifi_status: ChannelStatus,
    ble_status: ChannelStatus,
    timestamp: u64,
}
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
