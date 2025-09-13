package com.mouse.nearclip

import android.bluetooth.BluetoothAdapter
import android.bluetooth.le.BluetoothLeScanner
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.bluetooth.le.ScanFilter
import android.os.ParcelUuid
import android.content.Context
import android.content.pm.PackageManager
import android.Manifest
import android.annotation.SuppressLint
import androidx.core.content.ContextCompat
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

data class DiscoveredDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transport: TransportType,
    val rssi: Int,
    val lastSeen: Long
)

enum class DeviceType {
    NEARCLIP,
    OTHER
}

enum class TransportType {
    BLE,
    WIFI
}

sealed class BLEDiscoveryError(message: String) : Exception(message) {
    object BluetoothDisabled : BLEDiscoveryError("Bluetooth is disabled")
    object BluetoothNotSupported : BLEDiscoveryError("Bluetooth is not supported")
    object PermissionDenied : BLEDiscoveryError("Bluetooth permission denied")
    data class ScanFailed(val errorCode: Int) : BLEDiscoveryError("Scan failed with code: $errorCode")
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
            scanMode = android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_POWER,
            scanInterval = 5000L,
            scanWindow = 1000L,
            enableLegacy = true,
            phy = android.bluetooth.le.ScanSettings.PHY_LE_ALL_SUPPORTED
        )
        
        fun aggressive() = BLEScanConfig(
            scanMode = android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_LATENCY,
            scanInterval = 1000L,
            scanWindow = 500L,
            enableLegacy = true,
            phy = android.bluetooth.le.ScanSettings.PHY_LE_ALL_SUPPORTED
        )
        
        fun powerSaving() = BLEScanConfig(
            scanMode = android.bluetooth.le.ScanSettings.SCAN_MODE_OPPORTUNISTIC,
            scanInterval = 10000L,
            scanWindow = 500L,
            enableLegacy = true,
            phy = 1 // Use 1 as fallback for PHY_LE_1M_MASK
        )
    }
}

@SuppressLint("MissingPermission")
class BLEScannerManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val scanConfig: BLEScanConfig = BLEScanConfig.default(),
    private val batteryManager: BatteryOptimizationManager = BatteryOptimizationManager(context)
) {
    private var isScanning = false
    private val discoveredDevices = mutableMapOf<String, DiscoveredDevice>()
    private val mockDeviceFlow = MutableSharedFlow<DiscoveredDevice>()
    private val scope = CoroutineScope(Dispatchers.Default)
    
    // Refactoring: Add device deduplication and caching
    private val deviceCache = mutableMapOf<String, DiscoveredDevice>()
    private val signalQualityCache = mutableMapOf<String, Float>()
    
    // Refactoring: Add scan statistics
    private var scanStartTime = 0L
    private var totalDevicesDiscovered = 0
    
    suspend fun startScan(): Result<Unit> {
        if (!bluetoothAdapter.isEnabled) {
            return Result.failure(BLEDiscoveryError.BluetoothDisabled)
        }
        
        // 检查是否应该减少发现频率
        if (batteryManager.shouldReduceDiscoveryFrequency()) {
            return Result.failure(BLEDiscoveryError.ScanFailed(0)) // 自定义错误码表示电池限制
        }
        
        // Refactoring: Initialize scan statistics
        scanStartTime = System.currentTimeMillis()
        totalDevicesDiscovered = 0
        
        isScanning = true
        return Result.success(Unit)
    }
    
    suspend fun stopScan(): Result<Unit> {
        isScanning = false
        return Result.success(Unit)
    }
    
    fun isScanning(): Boolean = isScanning
    
    // 添加电池优化相关的方法
    fun getBatteryOptimizationStatus(): Boolean {
        return batteryManager.isBatteryOptimizationEnabled()
    }
    
    fun getBatteryLevel(): Int {
        return batteryManager.getBatteryLevel()
    }
    
    fun isLowPowerMode(): Boolean {
        return batteryManager.isLowPowerMode()
    }
    
    fun getCurrentDiscoveryStrategy(): DiscoveryStrategy {
        return batteryManager.getOptimalDiscoveryStrategy()
    }
    
    fun getBatteryMetrics(): BatteryMetrics {
        return batteryManager.getPerformanceMetrics()
    }
    
    suspend fun requestBatteryOptimization(): Boolean {
        return batteryManager.requestDisableBatteryOptimization()
    }
    
    suspend fun startScanFlow(): Flow<DiscoveredDevice> = callbackFlow {
        if (!bluetoothAdapter.isEnabled) {
            close(BLEDiscoveryError.BluetoothDisabled)
            return@callbackFlow
        }
        
        // 使用电池优化的扫描参数
        val optimizedParams = batteryManager.getScanParameters()
        
        // For testing purposes, combine both real BLE scanning and mock devices
        val realScanner = bluetoothAdapter.bluetoothLeScanner
        
        // Handle mock devices if any
        val mockJob = scope.launch {
            mockDeviceFlow.collect { mockDevice ->
                trySend(mockDevice)
            }
        }
        
        // Real BLE scanning
        if (realScanner != null) {
            val scanCallback = object : ScanCallback() {
                      override fun onScanResult(callbackType: Int, result: ScanResult) {
                    val device = result.device
                    if (isNearClipDevice(result)) {
                        // Refactoring: Add device deduplication and signal quality assessment
                        val deviceId = device.address
                        val signalQuality = calculateSignalQuality(result.rssi)
                        
                        // Update or create device
                        val discoveredDevice = deviceCache[deviceId]?.copy(
                            rssi = result.rssi,
                            lastSeen = System.currentTimeMillis()
                        ) ?: DiscoveredDevice(
                            id = deviceId,
                            name = device.name ?: "Unknown Device",
                            type = fromBluetoothDeviceType(device.type),
                            transport = TransportType.BLE,
                            rssi = result.rssi,
                            lastSeen = System.currentTimeMillis()
                        )
                        
                        // Cache device and signal quality
                        deviceCache[deviceId] = discoveredDevice
                        signalQualityCache[deviceId] = signalQuality
                        discoveredDevices[deviceId] = discoveredDevice
                        totalDevicesDiscovered++
                        
                        trySend(discoveredDevice)
                    }
                }
                
                override fun onScanFailed(errorCode: Int) {
                    close(BLEDiscoveryError.ScanFailed(errorCode))
                }
            }
            
            // 使用电池优化的扫描参数
            val scanSettings = ScanSettings.Builder()
                .setScanMode(optimizedParams.mode)
                .build()
            
            val scanFilter = ScanFilter.Builder()
                .setServiceUuid(ParcelUuid.fromString("0000FE6C-0000-1000-8000-00805F9B34FB")) // NearClip Service UUID
                .build()
            
            realScanner.startScan(listOf(scanFilter), scanSettings, scanCallback)
            isScanning = true
            
            awaitClose {
                realScanner.stopScan(scanCallback)
                mockJob.cancel()
                isScanning = false
            }
        } else {
            // If no BLE scanner available, just use mock flow
            awaitClose {
                mockJob.cancel()
                isScanning = false
            }
        }
    }
    
    fun addMockDevice(device: Any) {
        // Create a mock device for testing
        val mockDevice = DiscoveredDevice(
            id = "mock-device-id",
            name = "Mock Device",
            type = DeviceType.NEARCLIP,
            transport = TransportType.BLE,
            rssi = -50,
            lastSeen = System.currentTimeMillis()
        )
        
        // Use scope to send to flow
        scope.launch {
            mockDeviceFlow.emit(mockDevice)
        }
    }
    
    fun getDiscoveredDevices(): List<DiscoveredDevice> = discoveredDevices.values.toList()
    
    fun clearDiscoveredDevices() {
        clearCache()
    }
    
    private fun isNearClipDevice(result: ScanResult): Boolean {
        return result.scanRecord?.serviceUuids?.any { 
            it.uuid.toString() == "0000FE6C-0000-1000-8000-00805F9B34FB" 
        } ?: false
    }
    
    // Refactoring: Add signal quality assessment
    private fun calculateSignalQuality(rssi: Int): Float {
        // RSSI ranges from -100 (weak) to -30 (strong)
        // Normalize to 0.0 to 1.0 range
        val normalized = when {
            rssi >= -30 -> 1.0f
            rssi <= -100 -> 0.0f
            else -> (rssi + 100) / 70.0f
        }
        return normalized.coerceIn(0.0f, 1.0f)
    }
    
    // Refactoring: Add methods to access scan statistics and device quality
    fun getSignalQuality(deviceId: String): Float? = signalQualityCache[deviceId]
    
    fun getScanStatistics(): ScanStatistics {
        val scanDuration = if (scanStartTime > 0) {
            System.currentTimeMillis() - scanStartTime
        } else 0L
        
        return ScanStatistics(
            scanDuration = scanDuration,
            totalDevicesDiscovered = totalDevicesDiscovered,
            uniqueDevices = deviceCache.size,
            isScanning = isScanning
        )
    }
    
    fun clearCache() {
        deviceCache.clear()
        signalQualityCache.clear()
        discoveredDevices.clear()
        totalDevicesDiscovered = 0
    }
}

// Refactoring: Add scan statistics data class
data class ScanStatistics(
    val scanDuration: Long,
    val totalDevicesDiscovered: Int,
    val uniqueDevices: Int,
    val isScanning: Boolean
)

// Extension function to convert Bluetooth device type to DeviceType
fun fromBluetoothDeviceType(bluetoothType: Int): DeviceType {
    return DeviceType.NEARCLIP // For now, assume all discovered devices are NearClip devices
}