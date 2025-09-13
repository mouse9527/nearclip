package com.nearclip.android.service

import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

class ConnectionMonitor {
    private val connectionQualities = mutableMapOf<String, ConnectionQuality>()
    private val monitorScope = CoroutineScope(Dispatchers.IO)
    private val qualityUpdates = MutableSharedFlow<Pair<String, ConnectionQuality>>()

    suspend fun startMonitoring(
        connection: DeviceConnection,
        onQualityUpdate: suspend (String, ConnectionQuality) -> Unit
    ) {
        monitorScope.launch {
            while (true) {
                val quality = measureConnectionQuality(connection)
                connectionQualities[connection.device.id] = quality
                qualityUpdates.emit(Pair(connection.device.id, quality))
                onQualityUpdate(connection.device.id, quality)
                delay(5000) // 每5秒测量一次
            }
        }
    }

    suspend fun stopMonitoring(deviceId: String) {
        connectionQualities.remove(deviceId)
    }

    fun getQualityScore(deviceId: String): Float {
        return connectionQualities[deviceId]?.getOverallScore() ?: 0f
    }

    fun getConnectionQuality(deviceId: String): ConnectionQuality? {
        return connectionQualities[deviceId]
    }

    fun getQualityUpdates(): Flow<Pair<String, ConnectionQuality>> = qualityUpdates.asSharedFlow()

    private suspend fun measureConnectionQuality(connection: DeviceConnection): ConnectionQuality {
        return when (connection.transport) {
            TransportType.WIFI -> measureWiFiQuality(connection)
            TransportType.BLE -> measureBLEQuality(connection)
            else -> ConnectionQuality(
                latency = 1000,
                packetLoss = 0.5f,
                throughput = 100f,
                stability = 0.5f
            )
        }
    }

    private suspend fun measureWiFiQuality(connection: DeviceConnection): ConnectionQuality {
        // TODO: 实现实际的WiFi质量测量
        return ConnectionQuality(
            latency = 50,
            packetLoss = 0.01f,
            throughput = 800f,
            stability = 0.95f
        )
    }

    private suspend fun measureBLEQuality(connection: DeviceConnection): ConnectionQuality {
        // TODO: 实现实际的BLE质量测量
        return ConnectionQuality(
            latency = 150,
            packetLoss = 0.05f,
            throughput = 50f,
            stability = 0.85f
        )
    }
}

class ReconnectScheduler(
    private val connectionConfig: ConnectionConfig
) {
    private val reconnectAttempts = mutableMapOf<String, Int>()
    private val reconnectJobs = mutableMapOf<String, Job>()
    private val schedulerScope = CoroutineScope(Dispatchers.IO)

    suspend fun scheduleReconnect(
        deviceId: String,
        connectionManager: DeviceConnectionManager,
        device: UnifiedDevice
    ) {
        val attempts = reconnectAttempts.getOrDefault(deviceId, 0)

        if (attempts >= connectionConfig.reconnectAttempts) {
            reconnectAttempts.remove(deviceId)
            return
        }

        reconnectAttempts[deviceId] = attempts + 1

        val job = schedulerScope.launch {
            val delayTime = calculateBackoffDelay(attempts)
            delay(delayTime)

            try {
                connectionManager.connectToDevice(device)
                reconnectAttempts.remove(deviceId)
            } catch (e: Exception) {
                // 重连失败，继续尝试
                scheduleReconnect(deviceId, connectionManager, device)
            }
        }

        reconnectJobs[deviceId] = job
    }

    fun cancelReconnect(deviceId: String) {
        reconnectJobs[deviceId]?.cancel()
        reconnectJobs.remove(deviceId)
        reconnectAttempts.remove(deviceId)
    }

    fun clearAll() {
        reconnectJobs.values.forEach { it.cancel() }
        reconnectJobs.clear()
        reconnectAttempts.clear()
    }

    private fun calculateBackoffDelay(attempt: Int): Long {
        return (1000L * kotlin.math.pow(2.0, attempt.toDouble())).toLong()
            .coerceAtMost(30000L) // 最大30秒
    }
}