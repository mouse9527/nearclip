# 平台层实现指南

## 概述

本文档为 macOS 和 Android 平台开发者提供设备存储的实现指南，确保密钥安全和数据一致性。

## macOS 实现指南

### 1. 当前问题

**KeychainManager.swift 的问题：**
```swift
// ❌ 当前实现 - 使用 UserDefaults
class KeychainManager {
    private let defaults = UserDefaults.standard  // 不安全！
    private let pairedDevicesKey = "com.nearclip.pairedDevices"
}
```

**问题：**
1. 类名是 `KeychainManager` 但实际用 `UserDefaults`
2. 密钥未加密存储
3. 任何应用都可以读取 UserDefaults
4. 不符合 macOS 安全最佳实践

### 2. 推荐实现

#### 2.1 完整的 KeychainManager 实现

```swift
import Foundation
import Security

/// 设备存储结构
struct StoredDevice: Codable, Identifiable, Equatable {
    let id: String
    let name: String
    let platform: String
    let addedAt: Date

    // 可选：连接信息
    var lastConnectedIP: String?
    var lastConnectedPort: Int?
    var mdnsName: String?
}

/// Keychain 和 UserDefaults 混合存储管理器
class KeychainManager {
    // MARK: - Properties

    /// UserDefaults 用于存储设备元数据
    private let defaults = UserDefaults.standard
    private let pairedDevicesKey = "com.nearclip.pairedDevices"

    /// Keychain 服务名
    private let keychainService = "com.nearclip.keychain"

    // MARK: - 设备元数据管理

    /// 保存设备列表（元数据）
    func savePairedDevices(_ devices: [StoredDevice]) -> Bool {
        guard let data = try? JSONEncoder().encode(devices) else {
            return false
        }
        defaults.set(data, forKey: pairedDevicesKey)
        return defaults.synchronize()
    }

    /// 加载设备列表（元数据）
    func loadPairedDevices() -> [StoredDevice] {
        guard let data = defaults.data(forKey: pairedDevicesKey),
              let devices = try? JSONDecoder().decode([StoredDevice].self, from: data) else {
            return []
        }
        return devices
    }

    /// 添加配对设备
    func addPairedDevice(_ device: StoredDevice) -> Bool {
        var devices = loadPairedDevices()

        // 检查是否已存在
        if let index = devices.firstIndex(where: { $0.id == device.id }) {
            devices[index] = device  // 更新
        } else {
            devices.append(device)  // 添加
        }

        return savePairedDevices(devices)
    }

    /// 移除配对设备
    func removePairedDevice(deviceId: String) -> Bool {
        var devices = loadPairedDevices()
        devices.removeAll { $0.id == deviceId }

        // 同时删除 Keychain 中的密钥
        deleteDeviceKey(deviceId: deviceId)

        return savePairedDevices(devices)
    }

    // MARK: - Keychain 密钥管理

    /// 保存设备公钥到 Keychain
    func saveDeviceKey(deviceId: String, publicKey: Data) -> Bool {
        // 删除旧密钥（如果存在）
        deleteDeviceKey(deviceId: deviceId)

        // 创建 Keychain 查询
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: "device.\(deviceId)",
            kSecValueData as String: publicKey,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock,
            kSecAttrSynchronizable as String: false  // 不同步到 iCloud
        ]

        let status = SecItemAdd(query as CFDictionary, nil)

        if status == errSecSuccess {
            print("✅ Saved key for device: \(deviceId)")
            return true
        } else {
            print("❌ Failed to save key for device: \(deviceId), status: \(status)")
            return false
        }
    }

    /// 从 Keychain 加载设备公钥
    func loadDeviceKey(deviceId: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: "device.\(deviceId)",
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecSuccess, let data = result as? Data {
            print("✅ Loaded key for device: \(deviceId)")
            return data
        } else {
            print("❌ Failed to load key for device: \(deviceId), status: \(status)")
            return nil
        }
    }

    /// 从 Keychain 删除设备密钥
    func deleteDeviceKey(deviceId: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: "device.\(deviceId)"
        ]

        let status = SecItemDelete(query as CFDictionary)

        if status == errSecSuccess || status == errSecItemNotFound {
            print("✅ Deleted key for device: \(deviceId)")
        } else {
            print("❌ Failed to delete key for device: \(deviceId), status: \(status)")
        }
    }

    /// 检查设备密钥是否存在
    func deviceKeyExists(deviceId: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: keychainService,
            kSecAttrAccount as String: "device.\(deviceId)",
            kSecReturnData as String: false
        ]

        let status = SecItemCopyMatching(query as CFDictionary, nil)
        return status == errSecSuccess
    }

    // MARK: - 清理

    /// 清除所有数据（用于测试或重置）
    func clearAll() {
        // 清除 UserDefaults
        defaults.removeObject(forKey: pairedDevicesKey)
        defaults.synchronize()

        // 清除所有 Keychain 项
        let devices = loadPairedDevices()
        for device in devices {
            deleteDeviceKey(deviceId: device.id)
        }
    }
}
```

#### 2.2 使用示例

**启动时加载设备：**
```swift
class ConnectionManager {
    private let keychainManager = KeychainManager()
    private var nearClipManager: FfiNearClipManager?

    func initialize() {
        // 1. 创建 NearClip 管理器
        let config = FfiNearClipConfig(
            deviceName: "My Mac",
            deviceId: "",
            wifiEnabled: true,
            bleEnabled: true,
            autoConnect: true,
            connectionTimeoutSecs: 30,
            heartbeatIntervalSecs: 10,
            maxRetries: 3
        )

        nearClipManager = try? FfiNearClipManager(
            config: config,
            callback: self
        )

        // 2. 从存储加载设备列表
        let storedDevices = keychainManager.loadPairedDevices()
        print("📱 Loaded \(storedDevices.count) devices from storage")

        // 3. 添加到 Rust 层
        for device in storedDevices {
            let ffiDevice = FfiDeviceInfo(
                id: device.id,
                name: device.name,
                platform: DevicePlatform(rawValue: device.platform) ?? .unknown,
                status: .disconnected
            )
            nearClipManager?.addPairedDevice(device: ffiDevice)
            print("  ✅ Added device: \(device.name)")
        }

        // 4. 启动管理器
        try? nearClipManager?.start()
        print("🚀 NearClip manager started")
    }
}
```

**配对新设备：**
```swift
extension ConnectionManager: FfiNearClipCallback {
    func onDeviceConnected(device: FfiDeviceInfo) {
        print("🔗 Device connected: \(device.name)")

        // 1. 保存设备元数据
        let storedDevice = StoredDevice(
            id: device.id,
            name: device.name,
            platform: device.platform.rawValue,
            addedAt: Date()
        )

        if keychainManager.addPairedDevice(storedDevice) {
            print("  ✅ Saved device metadata")
        } else {
            print("  ❌ Failed to save device metadata")
        }

        // 2. 保存设备公钥（如果有）
        if let publicKey = getPublicKeyForDevice(device.id) {
            if keychainManager.saveDeviceKey(deviceId: device.id, publicKey: publicKey) {
                print("  ✅ Saved device key to Keychain")
            } else {
                print("  ❌ Failed to save device key")
            }
        }

        // 3. 通知 Rust 层（如果还没添加）
        nearClipManager?.addPairedDevice(device: device)
    }

    func onDeviceDisconnected(deviceId: String) {
        print("🔌 Device disconnected: \(deviceId)")
    }

    func onDeviceUnpaired(deviceId: String) {
        print("❌ Device unpaired: \(deviceId)")

        // 从存储移除
        if keychainManager.removePairedDevice(deviceId: deviceId) {
            print("  ✅ Removed device from storage")
        }

        // 从 Rust 层移除
        nearClipManager?.removePairedDevice(deviceId: deviceId)
    }

    // ... 其他回调方法
}
```

**移除设备：**
```swift
func unpairDevice(deviceId: String) {
    // 1. 从 Rust 层移除
    nearClipManager?.removePairedDevice(deviceId: deviceId)

    // 2. 从存储移除（包括 Keychain）
    if keychainManager.removePairedDevice(deviceId: deviceId) {
        print("✅ Device removed: \(deviceId)")
    } else {
        print("❌ Failed to remove device: \(deviceId)")
    }
}
```

### 3. 测试

```swift
import XCTest

class KeychainManagerTests: XCTestCase {
    var manager: KeychainManager!

    override func setUp() {
        super.setUp()
        manager = KeychainManager()
        manager.clearAll()  // 清理测试环境
    }

    override func tearDown() {
        manager.clearAll()
        super.tearDown()
    }

    func testSaveAndLoadDevices() {
        let device = StoredDevice(
            id: "test-1",
            name: "Test Device",
            platform: "macOS",
            addedAt: Date()
        )

        XCTAssertTrue(manager.addPairedDevice(device))

        let devices = manager.loadPairedDevices()
        XCTAssertEqual(devices.count, 1)
        XCTAssertEqual(devices.first?.id, "test-1")
    }

    func testSaveAndLoadKey() {
        let deviceId = "test-device"
        let publicKey = Data(repeating: 0x04, count: 65)

        XCTAssertTrue(manager.saveDeviceKey(deviceId: deviceId, publicKey: publicKey))

        let loadedKey = manager.loadDeviceKey(deviceId: deviceId)
        XCTAssertNotNil(loadedKey)
        XCTAssertEqual(loadedKey, publicKey)
    }

    func testRemoveDevice() {
        let device = StoredDevice(
            id: "test-1",
            name: "Test Device",
            platform: "macOS",
            addedAt: Date()
        )
        let publicKey = Data(repeating: 0x04, count: 65)

        manager.addPairedDevice(device)
        manager.saveDeviceKey(deviceId: device.id, publicKey: publicKey)

        XCTAssertTrue(manager.removePairedDevice(deviceId: device.id))

        let devices = manager.loadPairedDevices()
        XCTAssertEqual(devices.count, 0)

        let loadedKey = manager.loadDeviceKey(deviceId: device.id)
        XCTAssertNil(loadedKey)
    }
}
```

---

## Android 实现指南

### 1. 当前实现（已经很好）

**SecureStorage.kt 的优势：**
```kotlin
class SecureStorage(private val context: Context) {
    // ✅ 使用 EncryptedSharedPreferences
    // ✅ 主密钥存储在 Android Keystore
    // ✅ AES-256-GCM 加密
    // ✅ 密钥和元数据分离存储
}
```

### 2. 推荐保持当前实现

Android 的 `SecureStorage` 实现已经符合最佳实践，建议保持不变。

### 3. 使用示例

**启动时加载设备：**
```kotlin
class ConnectionManager(private val context: Context) {
    private val secureStorage = SecureStorage(context)
    private var nearClipManager: FfiNearClipManager? = null

    fun initialize() {
        // 1. 创建 NearClip 管理器
        val config = FfiNearClipConfig(
            deviceName = "My Android",
            deviceId = "",
            wifiEnabled = true,
            bleEnabled = true,
            autoConnect = true,
            connectionTimeoutSecs = 30u,
            heartbeatIntervalSecs = 10u,
            maxRetries = 3u
        )

        nearClipManager = FfiNearClipManager(config, this)

        // 2. 从存储加载设备列表
        val storedDevices = secureStorage.loadPairedDevices()
        Log.d(TAG, "📱 Loaded ${storedDevices.size} devices from storage")

        // 3. 添加到 Rust 层
        storedDevices.forEach { device ->
            nearClipManager?.addPairedDevice(device)
            Log.d(TAG, "  ✅ Added device: ${device.name}")
        }

        // 4. 启动管理器
        nearClipManager?.start()
        Log.d(TAG, "🚀 NearClip manager started")
    }

    companion object {
        private const val TAG = "ConnectionManager"
    }
}
```

**配对新设备：**
```kotlin
class ConnectionManager(
    private val context: Context
) : FfiNearClipCallback {

    override fun onDeviceConnected(device: FfiDeviceInfo) {
        Log.d(TAG, "🔗 Device connected: ${device.name}")

        // 1. 保存设备信息
        val devices = secureStorage.loadPairedDevices().toMutableList()

        // 检查是否已存在
        val existingIndex = devices.indexOfFirst { it.id == device.id }
        if (existingIndex >= 0) {
            devices[existingIndex] = device  // 更新
        } else {
            devices.add(device)  // 添加
        }

        secureStorage.savePairedDevices(devices)
        Log.d(TAG, "  ✅ Saved device metadata")

        // 2. 保存设备公钥（如果有）
        getPublicKeyForDevice(device.id)?.let { publicKey ->
            secureStorage.saveDeviceKeys(device.id, publicKey)
            Log.d(TAG, "  ✅ Saved device key")
        }

        // 3. 通知 Rust 层（如果还没添加）
        nearClipManager?.addPairedDevice(device)
    }

    override fun onDeviceDisconnected(deviceId: String) {
        Log.d(TAG, "🔌 Device disconnected: $deviceId")
    }

    override fun onDeviceUnpaired(deviceId: String) {
        Log.d(TAG, "❌ Device unpaired: $deviceId")

        // 从存储移除
        val devices = secureStorage.loadPairedDevices()
            .filter { it.id != deviceId }
        secureStorage.savePairedDevices(devices)

        // 删除密钥
        secureStorage.deleteDeviceKeys(deviceId)
        Log.d(TAG, "  ✅ Removed device from storage")

        // 从 Rust 层移除
        nearClipManager?.removePairedDevice(deviceId)
    }

    // ... 其他回调方法
}
```

**移除设备：**
```kotlin
fun unpairDevice(deviceId: String) {
    // 1. 从 Rust 层移除
    nearClipManager?.removePairedDevice(deviceId)

    // 2. 从存储移除
    val devices = secureStorage.loadPairedDevices()
        .filter { it.id != deviceId }
    secureStorage.savePairedDevices(devices)

    // 3. 删除密钥
    secureStorage.deleteDeviceKeys(deviceId)

    Log.d(TAG, "✅ Device removed: $deviceId")
}
```

### 4. 测试

```kotlin
@RunWith(AndroidJUnit4::class)
class SecureStorageTest {
    private lateinit var context: Context
    private lateinit var secureStorage: SecureStorage

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        secureStorage = SecureStorage(context)
        secureStorage.clearAll()  // 清理测试环境
    }

    @After
    fun tearDown() {
        secureStorage.clearAll()
    }

    @Test
    fun testSaveAndLoadDevices() {
        val device = FfiDeviceInfo(
            id = "test-1",
            name = "Test Device",
            platform = DevicePlatform.ANDROID,
            status = DeviceStatus.DISCONNECTED
        )

        secureStorage.savePairedDevices(listOf(device))

        val devices = secureStorage.loadPairedDevices()
        assertEquals(1, devices.size)
        assertEquals("test-1", devices.first().id)
    }

    @Test
    fun testSaveAndLoadKeys() {
        val deviceId = "test-device"
        val publicKey = ByteArray(65) { 0x04 }

        secureStorage.saveDeviceKeys(deviceId, publicKey)

        val (loadedPublicKey, _) = secureStorage.loadDeviceKeys(deviceId)
        assertNotNull(loadedPublicKey)
        assertArrayEquals(publicKey, loadedPublicKey)
    }

    @Test
    fun testRemoveDevice() {
        val device = FfiDeviceInfo(
            id = "test-1",
            name = "Test Device",
            platform = DevicePlatform.ANDROID,
            status = DeviceStatus.DISCONNECTED
        )
        val publicKey = ByteArray(65) { 0x04 }

        secureStorage.savePairedDevices(listOf(device))
        secureStorage.saveDeviceKeys(device.id, publicKey)

        // 移除设备
        secureStorage.savePairedDevices(emptyList())
        secureStorage.deleteDeviceKeys(device.id)

        val devices = secureStorage.loadPairedDevices()
        assertEquals(0, devices.size)

        val (loadedKey, _) = secureStorage.loadDeviceKeys(device.id)
        assertNull(loadedKey)
    }
}
```

---

## 通用最佳实践

### 1. 启动流程

**标准启动顺序：**
1. 创建 `FfiNearClipManager`
2. 从平台存储加载设备列表
3. 对每个设备调用 `addPairedDevice()`
4. 调用 `start()` 启动管理器

### 2. 配对流程

**标准配对顺序：**
1. 用户发起配对
2. Rust 层执行密钥交换
3. 回调 `onDeviceConnected()`
4. 平台层保存设备元数据
5. 平台层保存设备密钥
6. 调用 `addPairedDevice()` 通知 Rust 层

### 3. 移除流程

**标准移除顺序：**
1. 调用 `removePairedDevice()` 通知 Rust 层
2. 从平台存储删除设备元数据
3. 从平台存储删除设备密钥

### 4. 错误处理

**常见错误：**
- Keychain/Keystore 访问失败
- 设备未解锁
- 存储空间不足
- 数据格式错误

**处理策略：**
- 记录详细日志
- 向用户显示友好错误消息
- 提供重试机制
- 必要时清理损坏的数据

### 5. 安全检查清单

- [ ] 密钥存储在平台安全存储中（Keychain/Keystore）
- [ ] 不在日志中打印密钥
- [ ] 使用 `kSecAttrAccessibleAfterFirstUnlock` (macOS)
- [ ] 使用 `EncryptedSharedPreferences` (Android)
- [ ] 定期验证存储完整性
- [ ] 提供清除所有数据的方法（用于测试）

---

## 迁移指南

### macOS 迁移步骤

**从 UserDefaults 迁移到 Keychain：**

```swift
func migrateToKeychain() {
    let devices = loadPairedDevices()

    for device in devices {
        // 假设旧版本在 UserDefaults 中存储了密钥
        if let oldKeyData = defaults.data(forKey: "device_key_\(device.id)") {
            // 迁移到 Keychain
            saveDeviceKey(deviceId: device.id, publicKey: oldKeyData)

            // 删除旧数据
            defaults.removeObject(forKey: "device_key_\(device.id)")
        }
    }

    defaults.synchronize()
    print("✅ Migration completed")
}
```

### Android 迁移步骤

**从普通 SharedPreferences 迁移到 EncryptedSharedPreferences：**

```kotlin
fun migrateToEncryptedPrefs() {
    val oldPrefs = context.getSharedPreferences("old_prefs", Context.MODE_PRIVATE)

    // 读取旧数据
    val devicesJson = oldPrefs.getString("paired_devices", null)
    if (devicesJson != null) {
        val devices = Json.decodeFromString<List<FfiDeviceInfo>>(devicesJson)

        // 保存到新存储
        savePairedDevices(devices)

        // 删除旧数据
        oldPrefs.edit().clear().apply()

        Log.d(TAG, "✅ Migration completed")
    }
}
```

---

## 总结

**macOS 开发者需要做的：**
1. ✅ 将密钥从 UserDefaults 迁移到 Keychain
2. ✅ 实现完整的 `KeychainManager`
3. ✅ 在启动时加载设备到 Rust 层
4. ✅ 在配对后保存设备和密钥
5. ✅ 添加单元测试

**Android 开发者需要做的：**
1. ✅ 保持当前的 `SecureStorage` 实现
2. ✅ 确保启动时加载设备到 Rust 层
3. ✅ 确保配对后正确保存设备和密钥
4. ✅ 添加单元测试

**关键原则：**
- 密钥必须使用平台安全存储
- 启动时从平台层加载设备到 Rust 层
- 配对后立即保存到平台存储
- 移除设备时同时清理 Rust 层和平台存储

---

**文档版本**: 1.0
**最后更新**: 2025-12-25
**作者**: Claude Code Agent
