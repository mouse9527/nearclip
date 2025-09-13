# Task 00108a: Android BLE 设备发现核心

## 任务描述

实现Android平台的BLE设备发现核心功能，专注于BLE扫描和NearClip设备识别。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class BLEScannerManagerTest {
    @Test
    fun testBLEScannerStartStop() {
        // RED: 测试BLE扫描器启动停止
        val scanner = BLEScannerManager(mockContext, mockBluetoothAdapter)
        
        assertFalse(scanner.isScanning())
        
        runBlocking {
            val result = scanner.startScan()
            assertTrue(result.isSuccess)
            assertTrue(scanner.isScanning())
            
            val stopResult = scanner.stopScan()
            assertTrue(stopResult.isSuccess)
            assertFalse(scanner.isScanning())
        }
    }

    @Test
    fun testBLEDeviceDiscovery() {
        // RED: 测试BLE设备发现
        val scanner = BLEScannerManager(mockContext, mockBluetoothAdapter)
        
        runBlocking {
            val devices = mutableListOf<DiscoveredDevice>()
            val job = launch {
                scanner.startScan().collect { device ->
                    devices.add(device)
                }
            }
            
            // 模拟发现设备
            scanner.addMockDevice(mockBLEDevice)
            
            delay(100)
            job.cancel()
            
            assertEquals(1, devices.size)
            assertEquals("mock-device-id", devices[0].id)
        }
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class BLEScannerManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter
) {
    private var isScanning = false
    private val scanCallbacks = mutableListOf<(DiscoveredDevice) -> Unit>()
    private val discoveredDevices = mutableMapOf<String, DiscoveredDevice>()
    
    suspend fun startScan(): Flow<DiscoveredDevice> = callbackFlow {
        if (!bluetoothAdapter.isEnabled) {
            close(BLEDiscoveryError.BluetoothDisabled)
            return@callbackFlow
        }
        
        val scanner = bluetoothAdapter.bluetoothLeScanner ?: run {
            close(BLEDiscoveryError.BluetoothNotSupported)
            return@callbackFlow
        }
        
        val scanCallback = object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                val device = result.device
                if (isNearClipDevice(result)) {
                    val discoveredDevice = DiscoveredDevice(
                        id = device.address,
                        name = device.name ?: "Unknown Device",
                        type = DeviceType.fromBluetoothDeviceType(device.type),
                        transport = TransportType.BLE,
                        rssi = result.rssi,
                        lastSeen = System.currentTimeMillis()
                    )
                    
                    trySend(discoveredDevice)
                }
            }
            
            override fun onScanFailed(errorCode: Int) {
                close(BLEDiscoveryError.ScanFailed(errorCode))
            }
        }
        
        val scanSettings = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_POWER)
            .build()
        
        val scanFilter = ScanFilter.Builder()
            .setServiceUuid(ParcelUuid(NearClipServiceUUID))
            .build()
        
        scanner.startScan(listOf(scanFilter), scanSettings, scanCallback)
        isScanning = true
        
        awaitClose {
            scanner.stopScan(scanCallback)
            isScanning = false
        }
    }
    
    suspend fun stopScan() {
        // Flow会自动处理停止
    }
    
    fun isScanning(): Boolean = isScanning
    
    private fun isNearClipDevice(result: ScanResult): Boolean {
        return result.scanRecord?.serviceUuids?.any { 
            it.uuid == NearClipServiceUUID 
        } ?: false
    }
}

data class DiscoveredDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transport: TransportType,
    val rssi: Int,
    val lastSeen: Long
)

enum class BLEDiscoveryError {
    BluetoothDisabled,
    BluetoothNotSupported,
    ScanFailed(Int),
    PermissionDenied
}
```

### REFACTOR阶段
```kotlin
class BLEScannerManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val scanConfig: BLEScanConfig = BLEScanConfig.default()
) {
    // 添加配置支持
    // 添加设备缓存和去重
    // 添加信号质量评估
}

data class BLEScanConfig(
    val scanMode: Int,
    val scanInterval: Long,
    val scanWindow: Long,
    val enableLegacy: Boolean,
    val phy: Int
) {
    companion object {
        fun default() = BLEScanConfig(
            scanMode = ScanSettings.SCAN_MODE_LOW_POWER,
            scanInterval = 5000L,
            scanWindow = 1000L,
            enableLegacy = true,
            phy = ScanSettings.PHY_LE_ALL_SUPPORTED
        )
        
        fun aggressive() = BLEScanConfig(
            scanMode = ScanSettings.SCAN_MODE_LOW_LATENCY,
            scanInterval = 1000L,
            scanWindow = 500L,
            enableLegacy = true,
            phy = ScanSettings.PHY_LE_ALL_SUPPORTED
        )
        
        fun powerSaving() = BLEScanConfig(
            scanMode = ScanSettings.SCAN_MODE_OPPORTUNISTIC,
            scanInterval = 10000L,
            scanWindow = 500L,
            enableLegacy = true,
            phy = ScanSettings.PHY_LE_1M_MASK
        )
    }
}
```

## 验收标准
- [ ] BLE扫描器正常启动和停止
- [ ] 能够发现NearClip BLE设备
- [ ] 正确处理蓝牙权限和状态
- [ ] 设备信息格式化正确
- [ ] 错误处理机制完善

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00108b: Android WiFi 设备发现核心](00108b-android-wifi-discovery-core.md)
- [Task 00108c: Android 权限管理](00108c-android-permission-management.md)