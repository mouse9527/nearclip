package com.nearclip.android.service

import android.bluetooth.BluetoothAdapter
import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.os.Build
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.util.*

class UnifiedDiscoveryManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val connectivityManager: ConnectivityManager,
    private val discoveryConfig: UnifiedDiscoveryConfig = UnifiedDiscoveryConfig.default()
) {
    private val bleScanner = BLEScannerManager(context, bluetoothAdapter)
    private val wifiDiscovery = WiFiDiscoveryManager(context, connectivityManager)
    private val discoveredDevices = mutableMapOf<String, UnifiedDevice>()
    private val deviceFlow = MutableSharedFlow<UnifiedDevice>()
    private val discoveryLock = Mutex()

    private var currentStrategy = DiscoveryStrategy.NONE
    private var isDiscovering = false

    // 网络状态监控
    private val networkCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            // 网络可用时切换到WiFi优先模式
            if (discoveryConfig.enableSmartSwitching) {
                updateDiscoveryStrategy()
            }
        }

        override fun onLost(network: Network) {
            // 网络丢失时切换到BLE模式
            if (discoveryConfig.enableSmartSwitching) {
                updateDiscoveryStrategy()
            }
        }
    }

    suspend fun startDiscovery(): Flow<UnifiedDevice> = callbackFlow {
        discoveryLock.withLock {
            if (isDiscovering) return@callbackFlow
            isDiscovering = true
        }

        // 注册网络状态监控
        registerNetworkCallback()

        // 初始化发现策略
        updateDiscoveryStrategy()

        try {
            // 监听设备发现事件
            val discoveryJob = launch {
                deviceFlow.collect { device ->
                    send(device)
                }
            }

            awaitClose {
                discoveryJob.cancel()
                stopDiscovery()
            }
        } finally {
            unregisterNetworkCallback()
        }
    }

    private fun registerNetworkCallback() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            val request = NetworkRequest.Builder()
                .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
                .build()
            connectivityManager.registerNetworkCallback(request, networkCallback)
        }
    }

    private fun unregisterNetworkCallback() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            try {
                connectivityManager.unregisterNetworkCallback(networkCallback)
            } catch (e: Exception) {
                // 忽略异常
            }
        }
    }

    private fun updateDiscoveryStrategy() {
        val newStrategy = when {
            !discoveryConfig.wifiEnabled && !discoveryConfig.bleEnabled ->
                DiscoveryStrategy.NONE
            discoveryConfig.wifiEnabled && discoveryConfig.bleEnabled &&
                isNetworkAvailable() && bluetoothAdapter.isEnabled ->
                DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY
            discoveryConfig.bleEnabled && discoveryConfig.wifiEnabled &&
                bluetoothAdapter.isEnabled ->
                DiscoveryStrategy.BLE_PRIMARY_WIFI_SECONDARY
            discoveryConfig.bleEnabled && bluetoothAdapter.isEnabled ->
                DiscoveryStrategy.BLE_ONLY
            discoveryConfig.wifiEnabled && isNetworkAvailable() ->
                DiscoveryStrategy.WIFI_ONLY
            else ->
                DiscoveryStrategy.NONE
        }

        if (newStrategy != currentStrategy) {
            currentStrategy = newStrategy
            applyDiscoveryStrategy(newStrategy)
        }
    }

    private fun applyDiscoveryStrategy(strategy: DiscoveryStrategy) {
        // 停止当前发现
        stopAllDiscoveries()

        when (strategy) {
            DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY -> {
                launchPrimaryWiFiWithBLEBackup()
            }
            DiscoveryStrategy.BLE_PRIMARY_WIFI_SECONDARY -> {
                launchPrimaryBLEWithWiFiBackup()
            }
            DiscoveryStrategy.BLE_ONLY -> {
                launchBLEDiscovery()
            }
            DiscoveryStrategy.WIFI_ONLY -> {
                launchWiFiDiscovery()
            }
            DiscoveryStrategy.NONE -> {
                // 不进行发现
            }
        }
    }

    private suspend fun launchPrimaryWiFiWithBLEBackup() {
        // 主发现：WiFi，备用：BLE
        launchWiFiDiscovery()
        delay(5000) // 5秒后启动BLE作为备用
        if (currentStrategy == DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY) {
            launchBLEDiscovery()
        }
    }

    private suspend fun launchPrimaryBLEWithWiFiBackup() {
        // 主发现：BLE，备用：WiFi
        launchBLEDiscovery()
        delay(3000) // 3秒后启动WiFi作为备用
        if (currentStrategy == DiscoveryStrategy.BLE_PRIMARY_WIFI_SECONDARY) {
            launchWiFiDiscovery()
        }
    }

    private suspend fun launchWiFiDiscovery() {
        if (!discoveryConfig.wifiEnabled) return

        CoroutineScope(Dispatchers.IO).launch {
            wifiDiscovery.startDiscovery().collect { wifiDevice ->
                mergeDevice(wifiDevice, TransportType.WIFI)
            }
        }
    }

    private suspend fun launchBLEDiscovery() {
        if (!discoveryConfig.bleEnabled) return

        CoroutineScope(Dispatchers.IO).launch {
            bleScanner.startScan().collect { bleDevice ->
                mergeDevice(bleDevice, TransportType.BLE)
            }
        }
    }

    private fun stopAllDiscoveries() {
        bleScanner.stopScan()
        wifiDiscovery.stopDiscovery()
    }

    private suspend fun mergeDevice(device: Device, transport: TransportType) {
        discoveryLock.withLock {
            val deviceId = device.id

            // 检查设备数量限制
            if (discoveredDevices.size >= discoveryConfig.maxDevices &&
                !discoveredDevices.containsKey(deviceId)) {
                return@withLock
            }

            val existingDevice = discoveredDevices[deviceId]
            val unifiedDevice = if (existingDevice != null) {
                // 合并设备信息
                mergeDeviceAttributes(existingDevice, device, transport)
            } else {
                // 创建新设备
                createUnifiedDevice(device, transport)
            }

            discoveredDevices[deviceId] = unifiedDevice
            deviceFlow.emit(unifiedDevice)
        }
    }

    private fun mergeDeviceAttributes(existing: UnifiedDevice, newDevice: Device, transport: TransportType): UnifiedDevice {
        val newTransports = existing.transports + transport
        val newQuality = calculateDeviceQuality(existing, newDevice, transport)
        val newLastSeen = maxOf(existing.lastSeen, newDevice.lastSeen)
        val newAttributes = mergeAttributes(existing.attributes, newDevice, transport)

        return existing.copy(
            transports = newTransports,
            quality = newQuality,
            lastSeen = newLastSeen,
            attributes = newAttributes
        )
    }

    private fun createUnifiedDevice(device: Device, transport: TransportType): UnifiedDevice {
        val baseQuality = when (transport) {
            TransportType.WIFI -> 0.85f
            TransportType.BLE -> calculateSignalQuality(device.rssi ?: -50)
            else -> 0.5f
        }

        return UnifiedDevice(
            id = device.id,
            name = device.name,
            type = device.type,
            transports = setOf(transport),
            quality = baseQuality,
            lastSeen = device.lastSeen,
            attributes = createDeviceAttributes(device, transport)
        )
    }

    private fun calculateDeviceQuality(existing: UnifiedDevice, newDevice: Device, transport: TransportType): Float {
        val transportScore = calculateTransportScore(existing.transports + transport)
        val signalScore = calculateSignalQuality(newDevice.rssi ?: -50)
        val recencyScore = calculateRecencyScore(newDevice.lastSeen)
        val connectionScore = calculateConnectionScore(newDevice.batteryLevel)

        return (transportScore * 0.4f + signalScore * 0.3f +
                recencyScore * 0.2f + connectionScore * 0.1f)
    }

    private fun calculateTransportScore(transports: Set<TransportType>): Float {
        return when {
            transports.contains(TransportType.WIFI) && transports.contains(TransportType.BLE) -> 1.0f
            transports.contains(TransportType.WIFI) -> 0.8f
            transports.contains(TransportType.BLE) -> 0.6f
            else -> 0.3f
        }
    }

    private fun calculateSignalQuality(rssi: Int): Float {
        return when {
            rssi > -50 -> 1.0f
            rssi > -60 -> 0.9f
            rssi > -70 -> 0.7f
            rssi > -80 -> 0.5f
            rssi > -90 -> 0.3f
            else -> 0.1f
        }
    }

    private fun calculateRecencyScore(lastSeen: Long): Float {
        val timeDiff = System.currentTimeMillis() - lastSeen
        return when {
            timeDiff < 5000 -> 1.0f      // 5秒内
            timeDiff < 15000 -> 0.8f    // 15秒内
            timeDiff < 30000 -> 0.6f    // 30秒内
            timeDiff < 60000 -> 0.4f    // 1分钟内
            else -> 0.2f
        }
    }

    private fun calculateConnectionScore(batteryLevel: Int?): Float {
        return when (batteryLevel) {
            null -> 0.8f  // 无电池信息
            in 80..100 -> 1.0f
            in 50..79 -> 0.8f
            in 20..49 -> 0.6f
            else -> 0.4f
        }
    }

    private fun mergeAttributes(existing: Map<String, Any>, newDevice: Device, transport: TransportType): Map<String, Any> {
        val newAttributes = existing.toMutableMap()

        // 合并传输方式特定属性
        when (transport) {
            TransportType.BLE -> {
                newDevice.rssi?.let { newAttributes["rssi_ble"] = it }
                newDevice.batteryLevel?.let { newAttributes["battery_level"] = it }
            }
            TransportType.WIFI -> {
                newAttributes["connection_type"] = "wifi"
            }
        }

        newAttributes["last_seen_transport"] = transport.name
        newAttributes["update_timestamp"] = System.currentTimeMillis()

        return newAttributes
    }

    private fun createDeviceAttributes(device: Device, transport: TransportType): Map<String, Any> {
        val attributes = mutableMapOf<String, Any>()

        when (transport) {
            TransportType.BLE -> {
                device.rssi?.let { attributes["rssi_ble"] = it }
                device.batteryLevel?.let { attributes["battery_level"] = it }
                attributes["connection_type"] = "ble"
            }
            TransportType.WIFI -> {
                attributes["connection_type"] = "wifi"
            }
        }

        attributes["discovered_transport"] = transport.name
        attributes["initial_timestamp"] = System.currentTimeMillis()

        return attributes
    }

    private fun isNetworkAvailable(): Boolean {
        return try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                val network = connectivityManager.activeNetwork
                val capabilities = connectivityManager.getNetworkCapabilities(network)
                capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true
            } else {
                val networkInfo = connectivityManager.activeNetworkInfo
                networkInfo?.isConnected == true
            }
        } catch (e: Exception) {
            false
        }
    }

    private fun stopDiscovery() {
        discoveryLock.withLock {
            isDiscovering = false
            stopAllDiscoveries()
        }
    }

    // Public API methods
    fun getDiscoveredDevices(): List<UnifiedDevice> = discoveredDevices.values.toList()
    fun getCurrentStrategy(): DiscoveryStrategy = currentStrategy
    fun isDiscovering(): Boolean = isDiscovering

    // 保持测试兼容性
    fun selectOptimalTransport(): TransportType {
        return when {
            isNetworkAvailable() && bluetoothAdapter.isEnabled -> TransportType.WIFI
            bluetoothAdapter.isEnabled -> TransportType.BLE
            else -> throw Exception("No transport available")
        }
    }

    // Test helper methods
    suspend fun addMockDevice(device: Any, deviceId: String) {
        val unifiedDevice = when (device) {
            is BluetoothDevice -> UnifiedDevice(
                id = deviceId,
                name = device.name,
                type = DeviceType.PHONE,
                transports = setOf(TransportType.BLE),
                quality = 0.8f,
                lastSeen = System.currentTimeMillis(),
                attributes = emptyMap()
            )
            is WiFiDevice -> UnifiedDevice(
                id = deviceId,
                name = device.name,
                type = DeviceType.DESKTOP,
                transports = setOf(TransportType.WIFI),
                quality = 0.9f,
                lastSeen = System.currentTimeMillis(),
                attributes = emptyMap()
            )
            else -> throw IllegalArgumentException("Unknown device type")
        }

        deviceFlow.emit(unifiedDevice)
    }
}

// Mock device classes for testing
data class BluetoothDevice(val id: String, val name: String)
data class WiFiDevice(val id: String, val name: String)