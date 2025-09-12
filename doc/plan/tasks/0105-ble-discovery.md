# Task 0105: 实现BLE发现 (TDD版本)

## 任务描述

按照TDD原则实现BLE设备发现功能，支持低功耗蓝牙协议，发现附近的NearClip设备。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/ble_discovery_tests.rs
#[cfg(test)]
mod ble_discovery_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ble_discovery_starts_and_scans() {
        // RED: 测试BLE发现服务的启动和扫描
        let mut discovery = BleDiscovery::new();
        
        // 初始状态应该是停止的
        assert!(!discovery.is_scanning());
        
        // 启动扫描应该成功
        assert!(discovery.start_scanning().await.is_ok());
        assert!(discovery.is_scanning());
        
        // 停止扫描应该成功
        assert!(discovery.stop_scanning().await.is_ok());
        assert!(!discovery.is_scanning());
    }
    
    #[tokio::test]
    async fn test_ble_device_discovery_by_service_uuid() {
        // RED: 测试BLE设备通过服务UUID发现
        let mut discovery = BleDiscovery::new();
        let mut event_receiver = discovery.subscribe_to_events();
        
        // 配置扫描NearClip服务UUID
        discovery.set_service_uuids(vec!["0000fef5-1212-efde-1523-785feabcd123"]);
        
        // 启动扫描
        discovery.start_scanning().await.unwrap();
        
        // 模拟发现BLE设备
        let test_device = BleDeviceInfo {
            address: "00:11:22:33:44:55".to_string(),
            name: Some("Test NearClip".to_string()),
            rssi: -65,
            service_uuids: vec!["0000fef5-1212-efde-1523-785feabcd123".to_string()],
            manufacturer_data: vec![],
        };
        
        discovery.simulate_device_discovery(test_device).await;
        
        // 应该收到设备发现事件
        let event = timeout(Duration::from_secs(1), event_receiver.recv()).await.unwrap().unwrap();
        match event {
            BleEvent::DeviceDiscovered(device) => {
                assert_eq!(device.address, "00:11:22:33:44:55");
                assert_eq!(device.name, Some("Test NearClip".to_string()));
            }
            _ => panic!("Expected DeviceDiscovered event"),
        }
        
        discovery.stop_scanning().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_ble_device_connection() {
        // RED: 测试BLE设备连接
        let mut discovery = BleDiscovery::new();
        
        // 模拟设备发现
        let device_address = "00:11:22:33:44:55";
        let test_device = BleDeviceInfo {
            address: device_address.to_string(),
            name: Some("Test Device".to_string()),
            rssi: -65,
            service_uuids: vec!["0000fef5-1212-efde-1523-785feabcd123".to_string()],
            manufacturer_data: vec![],
        };
        
        discovery.add_discovered_device(test_device).await;
        
        // 连接设备应该成功
        assert!(discovery.connect_device(device_address).await.is_ok());
        
        // 设备应该显示为已连接
        assert!(discovery.is_device_connected(device_address));
        
        // 断开连接应该成功
        assert!(discovery.disconnect_device(device_address).await.is_ok());
        assert!(!discovery.is_device_connected(device_address));
    }
    
    #[tokio::test]
    async fn test_ble_rssi_signal_strength_filtering() {
        // RED: 测试BLE信号强度过滤
        let mut discovery = BleDiscovery::new();
        
        // 设置RSSI过滤阈值
        discovery.set_rssi_threshold(-80);
        
        // 模拟信号强的设备
        let strong_device = BleDeviceInfo {
            address: "00:11:22:33:44:55".to_string(),
            name: Some("Strong Signal".to_string()),
            rssi: -65, // 强信号
            service_uuids: vec!["0000fef5-1212-efde-1523-785feabcd123".to_string()],
            manufacturer_data: vec![],
        };
        
        // 模拟信号弱的设备
        let weak_device = BleDeviceInfo {
            address: "66:77:88:99:aa:bb".to_string(),
            name: Some("Weak Signal".to_string()),
            rssi: -90, // 弱信号，应该被过滤
            service_uuids: vec!["0000fef5-1212-efde-1523-785feabcd123".to_string()],
            manufacturer_data: vec![],
        };
        
        discovery.start_scanning().await.unwrap();
        
        // 强信号设备应该被发现
        discovery.simulate_device_discovery(strong_device).await;
        let discovered_devices = discovery.get_discovered_devices().await;
        assert_eq!(discovered_devices.len(), 1);
        
        // 弱信号设备应该被过滤
        discovery.simulate_device_discovery(weak_device).await;
        let discovered_devices = discovery.get_discovered_devices().await;
        assert_eq!(discovered_devices.len(), 1); // 仍然只有1个设备
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct BleDeviceInfo {
    pub address: String,
    pub name: Option<String>,
    pub rssi: i32,
    pub service_uuids: Vec<String>,
    pub manufacturer_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum BleEvent {
    DeviceDiscovered(BleDeviceInfo),
    DeviceLost(String),
    DeviceConnected(String),
    DeviceDisconnected(String),
    ScanStarted,
    ScanStopped,
}

pub struct BleDiscovery {
    is_scanning: bool,
    service_uuids: Vec<String>,
    rssi_threshold: i32,
    discovered_devices: HashMap<String, BleDeviceInfo>,
    connected_devices: HashMap<String, BleConnection>,
    event_sender: broadcast::Sender<BleEvent>,
}

#[derive(Debug)]
struct BleConnection {
    device_address: String,
    connected_at: SystemTime,
}

impl BleDiscovery {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            is_scanning: false,
            service_uuids: vec![],
            rssi_threshold: -100, // 默认不过滤
            discovered_devices: HashMap::new(),
            connected_devices: HashMap::new(),
            event_sender,
        }
    }
    
    pub fn is_scanning(&self) -> bool {
        self.is_scanning
    }
    
    pub fn set_service_uuids(&mut self, uuids: Vec<&str>) {
        self.service_uuids = uuids.into_iter().map(|s| s.to_string()).collect();
    }
    
    pub fn set_rssi_threshold(&mut self, threshold: i32) {
        self.rssi_threshold = threshold;
    }
    
    pub async fn start_scanning(&mut self) -> Result<(), BleError> {
        if self.is_scanning {
            return Err(BleError::AlreadyScanning);
        }
        
        self.is_scanning = true;
        let _ = self.event_sender.send(BleEvent::ScanStarted);
        Ok(())
    }
    
    pub async fn stop_scanning(&mut self) -> Result<(), BleError> {
        if !self.is_scanning {
            return Err(BleError::NotScanning);
        }
        
        self.is_scanning = false;
        let _ = self.event_sender.send(BleEvent::ScanStopped);
        Ok(())
    }
    
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<BleEvent> {
        self.event_sender.subscribe()
    }
    
    pub async fn connect_device(&mut self, address: &str) -> Result<(), BleError> {
        if self.connected_devices.contains_key(address) {
            return Err(BleError::AlreadyConnected);
        }
        
        let connection = BleConnection {
            device_address: address.to_string(),
            connected_at: SystemTime::now(),
        };
        
        self.connected_devices.insert(address.to_string(), connection);
        let _ = self.event_sender.send(BleEvent::DeviceConnected(address.to_string()));
        Ok(())
    }
    
    pub async fn disconnect_device(&mut self, address: &str) -> Result<(), BleError> {
        if self.connected_devices.remove(address).is_none() {
            return Err(BleError::NotConnected);
        }
        
        let _ = self.event_sender.send(BleEvent::DeviceDisconnected(address.to_string()));
        Ok(())
    }
    
    pub fn is_device_connected(&self, address: &str) -> bool {
        self.connected_devices.contains_key(address)
    }
    
    pub async fn get_discovered_devices(&self) -> Vec<&BleDeviceInfo> {
        self.discovered_devices.values().collect()
    }
    
    // 测试辅助方法
    pub async fn simulate_device_discovery(&mut self, device: BleDeviceInfo) {
        if self.matches_filter_criteria(&device) {
            self.discovered_devices.insert(device.address.clone(), device.clone());
            let _ = self.event_sender.send(BleEvent::DeviceDiscovered(device));
        }
    }
    
    pub async fn add_discovered_device(&mut self, device: BleDeviceInfo) {
        self.simulate_device_discovery(device).await;
    }
    
    fn matches_filter_criteria(&self, device: &BleDeviceInfo) -> bool {
        // RSSI过滤
        if device.rssi < self.rssi_threshold {
            return false;
        }
        
        // 服务UUID过滤
        if !self.service_uuids.is_empty() {
            let has_matching_service = device.service_uuids.iter()
                .any(|uuid| self.service_uuids.contains(uuid));
            if !has_matching_service {
                return false;
            }
        }
        
        true
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct BleScanConfig {
    pub service_uuids: Vec<String>,
    pub rssi_threshold: i32,
    pub scan_duration: Duration,
    pub scan_mode: BleScanMode,
    pub callback_type: BleCallbackType,
}

#[derive(Debug, Clone)]
pub enum BleScanMode {
    LowPower,
    Balanced,
    LowLatency,
    Opportunistic,
}

#[derive(Debug, Clone)]
pub enum BleCallbackType {
    AllMatches,
    FirstMatch,
    LossOnly,
}

impl BleDiscovery {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            is_scanning: false,
            scan_config: BleScanConfig {
                service_uuids: vec![],
                rssi_threshold: -100,
                scan_duration: Duration::from_secs(10),
                scan_mode: BleScanMode::Balanced,
                callback_type: BleCallbackType::AllMatches,
            },
            discovered_devices: HashMap::new(),
            connected_devices: HashMap::new(),
            event_sender,
        }
    }
    
    pub fn with_scan_config(mut self, config: BleScanConfig) -> Self {
        self.scan_config = config;
        self
    }
    
    pub async fn start_scanning(&mut self) -> Result<(), BleError> {
        if self.is_scanning {
            return Err(BleError::AlreadyScanning);
        }
        
        // 验证扫描配置
        self.validate_scan_config()?;
        
        self.is_scanning = true;
        
        // 启动实际的BLE扫描任务
        self.start_ble_scan().await?;
        
        let _ = self.event_sender.send(BleEvent::ScanStarted);
        Ok(())
    }
    
    fn validate_scan_config(&self) -> Result<(), BleError> {
        // 验证服务UUID格式
        for uuid in &self.scan_config.service_uuids {
            if !Self::is_valid_uuid(uuid) {
                return Err(BleError::InvalidServiceUuid(uuid.clone()));
            }
        }
        
        // 验证扫描持续时间
        if self.scan_config.scan_duration.is_zero() {
            return Err(BleError::InvalidScanDuration);
        }
        
        Ok(())
    }
    
    fn is_valid_uuid(uuid: &str) -> bool {
        // 简单的UUID格式验证
        uuid.len() == 36 && uuid.matches(char::is_alphanumeric).count() == 32
    }
    
    async fn start_ble_scan(&self) -> Result<(), BleError> {
        // 这里会启动实际的BLE扫描
        // 目前是模拟实现
        Ok(())
    }
    
    pub fn update_device_rssi(&mut self, address: &str, new_rssi: i32) {
        if let Some(device) = self.discovered_devices.get_mut(address) {
            let old_rssi = device.rssi;
            device.rssi = new_rssi;
            
            // 如果RSSI变化超过阈值，发送事件
            if (old_rssi - new_rssi).abs() > 10 {
                let _ = self.event_sender.send(BleEvent::DeviceRssiChanged {
                    address: address.to_string(),
                    old_rssi,
                    new_rssi,
                });
            }
        }
    }
    
    pub fn get_device_quality_score(&self, address: &str) -> Option<f32> {
        self.discovered_devices.get(address).map(|device| {
            // 基于RSSI计算质量分数
            let rssi_score = if device.rssi > -50 { 1.0 }
                           else if device.rssi > -70 { 0.8 }
                           else if device.rssi > -90 { 0.6 }
                           else { 0.3 };
            
            // 基于服务数量加分
            let service_score = (device.service_uuids.len() as f32 * 0.1).min(0.3);
            
            (rssi_score + service_score).min(1.0)
        })
    }
}
```

### 必须编写的测试类型

#### 1. 单元测试 (Unit Tests)
- BLE扫描生命周期测试
- 设备发现和过滤测试
- 连接管理测试
- 信号强度测试
- 配置管理测试

#### 2. 集成测试 (Integration Tests)
- BLE库集成测试
- 权限处理测试
- 后台扫描测试
- 设备配对测试

#### 3. 硬件测试 (Hardware Tests)
- 实际BLE设备测试
- 多设备环境测试
- 信号干扰测试
- 电池消耗测试

### 测试覆盖率要求
- **单元测试覆盖率**: > 90%
- **集成测试覆盖率**: > 80%
- **硬件测试覆盖率**: > 70%

## Clean Architecture要求

### 依赖倒置原则
BLE发现应该依赖domain层的Discovery接口：

```rust
// rust-core/infrastructure/discovery/ble_discovery.rs
pub struct BleDiscovery {
    // BLE具体实现
}

#[async_trait]
impl DiscoveryMethod for BleDiscovery {
    async fn start_discovery(&self) -> Result<(), DiscoveryError> {
        // 实现BLE发现
    }
}
```

### 接口隔离原则
定义BLE特定的接口：

```rust
// rust-core/domain/interfaces/ble.rs
#[async_trait]
pub trait BleScanner: Send + Sync {
    async fn start_scan(&self) -> Result<(), BleError>;
    async fn stop_scan(&self) -> Result<(), BleError>;
    fn is_scanning(&self) -> bool;
    fn get_discovered_devices(&self) -> Vec<&BleDeviceInfo>;
}

#[async_trait]
pub trait BleConnector: Send + Sync {
    async fn connect_device(&self, address: &str) -> Result<(), BleError>;
    async fn disconnect_device(&self, address: &str) -> Result<(), BleError>;
    fn is_connected(&self, address: &str) -> bool;
}
```

## XP实践要求

### 1. 结对编程
- 所有BLE代码必须两人结对完成
- 一人负责BLE协议逻辑，另一人负责设备管理
- 定期切换角色

### 2. 持续集成
- 每次提交都必须通过所有BLE测试
- 自动化构建和测试
- 快速反馈循环

### 3. 代码审查
- 所有代码必须经过同行审查
- 检查TDD流程是否正确遵循
- 确保BLE协议实现的正确性

### 4. 重构勇气
- 定期重构以保持代码质量
- 消除BLE逻辑中的重复代码
- 提高代码可读性

### 5. 简单设计
- 只实现当前需要的BLE功能
- 避免过度设计BLE实现
- 保持BLE逻辑简单明了

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 90%
- [ ] 遵循Clean Architecture原则
- [ ] 通过代码审查
- [ ] 集成测试通过
- [ ] 硬件测试通过
- [ ] 文档更新完整

## 依赖任务

- Task 0102: 实现设备发现服务
- Task 0103: 实现传输层抽象

## 后续任务

- Task 0106: 实现智能传输选择器
- Task 0107: 实现设备连接管理
- Task 0108: 实现安全配对机制