package com.nearclip.android.service

data class UnifiedDevice(
    val id: String,
    val name: String,
    val type: DeviceType,
    val transports: Set<TransportType>,
    val quality: Float,
    val lastSeen: Long,
    val attributes: Map<String, Any>
)

enum class TransportType {
    WIFI,
    BLE,
    HYBRID
}