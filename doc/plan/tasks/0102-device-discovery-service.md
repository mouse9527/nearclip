# Task 0102: 实现设备发现服务 (TDD版本)

## 任务描述

按照TDD原则实现设备发现服务，支持mDNS和BLE两种发现方式，为应用层提供统一的设备发现接口。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_discovery_tests.rs
#[cfg(test)]
mod device_discovery_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_device_discovery_starts_and_stops() {
        // RED: 测试设备发现服务的启动和停止
        let discovery = DeviceDiscoveryService::new();
        
        // 初始状态应该是停止的
        assert!(!discovery.is_running());
        
        // 启动发现应该成功
        assert!(discovery.start_discovery().await.is_ok());
        assert!(discovery.is_running());
        
        // 停止发现应该成功
        assert!(discovery.stop_discovery().await.is_ok());
        assert!(!discovery.is_running());
    }
    
    #[tokio::test]
    async fn test_device_discovery_emits_events() {
        // RED: 测试设备发现事件通知
        let discovery = DeviceDiscoveryService::new();
        let mut event_receiver = discovery.subscribe_to_events();
        
        // 启动发现
        discovery.start_discovery().await.unwrap();
        
        // 模拟发现设备
        let test_device = Device::new("test-device", "Test Device");
        discovery.simulate_device_discovery(test_device.clone()).await;
        
        // 应该收到设备发现事件
        let event = timeout(Duration::from_secs(1), event_receiver.recv()).await.unwrap().unwrap();
        match event {
            DiscoveryEvent::DeviceDiscovered(device) => {
                assert_eq!(device.id(), test_device.id());
            }
            _ => panic!("Expected DeviceDiscovered event"),
        }
        
        discovery.stop_discovery().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_hybrid_discovery_combines_results() {
        // RED: 测试混合发现模式
        let mut discovery = DeviceDiscoveryService::new();
        
        // 配置同时使用mDNS和BLE发现
        discovery.enable_mdns_discovery(true);
        discovery.enable_ble_discovery(true);
        
        discovery.start_discovery().await.unwrap();
        
        // 模拟mDNS发现的设备
        let mdns_device = Device::new("mdns-device", "mDNS Device");
        discovery.simulate_mdns_discovery(mdns_device.clone()).await;
        
        // 模拟BLE发现的设备
        let ble_device = Device::new("ble-device", "BLE Device");
        discovery.simulate_ble_discovery(ble_device.clone()).await;
        
        // 获取发现的设备列表
        let discovered_devices = discovery.get_discovered_devices().await;
        assert_eq!(discovered_devices.len(), 2);
        
        discovery.stop_discovery().await.unwrap();
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum DiscoveryEvent {
    DeviceDiscovered(Device),
    DeviceLost(String),
    DiscoveryStarted,
    DiscoveryStopped,
}

pub struct DeviceDiscoveryService {
    is_running: bool,
    mdns_enabled: bool,
    ble_enabled: bool,
    discovered_devices: HashMap<String, Device>,
    event_sender: broadcast::Sender<DiscoveryEvent>,
}

impl DeviceDiscoveryService {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            is_running: false,
            mdns_enabled: false,
            ble_enabled: false,
            discovered_devices: HashMap::new(),
            event_sender,
        }
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn enable_mdns_discovery(&mut self, enabled: bool) {
        self.mdns_enabled = enabled;
    }
    
    pub fn enable_ble_discovery(&mut self, enabled: bool) {
        self.ble_enabled = enabled;
    }
    
    pub async fn start_discovery(&mut self) -> Result<(), DiscoveryError> {
        self.is_running = true;
        let _ = self.event_sender.send(DiscoveryEvent::DiscoveryStarted);
        Ok(())
    }
    
    pub async fn stop_discovery(&mut self) -> Result<(), DiscoveryError> {
        self.is_running = false;
        let _ = self.event_sender.send(DiscoveryEvent::DiscoveryStopped);
        Ok(())
    }
    
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<DiscoveryEvent> {
        self.event_sender.subscribe()
    }
    
    pub async fn get_discovered_devices(&self) -> Vec<Device> {
        self.discovered_devices.values().cloned().collect()
    }
    
    // 测试辅助方法
    pub async fn simulate_device_discovery(&mut self, device: Device) {
        self.discovered_devices.insert(device.id().to_string(), device.clone());
        let _ = self.event_sender.send(DiscoveryEvent::DeviceDiscovered(device));
    }
    
    pub async fn simulate_mdns_discovery(&mut self, device: Device) {
        self.simulate_device_discovery(device).await;
    }
    
    pub async fn simulate_ble_discovery(&mut self, device: Device) {
        self.simulate_device_discovery(device).await;
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
impl DeviceDiscoveryService {
    // 重构后的代码，保持测试绿色
    pub async fn start_discovery(&mut self) -> Result<(), DiscoveryError> {
        if self.is_running {
            return Err(DiscoveryError::AlreadyRunning);
        }
        
        if !self.mdns_enabled && !self.ble_enabled {
            return Err(DiscoveryError::NoDiscoveryMethodEnabled);
        }
        
        self.is_running = true;
        
        // 启动实际的发现任务
        if self.mdns_enabled {
            self.start_mdns_discovery().await?;
        }
        
        if self.ble_enabled {
            self.start_ble_discovery().await?;
        }
        
        let _ = self.event_sender.send(DiscoveryEvent::DiscoveryStarted);
        Ok(())
    }
    
    async fn start_mdns_discovery(&self) -> Result<(), DiscoveryError> {
        // 启动mDNS发现任务的逻辑
        Ok(())
    }
    
    async fn start_ble_discovery(&self) -> Result<(), DiscoveryError> {
        // 启动BLE发现任务的逻辑
        Ok(())
    }
    
    pub fn add_discovered_device(&mut self, device: Device) {
        let device_id = device.id().to_string();
        
        if self.discovered_devices.contains_key(&device_id) {
            // 设备已存在，更新信息
            self.discovered_devices.insert(device_id, device);
        } else {
            // 新设备
            self.discovered_devices.insert(device_id, device.clone());
            let _ = self.event_sender.send(DiscoveryEvent::DeviceDiscovered(device));
        }
    }
}
```

### 必须编写的测试类型

#### 1. 单元测试 (Unit Tests)
- 发现服务生命周期测试
- 事件系统测试
- 设备管理测试
- 配置管理测试

#### 2. 集成测试 (Integration Tests)
- mDNS发现实现集成
- BLE发现实现集成
- 事件系统与外部组件集成

#### 3. 性能测试 (Performance Tests)
- 发现响应时间测试
- 内存使用测试
- 并发发现测试

### 测试覆盖率要求
- **单元测试覆盖率**: > 90%
- **集成测试覆盖率**: > 80%
- **关键路径测试**: 100%

## Clean Architecture要求

### 依赖倒置原则
设备发现服务作为Use Case，应该依赖domain层的接口：

```rust
// rust-core/application/use_cases/device_discovery.rs
pub struct DeviceDiscoveryService {
    mdns_discovery: Arc<dyn MdnsDiscovery>,
    ble_discovery: Arc<dyn BleDiscovery>,
    device_repository: Arc<dyn DeviceRepository>,
}
```

### 接口隔离原则
定义清晰的发现接口：

```rust
// rust-core/domain/interfaces/discovery.rs
#[async_trait]
pub trait DiscoveryMethod: Send + Sync {
    async fn start_discovery(&self) -> Result<(), DiscoveryError>;
    async fn stop_discovery(&self) -> Result<(), DiscoveryError>;
    fn subscribe_to_events(&self) -> broadcast::Receiver<DiscoveryEvent>;
}
```

## XP实践要求

### 1. 结对编程
- 所有发现服务代码必须两人结对完成
- 一人负责mDNS发现逻辑，另一人负责BLE发现逻辑
- 定期切换角色

### 2. 持续集成
- 每次提交都必须通过所有发现测试
- 自动化构建和测试
- 快速反馈循环

### 3. 代码审查
- 所有代码必须经过同行审查
- 检查TDD流程是否正确遵循
- 确保发现逻辑的健壮性

### 4. 重构勇气
- 定期重构以保持代码质量
- 消除发现逻辑中的重复代码
- 提高代码可读性

### 5. 简单设计
- 只实现当前需要的发现功能
- 避免过度设计发现策略
- 保持发现逻辑简单明了

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 90%
- [ ] 遵循Clean Architecture原则
- [ ] 通过代码审查
- [ ] 集成测试通过
- [ ] 性能测试通过
- [ ] 文档更新完整

## 依赖任务

- [Task 0101: 实现设备抽象层](0101-device-abstraction-layer.md)

## 后续任务

- [Task 0103: 实现传输层抽象](0103-transport-abstraction.md)
- [Task 0104: 实现mDNS发现](0104-mdns-discovery.md)
- [Task 0105: 实现BLE发现](0105-ble-discovery.md)