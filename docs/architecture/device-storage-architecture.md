# 设备存储架构文档

## 概述

NearClip 的设备存储采用**分层架构**，Rust 层管理运行时状态，平台层负责持久化存储。这种设计确保了密钥的安全性和跨平台的一致性。

## 架构设计

### 1. 分层职责

```
┌─────────────────────────────────────────────────────────┐
│                    平台层 (Swift/Kotlin)                  │
│  ┌──────────────────────────────────────────────────┐   │
│  │  持久化存储                                        │   │
│  │  - macOS: Keychain (密钥) + UserDefaults (元数据)  │   │
│  │  - Android: Keystore (密钥) + EncryptedPrefs (元数据)│   │
│  └──────────────────────────────────────────────────┘   │
│                          ↕ FFI                          │
├─────────────────────────────────────────────────────────┤
│                    Rust 层 (Core)                        │
│  ┌──────────────────────────────────────────────────┐   │
│  │  运行时状态管理                                     │   │
│  │  - NearClipManager: 设备列表                       │   │
│  │  - TransportManager: 连接状态                      │   │
│  │  - DeviceInfo: 运行时信息                          │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### 2. 数据结构

#### 2.1 Rust 层数据结构

**DeviceInfo** (运行时信息)
```rust
pub struct DeviceInfo {
    id: String,                    // 设备唯一标识符
    name: String,                  // 设备显示名称
    platform: DevicePlatform,      // 设备平台
    status: DeviceStatus,          // 连接状态（运行时）
    last_seen: Option<Instant>,    // 最后活动时间（运行时）
}
```

**PairedDevice** (配对信息)
```rust
pub struct PairedDevice {
    pub device_id: String,                 // 设备 ID
    pub public_key_bytes: Vec<u8>,         // 对方公钥（65字节）
    pub connection_info: Option<ConnectionInfo>, // 连接信息
    pub shared_secret_hash: String,        // 共享密钥哈希（SHA256）
    pub paired_at: u64,                    // 配对时间戳
}

pub struct ConnectionInfo {
    pub ip: Option<String>,       // IP 地址
    pub port: Option<u16>,        // 端口号
    pub mdns_name: Option<String>, // mDNS 服务名
}
```

**安全设计：**
- ❌ **不存储完整共享密钥**
- ✅ **只存储 SHA256 哈希用于验证**
- ✅ **实际加密通信时重新计算共享密钥**

#### 2.2 FFI 层数据结构

**FfiDeviceInfo** (跨语言传输)
```rust
pub struct FfiDeviceInfo {
    pub id: String,
    pub name: String,
    pub platform: DevicePlatform,
    pub status: DeviceStatus,
}
```

#### 2.3 平台层数据结构

**macOS (StoredDevice)**
```swift
struct StoredDevice: Codable {
    let id: String
    let name: String
    let platform: String
    let addedAt: Date
}
```

**Android (FfiDeviceInfo + 密钥)**
```kotlin
data class FfiDeviceInfo(
    val id: String,
    val name: String,
    val platform: DevicePlatform,
    val status: DeviceStatus
)

// 密钥单独存储在 EncryptedSharedPreferences
```

### 3. 存储实现

#### 3.1 Rust 层存储

**FileDeviceStore** (可选，用于测试或桌面应用)
- **位置**: `crates/nearclip-crypto/src/device_store.rs`
- **格式**: JSON 文件
- **路径**: `paired_devices.json`
- **特性**:
  - 原子写入（临时文件 + 重命名）
  - 版本控制
  - 完整的 CRUD 操作

**使用场景：**
- 桌面应用（macOS/Linux/Windows）
- 测试环境
- 不需要平台安全存储的场景

#### 3.2 macOS 层存储

**当前实现** (`KeychainManager.swift`)
```swift
class KeychainManager {
    private let defaults = UserDefaults.standard
    private let pairedDevicesKey = "com.nearclip.pairedDevices"

    func savePairedDevices(_ devices: [StoredDevice]) -> Bool
    func loadPairedDevices() -> [StoredDevice]
    func addPairedDevice(_ device: StoredDevice) -> Bool
    func removePairedDevice(deviceId: String) -> Bool
}
```

**问题：**
- ❌ 当前使用 `UserDefaults`，不够安全
- ❌ 密钥未存储在 Keychain
- ❌ 缺少 `connection_info` 和 `paired_at`

**推荐实现：**
```swift
class KeychainManager {
    // 元数据存储在 UserDefaults
    private let defaults = UserDefaults.standard

    // 密钥存储在 Keychain
    func saveDeviceKey(deviceId: String, publicKey: Data) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "nearclip.device.\(deviceId)",
            kSecValueData as String: publicKey,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock
        ]
        SecItemDelete(query as CFDictionary)
        return SecItemAdd(query as CFDictionary, nil) == errSecSuccess
    }

    func loadDeviceKey(deviceId: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "nearclip.device.\(deviceId)",
            kSecReturnData as String: true
        ]
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        return status == errSecSuccess ? (result as? Data) : nil
    }
}
```

#### 3.3 Android 层存储

**当前实现** (`SecureStorage.kt`)
```kotlin
class SecureStorage(private val context: Context) {
    private val masterKey: MasterKey by lazy {
        MasterKey.Builder(context)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()
    }

    private val encryptedPrefs: SharedPreferences by lazy {
        EncryptedSharedPreferences.create(
            context,
            "nearclip_secure_prefs",
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
    }

    fun savePairedDevices(devices: List<FfiDeviceInfo>)
    fun loadPairedDevices(): List<FfiDeviceInfo>
    fun saveDeviceKeys(deviceId: String, publicKey: ByteArray, privateKey: ByteArray? = null)
    fun loadDeviceKeys(deviceId: String): Pair<ByteArray?, ByteArray?>
}
```

**优势：**
- ✅ 使用 EncryptedSharedPreferences (AES-256-GCM)
- ✅ 主密钥存储在 Android Keystore
- ✅ 密钥和元数据分离存储
- ✅ 有版本控制和迁移机制

**推荐保持当前实现**

### 4. 数据流程

#### 4.1 配对流程

```
1. 用户发起配对
   ↓
2. Rust 层执行 ECDH 密钥交换
   ↓
3. 创建 PairedDevice 对象
   ↓
4. 通过 FFI 传递到平台层
   ↓
5. 平台层存储：
   - macOS: 元数据 → UserDefaults, 密钥 → Keychain
   - Android: 元数据 → EncryptedPrefs, 密钥 → EncryptedPrefs
   ↓
6. 平台层调用 add_paired_device(FfiDeviceInfo)
   ↓
7. Rust 层更新运行时设备列表
```

#### 4.2 启动流程

```
1. 应用启动
   ↓
2. 平台层从存储加载设备列表
   - macOS: UserDefaults + Keychain
   - Android: EncryptedPrefs
   ↓
3. 对每个设备调用 add_paired_device(FfiDeviceInfo)
   ↓
4. Rust 层构建运行时设备列表
   ↓
5. 开始 mDNS 发现和自动连接
```

#### 4.3 连接流程

```
1. 发现设备（mDNS 或 BLE）
   ↓
2. 检查是否在配对列表中
   ↓
3. 建立连接（WiFi 或 BLE）
   ↓
4. 更新设备状态为 Connected
   ↓
5. （可选）更新 connection_info 到平台存储
```

### 5. FFI 接口

#### 5.1 设备管理接口

```rust
// 添加配对设备（启动时或配对后调用）
pub fn add_paired_device(&self, device: FfiDeviceInfo)

// 移除配对设备
pub fn remove_paired_device(&self, device_id: String)

// 获取所有配对设备
pub fn get_paired_devices(&self) -> Vec<FfiDeviceInfo>

// 获取已连接设备
pub fn get_connected_devices(&self) -> Vec<FfiDeviceInfo>

// 获取设备状态
pub fn get_device_status(&self, device_id: String) -> Option<DeviceStatus>
```

#### 5.2 UDL 定义

```udl
interface FfiNearClipManager {
    // 设备管理
    sequence<FfiDeviceInfo> get_paired_devices();
    sequence<FfiDeviceInfo> get_connected_devices();
    void add_paired_device(FfiDeviceInfo device);
    void remove_paired_device(string device_id);
    DeviceStatus? get_device_status(string device_id);
}
```

### 6. 安全考虑

#### 6.1 密钥存储

**原则：**
- ✅ 密钥必须存储在平台安全存储中
- ✅ macOS: Keychain
- ✅ Android: Keystore (通过 EncryptedSharedPreferences)
- ❌ 不要存储在普通文件或 UserDefaults

**密钥类型：**
1. **公钥** (65字节) - 对方设备的公钥
2. **私钥** (32字节) - 本设备的私钥（如果需要持久化）
3. **共享密钥哈希** (32字节) - SHA256 哈希，用于验证

**注意：**
- 完整的共享密钥（ECDH 结果）**不应该持久化**
- 每次连接时重新计算共享密钥
- 只存储哈希用于验证配对关系

#### 6.2 数据加密

**Android (推荐):**
```kotlin
// 使用 EncryptedSharedPreferences
val encryptedPrefs = EncryptedSharedPreferences.create(
    context,
    "nearclip_secure_prefs",
    masterKey,
    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
)
```

**macOS (推荐):**
```swift
// 使用 Keychain
let query: [String: Any] = [
    kSecClass as String: kSecClassGenericPassword,
    kSecAttrAccount as String: "nearclip.device.\(deviceId)",
    kSecValueData as String: publicKey,
    kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock
]
```

#### 6.3 访问控制

**macOS Keychain:**
- 使用 `kSecAttrAccessibleAfterFirstUnlock`
- 设备解锁后才能访问
- 支持 Touch ID/Face ID 保护

**Android Keystore:**
- 主密钥存储在硬件安全模块（如果可用）
- 支持生物识别保护
- 自动处理密钥轮换

### 7. 最佳实践

#### 7.1 启动时加载设备

**macOS 示例：**
```swift
class ConnectionManager {
    func initialize() {
        // 1. 从存储加载设备列表
        let storedDevices = keychainManager.loadPairedDevices()

        // 2. 转换为 FfiDeviceInfo
        let ffiDevices = storedDevices.map { device in
            FfiDeviceInfo(
                id: device.id,
                name: device.name,
                platform: DevicePlatform(rawValue: device.platform) ?? .unknown,
                status: .disconnected
            )
        }

        // 3. 添加到 Rust 层
        for device in ffiDevices {
            nearClipManager.addPairedDevice(device: device)
        }

        // 4. 启动管理器
        try? nearClipManager.start()
    }
}
```

**Android 示例：**
```kotlin
class ConnectionManager(private val context: Context) {
    private val secureStorage = SecureStorage(context)

    fun initialize() {
        // 1. 从存储加载设备列表
        val storedDevices = secureStorage.loadPairedDevices()

        // 2. 添加到 Rust 层
        storedDevices.forEach { device ->
            nearClipManager.addPairedDevice(device)
        }

        // 3. 启动管理器
        nearClipManager.start()
    }
}
```

#### 7.2 配对后保存设备

**macOS 示例：**
```swift
func onDeviceConnected(device: FfiDeviceInfo) {
    // 1. 保存元数据
    let storedDevice = StoredDevice(
        id: device.id,
        name: device.name,
        platform: device.platform.rawValue,
        addedAt: Date()
    )
    keychainManager.addPairedDevice(storedDevice)

    // 2. 保存密钥（如果有）
    if let publicKey = getPublicKeyForDevice(device.id) {
        keychainManager.saveDeviceKey(deviceId: device.id, publicKey: publicKey)
    }

    // 3. 通知 Rust 层
    nearClipManager.addPairedDevice(device: device)
}
```

**Android 示例：**
```kotlin
fun onDeviceConnected(device: FfiDeviceInfo) {
    // 1. 保存设备信息
    val devices = secureStorage.loadPairedDevices().toMutableList()
    devices.add(device)
    secureStorage.savePairedDevices(devices)

    // 2. 保存密钥（如果有）
    getPublicKeyForDevice(device.id)?.let { publicKey ->
        secureStorage.saveDeviceKeys(device.id, publicKey)
    }

    // 3. 通知 Rust 层
    nearClipManager.addPairedDevice(device)
}
```

#### 7.3 移除设备

**macOS 示例：**
```swift
func unpairDevice(deviceId: String) {
    // 1. 从 Rust 层移除
    nearClipManager.removePairedDevice(deviceId: deviceId)

    // 2. 从存储移除
    keychainManager.removePairedDevice(deviceId: deviceId)

    // 3. 删除密钥
    keychainManager.deleteDeviceKey(deviceId: deviceId)
}
```

**Android 示例：**
```kotlin
fun unpairDevice(deviceId: String) {
    // 1. 从 Rust 层移除
    nearClipManager.removePairedDevice(deviceId)

    // 2. 从存储移除
    val devices = secureStorage.loadPairedDevices()
        .filter { it.id != deviceId }
    secureStorage.savePairedDevices(devices)

    // 3. 删除密钥
    secureStorage.deleteDeviceKeys(deviceId)
}
```

### 8. 数据迁移

#### 8.1 版本控制

**Rust 层 (FileDeviceStore):**
```rust
const STORE_VERSION: u8 = 1;

struct StoreFile {
    version: u8,
    devices: Vec<PairedDevice>,
}
```

**Android 层:**
```kotlin
private const val STORAGE_VERSION_KEY = "storage_version"
private const val CURRENT_VERSION = 1

fun migrateIfNeeded() {
    val currentVersion = encryptedPrefs.getInt(STORAGE_VERSION_KEY, 0)
    if (currentVersion < CURRENT_VERSION) {
        // 执行迁移
        performMigration(currentVersion, CURRENT_VERSION)
        encryptedPrefs.edit()
            .putInt(STORAGE_VERSION_KEY, CURRENT_VERSION)
            .apply()
    }
}
```

#### 8.2 迁移策略

**添加新字段：**
- 使用可选字段（Option/Optional）
- 提供默认值
- 向后兼容

**修改字段类型：**
- 创建新字段
- 保留旧字段一段时间
- 逐步迁移数据

**删除字段：**
- 先标记为 deprecated
- 几个版本后再删除
- 提供迁移工具

### 9. 测试建议

#### 9.1 单元测试

**Rust 层：**
```rust
#[test]
fn test_device_store_crud() {
    let store = FileDeviceStore::new();
    let device = sample_device("device-1");

    // 保存
    store.save(&device).unwrap();

    // 加载
    let loaded = store.load("device-1").unwrap();
    assert!(loaded.is_some());

    // 删除
    store.delete("device-1").unwrap();
    assert!(!store.exists("device-1").unwrap());
}
```

**平台层：**
```swift
func testKeychainStorage() {
    let manager = KeychainManager()
    let device = StoredDevice(id: "test-1", name: "Test", platform: "macOS", addedAt: Date())

    // 保存
    XCTAssertTrue(manager.addPairedDevice(device))

    // 加载
    let devices = manager.loadPairedDevices()
    XCTAssertEqual(devices.count, 1)

    // 删除
    XCTAssertTrue(manager.removePairedDevice(deviceId: "test-1"))
}
```

#### 9.2 集成测试

**测试场景：**
1. 启动时加载设备列表
2. 配对新设备并保存
3. 重启应用后设备仍然存在
4. 移除设备后不再出现
5. 密钥正确存储和加载

### 10. 故障排查

#### 10.1 常见问题

**问题 1: 启动后设备列表为空**
- 检查平台层是否正确加载设备
- 检查是否调用了 `add_paired_device`
- 检查存储文件/Keychain 是否有数据

**问题 2: 密钥丢失**
- 检查 Keychain/Keystore 访问权限
- 检查设备是否解锁
- 检查密钥是否正确保存

**问题 3: 设备重复**
- 检查设备 ID 是否唯一
- 检查是否多次调用 `add_paired_device`
- 使用 `get_paired_devices` 验证

#### 10.2 调试工具

**Rust 层日志：**
```rust
tracing::debug!("Loaded {} devices from store", devices.len());
tracing::info!("Added new device: {}", device.device_id);
```

**平台层日志：**
```swift
print("Loaded \(devices.count) devices from Keychain")
```

```kotlin
Log.d("SecureStorage", "Loaded ${devices.size} devices")
```

### 11. 未来改进

#### 11.1 短期改进

1. **macOS 使用真正的 Keychain**
   - 将密钥从 UserDefaults 迁移到 Keychain
   - 添加 Touch ID/Face ID 保护

2. **添加连接信息持久化**
   - 保存最后成功连接的 IP/端口
   - 加速重连过程

3. **添加设备元数据**
   - 最后连接时间
   - 连接次数统计
   - 设备图标/颜色

#### 11.2 长期改进

1. **云同步**
   - iCloud Keychain (macOS/iOS)
   - Google Drive (Android)
   - 跨设备同步配对列表

2. **设备分组**
   - 工作设备 vs 个人设备
   - 自定义分组

3. **高级安全**
   - 设备指纹验证
   - 定期密钥轮换
   - 异常检测

## 总结

NearClip 的设备存储架构采用**分层设计**，充分利用了平台原生的安全存储能力。Rust 层专注于运行时状态管理和业务逻辑，平台层负责持久化和密钥安全。这种设计既保证了安全性，又保持了代码的简洁性和可维护性。

**关键原则：**
1. ✅ 密钥必须使用平台安全存储
2. ✅ Rust 层管理运行时，平台层管理持久化
3. ✅ 通过 FFI 保持数据同步
4. ✅ 不存储完整共享密钥，只存储哈希
5. ✅ 启动时从平台层加载设备到 Rust 层

---

**文档版本**: 1.0
**最后更新**: 2025-12-25
**作者**: Claude Code Agent
