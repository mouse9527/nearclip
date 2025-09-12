# Task 0103: 实现传输层抽象接口 (TDD版本)

## 任务描述

按照TDD原则实现统一的传输层抽象接口，为应用层提供一致的传输API，支持WiFi和BLE两种传输方式。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transport_trait_tests.rs
#[cfg(test)]
mod transport_trait_tests {
    use super::*;
    
    #[test]
    fn test_transport_trait_connection_cycle() {
        // RED: 测试Transport trait的基本连接生命周期
        // 这个测试会失败，因为我们还没有实现任何Transport
        
        let mut transport = create_mock_transport();
        
        // 初始状态应该是未连接
        assert!(!transport.is_connected());
        
        // 连接应该成功
        assert!(transport.connect().is_ok());
        assert!(transport.is_connected());
        
        // 断开连接应该成功
        assert!(transport.disconnect().is_ok());
        assert!(!transport.is_connected());
    }
    
    #[tokio::test]
    async fn test_transport_data_transfer() {
        // RED: 测试数据传输功能
        let mut transport = create_mock_transport();
        transport.connect().await.unwrap();
        
        let test_data = b"Hello, World!";
        
        // 发送数据应该成功
        assert!(transport.send_data(test_data).await.is_ok());
        
        // 接收数据应该返回发送的数据
        let received_data = transport.receive_data().await.unwrap();
        assert_eq!(received_data, test_data);
    }
    
    #[test]
    fn test_transport_quality_assessment() {
        // RED: 测试传输质量评估
        let transport = create_mock_transport();
        
        // 应该能够获取传输质量分数
        let quality_score = transport.get_quality_score();
        assert!(quality_score >= 0.0 && quality_score <= 1.0);
        
        // 应该能够获取传输类型
        let transport_type = transport.get_transport_type();
        assert!(!transport_type.is_empty());
    }
    
    #[tokio::test]
    async fn test_transport_error_handling() {
        // RED: 测试传输错误处理
        let mut transport = create_mock_transport();
        
        // 未连接时发送数据应该返回错误
        let result = transport.send_data(b"test").await;
        assert!(matches!(result, Err(TransportError::NotConnected)));
        
        // 未连接时接收数据应该返回错误
        let result = transport.receive_data().await;
        assert!(matches!(result, Err(TransportError::NotConnected)));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum TransportError {
    NotConnected,
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    Timeout,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransportType {
    Wifi,
    Ble,
    Hybrid,
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<(), TransportError>;
    async fn disconnect(&mut self) -> Result<(), TransportError>;
    fn is_connected(&self) -> bool;
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    async fn receive_data(&mut self) -> Result<Vec<u8>, TransportError>;
    fn get_quality_score(&self) -> f32;
    fn get_transport_type(&self) -> TransportType;
}

// Mock实现用于测试
pub struct MockTransport {
    is_connected: bool,
    pending_data: VecDeque<Vec<u8>>,
    transport_type: TransportType,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            pending_data: VecDeque::new(),
            transport_type: TransportType::Wifi,
        }
    }
    
    pub fn with_transport_type(transport_type: TransportType) -> Self {
        Self {
            is_connected: false,
            pending_data: VecDeque::new(),
            transport_type,
        }
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn connect(&mut self) -> Result<(), TransportError> {
        self.is_connected = true;
        Ok(())
    }
    
    async fn disconnect(&mut self) -> Result<(), TransportError> {
        self.is_connected = false;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.is_connected {
            return Err(TransportError::NotConnected);
        }
        self.pending_data.push_back(data.to_vec());
        Ok(())
    }
    
    async fn receive_data(&mut self) -> Result<Vec<u8>, TransportError> {
        if !self.is_connected {
            return Err(TransportError::NotConnected);
        }
        self.pending_data.pop_front()
            .ok_or(TransportError::ReceiveFailed("No data available".to_string()))
    }
    
    fn get_quality_score(&self) -> f32 {
        0.8 // 固定质量分数用于测试
    }
    
    fn get_transport_type(&self) -> TransportType {
        self.transport_type.clone()
    }
}

fn create_mock_transport() -> MockTransport {
    MockTransport::new()
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug, Clone)]
pub struct TransportMetrics {
    pub latency_ms: u32,
    pub bandwidth_mbps: f32,
    pub packet_loss_rate: f32,
    pub signal_strength: i32,
}

#[derive(Debug)]
pub struct ConnectionState {
    pub is_connected: bool,
    pub last_connected: Option<SystemTime>,
    pub connection_attempts: u32,
    pub metrics: TransportMetrics,
}

impl MockTransport {
    // 重构后的代码，保持测试绿色
    pub fn new_with_metrics(transport_type: TransportType, metrics: TransportMetrics) -> Self {
        Self {
            is_connected: false,
            pending_data: VecDeque::new(),
            transport_type,
            state: ConnectionState {
                is_connected: false,
                last_connected: None,
                connection_attempts: 0,
                metrics,
            },
        }
    }
    
    fn calculate_quality_score(&self) -> f32 {
        let metrics = &self.state.metrics;
        
        // 基于多个指标计算综合质量分数
        let latency_score = if metrics.latency_ms < 50 { 1.0 } 
                           else if metrics.latency_ms < 100 { 0.8 } 
                           else if metrics.latency_ms < 200 { 0.6 } 
                           else { 0.3 };
        
        let bandwidth_score = if metrics.bandwidth_mbps > 10 { 1.0 }
                             else if metrics.bandwidth_mbps > 5 { 0.8 }
                             else if metrics.bandwidth_mbps > 1 { 0.6 }
                             else { 0.3 };
        
        let packet_loss_score = 1.0 - metrics.packet_loss_rate;
        let signal_score = metrics.signal_strength.max(-100).min(0) as f32 / -100.0;
        
        // 加权平均
        (latency_score * 0.3 + bandwidth_score * 0.3 + 
         packet_loss_score * 0.2 + signal_score * 0.2).max(0.0).min(1.0)
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn connect(&mut self) -> Result<(), TransportError> {
        self.state.connection_attempts += 1;
        
        // 模拟连接成功概率
        if self.state.connection_attempts > 3 {
            return Err(TransportError::ConnectionFailed(
                "Connection failed after multiple attempts".to_string()
            ));
        }
        
        self.is_connected = true;
        self.state.is_connected = true;
        self.state.last_connected = Some(SystemTime::now());
        Ok(())
    }
    
    fn get_quality_score(&self) -> f32 {
        if !self.is_connected {
            0.0
        } else {
            self.calculate_quality_score()
        }
    }
}
```

### 必须编写的测试类型

#### 1. 单元测试 (Unit Tests)
- Transport trait接口测试
- 连接状态管理测试
- 数据传输测试
- 错误处理测试
- 质量评估测试

#### 2. 集成测试 (Integration Tests)
- Transport trait与具体实现的集成
- 事件通知系统的集成测试
- 配置系统的集成测试

#### 3. 性能测试 (Performance Tests)
- 传输质量评估性能
- 大数据量传输性能
- 并发连接性能

### 测试覆盖率要求
- **单元测试覆盖率**: > 90%
- **集成测试覆盖率**: > 80%
- **关键路径测试**: 100%

## Clean Architecture要求

### 依赖倒置原则
Transport trait必须定义在domain层，具体实现在infrastructure层：

```rust
// rust-core/domain/entities/transport.rs
#[async_trait]
pub trait Transport: Send + Sync {
    // trait定义，不依赖具体实现
}

// rust-core/infrastructure/transports/mock_transport.rs
pub struct MockTransport {
    // 具体实现
}

#[async_trait]
impl Transport for MockTransport {
    // 实现trait
}
```

### 接口隔离原则
使用最小必要的接口，避免接口污染：

```rust
// 小接口，单一职责
pub trait ConnectionManager {
    async fn connect(&mut self) -> Result<(), TransportError>;
    async fn disconnect(&mut self) -> Result<(), TransportError>;
    fn is_connected(&self) -> bool;
}

pub trait DataTransfer {
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    async fn receive_data(&mut self) -> Result<Vec<u8>, TransportError>;
}

// Transport trait组合小接口
#[async_trait]
pub trait Transport: ConnectionManager + DataTransfer {
    // 额外的方法
    fn get_quality_score(&self) -> f32;
    fn get_transport_type(&self) -> TransportType;
}
```

## XP实践要求

### 1. 结对编程
- 所有Transport trait实现必须由两人结对完成
- 一人编写测试，另一人编写实现
- 定期切换角色

### 2. 持续集成
- 每次提交都必须通过所有测试
- 自动化构建和测试
- 快速反馈循环

### 3. 代码审查
- 所有代码必须经过同行审查
- 检查TDD流程是否正确遵循
- 确保没有过度设计

### 4. 重构勇气
- 定期重构以保持代码质量
- 消除代码重复
- 提高代码可读性

### 5. 简单设计
- 只实现当前需要的功能
- 避免过度设计和镀金
- 保持代码简洁明了

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 90%
- [ ] 遵循Clean Architecture原则
- [ ] 通过代码审查
- [ ] 性能测试通过
- [ ] 集成测试通过
- [ ] 文档更新完整

## 依赖任务

- Task 0101: 实现设备抽象层
- Task 0102: 实现设备发现服务

## 后续任务

- Task 0104: 实现WiFi传输
- Task 0105: 实现BLE传输
- Task 0106: 实现智能传输选择器