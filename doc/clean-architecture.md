# Clean Architecture 设计文档

## 架构概览

本项目严格遵循Clean Architecture原则，确保业务逻辑独立于框架、UI和数据库。

## 依赖规则

**核心原则**: 源代码依赖关系必须指向内部。内层不应该知道外层的任何信息。

```
┌─────────────────────────────────────────────┐
│                Frameworks & Drivers          │
│          (UI, Database, External APIs)      │
├─────────────────────────────────────────────┤
│              Interface Adapters             │
│     (Controllers, Presenters, Gateways)     │
├─────────────────────────────────────────────┤
│                Use Cases                   │
│        (Application Business Rules)         │
├─────────────────────────────────────────────┤
│                  Entities                  │
│           (Enterprise Business Rules)       │
└─────────────────────────────────────────────┘
```

## 分层详细设计

### 1. Entities Layer (实体层)
**职责**: 最核心的业务规则，独立于任何外部因素

```rust
// rust-core/domain/entities/
├── device.rs           // 设备实体和核心业务规则
├── encryption.rs       // 加密算法实体
├── transport.rs        // 传输抽象实体
└── clipboard.rs        // 剪贴板实体
```

**关键实体**:
- `Device`: 设备标识、状态、能力
- `EncryptionKey`: 密钥管理和生命周期
- `TransportMethod`: 传输方式抽象
- `ClipboardContent`: 剪贴板内容类型

### 2. Use Cases Layer (用例层)
**职责**: 应用特定的业务规则，协调Entities和外部操作

```rust
// rust-core/application/use_cases/
├── device_discovery.rs     // 设备发现用例
├── device_pairing.rs       // 设备配对用例
├── content_sync.rs         // 内容同步用例
├── transport_selection.rs  // 传输选择用例
└── device_management.rs    // 设备管理用例
```

**关键用例**:
- `DiscoverDevices`: 设备发现和聚合
- `PairDevices`: 安全配对流程
- `SynchronizeClipboard`: 剪贴板同步逻辑
- `SelectOptimalTransport`: 智能传输选择
- `ManageDeviceConnections`: 设备连接管理

### 3. Interface Adapters Layer (接口适配器层)
**职责**: 转换数据格式，连接外层和内层

```rust
// rust-core/interfaces/
├── controllers/
│   ├── device_controller.rs      // 设备控制器
│   └── sync_controller.rs        // 同步控制器
├── presenters/
│   ├── device_presenter.rs      // 设备展示器
│   └── connection_presenter.rs  // 连接展示器
├── repositories/
│   ├── device_repository.rs      // 设备数据访问
│   └── pairing_repository.rs     // 配对数据访问
└── gateways/
    ├── network_gateway.rs       // 网络网关
    └── ble_gateway.rs          // BLE网关
```

### 4. Frameworks & Drivers Layer (框架和驱动层)
**职责**: 具体的技术实现细节

```rust
// rust-core/infrastructure/
├── transports/
│   ├── wifi_transport.rs       // WiFi传输实现
│   ├── ble_transport.rs        // BLE传输实现
│   └── transport_factory.rs    // 传输工厂
├── persistence/
│   ├── sqlite_device_store.rs   // SQLite设备存储
│   └── secure_key_store.rs     // 安全密钥存储
├── discovery/
│   ├── mdns_discovery.rs       // mDNS发现实现
│   └── ble_discovery.rs        // BLE发现实现
└── external/
    ├── android_ffi.rs          // Android FFI绑定
    ├── ios_ffi.rs             // iOS FFI绑定
    └── desktop_ffi.rs        // 桌面FFI绑定
```

## 依赖注入和控制反转

### 依赖注入容器
```rust
// rust-core/application/container.rs
pub struct ApplicationContainer {
    device_repository: Arc<dyn DeviceRepository>,
    pairing_repository: Arc<dyn PairingRepository>,
    transport_factory: Arc<dyn TransportFactory>,
    discovery_service: Arc<dyn DiscoveryService>,
    encryption_service: Arc<dyn EncryptionService>,
}

impl ApplicationContainer {
    pub fn new() -> Self {
        Self {
            device_repository: Arc::new(SqliteDeviceStore::new()),
            pairing_repository: Arc::new(SecureKeyStore::new()),
            transport_factory: Arc::new(TransportFactoryImpl::new()),
            discovery_service: Arc::new(HybridDiscoveryService::new()),
            encryption_service: Arc::new(EcdhAesEncryptionService::new()),
        }
    }
    
    pub fn device_discovery_use_case(&self) -> DeviceDiscoveryUseCase {
        DeviceDiscoveryUseCase::new(
            self.discovery_service.clone(),
            self.device_repository.clone(),
        )
    }
}
```

## TDD实施策略

### 测试驱动开发流程

#### 1. 单元测试策略
```rust
// tests/domain/device_tests.rs
#[cfg(test)]
mod device_tests {
    use super::*;
    
    #[test]
    fn test_device_creation() {
        // RED: 先写失败的测试
        let device = Device::new("test-device", "Test Device");
        assert_eq!(device.id(), "test-device");
        assert_eq!(device.name(), "Test Device");
        assert_eq!(device.status(), DeviceStatus::Discovered);
    }
    
    #[test]
    fn test_device_connection_state_transition() {
        // RED: 测试状态转换
        let mut device = Device::new("test-device", "Test Device");
        
        // GREEN: 写最小代码让测试通过
        device.connect().unwrap();
        assert_eq!(device.status(), DeviceStatus::Connected);
        
        device.disconnect().unwrap();
        assert_eq!(device.status(), DeviceStatus::Disconnected);
    }
}
```

#### 2. 集成测试策略
```rust
// tests/integration/transport_selection_tests.rs
#[cfg(test)]
mod transport_selection_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transport_selection_wifi_preferred() {
        // RED: 测试WiFi优先选择逻辑
        let context = ContextInfo {
            network_context: NetworkContext {
                wifi_available: true,
                wifi_quality: NetworkQuality::Good,
                // ... 其他字段
            },
            // ... 其他上下文
        };
        
        let selector = TransportSelector::new();
        let available_transports = vec![TransportType::Wifi, TransportType::Ble];
        
        let result = selector.select_best_transport(&available_transports, &context).await;
        
        // GREEN: 验证选择结果
        assert_eq!(result.selected_transport, TransportType::Wifi);
    }
}
```

#### 3. 接受测试策略
```rust
// tests/acceptance/device_pairing_tests.rs
#[cfg(test)]
mod device_pairing_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complete_device_pairing_workflow() {
        // RED: 测试完整的配对工作流
        let mut pairing_service = DevicePairingService::new();
        
        // 模拟设备A和设备B
        let device_a = create_test_device("device-a");
        let device_b = create_test_device("device-b");
        
        // GREEN: 执行配对流程
        let pairing_result = pairing_service.pair_devices(&device_a, &device_b).await;
        
        assert!(pairing_result.is_ok());
        
        // 验证配对状态
        assert!(pairing_service.is_paired(&device_a.id(), &device_b.id()).await);
        assert!(pairing_service.has_shared_secret(&device_a.id(), &device_b.id()).await);
    }
}
```

## XP实践实现

### 1. 持续集成配置
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v2
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: |
        cargo test --lib
        cargo test --integration
        cargo clippy -- -D warnings
    - name: Check formatting
      run: cargo fmt -- --check
```

### 2. 编码标准
```rust
// rust-core/.clippy.toml
max_line_length = 100
tab_spaces = 4
newline_style = "Unix"

# 强制使用Result而不是panic
disallowed_names = ["panic", "unreachable"]
```

### 3. 简单设计原则
- **KISS**: 保持简单，避免过度设计
- **YAGNI**: 只实现当前需要的功能
- **DRY**: 避免重复代码
- **SOLID**: 遵循SOLID原则

## 文件结构

```
rust-core/
├── domain/                    # 实体层
│   ├── entities/
│   │   ├── device.rs
│   │   ├── encryption.rs
│   │   └── transport.rs
│   └── errors.rs              # 领域错误
├── application/               # 应用层
│   ├── use_cases/
│   ├── dto/                   # 数据传输对象
│   └── container.rs           # 依赖注入容器
├── interfaces/                # 接口适配器层
│   ├── controllers/
│   ├── presenters/
│   ├── repositories/
│   └── gateways/
├── infrastructure/            # 基础设施层
│   ├── transports/
│   ├── persistence/
│   ├── discovery/
│   └── external/
├── tests/                     # 测试
│   ├── unit/
│   ├── integration/
│   └── acceptance/
├── lib.rs                     # 库入口
└── main.rs                    # 应用入口
```

## 关键设计决策

### 1. 依赖注入原则
- 所有依赖通过接口注入
- 使用Arc<dyn Trait>实现多态
- 避免硬编码的具体实现

### 2. 错误处理策略
- 使用Result<T, E>进行错误处理
- 领域特定的错误类型
- 避免panic和unwrap

### 3. 异步处理
- 使用async/await进行异步编程
- 明确的生命周期管理
- 避免阻塞操作

### 4. 测试覆盖要求
- 单元测试覆盖率 > 90%
- 所有公共API都有集成测试
- 关键业务流程有接受测试