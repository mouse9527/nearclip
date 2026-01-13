# Task 2.1 实施计划：BLE 传输加密

**任务**: 实现 BLE 传输端到端加密
**优先级**: 🔴 最高
**估计时间**: 10-14 小时
**开始日期**: 2026-01-13

---

## 执行摘要

为 BLE 传输添加端到端加密，使用配对时交换的 ECDH 共享密钥。

### 当前状态分析

**✅ 已完成的基础设施**:
1. `nearclip-crypto::Aes256Gcm` - AES-256-GCM 加密器（完整）
2. `nearclip-transport::EncryptedTransport` - 加密传输包装器（完整）
3. ECDH 密钥交换协议（任务 1.3 完成）
4. BLE Transport 分片/重组逻辑（完整）

**❌ 缺失的部分**:
1. **shared_secret 派生逻辑** - 配对时生成 ECDH 共享密钥
2. **BLE Transport 加密集成** - 使用 `EncryptedTransport` 包装
3. **密钥管理** - 存储和检索设备加密密钥
4. **FFI 配置** - 允许启用/禁用加密

---

## 架构设计

### 当前架构

```
┌──────────────────────────────────────┐
│   NearClipManager                    │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│   BleTransport                       │
│   - send() → serialize → chunk       │
│   - recv() → reassemble → parse      │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│   Platform BLE (Swift/Kotlin)        │
└──────────────────────────────────────┘
```

**问题**: 没有加密，数据以明文传输

### 目标架构

```
┌──────────────────────────────────────┐
│   NearClipManager                    │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│   EncryptedTransport                 │ ← NEW!
│   - send() → encrypt                 │
│   - recv() → decrypt                 │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│   BleTransport                       │
│   - send() → serialize → chunk       │
│   - recv() → reassemble → parse      │
└──────────────┬───────────────────────┘
               │
┌──────────────▼───────────────────────┐
│   Platform BLE (Swift/Kotlin)        │
└──────────────────────────────────────┘
```

**优势**:
- 透明加密：上层无需感知
- 使用现有 `EncryptedTransport`
- 分片在加密之后发生

---

## 实施步骤

### Step 1: 实现 ECDH 共享密钥派生 (4 小时)

#### 1.1 在 `nearclip-crypto` 中添加 ECDH 支持

**文件**: `crates/nearclip-crypto/src/ecdh.rs` (新文件)

```rust
//! ECDH 密钥交换和密钥派生

use p256::{
    ecdh::EphemeralSecret,
    PublicKey,
};
use hkdf::Hkdf;
use sha2::Sha256;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EcdhError {
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Key derivation failed")]
    KeyDerivationFailed,
}

/// Derive shared secret from ECDH key exchange
///
/// # Arguments
/// * `local_private_key` - Our ephemeral private key (32 bytes)
/// * `remote_public_key` - Peer's public key (65 bytes uncompressed)
///
/// # Returns
/// 32-byte shared secret
pub fn derive_shared_secret(
    local_private_key: &[u8],
    remote_public_key: &[u8],
) -> Result<Vec<u8>, EcdhError> {
    // Parse keys
    let secret = EphemeralSecret::from_bytes(local_private_key)
        .map_err(|_| EcdhError::InvalidPublicKey)?;

    let peer_public = PublicKey::from_sec1_bytes(remote_public_key)
        .map_err(|_| EcdhError::InvalidPublicKey)?;

    // Perform ECDH
    let shared_secret = secret.diffie_hellman(&peer_public);

    // Derive encryption key using HKDF-SHA256
    let hk = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
    let mut okm = vec![0u8; 32]; // AES-256 key
    hk.expand(b"nearclip-encryption-v1", &mut okm)
        .map_err(|_| EcdhError::KeyDerivationFailed)?;

    Ok(okm)
}
```

#### 1.2 在配对流程中调用密钥派生

**文件**: `crates/nearclip-device/src/pairing.rs`

**修改位置 1** (第 255 行):
```rust
// 当前
shared_secret: vec![], // TODO: derive shared secret

// 修改为
use nearclip_crypto::ecdh::derive_shared_secret;

shared_secret: derive_shared_secret(
    &self.local_private_key,
    &resp.public_key
).map_err(|e| PairingError::CryptoError(e.to_string()))?,
```

**修改位置 2** (第 355 行):
```rust
// 当前
shared_secret: vec![], // TODO: derive shared secret

// 修改为
shared_secret: derive_shared_secret(
    &self.local_private_key,
    &request.public_key
).map_err(|e| PairingError::CryptoError(e.to_string()))?,
```

#### 1.3 确保私钥可用

**需要验证**: `PairingManager` 是否存储了本地私钥？

查看 `pairing.rs` 中的 `PairingManager` 结构：
```rust
pub struct PairingManager {
    local_device_id: String,
    local_device_name: String,
    local_platform: String,
    local_public_key: Vec<u8>,  // ✅ 有公钥
    // ❌ 缺少私钥！
}
```

**需要添加**:
```rust
local_private_key: Vec<u8>,  // ECDH P-256 私钥
```

**验收标准**:
- [x] `derive_shared_secret()` 函数实现完成
- [ ] 配对时成功派生共享密钥
- [ ] 共享密钥存储到 `PairedDevice`
- [ ] 单元测试覆盖

---

### Step 2: 集成 EncryptedTransport 到 BLE Transport (3 小时)

#### 2.1 修改 BleTransport 构造函数

**文件**: `crates/nearclip-transport/src/ble.rs`

**当前**:
```rust
impl BleTransport {
    pub fn new(device_id: String, sender: Arc<dyn BleSender>) -> Self {
        Self {
            device_id,
            sender,
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            connected: AtomicBool::new(true),
            message_id_counter: AtomicU16::new(0),
            reassemblers: Arc::new(Mutex::new(HashMap::new())),
            pending_acks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
```

**修改为**:
```rust
pub struct BleTransport {
    device_id: String,
    sender: Arc<dyn BleSender>,
    recv_queue: Arc<Mutex<VecDeque<Message>>>,
    recv_notify: Arc<Notify>,
    connected: AtomicBool,
    message_id_counter: AtomicU16,
    reassemblers: Arc<Mutex<HashMap<u16, Reassembler>>>,
    pending_acks: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>>,
    /// Optional encryption cipher
    encryption: Option<Aes256Gcm>,  // NEW!
}

impl BleTransport {
    /// Create new BLE transport with optional encryption
    pub fn new(
        device_id: String,
        sender: Arc<dyn BleSender>,
        shared_secret: Option<&[u8]>,  // NEW!
    ) -> Result<Self, TransportError> {
        let encryption = if let Some(secret) = shared_secret {
            Some(Aes256Gcm::new(secret)
                .map_err(|e| TransportError::Other(format!("Encryption init failed: {}", e)))?)
        } else {
            None
        };

        Ok(Self {
            device_id,
            sender,
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            connected: AtomicBool::new(true),
            message_id_counter: AtomicU16::new(0),
            reassemblers: Arc::new(Mutex::new(HashMap::new())),
            pending_acks: Arc::new(Mutex::new(HashMap::new())),
            encryption,
        })
    }
}
```

#### 2.2 修改 send() 方法支持加密

**文件**: `crates/nearclip-transport/src/ble.rs`

**在 send() 方法中**:
```rust
async fn send(&self, msg: &Message) -> Result<(), TransportError> {
    // Serialize message
    let data = msg.serialize()
        .map_err(|e| TransportError::Serialization(e.to_string()))?;

    // Encrypt if encryption is enabled
    let data = if let Some(ref cipher) = self.encryption {
        cipher.encrypt(&data)
            .map_err(|e| TransportError::Other(format!("Encryption failed: {}", e)))?
    } else {
        data
    };

    // Rest of chunking logic...
}
```

#### 2.3 修改 on_data_received() 方法支持解密

**在 process_chunk() 后**:
```rust
if let Some(msg) = result.message {
    // Decrypt if encryption is enabled
    let decrypted_msg = if let Some(ref cipher) = self.encryption {
        // Message payload is encrypted, decrypt it
        let decrypted_data = cipher.decrypt(&msg.payload)
            .map_err(|e| TransportError::Other(format!("Decryption failed: {}", e)))?;
        Message::deserialize(&decrypted_data)
            .map_err(|e| TransportError::Deserialization(e.to_string()))?
    } else {
        msg
    };

    // Queue decrypted message
    let mut queue = self.recv_queue.lock().await;
    queue.push_back(decrypted_msg);
    self.recv_notify.notify_one();
}
```

**验收标准**:
- [ ] BleTransport 接受可选的 shared_secret
- [ ] 发送时自动加密
- [ ] 接收时自动解密
- [ ] 加密开销 < 10%

---

### Step 3: 更新 BleController 和 FFI (2-3 小时)

#### 3.1 BleController 传递共享密钥

**文件**: `crates/nearclip-ble/src/controller.rs`

**需要**:
- 在创建 `BleTransport` 时从 `DeviceManager` 获取共享密钥
- 传递给 `BleTransport::new()`

#### 3.2 FFI 层配置

**文件**: `crates/nearclip-ffi/src/lib.rs`

**添加配置选项**:
```rust
pub struct FfiBleConfig {
    pub enable_encryption: bool,  // NEW!
}
```

**验收标准**:
- [ ] BleController 正确传递共享密钥
- [ ] FFI 允许配置加密开关
- [ ] 默认启用加密

---

### Step 4: 测试和验证 (2-3 小时)

#### 4.1 单元测试

**文件**: `crates/nearclip-crypto/src/ecdh.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_shared_secret() {
        // Test vectors from NIST
        let private_key = [/* ... */];
        let public_key = [/* ... */];

        let secret = derive_shared_secret(&private_key, &public_key).unwrap();
        assert_eq!(secret.len(), 32);
    }
}
```

#### 4.2 集成测试

**文件**: `crates/nearclip-transport/tests/ble_encryption_test.rs` (新文件)

```rust
#[tokio::test]
async fn test_encrypted_ble_transport() {
    // Create two transports with shared secret
    let shared_secret = [0u8; 32];

    let transport1 = BleTransport::new(
        "device1".to_string(),
        sender1,
        Some(&shared_secret),
    ).unwrap();

    let transport2 = BleTransport::new(
        "device2".to_string(),
        sender2,
        Some(&shared_secret),
    ).unwrap();

    // Send message from transport1
    let msg = Message::new(MessageType::Heartbeat, vec![1, 2, 3], "device2".to_string());
    transport1.send(&msg).await.unwrap();

    // Receive on transport2 (should decrypt automatically)
    let received = transport2.recv().await.unwrap();
    assert_eq!(received.payload, vec![1, 2, 3]);
}
```

#### 4.3 性能测试

**测试加密开销**:
- 发送 10MB 数据，测量加密时间
- 目标：加密开销 < 10% of total time

**验收标准**:
- [ ] 所有单元测试通过
- [ ] 集成测试通过
- [ ] 加密/解密正确性验证
- [ ] 性能测试达标

---

## 文件修改清单

### 新增文件
| 文件 | 行数估计 | 描述 |
|------|---------|------|
| `crates/nearclip-crypto/src/ecdh.rs` | ~80 | ECDH 密钥派生 |
| `crates/nearclip-transport/tests/ble_encryption_test.rs` | ~100 | 集成测试 |

### 修改文件
| 文件 | 修改范围 | 描述 |
|------|---------|------|
| `crates/nearclip-crypto/src/lib.rs` | +2 行 | 导出 `ecdh` 模块 |
| `crates/nearclip-device/src/pairing.rs` | 2 处修改 | 调用密钥派生 |
| `crates/nearclip-transport/src/ble.rs` | ~50 行修改 | 添加加密支持 |
| `crates/nearclip-ble/src/controller.rs` | ~20 行修改 | 传递共享密钥 |
| `crates/nearclip-ffi/src/lib.rs` | ~10 行添加 | 配置选项 |
| `Cargo.toml` (多个) | 依赖添加 | `hkdf`, `p256` |

---

## 依赖关系

### Cargo 依赖

**`nearclip-crypto/Cargo.toml`**:
```toml
[dependencies]
p256 = { version = "0.13", features = ["ecdh"] }
hkdf = "0.12"
sha2 = "0.10"  # 已有
```

### 模块依赖

```
nearclip-crypto (ecdh)
    ↓
nearclip-device (pairing)
    ↓
nearclip-transport (ble)
    ↓
nearclip-ble (controller)
    ↓
nearclip-ffi
```

---

## 风险和缓解

### 风险 1: ECDH 密钥格式不兼容
**影响**: 高
**概率**: 中
**缓解**:
- 使用标准 P-256 曲线
- 公钥使用 SEC1 uncompressed 格式（65 字节）
- 提前验证密钥格式

### 风险 2: 加密性能下降
**影响**: 中
**概率**: 低
**缓解**:
- AES-256-GCM 硬件加速（现代 CPU）
- 性能测试和基准
- 如需要可调整加密算法

### 风险 3: 密钥存储安全性
**影响**: 高
**概率**: 低
**缓解**:
- macOS 已使用 Keychain（任务 1.2 完成）
- Android 需要确认使用 Android Keystore
- 共享密钥不应明文日志

---

## 时间估算

| 步骤 | 预计时间 |
|------|---------|
| Step 1: ECDH 密钥派生 | 4 小时 |
| Step 2: BLE 加密集成 | 3 小时 |
| Step 3: BleController/FFI | 2 小时 |
| Step 4: 测试和验证 | 3 小时 |
| **总计** | **12 小时** |

---

## 验收标准

### 功能验收
- [ ] 配对时成功派生 ECDH 共享密钥
- [ ] BLE 传输数据使用 AES-256-GCM 加密
- [ ] 发送端自动加密，接收端自动解密
- [ ] 密钥存储在 `PairedDevice.shared_secret`
- [ ] FFI 层可配置加密开关

### 性能验收
- [ ] 加密开销 < 10%
- [ ] BLE 传输延迟无明显增加（< 10ms）
- [ ] 内存使用稳定

### 安全验收
- [ ] 使用 HKDF-SHA256 派生密钥
- [ ] 每次加密生成唯一 nonce
- [ ] 共享密钥加密存储（Keychain/Keystore）
- [ ] 无密钥泄漏到日志

---

## 下一步行动

### 立即开始
1. ✅ 创建此实施计划
2. ⏳ 添加 `ecdh.rs` 模块
3. ⏳ 实现密钥派生函数
4. ⏳ 修改配对流程
5. ⏳ 集成到 BLE Transport

### 后续任务
- 手动测试配对和加密传输
- 性能基准测试
- 安全审计

---

**文档创建**: 2026-01-13
**作者**: Mouse（与 Claude Code 协作）
**状态**: 规划中
