package com.nearclip.android.service

data class UnifiedDiscoveryConfig(
    val wifiEnabled: Boolean,
    val bleEnabled: Boolean,
    val discoveryTimeout: Long,
    val enableSmartSwitching: Boolean,
    val maxDevices: Int
) {
    companion object {
        fun default() = UnifiedDiscoveryConfig(
            wifiEnabled = true,
            bleEnabled = true,
            discoveryTimeout = 30000L,
            enableSmartSwitching = true,
            maxDevices = 20
        )
    }
}

enum class DiscoveryStrategy {
    WIFI_PRIMARY_BLE_SECONDARY,
    BLE_PRIMARY_WIFI_SECONDARY,
    BLE_ONLY,
    WIFI_ONLY,
    NONE
}