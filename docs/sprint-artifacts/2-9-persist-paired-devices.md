# Story 2.9: 持久化配对设备信息

Status: done

## Story

As a 用户,
I want 配对信息保存到本地,
So that 重启后自动重连无需重新配对.

## Acceptance Criteria

1. **Given** 设备配对成功 **When** 保存配对信息 **Then** 存储设备 ID、公钥、连接偏好
2. **And** 使用平台安全存储（预留接口）
3. **And** 支持读取、更新、删除操作
4. **And** 测试验证持久化正确

## Tasks / Subtasks

- [x] Task 1: 定义存储接口 trait (AC: 2)
  - [x] 1.1 创建 `crates/nearclip-crypto/src/device_store.rs`
  - [x] 1.2 定义 `DeviceStore` trait（save, load, delete, list, exists, count）
  - [x] 1.3 使用 `CryptoError::DeviceStore(String)` 错误变体
  - [x] 1.4 同步接口设计（异步预留通过 trait 扩展实现）

- [x] Task 2: 实现文件存储后端 (AC: 1, 3)
  - [x] 2.1 创建 `FileDeviceStore` 结构体
  - [x] 2.2 实现 `DeviceStore` trait
  - [x] 2.3 使用 JSON 格式存储 `PairedDevice` 列表
  - [x] 2.4 存储位置：用户数据目录（可配置）
  - [x] 2.5 使用原子写入（临时文件 + 重命名）防止并发问题

- [x] Task 3: 实现 CRUD 操作 (AC: 1, 3)
  - [x] 3.1 `save(device: &PairedDevice)` - 保存/更新设备
  - [x] 3.2 `load(device_id: &str)` - 按 ID 加载单个设备
  - [x] 3.3 `load_all()` - 加载所有已配对设备
  - [x] 3.4 `delete(device_id: &str)` - 删除指定设备
  - [x] 3.5 `exists(device_id: &str)` - 检查设备是否存在

- [x] Task 4: 扩展 CryptoError (AC: 1, 3)
  - [x] 4.1 添加 `DeviceStore(String)` 错误变体
  - [x] 4.2 错误信息包含具体 I/O 错误描述
  - [x] 4.3 保持与现有错误类型兼容

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `lib.rs` 中添加 `pub mod device_store;`
  - [x] 5.2 添加 re-exports: `DeviceStore`, `FileDeviceStore`, `FileDeviceStoreConfig`
  - [x] 5.3 更新模块文档

- [x] Task 6: 编写单元测试 (AC: 4)
  - [x] 6.1 测试 `FileDeviceStore` 保存设备
  - [x] 6.2 测试 `FileDeviceStore` 加载设备
  - [x] 6.3 测试 `FileDeviceStore` 删除设备
  - [x] 6.4 测试 `FileDeviceStore` 更新设备（相同 ID 覆盖）
  - [x] 6.5 测试加载不存在的设备返回 None
  - [x] 6.6 测试空存储返回空列表

- [x] Task 7: 编写集成测试 (AC: 4)
  - [x] 7.1 创建 `tests/device_store_integration.rs`
  - [x] 7.2 测试：配对 → 保存 → 重新加载 → 验证数据一致
  - [x] 7.3 测试：多设备存储和加载
  - [x] 7.4 测试：删除后再加载返回 None

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-crypto` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-crypto` 确保测试通过 (132 tests)
  - [x] 8.3 运行 `cargo clippy -p nearclip-crypto` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, CryptoError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**日志规则：**
- ✅ 使用 `tracing` 记录日志
- ✅ 存储操作使用 `debug!` 级别
- ✅ 错误使用 `warn!` 或 `error!` 级别
- ❌ 禁止使用 `println!()`

**平台存储策略 (来自 architecture.md):**
- 密钥存储：平台密钥库（macOS Keychain / Android Keystore）
- 本 Story 实现文件存储作为默认后端
- 预留 trait 接口供未来平台特定实现

### Previous Story (2-8) Intelligence

**关键学习：**
- `PairedDevice` 结构体已实现 Serialize/Deserialize
- `PairedDevice::to_json()` 和 `from_json()` 已可用
- 使用 `sha2` 计算共享密钥哈希
- 安全设计：不存储完整共享密钥

**已建立的模式：**
- 配置结构体 + 主结构体模式
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- `#[instrument]` 属性进行函数追踪
- Builder pattern 用于可选字段配置

### 已有代码可复用

**nearclip-crypto/src/pairing.rs (Story 2-8):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairedDevice {
    pub device_id: String,
    pub public_key_bytes: Vec<u8>,
    pub connection_info: Option<ConnectionInfo>,
    pub shared_secret_hash: String,
    pub paired_at: u64,
}

impl PairedDevice {
    pub fn to_json(&self) -> Result<String, CryptoError>;
    pub fn from_json(json: &str) -> Result<Self, CryptoError>;
    pub fn verify_shared_secret(&self, shared_secret: &[u8]) -> bool;
}
```

### 设计决策

**DeviceStore Trait:**
```rust
/// 设备存储接口
///
/// 定义配对设备的持久化操作。不同平台可实现不同后端：
/// - `FileDeviceStore` - JSON 文件存储（默认）
/// - `KeychainDeviceStore` - macOS Keychain（未来）
/// - `KeystoreDeviceStore` - Android Keystore（未来）
pub trait DeviceStore {
    /// 保存或更新设备信息
    fn save(&self, device: &PairedDevice) -> Result<(), CryptoError>;

    /// 按 ID 加载设备
    fn load(&self, device_id: &str) -> Result<Option<PairedDevice>, CryptoError>;

    /// 加载所有已配对设备
    fn load_all(&self) -> Result<Vec<PairedDevice>, CryptoError>;

    /// 删除设备
    fn delete(&self, device_id: &str) -> Result<bool, CryptoError>;

    /// 检查设备是否存在
    fn exists(&self, device_id: &str) -> Result<bool, CryptoError>;
}
```

**FileDeviceStore 结构：**
```rust
use std::path::PathBuf;

/// 文件存储配置
#[derive(Debug, Clone)]
pub struct FileDeviceStoreConfig {
    /// 存储目录
    pub directory: PathBuf,
    /// 文件名
    pub filename: String,
}

impl Default for FileDeviceStoreConfig {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            filename: "paired_devices.json".to_string(),
        }
    }
}

/// 基于文件的设备存储
///
/// 使用 JSON 格式存储配对设备列表。
pub struct FileDeviceStore {
    config: FileDeviceStoreConfig,
}

impl FileDeviceStore {
    /// 创建默认配置的存储
    pub fn new() -> Self;

    /// 使用自定义配置
    pub fn with_config(config: FileDeviceStoreConfig) -> Self;

    /// 获取存储文件路径
    pub fn file_path(&self) -> PathBuf;
}

impl DeviceStore for FileDeviceStore {
    // ... trait 实现
}
```

**存储文件格式：**
```json
{
  "version": 1,
  "devices": [
    {
      "device_id": "macbook-pro-2024",
      "public_key_bytes": [4, 123, ...],
      "connection_info": {
        "ip": "192.168.1.100",
        "port": 8765,
        "mdns_name": "macbook._nearclip._tcp.local"
      },
      "shared_secret_hash": "base64-encoded-sha256",
      "paired_at": 1702656000
    }
  ]
}
```

### 依赖说明

**已有依赖（无需添加）：**
```toml
[dependencies]
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
```

**可选添加（文件锁）：**
```toml
[dependencies]
fs2 = "0.4"  # 文件锁支持（如需要）
```

### 代码模板

**device_store.rs:**
```rust
//! 配对设备存储
//!
//! 提供配对设备的持久化存储功能。
//!
//! # Example
//!
//! ```ignore
//! use nearclip_crypto::{FileDeviceStore, DeviceStore, PairedDevice};
//!
//! // 创建存储
//! let store = FileDeviceStore::new();
//!
//! // 保存设备
//! let device = PairedDevice::new(...);
//! store.save(&device).unwrap();
//!
//! // 加载设备
//! let loaded = store.load("device-id").unwrap();
//! ```

use crate::{CryptoError, PairedDevice};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, instrument, warn};

/// 存储文件格式版本
const STORE_VERSION: u8 = 1;

/// 设备存储接口
pub trait DeviceStore {
    fn save(&self, device: &PairedDevice) -> Result<(), CryptoError>;
    fn load(&self, device_id: &str) -> Result<Option<PairedDevice>, CryptoError>;
    fn load_all(&self) -> Result<Vec<PairedDevice>, CryptoError>;
    fn delete(&self, device_id: &str) -> Result<bool, CryptoError>;
    fn exists(&self, device_id: &str) -> Result<bool, CryptoError>;
}

/// 存储文件内容结构
#[derive(Debug, Serialize, Deserialize)]
struct StoreFile {
    version: u8,
    devices: Vec<PairedDevice>,
}

// ... FileDeviceStore 实现
```

### 安全考虑

**文件权限：**
- 存储文件应设置适当权限（仅用户可读写）
- 在 Unix 系统上使用 0600 权限

**并发访问：**
- 使用文件锁防止多进程同时写入
- 或使用原子写入（写入临时文件后重命名）

**敏感数据：**
- `shared_secret_hash` 是哈希值，不是原始密钥
- `public_key_bytes` 是公钥，可安全存储
- 未来平台存储可加密整个文件

### 文件位置

**目标文件：**
- `crates/nearclip-crypto/src/device_store.rs` - DeviceStore trait 和 FileDeviceStore（新建）
- `crates/nearclip-crypto/src/keypair.rs` - 扩展 CryptoError（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-crypto/tests/device_store_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-crypto/src/pairing.rs` - PairedDevice 定义

### 与上游 Story 2-8 的关系

Story 2-8 已实现以下功能，本 Story 直接使用：
- `PairedDevice` 结构体（含 Serialize/Deserialize）
- `PairedDevice::to_json()` / `from_json()` 方法
- `ConnectionInfo` 结构体

### 与下游的关系

本 Story 为以下功能提供基础：
- Epic 4/5 客户端：启动时加载已配对设备
- Story 3.x：基于已配对设备建立连接
- Epic 6：加密存储（扩展 DeviceStore trait）

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-crypto/src/device_store.rs` - DeviceStore trait 和 FileDeviceStore（新建）
- `crates/nearclip-crypto/src/keypair.rs` - CryptoError 扩展（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出和文档（修改）
- `crates/nearclip-crypto/tests/device_store_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] device_store.rs 文件已创建
- [x] DeviceStore trait 定义完整
- [x] FileDeviceStore 实现 DeviceStore trait
- [x] CRUD 操作正常工作
- [x] CryptoError 已扩展新变体
- [x] 所有单元测试通过 (18 device_store unit tests)
- [x] 集成测试验证完整存储流程 (8 integration tests)
- [x] `cargo build -p nearclip-crypto` 成功
- [x] `cargo test -p nearclip-crypto` 成功 (132 tests total)
- [x] `cargo clippy -p nearclip-crypto` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |
| 2025-12-15 | Implementation completed - DeviceStore trait, FileDeviceStore |

---

Story created: 2025-12-15
Story completed: 2025-12-15
