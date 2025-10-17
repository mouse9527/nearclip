package com.nearclip.data.network.proto

import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.model.DeviceType
import kotlinx.serialization.Serializable

/**
 * Protocol Buffers设备数据模型
 * 用于跨平台数据序列化和反序列化
 */

@Serializable
data class DeviceProto(
    val deviceId: String,
    val deviceName: String,
    val deviceType: String,
    val publicKey: String,
    val lastSeen: Long,
    val connectionStatus: String,
    val metadata: Map<String, String> = emptyMap()
)

/**
 * Protocol Buffers消息管理器
 * 负责序列化和反序列化设备数据
 */
object DeviceProtoManager {

    /**
     * 将Device对象转换为Proto格式
     */
    fun toProto(device: Device): DeviceProto {
        return DeviceProto(
            deviceId = device.deviceId,
            deviceName = device.deviceName,
            deviceType = device.deviceType.name,
            publicKey = device.publicKey,
            lastSeen = device.lastSeen,
            connectionStatus = device.connectionStatus.name,
            metadata = mapOf(
                "version" to "1.0",
                "platform" to "android"
            )
        )
    }

    /**
     * 将Proto格式转换为Device对象
     */
    fun fromProto(proto: DeviceProto): Device {
        return try {
            Device(
                deviceId = proto.deviceId,
                deviceName = proto.deviceName,
                deviceType = DeviceType.valueOf(proto.deviceType),
                publicKey = proto.publicKey,
                lastSeen = proto.lastSeen,
                connectionStatus = ConnectionStatus.valueOf(proto.connectionStatus)
            )
        } catch (e: IllegalArgumentException) {
            // 如果枚举值无效，使用默认值
            Device(
                deviceId = proto.deviceId,
                deviceName = proto.deviceName,
                deviceType = DeviceType.UNKNOWN,
                publicKey = proto.publicKey,
                lastSeen = proto.lastSeen,
                connectionStatus = ConnectionStatus.DISCONNECTED
            )
        }
    }

    /**
     * 将DeviceProto序列化为字节数组
     */
    fun serialize(proto: DeviceProto): ByteArray {
        return try {
            // 这里将来会使用真正的Protocol Buffers序列化
            // 目前使用简单的序列化作为占位符
            proto.toString().toByteArray()
        } catch (e: Exception) {
            throw ProtoException("序列化失败: ${e.message}")
        }
    }

    /**
     * 从字节数组反序列化DeviceProto
     */
    fun deserialize(data: ByteArray): DeviceProto {
        return try {
            // 这里将来会使用真正的Protocol Buffers反序列化
            // 目前使用简单的反序列化作为占位符
            val jsonString = String(data)
            parseFromJson(jsonString)
        } catch (e: Exception) {
            throw ProtoException("反序列化失败: ${e.message}")
        }
    }

    /**
     * 验证Proto数据的完整性
     */
    fun validate(proto: DeviceProto): Boolean {
        return try {
            proto.deviceId.isNotEmpty() &&
            proto.deviceName.isNotEmpty() &&
            proto.publicKey.isNotEmpty() &&
            proto.lastSeen > 0
        } catch (e: Exception) {
            false
        }
    }

    /**
     * 创建设备发现消息
     */
    fun createDiscoveryMessage(): DeviceProto {
        return DeviceProto(
            deviceId = "discovery",
            deviceName = "Device Discovery",
            deviceType = DeviceType.UNKNOWN.name,
            publicKey = "",
            lastSeen = System.currentTimeMillis(),
            connectionStatus = ConnectionStatus.DISCONNECTED.name,
            metadata = mapOf(
                "messageType" to "discovery",
                "timestamp" to System.currentTimeMillis().toString()
            )
        )
    }

    /**
     * 从JSON字符串解析Proto对象（占位符实现）
     */
    private fun parseFromJson(jsonString: String): DeviceProto {
        // 这是一个简化的解析实现
        // 真实的实现会使用Protocol Buffers
        return DeviceProto(
            deviceId = "mock-id",
            deviceName = "Mock Device",
            deviceType = DeviceType.ANDROID.name,
            publicKey = "mock-key",
            lastSeen = System.currentTimeMillis(),
            connectionStatus = ConnectionStatus.DISCONNECTED.name
        )
    }
}

/**
 * Protocol Buffers异常类
 */
class ProtoException(message: String, cause: Throwable? = null) : Exception(message, cause)

/**
 * 剪贴板数据Proto
 */
@Serializable
data class ClipboardDataProto(
    val deviceId: String,
    val data: String,
    val timestamp: Long,
    val dataType: String,
    val encrypted: Boolean = false
)

/**
 * 连接请求Proto
 */
@Serializable
data class ConnectionRequestProto(
    val deviceId: String,
    val publicKey: String,
    val timestamp: Long,
    val requestType: String
)