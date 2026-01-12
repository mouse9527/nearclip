# NearClip v2 实施清单与时间估算

**文档版本**: 1.0
**创建日期**: 2026-01-12
**整体完成度**: 65%

---

## 快速参考

### 总体估算
- **总工作量**: 62-94 小时
- **全职完成**: 8-12 工作日
- **兼职完成**: 12-15 周（假设每周 6-8 小时）
- **建议开始**: 任务 1.1（简化 macOS BLE）或 任务 1.2（Keychain 安全）

### 关键路径
```
任务 1.3 (配对 FFI) → 任务 2.1 (BLE 加密) → 任务 3.1 (传输统一) → 任务 4.1 (测试)
```

---

## 阶段 1: 基础功能修复（2-3 周）

### ✅ 任务 1.1: 简化平台层 BLE 代码
**时间**: 12-16 小时
**优先级**: 🔴 最高
**风险**: 低
**可并行**: 是

#### 时间分解
| 子任务 | macOS | Android | 总计 |
|--------|-------|---------|------|
| 删除数据分片器 | 1.5h | 2h | 3.5h |
| 删除数据重组器 | 2h | 1h | 3h |
| 删除自动重连逻辑 | 1h | 1h | 2h |
| 删除发现限流逻辑 | 0.5h | 0.5h | 1h |
| 清理导入和依赖 | 0.5h | 0.5h | 1h |
| 测试验证 | 1.5h | 1.5h | 3h |
| **小计** | **7h** | **6.5h** | **13.5h** |

#### 检查清单
**macOS**:
- [ ] 删除 `DataReassembler` 类（第 1030-1079 行）
- [ ] 删除 `DataChunker` 类（第 1082-1153 行）
- [ ] 删除 `pendingDiscoveryConnections` 及相关逻辑
- [ ] 删除 `lastDiscoveryAttempt` 及相关逻辑
- [ ] 删除自动重连相关代码
- [ ] 保留所有 CoreBluetooth API 调用
- [ ] 确认 `BleManagerDelegate` 回调仍可用
- [ ] 编译通过，无警告
- [ ] 行数 < 300

**Android**:
- [ ] 删除 `DataReassembler` 类（第 1044-1087 行）
- [ ] 删除 `DataChunker` 类（第 1089-1179 行）
- [ ] 删除发现连接限流逻辑
- [ ] 删除自动重连逻辑
- [ ] 保留所有 BluetoothGatt API 调用
- [ ] 编译通过，无警告
- [ ] 行数 < 300

#### 验证步骤
1. 编译 macOS/Android 项目
2. 基本 BLE 扫描仍可用
3. 连接/断开仍可用
4. 数据发送由 Rust 层处理

---

### ✅ 任务 1.2: 修复 macOS Keychain 存储
**时间**: 6-8 小时
**优先级**: 🔴 最高（安全）
**风险**: 中
**可并行**: 是

#### 时间分解
| 子任务 | 时间 |
|--------|------|
| 实现 Keychain API（保存/加载/删除） | 4h |
| 添加数据迁移逻辑 | 2h |
| 错误处理和重试机制 | 1h |
| 单元测试 | 1h |
| **总计** | **8h** |

#### 检查清单
- [ ] 删除 `UserDefaults` 依赖
- [ ] 实现 `saveDevice()` 使用 `SecItemAdd`
- [ ] 实现 `loadDevice()` 使用 `SecItemCopyMatching`
- [ ] 实现 `deleteDevice()` 使用 `SecItemDelete`
- [ ] 实现 `migrateFromUserDefaults()` 迁移逻辑
- [ ] 添加 `KeychainError` 错误类型
- [ ] 单元测试覆盖所有操作
- [ ] 集成测试验证 FFI 集成
- [ ] 迁移测试验证旧数据迁移

#### 代码模板
```swift
// 1. 保存设备
func saveDevice(_ device: FfiDeviceInfo) throws {
    let deviceData = try JSONEncoder().encode(device)
    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrAccount as String: device.device_id,
        kSecValueData as String: deviceData,
        kSecAttrService as String: "com.nearclip.devices"
    ]
    SecItemDelete(query as CFDictionary)
    let status = SecItemAdd(query as CFDictionary, nil)
    guard status == errSecSuccess else {
        throw KeychainError.saveFailed(status)
    }
}

// 2. 加载设备
func loadDevice(_ deviceId: String) throws -> FfiDeviceInfo? {
    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrAccount as String: deviceId,
        kSecAttrService as String: "com.nearclip.devices",
        kSecReturnData as String: true
    ]
    var result: AnyObject?
    let status = SecItemCopyMatching(query as CFDictionary, &result)
    guard status == errSecSuccess, let data = result as? Data else {
        return nil
    }
    return try JSONDecoder().decode(FfiDeviceInfo.self, from: data)
}

// 3. 删除设备
func deleteDevice(_ deviceId: String) throws {
    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrAccount as String: deviceId,
        kSecAttrService as String: "com.nearclip.devices"
    ]
    let status = SecItemDelete(query as CFDictionary)
    guard status == errSecSuccess || status == errSecItemNotFound else {
        throw KeychainError.deleteFailed(status)
    }
}
```

---

### ✅ 任务 1.3: 实现双向配对 FFI 集成
**时间**: 8-12 小时
**优先级**: 🔴 最高
**风险**: 中
**依赖**: 无
**阻塞**: 任务 2.1

#### 时间分解
| 子任务 | 时间 |
|--------|------|
| Rust: 实现 `generate_qr_code()` | 2h |
| Rust: 实现 `pair_with_qr_code()` | 2h |
| Rust: 添加配对回调接口 | 1h |
| macOS: 集成 FFI 配对方法 | 2h |
| Android: 集成 FFI 配对方法 | 2h |
| 端到端测试（macOS ↔ Android） | 3h |
| **总计** | **12h** |

#### 检查清单

**Rust FFI** (`crates/nearclip-ffi/src/lib.rs`):
- [ ] 实现 `generate_qr_code()` 方法
- [ ] 实现 `pair_with_qr_code()` 方法
- [ ] 添加 `FfiPairingCallback` 接口
- [ ] 确认 UDL 定义匹配
- [ ] 单元测试验证序列化/反序列化
- [ ] 错误处理完整

**macOS** (`ConnectionManager.swift`):
- [ ] 添加 `startPairing()` 方法调用 FFI
- [ ] 添加 `scanQRCode()` 方法调用 FFI
- [ ] 实现 QR 码显示 UI
- [ ] 删除旧的单向配对逻辑
- [ ] 编译通过

**Android** (`ConnectionManager.kt`):
- [ ] 添加 `startPairing()` 方法调用 FFI
- [ ] 添加 `scanQRCode()` 方法调用 FFI
- [ ] 实现 QR 码扫描功能
- [ ] 删除旧的单向配对逻辑
- [ ] 编译通过

**测试**:
- [ ] macOS 生成 QR 码成功
- [ ] Android 扫描 QR 码配对成功
- [ ] 两端都保存设备信息
- [ ] 配对拒绝流程正常
- [ ] 配对成功率 > 95%

#### 代码模板

**Rust FFI**:
```rust
impl FfiNearClipManager {
    pub fn generate_qr_code(&self) -> Result<String, NearClipError> {
        let pairing_manager = self.inner.pairing_manager()?;
        let pairing_data = pairing_manager.generate_pairing_data()?;
        let qr_string = serde_json::to_string(&pairing_data)?;
        Ok(qr_string)
    }

    pub fn pair_with_qr_code(&self, qr_data: String) -> Result<FfiDeviceInfo, NearClipError> {
        let pairing_manager = self.inner.pairing_manager()?;
        let pairing_data: PairingData = serde_json::from_str(&qr_data)?;
        let device = pairing_manager.pair_with_device(pairing_data).await?;
        Ok(FfiDeviceInfo::from(device))
    }
}
```

---

## 阶段 2: 安全增强（1-2 周）

### ✅ 任务 2.1: 实现 BLE 传输加密
**时间**: 10-14 小时
**优先级**: 🔴 高
**风险**: 高
**依赖**: 任务 1.3
**阻塞**: 任务 3.1

#### 时间分解
| 子任务 | 时间 |
|--------|------|
| 集成加密引擎到 BleController | 4h |
| 配对时密钥交换和派生 | 3h |
| 更新消息协议（添加加密标识） | 2h |
| 性能测试和优化 | 2h |
| 安全测试（密钥验证、解密失败） | 2h |
| **总计** | **13h** |

#### 检查清单

**Rust BLE 控制器** (`crates/nearclip-ble/src/controller.rs`):
- [ ] 添加 `CryptoEngine` 字段
- [ ] 添加 `device_keys: HashMap<String, Vec<u8>>` 存储
- [ ] 实现 `send_encrypted()` 方法
- [ ] 实现 `on_data_received()` 解密逻辑
- [ ] 单元测试覆盖加密/解密

**配对管理器** (`crates/nearclip-device/src/pairing.rs`):
- [ ] 实现 `complete_pairing()` 密钥交换
- [ ] 实现 `derive_key()` HKDF-SHA256 派生
- [ ] 存储设备密钥到安全存储
- [ ] 单元测试验证密钥派生

**协议层** (`crates/nearclip-protocol/src/message.rs`):
- [ ] 定义 `EncryptedMessage` 结构
- [ ] 实现 `Message::encrypt()`
- [ ] 实现 `Message::decrypt()`
- [ ] 单元测试验证序列化

**测试**:
- [ ] 加密数据传输成功
- [ ] 解密数据正确
- [ ] 错误密钥被拒绝
- [ ] 加密开销 < 10%
- [ ] 性能指标达标

#### 代码模板

**BLE 控制器加密集成**:
```rust
pub struct BleController {
    crypto: Arc<CryptoEngine>,
    device_keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    // ... 其他字段
}

impl BleController {
    pub async fn send_encrypted(&self, device_id: &str, data: Vec<u8>) -> Result<()> {
        let key = self.device_keys.read().await
            .get(device_id)
            .ok_or(BleError::NoEncryptionKey)?
            .clone();
        let encrypted = self.crypto.encrypt(&data, &key)?;
        self.send_data(device_id, encrypted).await
    }

    async fn on_data_received(&self, device_id: &str, encrypted_data: Vec<u8>) -> Result<()> {
        let key = self.device_keys.read().await
            .get(device_id)
            .ok_or(BleError::NoEncryptionKey)?
            .clone();
        let data = self.crypto.decrypt(&encrypted_data, &key)?;
        self.handle_plaintext_data(device_id, data).await
    }
}
```

---

## 阶段 3: 传输优化（1-2 周）

### ✅ 任务 3.1: 实现传输层统一
**时间**: 16-20 小时
**优先级**: 🟡 中
**风险**: 高
**依赖**: 任务 2.1

#### 时间分解
| 子任务 | 时间 |
|--------|------|
| 设计 TransportManager 架构 | 4h |
| 实现 WiFi/BLE 统一接口 | 6h |
| 实现通道选择和切换逻辑 | 4h |
| 集成到 FFI 层 | 3h |
| 端到端测试（通道切换） | 3h |
| **总计** | **20h** |

#### 检查清单

**传输管理器** (`crates/nearclip-transport/src/manager.rs`):
- [ ] 定义 `TransportManager` 结构
- [ ] 实现 `select_channel()` 通道选择
- [ ] 实现 `handle_channel_switch()` 无缝切换
- [ ] 实现健康检查和故障转移
- [ ] 单元测试覆盖切换逻辑

**FFI 集成** (`crates/nearclip-ffi/src/lib.rs`):
- [ ] 暴露传输管理接口
- [ ] 添加通道状态回调
- [ ] 更新文档

**测试**:
- [ ] WiFi 可用时优先使用
- [ ] WiFi 断开时自动切换到 BLE
- [ ] 切换延迟 < 1 秒
- [ ] 数据不丢失

#### 代码模板
```rust
pub struct TransportManager {
    wifi: Arc<WifiTransport>,
    ble: Arc<BleTransport>,
    active_transports: Arc<RwLock<HashMap<String, Channel>>>,
}

impl TransportManager {
    pub async fn send(&self, device_id: &str, msg: &Message) -> Result<()> {
        let channel = self.select_channel(device_id).await;
        match channel {
            Channel::Wifi => self.wifi.send(msg).await,
            Channel::Ble => self.ble.send(msg).await,
        }
    }

    async fn select_channel(&self, device_id: &str) -> Channel {
        if self.wifi.is_available(device_id).await {
            Channel::Wifi
        } else {
            Channel::Ble
        }
    }
}
```

---

## 阶段 4: 质量保证（1 周）

### ✅ 任务 4.1: 集成测试覆盖
**时间**: 12-16 小时
**优先级**: 🟡 中
**依赖**: 任务 1-3

#### 时间分解
| 测试类型 | 时间 |
|----------|------|
| 配对流程测试 | 4h |
| 数据传输测试 | 4h |
| 边界情况测试 | 3h |
| 性能测试 | 3h |
| **总计** | **14h** |

#### 检查清单

**配对流程测试**:
- [ ] QR 码生成正确
- [ ] QR 码解析正确
- [ ] 双向配对成功
- [ ] 密钥交换成功
- [ ] 配对拒绝处理

**数据传输测试**:
- [ ] WiFi 传输正确
- [ ] BLE 传输正确
- [ ] 加密数据正确
- [ ] 通道切换正确

**边界情况测试**:
- [ ] 网络中断恢复
- [ ] 设备离线/上线
- [ ] 超时处理
- [ ] 并发连接

**性能测试**:
- [ ] 大文件传输 (> 10MB)
- [ ] 并发设备连接
- [ ] 内存使用 < 50MB
- [ ] CPU 使用 < 10%

---

## 阶段 5: 优化完善（1 周）

### ✅ 任务 5.1: 性能优化
**时间**: 8-10 小时
**优先级**: 🟢 低

#### 优化清单
- [ ] 减少锁竞争（使用细粒度锁）
- [ ] 序列化缓冲区复用
- [ ] 连接池管理
- [ ] BLE 自适应 MTU
- [ ] 内存池复用

### ✅ 任务 5.2: 文档完善
**时间**: 6-8 小时
**优先级**: 🟢 低

#### 文档清单
- [ ] API 文档（Rust doc）
- [ ] 架构图更新
- [ ] 部署指南
- [ ] 故障排查手册
- [ ] 性能调优指南

---

## 总体时间估算汇总

| 阶段 | 任务 | 最小时间 | 最大时间 | 平均时间 |
|------|------|----------|----------|----------|
| **阶段 1** | 1.1 平台简化 | 12h | 16h | 14h |
| | 1.2 Keychain 修复 | 6h | 8h | 7h |
| | 1.3 配对 FFI | 8h | 12h | 10h |
| | **小计** | **26h** | **36h** | **31h** |
| **阶段 2** | 2.1 BLE 加密 | 10h | 14h | 12h |
| | **小计** | **10h** | **14h** | **12h** |
| **阶段 3** | 3.1 传输统一 | 16h | 20h | 18h |
| | **小计** | **16h** | **20h** | **18h** |
| **阶段 4** | 4.1 集成测试 | 12h | 16h | 14h |
| | **小计** | **12h** | **16h** | **14h** |
| **阶段 5** | 5.1 性能优化 | 8h | 10h | 9h |
| | 5.2 文档完善 | 6h | 8h | 7h |
| | **小计** | **14h** | **18h** | **16h** |
| | **总计** | **78h** | **104h** | **91h** |

---

## 建议工作计划

### 方案 A: 全职专注（8-13 周日）
```
周 1-2: 任务 1.1, 1.2 (平台简化 + Keychain)
周 3-4: 任务 1.3 (配对 FFI)
周 5-6: 任务 2.1 (BLE 加密)
周 7-9: 任务 3.1 (传输统一)
周 10-11: 任务 4.1 (测试)
周 12-13: 任务 5.1, 5.2 (优化文档)
```

### 方案 B: 兼职（每周 6-8 小时，12-15 周）
```
第 1-2 周: 任务 1.1 (macOS 简化)
第 3 周: 任务 1.1 (Android 简化)
第 4 周: 任务 1.2 (Keychain)
第 5-6 周: 任务 1.3 (配对 FFI)
第 7-8 周: 任务 2.1 (加密)
第 9-11 周: 任务 3.1 (传输)
第 12-13 周: 任务 4.1 (测试)
第 14-15 周: 任务 5.1, 5.2 (优化)
```

### 方案 C: 快速通道（优先核心功能，6-8 周）
```
周 1: 任务 1.2 (Keychain) - 安全优先
周 2-3: 任务 1.3 (配对 FFI) - 核心功能
周 4-5: 任务 2.1 (加密) - 安全增强
周 6: 任务 4.1 (核心测试)
周 7-8: 任务 1.1 (平台简化) - 代码清理
```

**推荐**: **方案 C**（快速通道）
- 优先解决安全和核心功能问题
- 平台简化可以延后（不影响功能）
- 6-8 周后可发布可用版本

---

## 下周行动计划

### 本周（第 1 周）
**目标**: 完成任务 1.2（Keychain 安全）

**周一** (4h):
- [ ] 实现 Keychain API（保存/加载/删除）
- [ ] 添加错误处理

**周三** (2h):
- [ ] 实现数据迁移逻辑
- [ ] 单元测试

**周五** (2h):
- [ ] 集成测试
- [ ] 提交 PR

### 下周（第 2 周）
**目标**: 开始任务 1.3（配对 FFI）

**周一** (4h):
- [ ] Rust FFI 实现 `generate_qr_code()`
- [ ] Rust FFI 实现 `pair_with_qr_code()`

**周三** (3h):
- [ ] macOS 集成 FFI 配对方法
- [ ] Android 集成 FFI 配对方法

**周五** (3h):
- [ ] 端到端测试
- [ ] 提交 PR

---

## 进度跟踪模板

```markdown
## 周报 - 2026 年第 X 周

### 本周完成
- [ ] 任务 X.X: 描述
  - 实际时间: Xh
  - 状态: ✅ 完成 / ⏳ 进行中 / ❌ 阻塞

### 下周计划
- [ ] 任务 X.X: 描述
  - 预计时间: Xh

### 风险/阻塞
- 描述风险或阻塞问题
- 缓解措施

### 需要帮助
- 列出需要支持的地方
```

---

**文档维护**: 每完成一个子任务，更新进度
**联系人**: Mouse (项目负责人)
