# Task 00108: Android 设备发现实现

## 任务描述
实现Android平台的设备发现功能，包括WiFi和BLE两种传输方式的设备发现。

## 验收标准

### Android设备发现核心功能
- [ ] 实现Android设备发现服务，支持后台和前台运行
- [ ] 集成Android BLE扫描功能，发现NearClip设备
- [ ] 实现WiFi网络设备发现（mDNS/UDP广播）
- [ ] 处理Android Doze模式和App Standby的设备发现策略
- [ ] 在Android 10+上正确处理蓝牙和WiFi权限请求
- [ ] 实现Android锁屏状态下的BLE扫描能力

### 权限处理
- [ ] 实现运行时权限请求（蓝牙、WiFi、位置）
- [ ] 处理Android 12+的蓝牙权限变化
- [ ] 适配不同Android版本的权限API

### 电池优化
- [ ] 实现智能扫描频率调整（低电量模式）
- [ ] 优化BLE扫描间隔和持续时间
- [ ] 实现后台服务的电池优化策略

### UI集成
- [ ] 创建Android设备列表界面（Jetpack Compose）
- [ ] 实现设备发现状态指示器
- [ ] 添加Android通知系统状态显示
- [ ] 支持设备选择和连接操作

### 后台处理
- [ ] 实现Android Foreground Service
- [ ] 处理应用被系统杀死的恢复机制
- [ ] 支持多用户环境下的设备发现

## 技术实现

### 核心组件
```kotlin
// Android设备发现服务
class AndroidDeviceDiscoveryService : Service() {
    // BLE扫描管理
    private val bleScanner: BLEScannerManager
    
    // WiFi设备发现
    private val wifiDiscovery: WiFiDiscoveryManager
    
    // 设备发现状态管理
    private val discoveryState: DiscoveryStateManager
}

// BLE扫描管理器
class BLEScannerManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter
) {
    suspend fun startScan(): Flow<DiscoveredDevice>
    suspend fun stopScan()
    fun isScanning(): Boolean
}

// WiFi设备发现管理器
class WiFiDiscoveryManager(
    private val context: Context,
    private val networkManager: ConnectivityManager
) {
    suspend fun startDiscovery(): Flow<DiscoveredDevice>
    suspend fun stopDiscovery()
    fun isActive(): Boolean
}
```

### 权限管理
```kotlin
class PermissionManager(
    private val activity: Activity
) {
    suspend fun requestRequiredPermissions(): Boolean
    fun hasRequiredPermissions(): Boolean
    fun shouldShowPermissionRationale(): Boolean
}
```

### 电池优化
```kotlin
class BatteryOptimizationManager(
    private val context: Context
) {
    fun isBatteryOptimizationEnabled(): Boolean
    fun requestDisableBatteryOptimization()
    fun getOptimalScanInterval(): Long
}
```

### UI组件
```kotlin
@Composable
fun DeviceDiscoveryScreen(
    discoveryState: DiscoveryState,
    onDeviceSelected: (Device) -> Unit,
    onRefresh: () -> Unit
) {
    // 设备列表UI
}

@Composable
fun DiscoveryStatusIndicator(
    isScanning: Boolean,
    discoveredDeviceCount: Int
) {
    // 状态指示器UI
}
```

## 测试要求

### 单元测试
- [ ] BLE扫描管理器单元测试
- [ ] WiFi设备发现单元测试
- [ ] 权限管理器单元测试
- [ ] 电池优化管理器单元测试

### 集成测试
- [ ] Android设备发现服务集成测试
- [ ] 权限请求流程测试
- [ ] 后台服务稳定性测试

### UI测试
- [ ] 设备列表界面测试
- [ ] 设备选择操作测试
- [ ] 状态指示器显示测试

## 性能要求

### 内存使用
- 设备发现服务内存占用 < 15MB
- UI组件内存占用 < 10MB

### CPU使用
- 空闲状态CPU使用 < 2%
- 扫描状态CPU使用 < 8%

### 电池消耗
- 后台设备发现电池消耗 < 3%/小时
- 前台设备发现电池消耗 < 5%/小时

## 依赖项

### Android依赖
```gradle
// BLE相关
implementation "androidx.bluetooth:bluetooth:1.0.0"

// WiFi网络相关
implementation "androidx.net:net:1.0.0"

// 权限处理
implementation "androidx.activity:activity-ktx:1.8.0"
implementation "androidx.fragment:fragment-ktx:1.6.0"

// Coroutines和Flow
implementation "org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.0"
implementation "androidx.lifecycle:lifecycle-runtime-ktx:2.6.0"

// Jetpack Compose
implementation "androidx.compose.ui:ui:1.5.0"
implementation "androidx.compose.material3:material3:1.1.0"
```

### Rust FFI依赖
```gradle
// Rust绑定
implementation "com.github.rust-unofficial:android-rust:0.1.0"
```

## 相关文件

- [任务 00101: 设备类型枚举](../tasks/00101-device-type-enum.md)
- [任务 00102: 设备状态枚举](../tasks/00102-device-status-enum.md)
- [任务 00103: 设备能力枚举](../tasks/00103-device-capability-enum.md)
- [任务 00104: 设备基础结构](../tasks/00104-device-basic-structure.md)
- [任务 00105: 发现事件枚举](../tasks/00105-discovery-event-enum.md)
- [任务 00106: 设备发现特质](../tasks/00106-device-discovery-trait.md)
- [任务 00107: 发现配置结构](../tasks/00107-discovery-config-structure.md)