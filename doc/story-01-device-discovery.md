# Story 1: 设备发现与连接

## INVEST 分析

- **Independent**: 可以独立开发，不依赖其他功能
- **Negotiable**: 发现方式可以调整 (mDNS/BLE/混合协议)
- **Valuable**: 用户能找到并连接其他设备，无论网络环境
- **Estimable**: 工作量可估算
- **Small**: 核心功能适合单个sprint完成
- **Testable**: 可以通过实际设备测试

## 用户场景

作为用户，我希望能够在各种网络环境下发现其他运行本应用的设备，这样我就可以选择要连接的设备，无需关心底层传输方式。

## 验收标准

- [ ] 应用启动后自动开始扫描附近设备（WiFi + BLE）
- [ ] 显示发现的设备列表，统一展示不同协议发现的设备
- [ ] 设备列表包含设备名称、连接方式和信号强度
- [ ] 支持手动刷新设备列表
- [ ] 能够连接到选中的设备，自动选择最佳传输方式
- [ ] 连接状态实时显示，包括传输方式信息
- [ ] 支持同时连接多个设备，使用不同传输方式
- [ ] 网络环境变化时自动切换传输方式
- [ ] 在无WiFi环境下使用BLE发现和连接

## 技术实现

### 核心架构
- **应用层**: 统一的设备管理接口
- **传输层**: 抽象传输接口，支持WiFi和BLE
- **发现层**: 混合发现机制（mDNS + BLE）
- **上下文感知**: 智能选择最佳传输方式

### 数据结构
```rust
struct Device {
    id: String,
    name: String,
    status: DeviceStatus,
    capabilities: DeviceCapabilities,
    discovery_methods: Vec<DiscoveryMethod>,
    // 应用层不关心具体传输方式
}

struct DeviceManager {
    transport_manager: TransportManager,
    device_registry: DeviceRegistry,
}

trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<(), TransportError>;
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    fn is_connected(&self) -> bool;
    fn get_quality_score(&self) -> f32;
}
```

## 相关任务

### 基础任务
- [task-0101.md](./task-0101.md) - 实现mDNS设备发现
- [task-0102.md](./task-0102.md) - 实现设备信息广播
- [task-0103.md](./task-0103.md) - 实现设备列表管理

### 传输层任务
- [task-0106.md](./task-0106.md) - 实现传输层抽象接口
- [task-0107.md](./task-0107.md) - 实现WiFi传输适配器
- [task-0108.md](./task-0108.md) - 实现BLE传输适配器

### 混合传输任务
- [task-0109.md](./task-0109.md) - 实现智能传输选择器
- [task-0110.md](./task-0110.md) - 实现传输自动切换机制
- [task-0111.md](./task-0111.md) - 实现BLE设备发现
- [task-0112.md](./task-0112.md) - 实现协议桥接机制

### 状态管理
- [task-0105.md](./task-0105.md) - 实现设备状态追踪
- [task-0113.md](./task-0113.md) - 实现上下文感知监控