package com.mouse.nearclip

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkInfo
import android.net.nsd.NsdManager
import android.net.nsd.NsdServiceInfo
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

data class WiFiDiscoveredDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transport: TransportType,
    val port: Int,
    val lastSeen: Long,
    val attributes: Map<String, ByteArray>
)

sealed class WiFiDiscoveryError(message: String) : Exception(message) {
    object NetworkNotAvailable : WiFiDiscoveryError("Network not available")
    data class DiscoveryFailed(val errorCode: Int) : WiFiDiscoveryError("Discovery failed with code: $errorCode")
    object PermissionDenied : WiFiDiscoveryError("WiFi discovery permission denied")
    object ServiceResolutionFailed : WiFiDiscoveryError("Service resolution failed")
}

data class WiFiDiscoveryConfig(
    val serviceType: String,
    val discoveryTimeout: Long,
    val enableMulticastDNS: Boolean,
    val enableUDPBroadcast: Boolean,
    val port: Int
) {
    companion object {
        fun default() = WiFiDiscoveryConfig(
            serviceType = "_nearclip._tcp",
            discoveryTimeout = 30000L,
            enableMulticastDNS = true,
            enableUDPBroadcast = true,
            port = 5353
        )
        
        fun aggressive() = WiFiDiscoveryConfig(
            serviceType = "_nearclip._tcp",
            discoveryTimeout = 10000L,
            enableMulticastDNS = true,
            enableUDPBroadcast = true,
            port = 5353
        )
        
        fun powerSaving() = WiFiDiscoveryConfig(
            serviceType = "_nearclip._tcp",
            discoveryTimeout = 60000L,
            enableMulticastDNS = true,
            enableUDPBroadcast = false,
            port = 5353
        )
    }
}

class WiFiDiscoveryManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager,
    private val discoveryConfig: WiFiDiscoveryConfig = WiFiDiscoveryConfig.default()
) {
    private var isActive = false
    private val discoveredDevices = mutableMapOf<String, WiFiDiscoveredDevice>()
    private val mockDeviceFlow = MutableSharedFlow<WiFiDiscoveredDevice>()
    private val scope = CoroutineScope(Dispatchers.Default)
    
    // Refactoring: Add device deduplication and caching
    private val deviceCache = mutableMapOf<String, WiFiDiscoveredDevice>()
    private val networkQualityCache = mutableMapOf<String, Float>()
    
    // Refactoring: Add discovery statistics
    private var discoveryStartTime = 0L
    private var totalDevicesDiscovered = 0
    
    private val nsdManager: NsdManager by lazy {
        context.getSystemService(Context.NSD_SERVICE) as NsdManager
    }
    
    suspend fun stopDiscovery(): Result<Unit> {
        isActive = false
        return Result.success(Unit)
    }
    
    fun isActive(): Boolean = isActive
    
    suspend fun startDiscovery(): Flow<WiFiDiscoveredDevice> = callbackFlow {
        if (!isNetworkAvailable()) {
            close(WiFiDiscoveryError.NetworkNotAvailable)
            return@callbackFlow
        }
        
        // Refactoring: Initialize discovery statistics
        discoveryStartTime = System.currentTimeMillis()
        totalDevicesDiscovered = 0
        
        // Handle mock devices if any
        val mockJob = scope.launch {
            mockDeviceFlow.collect { mockDevice ->
                trySend(mockDevice)
            }
        }
        
        // Real NSD discovery
        val discoveryListener = object : NsdManager.DiscoveryListener {
            override fun onDiscoveryStarted(regType: String) {
                isActive = true
            }
            
            override fun onServiceFound(service: NsdServiceInfo) {
                if (isNearClipService(service)) {
                    nsdManager.resolveService(service, object : NsdManager.ResolveListener {
                        override fun onServiceResolved(resolvedService: NsdServiceInfo) {
                            val device = createDiscoveredDevice(resolvedService)
                            
                            // Refactoring: Add device deduplication and network quality assessment
                            val deviceId = device.id
                            val networkQuality = calculateNetworkQuality(deviceId)
                            
                            // Update or create device
                            val discoveredDevice = deviceCache[deviceId]?.copy(
                                lastSeen = System.currentTimeMillis()
                            ) ?: device
                            
                            // Cache device and network quality
                            deviceCache[deviceId] = discoveredDevice
                            networkQualityCache[deviceId] = networkQuality
                            discoveredDevices[deviceId] = discoveredDevice
                            totalDevicesDiscovered++
                            
                            trySend(discoveredDevice)
                        }
                        
                        override fun onResolveFailed(serviceInfo: NsdServiceInfo, errorCode: Int) {
                            // Handle resolve failure
                        }
                    })
                }
            }
            
            override fun onServiceLost(service: NsdServiceInfo) {
                // Handle service lost
                removeDevice(service.serviceName)
            }
            
            override fun onDiscoveryStopped(serviceType: String) {
                isActive = false
                channel.close()
            }
            
            override fun onStartDiscoveryFailed(serviceType: String, errorCode: Int) {
                close(WiFiDiscoveryError.DiscoveryFailed(errorCode))
            }
            
            override fun onStopDiscoveryFailed(serviceType: String, errorCode: Int) {
                // Handle stop discovery failure
            }
        }
        
        try {
            nsdManager.discoverServices(
                discoveryConfig.serviceType,
                NsdManager.PROTOCOL_DNS_SD,
                discoveryListener
            )
        } catch (e: Exception) {
            close(WiFiDiscoveryError.DiscoveryFailed(-1))
        }
        
        awaitClose {
            if (isActive) {
                try {
                    nsdManager.stopServiceDiscovery(discoveryListener)
                } catch (e: Exception) {
                    // Ignore cleanup errors
                }
            }
            mockJob.cancel()
            isActive = false
        }
    }
    
    fun addMockDevice(device: Any) {
        // Create a mock device for testing
        val mockDevice = WiFiDiscoveredDevice(
            id = "mock-device-ip",
            name = "Mock WiFi Device",
            type = DeviceType.NEARCLIP,
            transport = TransportType.WIFI,
            port = 8080,
            lastSeen = System.currentTimeMillis(),
            attributes = mapOf()
        )
        
        // Use scope to send to flow
        scope.launch {
            mockDeviceFlow.emit(mockDevice)
        }
    }
    
    fun getDiscoveredDevices(): List<WiFiDiscoveredDevice> = discoveredDevices.values.toList()
    
    fun clearDiscoveredDevices() {
        clearCache()
    }
    
    private fun isNetworkAvailable(): Boolean {
        val network = connectivityManager.activeNetworkInfo
        return network != null && network.isConnected
    }
    
    private fun isNearClipService(service: NsdServiceInfo): Boolean {
        return service.serviceType == discoveryConfig.serviceType
    }
    
    private fun createDiscoveredDevice(service: NsdServiceInfo): WiFiDiscoveredDevice {
        return WiFiDiscoveredDevice(
            id = service.host.hostAddress ?: "unknown",
            name = service.serviceName,
            type = DeviceType.NEARCLIP,
            transport = TransportType.WIFI,
            port = service.port,
            lastSeen = System.currentTimeMillis(),
            attributes = service.attributes.mapValues { it.value }
        )
    }
    
    private fun removeDevice(serviceName: String) {
        // Remove from device lists
        discoveredDevices.remove(serviceName)
        deviceCache.remove(serviceName)
        networkQualityCache.remove(serviceName)
    }
    
    // Refactoring: Add network quality assessment
    private fun calculateNetworkQuality(deviceId: String): Float {
        // Simple network quality assessment based on discovery time
        // In a real implementation, this would use ping time, bandwidth, etc.
        return 0.8f // Default good quality
    }
    
    // Refactoring: Add methods to access discovery statistics and device quality
    fun getNetworkQuality(deviceId: String): Float? = networkQualityCache[deviceId]
    
    fun getDiscoveryStatistics(): WiFiDiscoveryStatistics {
        val discoveryDuration = if (discoveryStartTime > 0) {
            System.currentTimeMillis() - discoveryStartTime
        } else 0L
        
        return WiFiDiscoveryStatistics(
            discoveryDuration = discoveryDuration,
            totalDevicesDiscovered = totalDevicesDiscovered,
            uniqueDevices = deviceCache.size,
            isActive = isActive
        )
    }
    
    fun clearCache() {
        deviceCache.clear()
        networkQualityCache.clear()
        discoveredDevices.clear()
        totalDevicesDiscovered = 0
    }
}

// Refactoring: Add discovery statistics data class
data class WiFiDiscoveryStatistics(
    val discoveryDuration: Long,
    val totalDevicesDiscovered: Int,
    val uniqueDevices: Int,
    val isActive: Boolean
)