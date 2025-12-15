# Story 2.8: 扫描并解析配对二维码

Status: done

## Story

As a 用户,
I want 扫描其他设备的二维码完成配对,
So that 两台设备建立信任关系.

## Acceptance Criteria

1. **Given** 目标设备显示配对二维码 **When** 扫描并解析二维码内容 **Then** 提取设备 ID、公钥、连接信息
2. **And** 通过 ECDH 完成密钥协商
3. **And** 双方确认配对成功
4. **And** 测试验证完整配对流程

## Tasks / Subtasks

- [x] Task 1: 实现二维码解析功能 (AC: 1)
  - [x] 1.1 创建 `crates/nearclip-crypto/src/qrcode_parser.rs`
  - [x] 1.2 实现 `QrCodeParser::parse_from_bytes(png_data: &[u8])` → `Result<String, CryptoError>`
  - [x] 1.3 实现 `QrCodeParser::parse_pairing_data(png_data: &[u8])` → `Result<PairingData, CryptoError>`
  - [x] 1.4 添加 `rqrr` 依赖到 Cargo.toml 用于二维码解码
  - [x] 1.5 处理无效二维码、无法解析等错误情况

- [x] Task 2: 实现配对数据提取 (AC: 1)
  - [x] 2.1 复用 `PairingData::from_json()` 解析 JSON
  - [x] 2.2 复用 `PairingData::validate()` 验证数据完整性
  - [x] 2.3 提取 `device_id`、`public_key`、`connection_info`
  - [x] 2.4 验证公钥格式（Base64 解码 + 长度检查）

- [x] Task 3: 实现 ECDH 密钥协商 (AC: 2)
  - [x] 3.1 创建 `PairingSession` 结构体管理配对会话
  - [x] 3.2 实现 `PairingSession::new(local_keypair: &EcdhKeyPair)` 构造函数
  - [x] 3.3 实现 `PairingSession::process_peer_data(peer_data: &PairingData)` → `Result<SharedSecret, CryptoError>`
  - [x] 3.4 使用 `EcdhKeyPair::compute_shared_secret()` 计算共享密钥
  - [x] 3.5 存储对方设备信息（device_id, public_key, connection_info）

- [x] Task 4: 定义配对结果数据结构 (AC: 3)
  - [x] 4.1 创建 `PairedDevice` 结构体（device_id, public_key_bytes, connection_info, shared_secret_hash, paired_at）
  - [x] 4.2 实现 `PairedDevice::new()` 构造函数
  - [x] 4.3 实现序列化/反序列化（为 Story 2-9 持久化做准备）
  - [x] 4.4 不存储完整 shared_secret，只存储哈希用于验证

- [x] Task 5: 扩展 CryptoError (AC: 1, 2, 3)
  - [x] 5.1 添加 `QrCodeParsing(String)` 错误变体
  - [x] 5.2 添加 `PairingFailed(String)` 错误变体
  - [x] 5.3 保持与现有错误类型兼容

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 中添加 `pub mod qrcode_parser;`
  - [x] 6.2 添加 re-exports: `QrCodeParser`, `PairingSession`, `PairedDevice`
  - [x] 6.3 更新模块文档

- [x] Task 7: 编写单元测试 (AC: 4)
  - [x] 7.1 测试 `QrCodeParser` 解析有效二维码
  - [x] 7.2 测试 `QrCodeParser` 处理无效二维码
  - [x] 7.3 测试 `PairingSession` 密钥协商
  - [x] 7.4 测试 `PairedDevice` 构造和序列化

- [x] Task 8: 编写集成测试 (AC: 4)
  - [x] 8.1 创建 `tests/pairing_flow_integration.rs`
  - [x] 8.2 测试：生成二维码 → 解析 → 密钥协商 → 配对成功
  - [x] 8.3 测试：双向配对（设备 A 扫描 B，B 扫描 A）
  - [x] 8.4 测试：配对数据完整性验证

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-crypto` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-crypto` 确保测试通过 (106 tests)
  - [x] 9.3 运行 `cargo clippy -p nearclip-crypto` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, CryptoError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**日志规则：**
- ✅ 使用 `tracing` 记录日志
- ✅ 配对流程使用 `info!` 级别
- ✅ 二维码解析使用 `debug!` 级别
- ✅ 错误使用 `warn!` 或 `error!` 级别
- ❌ 禁止使用 `println!()`

**Crypto 模块在架构中的位置：**
```
nearclip-ffi → nearclip-core → nearclip-sync → nearclip-net → nearclip-crypto
                                                    ↓
                                               nearclip-ble → nearclip-crypto
```

### Previous Story (2-7) Intelligence

**关键学习：**
- 使用 `image` crate 处理 PNG 图片
- `PairingData` 和 `ConnectionInfo` 结构体已完整实现
- `PairingData::from_json()` 和 `validate()` 可直接复用
- JSON 格式包含 `version` 字段用于向后兼容
- 使用 `#[instrument]` 属性进行函数追踪
- Builder pattern 用于可选字段配置

**已建立的模式：**
- 配置结构体 + 主结构体模式
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- 使用 `base64::engine::general_purpose::STANDARD` 进行编解码

### 已有代码可复用

**nearclip-crypto/src/pairing.rs (Story 2-7):**
```rust
pub struct PairingData {
    pub version: u8,
    pub device_id: String,
    pub public_key: String,  // Base64 编码
    pub connection_info: Option<ConnectionInfo>,
}

impl PairingData {
    pub fn from_json(json: &str) -> Result<Self, CryptoError>;
    pub fn validate(&self) -> Result<(), CryptoError>;
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, CryptoError>;
}
```

**nearclip-crypto/src/keypair.rs:**
```rust
pub struct EcdhKeyPair { ... }

impl EcdhKeyPair {
    pub fn generate() -> Self;
    pub fn compute_shared_secret(&self, peer_public_bytes: &[u8]) -> Result<Vec<u8>, CryptoError>;
    pub fn public_key_bytes(&self) -> Vec<u8>;
}
```

### 设计决策

**QrCodeParser 结构：**
```rust
/// 二维码解析器
///
/// 用于解析配对二维码中的内容。
pub struct QrCodeParser;

impl QrCodeParser {
    /// 从 PNG 图片字节解析二维码内容
    pub fn parse_from_bytes(png_data: &[u8]) -> Result<String, CryptoError>;

    /// 从 PNG 图片字节解析配对数据
    pub fn parse_pairing_data(png_data: &[u8]) -> Result<PairingData, CryptoError>;
}
```

**PairingSession 结构：**
```rust
/// 配对会话
///
/// 管理配对过程中的状态，包括密钥协商。
pub struct PairingSession {
    local_keypair: EcdhKeyPair,
    peer_device_id: Option<String>,
    peer_public_key: Option<Vec<u8>>,
    shared_secret: Option<Vec<u8>>,
}

impl PairingSession {
    /// 创建新的配对会话
    pub fn new(local_keypair: EcdhKeyPair) -> Self;

    /// 处理对方设备的配对数据
    ///
    /// 验证数据、提取公钥、计算共享密钥
    pub fn process_peer_data(&mut self, peer_data: &PairingData) -> Result<(), CryptoError>;

    /// 完成配对，返回已配对设备信息
    pub fn complete(self) -> Result<PairedDevice, CryptoError>;

    /// 获取共享密钥（用于后续加密通信）
    pub fn shared_secret(&self) -> Option<&[u8]>;
}
```

**PairedDevice 结构：**
```rust
use serde::{Deserialize, Serialize};

/// 已配对设备信息
///
/// 存储配对成功后的设备信息，用于后续连接和通信。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedDevice {
    /// 设备 ID
    pub device_id: String,
    /// 对方公钥（字节）
    pub public_key_bytes: Vec<u8>,
    /// 连接信息
    pub connection_info: Option<ConnectionInfo>,
    /// 共享密钥的哈希（用于验证，不存储完整密钥）
    pub shared_secret_hash: String,
    /// 配对时间（Unix 时间戳）
    pub paired_at: u64,
}

impl PairedDevice {
    /// 从配对会话创建
    pub fn from_session(
        session: PairingSession,
        peer_data: &PairingData,
    ) -> Result<Self, CryptoError>;
}
```

### 依赖说明

**需要添加的依赖：**
```toml
[dependencies]
rqrr = "0.8"  # 二维码解码库
sha2 = "0.10"  # SHA256 用于 shared_secret_hash
```

**已有依赖（无需添加）：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
base64.workspace = true
serde.workspace = true
serde_json.workspace = true
image.workspace = true
p256.workspace = true
```

### 代码模板

**qrcode_parser.rs:**
```rust
//! 二维码解析
//!
//! 提供二维码解析功能，用于扫描配对二维码。
//!
//! # Example
//!
//! ```ignore
//! use nearclip_crypto::{QrCodeParser, QrCodeGenerator, PairingData};
//!
//! // 生成二维码
//! let data = PairingData::new("device-id".to_string(), &[0x04; 65]);
//! let generator = QrCodeGenerator::new();
//! let png = generator.generate_png(&data).unwrap();
//!
//! // 解析二维码
//! let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
//! assert_eq!(parsed.device_id, "device-id");
//! ```

use crate::{CryptoError, PairingData};
use image::ImageReader;
use rqrr::PreparedImage;
use std::io::Cursor;
use tracing::{debug, instrument, warn};

/// 二维码解析器
pub struct QrCodeParser;

impl QrCodeParser {
    /// 从 PNG 图片字节解析二维码内容
    #[instrument(skip(png_data), fields(data_len = png_data.len()))]
    pub fn parse_from_bytes(png_data: &[u8]) -> Result<String, CryptoError> {
        // 1. 解码 PNG 图片
        let img = ImageReader::new(Cursor::new(png_data))
            .with_guessed_format()
            .map_err(|e| CryptoError::QrCodeParsing(format!("Invalid image format: {}", e)))?
            .decode()
            .map_err(|e| CryptoError::QrCodeParsing(format!("Failed to decode image: {}", e)))?;

        // 2. 转换为灰度图
        let gray = img.to_luma8();

        // 3. 准备二维码检测
        let mut prepared = PreparedImage::prepare(gray);

        // 4. 检测和解码二维码
        let grids = prepared.detect_grids();
        if grids.is_empty() {
            warn!("No QR code found in image");
            return Err(CryptoError::QrCodeParsing("No QR code found".to_string()));
        }

        // 5. 解码第一个检测到的二维码
        let (_, content) = grids[0]
            .decode()
            .map_err(|e| CryptoError::QrCodeParsing(format!("Failed to decode QR: {}", e)))?;

        debug!("QR code decoded, content length: {}", content.len());
        Ok(content)
    }

    /// 从 PNG 图片字节解析配对数据
    #[instrument(skip(png_data))]
    pub fn parse_pairing_data(png_data: &[u8]) -> Result<PairingData, CryptoError> {
        let json = Self::parse_from_bytes(png_data)?;
        let data = PairingData::from_json(&json)?;
        data.validate()?;
        Ok(data)
    }
}
```

### 配对流程说明

```
设备 A (显示二维码)              设备 B (扫描二维码)
─────────────────              ─────────────────
1. 生成 EcdhKeyPair
2. 创建 PairingData
3. 生成二维码 (PNG)
4. 显示二维码
                               5. 扫描二维码
                               6. 解析 PairingData
                               7. 验证数据
                               8. 计算 shared_secret
                               9. 创建 PairedDevice
                               10. (可选) 发送确认到 A
```

### 安全考虑

**共享密钥处理：**
- 不直接存储 `shared_secret`，只存储其 SHA256 哈希用于验证
- 使用 `zeroize` 确保内存中的密钥被清零
- 实际加密通信时使用 HKDF 派生会话密钥

**公钥验证：**
- 验证 Base64 格式正确
- 验证长度（33 字节压缩 或 65 字节未压缩）
- P-256 曲线点验证由 `p256` crate 处理

### 文件位置

**目标文件：**
- `crates/nearclip-crypto/src/qrcode_parser.rs` - 二维码解析（新建）
- `crates/nearclip-crypto/src/pairing.rs` - 添加 PairingSession, PairedDevice（修改）
- `crates/nearclip-crypto/src/keypair.rs` - 扩展 CryptoError（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-crypto/Cargo.toml` - 添加 rqrr, sha2 依赖（修改）
- `crates/nearclip-crypto/tests/pairing_flow_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-crypto/src/pairing.rs` - PairingData 定义和模式
- `crates/nearclip-crypto/src/keypair.rs` - EcdhKeyPair 和 CryptoError

### 与上游 Story 2-7 的关系

Story 2-7 已实现以下功能，本 Story 直接复用：
- `PairingData` 结构体和 JSON 序列化
- `PairingData::from_json()` 解析 JSON
- `PairingData::validate()` 验证数据完整性
- `QrCodeGenerator::generate_png()` 生成测试用二维码
- `ConnectionInfo` 结构体

### 与下游 Story 2-9 的关系

Story 2-9 将实现配对设备持久化，需要使用本 Story 定义的：
- `PairedDevice` 结构体（可序列化/反序列化）
- `PairingSession::complete()` 返回的配对结果

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-crypto/src/qrcode_parser.rs` - QrCodeParser（新建）
- `crates/nearclip-crypto/src/pairing.rs` - PairingSession, PairedDevice（修改）
- `crates/nearclip-crypto/src/keypair.rs` - CryptoError 扩展（修改）
- `crates/nearclip-crypto/src/lib.rs` - 模块导出和文档（修改）
- `crates/nearclip-crypto/Cargo.toml` - 添加 rqrr, sha2 依赖（修改）
- `crates/nearclip-crypto/tests/pairing_flow_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] qrcode_parser.rs 文件已创建
- [x] QrCodeParser 实现 parse_from_bytes 和 parse_pairing_data
- [x] PairingSession 结构体完整
- [x] PairedDevice 结构体完整（含 Serialize/Deserialize）
- [x] CryptoError 已扩展新变体
- [x] ECDH 密钥协商正常工作
- [x] 所有单元测试通过 (78 unit tests)
- [x] 集成测试验证完整配对流程 (12 integration tests + 13 qrcode tests)
- [x] `cargo build -p nearclip-crypto` 成功
- [x] `cargo test -p nearclip-crypto` 成功 (106 tests total)
- [x] `cargo clippy -p nearclip-crypto` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |
| 2025-12-15 | Implementation completed - QrCodeParser, PairingSession, PairedDevice |

---

Story created: 2025-12-15
Story completed: 2025-12-15
