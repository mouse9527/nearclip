# XP实践指南

本文档提供NearClip项目的XP（极限编程）最佳实践指南。

## XP核心价值观

### 沟通 (Communication)
- 面对面沟通优于书面文档
- 结对编程促进知识共享
- 每日站会保持信息同步

### 简单 (Simplicity)  
- KISS (Keep It Simple, Stupid)
- YAGNI (You Ain't Gonna Need It)
- DTSTTCPW (Do The Simplest Thing That Could Possibly Work)

### 反馈 (Feedback)
- TDD提供即时反馈
- 持续集成提供构建反馈
- 结对编程提供设计反馈

### 勇气 (Courage)
- 重构的勇气
- 丢弃代码的勇气
- 承认错误的勇气

### 尊重 (Respect)
- 尊重团队成员
- 尊重代码质量
- 尊重用户需求

## XP实践实施

### 1. 结对编程 (Pair Programming)

#### 角色分工
```
Driver (驾驶员)                     | Navigator (领航员)
-----------------------------------|-----------------------------------
控制键盘和鼠标                      | 思考整体方向
编写具体的代码                      | 指导设计和策略
关注当前的实现细节                  | 关注未来的扩展性
执行测试和重构                      | 识别问题和改进机会
```

#### 切换频率
- 每25-30分钟切换一次角色
- 疲劳时立即切换
- 完成一个任务单元时切换

#### 结对规则
- **不要单独编写生产代码**
- **保持对话活跃**
- **互相学习**
- **定期休息**

#### 实施示例
```rust
// Driver: 编写测试
#[test]
fn test_device_discovery_returns_devices() {
    let discovery = DeviceDiscovery::new();
    let devices = discovery.discover().await.unwrap();
    assert!(!devices.is_empty());
}

// Navigator: "我们应该考虑网络不可用的情况"
// Driver: 好的，让我添加一个测试

#[test]
fn test_device_discovery_handles_network_failure() {
    let discovery = DeviceDiscovery::with_network_failure();
    let result = discovery.discover().await;
    assert!(matches!(result, Err(DiscoveryError::NetworkUnavailable)));
}
```

### 2. 测试驱动开发 (TDD)

#### 完整的TDD循环
```rust
// 1. RED: 编写失败的测试
#[test]
fn test_ble_device_discovery_filters_by_service_uuid() {
    let discovery = BleDiscovery::new();
    let devices = discovery.discover_by_service("nearclip").await.unwrap();
    
    let filtered_devices: Vec<_> = devices.iter()
        .filter(|d| d.services().contains(&"nearclip".to_string()))
        .collect();
    
    assert!(!filtered_devices.is_empty());
}

// 2. GREEN: 编写最小实现
impl BleDiscovery {
    pub async fn discover_by_service(&self, service_uuid: &str) -> Result<Vec<BleDevice>, DiscoveryError> {
        // 最小实现，返回硬编码数据
        Ok(vec![BleDevice::mock_with_service(service_uuid)])
    }
}

// 3. REFACTOR: 重构为真实实现
impl BleDiscovery {
    pub async fn discover_by_service(&self, service_uuid: &str) -> Result<Vec<BleDevice>, DiscoveryError> {
        let all_devices = self.scan_all_devices().await?;
        let filtered_devices = all_devices.into_iter()
            .filter(|d| d.services().contains(&service_uuid.to_string()))
            .collect();
        Ok(filtered_devices)
    }
}
```

### 3. 持续集成 (Continuous Integration)

#### CI/CD流水线
```yaml
# .github/workflows/ci.yml
name: Continuous Integration
on: 
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run unit tests
      run: cargo test --lib
    
    - name: Run integration tests
      run: cargo test --test "*"
    
    - name: Generate coverage
      if: matrix.os == 'ubuntu-latest' && matrix.rust == 'stable'
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml
    
    - name: Upload coverage
      if: matrix.os == 'ubuntu-latest' && matrix.rust == 'stable'
      uses: codecov/codecov-action@v3

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Build release
      run: cargo build --release
```

#### 分支策略
```
main                    # 生产分支，始终保持可部署状态
├── develop             # 开发分支，集成最新功能
├── feature/hybrid-transport # 功能分支
├── bugfix/memory-leak  # Bug修复分支
└── hotfix/security-patch # 紧急修复分支
```

### 4. 小版本发布 (Small Releases)

#### 发布频率
- **功能发布**: 每1-2周
- **Bug修复发布**: 按需
- **主版本发布**: 每季度评估

#### 发布检查清单
- [ ] 所有测试通过
- [ ] 代码审查完成
- [ ] 文档更新
- [ ] 性能测试通过
- [ ] 安全测试通过
- [ ] 版本号更新

#### 版本号规则
```rust
// Cargo.toml
[package]
version = "0.1.0"  # MAJOR.MINOR.PATCH

// PATCH: Bug修复，向后兼容
// MINOR: 新功能，向后兼容  
// MAJOR: 破坏性变更
```

### 5. 简单设计 (Simple Design)

#### 设计原则
1. **通过所有测试**
2. **消除重复** (DRY)
3. **明确表达意图**
4. **最小化组件数量**

#### 重构勇气
```rust
// 重构前: 重复的代码
impl WifiTransport {
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.is_connected {
            return Err(TransportError::NotConnected);
        }
        // 发送逻辑...
    }
}

impl BleTransport {
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.is_connected {
            return Err(TransportError::NotConnected);
        }
        // 发送逻辑...
    }
}

// 重构后: 提取共同逻辑
trait DataSender {
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    
    fn ensure_connected(&self) -> Result<(), TransportError> {
        if !self.is_connected {
            Err(TransportError::NotConnected)
        } else {
            Ok(())
        }
    }
}
```

### 6. 编码标准 (Coding Standards)

#### Rust代码风格
```rust
// 好的命名
pub struct DeviceDiscoveryService {
    discovered_devices: Vec<DeviceInfo>,
    scan_duration: Duration,
}

// 方法命名清晰
impl DeviceDiscoveryService {
    pub async fn discover_devices(&mut self) -> Result<Vec<DeviceInfo>, DiscoveryError> {
        // ...
    }
    
    pub fn set_scan_duration(&mut self, duration: Duration) {
        self.scan_duration = duration;
    }
}

// 错误处理
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("Network unavailable: {0}")]
    NetworkUnavailable(String),
    
    #[error("Timeout during discovery")]
    Timeout,
    
    #[error("Permission denied")]
    PermissionDenied,
}
```

#### 文档注释
```rust
/// 设备发现服务
/// 
/// 负责发现网络中的其他NearClip设备。
/// 支持mDNS和BLE两种发现方式。
pub struct DeviceDiscoveryService {
    // ...
}

impl DeviceDiscoveryService {
    /// 开始设备发现
    /// 
    /// # 参数
    /// * `timeout` - 发现超时时间
    /// 
    /// # 返回
    /// 返回发现的设备列表，或在超时时返回错误
    /// 
    /// # 示例
    /// ```
    /// let mut service = DeviceDiscoveryService::new();
    /// let devices = service.start_discovery(Duration::from_secs(30)).await?;
    /// ```
    pub async fn start_discovery(&mut self, timeout: Duration) -> Result<Vec<DeviceInfo>, DiscoveryError> {
        // ...
    }
}
```

### 7. 集体代码所有权 (Collective Code Ownership)

#### 代码审查清单
- [ ] 代码遵循TDD流程
- [ ] 测试覆盖率符合要求
- [ ] 代码风格一致
- [ ] 错误处理完善
- [ ] 性能考虑充分
- [ ] 安全问题检查
- [ ] 文档更新完整

#### 知识共享
- **定期技术分享**: 每周一次
- **结对编程轮换**: 每天更换结对对象
- **代码漫游**: 定期代码审查

### 8. 可持续节奏 (Sustainable Pace)

#### 工作时间
- **标准工作周**: 40小时
- **无加班文化**: 紧急情况除外
- **休息时间**: 每25分钟休息5分钟

#### 避免倦怠
- **定期休息**: 每工作2小时休息15分钟
- **环境切换**: 到户外走走
- **学习时间**: 工作时间中保留学习时间

## XP实践检查清单

### 每日实践
- [ ] 进行每日站会
- [ ] 结对编程
- [ ] TDD红绿重构
- [ ] 频繁提交代码

### 每周实践  
- [ ] 代码审查
- [ ** 集成测试
- [ ] 重构会话
- [ ] 技术分享

### 每月实践
- [ ] 回顾会议
- [ ] 发布计划
- [ ] 团队建设
- [ ] 流程改进

## 工具和资源

### 必要工具
- **版本控制**: Git + GitHub/GitLab
- **CI/CD**: GitHub Actions
- **项目管理**: GitHub Projects
- **沟通工具**: Slack/Teams
- **代码质量**: SonarQube

### 推荐阅读
- "Extreme Programming Explained" by Kent Beck
- "The Clean Coder" by Robert C. Martin
- "Refactoring" by Martin Fowler
- "Continuous Delivery" by Jez Humble

### 在线资源
- XP联盟: https://www.extremeprogramming.org/
- Agile Manifesto: https://agilemanifesto.org/
- Rust最佳实践: https://rust-lang.github.io/api-guidelines/