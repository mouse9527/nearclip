# Story 2.7: 生成配对二维码

Status: done

## Story

As a 用户,
I want 生成包含配对信息的二维码,
So that 其他设备可以扫描配对.

## Acceptance Criteria

1. **Given** 设备公钥已生成 **When** 生成配对二维码 **Then** 二维码包含设备 ID、公钥、连接信息
2. **And** 使用 JSON 格式编码
3. **And** 返回二维码图片数据（PNG）
4. **And** 测试验证二维码可解析

## Tasks / Subtasks

- [x] Task 1: 定义配对数据结构 (AC: 1, 2)
  - [x] 1.1 创建 `crates/nearclip-crypto/src/pairing.rs`
  - [x] 1.2 定义 `PairingData` 结构体（device_id, public_key, connection_info）
  - [x] 1.3 定义 `ConnectionInfo` 结构体（ip, port, mdns_name）
  - [x] 1.4 实现 `PairingData::new()` 构造函数
  - [x] 1.5 实现 Serialize/Deserialize trait（serde）

- [x] Task 2: 实现 JSON 编码 (AC: 2)
  - [x] 2.1 实现 `PairingData::to_json()` 方法
  - [x] 2.2 实现 `PairingData::from_json()` 方法
  - [x] 2.3 添加 JSON 格式版本字段以支持未来扩展
  - [x] 2.4 验证公钥为有效 Base64

- [x] Task 3: 实现二维码生成 (AC: 3)
  - [x] 3.1 添加 `qrcode` 和 `image` 依赖到 Cargo.toml
  - [x] 3.2 创建 `QrCodeGenerator` 结构体
  - [x] 3.3 实现 `generate_png(data: &PairingData)` → `Result<Vec<u8>, CryptoError>`
  - [x] 3.4 配置二维码大小和纠错级别（建议 L 级别）
  - [x] 3.5 返回 PNG 图片字节数据

- [x] Task 4: 扩展 CryptoError (AC: 1, 3)
  - [x] 4.1 添加 `QrCodeGeneration(String)` 错误变体
  - [x] 4.2 添加 `JsonSerialization(String)` 错误变体
  - [x] 4.3 保持与现有错误类型兼容

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `lib.rs` 中添加 `pub mod pairing;`
  - [x] 5.2 添加 re-exports: `PairingData`, `ConnectionInfo`, `QrCodeGenerator`
  - [x] 5.3 更新模块文档

- [x] Task 6: 编写单元测试 (AC: 4)
  - [x] 6.1 测试 `PairingData` 构造和验证
  - [x] 6.2 测试 JSON 序列化/反序列化往返
  - [x] 6.3 测试 `QrCodeGenerator` 生成有效 PNG
  - [x] 6.4 测试生成的二维码可被解析

- [x] Task 7: 编写集成测试 (AC: 4)
  - [x] 7.1 创建 `tests/pairing_qrcode_integration.rs`
  - [x] 7.2 测试：生成密钥对 → 创建 PairingData → 生成二维码 → 解析验证
  - [x] 7.3 测试：二维码解析后数据完整性
  - [x] 7.4 测试：各种数据长度的二维码生成

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-crypto` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-crypto` 确保测试通过 (87 tests)
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
- ✅ 二维码生成使用 `debug!` 级别
- ✅ 错误使用 `warn!` 或 `error!` 级别
- ❌ 禁止使用 `println!()`

**Crypto 模块在架构中的位置：**
```
nearclip-ffi → nearclip-core → nearclip-sync → nearclip-net → nearclip-crypto
                                                    ↓
                                               nearclip-ble → nearclip-crypto
```

### Previous Story (2-6) Intelligence

**关键学习：**
- 使用平台抽象层模式
- 使用 `tokio::sync::broadcast` 实现事件广播
- 使用 `Arc<RwLock<>>` 保护共享状态
- 实现 Drop trait 确保资源正确清理
- 添加 Base64 验证
- 使用 `#[instrument]` 属性进行函数追踪

**已建立的模式：**
- 配置结构体 + 主结构体模式
- Builder pattern 方法签名
- 使用 `#[instrument]` 属性进行函数追踪

### 已有代码可复用

**nearclip-crypto/src/keypair.rs:**
- `EcdhKeyPair::public_key_bytes()` - 65 字节未压缩公钥
- `EcdhKeyPair::public_key_bytes_compressed()` - 33 字节压缩公钥
- `CryptoError` - 需扩展新变体

**常量复用：**
- 公钥长度: 65 字节（未压缩）或 33 字节（压缩）

### 设计决策

**PairingData 结构：**
```rust
use serde::{Deserialize, Serialize};

/// 配对数据版本
pub const PAIRING_DATA_VERSION: u8 = 1;

/// 配对数据结构
///
/// 包含设备配对所需的所有信息，用于生成二维码。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingData {
    /// 数据格式版本（用于向后兼容）
    pub version: u8,
    /// 设备唯一标识符
    pub device_id: String,
    /// 公钥（Base64 编码）
    pub public_key: String,
    /// 连接信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<ConnectionInfo>,
}

/// 连接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// IP 地址
    pub ip: Option<String>,
    /// 端口号
    pub port: Option<u16>,
    /// mDNS 服务名
    pub mdns_name: Option<String>,
}

impl PairingData {
    /// 创建新的配对数据
    pub fn new(device_id: String, public_key_bytes: &[u8]) -> Self;

    /// 带连接信息的配对数据
    pub fn with_connection_info(mut self, info: ConnectionInfo) -> Self;

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> Result<String, CryptoError>;

    /// 从 JSON 字符串解析
    pub fn from_json(json: &str) -> Result<Self, CryptoError>;

    /// 验证配对数据
    pub fn validate(&self) -> Result<(), CryptoError>;
}
```

**QrCodeGenerator 结构：**
```rust
/// 二维码生成配置
#[derive(Debug, Clone)]
pub struct QrCodeConfig {
    /// 二维码模块大小（像素）
    pub module_size: u32,
    /// 边距（模块数）
    pub margin: u32,
    /// 纠错级别
    pub error_correction: QrCodeErrorCorrection,
}

#[derive(Debug, Clone, Copy)]
pub enum QrCodeErrorCorrection {
    Low,      // ~7% 恢复能力
    Medium,   // ~15% 恢复能力
    Quartile, // ~25% 恢复能力
    High,     // ~30% 恢复能力
}

impl Default for QrCodeConfig {
    fn default() -> Self {
        Self {
            module_size: 10,
            margin: 4,
            error_correction: QrCodeErrorCorrection::Low,
        }
    }
}

/// 二维码生成器
pub struct QrCodeGenerator {
    config: QrCodeConfig,
}

impl QrCodeGenerator {
    /// 创建默认配置的生成器
    pub fn new() -> Self;

    /// 使用自定义配置
    pub fn with_config(config: QrCodeConfig) -> Self;

    /// 生成二维码 PNG 图片
    pub fn generate_png(&self, data: &PairingData) -> Result<Vec<u8>, CryptoError>;

    /// 生成二维码 PNG 并保存到文件（仅用于测试/调试）
    #[cfg(test)]
    pub fn generate_to_file(&self, data: &PairingData, path: &str) -> Result<(), CryptoError>;
}
```

### 依赖说明

**需要添加的依赖：**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
qrcode = "0.14"
image = { version = "0.25", default-features = false, features = ["png"] }
```

**已有依赖（无需添加）：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
base64 = "0.22"
```

### 代码模板

**pairing.rs:**
```rust
//! 配对数据和二维码生成
//!
//! 提供设备配对所需的数据结构和二维码生成功能。
//!
//! # Example
//!
//! ```
//! use nearclip_crypto::{EcdhKeyPair, PairingData, QrCodeGenerator};
//!
//! // 生成密钥对
//! let keypair = EcdhKeyPair::generate();
//!
//! // 创建配对数据
//! let pairing_data = PairingData::new(
//!     "my-device-id".to_string(),
//!     &keypair.public_key_bytes(),
//! );
//!
//! // 生成二维码
//! let generator = QrCodeGenerator::new();
//! let png_data = generator.generate_png(&pairing_data).unwrap();
//! ```

use crate::CryptoError;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

// ... 结构体和实现
```

### 二维码内容示例

生成的 JSON 格式：
```json
{
  "version": 1,
  "device_id": "macbook-pro-2024",
  "public_key": "BPuM4yPp...base64...",
  "connection_info": {
    "ip": "192.168.1.100",
    "port": 8765,
    "mdns_name": "macbook-pro._nearclip._tcp.local"
  }
}
```

### 文件位置

**目标文件：**
- `crates/nearclip-crypto/src/pairing.rs` - 配对数据和二维码生成（新建）
- `crates/nearclip-crypto/src/keypair.rs` - 扩展 CryptoError（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-crypto/Cargo.toml` - 添加依赖（修改）
- `crates/nearclip-crypto/tests/pairing_qrcode_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-crypto/src/keypair.rs` - CryptoError 定义和模式
- `crates/nearclip-ble/src/central.rs` - 验证和错误处理模式

### 与下游 Story 2-8 的关系

Story 2-8 将实现二维码扫描和解析，需要使用本 Story 定义的：
- `PairingData::from_json()` 解析扫描到的数据
- `PairingData::validate()` 验证数据完整性
- `ConnectionInfo` 提取连接信息

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-crypto/src/pairing.rs` - PairingData, ConnectionInfo, QrCodeGenerator（新建）
- `crates/nearclip-crypto/src/keypair.rs` - CryptoError 扩展（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出和文档（修改）
- `crates/nearclip-crypto/Cargo.toml` - 添加 serde, qrcode, image 依赖（修改）
- `crates/nearclip-crypto/tests/pairing_qrcode_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] pairing.rs 文件已创建
- [x] PairingData 结构体完整（含 Serialize/Deserialize）
- [x] ConnectionInfo 结构体完整
- [x] QrCodeGenerator 实现 generate_png
- [x] CryptoError 已扩展新变体
- [x] JSON 序列化/反序列化正常
- [x] 二维码生成有效 PNG
- [x] 所有单元测试通过
- [x] 集成测试验证完整流程
- [x] `cargo build -p nearclip-crypto` 成功
- [x] `cargo test -p nearclip-crypto` 成功 (87 tests: 53 unit + 13 integration + 3 tls + 18 doc)
- [x] `cargo clippy -p nearclip-crypto` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |
| 2025-12-15 | Implementation completed - PairingData, ConnectionInfo, QrCodeGenerator |

---

Story created: 2025-12-15
Story completed: 2025-12-15
