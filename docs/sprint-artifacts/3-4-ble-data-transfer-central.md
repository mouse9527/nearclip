# Story 3.4: 实现 BLE 数据传输（中心端）

Status: done

## Story

As a 用户,
I want 通过蓝牙发送数据,
So that 无 WiFi 时也能同步.

## Acceptance Criteria

1. **Given** 已发现目标 BLE 设备 **When** 连接并发送数据 **Then** 自动分片发送（MTU 限制）
2. **And** 等待确认响应
3. **And** 超时重试
4. **And** 测试验证发送成功

## Tasks / Subtasks

- [x] Task 1: 定义数据发送回调接口 (AC: 2)
  - [x] 1.1 创建 `DataSenderCallback` trait
  - [x] 1.2 定义 `on_send_complete(&self, message_id: u16)` 方法
  - [x] 1.3 定义 `on_send_error(&self, message_id: u16, error: BleError)` 方法
  - [x] 1.4 定义 `on_ack_received(&self, message_id: u16)` 方法

- [x] Task 2: 实现中心端数据发送器配置 (AC: 1, 3)
  - [x] 2.1 创建 `crates/nearclip-ble/src/central_data.rs`
  - [x] 2.2 定义 `CentralDataConfig` 结构体
  - [x] 2.3 实现 builder pattern: `with_mtu()`, `with_send_timeout()`, `with_retry_count()`
  - [x] 2.4 实现 `validate()` 方法

- [x] Task 3: 实现中心端数据发送器 (AC: 1, 2, 3)
  - [x] 3.1 创建 `CentralDataSender` 结构体
  - [x] 3.2 实现 `CentralDataSender::new(config, callback)`
  - [x] 3.3 实现 `connect(device_id: &str)` → 连接到外设
  - [x] 3.4 实现 `send(data: &[u8])` → 分片发送数据
  - [x] 3.5 实现 `disconnect()` → 断开连接
  - [x] 3.6 内部使用 `Chunker` 分片数据
  - [x] 3.7 实现发送状态跟踪

- [x] Task 4: 实现 ACK 等待和超时重试 (AC: 2, 3)
  - [x] 4.1 实现 ACK 等待机制 (等待 DATA_ACK_CHARACTERISTIC 通知)
  - [x] 4.2 实现超时检测
  - [x] 4.3 实现重试逻辑 (基于 retry_count 配置)
  - [x] 4.4 失败后触发 callback

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `lib.rs` 添加 `mod central_data;`
  - [x] 5.2 添加 re-exports: CentralDataSender, CentralDataConfig, DataSenderCallback

- [x] Task 6: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 6.1 测试 `CentralDataConfig` 配置验证
  - [x] 6.2 测试 `CentralDataSender` 创建
  - [x] 6.3 测试发送状态跟踪
  - [x] 6.4 测试超时重试逻辑

- [x] Task 7: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 创建 `tests/ble_central_data_integration.rs`
  - [x] 7.2 测试：完整数据发送流程 (mock)
  - [x] 7.3 测试：分片数据发送
  - [x] 7.4 测试：回调正确触发
  - [x] 7.5 测试：错误情况处理

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-ble` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-ble` 确保测试通过
  - [x] 8.3 运行 `cargo clippy -p nearclip-ble` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **平台实现**: 当前返回 `PlatformNotSupported`，平台特定实现后续通过 feature flags 添加
2. **分片复用**: 复用 Story 3-3 实现的 `Chunker` 进行数据分片
3. **异步模式**: 使用 tokio async/await
4. **错误处理**: 使用 `BleError` 枚举

### 与 Story 3-3 的关系

Story 3-3 实现了外设端 (Peripheral) 数据接收:
- `PeripheralDataReceiver` 接收分片并重组
- `DataReceiverCallback` 接收完整数据回调

Story 3-4 实现中心端 (Central) 数据发送:
- `CentralDataSender` 分片并发送数据
- `DataSenderCallback` 发送完成/失败回调

### 数据流

```
Central (发送方)                    Peripheral (接收方)
     |                                    |
     |-- connect() ---------------------->|
     |                                    |
     |-- write(chunk[0]) --------------->| (DATA_TRANSFER_CHARACTERISTIC)
     |<-- notify(ack) -------------------| (DATA_ACK_CHARACTERISTIC)
     |                                    |
     |-- write(chunk[1]) --------------->|
     |<-- notify(ack) -------------------|
     |                                    |
     |-- write(chunk[N]) --------------->|
     |<-- notify(complete) --------------|
     |                                    |
     |-- disconnect() ------------------>|
```

### GATT 特征使用

- `DATA_TRANSFER_CHARACTERISTIC_UUID`: 中心端写入分片数据
- `DATA_ACK_CHARACTERISTIC_UUID`: 订阅通知以接收 ACK

### 设计决策

1. **同步发送**: `send()` 方法是异步的，等待所有分片发送完成或失败
2. **回调通知**: 通过 `DataSenderCallback` 通知上层发送状态
3. **重试策略**: 单个分片发送失败后重试，超过次数则整个消息失败
4. **消息 ID**: 使用 `Chunker` 生成的 message_id 跟踪发送状态

## Checklist

- [x] All tasks completed
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
