# TDD开发指南

本文档提供NearClip项目的TDD（测试驱动开发）详细指南。

## TDD核心原则

### 红绿重构循环

```
RED → 写一个失败的测试
GREEN → 写最少代码让测试通过
REFACTOR → 重构代码，保持测试绿色
```

### 关键实践

1. **先写测试**: 在编写任何生产代码之前，先写一个失败的测试
2. **最小实现**: 只写刚好让测试通过的代码，不多不少
3. **持续重构**: 定期重构代码，消除重复，提高设计

## TDD实施步骤

### 第一步：RED - 编写失败的测试

```rust
// tests/domain/device_tests.rs
#[cfg(test)]
mod device_tests {
    use super::*;
    
    #[test]
    fn test_device_creation() {
        // RED: 这个测试会失败，因为我们还没有Device实现
        let device = Device::new("test-device", "Test Device");
        assert_eq!(device.id(), "test-device");
    }
}
```

### 第二步：GREEN - 最小实现

```rust
// 只写刚好让测试通过的代码
pub struct Device {
    id: String,
    name: String,
}

impl Device {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}
```

### 第三步：REFACTOR - 重构代码

```rust
// 重构以改善设计，保持测试绿色
impl Device {
    // 可以提取常量，改善命名等
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}
```

## 测试类型和策略

### 1. 单元测试 (Unit Tests)

**目的**: 测试单个函数或方法的行为

**位置**: `tests/unit/` 或与代码同目录的 `tests/` 子目录

**示例**:
```rust
#[cfg(test)]
mod device_tests {
    use super::*;
    
    #[test]
    fn test_device_status_transition() {
        let mut device = Device::new("test", "Test Device");
        
        // 测试状态转换
        assert_eq!(device.status(), DeviceStatus::Disconnected);
        device.connect().unwrap();
        assert_eq!(device.status(), DeviceStatus::Connected);
    }
    
    #[test]
    fn test_device_error_handling() {
        let mut device = Device::new("test", "Test Device");
        
        // 测试错误路径
        let result = device.send_data(b"data");
        assert!(matches!(result, Err(DeviceError::NotConnected)));
    }
}
```

### 2. 集成测试 (Integration Tests)

**目的**: 测试多个组件的交互

**位置**: `tests/integration/`

**示例**:
```rust
#[tokio::test]
async fn test_device_discovery_integration() {
    // 测试发现服务与存储的集成
    let discovery_service = DiscoveryService::new();
    let storage = MockStorage::new();
    
    let devices = discovery_service.discover_devices().await;
    storage.save_devices(&devices).await.unwrap();
    
    let retrieved_devices = storage.load_devices().await.unwrap();
    assert_eq!(devices.len(), retrieved_devices.len());
}
```

### 3. 接受测试 (Acceptance Tests)

**目的**: 测试完整的用户场景

**位置**: `tests/acceptance/`

**示例**:
```rust
#[tokio::test]
async fn test_complete_device_pairing_workflow() {
    // 测试完整的配对流程
    let app = TestApplication::new().await;
    
    // 模拟用户操作
    let device_a = app.create_device("Device A");
    let device_b = app.create_device("Device B");
    
    // 执行配对
    app.pair_devices(&device_a, &device_b).await.unwrap();
    
    // 验证结果
    assert!(app.is_paired(&device_a, &device_b).await);
    assert!(app.can_send_data(&device_a, &device_b).await);
}
```

## Clean Architecture中的TDD

### 依赖注入测试

```rust
#[cfg(test)]
mod device_service_tests {
    use super::*;
    
    #[test]
    fn test_device_service_with_mock_repository() {
        // 使用Mock对象测试业务逻辑
        let mock_repo = MockDeviceRepository::new();
        let service = DeviceService::new(Arc::new(mock_repo));
        
        let device = service.create_device("test-device").unwrap();
        assert_eq!(device.name(), "test-device");
    }
}
```

### 接口测试

```rust
#[cfg(test)]
mod transport_trait_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transport_trait_compliance() {
        // 测试所有Transport实现都符合trait要求
        let transports: Vec<Box<dyn Transport>> = vec![
            Box::new(WifiTransport::new()),
            Box::new(BleTransport::new()),
        ];
        
        for transport in transports {
            test_transport_lifecycle(transport).await;
        }
    }
}
```

## 测试最佳实践

### 1. 测试命名

```
// 好的测试名称
test_device_creation_with_valid_id()
test_device_connection_failure_when_already_connected()
test_data_transfer_with_large_payload()

// 避免的测试名称
test_device()
test_connection()
test_data()
```

### 2. 测试组织

```rust
#[cfg(test)]
mod device_tests {
    use super::*;
    
    mod creation {
        use super::*;
        
        #[test]
        fn test_creates_device_with_valid_inputs() {
            // ...
        }
        
        #[test]
        fn test_rejects_empty_device_id() {
            // ...
        }
    }
    
    mod connection {
        use super::*;
        
        #[test]
        fn test_connects_successfully() {
            // ...
        }
        
        #[test]
        fn test_fails_when_already_connected() {
            // ...
        }
    }
}
```

### 3. Mock和Stub

```rust
// 使用Mock对象
#[derive(Debug)]
struct MockDeviceRepository {
    devices: RefCell<Vec<Device>>,
    should_fail: Cell<bool>,
}

impl DeviceRepository for MockDeviceRepository {
    async fn save_device(&self, device: &Device) -> Result<(), RepositoryError> {
        if self.should_fail.get() {
            return Err(RepositoryError::SaveFailed);
        }
        self.devices.borrow_mut().push(device.clone());
        Ok(())
    }
    
    async fn find_device(&self, id: &str) -> Option<Device> {
        self.devices.borrow().iter().find(|d| d.id() == id).cloned()
    }
}
```

### 4. 测试数据管理

```rust
// Test Factory模式
pub struct DeviceFactory;

impl DeviceFactory {
    pub fn create_connected_device(id: &str) -> Device {
        let mut device = Device::new(id, "Test Device");
        device.connect().unwrap();
        device
    }
    
    pub fn create_device_with_rssi(id: &str, rssi: i32) -> Device {
        let mut device = Device::new(id, "Test Device");
        device.set_rssi(rssi);
        device
    }
}
```

## 测试覆盖率要求

### 覆盖率目标
- **单元测试**: > 90%
- **集成测试**: > 80%
- **关键业务逻辑**: 100%

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration

# 生成覆盖率报告
cargo tarpaulin

# 运行特定测试
cargo test device_creation
```

## XP实践集成

### 结对编程中的TDD

```
Driver (键盘) | Navigator (思考)
----------------|----------------
编写测试        | 讨论测试场景
编写实现代码    | 指导设计方向
重构代码        | 识别改进机会
```

### 持续集成

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Run tests
      run: |
        cargo test --lib
        cargo test --integration
        cargo tarpaulin --out Xml
    - name: Upload coverage
      uses: codecov/codecov-action@v1
```

## 常见反模式

### 避免这些做法

1. **编写"实现测试"**: 测试具体实现而不是行为
2. **忽略失败的测试**: 暂时注释掉失败的测试
3. **过度Mock**: Mock太多，失去测试价值
4. **测试私有方法**: 只测试公共接口
5. **重复的测试代码**: 使用helper函数和factory

### 正确的做法

1. **测试行为而非实现**: 关注"做什么"而不是"怎么做"
2. **保持测试独立**: 每个测试都应该独立运行
3. **使用有意义的断言**: 明确验证预期行为
4. **保持测试简单**: 避免复杂的测试逻辑
5. **定期运行测试**: 频繁运行测试获得快速反馈

## 工具和资源

### 必要工具
- **cargo-nextest**: 更快的测试运行器
- **cargo-tarpaulin**: 代码覆盖率工具
- **mockall**: Mock对象库
- **proptest**: 基于属性的测试

### 学习资源
- "Test-Driven Development" by Kent Beck
- "Growing Object-Oriented Software" by Steve Freeman
- Rust测试文档: https://doc.rust-lang.org/book/ch11-00-testing.html