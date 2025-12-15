# Story 3.5: 实现剪贴板内容发送

Status: done

## Story

As a 用户,
I want 复制内容后自动发送到其他设备,
So that 实现无感同步.

## Acceptance Criteria

1. **Given** 已配对且连接的设备 **When** 调用发送剪贴板函数 **Then** 构建 ClipboardSync 消息
2. **And** 选择可用通道（WiFi 优先）
3. **And** 发送并等待 ACK
4. **And** 测试验证消息到达

## Tasks / Subtasks

- [x] Task 1: 定义发送回调接口 (AC: 3)
  - [x] 1.1 创建 `ClipboardSendCallback` trait
  - [x] 1.2 定义 `on_send_success(&self, device_id: &str)` 方法
  - [x] 1.3 定义 `on_send_failure(&self, device_id: &str, error: SyncError)` 方法
  - [x] 1.4 定义 `on_ack_received(&self, device_id: &str)` 方法

- [x] Task 2: 定义通道抽象 (AC: 2)
  - [x] 2.1 创建 `crates/nearclip-sync/src/channel.rs`
  - [x] 2.2 定义 `Channel` 枚举 (Wifi, Ble)
  - [x] 2.3 定义 `ChannelStatus` 枚举 (Available, Unavailable, Busy)
  - [x] 2.4 定义 `ChannelSelector` trait 用于通道选择策略

- [x] Task 3: 实现剪贴板发送器配置 (AC: 1, 3)
  - [x] 3.1 创建 `crates/nearclip-sync/src/sender.rs`
  - [x] 3.2 定义 `ClipboardSenderConfig` 结构体
  - [x] 3.3 实现 builder pattern: `with_ack_timeout()`, `with_retry_count()`
  - [x] 3.4 实现 `validate()` 方法

- [x] Task 4: 实现剪贴板发送器核心 (AC: 1, 2, 3)
  - [x] 4.1 创建 `ClipboardSender` 结构体
  - [x] 4.2 实现 `ClipboardSender::new(config, callback)`
  - [x] 4.3 实现 `send(&self, content: &[u8], device_id: &str)` 方法
  - [x] 4.4 内部构建 `Message::clipboard_sync()` 消息
  - [x] 4.5 内部实现通道选择（WiFi 优先）
  - [x] 4.6 实现 ACK 等待逻辑

- [x] Task 5: 定义同步错误类型 (AC: 3)
  - [x] 5.1 创建 `SyncError` 枚举
  - [x] 5.2 包含 ChannelUnavailable, Timeout, SendFailed 等变体

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 添加 `mod channel;` 和 `mod sender;`
  - [x] 6.2 添加 re-exports

- [x] Task 7: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 测试 `ClipboardSenderConfig` 配置验证
  - [x] 7.2 测试 `ClipboardSender` 创建
  - [x] 7.3 测试消息构建
  - [x] 7.4 测试通道选择逻辑

- [x] Task 8: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 创建 `tests/clipboard_send_integration.rs`
  - [x] 8.2 测试：完整发送流程 (mock)
  - [x] 8.3 测试：回调正确触发
  - [x] 8.4 测试：错误情况处理

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-sync` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-sync` 确保测试通过
  - [x] 9.3 运行 `cargo clippy -p nearclip-sync` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **消息协议**: 复用 Story 1-4 实现的 `Message` 和 `MessageType::ClipboardSync`
2. **通道选择**: WiFi 优先，BLE 备选
3. **异步模式**: 使用 tokio async/await
4. **错误处理**: 定义 `SyncError` 枚举

### 与其他 Story 的关系

- Story 3-1/3-2: TCP 服务端/客户端 (WiFi 通道)
- Story 3-3/3-4: BLE 数据传输 (BLE 通道)
- Story 3-6: 剪贴板内容接收 (配对实现)
- Story 3-7: 通道状态监测 (后续增强)

### 数据流

```
ClipboardSender                         Target Device
     |                                       |
     |-- select_channel() --> WiFi/BLE      |
     |                                       |
     |-- Message::clipboard_sync() -------->|
     |                                       |
     |<-- Message::ack() -------------------|
     |                                       |
     |-- callback.on_ack_received() ------->|
```

### 设计决策

1. **通道抽象**: `ChannelSelector` trait 允许不同的选择策略
2. **回调通知**: 通过 `ClipboardSendCallback` 通知上层发送状态
3. **ACK 等待**: 可配置超时时间
4. **重试**: 当前阶段不实现重试（Story 3-9 实现）

### 当前阶段简化

本 Story 实现核心发送逻辑框架，实际通道连接和发送将返回模拟结果或 `PlatformNotSupported`：
- WiFi 通道需要已建立的 `TcpConnection`
- BLE 通道需要已连接的 `CentralDataSender`
- 通道的实际连接管理将在 Story 3-11 (核心协调层) 中实现

## Checklist

- [x] All tasks completed
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
