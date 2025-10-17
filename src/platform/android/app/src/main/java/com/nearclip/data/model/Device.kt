package com.nearclip.data.model

import androidx.room.Entity
import androidx.room.PrimaryKey

/**
 * 设备数据模型
 */
@Entity(tableName = "devices")
data class Device(
    @PrimaryKey
    val deviceId: String,
    val deviceName: String,
    val deviceType: DeviceType,
    val publicKey: String,
    val lastSeen: Long,
    val connectionStatus: ConnectionStatus,
    val isPaired: Boolean = false,
    val createdAt: Long = System.currentTimeMillis(),
    val updatedAt: Long = System.currentTimeMillis()
) {
    /**
     * 检查设备是否已连接
     */
    val isConnected: Boolean
        get() = connectionStatus == ConnectionStatus.CONNECTED

    /**
     * 检查设备是否可连接
     */
    val isConnectable: Boolean
        get() = connectionStatus == ConnectionStatus.DISCONNECTED || connectionStatus == ConnectionStatus.PAIRING

    override fun toString(): String {
        return "Device(id=$deviceId, name=$deviceName, type=$deviceType, status=$connectionStatus)"
    }
}

/**
 * 设备类型枚举
 */
enum class DeviceType(val value: String) {
    ANDROID("ANDROID"),
    MAC("MAC"),
    WINDOWS("WINDOWS"),
    LINUX("LINUX"),
    UNKNOWN("UNKNOWN");

    companion object {
        fun fromString(value: String): DeviceType {
            return values().find { it.value == value } ?: UNKNOWN
        }
    }
}

/**
 * 连接状态枚举
 */
enum class ConnectionStatus(val value: String) {
    DISCONNECTED("DISCONNECTED"),
    CONNECTING("CONNECTING"),
    CONNECTED("CONNECTED"),
    PAIRING("PAIRING"),
    ERROR("ERROR");

    companion object {
        fun fromString(value: String): ConnectionStatus {
            return values().find { it.value == value } ?: DISCONNECTED
        }
    }
}