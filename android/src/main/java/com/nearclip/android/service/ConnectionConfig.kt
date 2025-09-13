package com.nearclip.android.service

data class ConnectionConfig(
    val maxConnections: Int,
    val connectionTimeout: Long,
    val reconnectAttempts: Int,
    val switchThreshold: Float,
    val enableAutoSwitch: Boolean,
    val pingInterval: Long,
    val monitorInterval: Long
) {
    companion object {
        fun default() = ConnectionConfig(
            maxConnections = 10,
            connectionTimeout = 15000L,
            reconnectAttempts = 3,
            switchThreshold = 0.6f,
            enableAutoSwitch = true,
            pingInterval = 5000L,
            monitorInterval = 10000L
        )
    }
}

data class ConnectionPool(
    val maxConnections: Int,
    private val activeConnections: MutableMap<String, DeviceConnection> = mutableMapOf()
) {
    fun addConnection(deviceId: String, connection: DeviceConnection): Boolean {
        if (activeConnections.size >= maxConnections) {
            // 移除最旧的连接
            val oldestConnection = activeConnections.entries
                .minByOrNull { it.value.startTime }
            oldestConnection?.let {
                activeConnections.remove(it.key)
            }
        }
        activeConnections[deviceId] = connection
        return true
    }

    fun removeConnection(deviceId: String): DeviceConnection? {
        return activeConnections.remove(deviceId)
    }

    fun getConnection(deviceId: String): DeviceConnection? {
        return activeConnections[deviceId]
    }

    fun getAllConnections(): List<DeviceConnection> = activeConnections.values.toList()

    fun getOldestConnection(): DeviceConnection? {
        return activeConnections.values.minByOrNull { it.startTime }
    }

    fun clear() {
        activeConnections.clear()
    }
}

data class ConnectionQuality(
    val latency: Long,
    val packetLoss: Float,
    val throughput: Float,
    val stability: Float,
    val timestamp: Long = System.currentTimeMillis()
) {
    fun getOverallScore(): Float {
        val latencyScore = when {
            latency < 50 -> 1.0f
            latency < 100 -> 0.8f
            latency < 200 -> 0.6f
            latency < 500 -> 0.4f
            else -> 0.2f
        }

        val packetLossScore = 1.0f - packetLoss.coerceIn(0f, 1f)
        val throughputScore = (throughput / 1000f).coerceIn(0f, 1f)
        val stabilityScore = stability.coerceIn(0f, 1f)

        return (latencyScore * 0.3f + packetLossScore * 0.3f +
                throughputScore * 0.2f + stabilityScore * 0.2f)
    }
}