# Task 00109: macOS 设备发现实现

## 任务描述
实现macOS平台的设备发现功能，包括WiFi和BLE两种传输方式的设备发现，适配macOS特有系统行为。

## 验收标准

### macOS设备发现核心功能
- [ ] 实现macOS设备发现代理，支持后台和前台运行
- [ ] 集成CoreBluetooth框架进行BLE设备扫描
- [ ] 实现WiFi网络设备发现（Bonjour/mDNS）
- [ ] 处理macOS系统休眠和唤醒后的设备重发现
- [ ] 正确处理macOS安全机制（网络访问、蓝牙权限）
- [ ] 在macOS菜单栏应用中显示设备发现状态

### 系统适配
- [ ] 适配不同macOS版本的网络和蓝牙API
- [ ] 处理macOS沙盒环境限制
- [ ] 支持macOS多个桌面空间的设备发现一致性
- [ ] 适配macOS系统偏好设置中的网络配置变化

### 权限和沙盒
- [ ] 实现蓝牙权限请求和处理
- [ ] 处理网络访问权限
- [ ] 适配macOS沙盒环境下的文件和系统访问
- [ ] 实现无权限时的优雅降级

### UI集成
- [ ] 创建macOS设备列表界面（SwiftUI）
- [ ] 实现菜单栏状态图标和弹出窗口
- [ ] 添加系统通知支持
- [ ] 支持设备选择和连接操作

### 后台处理
- [ ] 实现macOS后台代理（Background Agent）
- [ ] 处理应用休眠和唤醒状态
- [ ] 实现系统启动时的自动启动
- [ ] 支持低电量模式优化

## 技术实现

### 核心组件
```swift
// macOS设备发现管理器
class MacOSDeviceDiscoveryManager: ObservableObject {
    // BLE扫描管理
    private let bleScanner: BLEScannerManager
    
    // WiFi设备发现（Bonjour）
    private let bonjourDiscovery: BonjourDiscoveryManager
    
    // 设备发现状态
    @Published var discoveryState: DiscoveryState
    
    // 系统状态监控
    private let systemMonitor: SystemMonitor
}

// CoreBluetooth扫描管理器
class BLEScannerManager: NSObject, CBCentralManagerDelegate {
    private var centralManager: CBCentralManager!
    private var discoveredPeripherals: [String: CBPeripheral] = []
    
    func startScanning() -> AsyncStream<DiscoveredDevice>
    func stopScanning()
    var isScanning: Bool { get }
}

// Bonjour设备发现管理器
class BonjourDiscoveryManager: NSObject, NetServiceBrowserDelegate, NetServiceDelegate {
    private var browser: NetServiceBrowser!
    private var discoveredServices: [String: NetService] = []
    
    func startDiscovery() -> AsyncStream<DiscoveredDevice>
    func stopDiscovery()
    var isActive: Bool { get }
}
```

### 系统权限管理
```swift
class SystemPermissionManager {
    static let shared = SystemPermissionManager()
    
    func requestBluetoothPermission() async -> Bool
    func hasBluetoothPermission() -> Bool
    func requestNetworkAccessPermission() async -> Bool
    func hasNetworkAccessPermission() -> Bool
    func shouldShowPermissionExplanation(for permission: SystemPermission) -> Bool
}

enum SystemPermission {
    case bluetooth
    case networkAccess
    case notifications
}
```

### 系统状态监控
```swift
class SystemMonitor {
    // 系统休眠/唤醒监控
    private var sleepObserver: NSObjectProtocol?
    
    // 网络状态监控
    private var networkMonitor: NWPathMonitor?
    
    // 电池状态监控
    private var batteryMonitor: BatteryMonitor?
    
    func startMonitoring() -> AsyncStream<SystemEvent>
    func stopMonitoring()
}

enum SystemEvent {
    case systemWillSleep
    case systemDidWake
    case networkDidChange(NetworkStatus)
    case batteryDidChange(BatteryStatus)
}
```

### UI组件
```swift
// 主界面
struct DeviceDiscoveryView: View {
    @StateObject private var discoveryManager = MacOSDeviceDiscoveryManager()
    
    var body: some View {
        DeviceListView(devices: discoveryManager.discoveredDevices)
            .toolbar {
                DiscoveryStatusIndicator(
                    isScanning: discoveryManager.isScanning,
                    deviceCount: discoveryManager.discoveredDevices.count
                )
            }
    }
}

// 菜单栏应用
struct MenuBarApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        MenuBarExtra("NearClip", systemImage: "antenna.radiowaves.left.and.right") {
            DeviceDiscoveryView()
        }
    }
}

// 设备列表
struct DeviceListView: View {
    let devices: [DiscoveredDevice]
    
    var body: some View {
        List(devices) { device in
            DeviceRow(device: device)
        }
    }
}
```

### 后台代理
```swift
class BackgroundAgent {
    private let discoveryManager: MacOSDeviceDiscoveryManager
    private let powerManager: PowerManager
    
    func startBackgroundDiscovery()
    func stopBackgroundDiscovery()
    func handleSystemEvent(_ event: SystemEvent)
}

class PowerManager {
    func isLowPowerModeEnabled() -> Bool
    func getOptimalScanInterval() -> TimeInterval
    func shouldSuspendDiscovery() -> Bool
}
```

## 测试要求

### 单元测试
- [ ] BLE扫描管理器单元测试
- [ ] Bonjour设备发现单元测试
- [ ] 系统权限管理器单元测试
- [ ] 系统状态监控单元测试

### 集成测试
- [ ] 设备发现服务集成测试
- [ ] 权限请求流程测试
- [ ] 系统事件处理测试

### UI测试
- [ ] 设备列表界面测试
- [ ] 菜单栏功能测试
- [ ] 状态指示器显示测试

## 性能要求

### 内存使用
- 设备发现服务内存占用 < 20MB
- UI组件内存占用 < 15MB

### CPU使用
- 空闲状态CPU使用 < 1%
- 扫描状态CPU使用 < 5%

### 电池消耗
- 后台设备发现电池消耗 < 2%/小时
- 前台设备发现电池消耗 < 4%/小时

## 依赖项

### Swift Package依赖
```swift
// Package.swift
let package = Package(
    name: "NearClip",
    platforms: [.macOS(.v13)],
    dependencies: [
        .package(url: "https://github.com/apple/swift-async-algorithms", from: "1.0.0"),
        .package(url: "https://github.com/pointfreeco/swift-composable-architecture", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "NearClipApp",
            dependencies: [
                "NearClipCore",
                .product(name: "AsyncAlgorithms", package: "swift-async-algorithms"),
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture")
            ]
        )
    ]
)
```

### Rust FFI依赖
```swift
// Rust绑定
import NearClipCore
```

## 相关文件

- [任务 00101: 设备类型枚举](../tasks/00101-device-type-enum.md)
- [任务 00102: 设备状态枚举](../tasks/00102-device-status-enum.md)
- [任务 00103: 设备能力枚举](../tasks/00103-device-capability-enum.md)
- [任务 00104: 设备基础结构](../tasks/00104-device-basic-structure.md)
- [任务 00105: 发现事件枚举](../tasks/00105-discovery-event-enum.md)
- [任务 00106: 设备发现特质](../tasks/00106-device-discovery-trait.md)
- [任务 00107: 发现配置结构](../tasks/00107-discovery-config-structure.md)