package com.nearclip.android.service

data class Device(
    val id: String,
    val name: String,
    val type: DeviceType,
    val capabilities: Set<DeviceCapability>,
    val transport: DeviceTransport,
    val lastSeen: Long = System.currentTimeMillis(),
    val rssi: Int? = null,
    val batteryLevel: Int? = null
)

enum class DeviceType {
    PHONE,
    TABLET,
    DESKTOP,
    LAPTOP,
    WATCH,
    TV,
    UNKNOWN
}

enum class DeviceCapability {
    CLIPBOARD_SYNC,
    FILE_TRANSFER,
    SCREEN_MIRRORING,
    REMOTE_CONTROL
}

enum class DeviceTransport {
    WIFI,
    BLE,
    HYBRID
}