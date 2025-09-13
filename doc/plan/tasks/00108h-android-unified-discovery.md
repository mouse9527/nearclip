# Task 00108h: Android 统一设备发现管理

## 任务描述

实现Android平台的统一设备发现管理器，将BLE和WiFi发现结果合并，为用户提供透明的设备发现体验。系统应智能选择最优发现方式，确保同一设备只显示一次。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class UnifiedDiscoveryManagerTest {
    @Test
    fun testDeviceDeduplication() {
        // RED: 测试设备去重
        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)
        
        runBlocking {
            val devices = mutableListOf<UnifiedDevice>()
            val job = launch {
                manager.startDiscovery().collect { device ->
                    devices.add(device)
                }
            }
            
            // 模拟通过BLE和WiFi发现同一个设备
            manager.addMockDevice(bluetoothDevice, "device-123")
            manager.addMockDevice(wifiDevice, "device-123")
            
            delay(100)
            job.cancel()
            
            assertEquals(1, devices.size) // 同一个设备只应出现一次
            assertEquals("device-123", devices[0].id)
        }
    }

    @Test
    fun testIntelligentTransportSelection() {
        // RED: 测试智能传输方式选择
        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)
        
        // 模拟WiFi网络可用
        `when`(mockNetworkInfo.isConnected).thenReturn(true)
        
        val selection = manager.selectOptimalTransport()
        assertEquals(TransportType.WIFI, selection)
    }

    @Test
    fun testDeviceMerging() {
        // RED: 测试设备信息合并
        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)
        
        runBlocking {
            val devices = mutableListOf<UnifiedDevice>()
            val job = launch {
                manager.startDiscovery().collect { device ->
                    devices.add(device)
                }
            }
            
            // 先通过BLE发现
            manager.addMockDevice(bluetoothDevice, "device-123")
            delay(50)
            
            // 再通过WiFi发现（应合并信息）
            manager.addMockDevice(wifiDevice, "device-123")
            delay(50)
            
            job.cancel()
            
            assertEquals(1, devices.size)
            assertTrue(devices[0].transports.contains(TransportType.BLE))
            assertTrue(devices[0].transports.contains(TransportType.WIFI))
        }
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
data class UnifiedDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transports: Set<TransportType>,
    val quality: Float,
    val lastSeen: Long,
    val attributes: Map<String, Any>
)

class UnifiedDiscoveryManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val connectivityManager: ConnectivityManager
) {
    private val bleScanner = BLEScannerManager(context, bluetoothAdapter)
    private val wifiDiscovery = WiFiDiscoveryManager(context, connectivityManager)
    private val discoveredDevices = mutableMapOf<String, UnifiedDevice>()
    private val mockDeviceFlow = MutableSharedFlow<UnifiedDevice>()
    
    suspend fun startDiscovery(): Flow<UnifiedDevice> = callbackFlow {
        // 智能选择发现方式
        val optimalTransport = selectOptimalTransport()
        
        // 启动相应的发现服务
        when (optimalTransport) {
            TransportType.WIFI -> launchWiFiDiscovery()
            TransportType.BLE -> launchBLEDiscovery()
            else -> launchBothDiscoveries()
        }
        
        awaitClose { stopAllDiscoveries() }
    }
    
    private fun selectOptimalTransport(): TransportType {
        val networkAvailable = isNetworkAvailable()
        val bluetoothEnabled = bluetoothAdapter.isEnabled
        
        return when {
            networkAvailable && bluetoothEnabled -> TransportType.WIFI // WiFi优先
            bluetoothEnabled -> TransportType.BLE
            else -> throw DiscoveryError.NoTransportAvailable
        }
    }
    
    private suspend fun launchWiFiDiscovery() {
        wifiDiscovery.startDiscovery().collect { wifiDevice ->
            mergeDevice(wifiDevice, TransportType.WIFI)
        }
    }
    
    private suspend fun launchBLEDiscovery() {
        bleScanner.startScanFlow().collect { bleDevice ->
            mergeDevice(bleDevice, TransportType.BLE)
        }
    }
    
    private fun mergeDevice(device: Any, transport: TransportType) {
        val deviceId = when (device) {
            is WiFiDiscoveredDevice -> device.id
            is DiscoveredDevice -> device.id
            else -> return
        }
        
        val existingDevice = discoveredDevices[deviceId]
        val unifiedDevice = if (existingDevice != null) {
            // 合并设备信息
            existingDevice.copy(
                transports = existingDevice.transports + transport,
                quality = calculateQuality(existingDevice, device, transport),
                lastSeen = maxOf(existingDevice.lastSeen, System.currentTimeMillis())
            )
        } else {
            // 创建新设备
            createUnifiedDevice(device, transport)
        }
        
        discoveredDevices[deviceId] = unifiedDevice
    }
    
    private fun createUnifiedDevice(device: Any, transport: TransportType): UnifiedDevice {
        return when (device) {
            is WiFiDiscoveredDevice -> UnifiedDevice(
                id = device.id,
                name = device.name,
                type = device.type,
                transports = setOf(transport),
                quality = 0.8f,
                lastSeen = device.lastSeen,
                attributes = mapOf("port" to device.port)
            )
            is DiscoveredDevice -> UnifiedDevice(
                id = device.id,
                name = device.name,
                type = device.type,
                transports = setOf(transport),
                quality = calculateSignalQuality(device.rssi),
                lastSeen = device.lastSeen,
                attributes = mapOf("rssi" to device.rssi)
            )
            else -> throw IllegalArgumentException("Unknown device type")
        }
    }
}
```

### REFACTOR阶段
```kotlin
class UnifiedDiscoveryManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val connectivityManager: ConnectivityManager,
    private val discoveryConfig: UnifiedDiscoveryConfig = UnifiedDiscoveryConfig.default()
) {
    // 添加网络状态监控
    private val networkCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            // 网络可用时切换到WiFi优先模式
            updateDiscoveryStrategy()
        }
        
        override fun onLost(network: Network) {
            // 网络丢失时切换到BLE模式
            updateDiscoveryStrategy()
        }
    }
    
    // 添加设备质量评估算法
    private fun calculateDeviceQuality(device: UnifiedDevice): Float {
        val transportScore = when {
            device.transports.contains(TransportType.WIFI) -> 0.8f
            device.transports.contains(TransportType.BLE) -> 0.6f
            else -> 0.3f
        }
        
        val signalScore = device.quality
        val recencyScore = calculateRecencyScore(device.lastSeen)
        
        return (transportScore + signalScore + recencyScore) / 3f
    }
    
    // 添加智能发现策略
    private fun updateDiscoveryStrategy() {
        val strategy = when {
            isNetworkAvailable() && bluetoothAdapter.isEnabled -> 
                DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY
            bluetoothAdapter.isEnabled -> 
                DiscoveryStrategy.BLE_ONLY
            else -> 
                DiscoveryStrategy.NONE
        }
        
        applyDiscoveryStrategy(strategy)
    }
}

data class UnifiedDiscoveryConfig(
    val wifiEnabled: Boolean,
    val bleEnabled: Boolean,
    val discoveryTimeout: Long,
    val enableSmartSwitching: Boolean,
    val maxDevices: Int
) {
    companion object {
        fun default() = UnifiedDiscoveryConfig(
            wifiEnabled = true,
            bleEnabled = true,
            discoveryTimeout = 30000L,
            enableSmartSwitching = true,
            maxDevices = 20
        )
    }
}

enum class DiscoveryStrategy {
    WIFI_PRIMARY_BLE_SECONDARY,
    BLE_PRIMARY_WIFI_SECONDARY,
    BLE_ONLY,
    WIFI_ONLY,
    NONE
}
```

## 验收标准
- [ ] BLE和WiFi发现的设备能够正确合并
- [ ] 同一设备在UI中只显示一次
- [ ] 系统根据网络状态智能选择发现方式
- [ ] 设备信息包含所有可用的传输方式
- [ ] 网络状态变化时自动调整发现策略
- [ ] 设备质量评分算法工作正常

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108b: Android BLE 设备发现核心](00108b-android-ble-discovery-core.md)
- [Task 00108c: Android WiFi 设备发现核心](00108c-android-wifi-discovery-core.md)

## 后续任务
- [Task 00108i: Android 设备连接管理](00108i-android-device-connection.md)