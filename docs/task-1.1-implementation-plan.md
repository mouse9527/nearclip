# Task 1.1 实施计划：简化平台层 BLE 代码

## 目标

将平台层 BLE 代码从业务逻辑层降级为纯硬件抽象层（HAL），删除所有业务逻辑，将代码量从 **2355 行减少到 ~500 行**。

- **macOS**: 1153 行 → ~250 行（减少 78%）
- **Android**: 1202 行 → ~250 行（减少 79%）

---

## 架构原则

### 当前问题
```
┌─────────────────────────────────────┐
│   Platform BLE Manager (1154 行)    │
│  ┌──────────────────────────────┐  │
│  │  Business Logic (业务逻辑)    │  │ ← 问题：业务逻辑在平台层
│  │  - 数据分片/重组             │  │
│  │  - 自动重连                  │  │
│  │  - 连接限流                  │  │
│  │  - 健康监控                  │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Hardware Abstraction        │  │
│  │  - CoreBluetooth/Android BLE │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

### 目标架构
```
┌──────────────────────────────────────┐
│   Rust BleController (业务逻辑)      │
│   - 数据分片/重组                    │
│   - 自动重连                         │
│   - 连接限流                         │
│   - 健康监控                         │
└──────────────────────────────────────┘
            ↓ FFI
┌──────────────────────────────────────┐
│   Platform BLE Manager (~250 行)     │ ← 只做硬件抽象
│   - CoreBluetooth/Android BLE API    │
│   - 状态查询                         │
│   - 错误处理                         │
└──────────────────────────────────────┘
```

---

## 第一步：分析现有代码

### macOS BleManager.swift 分析

| 代码段 | 行数 | 类型 | 操作 |
|--------|------|------|------|
| **业务逻辑（需删除）** | | | |
| DataReassembler 类 | 80 | 数据重组 | ❌ 删除 → Rust Reassembler |
| DataChunker 类 | 70 | 数据分片 | ❌ 删除 → Rust Chunker |
| 连接限流逻辑 | 30 | 业务策略 | ❌ 删除 → Rust BleController |
| 自动重连逻辑 | 30 | 业务策略 | ❌ 删除 → Rust BleController |
| 健康监控逻辑 | 20 | 业务策略 | ❌ 删除 → Rust BleController |
| **小计（删除）** | **~230** | | |
| **硬件抽象（保留）** | | | |
| CBCentralManager 代理 | 150 | HAL | ✅ 保留并简化 |
| CBPeripheralManager 代理 | 180 | HAL | ✅ 保留并简化 |
| CBPeripheral 代理 | 170 | HAL | ✅ 保留并简化 |
| 基本操作 | 150 | API | ✅ 保留 |
| **小计（保留）** | **~650** | | |
| **目标（简化后）** | **~250** | | |

### Android BleManager.kt 分析

| 代码段 | 行数 | 类型 | 操作 |
|--------|------|------|------|
| **业务逻辑（需删除）** | | | |
| DataReassembler 类 | 43 | 数据重组 | ❌ 删除 |
| DataChunker 类 | 90 | 数据分片 | ❌ 删除 |
| 连接限流逻辑 | 25 | 业务策略 | ❌ 删除 |
| 自动重连逻辑 | 30 | 业务策略 | ❌ 删除 |
| 健康监控逻辑 | 20 | 业务策略 | ❌ 删除 |
| **小计（删除）** | **~208** | | |
| **硬件抽象（保留）** | | | |
| BluetoothGatt 回调 | 200 | HAL | ✅ 保留并简化 |
| BluetoothGattServer 回调 | 150 | HAL | ✅ 保留并简化 |
| 基本操作 | 150 | API | ✅ 保留 |
| **小计（保留）** | **~500** | | |
| **目标（简化后）** | **~250** | | |

---

## 第二步：定义简化后的接口

### 平台 BLE Manager 应该提供的接口

```swift
// macOS BleManager.swift（简化版）

protocol BleManagerDelegate: AnyObject {
    // 设备发现
    func bleManager(_ manager: BleManager, didDiscoverDevice peripheralUuid: String, advertisementData: [String: Any], rssi: Int)

    // 连接事件
    func bleManager(_ manager: BleManager, didConnectDevice peripheralUuid: String)
    func bleManager(_ manager: BleManager, didDisconnectDevice peripheralUuid: String, error: Error?)

    // 数据接收（原始字节，不做重组）
    func bleManager(_ manager: BleManager, didReceiveData data: Data, fromDevice peripheralUuid: String, characteristic: String)

    // 错误
    func bleManager(_ manager: BleManager, didFailWithError error: Error)
}

class BleManager {
    // 扫描控制
    func startScanning()
    func stopScanning()

    // 连接管理
    func connect(peripheralUuid: String)
    func disconnect(peripheralUuid: String)

    // GATT 操作（原始）
    func readCharacteristic(peripheralUuid: String, characteristic: String)
    func writeCharacteristic(peripheralUuid: String, characteristic: String, data: Data)
    func subscribeCharacteristic(peripheralUuid: String, characteristic: String)

    // 广播（Peripheral 模式）
    func startAdvertising(serviceData: Data?)
    func stopAdvertising()

    // 状态查询
    func isConnected(peripheralUuid: String) -> Bool
    func getMtu(peripheralUuid: String) -> Int

    // 就这些！不再有其他业务逻辑
}
```

---

## 第三步：Rust 层增强

### 需要在 Rust BleController 中添加的功能

**文件**: `crates/nearclip-transport/src/ble.rs`

当前 Rust 层已经有：
- ✅ `Chunker` - 数据分片
- ✅ `Reassembler` - 数据重组
- ✅ `BleSender` trait - 发送接口

需要添加：
- ❌ 自动重连逻辑
- ❌ 连接限流/节流
- ❌ 健康监控（心跳检测）
- ❌ 设备发现过滤

**实现策略**:
1. 大部分逻辑已经在 `nearclip-ble` crate 中
2. 只需在 BLE Transport 中调用这些功能
3. 通过 FFI 暴露配置接口

---

## 第四步：实施步骤

### Step 1: 备份现有代码（5分钟）
```bash
cp macos/NearClip/Sources/NearClip/BleManager.swift macos/NearClip/Sources/NearClip/BleManager.swift.backup
cp android/app/src/main/java/com/nearclip/service/BleManager.kt android/app/src/main/java/com/nearclip/service/BleManager.kt.backup
```

### Step 2: macOS 简化（2-3小时）

#### 2.1 删除业务逻辑类
- [ ] 删除 `DataReassembler` 类（第 1037-1079 行）
- [ ] 删除 `DataChunker` 类（第 1083-1153 行）
- [ ] 删除连接限流相关属性和方法
- [ ] 删除自动重连逻辑

#### 2.2 简化代理方法
- [ ] `CBCentralManagerDelegate`: 只保留原始回调，不做处理
- [ ] `CBPeripheralDelegate`: 只转发数据，不做重组
- [ ] `CBPeripheralManagerDelegate`: 简化广播逻辑

#### 2.3 清理接口
- [ ] 简化 `BleManagerDelegate` 协议
- [ ] 移除所有业务相关的公开方法
- [ ] 只保留硬件操作方法

### Step 3: Android 简化（2-3小时）

#### 3.1 删除业务逻辑类
- [ ] 删除 `DataReassembler` 类
- [ ] 删除 `DataChunker` 类
- [ ] 删除连接限流相关代码
- [ ] 删除自动重连逻辑

#### 3.2 简化回调
- [ ] `BluetoothGattCallback`: 只转发事件
- [ ] `BluetoothGattServerCallback`: 简化
- [ ] 移除所有数据处理逻辑

#### 3.3 清理接口
- [ ] 简化 `BleManagerCallback` 接口
- [ ] 移除业务相关方法

### Step 4: Rust 层对接（3-4小时）

#### 4.1 在 BLE Transport 中处理所有业务逻辑
- [ ] 使用现有的 `Chunker`/`Reassembler`
- [ ] 添加连接管理逻辑
- [ ] 添加健康监控

#### 4.2 更新 FFI 接口
- [ ] 确保 `BleSender` trait 足够简单
- [ ] 添加配置方法（如果需要）

### Step 5: 集成测试（2-3小时）

#### 5.1 编译测试
- [ ] macOS 编译通过
- [ ] Android 编译通过
- [ ] 无编译警告

#### 5.2 功能测试
- [ ] BLE 扫描正常
- [ ] BLE 连接正常
- [ ] 数据传输正常
- [ ] 重组功能正常（由 Rust 处理）
- [ ] 自动重连正常（由 Rust 处理）

#### 5.3 代码审查
- [ ] macOS BleManager \u003c 300 行
- [ ] Android BleManager \u003c 300 行
- [ ] 无业务逻辑残留
- [ ] 代码清晰易读

---

## 验收标准

### 代码量目标
- ✅ macOS BleManager.swift: **1153 → ~250 行**（减少 ~900 行）
- ✅ Android BleManager.kt: **1202 → ~250 行**（减少 ~950 行）
- ✅ 总共删除 **~1850 行代码**

### 功能完整性
- ✅ BLE 扫描/连接功能正常
- ✅ 数据传输正常（分片/重组由 Rust 处理）
- ✅ 自动重连正常（由 Rust 处理）
- ✅ Peripheral 模式正常（广播）

### 代码质量
- ✅ 无编译警告
- ✅ 平台层只做硬件抽象
- ✅ 所有业务逻辑在 Rust 层
- ✅ 代码清晰，易于维护

---

## 风险与缓解

### 风险 1: 功能回归
**缓解**:
- 先在 Rust 层实现所有业务逻辑
- 保留备份文件
- 逐步删除，每次删除后测试

### 风险 2: FFI 接口不足
**缓解**:
- 提前审查 `BleSender` trait
- 如需添加方法，先在 Rust 层设计

### 风险 3: 平台特性丢失
**缓解**:
- 仔细审查删除的代码
- 确保硬件特定的处理保留（如 MTU 协商）

---

## 时间估算

| 任务 | 估计时间 |
|------|---------|
| 代码分析 | 1 小时 |
| macOS 简化 | 3 小时 |
| Android 简化 | 3 小时 |
| Rust 层对接 | 3 小时 |
| 测试调试 | 3 小时 |
| **总计** | **13 小时** |

---

## 下一步

1. ✅ 创建此计划文档
2. ⏳ 分析需要删除的具体代码段
3. ⏳ 开始 macOS 简化
4. ⏳ 开始 Android 简化
5. ⏳ Rust 层对接
6. ⏳ 集成测试
7. ⏳ 创建 git commit
