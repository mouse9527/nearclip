# Task 4.1 实施计划：集成测试覆盖

**任务**: 为已完成功能添加全面的集成测试覆盖
**优先级**: 🟡 中
**估计时间**: 12-16 小时
**依赖**: 任务 1.1-3.1（所有核心功能已完成）
**风险**: 低
**状态**: ⏳ 进行中
**开始日期**: 2026-01-13

---

## 执行摘要

阶段 1-3 已完成核心功能实现，但缺少全面的集成测试验证。本任务旨在：
1. 补充 BLE 加密传输的集成测试
2. 补充传输层统一（WiFi/BLE 切换）的集成测试
3. 添加端到端配对流程测试
4. 添加性能基准测试

---

## 一、测试范围分析

### 1.1 现有测试覆盖情况

#### ✅ 已有单元测试
| 模块 | 文件 | 测试数量 | 覆盖率 |
|------|------|----------|--------|
| TransportManager | `crates/nearclip-transport/src/manager.rs` | 10 个 | ~80% |
| BleTransport | `crates/nearclip-transport/src/ble.rs` | ? | ? |
| PairingManager | `crates/nearclip-device/src/pairing.rs` | ? | ? |
| Encryption | `crates/nearclip-crypto/src/` | ? | ? |

#### ⚠️ 缺失的测试
1. **BLE 加密传输集成测试** (优先级: 高)
   - 端到端加密/解密流程
   - 加密失败场景（密钥不匹配）
   - 性能开销验证（加密开销 < 10%）

2. **传输层故障转移测试** (优先级: 高)
   - WiFi → BLE 自动切换
   - 主通道失败时的 failover
   - 多设备并发场景

3. **配对流程集成测试** (优先级: 中)
   - QR 码生成/扫描端到端流程
   - ECDH 密钥交换验证
   - 配对拒绝场景

4. **性能基准测试** (优先级: 中)
   - 通道选择延迟
   - 加密/解密吞吐量
   - 100 设备并发性能

---

## 二、测试实施计划

### Step 1: BLE 加密传输集成测试 (4-5 小时)

#### 目标
验证 BLE 传输的端到端加密功能正常工作。

#### 测试文件
新建: `crates/nearclip-transport/tests/integration/ble_encryption_test.rs`

#### 测试用例设计

##### Test 1.1: 端到端加密/解密
```rust
#[tokio::test]
async fn test_ble_encryption_roundtrip() {
    // 1. 创建两个设备的 ECDH 密钥对
    let device_a_keypair = EcdhKeyPair::generate();
    let device_b_keypair = EcdhKeyPair::generate();

    // 2. 计算共享密钥
    let shared_secret_a = device_a_keypair.compute_shared_secret(device_b_keypair.public_key())?;
    let shared_secret_b = device_b_keypair.compute_shared_secret(device_a_keypair.public_key())?;
    assert_eq!(shared_secret_a, shared_secret_b);

    // 3. 创建加密的 BLE 传输（使用 Mock）
    let transport_a = MockBleTransport::new_with_encryption(shared_secret_a);
    let transport_b = MockBleTransport::new_with_encryption(shared_secret_b);

    // 4. 发送加密消息
    let msg = Message::Clipboard(ClipboardData::Text("secret message".to_string()));
    transport_a.send(&msg).await?;

    // 5. 接收并解密消息
    let received_msg = transport_b.receive().await?;
    assert_eq!(msg, received_msg);
}
```

##### Test 1.2: 密钥不匹配场景
```rust
#[tokio::test]
async fn test_ble_encryption_key_mismatch() {
    // 1. 使用不同的密钥
    let shared_secret_a = EcdhKeyPair::generate().compute_shared_secret(...)?;
    let shared_secret_b = EcdhKeyPair::generate().compute_shared_secret(...)?; // 不同密钥

    // 2. 创建传输
    let transport_a = MockBleTransport::new_with_encryption(shared_secret_a);
    let transport_b = MockBleTransport::new_with_encryption(shared_secret_b);

    // 3. 发送消息
    let msg = Message::Clipboard(ClipboardData::Text("test".to_string()));
    transport_a.send(&msg).await?;

    // 4. 接收应该失败（解密错误）
    let result = transport_b.receive().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TransportError::DecryptionFailed));
}
```

##### Test 1.3: 加密性能开销
```rust
#[tokio::test]
async fn test_ble_encryption_performance_overhead() {
    // 1. 创建加密和非加密传输
    let transport_encrypted = MockBleTransport::new_with_encryption(shared_secret);
    let transport_plain = MockBleTransport::new_without_encryption();

    // 2. 准备测试消息（10 KB 数据）
    let msg = Message::Clipboard(ClipboardData::Text("x".repeat(10_000)));

    // 3. 测试加密传输时间
    let start = Instant::now();
    for _ in 0..100 {
        transport_encrypted.send(&msg).await?;
    }
    let encrypted_duration = start.elapsed();

    // 4. 测试非加密传输时间
    let start = Instant::now();
    for _ in 0..100 {
        transport_plain.send(&msg).await?;
    }
    let plain_duration = start.elapsed();

    // 5. 验证加密开销 < 10%
    let overhead = (encrypted_duration.as_millis() - plain_duration.as_millis()) as f64
        / plain_duration.as_millis() as f64;
    assert!(overhead < 0.10, "Encryption overhead: {:.2}%", overhead * 100.0);
}
```

#### 实施步骤
1. **创建 MockBleTransport** (2 小时)
   - 实现 `Transport` trait
   - 模拟 BLE 分块/重组逻辑
   - 支持加密/非加密模式

2. **实现测试用例** (2 小时)
   - Test 1.1: 端到端加密
   - Test 1.2: 密钥不匹配
   - Test 1.3: 性能开销

3. **验证和修复** (1 小时)
   - 运行测试，确保全部通过
   - 修复发现的问题

---

### Step 2: 传输层故障转移测试 (3-4 小时)

#### 目标
验证 `TransportManager` 的故障转移和无缝切换功能。

#### 测试文件
新建: `crates/nearclip-transport/tests/integration/failover_test.rs`

#### 测试用例设计

##### Test 2.1: WiFi 失败时自动切换到 BLE
```rust
#[tokio::test]
async fn test_failover_wifi_to_ble() {
    let manager = TransportManager::new(TransportManagerConfig {
        failover_on_error: true,
        ..Default::default()
    });

    // 1. 添加 WiFi（会失败）和 BLE（正常）传输
    let wifi_transport = MockTransport::new_failing(Channel::Wifi);
    let ble_transport = MockTransport::new_connected(Channel::Ble);

    manager.add_transport("device_1", wifi_transport).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // 2. 发送消息
    let msg = Message::Ping;
    let result = manager.send_to_device("device_1", &msg).await;

    // 3. 验证成功（通过 BLE）
    assert!(result.is_ok());
    assert_eq!(ble_transport.sent_messages().len(), 1);
}
```

##### Test 2.2: 禁用故障转移时不切换
```rust
#[tokio::test]
async fn test_no_failover_when_disabled() {
    let manager = TransportManager::new(TransportManagerConfig {
        failover_on_error: false, // 禁用
        ..Default::default()
    });

    // 1. WiFi 失败
    let wifi_transport = MockTransport::new_failing(Channel::Wifi);
    manager.add_transport("device_1", wifi_transport).await;

    // 2. BLE 正常
    let ble_transport = MockTransport::new_connected(Channel::Ble);
    manager.add_transport("device_1", ble_transport.clone()).await;

    // 3. 发送消息应该失败（不尝试 BLE）
    let result = manager.send_to_device("device_1", &Message::Ping).await;
    assert!(result.is_err());
    assert_eq!(ble_transport.sent_messages().len(), 0); // BLE 未使用
}
```

##### Test 2.3: 无缝切换（WiFi 断开 → BLE 接管）
```rust
#[tokio::test]
async fn test_seamless_switch_on_disconnect() {
    let manager = TransportManager::new_default();

    // 1. 初始：WiFi 和 BLE 都连接
    let wifi_transport = MockTransport::new_connected(Channel::Wifi);
    let ble_transport = MockTransport::new_connected(Channel::Ble);

    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // 2. 第一次发送：使用 WiFi
    manager.send_to_device("device_1", &Message::Ping).await?;
    assert_eq!(wifi_transport.sent_messages().len(), 1);

    // 3. WiFi 断开
    wifi_transport.disconnect();

    // 4. 第二次发送：自动使用 BLE
    manager.send_to_device("device_1", &Message::Ping).await?;
    assert_eq!(ble_transport.sent_messages().len(), 1);
}
```

#### 实施步骤
1. **扩展 MockTransport** (1 小时)
   - 添加失败模式（`new_failing()`）
   - 添加消息记录（`sent_messages()`）
   - 支持动态断开（`disconnect()`）

2. **实现测试用例** (2 小时)
   - Test 2.1: WiFi → BLE failover
   - Test 2.2: 禁用 failover
   - Test 2.3: 无缝切换

3. **验证** (1 小时)

---

### Step 3: 配对流程端到端测试 (3-4 小时)

#### 目标
验证完整的 QR 码配对流程（FFI 层 → Rust 层）。

#### 测试文件
新建: `crates/nearclip-ffi/tests/integration/pairing_test.rs`

#### 测试用例设计

##### Test 3.1: QR 码配对端到端流程
```rust
#[tokio::test]
async fn test_qr_code_pairing_e2e() {
    // 1. 创建两个 FfiNearClipManager 实例（模拟两个设备）
    let manager_a = FfiNearClipManager::new("device_a".to_string(), MockBleManager::new());
    let manager_b = FfiNearClipManager::new("device_b".to_string(), MockBleManager::new());

    // 2. Device A 生成 QR 码
    let qr_code = manager_a.generate_qr_code()?;
    assert!(qr_code.contains("device_id"));
    assert!(qr_code.contains("public_key"));

    // 3. Device B 扫描 QR 码配对
    let paired_device = manager_b.pair_with_qr_code(qr_code).await?;
    assert_eq!(paired_device.device_id, "device_a");

    // 4. 验证共享密钥已存储（两端）
    let secret_b = manager_b.get_shared_secret("device_a")?;
    assert!(secret_b.is_some());

    // 5. Device B 生成 QR 码给 A 扫描（双向配对）
    let qr_code_b = manager_b.generate_qr_code()?;
    let paired_device_a = manager_a.pair_with_qr_code(qr_code_b).await?;
    assert_eq!(paired_device_a.device_id, "device_b");

    // 6. 验证两端共享密钥一致
    let secret_a = manager_a.get_shared_secret("device_b")?;
    assert_eq!(secret_a, secret_b);
}
```

##### Test 3.2: 密钥交换验证
```rust
#[tokio::test]
async fn test_ecdh_key_exchange() {
    let manager_a = FfiNearClipManager::new(...);
    let manager_b = FfiNearClipManager::new(...);

    // 1. A 生成 QR 码
    let qr_a = manager_a.generate_qr_code()?;
    let pairing_data_a: PairingData = serde_json::from_str(&qr_a)?;

    // 2. B 扫描并配对
    manager_b.pair_with_qr_code(qr_a).await?;

    // 3. B 生成 QR 码
    let qr_b = manager_b.generate_qr_code()?;
    let pairing_data_b: PairingData = serde_json::from_str(&qr_b)?;

    // 4. A 扫描并配对
    manager_a.pair_with_qr_code(qr_b).await?;

    // 5. 手动验证 ECDH 共享密钥
    let keypair_a = manager_a.get_local_keypair();
    let keypair_b = manager_b.get_local_keypair();

    let secret_a_computed = keypair_a.compute_shared_secret(&pairing_data_b.public_key)?;
    let secret_b_computed = keypair_b.compute_shared_secret(&pairing_data_a.public_key)?;

    assert_eq!(secret_a_computed, secret_b_computed);

    // 6. 验证与存储的密钥一致
    let secret_a_stored = manager_a.get_shared_secret("device_b")?.unwrap();
    assert_eq!(secret_a_stored, secret_a_computed);
}
```

#### 实施步骤
1. **创建测试辅助工具** (1 小时)
   - `MockBleManager` 实现
   - 辅助函数：`assert_pairing_success()`

2. **实现测试用例** (2 小时)
   - Test 3.1: 端到端配对
   - Test 3.2: 密钥交换验证

3. **验证** (1 小时)

---

### Step 4: 性能基准测试 (2-3 小时)

#### 目标
验证核心功能的性能指标。

#### 测试文件
新建: `crates/nearclip-transport/benches/transport_bench.rs`

#### 基准测试设计

##### Bench 4.1: 通道选择延迟
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_channel_selection(c: &mut Criterion) {
    let manager = TransportManager::new_default();

    // 添加 10 个设备，每个有 WiFi + BLE
    for i in 0..10 {
        let device_id = format!("device_{}", i);
        manager.add_transport(&device_id, MockTransport::new_wifi());
        manager.add_transport(&device_id, MockTransport::new_ble());
    }

    c.bench_function("get_best_transport", |b| {
        b.iter(|| {
            manager.get_best_transport(black_box("device_5"))
        });
    });
}

criterion_group!(benches, bench_channel_selection);
criterion_main!(benches);
```

**验收标准**: 通道选择 < 1ms

##### Bench 4.2: 加密/解密吞吐量
```rust
fn bench_encryption_throughput(c: &mut Criterion) {
    let shared_secret = [0u8; 32]; // 测试密钥
    let cipher = Aes256Gcm::new_from_slice(&shared_secret).unwrap();

    // 测试不同大小的数据
    for size in [1024, 10_240, 102_400] { // 1KB, 10KB, 100KB
        let data = vec![0u8; size];

        c.bench_function(&format!("encrypt_{}KB", size / 1024), |b| {
            b.iter(|| {
                encrypt_message(black_box(&cipher), black_box(&data))
            });
        });

        c.bench_function(&format!("decrypt_{}KB", size / 1024), |b| {
            let encrypted = encrypt_message(&cipher, &data);
            b.iter(|| {
                decrypt_message(black_box(&cipher), black_box(&encrypted))
            });
        });
    }
}
```

**验收标准**:
- 1KB: < 100 μs
- 10KB: < 500 μs
- 100KB: < 3 ms

##### Bench 4.3: 多设备并发性能
```rust
fn bench_concurrent_devices(c: &mut Criterion) {
    let manager = TransportManager::new_default();

    // 添加 100 个设备
    for i in 0..100 {
        manager.add_transport(&format!("device_{}", i), MockTransport::new_wifi());
    }

    c.bench_function("broadcast_100_devices", |b| {
        b.iter(|| {
            manager.broadcast(black_box(&Message::Ping))
        });
    });
}
```

**验收标准**: 100 设备广播 < 100ms

#### 实施步骤
1. **设置 Criterion.rs** (30 分钟)
   - 添加 `Cargo.toml` 依赖
   - 创建 `benches/` 目录

2. **实现基准测试** (1.5 小时)
   - Bench 4.1: 通道选择
   - Bench 4.2: 加密吞吐量
   - Bench 4.3: 并发性能

3. **运行并分析结果** (1 小时)
   - `cargo bench`
   - 生成性能报告
   - 识别优化点

---

## 三、测试基础设施

### 3.1 需要创建的 Mock 组件

#### MockBleTransport
```rust
// crates/nearclip-transport/tests/common/mock_ble_transport.rs

pub struct MockBleTransport {
    encryption: Option<Aes256Gcm>,
    sent_messages: Arc<Mutex<Vec<Message>>>,
    received_messages: Arc<Mutex<Vec<Message>>>,
    is_connected: AtomicBool,
    channel: Channel,
}

impl MockBleTransport {
    pub fn new_with_encryption(shared_secret: SharedSecret) -> Self { ... }
    pub fn new_without_encryption() -> Self { ... }
    pub fn sent_messages(&self) -> Vec<Message> { ... }
    pub fn simulate_receive(&self, msg: Message) { ... }
}

#[async_trait]
impl Transport for MockBleTransport {
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        // 模拟加密 + 分块 + 发送
        let serialized = bincode::serialize(msg)?;
        let encrypted = if let Some(cipher) = &self.encryption {
            encrypt(cipher, &serialized)?
        } else {
            serialized
        };

        self.sent_messages.lock().unwrap().push(msg.clone());
        Ok(())
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        // 从队列取出消息 + 解密
        ...
    }

    fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Relaxed)
    }

    fn channel(&self) -> Channel {
        self.channel
    }
}
```

#### MockTransport (扩展现有的)
```rust
// 添加失败模式
impl MockTransport {
    pub fn new_failing(channel: Channel) -> Arc<Self> {
        // 总是返回发送失败
    }

    pub fn disconnect(&self) {
        self.is_connected.store(false, Ordering::Relaxed);
    }

    pub fn sent_messages(&self) -> Vec<Message> {
        self.sent_messages.lock().unwrap().clone()
    }
}
```

#### MockBleManager (FFI 测试用)
```rust
// crates/nearclip-ffi/tests/common/mock_ble_manager.rs

pub struct MockBleManager {
    scan_started: AtomicBool,
    advertising_started: AtomicBool,
}

impl MockBleManager {
    pub fn new() -> Self { ... }
}

// 实现 FFI BleManager trait
```

### 3.2 测试辅助函数

```rust
// crates/nearclip-transport/tests/common/helpers.rs

/// 创建测试用的 TransportManager
pub fn create_test_manager() -> TransportManager {
    TransportManager::new(TransportManagerConfig::default())
}

/// 创建一对已配对的设备
pub async fn create_paired_devices() -> (FfiNearClipManager, FfiNearClipManager) {
    let manager_a = FfiNearClipManager::new("device_a".to_string(), MockBleManager::new());
    let manager_b = FfiNearClipManager::new("device_b".to_string(), MockBleManager::new());

    // 执行配对流程
    let qr_a = manager_a.generate_qr_code().unwrap();
    manager_b.pair_with_qr_code(qr_a).await.unwrap();

    let qr_b = manager_b.generate_qr_code().unwrap();
    manager_a.pair_with_qr_code(qr_b).await.unwrap();

    (manager_a, manager_b)
}

/// 断言消息相等（忽略时间戳等）
pub fn assert_message_eq(a: &Message, b: &Message) {
    match (a, b) {
        (Message::Clipboard(data_a), Message::Clipboard(data_b)) => {
            assert_eq!(data_a, data_b);
        }
        _ => panic!("Message types don't match"),
    }
}
```

---

## 四、时间分配

| 步骤 | 任务 | 估计时间 |
|------|------|----------|
| Step 1 | BLE 加密传输集成测试 | 4-5 小时 |
| Step 2 | 传输层故障转移测试 | 3-4 小时 |
| Step 3 | 配对流程端到端测试 | 3-4 小时 |
| Step 4 | 性能基准测试 | 2-3 小时 |
| **总计** | | **12-16 小时** |

---

## 五、验收标准

### 功能验收
- [ ] BLE 加密传输集成测试通过（3 个测试）
- [ ] 传输层故障转移测试通过（3 个测试）
- [ ] 配对流程端到端测试通过（2 个测试）
- [ ] 所有测试可通过 `cargo test` 运行

### 性能验收
- [ ] 通道选择延迟 < 1ms
- [ ] 加密开销 < 10%
- [ ] 100 设备并发 < 100ms
- [ ] 加密吞吐量达标（见 Bench 4.2）

### 质量验收
- [ ] 测试覆盖率提升到 > 80%
- [ ] 所有测试文档化（注释清晰）
- [ ] CI/CD 集成（GitHub Actions）

---

## 六、实施顺序

### 第一天（4 小时）
1. 创建测试基础设施（Mock 组件）
2. 实现 Step 1: BLE 加密传输测试

### 第二天（4 小时）
3. 实现 Step 2: 传输层故障转移测试
4. 开始 Step 3: 配对流程测试

### 第三天（4 小时）
5. 完成 Step 3: 配对流程测试
6. 实现 Step 4: 性能基准测试
7. 验证和文档更新

---

## 七、技术亮点

### 7.1 测试策略
- **分层测试**: 单元测试（已有）→ 集成测试（本次）→ 端到端测试（未来）
- **Mock 优先**: 使用 Mock 组件隔离测试，避免真实 BLE 依赖
- **性能基准**: 使用 Criterion.rs 生成专业性能报告

### 7.2 最佳实践
- **测试命名**: `test_[feature]_[scenario]_[expected_result]`
- **断言清晰**: 使用 `assert_eq!` 和自定义断言函数
- **错误场景**: 不仅测试成功路径，也测试失败场景

---

## 八、风险和缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| Mock 组件与实际行为不一致 | 中 | 高 | 参考现有 MockTransport 实现，保持一致性 |
| 异步测试复杂度高 | 中 | 中 | 使用 `tokio::test` 简化异步测试 |
| 性能基准不稳定 | 低 | 中 | 多次运行取平均值，使用 Criterion.rs |
| 测试依赖真实 BLE 硬件 | 低 | 高 | 完全使用 Mock，避免硬件依赖 |

---

## 九、下一步行动

### 立即开始
1. ✅ 创建 Task 4.1 实施计划文档（本文档）
2. ⏳ 创建测试基础设施（Mock 组件）
3. ⏳ 实现 Step 1: BLE 加密传输测试

### 后续任务
- Task 4.2: 端到端平台测试（macOS/Android）
- Task 4.3: CI/CD 集成
- Task 4.4: 文档完善

---

## 附录 A: 测试文件结构

```
crates/
├── nearclip-transport/
│   ├── tests/
│   │   ├── common/
│   │   │   ├── mod.rs
│   │   │   ├── mock_ble_transport.rs
│   │   │   ├── mock_transport.rs
│   │   │   └── helpers.rs
│   │   ├── integration/
│   │   │   ├── ble_encryption_test.rs
│   │   │   └── failover_test.rs
│   ├── benches/
│   │   └── transport_bench.rs
│
├── nearclip-ffi/
│   ├── tests/
│   │   ├── common/
│   │   │   ├── mod.rs
│   │   │   └── mock_ble_manager.rs
│   │   ├── integration/
│   │   │   └── pairing_test.rs
```

---

## 附录 B: Cargo.toml 修改

### nearclip-transport/Cargo.toml
```toml
[dev-dependencies]
tokio = { version = "1.35", features = ["test-util", "macros"] }
criterion = "0.5"

[[bench]]
name = "transport_bench"
harness = false
```

### nearclip-ffi/Cargo.toml
```toml
[dev-dependencies]
tokio = { version = "1.35", features = ["test-util", "macros"] }
```

---

**创建日期**: 2026-01-13
**负责人**: Mouse（与 Claude Code 协作）
**状态**: ⏳ 进行中
