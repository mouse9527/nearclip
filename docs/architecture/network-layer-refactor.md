# NearClip 网络层重构架构设计

## 1. 概述

### 1.1 重构目标

将所有网络逻辑（WiFi + BLE）完全封装在 Rust 层，平台侧只需要：
1. 实现 Rust 定义的 BLE 硬件抽象接口（依赖倒置）
2. 访问系统剪贴板 API
3. 展示 UI

### 1.2 当前问题

| 问题 | 现状 | 影响 |
|------|------|------|
| BLE 代码重复 | macOS 1123 行 + Android 1095 行 | 维护成本高，容易不一致 |
| 逻辑分散 | 配对、重连、健康检查在平台层 | 难以统一行为 |
| 历史存储分散 | 两端各自实现 | 数据格式不一致 |
| 设备存储重复 | Rust + 平台层都有 | 状态同步问题 |

### 1.3 目标架构

```
┌─────────────────────────────────────────────────────────────┐
│                   平台层 (~200 行)                           │
│  - 实现 FfiBleHardware 接口                                  │
│  - 剪贴板系统 API                                            │
│  - UI 展示                                                   │
└──────────────────────┬───────────────────────────────────────┘
                       │ FFI (高层抽象)
┌──────────────────────▼───────────────────────────────────────┐
│                    Rust 层 (完整网络栈)                      │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ NearClipManager (统一入口)                              │  │
│  └────────────────────────────────────────────────────────┘  │
│                           │                                  │
│  ┌────────────────────────▼───────────────────────────────┐  │
│  │ BleController (新增 - BLE 控制逻辑)                     │  │
│  │  - 扫描控制 (开始/停止/过滤)                            │  │
│  │  - 连接管理 (连接/断开/重连)                            │  │
│  │  - 健康检查                                             │  │
│  │  - 配对验证                                             │  │
│  └────────────────────────────────────────────────────────┘  │
│                           │                                  │
│  ┌────────────────────────▼───────────────────────────────┐  │
│  │ TransportManager (已有)                                 │  │
│  │  - WifiTransport                                        │  │
│  │  - BleTransport                                         │  │
│  │  - 通道选择/故障转移                                     │  │
│  └────────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ HistoryManager (新增)                                   │  │
│  │  - 同步历史存储                                          │  │
│  │  - SQLite 持久化                                         │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                       │
                       │ 依赖倒置
                       ▼
┌──────────────────────────────────────────────────────────────┐
│              FfiBleHardware (平台实现)                        │
│  - start_scan() / stop_scan()                                │
│  - connect() / disconnect()                                  │
│  - write_data()                                              │
│  - 回调: on_device_discovered / on_data_received             │
└──────────────────────────────────────────────────────────────┘
```

## 2. 详细设计

### 2.1 新增 FFI 接口：FfiBleHardware

```udl
// 平台实现的 BLE 硬件抽象接口
callback interface FfiBleHardware {
    // === 扫描控制 ===
    // 开始扫描 NearClip 设备
    void start_scan();

    // 停止扫描
    void stop_scan();

    // === 连接管理 ===
    // 连接到指定设备 (通过 peripheral UUID)
    void connect(string peripheral_uuid);

    // 断开连接
    void disconnect(string peripheral_uuid);

    // === 数据传输 ===
    // 写入数据到设备 (已分块的单个 chunk)
    // 返回空字符串表示成功，否则返回错误信息
    string write_data(string peripheral_uuid, bytes data);

    // === 状态查询 ===
    // 获取设备的 MTU
    u32 get_mtu(string peripheral_uuid);

    // 检查是否已连接
    boolean is_connected(string peripheral_uuid);

    // === 广播控制 ===
    // 开始广播本机服务
    void start_advertising();

    // 停止广播
    void stop_advertising();
};

// Rust 回调平台的事件接口
interface FfiBleEvents {
    // === 扫描事件 ===
    // 发现设备
    void on_device_discovered(
        string peripheral_uuid,  // 平台层的设备标识
        string device_id,        // NearClip 设备 ID (从 characteristic 读取)
        string public_key_hash,  // 公钥哈希
        i32 rssi
    );

    // 设备丢失 (超时未再次发现)
    void on_device_lost(string peripheral_uuid);

    // === 连接事件 ===
    // 连接成功
    void on_connected(string peripheral_uuid);

    // 连接断开
    void on_disconnected(string peripheral_uuid, string reason);

    // 连接失败
    void on_connection_failed(string peripheral_uuid, string error);

    // === 数据事件 ===
    // 收到数据 (单个 chunk)
    void on_data_received(string peripheral_uuid, bytes data);

    // 写入完成确认
    void on_write_completed(string peripheral_uuid);

    // === 状态事件 ===
    // 蓝牙状态变化
    void on_bluetooth_state_changed(boolean powered_on);
};
```

### 2.2 新增 Rust 模块：BleController

```rust
// crates/nearclip-ble/src/controller.rs

/// BLE 控制器 - 管理所有 BLE 逻辑
///
/// 职责：
/// - 扫描控制和设备发现
/// - 连接生命周期管理
/// - 自动重连
/// - 健康检查
/// - 配对验证
pub struct BleController {
    /// 平台 BLE 硬件接口
    hardware: Arc<dyn FfiBleHardware>,

    /// 发现的设备: peripheral_uuid -> DiscoveredDevice
    discovered_devices: RwLock<HashMap<String, DiscoveredDevice>>,

    /// 已连接设备: peripheral_uuid -> ConnectedDevice
    connected_devices: RwLock<HashMap<String, ConnectedDevice>>,

    /// peripheral_uuid -> device_id 映射
    uuid_to_device_id: RwLock<HashMap<String, String>>,

    /// 重连状态
    reconnect_state: RwLock<HashMap<String, ReconnectState>>,

    /// 配置
    config: BleControllerConfig,

    /// 事件回调
    callback: Arc<dyn BleControllerCallback>,
}

pub struct BleControllerConfig {
    /// 扫描超时 (ms)
    pub scan_timeout_ms: u64,

    /// 设备丢失超时 (ms)
    pub device_lost_timeout_ms: u64,

    /// 自动重连
    pub auto_reconnect: bool,

    /// 最大重连次数
    pub max_reconnect_attempts: u32,

    /// 重连基础延迟 (ms)
    pub reconnect_base_delay_ms: u64,

    /// 健康检查间隔 (ms)
    pub health_check_interval_ms: u64,

    /// 连接超时 (ms)
    pub connection_timeout_ms: u64,
}

impl BleController {
    /// 开始扫描
    pub async fn start_scan(&self) -> Result<(), BleError>;

    /// 停止扫描
    pub async fn stop_scan(&self);

    /// 连接到设备 (通过 device_id)
    pub async fn connect(&self, device_id: &str) -> Result<(), BleError>;

    /// 断开设备
    pub async fn disconnect(&self, device_id: &str) -> Result<(), BleError>;

    /// 发送数据 (自动分块)
    pub async fn send_data(&self, device_id: &str, data: &[u8]) -> Result<(), BleError>;

    /// 获取已发现设备列表
    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice>;

    /// 获取已连接设备列表
    pub async fn get_connected_devices(&self) -> Vec<String>;

    // === 平台回调处理 ===

    /// 处理设备发现事件
    pub async fn handle_device_discovered(
        &self,
        peripheral_uuid: &str,
        device_id: &str,
        public_key_hash: &str,
        rssi: i32,
    );

    /// 处理连接成功事件
    pub async fn handle_connected(&self, peripheral_uuid: &str);

    /// 处理断开连接事件
    pub async fn handle_disconnected(&self, peripheral_uuid: &str, reason: &str);

    /// 处理数据接收事件
    pub async fn handle_data_received(&self, peripheral_uuid: &str, data: &[u8]);

    // === 内部方法 ===

    /// 启动健康检查任务
    async fn start_health_check(&self);

    /// 启动设备丢失检测任务
    async fn start_device_lost_detection(&self);

    /// 执行重连
    async fn attempt_reconnect(&self, device_id: &str);

    /// 验证设备配对状态
    async fn verify_pairing(&self, device_id: &str, public_key_hash: &str) -> bool;
}
```

### 2.3 新增 Rust 模块：HistoryManager

```rust
// crates/nearclip-core/src/history.rs

/// 同步历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistoryEntry {
    pub id: String,
    pub timestamp: i64,
    pub direction: SyncDirection,
    pub device_id: String,
    pub device_name: String,
    pub content_type: ContentType,
    pub content_preview: String,  // 前 100 字符
    pub content_size: u64,
    pub channel: Channel,  // WiFi 或 BLE
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDirection {
    Sent,
    Received,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    File,
    Unknown,
}

/// 历史管理器
pub struct HistoryManager {
    /// SQLite 连接
    db: SqliteConnection,

    /// 最大记录数
    max_entries: usize,
}

impl HistoryManager {
    /// 创建历史管理器
    pub fn new(db_path: &Path, max_entries: usize) -> Result<Self, HistoryError>;

    /// 添加历史记录
    pub async fn add_entry(&self, entry: SyncHistoryEntry) -> Result<(), HistoryError>;

    /// 获取历史记录 (分页)
    pub async fn get_entries(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<SyncHistoryEntry>, HistoryError>;

    /// 获取最近 N 条记录
    pub async fn get_recent(&self, count: usize) -> Result<Vec<SyncHistoryEntry>, HistoryError>;

    /// 按设备过滤
    pub async fn get_by_device(
        &self,
        device_id: &str,
        limit: usize,
    ) -> Result<Vec<SyncHistoryEntry>, HistoryError>;

    /// 清空历史
    pub async fn clear(&self) -> Result<(), HistoryError>;

    /// 删除旧记录 (保留最近 N 条)
    pub async fn prune(&self) -> Result<usize, HistoryError>;
}
```

### 2.4 简化后的 FFI 接口

```udl
// 简化后的 FfiNearClipManager
interface FfiNearClipManager {
    [Throws=NearClipError]
    constructor(
        FfiNearClipConfig config,
        FfiNearClipCallback callback,
        FfiBleHardware? ble_hardware  // 可选，平台提供 BLE 硬件接口
    );

    // === 生命周期 ===
    [Throws=NearClipError]
    void start();
    void stop();
    boolean is_running();

    // === 剪贴板同步 ===
    [Throws=NearClipError]
    void sync_clipboard(bytes content);

    // === 设备管理 ===
    sequence<FfiDeviceInfo> get_paired_devices();
    sequence<FfiDeviceInfo> get_connected_devices();
    sequence<FfiDeviceInfo> get_discovered_devices();  // 新增

    [Throws=NearClipError]
    void connect_device(string device_id);

    [Throws=NearClipError]
    void disconnect_device(string device_id);

    [Throws=NearClipError]
    void pair_device(string device_id);  // 新增：配对发现的设备

    [Throws=NearClipError]
    void unpair_device(string device_id);

    // === 扫描控制 (Rust 控制，平台执行) ===
    void start_discovery();   // 开始发现设备 (WiFi mDNS + BLE 扫描)
    void stop_discovery();    // 停止发现

    // === 历史记录 ===
    sequence<FfiSyncHistoryEntry> get_sync_history(u32 limit);
    void clear_sync_history();

    // === 配对码 ===
    string generate_pairing_code();
    [Throws=NearClipError]
    void add_device_from_code(string code);

    // === BLE 事件处理 (平台回调) ===
    void on_ble_device_discovered(
        string peripheral_uuid,
        string device_id,
        string public_key_hash,
        i32 rssi
    );
    void on_ble_connected(string peripheral_uuid);
    void on_ble_disconnected(string peripheral_uuid, string reason);
    void on_ble_data_received(string peripheral_uuid, bytes data);
    void on_bluetooth_state_changed(boolean powered_on);
};

// 简化后的回调接口
callback interface FfiNearClipCallback {
    // === 设备事件 ===
    void on_device_discovered(FfiDeviceInfo device);  // 新增
    void on_device_connected(FfiDeviceInfo device);
    void on_device_disconnected(string device_id);
    void on_device_paired(FfiDeviceInfo device);      // 新增
    void on_device_unpaired(string device_id);

    // === 同步事件 ===
    void on_clipboard_received(bytes content, string from_device);
    void on_sync_completed(string device_id);         // 新增
    void on_sync_error(string error_message);

    // === BLE 控制命令 (Rust -> 平台) ===
    void on_ble_start_scan();
    void on_ble_stop_scan();
    void on_ble_connect(string peripheral_uuid);
    void on_ble_disconnect(string peripheral_uuid);
    void on_ble_write_data(string peripheral_uuid, bytes data);
    void on_ble_start_advertising();
    void on_ble_stop_advertising();
};
```

## 3. 实施计划

### 第一阶段：BLE 控制权转移 (核心)

**目标**：将 BLE 扫描、连接、重连逻辑移到 Rust 层

#### 步骤 1.1：定义 BLE 硬件抽象接口
- [ ] 更新 `nearclip.udl` 添加 `FfiBleHardware` 接口
- [ ] 更新 `nearclip.udl` 添加 BLE 事件回调方法
- [ ] 重新生成 FFI 绑定

#### 步骤 1.2：实现 BleController
- [ ] 创建 `crates/nearclip-ble/src/controller.rs`
- [ ] 实现扫描控制逻辑
- [ ] 实现连接管理逻辑
- [ ] 实现自动重连（指数退避）
- [ ] 实现健康检查
- [ ] 实现设备丢失检测

#### 步骤 1.3：集成到 NearClipManager
- [ ] 在 `FfiNearClipManager` 中集成 `BleController`
- [ ] 实现 BLE 事件处理方法
- [ ] 连接 `BleController` 和 `TransportManager`

#### 步骤 1.4：简化平台层 BleManager
- [ ] macOS: 重写 `BleManager.swift` (~200 行)
  - 只保留 CoreBluetooth API 调用
  - 移除所有业务逻辑
- [ ] Android: 重写 `BleManager.kt` (~200 行)
  - 只保留 Android BLE API 调用
  - 移除所有业务逻辑

### 第二阶段：统一历史存储

**目标**：在 Rust 层实现同步历史存储

#### 步骤 2.1：实现 HistoryManager
- [ ] 添加 `rusqlite` 依赖
- [ ] 创建 `crates/nearclip-core/src/history.rs`
- [ ] 实现 SQLite 存储
- [ ] 实现历史记录 CRUD

#### 步骤 2.2：集成到 FFI
- [ ] 添加 `FfiSyncHistoryEntry` 类型
- [ ] 添加 `get_sync_history()` 方法
- [ ] 添加 `clear_sync_history()` 方法

#### 步骤 2.3：移除平台层历史存储
- [ ] 删除 `macos/.../SyncHistoryManager.swift`
- [ ] 删除 `android/.../SyncHistoryRepository.kt`
- [ ] 更新 UI 层调用 FFI 接口

### 第三阶段：统一设备存储

**目标**：只使用 Rust 层的设备存储

#### 步骤 3.1：增强 Rust DeviceStore
- [ ] 确保 `FileDeviceStore` 功能完整
- [ ] 添加设备元数据存储

#### 步骤 3.2：移除平台层设备存储
- [ ] macOS: 移除 UserDefaults 设备存储
- [ ] Android: 移除 SecureStorage 设备存储
- [ ] 更新平台层只通过 FFI 访问设备列表

### 第四阶段：清理和优化

#### 步骤 4.1：移除废弃代码
- [ ] 清理平台层不再使用的代码
- [ ] 移除重复的数据结构定义

#### 步骤 4.2：测试
- [ ] 单元测试 BleController
- [ ] 单元测试 HistoryManager
- [ ] 集成测试 BLE 流程
- [ ] 端到端测试

#### 步骤 4.3：文档
- [ ] 更新架构文档
- [ ] 更新 API 文档
- [ ] 更新平台集成指南

## 4. 数据流设计

### 4.1 BLE 扫描流程 (重构后)

```
用户点击"扫描设备"
    │
    ▼
平台 UI 调用 FFI
    │ manager.start_discovery()
    ▼
Rust: NearClipManager
    │
    ├─► mDNS 发现 (WiFi)
    │
    └─► BleController.start_scan()
            │
            │ 回调平台
            ▼
        callback.on_ble_start_scan()
            │
            ▼
        平台: BleManager.startScan()
            │ CoreBluetooth / Android BLE
            ▼
        发现设备，读取 characteristics
            │
            │ 回调 Rust
            ▼
        FFI: on_ble_device_discovered(uuid, device_id, pubkey_hash, rssi)
            │
            ▼
        Rust: BleController.handle_device_discovered()
            │
            ├─► 验证 pubkey_hash (是否已配对)
            │
            └─► 回调上层
                    │
                    ▼
                callback.on_device_discovered(device_info)
                    │
                    ▼
                平台 UI 显示发现的设备
```

### 4.2 BLE 连接流程 (重构后)

```
用户点击"连接设备"
    │
    ▼
平台 UI 调用 FFI
    │ manager.connect_device(device_id)
    ▼
Rust: NearClipManager
    │
    └─► BleController.connect(device_id)
            │
            │ 查找 peripheral_uuid
            │ 回调平台
            ▼
        callback.on_ble_connect(peripheral_uuid)
            │
            ▼
        平台: BleManager.connect(uuid)
            │ CoreBluetooth / Android BLE
            ▼
        连接成功
            │
            │ 回调 Rust
            ▼
        FFI: on_ble_connected(peripheral_uuid)
            │
            ▼
        Rust: BleController.handle_connected()
            │
            ├─► 创建 BleTransport
            │
            ├─► 注册到 TransportManager
            │
            └─► 回调上层
                    │
                    ▼
                callback.on_device_connected(device_info)
```

### 4.3 BLE 数据发送流程 (重构后)

```
Rust: sync_clipboard(content)
    │
    ▼
TransportManager.send_to_device(device_id, message)
    │
    │ 选择最佳通道 (WiFi 优先)
    │ 假设选择 BLE
    ▼
BleTransport.send(message)
    │
    ├─► 序列化消息
    │
    ├─► Chunker.chunk() 分块
    │
    └─► 对每个 chunk:
            │
            │ 调用 BleSender
            ▼
        sender.send_ble_data(device_id, chunk)
            │
            │ BleSender 实现调用回调
            ▼
        callback.on_ble_write_data(peripheral_uuid, chunk)
            │
            ▼
        平台: BleManager.writeData(uuid, chunk)
            │
            ▼
        CoreBluetooth / Android BLE 写入
```

## 5. 平台层简化示例

### 5.1 macOS BleManager (简化后 ~200 行)

```swift
/// 简化后的 BLE 管理器 - 只负责硬件操作
final class BleManager: NSObject {
    private var centralManager: CBCentralManager!
    private var peripheralManager: CBPeripheralManager!
    private var peripherals: [UUID: CBPeripheral] = [:]

    weak var ffiManager: FfiNearClipManager?

    override init() {
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: nil)
        peripheralManager = CBPeripheralManager(delegate: self, queue: nil)
    }

    // MARK: - 扫描

    func startScan() {
        centralManager.scanForPeripherals(withServices: [BleUUID.service])
    }

    func stopScan() {
        centralManager.stopScan()
    }

    // MARK: - 连接

    func connect(_ uuid: UUID) {
        guard let peripheral = peripherals[uuid] else { return }
        centralManager.connect(peripheral)
    }

    func disconnect(_ uuid: UUID) {
        guard let peripheral = peripherals[uuid] else { return }
        centralManager.cancelPeripheralConnection(peripheral)
    }

    // MARK: - 数据传输

    func writeData(_ uuid: UUID, data: Data) {
        guard let peripheral = peripherals[uuid],
              let characteristic = findDataCharacteristic(peripheral) else { return }
        peripheral.writeValue(data, for: characteristic, type: .withResponse)
    }
}

// MARK: - CBCentralManagerDelegate

extension BleManager: CBCentralManagerDelegate {
    func centralManager(_ central: CBCentralManager,
                       didDiscover peripheral: CBPeripheral,
                       advertisementData: [String: Any],
                       rssi: NSNumber) {
        peripherals[peripheral.identifier] = peripheral
        peripheral.delegate = self
        // 连接以读取 characteristics
        central.connect(peripheral)
    }

    func centralManager(_ central: CBCentralManager,
                       didConnect peripheral: CBPeripheral) {
        peripheral.discoverServices([BleUUID.service])
    }

    func centralManager(_ central: CBCentralManager,
                       didDisconnectPeripheral peripheral: CBPeripheral,
                       error: Error?) {
        ffiManager?.onBleDisconnected(
            peripheralUuid: peripheral.identifier.uuidString,
            reason: error?.localizedDescription ?? "unknown"
        )
    }
}

// MARK: - CBPeripheralDelegate

extension BleManager: CBPeripheralDelegate {
    func peripheral(_ peripheral: CBPeripheral,
                   didDiscoverCharacteristicsFor service: CBService,
                   error: Error?) {
        // 读取 device_id 和 public_key_hash
        // 然后回调 Rust
        ffiManager?.onBleDeviceDiscovered(
            peripheralUuid: peripheral.identifier.uuidString,
            deviceId: deviceId,
            publicKeyHash: publicKeyHash,
            rssi: lastRssi
        )
    }

    func peripheral(_ peripheral: CBPeripheral,
                   didUpdateValueFor characteristic: CBCharacteristic,
                   error: Error?) {
        guard let data = characteristic.value else { return }
        ffiManager?.onBleDataReceived(
            peripheralUuid: peripheral.identifier.uuidString,
            data: data
        )
    }
}
```

## 6. 风险和缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| BLE 平台差异 | 某些行为在 iOS/Android 不一致 | 在 Rust 层抽象，平台层只做最小实现 |
| 性能影响 | FFI 调用开销 | 批量操作，减少跨边界调用 |
| 调试困难 | 跨层问题难以定位 | 完善日志，添加 tracing |
| 迁移风险 | 可能引入新 bug | 分阶段迁移，保持旧代码可回退 |

## 7. 成功指标

- [ ] 平台层 BLE 代码从 2200+ 行减少到 ~400 行
- [ ] 所有 BLE 逻辑测试覆盖率 > 80%
- [ ] 历史存储统一，数据格式一致
- [ ] 设备存储统一，无状态同步问题
- [ ] 端到端测试通过
