# Task 00108c: Android WiFi 设备发现核心

## 任务描述

实现Android平台的WiFi设备发现核心功能，专注于mDNS/UDP广播方式的NearClip设备发现。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class WiFiDiscoveryManagerTest {
    @Test
    fun testWiFiDiscoveryStartStop() {
        // RED: 测试WiFi发现启动停止
        val manager = WiFiDiscoveryManager(mockContext, mockConnectivityManager)
        
        assertFalse(manager.isActive())
        
        runBlocking {
            val result = manager.startDiscovery()
            assertTrue(result.isSuccess)
            assertTrue(manager.isActive())
            
            val stopResult = manager.stopDiscovery()
            assertTrue(stopResult.isSuccess)
            assertFalse(manager.isActive())
        }
    }

    @Test
    fun testWiFiDeviceDiscovery() {
        // RED: 测试WiFi设备发现
        val manager = WiFiDiscoveryManager(mockContext, mockConnectivityManager)
        
        runBlocking {
            val devices = mutableListOf<DiscoveredDevice>()
            val job = launch {
                manager.startDiscovery().collect { device ->
                    devices.add(device)
                }
            }
            
            // 模拟发现设备
            manager.addMockDevice(mockWiFiDevice)
            
            delay(100)
            job.cancel()
            
            assertEquals(1, devices.size)
            assertEquals("mock-device-ip", devices[0].id)
        }
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class WiFiDiscoveryManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager
) {
    private var isActive = false
    private val nsdManager: NsdManager by lazy {
        context.getSystemService(Context.NSD_SERVICE) as NsdManager
    }
    
    suspend fun startDiscovery(): Flow<DiscoveredDevice> = callbackFlow {
        if (!isNetworkAvailable()) {
            close(WiFiDiscoveryError.NetworkNotAvailable)
            return@callbackFlow
        }
        
        val discoveryListener = object : NsdManager.DiscoveryListener {
            override fun onDiscoveryStarted(regType: String) {
                isActive = true
            }
            
            override fun onServiceFound(service: NsdServiceInfo) {
                if (isNearClipService(service)) {
                    nsdManager.resolveService(service, resolveListener)
                }
            }
            
            override fun onServiceLost(service: NsdServiceInfo) {
                // 处理服务丢失
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
                // 处理停止失败
            }
        }
        
        val resolveListener = object : NsdManager.ResolveListener {
            override fun onServiceResolved(service: NsdServiceInfo) {
                val device = DiscoveredDevice(
                    id = service.host.hostAddress ?: return,
                    name = service.serviceName,
                    type = DeviceType.Network,
                    transport = TransportType.WiFi,
                    port = service.port,
                    lastSeen = System.currentTimeMillis(),
                    attributes = service.attributes
                )
                
                trySend(device)
            }
            
            override fun onResolveFailed(serviceInfo: NsdServiceInfo, errorCode: Int) {
                // 处理解析失败
            }
        }
        
        try {
            nsdManager.discoverServices(
                NearClipServiceType,
                NsdManager.PROTOCOL_DNS_SD,
                discoveryListener
            )
        } catch (e: Exception) {
            close(WiFiDiscoveryError.DiscoveryFailed(-1))
        }
        
        awaitClose {
            if (isActive) {
                nsdManager.stopServiceDiscovery(discoveryListener)
            }
        }
    }
    
    suspend fun stopDiscovery() {
        // Flow会自动处理停止
    }
    
    fun isActive(): Boolean = isActive
    
    private fun isNetworkAvailable(): Boolean {
        val network = connectivityManager.activeNetworkInfo
        return network != null && network.isConnected
    }
    
    private fun isNearClipService(service: NsdServiceInfo): Boolean {
        return service.serviceType == NearClipServiceType
    }
    
    private fun removeDevice(serviceName: String) {
        // 从设备列表中移除
    }
}

data class DiscoveredDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transport: TransportType,
    val port: Int,
    val lastSeen: Long,
    val attributes: Map<String, ByteArray>
)

enum class WiFiDiscoveryError {
    NetworkNotAvailable,
    DiscoveryFailed(Int),
    PermissionDenied,
    ServiceResolutionFailed
}
```

### REFACTOR阶段
```kotlin
class WiFiDiscoveryManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager,
    private val discoveryConfig: WiFiDiscoveryConfig = WiFiDiscoveryConfig.default()
) {
    // 添加UDP广播支持
    // 添加网络状态监控
    // 添加设备缓存和超时处理
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
    }
}
```

## 验收标准
- [ ] WiFi发现服务正常启动和停止
- [ ] 能够通过mDNS发现NearClip设备
- [ ] 正确处理网络状态变化
- [ ] 设备信息格式化正确
- [ ] 错误处理机制完善

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108a: Android BLE 设备发现核心](00108a-android-ble-discovery-core.md)

## 后续任务
- [Task 00108c: Android 权限管理](00108c-android-permission-management.md)