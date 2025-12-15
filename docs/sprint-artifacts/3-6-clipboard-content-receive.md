# Story 3.6: 实现剪贴板内容接收

Status: done

## Story

As a 用户,
I want 收到其他设备的剪贴板内容,
So that 可以直接粘贴.

## Acceptance Criteria

1. **Given** 监听已启动 **When** 收到 ClipboardSync 消息 **Then** 解析消息内容
2. **And** 通过回调通知上层
3. **And** 发送 ACK 确认
4. **And** 测试验证接收流程

## Tasks / Subtasks

- [x] Task 1: 定义接收回调接口 (AC: 2)
  - [x] 1.1 创建 `ClipboardReceiveCallback` trait
  - [x] 1.2 定义 `on_clipboard_received(&self, content: &[u8], device_id: &str)` 方法
  - [x] 1.3 定义 `on_receive_error(&self, error: SyncError)` 方法

- [x] Task 2: 实现剪贴板接收器配置 (AC: 1)
  - [x] 2.1 创建 `crates/nearclip-sync/src/receiver.rs`
  - [x] 2.2 定义 `ClipboardReceiverConfig` 结构体
  - [x] 2.3 实现 builder pattern: `with_max_message_size()`, `with_message_timeout()`
  - [x] 2.4 实现 `validate()` 方法

- [x] Task 3: 实现剪贴板接收器核心 (AC: 1, 2, 3)
  - [x] 3.1 创建 `ClipboardReceiver` 结构体
  - [x] 3.2 实现 `ClipboardReceiver::new(config, callback)`
  - [x] 3.3 实现 `handle_message(&self, data: &[u8])` 方法
  - [x] 3.4 内部解析 `Message` 结构
  - [x] 3.5 验证消息类型为 `ClipboardSync`
  - [x] 3.6 触发回调通知上层

- [x] Task 4: 实现 ACK 响应生成 (AC: 3)
  - [x] 4.1 实现 `create_ack(&self, original_message: &Message)` 方法
  - [x] 4.2 使用 `Message::ack()` 构建 ACK 消息
  - [x] 4.3 返回序列化后的 ACK 数据

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `lib.rs` 添加 `mod receiver;`
  - [x] 5.2 添加 re-exports

- [x] Task 6: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 6.1 测试 `ClipboardReceiverConfig` 配置验证
  - [x] 6.2 测试 `ClipboardReceiver` 创建
  - [x] 6.3 测试消息解析
  - [x] 6.4 测试 ACK 生成

- [x] Task 7: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 创建 `tests/clipboard_receive_integration.rs`
  - [x] 7.2 测试：完整接收流程
  - [x] 7.3 测试：回调正确触发
  - [x] 7.4 测试：错误情况处理 (无效消息、超大消息等)

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-sync` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-sync` 确保测试通过
  - [x] 8.3 运行 `cargo clippy -p nearclip-sync` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **消息协议**: 复用 Story 1-4 实现的 `Message` 和 `MessageType::ClipboardSync`
2. **异步模式**: 使用 tokio async/await
3. **错误处理**: 复用 Story 3-5 的 `SyncError` 枚举

### 与其他 Story 的关系

- Story 3-5: 剪贴板内容发送 (配对实现)
- Story 3-1/3-2: TCP 服务端/客户端 (WiFi 通道数据来源)
- Story 3-3/3-4: BLE 数据传输 (BLE 通道数据来源)

### 数据流

```
Source Device                           ClipboardReceiver
     |                                       |
     |-- Message::clipboard_sync() -------->|
     |                                       |-- handle_message()
     |                                       |-- parse Message
     |                                       |-- validate ClipboardSync
     |                                       |-- callback.on_clipboard_received()
     |<-- Message::ack() -------------------|-- create_ack()
```

### 设计决策

1. **被动接收**: `ClipboardReceiver` 不主动监听，由上层传入接收到的数据
2. **同步回调**: 收到消息后立即触发回调通知上层
3. **ACK 生成**: 提供生成 ACK 消息的方法，由上层决定如何发送
4. **消息验证**: 验证消息类型、大小、时间戳等

### 与 Sender 的对称性

| Sender | Receiver |
|--------|----------|
| `ClipboardSendCallback` | `ClipboardReceiveCallback` |
| `ClipboardSenderConfig` | `ClipboardReceiverConfig` |
| `ClipboardSender` | `ClipboardReceiver` |
| `send()` | `handle_message()` |
| 等待 ACK | 生成 ACK |

## Checklist

- [x] All tasks completed
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
