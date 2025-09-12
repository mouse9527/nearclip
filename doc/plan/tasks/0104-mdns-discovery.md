# Task 0104: 实现mDNS发现 (TDD版本)

## 任务描述

按照TDD原则实现mDNS设备发现功能，支持Zeroconf协议，在局域网内自动发现NearClip设备。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/mdns_discovery_tests.rs
#[cfg(test)]
mod mdns_discovery_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mdns_discovery_starts_and_browses() {
        // RED: 测试mDNS发现服务的启动和浏览
        let mut discovery = MdnsDiscovery::new();
        
        // 初始状态应该是停止的
        assert!(!discovery.is_browsing());
        
        // 启动浏览应该成功
        assert!(discovery.start_browsing("_nearclip._tcp").await.is_ok());
        assert!(discovery.is_browsing());
        
        // 停止浏览应该成功
        assert!(discovery.stop_browsing().await.is_ok());
        assert!(!discovery.is_browsing());
    }
    
    #[tokio::test]
    async fn test_mdns_service_registration() {
        // RED: 测试mDNS服务注册
        let mut discovery = MdnsDiscovery::new();
        
        // 注册服务应该成功
        let service_info = ServiceInfo {
            service_type: "_nearclip._tcp".to_string(),
            instance_name: "My NearClip".to_string(),
            port: 8080,
            properties: vec![
                ("version".to_string(), "1.0".to_string()),
                ("device_id".to_string(), "test-device-123".to_string()),
            ],
        };
        
        assert!(discovery.register_service(service_info.clone()).await.is_ok());
        
        // 服务应该被注册
        assert!(discovery.is_service_registered("My NearClip"));
        
        // 注销服务应该成功
        assert!(discovery.unregister_service("My NearClip").await.is_ok());
        assert!(!discovery.is_service_registered("My NearClip"));
    }
    
    #[tokio::test]
    async fn test_mdns_device_discovery_events() {
        // RED: 测试mDNS设备发现事件
        let mut discovery = MdnsDiscovery::new();
        let mut event_receiver = discovery.subscribe_to_events();
        
        // 启动浏览
        discovery.start_browsing("_nearclip._tcp").await.unwrap();
        
        // 模拟发现服务
        let test_service = ServiceInfo {
            service_type: "_nearclip._tcp".to_string(),
            instance_name: "Test Device".to_string(),
            port: 8080,
            properties: vec![
                ("version".to_string(), "1.0".to_string()),
                ("device_id".to_string(), "test-device-456".to_string()),
                ("device_name".to_string(), "Test Device".to_string()),
            ],
        };
        
        discovery.simulate_service_discovery(test_service).await;
        
        // 应该收到设备发现事件
        let event = timeout(Duration::from_secs(1), event_receiver.recv()).await.unwrap().unwrap();
        match event {
            MdnsEvent::ServiceDiscovered(service) => {
                assert_eq!(service.instance_name, "Test Device");
                assert_eq!(service.properties.iter()
                    .find(|(k, _)| k == "device_id")
                    .unwrap().1, "test-device-456");
            }
            _ => panic!("Expected ServiceDiscovered event"),
        }
        
        discovery.stop_browsing().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_mdns_service_resolution() {
        // RED: 测试mDNS服务解析
        let discovery = MdnsDiscovery::new();
        
        // 模拟一个需要解析的服务
        let service_name = "Test Device._nearclip._tcp.local.";
        let resolved_info = discovery.resolve_service(service_name).await.unwrap();
        
        assert_eq!(resolved_info.instance_name, "Test Device");
        assert!(!resolved_info.addresses.is_empty());
        assert!(resolved_info.port > 0);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub service_type: String,
    pub instance_name: String,
    pub port: u16,
    pub properties: Vec<(String, String)>,
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum MdnsEvent {
    ServiceDiscovered(ServiceInfo),
    ServiceLost(String),
    ServiceResolved(ServiceInfo),
    BrowseStarted,
    BrowseStopped,
}

pub struct MdnsDiscovery {
    is_browsing: bool,
    registered_services: HashMap<String, ServiceInfo>,
    event_sender: broadcast::Sender<MdnsEvent>,
}

impl MdnsDiscovery {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            is_browsing: false,
            registered_services: HashMap::new(),
            event_sender,
        }
    }
    
    pub fn is_browsing(&self) -> bool {
        self.is_browsing
    }
    
    pub async fn start_browsing(&mut self, service_type: &str) -> Result<(), MdnsError> {
        if self.is_browsing {
            return Err(MdnsError::AlreadyBrowsing);
        }
        
        self.is_browsing = true;
        let _ = self.event_sender.send(MdnsEvent::BrowseStarted);
        Ok(())
    }
    
    pub async fn stop_browsing(&mut self) -> Result<(), MdnsError> {
        if !self.is_browsing {
            return Err(MdnsError::NotBrowsing);
        }
        
        self.is_browsing = false;
        let _ = self.event_sender.send(MdnsEvent::BrowseStopped);
        Ok(())
    }
    
    pub async fn register_service(&mut self, service_info: ServiceInfo) -> Result<(), MdnsError> {
        self.registered_services.insert(
            service_info.instance_name.clone(),
            service_info.clone()
        );
        Ok(())
    }
    
    pub async fn unregister_service(&mut self, instance_name: &str) -> Result<(), MdnsError> {
        self.registered_services.remove(instance_name);
        Ok(())
    }
    
    pub fn is_service_registered(&self, instance_name: &str) -> bool {
        self.registered_services.contains_key(instance_name)
    }
    
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<MdnsEvent> {
        self.event_sender.subscribe()
    }
    
    pub async fn resolve_service(&self, service_name: &str) -> Result<ServiceInfo, MdnsError> {
        // 解析服务名称
        let instance_name = service_name.split('.').next().unwrap_or("Unknown");
        
        Ok(ServiceInfo {
            service_type: "_nearclip._tcp".to_string(),
            instance_name: instance_name.to_string(),
            port: 8080,
            properties: vec![],
            addresses: vec!["192.168.1.100".to_string()],
        })
    }
    
    // 测试辅助方法
    pub async fn simulate_service_discovery(&self, service: ServiceInfo) {
        let _ = self.event_sender.send(MdnsEvent::ServiceDiscovered(service));
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct MdnsDiscovery {
    is_browsing: bool,
    browse_service_type: Option<String>,
    registered_services: HashMap<String, RegisteredService>,
    discovered_services: HashMap<String, ServiceInfo>,
    event_sender: broadcast::Sender<MdnsEvent>,
    discovery_timeout: Duration,
}

#[derive(Debug)]
struct RegisteredService {
    info: ServiceInfo,
    registered_at: SystemTime,
}

impl MdnsDiscovery {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            is_browsing: false,
            browse_service_type: None,
            registered_services: HashMap::new(),
            discovered_services: HashMap::new(),
            event_sender,
            discovery_timeout: Duration::from_secs(30),
        }
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.discovery_timeout = timeout;
        self
    }
    
    pub async fn start_browsing(&mut self, service_type: &str) -> Result<(), MdnsError> {
        if self.is_browsing {
            return Err(MdnsError::AlreadyBrowsing);
        }
        
        // 验证服务类型格式
        if !service_type.ends_with("._tcp") && !service_type.ends_with("._udp") {
            return Err(MdnsError::InvalidServiceType);
        }
        
        self.is_browsing = true;
        self.browse_service_type = Some(service_type.to_string());
        
        // 启动实际的mDNS浏览任务
        self.start_mdns_browser(service_type).await?;
        
        let _ = self.event_sender.send(MdnsEvent::BrowseStarted);
        Ok(())
    }
    
    async fn start_mdns_browser(&self, service_type: &str) -> Result<(), MdnsError> {
        // 这里会启动实际的mDNS浏览器
        // 目前是模拟实现
        Ok(())
    }
    
    pub fn add_discovered_service(&mut self, service: ServiceInfo) {
        let service_key = format!("{}.{}", service.instance_name, service.service_type);
        
        if !self.discovered_services.contains_key(&service_key) {
            self.discovered_services.insert(service_key.clone(), service.clone());
            let _ = self.event_sender.send(MdnsEvent::ServiceDiscovered(service));
        }
    }
    
    pub fn remove_discovered_service(&mut self, service_name: &str) {
        let service_key = format!("{}._nearclip._tcp", service_name);
        if self.discovered_services.remove(&service_key).is_some() {
            let _ = self.event_sender.send(MdnsEvent::ServiceLost(service_name.to_string()));
        }
    }
    
    pub fn get_discovered_services(&self) -> Vec<&ServiceInfo> {
        self.discovered_services.values().collect()
    }
}
```

### 必须编写的测试类型

#### 1. 单元测试 (Unit Tests)
- mDNS浏览生命周期测试
- 服务注册和注销测试
- 事件系统测试
- 服务解析测试
- 错误处理测试

#### 2. 集成测试 (Integration Tests)
- mDNS库集成测试
- 网络条件测试
- 多服务并发测试
- 服务缓存测试

#### 3. 网络测试 (Network Tests)
- 实际网络环境测试
- 防火墙穿透测试
- 多子网测试
- 重连机制测试

### 测试覆盖率要求
- **单元测试覆盖率**: > 90%
- **集成测试覆盖率**: > 80%
- **网络测试覆盖率**: > 70%

## Clean Architecture要求

### 依赖倒置原则
mDNS发现应该依赖domain层的Discovery接口：

```rust
// rust-core/infrastructure/discovery/mdns_discovery.rs
pub struct MdnsDiscovery {
    // mDNS具体实现
}

#[async_trait]
impl DiscoveryMethod for MdnsDiscovery {
    async fn start_discovery(&self) -> Result<(), DiscoveryError> {
        // 实现mDNS发现
    }
}
```

### 接口隔离原则
定义mDNS特定的接口：

```rust
// rust-core/domain/interfaces/mdns.rs
#[async_trait]
pub trait MdnsBrowser: Send + Sync {
    async fn start_browsing(&self, service_type: &str) -> Result<(), MdnsError>;
    async fn stop_browsing(&self) -> Result<(), MdnsError>;
    fn browse_services(&self) -> Vec<&ServiceInfo>;
}

#[async_trait]
pub trait MdnsRegistrar: Send + Sync {
    async fn register_service(&self, service: ServiceInfo) -> Result<(), MdnsError>;
    async fn unregister_service(&self, instance_name: &str) -> Result<(), MdnsError>;
}
```

## XP实践要求

### 1. 结对编程
- 所有mDNS代码必须两人结对完成
- 一人负责mDNS协议逻辑，另一人负责事件系统
- 定期切换角色

### 2. 持续集成
- 每次提交都必须通过所有mDNS测试
- 自动化构建和测试
- 快速反馈循环

### 3. 代码审查
- 所有代码必须经过同行审查
- 检查TDD流程是否正确遵循
- 确保mDNS协议实现的正确性

### 4. 重构勇气
- 定期重构以保持代码质量
- 消除mDNS逻辑中的重复代码
- 提高代码可读性

### 5. 简单设计
- 只实现当前需要的mDNS功能
- 避免过度设计mDNS实现
- 保持mDNS逻辑简单明了

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 90%
- [ ] 遵循Clean Architecture原则
- [ ] 通过代码审查
- [ ] 集成测试通过
- [ ] 网络测试通过
- [ ] 文档更新完整

## 依赖任务

- [Task 0102: 定义设备状态枚举](0102-device-status-enum.md)
- [Task 0103: 实现传输层抽象接口](0103-transport-abstraction.md)

## 后续任务

- [Task 0105: 实现BLE发现](0105-ble-discovery.md)
- [Task 0106: 实现智能传输选择器](0106-intelligent-transport-selector.md)
- [Task 0107: 实现设备连接管理](0107-device-connection-manager.md)