# Story 4.8: 实现 Keychain 存储

Status: done

## Story

As a macOS 用户,
I want 配对信息安全存储在 Keychain 中,
So that 数据安全且应用重启后保留.

## Acceptance Criteria

1. **Given** 添加新设备 **When** 配对成功 **Then** 设备信息存储到 Keychain
2. **And** 应用启动时从 Keychain 加载已配对设备
3. **And** 删除设备时从 Keychain 移除
4. **And** 使用 Security framework 安全访问

## Tasks / Subtasks

- [x] Task 1: 创建 KeychainManager (AC: 1, 3, 4)
  - [x] 1.1 实现 save() 方法存储数据 (SecItemAdd)
  - [x] 1.2 实现 load() 方法读取数据 (SecItemCopyMatching)
  - [x] 1.3 实现 delete() 方法删除数据 (SecItemDelete)
  - [x] 1.4 使用 kSecClassGenericPassword

- [x] Task 2: 实现设备列表存储 (AC: 1, 2, 3)
  - [x] 2.1 StoredDevice Codable 结构序列化为 JSON
  - [x] 2.2 savePairedDevices/loadPairedDevices 方法

- [x] Task 3: 集成到 ConnectionManager (AC: 1, 2, 3)
  - [x] 3.1 init() 时调用 loadPairedDevicesFromKeychain()
  - [x] 3.2 addPairedDevice() 保存到 Keychain
  - [x] 3.3 removePairedDevice() 从 Keychain 移除

- [x] Task 4: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 4.1 运行 swift build 成功
  - [x] 4.2 Keychain 存储功能完整

## Dev Notes

### 架构约束

1. **Security Framework**: macOS 原生 Keychain API
2. **kSecClassGenericPassword**: 存储通用密码/数据
3. **JSON 序列化**: Codable 协议序列化设备列表

### Keychain 配置

| 属性 | 值 |
|------|-----|
| kSecClass | kSecClassGenericPassword |
| kSecAttrService | "com.nearclip.pairing" |
| kSecAttrAccount | "paired-devices" |

### 与其他 Story 的关系

- Story 4-6: 配对界面添加设备
- Story 4-7: 设置界面删除设备
- Story 2-9: Rust 端设备持久化

### 实现的 KeychainManager

```swift
final class KeychainManager {
    static let shared = KeychainManager()
    private let service = "com.nearclip.pairing"

    func savePairedDevices(_ devices: [StoredDevice]) -> Bool
    func loadPairedDevices() -> [StoredDevice]
    func addPairedDevice(_ device: StoredDevice) -> Bool
    func removePairedDevice(deviceId: String) -> Bool
}

struct StoredDevice: Codable {
    let id: String
    let name: String
    let platform: String
    let addedAt: Date
}
```

### ConnectionManager 集成

```swift
private init() {
    loadPairedDevicesFromKeychain()  // 启动时加载
}

func addPairedDevice(_ device: DeviceDisplay) {
    savePairedDeviceToKeychain(device)  // 保存到 Keychain
    // + 更新 FFI
}

func removePairedDevice(_ deviceId: String) {
    removePairedDeviceFromKeychain(deviceId)  // 从 Keychain 移除
    // + 更新 FFI
}
```

### 文件结构

```
Sources/NearClip/
├── KeychainManager.swift    # NEW - Keychain 存储
├── ConnectionManager.swift  # UPDATED - Keychain 集成
├── PairingView.swift        # UPDATED - 使用 addPairedDevice
├── SettingsView.swift       # UPDATED - 使用 removePairedDevice
└── ...
```

## Checklist

- [x] All tasks completed
- [x] Keychain save works (SecItemAdd)
- [x] Keychain load works (SecItemCopyMatching)
- [x] Keychain delete works (SecItemDelete)
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
