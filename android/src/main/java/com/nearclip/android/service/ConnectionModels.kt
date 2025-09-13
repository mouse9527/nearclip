package com.nearclip.android.service

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

sealed class ConnectionError : Exception() {
    object NoTransportAvailable : ConnectionError()
    object AllTransportsFailed : ConnectionError()
    object ConnectionTimeout : ConnectionError()
    data class ConnectionFailed(val cause: String) : ConnectionError()
}