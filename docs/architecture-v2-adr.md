# NearClip 架构决策记录 v2

**文档类型**: 架构决策
**版本**: 2.0
**日期**: 2025-12-26
**状态**: 批准执行
**作者**: Winston (Architect)

---

## 1. 执行摘要

本文档记录 NearClip v2 架构的关键决策。核心原则是 **"Rust 做所有网络相关的事情，平台只提供硬件访问"**。

**关键变更**:
- 设备管理从平台层迁移到 Rust 层（SQLite）
- BLE 所有逻辑集中到 Rust 层
- 实现双向配对握手协议
- 端到端加密覆盖所有通道（WiFi + BLE）

---

## 2. 背景

### 2.1 当前架构问题

| 问题 | 影响 |
|------|------|
| BLE 配对单向 | macOS 无法看到 Android 设备 |
| 平台实现分散 | Swift/Kotlin 各自实现相同逻辑 |
| 设备存储在平台 | Rust 层无法直接管理状态 |
| BLE 分片在平台 | 实现不一致，bug 难定位 |

### 2.2 根本原因

```
平台层职责过重：
- BLE 扫描、连接、数据分片
- 设备存储（Keychain/SharedPreferences）
- 状态管理

Rust 层控制不足：
- 无法直接管理设备
- 无法控制 BLE 逻辑细节
- 协议执行依赖平台配合
```

---

## 3. 架构原则

### 3.1 核心原则

> **AD-001: Rust-First 网络层**
> 所有网络相关逻辑必须在 Rust 层实现。平台层仅提供硬件访问抽象。

**理由**:
- 跨平台行为一致
- 代码复用，减少维护成本
- 类型安全，减少 bug
- 便于测试

**后果**:
- 平台代码减少约 50%
- Rust 代码增加约 20%
- FFI 接口简化

### 3.2 设备管理原则

> **AD-002: 统一设备存储**
> 设备信息存储在 Rust 层 SQLite，平台不维护设备状态。

**理由**:
- 单一数据源，避免不一致
- 跨平台同步状态
- 简化平台代码

**后果**:
- 移除 `FfiDeviceStorage` 接口
- Rust 层管理 CRUD 操作
- 平台只通过 FFI 查询设备

### 3.3 配对协议原则

> **AD-003: 双向配对握手**
> 配对必须是双向的，两端都保存对方设备信息。

**理由**:
- 当前单向配对导致设备列表不同步
- 允许任一端发起配对
- 支持点对点拓扑

**后果**:
- 新增配对握手消息类型
- 配对流程增加 2 个来回
- 需要用户确认（可选）

### 3.4 加密原则

> **AD-004: 端到端加密**
> 所有通道（WiFi、BLE）的数据传输必须加密。

**理由**:
- BLE 不安全，易被监听
- 保护用户隐私
- 防止中间人攻击

**后果**:
- BLE 数据传输增加加密层
- 性能轻微下降（可接受）
- 需要管理每设备密钥

---

## 4. 架构设计

### 4.1 模块组织

```
nearclip (workspace)
├── nearclip-core          # 核心协调层
├── nearclip-device        # 设备管理层 [新增]
├── nearclip-protocol      # 协议层 [重构]
├── nearclip-transport     # 传输抽象层
├── nearclip-wifi          # WiFi 传输 [重命名]
├── nearclip-ble           # BLE 传输 [重构]
├── nearclip-crypto        # 加密层
└── nearclip-ffi           # FFI 绑定层 [简化]
```

### 4.2 设备管理层 (nearclip-device)

**职责**: 设备发现、配对、存储、状态管理

```rust
pub struct DeviceManager {
    store: DeviceStore,           // SQLite 存储
    discovery: DiscoveryManager,   // 设备发现
    pairing: PairingManager,      // 配对管理
    connections: ConnectionTracker, // 连接跟踪
}

pub struct DeviceStore {
    db: rusqlite::Connection,
}

// 数据库表
CREATE TABLE devices (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    platform TEXT NOT NULL,
    public_key BLOB NOT NULL,
    shared_secret BLOB NOT NULL,
    paired_at INTEGER NOT NULL,
    last_connected INTEGER,
    last_seen INTEGER
);

CREATE INDEX idx_devices_platform ON devices(platform);
CREATE INDEX idx_devices_last_seen ON devices(last_seen);
```

### 4.3 配对协议

**消息序列**:

```
Scanner                    Responder
   │                          │
   │  1. Scan QR Code         │
   │ ──────────────────────> │
   │     {device_id, pk, ...} │
   │                          │
   │  2. Connect (WiFi/BLE)   │
   │ ──────────────────────> │
   │                          │
   │  3. PairingRequest       │
   │ ──────────────────────> │
   │                          │
   │  4. PairingResponse      │
   │ <────────────────────── │
   │                          │
   │  5. PairingConfirm       │
   │ ──────────────────────> │
   │                          │
   │  6. PairingComplete      │
   │ <────────────────────── │
   │                          │
   ═══ ═══ ═══ ═══ ═══ ═══ ══
      Both paired and saved
```

**消息定义**:

```rust
#[derive(Serialize, Deserialize)]
pub enum PairingMessage {
    PairingRequest {
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
    },
    PairingResponse {
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
        signature: Vec<u8>,
    },
    PairingConfirm {
        signature: Vec<u8>,
    },
    PairingComplete,
    PairingRejected {
        reason: String,
    },
}
```

### 4.4 BLE 层重构

**平台接口简化**:

```rust
// 之前：平台实现高级逻辑
trait FfiBleHardware {
    fn start_scan(&self);
    fn connect(&self, peripheral_uuid: String);
    fn write_data(&self, peripheral_uuid: String, data: Vec<u8>) -> String;
    fn is_connected(&self, peripheral_uuid: String) -> bool;
    // ... 约 10 个方法
}

// 之后：平台只提供原始 API
trait BleHardware {
    fn start_scan(&self);
    fn stop_scan(&self);
    fn connect(&self, peripheral_id: &str);
    fn disconnect(&self, peripheral_id: &str);
    fn read_characteristic(&self, peripheral_id: &str, uuid: &str) -> Result<Vec<u8>>;
    fn write_characteristic(&self, peripheral_id: &str, uuid: &str, data: &[u8]) -> Result<()>;
    fn subscribe_characteristic(&self, peripheral_id: &str, uuid: &str) -> Result<()>;
    fn start_advertising(&self, service_data: &[u8]);
    fn stop_advertising(&self);
    fn is_connected(&self, peripheral_id: &str) -> bool;
    fn get_mtu(&self, peripheral_id: &str) -> u16;
}
```

**Rust 层接管**:

- 设备发现和服务解析
- 连接状态管理
- 数据分片和重组
- 配对协议执行

### 4.5 加密设计

**密钥管理**:

```rust
pub struct CryptoEngine {
    keypair: EcdhKeyPair,                      // 本机密钥对
    shared_keys: HashMap<String, SharedKey>,  // 设备 -> 共享密钥
}

impl CryptoEngine {
    // 配对时调用
    pub fn derive_shared_key(
        &mut self,
        device_id: &str,
        peer_public_key: &[u8],
    ) -> Result<SharedKey>;

    // 发送前加密
    pub fn encrypt(
        &self,
        device_id: &str,
        plaintext: &[u8],
    ) -> Result<Vec<u8>>;

    // 接收后解密
    pub fn decrypt(
        &self,
        device_id: &str,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>>;
}
```

**加密方案**:
- 密钥交换: ECDH (P-256)
- 对称加密: AES-256-GCM
- 密钥派生: HKDF-SHA256
- 签名: Ed25519 (配对确认)

---

## 5. FFI 接口

### 5.1 简化后的接口

```udl
namespace nearclip {
    enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    };

    void init_logging(LogLevel level);

    [Error]
    interface NearClipError {
        Network(string message);
        Bluetooth(string message);
        Crypto(string message);
        DeviceNotFound(string device_id);
        PairingFailed(string reason);
        InvalidQrCode(string details);
    };

    [Throws=NearClipError]
    interface NearClipManager {
        constructor(Config config, Callback callback);

        // 生命周期
        void start();
        void stop();
        boolean is_running();

        // 配对
        DeviceInfo pair_with_qr_code(string qr_data);
        string generate_qr_code();

        // 同步
        void sync_clipboard(bytes content);

        // 设备管理
        sequence<DeviceInfo> get_paired_devices();
        sequence<DeviceInfo> get_connected_devices();
        void unpair_device(string device_id);

        // BLE 硬件设置
        void set_ble_hardware(BleHardware hardware);

        // BLE 事件（平台调用）
        void on_ble_scan_result(string peripheral_id, i32 rssi, bytes adv_data);
        void on_ble_connected(string peripheral_id);
        void on_ble_disconnected(string peripheral_id);
        void on_ble_characteristic_read(string peripheral_id, string char_uuid, bytes data);
        void on_ble_characteristic_changed(string peripheral_id, string char_uuid, bytes data);
    };

    callback interface BleHardware {
        void start_scan();
        void stop_scan();
        void connect(string peripheral_id);
        void disconnect(string peripheral_id);
        [Throws=NearClipError]
        bytes read_characteristic(string peripheral_id, string char_uuid);
        [Throws=NearClipError]
        void write_characteristic(string peripheral_id, string char_uuid, bytes data);
        [Throws=NearClipError]
        void subscribe_characteristic(string peripheral_id, string char_uuid);
        void start_advertising(bytes service_data);
        void stop_advertising();
        boolean is_connected(string peripheral_id);
        u16 get_mtu(string peripheral_id);
    };

    callback interface Callback {
        void on_device_connected(DeviceInfo device);
        void on_device_disconnected(string device_id);
        void on_clipboard_received(bytes content, string from_device);
        void on_error(string message);
    };

    struct Config {
        string device_name;
        boolean wifi_enabled;
        boolean ble_enabled;
        boolean auto_connect;
    };

    struct DeviceInfo {
        string id;
        string name;
        DevicePlatform platform;
    };

    enum DevicePlatform {
        MacOS,
        Android,
        Windows,
        Ios,
    };
}
```

### 5.2 移除的接口

- ~~`FfiDeviceStorage`~~ → 设备存储在 Rust 层
- ~~`FfiBleControllerConfig`~~ → 配置在 Rust 层
- ~~`FfiSyncHistoryEntry`~~ → 历史记录在 Rust 层

---

## 6. 数据流

### 6.1 配对流程

```
┌──────────────────────────────────────────────────────────────────┐
│                         配对序列图                               │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Android (Scanner)              macOS (Responder)               │
│                                                                  │
│  1. 用户扫描 macOS 的 QR 码                                      │
│     → 获取 {device_id, public_key, ...}                         │
│                                                                  │
│  2. pair_with_qr_code(qr_data)                                  │
│     → 解析 QR 数据                                               │
│     → 通过 BLE 连接到 macOS                                      │
│                                                                  │
│  3. 发送 PairingRequest                                         │
│     {device_id: A, public_key: A_pk, nonce: N1}                 │
│     ─────────────────────────────────────────────────────────>  │
│                                                                  │
│  4. macOS 收到请求                                               │
│     → 用户确认（可选）                                           │
│     → 计算共享密钥: ECDH(A_pk, B_sk)                             │
│     → 保存设备 A 到数据库                                        │
│     → 发送 PairingResponse                                       │
│     {device_id: B, public_key: B_pk, nonce: N2, sig: SIGN(B)}  │
│     <─────────────────────────────────────────────────────────  │
│                                                                  │
│  5. Android 收到响应                                             │
│     → 计算共享密钥: ECDH(B_pk, A_sk)                             │
│     → 验证签名                                                   │
│     → 保存设备 B 到数据库                                        │
│     → 发送 PairingConfirm                                       │
│     {sig: SIGN(A)}                                               │
│     ─────────────────────────────────────────────────────────>  │
│                                                                  │
│  6. macOS 验证确认                                               │
│     → 配对完成                                                   │
│     → 发送 PairingComplete                                       │
│     <─────────────────────────────────────────────────────────  │
│                                                                  │
│  ═══════════════════════════════════════════════════════════════ │
│              双方都保存了对方的设备信息                          │
│              可以开始双向同步剪贴板                              │
│  ═══════════════════════════════════════════════════════════════ │
└──────────────────────────────────────────────────────────────────┘
```

### 6.2 剪贴板同步流程

```
┌──────────────────────────────────────────────────────────────────┐
│                    剪贴板同步序列图                              │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  macOS                          Android                         │
│  (剪贴板变化)                                                   │
│                                                                  │
│  1. ClipboardMonitor 监听到变化                                 │
│     → 获取剪贴板内容                                             │
│     → 调用 sync_clipboard(content)                              │
│                                                                  │
│  2. NearClipManager.sync_clipboard()                            │
│     → 获取已连接设备列表                                         │
│     → 对每个设备：                                               │
│       - CryptoEngine.encrypt(device_id, content)                │
│       - TransportManager.send(device_id, encrypted_data)        │
│                                                                  │
│  3. TransportManager 选择通道                                    │
│     → WiFi 优先，BLE 备选                                        │
│     → 自动选择最佳通道                                           │
│                                                                  │
│  4. WiFi/BLE 传输                                                │
│     ─────────────────────────────────────────────────────────>  │
│                                                                  │
│  5. Android 接收数据                                            │
│     → TransportManager.recv()                                   │
│     → CryptoEngine.decrypt(device_id, data)                     │
│     → Callback.on_clipboard_received()                           │
│     → ClipboardWriter 写入剪贴板                                 │
│                                                                  │
│  6. 发送 ACK                                                     │
│     <─────────────────────────────────────────────────────────  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 7. 实施计划

### 7.1 阶段划分

| 阶段 | 时长 | 交付物 | 依赖 |
|------|------|--------|------|
| Phase 1 | 1-2 周 | nearclip-device crate | - |
| Phase 2 | 1-2 周 | BLE 层重构 | Phase 1 |
| Phase 3 | 1 周 | 双向配对协议 | Phase 2 |
| Phase 4 | 1 周 | 传输层统一 | Phase 3 |
| Phase 5 | 1 周 | 平台适配 | Phase 4 |

**总计**: 5-7 周

### 7.2 风险管理

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| BLE API 平台差异大 | 中 | 高 | 设计最小公共接口，使用适配器模式 |
| SQLite 性能问题 | 低 | 中 | 使用 WAL 模式，批量写入 |
| 配对协议复杂度 | 中 | 中 | 详细状态机，充分测试 |
| 向后兼容性 | 低 | 低 | 协议版本号，优雅降级 |

---

## 8. 验收标准

- [ ] 所有网络逻辑在 Rust 层实现
- [ ] 平台 BLE 代码 < 200 行
- [ ] 双向配对成功，两端都显示设备
- [ ] WiFi/BLE 无缝切换
- [ ] 端到端加密覆盖所有通道
- [ ] 单元测试覆盖率 > 80%
- [ ] 集成测试通过（macOS ↔ Android）

---

## 9. 参考资料

- 当前架构文档: `/Users/mouse/project/mouse/nearclip/docs/architecture.md`
- 项目上下文: `/Users/mouse/project/mouse/nearclip/docs/project_context.md`
- 架构重设计: `/Users/mouse/project/mouse/nearclip/docs/architecture-v2-redesign.md`

---

**文档结束**

下一步：创建技术规范供 dev 代理执行。
