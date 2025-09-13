package com.nearclip.android.service

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

class DeviceConnectionManager(
    private val context: Context,
    private val connectivityManager: ConnectivityManager,
    private val connectionConfig: ConnectionConfig = ConnectionConfig.default()
) {
    private val connectionPool = ConnectionPool(connectionConfig.maxConnections)
    private val connectionStates = mutableMapOf<String, ConnectionState>()
    private val connectionEvents = MutableSharedFlow<ConnectionEvent>()
    private val connectionLock = Mutex()

    // 添加连接质量监控
    private val connectionMonitor = ConnectionMonitor()

    // 添加智能重连机制
    private val reconnectScheduler = ReconnectScheduler(connectionConfig)

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

    private val managerScope = CoroutineScope(Dispatchers.Main + SupervisorJob())

    init {
        registerNetworkCallback()
    }

    suspend fun connectToDevice(device: UnifiedDevice): Result<Unit> {
        return connectionLock.withLock {
            try {
                if (connectionStates[device.id] == ConnectionState.Connected) {
                    return Result.success(Unit)
                }

                updateConnectionState(device.id, ConnectionState.Connecting)

                // 智能选择传输方式
                val optimalTransport = selectOptimalTransport(device)

                val connectionResult = when (optimalTransport) {
                    TransportType.WIFI -> connectViaWiFi(device)
                    TransportType.BLE -> connectViaBLE(device)
                    else -> throw ConnectionError.NoTransportAvailable
                }

                if (connectionResult.isSuccess) {
                    val connection = DeviceConnection(
                        device = device,
                        transport = optimalTransport,
                        state = ConnectionState.Connected
                    )

                    connectionPool.addConnection(device.id, connection)
                    updateConnectionState(device.id, ConnectionState.Connected)
                    connectionEvents.emit(ConnectionEvent.Connected(device.id, optimalTransport))

                    // 启动连接监控
                    startConnectionMonitoring(connection)

                    Result.success(Unit)
                } else {
                    // 尝试回退机制
                    attemptFallbackConnection(device, optimalTransport)
                }

            } catch (e: Exception) {
                updateConnectionState(device.id, ConnectionState.Failed)
                connectionEvents.emit(ConnectionEvent.Failed(device.id, e.message ?: "Unknown error"))

                // 如果启用自动重连，安排重连
                if (connectionConfig.reconnectAttempts > 0) {
                    reconnectScheduler.scheduleReconnect(device.id, this, device)
                }

                Result.failure(e)
            }
        }
    }

    private fun selectOptimalTransport(device: UnifiedDevice): TransportType {
        val networkAvailable = isNetworkAvailable()
        val currentQuality = connectionMonitor.getQualityScore(device.id)

        return when {
            // 如果当前连接质量良好，保持当前传输
            currentQuality > 0.8f && connectionPool.getConnection(device.id) != null ->
                connectionPool.getConnection(device.id)!!.transport

            // WiFi优先策略（当网络可用且质量好）
            device.transports.contains(TransportType.WIFI) && networkAvailable &&
                currentQuality < connectionConfig.switchThreshold ->
                TransportType.WIFI

            // BLE回退
            device.transports.contains(TransportType.BLE) ->
                TransportType.BLE

            else ->
                throw ConnectionError.NoTransportAvailable
        }
    }

    private suspend fun connectViaWiFi(device: UnifiedDevice): Result<Unit> {
        return withTimeout(connectionConfig.connectionTimeout) {
            try {
                val socket = createWiFiConnection(device)
                if (socket != null) {
                    Result.success(Unit)
                } else {
                    Result.failure(ConnectionError.ConnectionFailed("WiFi connection failed"))
                }
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    private suspend fun connectViaBLE(device: UnifiedDevice): Result<Unit> {
        return withTimeout(connectionConfig.connectionTimeout) {
            try {
                val gattConnection = createBLEConnection(device)
                if (gattConnection != null) {
                    Result.success(Unit)
                } else {
                    Result.failure(ConnectionError.ConnectionFailed("BLE connection failed"))
                }
            } catch (e: Exception) {
                Result.failure(e)
            }
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
                val connection = DeviceConnection(
                    device = device,
                    transport = transport,
                    state = ConnectionState.Connected
                )

                connectionPool.addConnection(device.id, connection)
                updateConnectionState(device.id, ConnectionState.Connected)
                connectionEvents.emit(ConnectionEvent.Connected(device.id, transport))

                // 启动连接监控
                startConnectionMonitoring(connection)

                return Result.success(Unit)
            }
        }

        return Result.failure(ConnectionError.AllTransportsFailed)
    }

    suspend fun disconnectFromDevice(deviceId: String): Result<Unit> {
        return connectionLock.withLock {
            try {
                val connection = connectionPool.getConnection(deviceId)
                if (connection != null) {
                    updateConnectionState(deviceId, ConnectionState.Disconnecting)

                    when (connection.transport) {
                        TransportType.WIFI -> disconnectWiFi(connection.device)
                        TransportType.BLE -> disconnectBLE(connection.device)
                    }

                    connectionPool.removeConnection(deviceId)
                    connectionMonitor.stopMonitoring(deviceId)
                    reconnectScheduler.cancelReconnect(deviceId)
                    updateConnectionState(deviceId, ConnectionState.Disconnected)
                    connectionEvents.emit(ConnectionEvent.Disconnected(deviceId))
                }

                Result.success(Unit)
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }

    fun getConnectionState(deviceId: String): ConnectionState {
        return connectionStates[deviceId] ?: ConnectionState.Disconnected
    }

    fun getActiveTransport(deviceId: String): TransportType? {
        return connectionPool.getConnection(deviceId)?.transport
    }

    fun getActiveConnections(): List<DeviceConnection> = connectionPool.getAllConnections()

    fun getConnectionEvents(): Flow<ConnectionEvent> = connectionEvents.asSharedFlow()

    fun getConnectionQuality(deviceId: String): ConnectionQuality? {
        return connectionMonitor.getConnectionQuality(deviceId)
    }

    private fun updateConnectionState(deviceId: String, state: ConnectionState) {
        connectionStates[deviceId] = state
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

    private fun registerNetworkCallback() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N && connectionConfig.enableAutoSwitch) {
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

    private fun handleNetworkLoss() {
        managerScope.launch {
            connectionPool.getAllConnections().forEach { connection ->
                if (connection.transport == TransportType.WIFI) {
                    // 尝试切换到BLE
                    attemptTransportSwitch(connection.device, TransportType.BLE)
                }
            }
        }
    }

    private fun handleNetworkRecovery() {
        managerScope.launch {
            connectionPool.getAllConnections().forEach { connection ->
                if (connection.transport == TransportType.BLE &&
                    connection.device.transports.contains(TransportType.WIFI)) {
                    // 评估是否切换回WiFi
                    if (shouldSwitchToWiFi(connection)) {
                        attemptTransportSwitch(connection.device, TransportType.WIFI)
                    }
                }
            }
        }
    }

    private fun shouldSwitchToWiFi(connection: DeviceConnection): Boolean {
        val qualityScore = connectionMonitor.getQualityScore(connection.device.id)
        val wifiAdvantage = calculateWifiAdvantage(connection.device)
        return qualityScore < connectionConfig.switchThreshold || wifiAdvantage > 0.7f
    }

    private fun calculateWifiAdvantage(device: UnifiedDevice): Float {
        // 基于设备特性和网络状况计算WiFi优势
        return when {
            device.type == DeviceType.DESKTOP || device.type == DeviceType.LAPTOP -> 0.9f
            device.capabilities.contains(DeviceCapability.FILE_TRANSFER) -> 0.8f
            else -> 0.6f
        }
    }

    private suspend fun attemptTransportSwitch(device: UnifiedDevice, newTransport: TransportType) {
        val currentConnection = connectionPool.getConnection(device.id)
        if (currentConnection == null || currentConnection.transport == newTransport) {
            return
        }

        try {
            // 先断开当前连接
            disconnectFromDevice(device.id)

            // 延迟后尝试新连接
            delay(1000)

            // 建立新连接
            val result = when (newTransport) {
                TransportType.WIFI -> connectViaWiFi(device)
                TransportType.BLE -> connectViaBLE(device)
                else -> Result.failure(ConnectionError.NoTransportAvailable)
            }

            if (result.isSuccess) {
                connectionEvents.emit(ConnectionEvent.TransportSwitched(device.id, newTransport))
            }
        } catch (e: Exception) {
            // 切换失败，尝试重新连接原始传输
            connectToDevice(device)
        }
    }

    private fun startConnectionMonitoring(connection: DeviceConnection) {
        managerScope.launch {
            connectionMonitor.startMonitoring(connection) { deviceId, quality ->
                // 根据连接质量进行自动切换
                if (connectionConfig.enableAutoSwitch && quality.getOverallScore() < connectionConfig.switchThreshold) {
                    val currentTransport = connectionPool.getConnection(deviceId)?.transport
                    val fallbackTransport = if (currentTransport == TransportType.WIFI) {
                        TransportType.BLE
                    } else {
                        TransportType.WIFI
                    }

                    if (connection.device.transports.contains(fallbackTransport)) {
                        attemptTransportSwitch(connection.device, fallbackTransport)
                    }
                }
            }
        }
    }

    // 模拟连接实现（实际实现需要具体的网络/蓝牙API）
    private suspend fun createWiFiConnection(device: UnifiedDevice): Any? {
        // TODO: 实现实际的WiFi连接逻辑
        delay(1000) // 模拟连接时间
        return "WiFi Socket"
    }

    private suspend fun createBLEConnection(device: UnifiedDevice): Any? {
        // TODO: 实现实际的BLE连接逻辑
        delay(1500) // 模拟连接时间
        return "BLE GATT Connection"
    }

    private suspend fun disconnectWiFi(device: UnifiedDevice) {
        // TODO: 实现WiFi断开逻辑
        delay(500)
    }

    private suspend fun disconnectBLE(device: UnifiedDevice) {
        // TODO: 实现BLE断开逻辑
        delay(500)
    }

    fun cleanup() {
        unregisterNetworkCallback()
        reconnectScheduler.clearAll()
        connectionPool.clear()
        managerScope.cancel()
    }

    // Test helper methods
    private val failedTransports = mutableSetOf<TransportType>()

    fun simulateConnectionFailure(transport: TransportType) {
        failedTransports.add(transport)
    }

    fun clearSimulatedFailures() {
        failedTransports.clear()
    }
}