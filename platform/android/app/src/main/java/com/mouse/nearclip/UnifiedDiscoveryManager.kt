package com.mouse.nearclip

import kotlinx.coroutines.flow.*
import kotlinx.coroutines.*
import android.content.Context
import android.bluetooth.BluetoothAdapter
import android.net.ConnectivityManager

class UnifiedDiscoveryManager(
    private val context: Context,
    private val bluetoothAdapter: BluetoothAdapter,
    private val connectivityManager: ConnectivityManager
) {
    private lateinit var bleScannerManager: BLEScannerManager
    private lateinit var wifiDiscoveryManager: WiFiDiscoveryManager
    private val batteryOptimizationManager = BatteryOptimizationManager(context)
    
    private val _discoveredDevices = MutableStateFlow<List<DiscoveredDevice>>(emptyList())
    val discoveredDevices: StateFlow<List<DiscoveredDevice>> = _discoveredDevices.asStateFlow()
    
    private val _isScanning = MutableStateFlow(false)
    val isScanning: StateFlow<Boolean> = _isScanning.asStateFlow()
    
    private val _currentStrategy = MutableStateFlow(DiscoveryStrategy.Balanced)
    val currentStrategy: StateFlow<DiscoveryStrategy> = _currentStrategy.asStateFlow()
    
    private var discoveryJob: Job? = null
    private val deviceMap = mutableMapOf<String, UnifiedDevice>()
    
    data class UnifiedDevice(
        val id: String,
        val name: String,
        val type: DeviceType,
        val transports: Set<TransportType>,
        val rssi: Int,
        val wifiQuality: Float?,
        val lastSeen: Long
    ) {
        fun toDiscoveredDevice(): DiscoveredDevice {
            return DiscoveredDevice(
                id = id,
                name = name,
                type = type,
                transport = transports.firstOrNull() ?: TransportType.BLE,
                rssi = rssi,
                lastSeen = lastSeen
            )
        }
    }
    
    suspend fun startDiscovery() {
        if (discoveryJob?.isActive == true) return
        
        // Update strategy based on battery optimization
        val optimalStrategy = batteryOptimizationManager.getOptimalDiscoveryStrategy()
        _currentStrategy.value = optimalStrategy
        _isScanning.value = true
        
        discoveryJob = CoroutineScope(Dispatchers.Default).launch {
            // Start BLE discovery if applicable
            if (shouldUseBLE(optimalStrategy) && bluetoothAdapter.isEnabled) {
                startBLEDiscovery()
            }
            
            // Start WiFi discovery if applicable
            if (shouldUseWiFi(optimalStrategy)) {
                startWiFiDiscovery()
            }
        }
    }
    
    suspend fun stopDiscovery() {
        discoveryJob?.cancel()
        discoveryJob = null
        
        // Stop BLE discovery
        if (::bleScannerManager.isInitialized) {
            bleScannerManager.stopScan()
        }
        
        // Stop WiFi discovery
        if (::wifiDiscoveryManager.isInitialized) {
            wifiDiscoveryManager.stopDiscovery()
        }
        
        _isScanning.value = false
    }
    
    fun getDiscoveryState(): UnifiedDiscoveryState {
        return UnifiedDiscoveryState(
            isScanning = _isScanning.value,
            strategy = _currentStrategy.value,
            discoveredDevices = _discoveredDevices.value.sortedByDescending { 
                calculateSignalQuality(it.rssi) 
            }
        )
    }
    
    fun refreshDevices() {
        // Clear current devices and restart discovery
        deviceMap.clear()
        _discoveredDevices.value = emptyList()
        
        CoroutineScope(Dispatchers.Main).launch {
            stopDiscovery()
            delay(500) // Small delay before restarting
            startDiscovery()
        }
    }
    
    private fun shouldUseBLE(strategy: DiscoveryStrategy): Boolean {
        return when (strategy) {
            DiscoveryStrategy.Aggressive -> true
            DiscoveryStrategy.Balanced -> true
            DiscoveryStrategy.PowerSaving -> false
        }
    }
    
    private fun shouldUseWiFi(strategy: DiscoveryStrategy): Boolean {
        return when (strategy) {
            DiscoveryStrategy.Aggressive -> true
            DiscoveryStrategy.Balanced -> true
            DiscoveryStrategy.PowerSaving -> true // WiFi is generally more power-efficient than BLE
        }
    }
    
    private fun initializeBLEScanner() {
        if (!::bleScannerManager.isInitialized) {
            bleScannerManager = BLEScannerManager(context, bluetoothAdapter)
        }
    }
    
    private fun initializeWiFiScanner() {
        if (!::wifiDiscoveryManager.isInitialized) {
            wifiDiscoveryManager = WiFiDiscoveryManager(context, connectivityManager)
        }
    }
    
    private suspend fun startBLEDiscovery() {
        try {
            initializeBLEScanner()
            bleScannerManager.startScan()
            
            bleScannerManager.startScanFlow().collect { device ->
                updateBLEDevice(device)
            }
        } catch (e: Exception) {
            // Handle BLE discovery errors gracefully
            println("BLE discovery error: ${e.message}")
        }
    }
    
    private suspend fun startWiFiDiscovery() {
        try {
            initializeWiFiScanner()
            wifiDiscoveryManager.startDiscovery().collect { device ->
                updateWiFiDevice(device)
            }
        } catch (e: Exception) {
            // Handle WiFi discovery errors gracefully
            println("WiFi discovery error: ${e.message}")
        }
    }
    
    private fun updateBLEDevice(device: DiscoveredDevice) {
        updateDevice(
            deviceId = device.id,
            name = device.name,
            type = device.type,
            transport = TransportType.BLE,
            rssi = device.rssi,
            wifiQuality = null
        )
    }
    
    private fun updateWiFiDevice(device: WiFiDiscoveredDevice) {
        updateDevice(
            deviceId = device.id,
            name = device.name,
            type = device.type,
            transport = TransportType.WIFI,
            rssi = -50, // Default RSSI for WiFi devices
            wifiQuality = wifiDiscoveryManager.getNetworkQuality(device.id)
        )
    }
    
    private fun updateDevice(
        deviceId: String,
        name: String,
        type: DeviceType,
        transport: TransportType,
        rssi: Int,
        wifiQuality: Float?
    ) {
        val currentTime = System.currentTimeMillis()
        
        val existingDevice = deviceMap[deviceId]
        
        val updatedDevice = if (existingDevice != null) {
            // Merge with existing device
            existingDevice.copy(
                transports = existingDevice.transports + transport,
                rssi = maxOf(existingDevice.rssi, rssi),
                wifiQuality = if (transport == TransportType.WIFI) {
                    wifiQuality
                } else {
                    existingDevice.wifiQuality
                },
                lastSeen = maxOf(existingDevice.lastSeen, currentTime)
            )
        } else {
            // Create new unified device
            UnifiedDevice(
                id = deviceId,
                name = name,
                type = type,
                transports = setOf(transport),
                rssi = rssi,
                wifiQuality = wifiQuality,
                lastSeen = currentTime
            )
        }
        
        // Update device map and flow
        deviceMap[deviceId] = updatedDevice
        _discoveredDevices.value = deviceMap.values.map { it.toDiscoveredDevice() }
    }
    
    fun getDevicesByTransport(transport: TransportType): List<DiscoveredDevice> {
        return _discoveredDevices.value.filter { device ->
            val unifiedDevice = deviceMap[device.id]
            unifiedDevice?.transports?.contains(transport) ?: false
        }
    }
    
    fun getBatteryMetrics(): BatteryMetrics {
        return batteryOptimizationManager.getPerformanceMetrics()
    }
}