# Task 4.4: 端到端平台测试实施计划

**任务**: 建立 macOS ↔ Android 端到端测试框架
**优先级**: 🟡 中
**估计时间**: 8-12 小时
**依赖**: Task 4.3 (FFI 测试)
**目标**: 验证跨平台剪贴板同步和设备配对功能

---

## 1. 背景

### 1.1 当前状态

根据 Task 4.3 的成果:
- ✅ FFI 层有 57 个单元测试 (100% 通过)
- ✅ FFI 测试覆盖率 60%+
- ⚠️ **缺少**: 跨平台集成测试
- ⚠️ **缺少**: 真实硬件 BLE 测试
- ⚠️ **缺少**: 端到端功能验证

### 1.2 测试挑战

**平台差异**:
- macOS: Swift + Xcode Test
- Android: Kotlin + JUnit + Espresso
- 无法在同一进程中运行

**硬件依赖**:
- BLE 需要真实硬件
- 剪贴板需要系统权限
- 网络需要实际连接

**测试复杂度**:
- 需要两台设备同时运行
- 需要模拟用户交互
- 需要验证异步操作

### 1.3 测试策略

由于端到端测试的复杂性,我们采用**分层测试策略**:

| 层级 | 测试类型 | 工具 | 自动化 |
|------|---------|------|--------|
| L1 | FFI 单元测试 | Rust cargo test | ✅ 已完成 |
| L2 | 平台集成测试 | XCTest / JUnit | ✅ 本任务 |
| L3 | 跨平台手动测试 | 手动测试指南 | ⏳ 后续 |
| L4 | E2E 自动化测试 | 自动化框架 | ❌ 未来任务 |

**本任务聚焦**: L2 平台集成测试

---

## 2. 测试范围

### 2.1 macOS 平台测试

#### Test Group 1: FFI 绑定验证 (5 tests)
```swift
// 验证 Swift 可以正确调用 FFI
func testFfiManagerCreation()
func testFfiGetDeviceId()
func testFfiGenerateQrCode()
func testFfiAddPairedDevice()
func testFfiGetPairedDevices()
```

#### Test Group 2: BLE 硬件接口 (5 tests)
```swift
// 验证 BLE 硬件桥接
func testBleHardwareStartScan()
func testBleHardwareConnect()
func testBleHardwareDisconnect()
func testBleHardwareReadCharacteristic()
func testBleHardwareWriteCharacteristic()
```

#### Test Group 3: 设备存储 (3 tests)
```swift
// 验证 Keychain 存储
func testDeviceStorageSaveDevice()
func testDeviceStorageLoadDevices()
func testDeviceStorageRemoveDevice()
```

#### Test Group 4: 回调机制 (3 tests)
```swift
// 验证回调正确触发
func testCallbackDeviceConnected()
func testCallbackClipboardReceived()
func testCallbackSyncError()
```

**macOS 总计**: 16 tests

### 2.2 Android 平台测试

#### Test Group 1: FFI 绑定验证 (5 tests)
```kotlin
// 验证 Kotlin 可以正确调用 FFI
@Test fun testFfiManagerCreation()
@Test fun testFfiGetDeviceId()
@Test fun testFfiGenerateQrCode()
@Test fun testFfiAddPairedDevice()
@Test fun testFfiGetPairedDevices()
```

#### Test Group 2: BLE 硬件接口 (5 tests)
```kotlin
// 验证 BLE 硬件桥接
@Test fun testBleHardwareStartScan()
@Test fun testBleHardwareConnect()
@Test fun testBleHardwareDisconnect()
@Test fun testBleHardwareReadCharacteristic()
@Test fun testBleHardwareWriteCharacteristic()
```

#### Test Group 3: 设备存储 (3 tests)
```kotlin
// 验证 EncryptedSharedPreferences 存储
@Test fun testDeviceStorageSaveDevice()
@Test fun testDeviceStorageLoadDevices()
@Test fun testDeviceStorageRemoveDevice()
```

#### Test Group 4: 回调机制 (3 tests)
```kotlin
// 验证回调正确触发
@Test fun testCallbackDeviceConnected()
@Test fun testCallbackClipboardReceived()
@Test fun testCallbackSyncError()
```

**Android 总计**: 16 tests

---

## 3. 测试架构

### 3.1 macOS 测试架构

```
macos/NearClip/
├── Tests/
│   └── NearClipTests/
│       ├── FfiBindingTests.swift      # FFI 绑定测试
│       ├── BleHardwareTests.swift     # BLE 硬件测试
│       ├── DeviceStorageTests.swift   # Keychain 存储测试
│       ├── CallbackTests.swift        # 回调机制测试
│       └── TestHelpers.swift          # 测试辅助函数
└── Package.swift                       # 添加测试 target
```

### 3.2 Android 测试架构

```
android/app/src/
├── test/                              # 单元测试
│   └── java/com/nearclip/
│       ├── FfiBindingTest.kt          # FFI 绑定测试
│       ├── BleHardwareTest.kt         # BLE 硬件测试
│       ├── DeviceStorageTest.kt       # 存储测试
│       └── CallbackTest.kt            # 回调测试
└── androidTest/                       # 仪器测试 (需要设备)
    └── java/com/nearclip/
        └── BleIntegrationTest.kt      # BLE 集成测试
```

---

## 4. Mock 策略

由于真实硬件测试复杂,我们使用 **Mock + 真实混合** 策略:

### 4.1 Mock BLE 硬件

**macOS Mock**:
```swift
class MockBleHardware: FfiBleHardware {
    var scanStarted = false
    var connectedDevices = Set<String>()
    var readResults: [String: Data] = [:]

    func startScan() { scanStarted = true }
    func stopScan() { scanStarted = false }
    func connect(peripheralUuid: String) {
        connectedDevices.insert(peripheralUuid)
    }
    // ... 其他方法
}
```

**Android Mock**:
```kotlin
class MockBleHardware : FfiBleHardware {
    var scanStarted = false
    val connectedDevices = mutableSetOf<String>()
    val readResults = mutableMapOf<String, ByteArray>()

    override fun startScan() { scanStarted = true }
    override fun stopScan() { scanStarted = false }
    override fun connect(peripheralUuid: String) {
        connectedDevices.add(peripheralUuid)
    }
    // ... 其他方法
}
```

### 4.2 Mock 设备存储

**macOS**: 使用内存存储代替 Keychain
**Android**: 使用内存存储代替 EncryptedSharedPreferences

### 4.3 真实组件

保留真实的 FFI 调用,确保:
- UniFFI 绑定正确生成
- 类型转换正确
- 内存管理安全

---

## 5. 实施步骤

### Step 1: macOS 测试基础设施 (3 小时)

1. **创建测试 target**
   - 修改 `Package.swift` 添加测试 target
   - 配置测试依赖

2. **实现 Mock 类**
   - `MockBleHardware.swift`
   - `MockDeviceStorage.swift`
   - `MockCallback.swift`

3. **实现 FFI 绑定测试**
   - `FfiBindingTests.swift` (5 tests)

4. **实现其他测试组**
   - `BleHardwareTests.swift` (5 tests)
   - `DeviceStorageTests.swift` (3 tests)
   - `CallbackTests.swift` (3 tests)

### Step 2: Android 测试基础设施 (3 小时)

1. **创建测试目录**
   - `test/java/com/nearclip/`
   - 配置 JUnit

2. **实现 Mock 类**
   - `MockBleHardware.kt`
   - `MockDeviceStorage.kt`
   - `MockCallback.kt`

3. **实现 FFI 绑定测试**
   - `FfiBindingTest.kt` (5 tests)

4. **实现其他测试组**
   - `BleHardwareTest.kt` (5 tests)
   - `DeviceStorageTest.kt` (3 tests)
   - `CallbackTest.kt` (3 tests)

### Step 3: 测试验证和文档 (2 小时)

1. **运行所有测试**
   - macOS: `swift test`
   - Android: `./gradlew test`

2. **修复失败的测试**

3. **创建测试文档**
   - 测试运行指南
   - 已知限制说明

---

## 6. 验收标准

### 6.1 测试数量

- [ ] macOS 测试: 16+ tests
- [ ] Android 测试: 16+ tests
- [ ] 总计: 32+ tests

### 6.2 测试通过率

- [ ] macOS 测试通过率 > 90%
- [ ] Android 测试通过率 > 90%

### 6.3 测试覆盖

- [ ] FFI 绑定验证完成
- [ ] BLE 硬件接口验证完成
- [ ] 设备存储验证完成
- [ ] 回调机制验证完成

### 6.4 文档完整

- [ ] 测试运行指南
- [ ] Mock 使用说明
- [ ] 已知限制文档

---

## 7. 不包含在本任务中

根据测试复杂度和资源限制,以下内容不在本任务范围:

### 7.1 ❌ 真实硬件 BLE 测试
**原因**: 需要两台物理设备,测试环境复杂
**替代**: Mock BLE 硬件接口
**后续**: Task 4.5 手动测试指南

### 7.2 ❌ 端到端自动化测试
**原因**: 需要复杂的测试编排系统
**替代**: 单平台集成测试
**后续**: 专门的 E2E 自动化任务

### 7.3 ❌ UI 自动化测试
**原因**: Compose/SwiftUI 测试复杂
**替代**: 单元测试 + 手动 UI 测试
**后续**: UI 测试专项任务

### 7.4 ❌ 性能压力测试
**原因**: 需要专门的性能测试框架
**替代**: 功能正确性验证
**后续**: 性能测试专项任务

---

## 8. 风险和缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| UniFFI 生成代码变化 | 高 | 使用版本锁定,验证绑定稳定性 |
| Mock 与真实行为差异 | 中 | 关键路径保留真实组件 |
| 平台特定 API 限制 | 中 | 隔离平台相关代码 |
| 测试环境配置复杂 | 低 | 提供详细设置文档 |

---

## 9. 时间估算

| 阶段 | 估计时间 |
|------|----------|
| macOS 测试实现 | 3 小时 |
| Android 测试实现 | 3 小时 |
| 测试调试和修复 | 2 小时 |
| 文档编写 | 1 小时 |
| 缓冲时间 | 2 小时 |
| **总计** | **11 小时** |

---

## 10. 成功指标

完成 Task 4.4 后:

1. **测试数量**: macOS 16+ tests, Android 16+ tests
2. **通过率**: 两个平台 > 90%
3. **覆盖率**: FFI 绑定、BLE 接口、存储、回调全覆盖
4. **文档**: 完整的测试运行指南
5. **自动化**: 可集成到 CI/CD 流程

---

## 11. 后续任务

完成 Task 4.4 后,建议继续:

- **Task 4.5**: CI/CD 集成 (自动运行平台测试)
- **Task 4.6**: 手动测试指南 (真实设备跨平台测试)
- **Task 4.7**: E2E 自动化框架 (长期目标)

---

**创建时间**: 2026-01-14
**预计完成**: 2026-01-15
**依赖任务**: Task 4.3 ✅
**目标**: 建立可自动化的平台集成测试基础

---

## 12. 实施决策

考虑到:
1. **测试复杂度**: 端到端测试需要双设备协调,非常复杂
2. **当前资源**: 单人开发,有限时间
3. **实际价值**: Mock 测试可以覆盖 80% 的集成问题
4. **优先级**: 功能正确性 > 完整集成测试

**决策**:
- ✅ **实施**: 平台集成测试 (Mock 为主)
- ✅ **实施**: FFI 绑定验证
- ⏳ **推迟**: 真实硬件端到端测试 → 手动测试指南
- ⏳ **推迟**: 自动化 E2E 测试 → 未来专项任务

这是一个**务实的平衡决策**,确保测试质量同时控制实施成本。
