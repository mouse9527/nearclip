# Task 00108i: Android 设备连接管理

## 任务描述

实现Android平台设备连接管理功能，建立BLE和WiFi设备连接，支持自动传输选择和连接切换。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class DeviceConnectionManagerTest {
    @Test
    fun testDeviceConnection() {
        // RED: 测试设备连接
        val manager = DeviceConnectionManager(mockContext)
        val device = createMockUnifiedDevice()
        
        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            assertEquals(ConnectionState.Connected, manager.getConnectionState(device.id))
        }
    }

    @Test
    fun testAutomaticTransportSelection() {
        // RED: 测试自动传输选择
        val manager = DeviceConnectionManager(mockContext)
        val device = createMockUnifiedDevice(
            transports = setOf(TransportType.WIFI, TransportType.BLE)
        )
        
        // 模拟WiFi网络可用
        `when`(mockNetworkInfo.isConnected).thenReturn(true)
        
        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            // 应选择WiFi作为优先传输
            assertEquals(TransportType.WIFI, manager.getActiveTransport(device.id))
        }
    }

    @Test
    fun testTransportFallback() {
        // RED: 测试传输回退机制
        val manager = DeviceConnectionManager(mockContext)
        val device = createMockUnifiedDevice(
            transports = setOf(TransportType.WIFI, TransportType.BLE)
        )
        
        // 先尝试WiFi连接（模拟失败）
        manager.simulateConnectionFailure(TransportType.WIFI)
        
        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            // 应自动回退到BLE
            assertEquals(TransportType.BLE, manager.getActiveTransport(device.id))
        }
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class DeviceConnectionManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager
) {
    private val activeConnections = mutableMapOf<String, DeviceConnection>()
    private val connectionStates = mutableMapOf<String, ConnectionState>()
    private val connectionEvents = MutableSharedFlow<ConnectionEvent>()
    
    suspend fun connectToDevice(device: UnifiedDevice): Result<Unit> {
        return try {
            updateConnectionState(device.id, ConnectionState.Connecting)
            
            // 智能选择传输方式
            val optimalTransport = selectOptimalTransport(device)
            
            val connectionResult = when (optimalTransport) {
                TransportType.WIFI -> connectViaWiFi(device)
                TransportType.BLE -> connectViaBLE(device)
                else -> throw ConnectionError.NoTransportAvailable
            }
            
            if (connectionResult.isSuccess) {
                activeConnections[device.id] = DeviceConnection(
                    device = device,
                    transport = optimalTransport,
                    state = ConnectionState.Connected
                )
                updateConnectionState(device.id, ConnectionState.Connected)
                connectionEvents.emit(ConnectionEvent.Connected(device.id, optimalTransport))
            } else {
                // 尝试回退机制
                attemptFallbackConnection(device, optimalTransport)
            }
            
            Result.success(Unit)
        } catch (e: Exception) {
            updateConnectionState(device.id, ConnectionState.Failed)
            connectionEvents.emit(ConnectionEvent.Failed(device.id, e.message ?: "Unknown error"))
            Result.failure(e)
        }
    }
    
    private fun selectOptimalTransport(device: UnifiedDevice): TransportType {
        // WiFi优先策略
        if (device.transports.contains(TransportType.WIFI) && isNetworkAvailable()) {
            return TransportType.WIFI
        }
        
        // BLE回退
        if (device.transports.contains(TransportType.BLE)) {
            return TransportType.BLE
        }
        
        throw ConnectionError.NoTransportAvailable
    }
    
    private suspend fun connectViaWiFi(device: UnifiedDevice): Result<Unit> {
        return try {
            // 实现WiFi连接逻辑
            val socket = createWiFiConnection(device)
            // 建立连接通道
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    private suspend fun connectViaBLE(device: UnifiedDevice): Result<Unit> {
        return try {
            // 实现BLE连接逻辑
            val gattConnection = createBLEConnection(device)
            // 建立GATT连接
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    private suspend fun attemptFallbackConnection(
        device: UnifiedDevice, 
        failedTransport: TransportType
    ): Result<Unit> {
        val fallbackTransports = device.transports - failedTransport
        
        for (transport in fallbackTransports) {
            val result = when (transport) {
                TransportType.WIFI -> connectViaWiFi(device)
                TransportType.BLE -> connectViaBLE(device)
                else -> continue
            }
            
            if (result.isSuccess) {
                activeConnections[device.id] = DeviceConnection(
                    device = device,
                    transport = transport,
                    state = ConnectionState.Connected
                )
                updateConnectionState(device.id, ConnectionState.Connected)
                connectionEvents.emit(ConnectionEvent.Connected(device.id, transport))
                return Result.success(Unit)
            }
        }
        
        return Result.failure(ConnectionError.AllTransportsFailed)
    }
    
    fun getConnectionState(deviceId: String): ConnectionState {
        return connectionStates[deviceId] ?: ConnectionState.Disconnected
    }
    
    fun getActiveTransport(deviceId: String): TransportType? {
        return activeConnections[deviceId]?.transport
    }
    
    fun getConnectionEvents(): Flow<ConnectionEvent> = connectionEvents.asSharedFlow()
    
    private fun updateConnectionState(deviceId: String, state: ConnectionState) {
        connectionStates[deviceId] = state
    }
}

data class DeviceConnection(
    val device: UnifiedDevice,
    val transport: TransportType,
    val state: ConnectionState,
    val startTime: Long = System.currentTimeMillis()
)

enum class ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
    Disconnecting
}

sealed class ConnectionEvent {
    data class Connected(val deviceId: String, val transport: TransportType) : ConnectionEvent()
    data class Disconnected(val deviceId: String) : ConnectionEvent()
    data class Failed(val deviceId: String, val error: String) : ConnectionEvent()
    data class TransportSwitched(val deviceId: String, val newTransport: TransportType) : ConnectionEvent()
}
```

### REFACTOR阶段
```kotlin
class DeviceConnectionManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager,
    private val connectionConfig: ConnectionConfig = ConnectionConfig.default()
) {
    // 添加连接质量监控
    private val connectionMonitor = ConnectionMonitor()
    
    // 添加智能重连机制
    private val reconnectScheduler = ReconnectScheduler()
    
    // 添加连接池管理
    private val connectionPool = ConnectionPool(maxConnections = connectionConfig.maxConnections)
    
    // 添加网络状态监听
    private val networkCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onLost(network: Network) {
            // 网络丢失时切换到BLE
            handleNetworkLoss()
        }
        
        override fun onAvailable(network: Network) {
            // 网络恢复时评估是否切换回WiFi
            handleNetworkRecovery()
        }
    }
    
    private fun handleNetworkLoss() {
        activeConnections.values.forEach { connection ->
            if (connection.transport == TransportType.WIFI) {
                // 尝试切换到BLE
                attemptTransportSwitch(connection.device, TransportType.BLE)
            }
        }
    }
    
    private fun handleNetworkRecovery() {
        activeConnections.values.forEach { connection ->
            if (connection.transport == TransportType.BLE && 
                connection.device.transports.contains(TransportType.WIFI)) {
                // 评估是否切换回WiFi
                if (shouldSwitchToWiFi(connection)) {
                    attemptTransportSwitch(connection.device, TransportType.WIFI)
                }
            }
        }
    }
    
    private fun shouldSwitchToWiFi(connection: DeviceConnection): Boolean {
        val qualityScore = connectionMonitor.getQualityScore(connection.device.id)
        val wifiAdvantage = calculateWifiAdvantage(connection.device)
        return qualityScore < connectionConfig.switchThreshold || wifiAdvantage > 0.7f
    }
}

data class ConnectionConfig(
    val maxConnections: Int,
    val connectionTimeout: Long,
    val reconnectAttempts: Int,
    val switchThreshold: Float,
    val enableAutoSwitch: Boolean,
    val pingInterval: Long
) {
    companion object {
        fun default() = ConnectionConfig(
            maxConnections = 10,
            connectionTimeout = 15000L,
            reconnectAttempts = 3,
            switchThreshold = 0.6f,
            enableAutoSwitch = true,
            pingInterval = 5000L
        )
    }
}
```

## 验收标准
- [ ] 能够通过WiFi和BLE建立设备连接
- [ ] 自动选择最优传输方式
- [ ] 连接失败时自动回退到其他传输方式
- [ ] 网络状态变化时智能切换传输方式
- [ ] 连接状态实时更新
- [ ] 连接质量监控和评分工作正常
- [ ] 重连机制工作正常

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108h: Android 统一设备发现管理](00108h-android-unified-discovery.md)
- [Task 00108f: Android 统一设备发现UI](00108f-android-discovery-ui.md)

## 后续任务
- [Task 00108j: Android 数据同步协议](00108j-android-data-sync-protocol.md)