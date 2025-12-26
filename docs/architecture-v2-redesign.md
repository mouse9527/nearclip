# NearClip 架构重设计方案 v2

**日期**: 2025-12-26
**作者**: Winston (Architect Agent)
**状态**: 草案

---

## 1. 问题分析

### 1.1 当前架构的问题

经过代码分析和测试，当前架构存在以下核心问题：

#### BLE 层问题
| 问题 | 描述 | 影响 |
|------|------|------|
| **单向配对** | Android 扫描 QR 码连接 macOS，但 macOS 不知道 Android 的设备信息 | macOS 无法将 Android 添加为已配对设备 |
| **平台实现分散** | BLE 扫描、连接、数据传输逻辑分散在 Swift/Kotlin 平台代码中 | 行为不一致，难以调试 |
| **缺乏双向握手** | 没有配对握手协议，无法交换设备信息 | 只有发起方知道对方 |
| **硬件抽象不完整** | `FfiBleHardware` 接口过于底层，平台需要实现大量逻辑 | 代码重复，bug 难以定位 |

#### 设备管理问题
| 问题 | 描述 | 影响 |
|------|------|------|
| **存储分散** | 设备存储由平台实现（Keychain/SharedPreferences） | Rust 层无法直接管理设备状态 |
| **状态不同步** | 平台存储和 Rust 内存状态可能不一致 | 设备列表显示错误 |
| **缺乏统一 ID** | device_id 和 peripheral_uuid 映射复杂 | 连接检查失败 |

#### 传输层问题
| 问题 | 描述 | 影响 |
|------|------|------|
| **WiFi/BLE 切换复杂** | 通道切换逻辑分散在多个模块 | 切换不可靠 |
| **BLE 数据分片在平台层** | Android/macOS 各自实现分片逻辑 | 实现不一致 |
| **缺乏统一连接管理** | WiFi 和 BLE 连接独立管理 | 难以实现无缝切换 |

### 1.2 根本原因

```
当前架构：平台层做太多事情
┌─────────────────────────────────────────────────────────┐
│  Platform Layer (Swift/Kotlin)                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ BLE 扫描、连接、数据分片、设备存储、状态管理    │   │  ← 问题：逻辑分散
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                          ↓ FFI
┌─────────────────────────────────────────────────────────┐
│  Rust Layer                                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 协议处理、加密、消息序列化                       │   │  ← 问题：控制权不足
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 2. 设计目标

### 2.1 核心原则

> **"Rust 做所有网络相关的事情，平台只提供硬件访问"**

```
目标架构：Rust 层控制一切
┌─────────────────────────────────────────────────────────┐
│  Platform Layer (Swift/Kotlin)                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 仅提供：BLE 原始 API、剪贴板 API、UI            │   │  ← 最小化
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                          ↓ FFI (薄层)
┌─────────────────────────────────────────────────────────┐
│  Rust Layer                                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 设备发现、配对握手、连接管理、数据传输、加密、  │   │
│  │ 设备存储、状态管理、通道切换、重试逻辑          │   │  ← 完全控制
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### 2.2 具体目标

| 目标 | 描述 | 验收标准 |
|------|------|----------|
| **统一设备管理** | Rust 层管理所有设备状态和存储 | 平台无需实现 `FfiDeviceStorage` |
| **双向配对协议** | 设计完整的配对握手协议 | 两端都能看到对方设备 |
| **BLE 逻辑集中** | 所有 BLE 逻辑在 Rust 层 | 平台只提供原始 BLE API |
| **端到端加密** | BLE 通道也使用加密 | 即使 BLE 被监听也安全 |
| **无缝通道切换** | WiFi/BLE 自动切换 | 用户无感知 |
| **跨平台一致** | 所有平台行为一致 | 同一套测试用例 |

---

## 3. 新架构设计

### 3.1 Crate 重组

```
nearclip/
├── nearclip-core          # 核心协调层（保留，增强）
├── nearclip-protocol      # 新：统一协议层（合并 sync）
├── nearclip-device        # 新：设备管理层
├── nearclip-transport     # 传输抽象层（重构）
├── nearclip-wifi          # WiFi 传输实现（从 net 重命名）
├── nearclip-ble           # BLE 传输实现（重构）
├── nearclip-crypto        # 加密层（保留，增强）
└── nearclip-ffi           # FFI 绑定层（简化）
```

### 3.2 各层职责

#### nearclip-core（核心协调层）
```rust
// 职责：统一入口，协调所有模块
pub struct NearClipManager {
    device_manager: DeviceManager,      // 设备管理
    transport_manager: TransportManager, // 传输管理
    protocol_handler: ProtocolHandler,   // 协议处理
    crypto_engine: CryptoEngine,         // 加密引擎
}

impl NearClipManager {
    // 生命周期
    pub async fn start(&self) -> Result<()>;
    pub async fn stop(&self) -> Result<()>;

    // 配对（核心流程）
    pub async fn pair_with_qr_code(&self, qr_data: &str) -> Result<DeviceInfo>;
    pub async fn generate_qr_code(&self) -> Result<QrCodeData>;

    // 同步
    pub async fn sync_clipboard(&self, content: &[u8]) -> Result<()>;

    // 设备管理
    pub fn get_paired_devices(&self) -> Vec<DeviceInfo>;
    pub fn get_connected_devices(&self) -> Vec<DeviceInfo>;
    pub async fn unpair_device(&self, device_id: &str) -> Result<()>;
}
```

#### nearclip-device（设备管理层）- 新增
```rust
// 职责：设备发现、配对、存储、状态管理
pub struct DeviceManager {
    store: DeviceStore,           // 持久化存储
    discovered: DiscoveredDevices, // 发现的设备
    paired: PairedDevices,        // 已配对设备
    connected: ConnectedDevices,  // 已连接设备
}

// 设备存储（Rust 实现，使用 SQLite）
pub struct DeviceStore {
    db: rusqlite::Connection,
}

impl DeviceStore {
    pub fn save_device(&self, device: &PairedDevice) -> Result<()>;
    pub fn remove_device(&self, device_id: &str) -> Result<()>;
    pub fn load_all_devices(&self) -> Result<Vec<PairedDevice>>;
    pub fn get_device(&self, device_id: &str) -> Result<Option<PairedDevice>>;
}

// 已配对设备（包含加密密钥）
pub struct PairedDevice {
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    pub public_key: Vec<u8>,        // 对方公钥
    pub shared_secret: Vec<u8>,     // 共享密钥（ECDH）
    pub paired_at: i64,
    pub last_connected: Option<i64>,
}
```

#### nearclip-protocol（协议层）- 重构
```rust
// 职责：定义所有消息类型和配对协议

// 配对协议消息
#[derive(Serialize, Deserialize)]
pub enum PairingMessage {
    // 步骤 1：发起方发送配对请求
    PairingRequest {
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
    },
    // 步骤 2：接收方发送配对响应
    PairingResponse {
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
        signature: Vec<u8>,  // 签名验证
    },
    // 步骤 3：发起方确认
    PairingConfirm {
        signature: Vec<u8>,
    },
    // 配对完成
    PairingComplete,
    // 配对拒绝
    PairingRejected { reason: String },
}

// 数据传输消息
#[derive(Serialize, Deserialize)]
pub enum DataMessage {
    ClipboardSync {
        content: Vec<u8>,      // 加密后的内容
        content_type: String,  // MIME 类型
        timestamp: i64,
    },
    Heartbeat {
        timestamp: i64,
    },
    Ack {
        message_id: u64,
    },
    Unpair {
        reason: String,
    },
}
```

#### nearclip-transport（传输抽象层）- 重构
```rust
// 职责：统一传输抽象，管理 WiFi/BLE 通道

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, data: &[u8]) -> Result<()>;
    async fn recv(&self) -> Result<Vec<u8>>;
    fn is_connected(&self) -> bool;
    fn channel_type(&self) -> ChannelType;
    async fn close(&self) -> Result<()>;
}

pub struct TransportManager {
    wifi: Option<WifiTransport>,
    ble: Option<BleTransport>,
    active_channel: ChannelType,
}

impl TransportManager {
    // 自动选择最佳通道发送
    pub async fn send(&self, device_id: &str, data: &[u8]) -> Result<()>;

    // 接收数据（任意通道）
    pub async fn recv(&self) -> Result<(String, Vec<u8>)>; // (device_id, data)

    // 通道管理
    pub async fn connect(&self, device_id: &str) -> Result<()>;
    pub async fn disconnect(&self, device_id: &str) -> Result<()>;
    pub fn get_channel(&self, device_id: &str) -> Option<ChannelType>;
}
```

#### nearclip-ble（BLE 传输层）- 重构
```rust
// 职责：BLE 传输实现，所有逻辑在 Rust 层

pub struct BleTransport {
    hardware: Arc<dyn BleHardware>,  // 平台提供的原始 API
    controller: BleController,        // BLE 控制逻辑
    chunker: DataChunker,            // 数据分片（移到 Rust）
}

// 平台只需实现这个最小接口
pub trait BleHardware: Send + Sync {
    // 扫描
    fn start_scan(&self);
    fn stop_scan(&self);

    // 连接（原始 BLE 操作）
    fn connect(&self, peripheral_id: &str);
    fn disconnect(&self, peripheral_id: &str);

    // GATT 操作
    fn read_characteristic(&self, peripheral_id: &str, char_uuid: &str) -> Result<Vec<u8>>;
    fn write_characteristic(&self, peripheral_id: &str, char_uuid: &str, data: &[u8]) -> Result<()>;
    fn subscribe_characteristic(&self, peripheral_id: &str, char_uuid: &str) -> Result<()>;

    // 广播
    fn start_advertising(&self, service_data: &[u8]);
    fn stop_advertising(&self);

    // 状态查询
    fn is_connected(&self, peripheral_id: &str) -> bool;
    fn get_mtu(&self, peripheral_id: &str) -> u16;
}

// BLE 控制器（所有逻辑在 Rust）
pub struct BleController {
    // 设备映射
    peripheral_to_device: HashMap<String, String>,
    device_to_peripheral: HashMap<String, String>,

    // 连接状态
    connected_devices: HashSet<String>,

    // 配置
    config: BleConfig,
}

impl BleController {
    // 扫描并发现设备
    pub async fn discover_devices(&self, timeout: Duration) -> Vec<DiscoveredDevice>;

    // 连接设备（包含服务发现、特征读取）
    pub async fn connect(&self, device_id: &str) -> Result<()>;

    // 发送数据（自动分片）
    pub async fn send(&self, device_id: &str, data: &[u8]) -> Result<()>;

    // 接收数据（自动重组）
    pub async fn recv(&self) -> Result<(String, Vec<u8>)>;
}
```

### 3.3 配对流程重设计

```
┌─────────────────┐                              ┌─────────────────┐
│   Device A      │                              │   Device B      │
│   (Scanner)     │                              │   (QR Display)  │
└────────┬────────┘                              └────────┬────────┘
         │                                                │
         │  1. 扫描 QR 码获取 B 的信息                    │
         │     {device_id, public_key, ble_service_data}  │
         │                                                │
         │  2. 通过 BLE 或 WiFi 连接到 B                  │
         │ ─────────────────────────────────────────────> │
         │                                                │
         │  3. 发送 PairingRequest                        │
         │     {A.device_id, A.name, A.public_key, nonce} │
         │ ─────────────────────────────────────────────> │
         │                                                │
         │                    4. B 验证请求，计算共享密钥  │
         │                       保存 A 的信息            │
         │                                                │
         │  5. 发送 PairingResponse                       │
         │     {B.device_id, B.name, B.public_key, sig}   │
         │ <───────────────────────────────────────────── │
         │                                                │
         │  6. A 验证响应，计算共享密钥                   │
         │     保存 B 的信息                              │
         │                                                │
         │  7. 发送 PairingConfirm                        │
         │     {signature}                                │
         │ ─────────────────────────────────────────────> │
         │                                                │
         │                    8. B 验证确认               │
         │                       配对完成                 │
         │                                                │
         │  9. 发送 PairingComplete                       │
         │ <───────────────────────────────────────────── │
         │                                                │
         │  ═══════════════════════════════════════════   │
         │           双方都已保存对方信息                 │
         │           可以开始同步剪贴板                   │
         │  ═══════════════════════════════════════════   │
```

### 3.4 加密设计

```rust
// nearclip-crypto 增强

pub struct CryptoEngine {
    keypair: EcdhKeyPair,           // 本机密钥对
    device_keys: HashMap<String, SharedKey>, // 每设备共享密钥
}

impl CryptoEngine {
    // 生成配对数据（用于 QR 码）
    pub fn generate_pairing_data(&self) -> PairingData;

    // 从配对数据计算共享密钥
    pub fn derive_shared_key(&self, peer_public_key: &[u8]) -> SharedKey;

    // 加密数据（使用设备特定密钥）
    pub fn encrypt(&self, device_id: &str, plaintext: &[u8]) -> Result<Vec<u8>>;

    // 解密数据
    pub fn decrypt(&self, device_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>>;
}

// 加密方案
// - 密钥交换：ECDH (P-256)
// - 对称加密：AES-256-GCM
// - 密钥派生：HKDF-SHA256
```

### 3.5 FFI 接口简化

```udl
// nearclip.udl - 简化版

namespace nearclip {
    void init_logging(LogLevel level);
};

interface FfiNearClipManager {
    [Throws=NearClipError]
    constructor(FfiConfig config, FfiCallback callback);

    // 生命周期
    [Throws=NearClipError]
    void start();
    void stop();
    boolean is_running();

    // 配对（核心）
    [Throws=NearClipError]
    FfiDeviceInfo pair_with_qr_code(string qr_data);

    [Throws=NearClipError]
    string generate_qr_code();  // 返回 QR 码数据

    // 同步
    [Throws=NearClipError]
    void sync_clipboard(bytes content);

    // 设备管理
    sequence<FfiDeviceInfo> get_paired_devices();
    sequence<FfiDeviceInfo> get_connected_devices();

    [Throws=NearClipError]
    void unpair_device(string device_id);

    // BLE 硬件设置（平台调用一次）
    void set_ble_hardware(FfiBleHardware hardware);

    // BLE 事件回调（平台调用）
    void on_ble_scan_result(string peripheral_id, i32 rssi, bytes adv_data);
    void on_ble_connected(string peripheral_id);
    void on_ble_disconnected(string peripheral_id);
    void on_ble_characteristic_read(string peripheral_id, string char_uuid, bytes data);
    void on_ble_characteristic_changed(string peripheral_id, string char_uuid, bytes data);
};

// 最小化的 BLE 硬件接口
callback interface FfiBleHardware {
    void start_scan();
    void stop_scan();
    void connect(string peripheral_id);
    void disconnect(string peripheral_id);
    string read_characteristic(string peripheral_id, string char_uuid);  // 返回错误或空
    string write_characteristic(string peripheral_id, string char_uuid, bytes data);
    string subscribe_characteristic(string peripheral_id, string char_uuid);
    void start_advertising(bytes service_data);
    void stop_advertising();
    boolean is_connected(string peripheral_id);
    u16 get_mtu(string peripheral_id);
};

// 回调接口
callback interface FfiCallback {
    void on_device_connected(FfiDeviceInfo device);
    void on_device_disconnected(string device_id);
    void on_clipboard_received(bytes content, string from_device);
    void on_pairing_request(FfiDeviceInfo device);  // 新：配对请求通知
    void on_error(string error_message);
};
```

---

## 4. 实现路径

### 4.1 阶段划分

```
Phase 1: 基础重构（1-2 周）
├── 创建 nearclip-device crate
├── 实现 DeviceStore（SQLite）
├── 重构 nearclip-protocol
└── 设计配对协议消息

Phase 2: BLE 重构（1-2 周）
├── 简化 FfiBleHardware 接口
├── 将数据分片移到 Rust 层
├── 实现 BleController 完整逻辑
└── 更新平台 BLE 实现

Phase 3: 配对流程（1 周）
├── 实现双向配对握手
├── 集成加密密钥交换
├── 测试 WiFi 和 BLE 配对
└── 修复边界情况

Phase 4: 传输层统一（1 周）
├── 重构 TransportManager
├── 实现无缝通道切换
├── 添加连接健康检查
└── 端到端测试

Phase 5: 平台适配（1 周）
├── 更新 macOS Swift 代码
├── 更新 Android Kotlin 代码
├── 简化平台层代码
└── 集成测试
```

### 4.2 详细任务

#### Phase 1: 基础重构

**任务 1.1: 创建 nearclip-device crate**
```bash
# 文件结构
crates/nearclip-device/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── store.rs        # DeviceStore (SQLite)
    ├── discovered.rs   # DiscoveredDevices
    ├── paired.rs       # PairedDevices
    ├── connected.rs    # ConnectedDevices
    └── manager.rs      # DeviceManager
```

**任务 1.2: 实现 DeviceStore**
```rust
// 使用 SQLite 存储设备信息
// 表结构：
// - devices: id, name, platform, public_key, shared_secret, paired_at, last_connected
// - 加密存储 shared_secret（使用平台密钥库派生的密钥）
```

**任务 1.3: 重构配对协议**
```rust
// 定义完整的配对消息类型
// 实现配对状态机
// 添加签名验证
```

#### Phase 2: BLE 重构

**任务 2.1: 简化 BLE 硬件接口**
- 移除平台层的数据分片逻辑
- 移除平台层的设备 ID 映射
- 只保留原始 BLE 操作

**任务 2.2: 实现 Rust 层 BLE 控制器**
- 设备发现和服务解析
- 连接管理和状态跟踪
- 数据分片和重组
- 自动重连逻辑

#### Phase 3: 配对流程

**任务 3.1: 实现双向握手**
- PairingRequest → PairingResponse → PairingConfirm → PairingComplete
- 两端都保存对方信息
- 支持配对拒绝

**任务 3.2: 集成加密**
- ECDH 密钥交换
- 共享密钥派生
- 后续通信加密

#### Phase 4: 传输层统一

**任务 4.1: 重构 TransportManager**
- 统一 WiFi/BLE 接口
- 自动通道选择
- 无缝切换

**任务 4.2: 健康检查**
- 心跳机制
- 连接超时检测
- 自动重连

#### Phase 5: 平台适配

**任务 5.1: 更新 macOS**
- 简化 BleManager.swift
- 移除设备存储逻辑
- 更新 FFI 调用

**任务 5.2: 更新 Android**
- 简化 BleManager.kt
- 移除设备存储逻辑
- 更新 FFI 调用

---

## 5. 风险和缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| BLE API 平台差异 | 接口设计困难 | 设计最小公共接口，平台特定逻辑用适配器 |
| SQLite 在移动端性能 | 存储延迟 | 使用 WAL 模式，批量写入 |
| 配对协议复杂度 | 实现 bug | 详细状态机设计，充分测试 |
| 向后兼容 | 旧版本无法配对 | 协议版本号，优雅降级 |

---

## 6. 成功标准

- [ ] 所有 BLE 逻辑在 Rust 层，平台代码 < 200 行
- [ ] 双向配对成功，两端都显示对方设备
- [ ] WiFi/BLE 无缝切换，用户无感知
- [ ] 端到端加密，BLE 通道也安全
- [ ] 单元测试覆盖率 > 80%
- [ ] 集成测试通过（macOS ↔ Android）

---

## 7. 附录

### 7.1 QR 码数据格式 v2

```json
{
  "version": 2,
  "device_id": "UUID",
  "device_name": "My Mac",
  "platform": "macos",
  "public_key": "base64...",
  "ble_service_uuid": "4E454152-434C-4950-0000-000000000001",
  "wifi_port": 12345,  // 可选
  "timestamp": 1703577600000
}
```

### 7.2 GATT 服务定义

```
Service UUID: 4E454152-434C-4950-0000-000000000001 ("NEARCLIP")

Characteristics:
- Device ID:       4E454152-434C-4950-0000-000000000002 (Read)
- Public Key Hash: 4E454152-434C-4950-0000-000000000003 (Read)
- Data TX:         4E454152-434C-4950-0000-000000000004 (Write No Response)
- Data RX:         4E454152-434C-4950-0000-000000000005 (Notify)
- Pairing:         4E454152-434C-4950-0000-000000000006 (Write, Notify) - 新增
```

### 7.3 数据分片格式

```
Chunk Header (8 bytes):
┌──────────────┬──────────────┬──────────────┬──────────────┐
│ Message ID   │ Sequence     │ Total        │ Flags        │
│ (4 bytes)    │ (2 bytes)    │ (2 bytes)    │ (reserved)   │
└──────────────┴──────────────┴──────────────┴──────────────┘

Chunk Payload:
┌──────────────────────────────────────────────────────────┐
│ Data (MTU - 8 bytes)                                     │
└──────────────────────────────────────────────────────────┘
```

---

**下一步**: 请确认此设计方案，我将开始 Phase 1 的实现。
