# Story 2.1: 实现 ECDH 密钥对生成

Status: Review

## Story

As a 用户,
I want 设备能生成安全的密钥对,
So that 可以与其他设备建立加密通信.

## Acceptance Criteria

1. **Given** nearclip-crypto crate 已创建 **When** 调用密钥生成函数 **Then** 生成 ECDH P-256 密钥对
2. **And** 私钥可安全存储（导出为字节数组）
3. **And** 公钥可导出为字节数组（用于配对交换）
4. **And** 可以使用私钥和对方公钥计算共享密钥
5. **And** 单元测试验证密钥对有效性

## Tasks / Subtasks

- [x] Task 1: 添加依赖并创建 keypair.rs 模块 (AC: 1) ✅
  - [x] 1.1 在 workspace Cargo.toml 添加 `p256` 依赖（替代 ring，因 ring 不支持私钥导出）
  - [x] 1.2 在 nearclip-crypto/Cargo.toml 添加 `p256.workspace = true`
  - [x] 1.3 在 `crates/nearclip-crypto/src/` 创建 `keypair.rs`
  - [x] 1.4 定义 `CryptoError` 错误类型

- [x] Task 2: 实现 EcdhKeyPair 结构体 (AC: 1, 2, 3) ✅
  - [x] 2.1 定义 `EcdhKeyPair` 结构体，包含 p256 的 SecretKey 和 PublicKey
  - [x] 2.2 实现 `EcdhKeyPair::generate()` 生成新密钥对
  - [x] 2.3 实现 `private_key_bytes()` 导出私钥字节
  - [x] 2.4 实现 `public_key_bytes()` 导出公钥字节
  - [x] 2.5 实现 `from_private_key_bytes()` 从私钥恢复密钥对

- [x] Task 3: 实现共享密钥计算 (AC: 4) ✅
  - [x] 3.1 实现 `compute_shared_secret()` 计算 ECDH 共享密钥
  - [x] 3.2 验证共享密钥双向一致（A 私钥 + B 公钥 = B 私钥 + A 公钥）

- [x] Task 4: 导出模块 (AC: 1) ✅
  - [x] 4.1 在 `lib.rs` 中添加 `pub mod keypair;`
  - [x] 4.2 重新导出 `EcdhKeyPair` 和 `CryptoError`
  - [x] 4.3 移除 `#![allow(dead_code)]` 和 `#![allow(unused_variables)]`

- [x] Task 5: 编写单元测试 (AC: 5) ✅
  - [x] 5.1 测试密钥对生成不会 panic
  - [x] 5.2 测试公钥导出长度正确（65 字节 uncompressed）
  - [x] 5.3 测试私钥导出长度正确（32 字节）
  - [x] 5.4 测试从私钥恢复密钥对
  - [x] 5.5 测试共享密钥计算一致性
  - [x] 5.6 测试无效私钥字节错误处理

- [x] Task 6: 验证构建 (AC: 1, 2, 3, 4, 5) ✅
  - [x] 6.1 运行 `cargo build` 确保无错误
  - [x] 6.2 运行 `cargo test -p nearclip-crypto` 确保测试通过
  - [x] 6.3 运行 `cargo clippy` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, CryptoError>` 或 `Result<T, NearClipError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**安全规则：**
- ✅ 密钥存储使用平台密钥库（Keychain/Keystore）- 后续 Story 实现
- ❌ 禁止明文存储配对密钥（本 Story 仅生成，存储由平台层负责）
- ❌ 禁止在日志中输出敏感数据（私钥、共享密钥）

**加密模块在架构中的位置：**
```
nearclip-ffi → nearclip-core → nearclip-sync/net/ble → nearclip-crypto
```
nearclip-crypto 是最底层的 crate，不依赖其他内部 crate。

### 文件位置

**目标文件：**
- `crates/nearclip-crypto/src/keypair.rs` - ECDH 密钥对实现
- `crates/nearclip-crypto/src/lib.rs` - 模块导出
- `crates/nearclip-crypto/Cargo.toml` - 添加依赖
- `Cargo.toml` (workspace) - 添加 ring 依赖

### 技术栈版本

| 依赖 | 版本 | 用途 |
|------|------|------|
| ring | 0.17.x | ECDH P-256 密钥生成和计算 |
| thiserror | workspace | 错误类型 |
| tracing | workspace | 日志记录 |

**为什么选择 ring：**
- rustls 0.23 已使用 ring 作为加密后端（features = ["ring"]）
- ring 是高性能、审计过的加密库
- 避免引入额外的加密依赖
- 支持 P-256 ECDH 密钥协商

### ECDH P-256 技术细节

**密钥长度：**
| 类型 | 长度 | 格式 |
|------|------|------|
| 私钥 | 32 字节 | 标量值 |
| 公钥（未压缩）| 65 字节 | 0x04 + X(32) + Y(32) |
| 共享密钥 | 32 字节 | ECDH 计算结果 |

**ring API 使用：**
```rust
use ring::agreement::{self, EphemeralPrivateKey, UnparsedPublicKey};

// 生成密钥对
let rng = ring::rand::SystemRandom::new();
let private_key = EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng)?;
let public_key = private_key.compute_public_key()?;

// 计算共享密钥
agreement::agree_ephemeral(
    private_key,
    &UnparsedPublicKey::new(&agreement::ECDH_P256, peer_public_key_bytes),
    |shared_secret| { /* use shared_secret */ }
)?;
```

**注意：** ring 的 `EphemeralPrivateKey` 设计为一次性使用，无法直接导出私钥字节。需要研究替代方案：
1. 使用 `ring::signature::EcdsaKeyPair` 配合 PKCS#8 格式
2. 使用 `p256` crate 替代 ring 的 ECDH（更灵活的密钥管理）

**推荐方案：使用 p256 crate**
由于需要支持私钥导出和从字节恢复，建议使用 `p256` crate：

```rust
use p256::ecdh::EphemeralSecret;
use p256::PublicKey;

// 生成密钥对
let private_key = EphemeralSecret::random(&mut OsRng);
let public_key = PublicKey::from(&private_key);

// 导出
let public_bytes = public_key.to_sec1_bytes(); // 33 or 65 bytes
```

### 依赖变更建议

在 workspace Cargo.toml 添加：
```toml
[workspace.dependencies]
# 在 Crypto & TLS 部分添加
p256 = "0.13"
rand_core = { version = "0.6", features = ["getrandom"] }
```

在 nearclip-crypto/Cargo.toml 添加：
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
rustls.workspace = true
p256.workspace = true
rand_core.workspace = true
```

### 代码模板

**keypair.rs 完整模板：**
```rust
//! ECDH P-256 密钥对生成和管理
//!
//! 提供设备配对所需的密钥对生成、导出和共享密钥计算功能。

use p256::{
    ecdh::EphemeralSecret,
    elliptic_curve::sec1::ToEncodedPoint,
    PublicKey, SecretKey,
};
use rand_core::OsRng;
use thiserror::Error;
use tracing::{debug, instrument};

/// 加密模块错误类型
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CryptoError {
    /// 无效的私钥字节
    #[error("Invalid private key bytes: {0}")]
    InvalidPrivateKey(String),

    /// 无效的公钥字节
    #[error("Invalid public key bytes: {0}")]
    InvalidPublicKey(String),

    /// 密钥协商失败
    #[error("Key agreement failed: {0}")]
    KeyAgreementFailed(String),
}

/// ECDH P-256 密钥对
///
/// 用于设备配对时的密钥交换。
///
/// # Example
///
/// ```
/// use nearclip_crypto::keypair::EcdhKeyPair;
///
/// // 生成新密钥对
/// let keypair = EcdhKeyPair::generate();
///
/// // 导出公钥用于交换
/// let public_bytes = keypair.public_key_bytes();
///
/// // 导出私钥用于安全存储
/// let private_bytes = keypair.private_key_bytes();
/// ```
#[derive(Clone)]
pub struct EcdhKeyPair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl EcdhKeyPair {
    /// 生成新的 ECDH P-256 密钥对
    #[instrument]
    pub fn generate() -> Self {
        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = secret_key.public_key();
        debug!("Generated new ECDH P-256 keypair");
        Self { secret_key, public_key }
    }

    /// 从私钥字节恢复密钥对
    ///
    /// # Arguments
    ///
    /// * `bytes` - 32 字节的私钥
    ///
    /// # Returns
    ///
    /// 恢复的密钥对，或错误如果字节无效
    #[instrument(skip(bytes))]
    pub fn from_private_key_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let secret_key = SecretKey::from_slice(bytes)
            .map_err(|e| CryptoError::InvalidPrivateKey(e.to_string()))?;
        let public_key = secret_key.public_key();
        debug!("Restored ECDH keypair from private key bytes");
        Ok(Self { secret_key, public_key })
    }

    /// 导出私钥字节
    ///
    /// 返回 32 字节的私钥标量值。
    ///
    /// **警告：** 私钥必须安全存储，不可记录到日志。
    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }

    /// 导出公钥字节（未压缩格式）
    ///
    /// 返回 65 字节的公钥（0x04 + X + Y）。
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key
            .to_encoded_point(false) // uncompressed
            .as_bytes()
            .to_vec()
    }

    /// 导出公钥字节（压缩格式）
    ///
    /// 返回 33 字节的公钥（0x02/0x03 + X）。
    pub fn public_key_bytes_compressed(&self) -> Vec<u8> {
        self.public_key
            .to_encoded_point(true) // compressed
            .as_bytes()
            .to_vec()
    }

    /// 计算与对方公钥的共享密钥
    ///
    /// # Arguments
    ///
    /// * `peer_public_bytes` - 对方的公钥字节（33 或 65 字节）
    ///
    /// # Returns
    ///
    /// 32 字节的共享密钥
    #[instrument(skip(self, peer_public_bytes))]
    pub fn compute_shared_secret(&self, peer_public_bytes: &[u8]) -> Result<Vec<u8>, CryptoError> {
        use p256::ecdh::diffie_hellman;
        use p256::elliptic_curve::sec1::FromEncodedPoint;
        use p256::EncodedPoint;

        let encoded_point = EncodedPoint::from_bytes(peer_public_bytes)
            .map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))?;

        let peer_public = PublicKey::from_encoded_point(&encoded_point);
        let peer_public = Option::from(peer_public)
            .ok_or_else(|| CryptoError::InvalidPublicKey("Invalid point on curve".to_string()))?;

        let shared_secret = diffie_hellman(
            self.secret_key.to_nonzero_scalar(),
            peer_public.as_affine(),
        );

        debug!("Computed shared secret successfully");
        Ok(shared_secret.raw_secret_bytes().to_vec())
    }
}

impl std::fmt::Debug for EcdhKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 不输出私钥内容
        f.debug_struct("EcdhKeyPair")
            .field("public_key", &hex::encode(self.public_key_bytes()))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = EcdhKeyPair::generate();
        assert_eq!(keypair.private_key_bytes().len(), 32);
        assert_eq!(keypair.public_key_bytes().len(), 65);
    }

    #[test]
    fn test_public_key_uncompressed_format() {
        let keypair = EcdhKeyPair::generate();
        let public_bytes = keypair.public_key_bytes();
        assert_eq!(public_bytes[0], 0x04); // uncompressed marker
    }

    #[test]
    fn test_public_key_compressed_format() {
        let keypair = EcdhKeyPair::generate();
        let public_bytes = keypair.public_key_bytes_compressed();
        assert_eq!(public_bytes.len(), 33);
        assert!(public_bytes[0] == 0x02 || public_bytes[0] == 0x03);
    }

    #[test]
    fn test_restore_from_private_key() {
        let original = EcdhKeyPair::generate();
        let private_bytes = original.private_key_bytes();

        let restored = EcdhKeyPair::from_private_key_bytes(&private_bytes).unwrap();

        assert_eq!(original.public_key_bytes(), restored.public_key_bytes());
    }

    #[test]
    fn test_invalid_private_key() {
        let invalid_bytes = vec![0u8; 31]; // wrong length
        let result = EcdhKeyPair::from_private_key_bytes(&invalid_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_secret_consistency() {
        let alice = EcdhKeyPair::generate();
        let bob = EcdhKeyPair::generate();

        let alice_shared = alice.compute_shared_secret(&bob.public_key_bytes()).unwrap();
        let bob_shared = bob.compute_shared_secret(&alice.public_key_bytes()).unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn test_shared_secret_with_compressed_key() {
        let alice = EcdhKeyPair::generate();
        let bob = EcdhKeyPair::generate();

        let alice_shared = alice.compute_shared_secret(&bob.public_key_bytes_compressed()).unwrap();
        let bob_shared = bob.compute_shared_secret(&alice.public_key_bytes()).unwrap();

        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn test_invalid_public_key() {
        let keypair = EcdhKeyPair::generate();
        let invalid_public = vec![0u8; 65]; // invalid point

        let result = keypair.compute_shared_secret(&invalid_public);
        assert!(result.is_err());
    }

    #[test]
    fn test_debug_does_not_leak_private_key() {
        let keypair = EcdhKeyPair::generate();
        let debug_str = format!("{:?}", keypair);

        let private_hex = hex::encode(keypair.private_key_bytes());
        assert!(!debug_str.contains(&private_hex));
    }
}
```

**lib.rs 更新模板：**
```rust
//! NearClip Crypto Module
//!
//! Cryptographic primitives for secure device pairing and communication.
//! Includes ECDH key exchange, TLS 1.3 configuration, and key management.

pub mod keypair;

// Re-export main types
pub use keypair::{CryptoError, EcdhKeyPair};

// Future modules:
// mod tls_config;   // TLS 1.3 configuration
// mod qrcode;       // QR code generation/parsing
// mod keystore;     // Platform key storage abstraction
```

**Cargo.toml 依赖添加（workspace）：**
```toml
[workspace.dependencies]
# Crypto & TLS 部分
p256 = "0.13"
rand_core = { version = "0.6", features = ["getrandom"] }
hex = "0.4"  # 用于测试和 Debug 输出
```

**Cargo.toml 依赖添加（nearclip-crypto）：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
rustls.workspace = true
p256.workspace = true
rand_core.workspace = true

[dev-dependencies]
hex.workspace = true
```

### References

- [Source: docs/architecture.md#Core Architectural Decisions]
- [Source: docs/project_context.md#Critical Implementation Rules]
- [Source: docs/epics.md#Story 2.1]
- [p256 crate documentation](https://docs.rs/p256)
- [ring ECDH documentation](https://docs.rs/ring/latest/ring/agreement/index.html)

## Dev Agent Record

### Context Reference

Story context created by create-story workflow with exhaustive artifact analysis.

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- cargo build: SUCCESS
- cargo test -p nearclip-crypto: 15 unit tests + 6 doc tests passed (21 total)
- cargo clippy: No warnings

### Completion Notes List

- 使用 `p256` crate 替代 `ring`，因为 ring 的 `EphemeralPrivateKey` 不支持私钥导出
- 添加 `p256 = { version = "0.13", features = ["ecdh"] }` 到 workspace 依赖
- 添加 `rand_core` 和 `hex` 依赖
- `CryptoError` 定义了三个变体: InvalidPrivateKey, InvalidPublicKey, KeyAgreementFailed
- `EcdhKeyPair` 实现了 Clone trait，便于密钥对复制
- Debug trait 实现安全，不输出私钥内容，只显示公钥前缀
- 支持压缩和未压缩两种公钥格式
- 共享密钥计算支持两种公钥格式输入
- 全面的单元测试覆盖所有功能和错误情况

### File List

**New Files:**
- crates/nearclip-crypto/src/keypair.rs

**Modified Files:**
- crates/nearclip-crypto/src/lib.rs
- crates/nearclip-crypto/Cargo.toml
- Cargo.toml (workspace)

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] keypair.rs 文件已创建
- [x] CryptoError 错误类型已定义
- [x] EcdhKeyPair 结构体实现完整
- [x] generate() 方法正确生成密钥对
- [x] private_key_bytes() 返回 32 字节
- [x] public_key_bytes() 返回 65 字节
- [x] from_private_key_bytes() 可恢复密钥对
- [x] compute_shared_secret() 计算正确
- [x] 所有单元测试通过
- [x] `cargo build` 成功
- [x] `cargo test -p nearclip-crypto` 成功
- [x] `cargo clippy` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-13 | Story created by create-story workflow |
| 2025-12-13 | Implementation completed - all tasks done, 21 tests passing |

---

Story created: 2025-12-13
Implementation completed: 2025-12-13
