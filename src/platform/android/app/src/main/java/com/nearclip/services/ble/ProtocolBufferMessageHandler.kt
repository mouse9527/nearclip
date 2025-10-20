package com.nearclip.services.ble

import android.util.Log
import com.google.protobuf.InvalidProtocolBufferException
import nearclip.protocol.BLEMessage
import nearclip.protocol.ConnectionState
import nearclip.protocol.DeviceInfo
import nearclip.protocol.DeviceType
import nearclip.protocol.MessageType
import java.nio.ByteBuffer

/**
 * Protocol Buffers消息处理器
 * 统一Android和Mac端的消息序列化/反序列化实现
 */
class ProtocolBufferMessageHandler {

    companion object {
        private const val TAG = "ProtocolBufferHandler"
    }

    /**
     * 序列化消息为字节数组
     */
    fun serializeMessage(message: TestMessage): ByteArray {
        return try {
            val protoMessage = BLEMessage.newBuilder()
                .setMessageId(message.messageId)
                .setType(convertToProtoMessageType(message.type))
                .setPayload(message.payload)
                .setTimestamp(message.timestamp)
                .setSequenceNumber(message.sequenceNumber)
                .build()

            protoMessage.toByteArray()
        } catch (e: Exception) {
            Log.e(TAG, "序列化消息失败: ${e.message}", e)
            // 回退到旧的字符串序列化方式
            fallbackSerializeMessage(message)
        }
    }

    /**
     * 反序列化字节数组为消息
     */
    fun deserializeMessage(data: ByteArray): TestMessage? {
        return try {
            val protoMessage = BLEMessage.parseFrom(data)
            TestMessage(
                messageId = protoMessage.messageId,
                type = convertFromProtoMessageType(protoMessage.type),
                payload = protoMessage.payload,
                timestamp = protoMessage.timestamp,
                sequenceNumber = protoMessage.sequenceNumber
            )
        } catch (e: InvalidProtocolBufferException) {
            Log.w(TAG, "Protocol Buffers解析失败，尝试回退方式: ${e.message}")
            // 回退到旧的字符串反序列化方式
            fallbackDeserializeMessage(data)
        } catch (e: Exception) {
            Log.e(TAG, "反序列化消息失败: ${e.message}", e)
            null
        }
    }

    /**
     * 转换消息类型为Protocol Buffers枚举
     */
    private fun convertToProtoMessageType(type: MessageType): MessageTypeProto {
        return when (type) {
            MessageType.PING -> MessageTypeProto.MESSAGE_TYPE_PING
            MessageType.PONG -> MessageTypeProto.MESSAGE_TYPE_PONG
            MessageType.DATA -> MessageTypeProto.MESSAGE_TYPE_DATA
            MessageType.ACK -> MessageTypeProto.MESSAGE_TYPE_ACK
        }
    }

    /**
     * 从Protocol Buffers枚举转换消息类型
     */
    private fun convertFromProtoMessageType(type: MessageTypeProto): MessageType {
        return when (type) {
            MessageTypeProto.MESSAGE_TYPE_PING -> MessageType.PING
            MessageTypeProto.MESSAGE_TYPE_PONG -> MessageType.PONG
            MessageTypeProto.MESSAGE_TYPE_DATA -> MessageType.DATA
            MessageTypeProto.MESSAGE_TYPE_ACK -> MessageType.ACK
            MessageTypeProto.MESSAGE_TYPE_UNSPECIFIED -> MessageType.DATA // 默认值
            else -> MessageType.DATA // 未知类型默认为DATA
        }
    }

    /**
     * 回退序列化方式（向后兼容）
     */
    private fun fallbackSerializeMessage(message: TestMessage): ByteArray {
        val data = "${message.messageId}|${message.type}|${message.payload}|${message.timestamp}|${message.sequenceNumber}"
        return data.toByteArray(Charsets.UTF_8)
    }

    /**
     * 回退反序列化方式（向后兼容）
     */
    private fun fallbackDeserializeMessage(data: ByteArray): TestMessage? {
        return try {
            val messageString = String(data, Charsets.UTF_8)
            val parts = messageString.split("|")

            if (parts.size >= 5) {
                TestMessage(
                    messageId = parts[0],
                    type = MessageType.valueOf(parts[1]),
                    payload = parts[2],
                    timestamp = parts[3].toLongOrNull() ?: System.currentTimeMillis(),
                    sequenceNumber = parts[4].toIntOrNull() ?: 0
                )
            } else {
                null
            }
        } catch (e: Exception) {
            Log.e(TAG, "回退反序列化也失败: ${e.message}", e)
            null
        }
    }

    /**
     * 验证消息格式
     */
    fun validateMessage(message: TestMessage): Boolean {
        return try {
            // 基本字段验证
            message.messageId.isNotBlank() &&
            message.type != null &&
            message.timestamp > 0 &&
            message.sequenceNumber >= 0 &&
            message.payload.length <= 1024 // 限制载荷大小
        } catch (e: Exception) {
            Log.e(TAG, "消息验证失败: ${e.message}", e)
            false
        }
    }

    /**
     * 创建设备信息Proto消息
     */
    fun createDeviceInfoProto(device: BleDevice): ByteArray {
        return try {
            val protoDevice = DeviceInfo.newBuilder()
                .setDeviceId(device.deviceId)
                .setDeviceName(device.deviceName)
                .setDeviceType(if (device.deviceName.contains("Android", ignoreCase = true))
                    DeviceType.DEVICE_TYPE_ANDROID else DeviceType.DEVICE_TYPE_MAC)
                .setRssi(device.rssi)
                .setIsNearclipDevice(device.deviceName.contains("NearClip", ignoreCase = true))
                .build()

            protoDevice.toByteArray()
        } catch (e: Exception) {
            Log.e(TAG, "创建设备信息Proto失败: ${e.message}", e)
            ByteArray(0)
        }
    }
}

// 为了避免命名冲突，重命名Protocol Buffers生成的枚举
typealias MessageTypeProto = nearclip.protocol.MessageType