# NearClip 架构 v2 实现指南

**状态**: 待开发
**优先级**: 高
**预计工期**: 5-6 周

---

## 背景

当前架构存在以下核心问题：
1. **单向配对**: Android 扫描 QR 码连接 macOS，但 macOS 不知道 Android 的设备信息
2. **平台逻辑分散**: BLE 扫描、连接、数据分片逻辑分散在 Swift/Kotlin 代码中
3. **设备存储分离**: 设备存储由平台实现，Rust 层无法直接管理

**目标**: Rust 层控制所有网络相关逻辑，平台只提供最小化的硬件访问接口。

---

## Phase 1: 设备管理层重构

### 目标
创建 `nearclip-device` crate，将设备存储从平台层移到 Rust 层。

### 任务清单

#### 1.1 创建 nearclip-device crate

```bash
# 在 crates/ 目录下创建
mkdir -p crates/nearclip-device/src
```

**Cargo.toml**:
```toml
[package]
name = "nearclip-device"
version = "0.1.0"
edition = "2021"

[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
uuid = { version = "1.0", features = ["v4"] }
chrono = "0.4"
```

**文件结构**:
```
crates/nearclip-device/src/
├── lib.rs           # 模块导出
├── error.rs         # 错误类型
├── store.rs         # DeviceStore (SQLite)
├── device.rs        # 设备数据结构
├── discovered.rs    # 发现的设备管理
├── paired.rs        # 已配对设备管理
├── connected.rs     # 已连接设备管理
└── manager.rs       # DeviceManager 统一入口
```

#### 1.2 实现 DeviceStore

**数据库表结构**:
```sql
-- 已配对设备表
CREATE TABLE IF NOT EXISTS paired_devices (
    device_id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    platform TEXT NOT NULL,
    public_key BLOB NOT NULL,
    shared_secret BLOB,  -- 加密存储
    paired_at INTEGER NOT NULL,
    last_connected INTEGER
);

-- 设备元数据表（可选）
CREATE TABLE IF NOT EXISTS device_metadata (
    device_id TEXT PRIMARY KEY,
    ble_peripheral_id TEXT,
    wifi_address TEXT,
    last_rssi INTEGER,
    FOREIGN KEY (device_id) REFERENCES paired_devices(device_id)
);
```

**核心接口**:
```rust
// src/store.rs
pub struct DeviceStore {
    conn: rusqlite::Connection,
}

impl DeviceStore {
    pub fn new(db_path: &Path) -> Result<Self, DeviceError>;
    pub fn save_device(&self, device: &PairedDevice) -> Result<(), DeviceError>;
    pub fn get_device(&self, device_id: &str) -> Result<Option<PairedDevice>, DeviceError>;
    pub fn remove_device(&self, device_id: &str) -> Result<(), DeviceError>;
    pub fn load_all_devices(&self) -> Result<Vec<PairedDevice>, DeviceError>;
    pub fn update_last_connected(&self, device_id: &str) -> Result<(), DeviceError>;
}
```

#### 1.3 实现 DeviceManager

```rust
// src/manager.rs
pub struct DeviceManager {
    store: DeviceStore,
    discovered: RwLock<HashMap<String, DiscoveredDevice>>,
    paired: RwLock<HashMap<String, PairedDevice>>,
    connected: RwLock<HashSet<String>>,
}

impl DeviceManager {
    pub fn new(db_path: &Path) -> Result<Self, DeviceError>;

    // 发现设备
    pub fn add_discovered(&self, device: DiscoveredDevice);
    pub fn remove_discovered(&self, device_id: &str);
    pub fn get_discovered(&self) -> Vec<DiscoveredDevice>;

    // 配对设备
    pub fn add_paired(&self, device: PairedDevice) -> Result<(), DeviceError>;
    pub fn remove_paired(&self, device_id: &str) -> Result<(), DeviceError>;
    pub fn get_paired(&self) -> Vec<PairedDevice>;
    pub fn is_paired(&self, device_id: &str) -> bool;

    // 连接状态
    pub fn set_connected(&self, device_id: &str, connected: bool);
    pub fn is_connected(&self, device_id: &str) -> bool;
    pub fn get_connected(&self) -> Vec<String>;
}
```

#### 1.4 更新 Cargo.toml (workspace)

在根目录 `Cargo.toml` 添加：
```toml
[workspace]
members = [
    "crates/nearclip-core",
    "crates/nearclip-device",  # 新增
    # ... 其他 crates
]
```

#### 1.5 集成到 nearclip-core

更新 `nearclip-core/Cargo.toml`:
```toml
[dependencies]
nearclip-device = { path = "../nearclip-device" }
```

更新 `NearClipManager` 使用 `DeviceManager`。

### 验收标准
- [ ] `nearclip-device` crate 编译通过
- [ ] 单元测试覆盖 DeviceStore CRUD 操作
- [ ] 集成测试验证设备持久化
- [ ] 移除 `FfiDeviceStorage` 接口（平台不再需要实现）

---

## Phase 2: BLE 层重构

### 目标
将所有 BLE 逻辑移到 Rust 层，平台只提供原始 BLE API。

### 任务清单

#### 2.1 简化 FfiBleHardware 接口

**新接口定义** (nearclip.udl):
```udl
callback interface FfiBleHardware {
    // 扫描
    void start_scan();
    void stop_scan();

    // 连接
    void connect(string peripheral_id);
    void disconnect(string peripheral_id);

    // GATT 操作
    [Throws=BleError]
    bytes read_characteristic(string peripheral_id, string service_uuid, string char_uuid);

    [Throws=BleError]
    void write_characteristic(string peripheral_id, string service_uuid, string char_uuid, bytes data);

    [Throws=BleError]
    void subscribe_characteristic(string peripheral_id, string service_uuid, string char_uuid);

    // 广播
    void start_advertising(bytes service_data);
    void stop_advertising();

    // 状态
    boolean is_connected(string peripheral_id);
    u16 get_mtu(string peripheral_id);
};
```

#### 2.2 移动数据分片到 Rust

将 `BleManager.kt` 和 `BleManager.swift` 中的 `DataChunker` 和 `DataReassembler` 移到 Rust：

```rust
// crates/nearclip-ble/src/chunk.rs (已存在，需增强)

pub struct DataChunker {
    message_id_counter: AtomicU32,
}

impl DataChunker {
    pub fn chunk(&self, data: &[u8], mtu: u16) -> Vec<Vec<u8>>;
}

pub struct DataReassembler {
    pending: HashMap<u32, PendingMessage>,
    timeout: Duration,
}

impl DataReassembler {
    pub fn add_chunk(&mut self, chunk: &[u8]) -> Option<Vec<u8>>;
    pub fn cleanup_expired(&mut self);
}
```

#### 2.3 重构 BleController

```rust
// crates/nearclip-ble/src/controller.rs

pub struct BleController {
    hardware: Arc<dyn BleHardware>,
    device_manager: Arc<DeviceManager>,

    // 映射
    peripheral_to_device: RwLock<HashMap<String, String>>,
    device_to_peripheral: RwLock<HashMap<String, String>>,

    // 数据处理
    chunker: DataChunker,
    reassemblers: RwLock<HashMap<String, DataReassembler>>,

    // 回调
    callback: Arc<dyn BleControllerCallback>,

    // 配置
    config: BleConfig,
}

impl BleController {
    // 扫描
    pub async fn start_discovery(&self) -> Result<(), BleError>;
    pub async fn stop_discovery(&self);

    // 连接（完整流程：连接 → 服务发现 → 读取设备信息）
    pub async fn connect(&self, device_id: &str) -> Result<(), BleError>;
    pub async fn disconnect(&self, device_id: &str) -> Result<(), BleError>;

    // 数据传输（自动分片/重组）
    pub async fn send(&self, device_id: &str, data: &[u8]) -> Result<(), BleError>;

    // 平台事件处理
    pub fn on_scan_result(&self, peripheral_id: &str, rssi: i32, adv_data: &[u8]);
    pub fn on_connected(&self, peripheral_id: &str);
    pub fn on_disconnected(&self, peripheral_id: &str);
    pub fn on_characteristic_changed(&self, peripheral_id: &str, char_uuid: &str, data: &[u8]);
}
```

#### 2.4 更新平台 BLE 实现

**Android (BleManager.kt)**:
- 移除 `DataChunker` 和 `DataReassembler` 类
- 移除 `peripheralDeviceIds` 映射
- 简化为只调用原始 BLE API
- 所有事件转发给 Rust 层

**macOS (BleManager.swift)**:
- 同样简化
- 移除业务逻辑

### 验收标准
- [ ] 平台 BLE 代码减少 50% 以上
- [ ] 数据分片/重组测试通过
- [ ] BLE 连接流程端到端测试通过

---

## Phase 3: 双向配对协议

### 目标
实现完整的双向配对握手，确保两端都保存对方信息。

### 任务清单

#### 3.1 定义配对协议消息

```rust
// crates/nearclip-protocol/src/pairing.rs (新文件或重构)

#[derive(Serialize, Deserialize, Debug)]
pub enum PairingMessage {
    /// 步骤 1: 发起方 → 接收方
    Request {
        version: u8,
        device_id: String,
        device_name: String,
        platform: String,
        public_key: Vec<u8>,
        nonce: [u8; 32],
        timestamp: i64,
    },

    /// 步骤 2: 接收方 → 发起方
    Response {
        device_id: String,
        device_name: String,
        platform: String,
        public_key: Vec<u8>,
        nonce: [u8; 32],
        signature: Vec<u8>,  // 签名: sign(request_nonce || response_nonce)
    },

    /// 步骤 3: 发起方 → 接收方
    Confirm {
        signature: Vec<u8>,  // 签名: sign(response_nonce || "confirm")
    },

    /// 步骤 4: 接收方 → 发起方
    Complete,

    /// 拒绝配对
    Rejected {
        reason: String,
    },
}
```

#### 3.2 实现配对状态机

```rust
// crates/nearclip-protocol/src/pairing_state.rs

pub enum PairingState {
    Idle,
    WaitingForResponse { request: PairingRequest, timeout: Instant },
    WaitingForConfirm { peer_info: PeerInfo, timeout: Instant },
    WaitingForComplete { timeout: Instant },
    Completed { peer_device: PairedDevice },
    Failed { reason: String },
}

pub struct PairingStateMachine {
    state: PairingState,
    crypto: Arc<CryptoEngine>,
    device_manager: Arc<DeviceManager>,
}

impl PairingStateMachine {
    pub fn initiate(&mut self, peer_public_key: &[u8]) -> Result<PairingMessage, PairingError>;
    pub fn handle_message(&mut self, msg: PairingMessage) -> Result<Option<PairingMessage>, PairingError>;
    pub fn is_complete(&self) -> bool;
    pub fn get_paired_device(&self) -> Option<&PairedDevice>;
}
```

#### 3.3 集成到 NearClipManager

```rust
// crates/nearclip-core/src/manager.rs

impl NearClipManager {
    /// 扫描 QR 码发起配对
    pub async fn pair_with_qr_code(&self, qr_data: &str) -> Result<DeviceInfo, NearClipError> {
        // 1. 解析 QR 码
        let pairing_data = parse_qr_code(qr_data)?;

        // 2. 连接到对方（BLE 或 WiFi）
        let transport = self.connect_for_pairing(&pairing_data).await?;

        // 3. 执行配对握手
        let mut state_machine = PairingStateMachine::new(...);
        let request = state_machine.initiate(&pairing_data.public_key)?;
        transport.send(&request.serialize()).await?;

        // 4. 等待响应并完成握手
        loop {
            let response = transport.recv().await?;
            let msg = PairingMessage::deserialize(&response)?;

            if let Some(reply) = state_machine.handle_message(msg)? {
                transport.send(&reply.serialize()).await?;
            }

            if state_machine.is_complete() {
                break;
            }
        }

        // 5. 保存配对设备
        let paired_device = state_machine.get_paired_device().unwrap();
        self.device_manager.add_paired(paired_device.clone())?;

        Ok(paired_device.into())
    }
}
```

#### 3.4 处理被动配对（作为 QR 码显示方）

```rust
impl NearClipManager {
    /// 处理收到的配对请求
    async fn handle_pairing_request(&self, request: PairingMessage, transport: &dyn Transport) -> Result<(), NearClipError> {
        // 1. 通知 UI 有配对请求
        self.callback.on_pairing_request(&request);

        // 2. 等待用户确认（或自动接受）
        // ...

        // 3. 执行配对握手（作为接收方）
        let mut state_machine = PairingStateMachine::new_responder(...);
        let response = state_machine.handle_message(request)?;

        if let Some(resp) = response {
            transport.send(&resp.serialize()).await?;
        }

        // 4. 继续握手直到完成
        // ...
    }
}
```

### 验收标准
- [ ] 配对协议单元测试通过
- [ ] Android 扫描 macOS QR 码，双方都显示对方设备
- [ ] macOS 扫描 Android QR 码，双方都显示对方设备
- [ ] 配对拒绝流程正常工作

---

## Phase 4: 传输层统一

### 目标
统一 WiFi/BLE 传输接口，实现无缝通道切换。

### 任务清单

#### 4.1 重构 Transport trait

```rust
// crates/nearclip-transport/src/traits.rs

#[async_trait]
pub trait Transport: Send + Sync {
    /// 发送数据
    async fn send(&self, data: &[u8]) -> Result<(), TransportError>;

    /// 接收数据
    async fn recv(&self) -> Result<Vec<u8>, TransportError>;

    /// 检查连接状态
    fn is_connected(&self) -> bool;

    /// 获取通道类型
    fn channel_type(&self) -> ChannelType;

    /// 关闭连接
    async fn close(&self) -> Result<(), TransportError>;

    /// 获取延迟估计（毫秒）
    fn estimated_latency(&self) -> Option<u32>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelType {
    Wifi,
    Ble,
}
```

#### 4.2 重构 TransportManager

```rust
// crates/nearclip-transport/src/manager.rs

pub struct TransportManager {
    wifi_transports: RwLock<HashMap<String, Arc<WifiTransport>>>,
    ble_transports: RwLock<HashMap<String, Arc<BleTransport>>>,
    preferred_channel: RwLock<HashMap<String, ChannelType>>,
    config: TransportConfig,
}

impl TransportManager {
    /// 发送数据（自动选择最佳通道）
    pub async fn send(&self, device_id: &str, data: &[u8]) -> Result<(), TransportError> {
        let transport = self.get_best_transport(device_id).await?;
        transport.send(data).await
    }

    /// 获取最佳传输通道
    async fn get_best_transport(&self, device_id: &str) -> Result<Arc<dyn Transport>, TransportError> {
        // 1. 检查是否有偏好通道
        if let Some(preferred) = self.preferred_channel.read().await.get(device_id) {
            if let Some(transport) = self.get_transport(device_id, *preferred).await {
                if transport.is_connected() {
                    return Ok(transport);
                }
            }
        }

        // 2. WiFi 优先
        if let Some(wifi) = self.wifi_transports.read().await.get(device_id) {
            if wifi.is_connected() {
                return Ok(wifi.clone());
            }
        }

        // 3. 回退到 BLE
        if let Some(ble) = self.ble_transports.read().await.get(device_id) {
            if ble.is_connected() {
                return Ok(ble.clone());
            }
        }

        Err(TransportError::NoAvailableChannel)
    }

    /// 连接设备（尝试所有可用通道）
    pub async fn connect(&self, device_id: &str) -> Result<ChannelType, TransportError>;

    /// 断开设备
    pub async fn disconnect(&self, device_id: &str) -> Result<(), TransportError>;

    /// 注册传输通道
    pub async fn register_wifi(&self, device_id: &str, transport: Arc<WifiTransport>);
    pub async fn register_ble(&self, device_id: &str, transport: Arc<BleTransport>);
}
```

#### 4.3 实现通道健康检查

```rust
// crates/nearclip-transport/src/health.rs

pub struct ChannelHealthChecker {
    transport_manager: Arc<TransportManager>,
    check_interval: Duration,
    timeout: Duration,
}

impl ChannelHealthChecker {
    pub fn start(&self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.check_interval);
            loop {
                interval.tick().await;
                self.check_all_channels().await;
            }
        });
    }

    async fn check_all_channels(&self) {
        // 发送心跳，检测超时，触发重连
    }
}
```

### 验收标准
- [ ] WiFi 断开时自动切换到 BLE
- [ ] WiFi 恢复时自动切换回 WiFi
- [ ] 通道切换对上层透明
- [ ] 心跳检测正常工作

---

## Phase 5: 平台适配

### 目标
更新 macOS 和 Android 平台代码，大幅简化。

### 任务清单

#### 5.1 更新 Android

**移除的代码**:
- `BleManager.kt` 中的 `DataChunker` 和 `DataReassembler`
- `BleManager.kt` 中的 `peripheralDeviceIds` 映射
- `NearClipService.kt` 中的 `DeviceStorageBridge`

**简化后的 BleManager.kt**:
```kotlin
class BleManager(private val context: Context) {
    // 只保留原始 BLE 操作
    fun startScanning()
    fun stopScanning()
    fun connect(peripheralAddress: String)
    fun disconnect(peripheralAddress: String)
    fun readCharacteristic(peripheralAddress: String, serviceUuid: UUID, charUuid: UUID): ByteArray?
    fun writeCharacteristic(peripheralAddress: String, serviceUuid: UUID, charUuid: UUID, data: ByteArray): Boolean
    fun subscribeCharacteristic(peripheralAddress: String, serviceUuid: UUID, charUuid: UUID): Boolean
    fun startAdvertising(serviceData: ByteArray)
    fun stopAdvertising()
    fun isConnected(peripheralAddress: String): Boolean
    fun getMtu(peripheralAddress: String): Int
}
```

#### 5.2 更新 macOS

**简化后的 BleManager.swift**:
```swift
class BleManager: NSObject, FfiBleHardware {
    // 只保留原始 BLE 操作
    func startScan()
    func stopScan()
    func connect(peripheralId: String)
    func disconnect(peripheralId: String)
    func readCharacteristic(peripheralId: String, serviceUuid: String, charUuid: String) throws -> Data
    func writeCharacteristic(peripheralId: String, serviceUuid: String, charUuid: String, data: Data) throws
    func subscribeCharacteristic(peripheralId: String, serviceUuid: String, charUuid: String) throws
    func startAdvertising(serviceData: Data)
    func stopAdvertising()
    func isConnected(peripheralId: String) -> Bool
    func getMtu(peripheralId: String) -> UInt16
}
```

#### 5.3 更新 FFI 调用

更新两个平台的 FFI 调用，使用新的简化接口。

### 验收标准
- [ ] Android 平台代码减少 40% 以上
- [ ] macOS 平台代码减少 40% 以上
- [ ] 端到端测试通过（macOS ↔ Android）
- [ ] 所有现有功能正常工作

---

## 测试计划

### 单元测试
- [ ] DeviceStore CRUD 操作
- [ ] 数据分片/重组
- [ ] 配对状态机
- [ ] 传输通道选择

### 集成测试
- [ ] 设备配对流程（WiFi）
- [ ] 设备配对流程（BLE）
- [ ] 剪贴板同步（WiFi）
- [ ] 剪贴板同步（BLE）
- [ ] 通道切换

### 端到端测试
- [ ] macOS → Android 配对和同步
- [ ] Android → macOS 配对和同步
- [ ] 断网恢复
- [ ] 多设备场景

---

## 风险和注意事项

1. **向后兼容**: 新版本无法与旧版本配对，需要清除旧配对数据
2. **SQLite 路径**: 需要平台提供数据库存储路径
3. **BLE 权限**: 确保平台正确请求 BLE 权限
4. **测试设备**: 需要真实设备测试，模拟器 BLE 支持有限

---

## 参考文档

- [架构设计文档](/docs/architecture-v2-redesign.md)
- [项目上下文](/docs/project_context.md)
- [现有架构文档](/docs/architecture.md)
