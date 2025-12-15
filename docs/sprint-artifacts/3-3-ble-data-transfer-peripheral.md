# Story 3.3: 实现 BLE 数据传输（外设端）

Status: done

## Story

As a 用户,
I want 通过蓝牙接收数据,
So that 无 WiFi 时也能同步.

## Acceptance Criteria

1. **Given** BLE 广播已启动 **When** 中心设备连接并写入数据 **Then** 接收完整的消息数据
2. **And** 支持分片重组（MTU 限制）
3. **And** 触发消息处理回调
4. **And** 测试验证数据完整性

## Tasks / Subtasks

- [x] Task 1: 扩展 GATT 特征定义 (AC: 1)
  - [x] 1.1 在 `gatt.rs` 添加 `DATA_TRANSFER_CHARACTERISTIC_UUID` (Write + Notify)
  - [x] 1.2 添加 `DATA_ACK_CHARACTERISTIC_UUID` (Read + Notify) 用于确认
  - [x] 1.3 定义 BLE MTU 相关常量 (DEFAULT_MTU, MAX_MTU, CHUNK_HEADER_SIZE)

- [x] Task 2: 实现数据分片模块 (AC: 2)
  - [x] 2.1 创建 `crates/nearclip-ble/src/chunk.rs`
  - [x] 2.2 定义 `ChunkHeader` 结构 (sequence_number, total_chunks, chunk_size, message_id)
  - [x] 2.3 实现 `Chunker::chunk(data, mtu)` → 分片数据列表
  - [x] 2.4 实现 `Reassembler` 结构体 (接收分片并重组)
  - [x] 2.5 实现 `Reassembler::add_chunk()` → 添加分片
  - [x] 2.6 实现 `Reassembler::is_complete()` → 检查是否完整
  - [x] 2.7 实现 `Reassembler::assemble()` → 重组完整消息

- [x] Task 3: 扩展 BleError 错误类型 (AC: 1, 2)
  - [x] 3.1 添加 `ChunkError(String)` 变体 - 分片/重组错误
  - [x] 3.2 添加 `DataTransfer(String)` 变体 - 数据传输错误
  - [x] 3.3 添加 `ConnectionFailed(String)` 变体 - 连接失败
  - [x] 3.4 添加 `Timeout(String)` 变体 - 超时错误

- [x] Task 4: 定义数据接收回调接口 (AC: 3)
  - [x] 4.1 创建 `DataReceiverCallback` trait
  - [x] 4.2 定义 `on_data_received(&self, data: Vec<u8>, from_device: &str)` 方法
  - [x] 4.3 定义 `on_receive_error(&self, error: BleError)` 方法

- [x] Task 5: 实现外设端数据接收器 (AC: 1, 2, 3)
  - [x] 5.1 创建 `crates/nearclip-ble/src/peripheral_data.rs`
  - [x] 5.2 定义 `PeripheralDataConfig` (MTU, timeout 等配置)
  - [x] 5.3 创建 `PeripheralDataReceiver` 结构体
  - [x] 5.4 实现 `PeripheralDataReceiver::new(config, callback)`
  - [x] 5.5 实现 `start()` → 开始监听数据写入
  - [x] 5.6 实现 `stop()` → 停止监听
  - [x] 5.7 内部使用 `Reassembler` 重组分片数据
  - [x] 5.8 数据完整后触发 callback

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 添加 `mod chunk;`
  - [x] 6.2 在 `lib.rs` 添加 `mod peripheral_data;`
  - [x] 6.3 添加 re-exports: chunk 相关类型
  - [x] 6.4 添加 re-exports: PeripheralDataReceiver, PeripheralDataConfig, DataReceiverCallback

- [x] Task 7: 编写单元测试 (AC: 2, 4)
  - [x] 7.1 测试 `Chunker::chunk()` - 小于 MTU 数据
  - [x] 7.2 测试 `Chunker::chunk()` - 等于 MTU 数据
  - [x] 7.3 测试 `Chunker::chunk()` - 大于 MTU 需分片
  - [x] 7.4 测试 `Reassembler` - 按序接收
  - [x] 7.5 测试 `Reassembler` - 乱序接收
  - [x] 7.6 测试 `Reassembler` - 重复分片处理
  - [x] 7.7 测试 `Reassembler` - 超时清理

- [x] Task 8: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 创建 `tests/ble_data_transfer_integration.rs`
  - [x] 8.2 测试：完整数据收发流程 (mock)
  - [x] 8.3 测试：分片数据重组
  - [x] 8.4 测试：回调正确触发
  - [x] 8.5 测试：错误情况处理

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-ble` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-ble` 确保测试通过
  - [x] 9.3 运行 `cargo clippy -p nearclip-ble` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- 所有公开函数必须返回 `Result<T, BleError>`
- 禁止在库代码中使用 `unwrap()` 或 `expect()`
- 禁止使用 `panic!()` 宏
- 使用 `thiserror` 定义错误类型

**日志规则：**
- 使用 `tracing` 记录日志
- 数据接收使用 `debug!` 级别
- 错误使用 `warn!` 或 `error!` 级别
- 禁止使用 `println!()`

**BLE MTU 说明：**
- 标准 BLE MTU: 23 bytes (ATT MTU)
- 有效 payload: 20 bytes (去除 ATT header)
- 协商后可达 512 bytes
- 需要支持动态 MTU 协商

### 已有代码可复用

**nearclip-ble/src/gatt.rs (Story 2-5):**
```rust
pub const NEARCLIP_SERVICE_UUID: Uuid = ...;
pub const DEVICE_ID_CHARACTERISTIC_UUID: Uuid = ...;
pub const PUBKEY_HASH_CHARACTERISTIC_UUID: Uuid = ...;
```

**nearclip-ble/src/peripheral.rs (Story 2-5):**
```rust
pub struct BleAdvertiser { ... }
pub struct BleAdvertiserConfig { ... }

impl BleAdvertiser {
    pub async fn new(config: BleAdvertiserConfig) -> Result<Self, BleError>;
    pub async fn start(&mut self) -> Result<(), BleError>;
    pub async fn stop(&mut self) -> Result<(), BleError>;
    pub async fn is_advertising(&self) -> bool;
}
```

**nearclip-ble/src/error.rs:**
```rust
#[derive(Debug, Error)]
pub enum BleError {
    #[error("BLE initialization failed: {0}")]
    Initialization(String),
    #[error("BLE not powered on")]
    NotPowered,
    #[error("Advertising failed: {0}")]
    Advertising(String),
    #[error("Service registration failed: {0}")]
    ServiceRegistration(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Platform not supported")]
    PlatformNotSupported,
}
```

### 设计决策

**数据分片格式：**
```rust
/// 分片头部 (8 bytes)
#[derive(Debug, Clone)]
pub struct ChunkHeader {
    /// 消息 ID (用于识别属于同一消息的分片)
    pub message_id: u16,      // 2 bytes
    /// 当前分片序号 (从 0 开始)
    pub sequence_number: u16, // 2 bytes
    /// 总分片数
    pub total_chunks: u16,    // 2 bytes
    /// 当前分片 payload 长度
    pub payload_length: u16,  // 2 bytes
}

/// 分片数据 = Header (8 bytes) + Payload (MTU - 8 bytes)
```

**MTU 常量定义：**
```rust
/// 默认 BLE ATT MTU (最小值)
pub const DEFAULT_BLE_MTU: usize = 23;

/// ATT 协议头部大小
pub const ATT_HEADER_SIZE: usize = 3;

/// 分片头部大小
pub const CHUNK_HEADER_SIZE: usize = 8;

/// 默认有效 payload 大小 = 23 - 3 - 8 = 12 bytes
pub const DEFAULT_CHUNK_PAYLOAD_SIZE: usize = DEFAULT_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE;

/// 最大协商 MTU
pub const MAX_BLE_MTU: usize = 512;

/// 最大有效 payload 大小 = 512 - 3 - 8 = 501 bytes
pub const MAX_CHUNK_PAYLOAD_SIZE: usize = MAX_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE;
```

**Reassembler 设计：**
```rust
pub struct Reassembler {
    /// 消息 ID
    message_id: u16,
    /// 预期分片总数
    total_chunks: u16,
    /// 已接收的分片 (sequence_number -> payload)
    chunks: HashMap<u16, Vec<u8>>,
    /// 创建时间 (用于超时清理)
    created_at: Instant,
    /// 超时时间
    timeout: Duration,
}

impl Reassembler {
    pub fn new(message_id: u16, total_chunks: u16, timeout: Duration) -> Self;
    pub fn add_chunk(&mut self, header: ChunkHeader, payload: Vec<u8>) -> Result<(), BleError>;
    pub fn is_complete(&self) -> bool;
    pub fn is_expired(&self) -> bool;
    pub fn assemble(self) -> Result<Vec<u8>, BleError>;
}
```

**数据接收回调：**
```rust
/// 数据接收回调接口
pub trait DataReceiverCallback: Send + Sync {
    /// 接收到完整数据时调用
    fn on_data_received(&self, data: Vec<u8>, from_device: &str);

    /// 接收出错时调用
    fn on_receive_error(&self, error: BleError);
}
```

**新增 GATT 特征：**
```rust
/// 数据传输特征 UUID (Write Without Response + Notify)
/// UUID: 4e454152-434c-4950-0000-000000000004
pub const DATA_TRANSFER_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c,             // CL
    0x49, 0x50,             // IP
    0x00, 0x00,             // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x04, // characteristic number
]);

/// 数据确认特征 UUID (Read + Notify)
/// 用于返回 ACK 和状态
/// UUID: 4e454152-434c-4950-0000-000000000005
pub const DATA_ACK_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52,
    0x43, 0x4c,
    0x49, 0x50,
    0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
]);
```

### 平台实现说明

**当前状态：**
本 Story 实现 Rust 层的数据分片/重组逻辑和接口定义。
实际的 BLE GATT 服务注册和数据写入监听需要平台特定实现：
- macOS: CoreBluetooth CBPeripheralManager
- Linux: BlueZ D-Bus API
- Android: 通过 uniffi JNI 调用 Android BLE API

**模拟测试策略：**
由于 BLE 硬件依赖，测试将：
1. 单元测试：直接测试 Chunker 和 Reassembler 逻辑
2. 集成测试：使用 mock 模拟 BLE 数据写入事件

### 依赖说明

**已有依赖（无需添加）：**
```toml
[dependencies]
tokio.workspace = true
thiserror.workspace = true
tracing.workspace = true
uuid.workspace = true
base64.workspace = true
```

**可能需要添加：**
```toml
[dependencies]
# 用于 HashMap 存储分片
# (标准库已包含，无需额外依赖)
```

### 代码模板

**chunk.rs:**
```rust
//! BLE 数据分片模块
//!
//! 处理 BLE MTU 限制下的数据分片和重组。

use crate::BleError;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, trace, warn};

/// 默认 BLE ATT MTU
pub const DEFAULT_BLE_MTU: usize = 23;

/// ATT 协议头部大小
pub const ATT_HEADER_SIZE: usize = 3;

/// 分片头部大小 (8 bytes)
pub const CHUNK_HEADER_SIZE: usize = 8;

/// 默认有效 payload 大小
pub const DEFAULT_CHUNK_PAYLOAD_SIZE: usize = DEFAULT_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE;

/// 最大协商 MTU
pub const MAX_BLE_MTU: usize = 512;

/// 默认重组超时时间
pub const DEFAULT_REASSEMBLE_TIMEOUT: Duration = Duration::from_secs(30);

/// 分片头部
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHeader {
    pub message_id: u16,
    pub sequence_number: u16,
    pub total_chunks: u16,
    pub payload_length: u16,
}

impl ChunkHeader {
    /// 从字节数组解析头部
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BleError> {
        if bytes.len() < CHUNK_HEADER_SIZE {
            return Err(BleError::ChunkError(format!(
                "Header too short: {} bytes, expected {}",
                bytes.len(),
                CHUNK_HEADER_SIZE
            )));
        }

        Ok(Self {
            message_id: u16::from_le_bytes([bytes[0], bytes[1]]),
            sequence_number: u16::from_le_bytes([bytes[2], bytes[3]]),
            total_chunks: u16::from_le_bytes([bytes[4], bytes[5]]),
            payload_length: u16::from_le_bytes([bytes[6], bytes[7]]),
        })
    }

    /// 序列化为字节数组
    pub fn to_bytes(&self) -> [u8; CHUNK_HEADER_SIZE] {
        let mut bytes = [0u8; CHUNK_HEADER_SIZE];
        bytes[0..2].copy_from_slice(&self.message_id.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.sequence_number.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.total_chunks.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.payload_length.to_le_bytes());
        bytes
    }
}

/// 数据分片器
pub struct Chunker;

impl Chunker {
    /// 将数据分片
    pub fn chunk(data: &[u8], message_id: u16, mtu: usize) -> Result<Vec<Vec<u8>>, BleError> {
        // 实现...
    }
}

/// 数据重组器
pub struct Reassembler {
    message_id: u16,
    total_chunks: u16,
    chunks: HashMap<u16, Vec<u8>>,
    created_at: Instant,
    timeout: Duration,
}

impl Reassembler {
    pub fn new(message_id: u16, total_chunks: u16, timeout: Duration) -> Self {
        // 实现...
    }

    pub fn add_chunk(&mut self, header: ChunkHeader, payload: Vec<u8>) -> Result<(), BleError> {
        // 实现...
    }

    pub fn is_complete(&self) -> bool {
        // 实现...
    }

    pub fn is_expired(&self) -> bool {
        // 实现...
    }

    pub fn assemble(self) -> Result<Vec<u8>, BleError> {
        // 实现...
    }
}
```

### 文件位置

**目标文件：**
- `crates/nearclip-ble/src/gatt.rs` - 扩展 GATT 特征 (修改)
- `crates/nearclip-ble/src/error.rs` - 扩展错误类型 (修改)
- `crates/nearclip-ble/src/chunk.rs` - 数据分片模块 (新建)
- `crates/nearclip-ble/src/peripheral_data.rs` - 外设数据接收 (新建)
- `crates/nearclip-ble/src/lib.rs` - 模块导出 (修改)
- `crates/nearclip-ble/tests/ble_data_transfer_integration.rs` - 集成测试 (新建)

**参考文件：**
- `crates/nearclip-ble/src/peripheral.rs` - 广播实现
- `crates/nearclip-net/src/tcp/connection.rs` - TCP 连接抽象参考

### 与上游的关系

本 Story 依赖：
- Story 2-5 (BLE 设备广播): `BleAdvertiser`, GATT 定义

### 与下游的关系

本 Story 为以下功能提供基础：
- Story 3.4: BLE 数据传输（中心端）
- Story 3.5: 剪贴板内容发送
- Story 3.8: 通道自动切换

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-ble/src/gatt.rs` - 扩展 GATT 特征 (修改)
- `crates/nearclip-ble/src/error.rs` - 扩展错误类型 (修改)
- `crates/nearclip-ble/src/chunk.rs` - 数据分片模块 (新建)
- `crates/nearclip-ble/src/peripheral_data.rs` - 外设数据接收 (新建)
- `crates/nearclip-ble/src/lib.rs` - 模块导出 (修改)
- `crates/nearclip-ble/tests/ble_data_transfer_integration.rs` - 集成测试 (新建)

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [ ] GATT 特征已扩展（DATA_TRANSFER, DATA_ACK）
- [ ] ChunkHeader 可正确序列化/反序列化
- [ ] Chunker 可正确分片数据
- [ ] Reassembler 可正确重组分片（包括乱序）
- [ ] PeripheralDataReceiver 可接收并重组数据
- [ ] DataReceiverCallback 被正确调用
- [ ] BleError 已扩展新变体
- [ ] 所有单元测试通过
- [ ] 集成测试验证完整流程
- [ ] `cargo build -p nearclip-ble` 成功
- [ ] `cargo test -p nearclip-ble` 成功
- [ ] `cargo clippy -p nearclip-ble` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created |

---

Story created: 2025-12-15
