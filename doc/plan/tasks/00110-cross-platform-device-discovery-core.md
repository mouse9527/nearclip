# Task 00110: 跨平台设备发现核心逻辑

## 任务描述
实现跨平台的设备发现核心逻辑，包括设备抽象、发现管理、事件处理等共享组件。

## 验收标准

### 核心抽象层
- [ ] 实现统一的设备抽象接口（UnifiedDevice）
- [ ] 创建设备发现特质（DeviceDiscovery trait）
- [ ] 实现发现事件系统（DiscoveryEvent enum）
- [ ] 创建发现配置管理（DiscoveryConfig）

### 传输层抽象
- [ ] 实现传输层特质（Transport trait）
- [ ] 创建传输管理器（TransportManager）
- [ ] 实现传输质量评估（TransportQuality）
- [ ] 支持传输方式动态切换

### 设备管理
- [ ] 实现设备缓存管理（DeviceCache）
- [ ] 创建设备状态管理（DeviceStateManager）
- [ ] 实现设备生命周期管理（DeviceLifecycle）
- [ ] 支持设备黑名单机制

### 事件系统
- [ ] 实现事件总线（EventBus）
- [ ] 创建事件处理器（EventHandler）
- [ ] 支持异步事件流处理
- [ ] 实现事件持久化和恢复

## 技术实现

### 核心抽象
```rust
// 统一设备抽象
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedDevice {
    pub id: DeviceId,
    pub name: String,
    pub capabilities: DeviceCapabilities,
    pub transport_info: Vec<TransportInfo>,
    pub last_seen: SystemTime,
    pub status: DeviceStatus,
}

// 设备发现特质
#[async_trait]
pub trait DeviceDiscovery: Send + Sync {
    async fn start_discovery(&mut self) -> Result<(), DiscoveryError>;
    async fn stop_discovery(&mut self) -> Result<(), DiscoveryError>;
    fn is_discovering(&self) -> bool;
    fn get_discovered_devices(&self) -> Vec<UnifiedDevice>;
    fn subscribe_to_events(&self) -> BroadcastReceiver<DiscoveryEvent>;
}

// 发现配置
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub scan_interval: Duration,
    pub scan_duration: Duration,
    pub enabled_transports: Vec<TransportType>,
    pub battery_optimization: bool,
    pub background_mode: bool,
}

// 传输层特质
#[async_trait]
pub trait Transport: Send + Sync {
    type Address: Clone + Send + Sync + std::fmt::Debug;
    
    async fn connect(&mut self, address: Self::Address) -> Result<(), TransportError>;
    async fn disconnect(&mut self) -> Result<(), TransportError>;
    async fn send(&mut self, data: &[u8]) -> Result<(), TransportError>;
    async fn receive(&mut self) -> Result<Vec<u8>, TransportError>;
    fn get_quality_score(&self) -> f32;
    fn is_connected(&self) -> bool;
    fn get_address(&self) -> Option<Self::Address>;
}
```

### 设备管理
```rust
// 设备缓存
pub struct DeviceCache {
    devices: HashMap<DeviceId, UnifiedDevice>,
    config: CacheConfig,
}

impl DeviceCache {
    pub fn new(config: CacheConfig) -> Self;
    pub fn add_device(&mut self, device: UnifiedDevice);
    pub fn remove_device(&mut self, device_id: &DeviceId);
    pub fn get_device(&self, device_id: &DeviceId) -> Option<&UnifiedDevice>;
    pub fn get_all_devices(&self) -> Vec<&UnifiedDevice>;
    pub fn update_device_status(&mut self, device_id: &DeviceId, status: DeviceStatus);
    pub fn cleanup_expired_devices(&mut self);
}

// 设备状态管理
pub struct DeviceStateManager {
    devices: HashMap<DeviceId, DeviceState>,
    event_bus: EventBus<DeviceEvent>,
}

impl DeviceStateManager {
    pub fn new(event_bus: EventBus<DeviceEvent>) -> Self;
    pub fn update_device_state(&mut self, device_id: DeviceId, new_state: DeviceState);
    pub fn get_device_state(&self, device_id: &DeviceId) -> Option<&DeviceState>;
    pub fn get_devices_by_status(&self, status: DeviceStatus) -> Vec<&DeviceId>;
}

// 设备生命周期管理
pub struct DeviceLifecycleManager {
    discovery_manager: Box<dyn DeviceDiscovery>,
    connection_manager: ConnectionManager,
    config: LifecycleConfig,
}

impl DeviceLifecycleManager {
    pub async fn start(&mut self) -> Result<(), DiscoveryError>;
    pub async fn stop(&mut self) -> Result<(), DiscoveryError>;
    pub async fn connect_to_device(&mut self, device_id: DeviceId) -> Result<(), ConnectionError>;
    pub async fn disconnect_from_device(&mut self, device_id: DeviceId) -> Result<(), ConnectionError>;
}
```

### 事件系统
```rust
// 事件总线
pub struct EventBus<T: Clone + Send + Sync + 'static> {
    senders: Vec<AsyncSender<T>>,
    receivers: Vec<AsyncReceiver<T>>,
}

impl<T: Clone + Send + Sync + 'static> EventBus<T> {
    pub fn new() -> Self;
    pub fn subscribe(&mut self) -> BroadcastReceiver<T>;
    pub async fn publish(&self, event: T);
    pub fn broadcast_count(&self) -> usize;
}

// 事件处理器
pub trait EventHandler<T: Clone + Send + Sync>: Send + Sync {
    async fn handle_event(&mut self, event: T) -> Result<(), EventError>;
    fn can_handle(&self, event_type: &str) -> bool;
}

// 事件管理器
pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler<DiscoveryEvent>>>,
    event_bus: EventBus<DiscoveryEvent>,
}

impl EventManager {
    pub fn new() -> Self;
    pub fn register_handler(&mut self, handler: Box<dyn EventHandler<DiscoveryEvent>>);
    pub async fn process_events(&mut self) -> Result<(), EventError>;
}
```

### 传输管理
```rust
// 传输管理器
pub struct TransportManager {
    transports: HashMap<TransportType, Box<dyn Transport>>,
    active_transport: Option<TransportType>,
    config: TransportConfig,
}

impl TransportManager {
    pub fn new(config: TransportConfig) -> Self;
    pub fn register_transport(&mut self, transport_type: TransportType, transport: Box<dyn Transport>);
    pub async fn select_best_transport(&mut self) -> Result<TransportType, TransportError>;
    pub async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    pub async fn receive_data(&mut self) -> Result<Vec<u8>, TransportError>;
    pub fn get_active_transport(&self) -> Option<&dyn Transport>;
    pub fn get_transport_quality(&self, transport_type: TransportType) -> Option<f32>;
}

// 传输质量评估
pub struct TransportQualityEvaluator {
    metrics: HashMap<TransportType, TransportMetrics>,
}

impl TransportQualityEvaluator {
    pub fn evaluate_quality(&self, transport_type: TransportType) -> f32;
    pub fn update_metrics(&mut self, transport_type: TransportType, metrics: TransportMetrics);
    pub fn get_best_transport(&self) -> Option<TransportType>;
}
```

## 测试要求

### 单元测试
- [ ] 设备抽象层单元测试
- [ ] 设备发现特质单元测试
- [ ] 传输层抽象单元测试
- [ ] 设备管理单元测试
- [ ] 事件系统单元测试

### 集成测试
- [ ] 设备发现流程集成测试
- [ ] 传输管理集成测试
- [ ] 事件处理集成测试

### 性能测试
- [ ] 设备缓存性能测试
- [ ] 事件系统吞吐量测试
- [ ] 传输切换性能测试

## 性能要求

### 内存使用
- 核心逻辑内存占用 < 10MB
- 设备缓存内存占用 < 5MB
- 事件系统内存占用 < 3MB

### CPU使用
- 空闲状态CPU使用 < 1%
- 设备发现状态CPU使用 < 3%
- 事件处理CPU使用 < 2%

### 响应时间
- 设备发现事件响应时间 < 100ms
- 传输切换时间 < 500ms
- 缓存查询时间 < 10ms

## 依赖项

### Rust依赖
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
tracing = "0.1"
tracing-subscriber = "0.3"
crossbeam = "0.8"
parking_lot = "0.12"
once_cell = "1.0"

# 网络
tokio-tungstenite = "0.20"
mdns-sd = "0.10"

# 蓝牙
btleplug = "0.11"

# 加密
aes-gcm = "0.10"
rand = "0.8"
sha2 = "0.10"

# 时间
chrono = { version = "0.4", features = ["serde"] }

# 配置
config = "0.13"
clap = { version = "4.0", features = ["derive"] }
```

### 开发依赖
```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
criterion = "0.5"
proptest = "1.0"
```

## 相关文件

- [任务 00101: 设备类型枚举](../tasks/00101-device-type-enum.md)
- [任务 00102: 设备状态枚举](../tasks/00102-device-status-enum.md)
- [任务 00103: 设备能力枚举](../tasks/00103-device-capability-enum.md)
- [任务 00104: 设备基础结构](../tasks/00104-device-basic-structure.md)
- [任务 00105: 发现事件枚举](../tasks/00105-discovery-event-enum.md)
- [任务 00106: 设备发现特质](../tasks/00106-device-discovery-trait.md)
- [任务 00107: 发现配置结构](../tasks/00107-discovery-config-structure.md)
- [任务 00108a: Android BLE 设备发现核心](../tasks/00108a-android-ble-discovery-core.md)
- [任务 00108b: Android WiFi 设备发现核心](../tasks/00108b-android-wifi-discovery-core.md)
- [任务 00108c: Android 权限管理](../tasks/00108c-android-permission-management.md)
- [任务 00108d: Android 电池优化](../tasks/00108d-android-battery-optimization.md)
- [任务 00108e: Android 发现UI组件](../tasks/00108e-android-discovery-ui.md)
- [任务 00108f: Android 后台服务](../tasks/00108f-android-background-service.md)
- [任务 00109: macOS设备发现实现](../tasks/00109-macos-device-discovery.md)