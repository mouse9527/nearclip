package com.nearclip.data.protocol

import com.nearclip.data.network.proto.*
import com.google.protobuf.ByteString
import java.util.*

sealed class ProtocolError(message: String) : Exception(message) {
    class InvalidFormat(message: String) : ProtocolError(message)
    class UnsupportedVersion(message: String) : ProtocolError(message)
    class SignatureVerificationFailed : ProtocolError("消息签名验证失败")
    class CryptographicError(message: String) : ProtocolError(message)
    class NetworkError(message: String) : ProtocolError(message)
}

interface ProtocolHandler {
    fun handleMessage(message: ByteArray): Result<ByteArray, ProtocolError>
    fun validateMessage(message: ByteArray): Result<Unit, ProtocolError>
}

/**
 * 设备发现处理器
 */
class DiscoveryHandler : ProtocolHandler {

    fun handleBroadcast(broadcast: DeviceBroadcast): Result<Unit, ProtocolError> {
        broadcast.validate()
        println("收到设备广播: ${broadcast.deviceName} (${broadcast.deviceId})")
        return Result.success(Unit)
    }

    fun createScanRequest(timeoutSeconds: Int): ScanRequest {
        return ScanRequest.newBuilder()
            .setTimeoutSeconds(timeoutSeconds)
            .addAllFilterTypes(emptyList())
            .addAllRequiredCapabilities(emptyList())
            .build()
    }

    override fun handleMessage(message: ByteArray): Result<ByteArray, ProtocolError> {
        return try {
            val broadcast = DeviceBroadcast.parseFrom(message)
            handleBroadcast(broadcast)
            Result.success(ByteArray(0))
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("解析设备广播消息失败: ${e.message}"))
        }
    }

    override fun validateMessage(message: ByteArray): Result<Unit, ProtocolError> {
        return try {
            val broadcast = DeviceBroadcast.parseFrom(message)
            broadcast.validate()
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("验证设备广播消息失败: ${e.message}"))
        }
    }
}

/**
 * 配对处理器
 */
class PairingHandler : ProtocolHandler {

    fun initiatePairing(targetId: String, deviceName: String): PairingRequest {
        return PairingRequest.newBuilder()
            .setInitiatorId(getDeviceId())
            .setTargetId(targetId)
            .setDeviceName(deviceName)
            .setNonce(generateNonce())
            .setTimestamp(System.currentTimeMillis())
            .build()
    }

    fun handlePairingResponse(response: PairingResponse): Result<Unit, ProtocolError> {
        response.validate()
        println("配对响应来自: ${response.responderId}")
        return Result.success(Unit)
    }

    override fun handleMessage(message: ByteArray): Result<ByteArray, ProtocolError> {
        return try {
            val response = PairingResponse.parseFrom(message)
            handlePairingResponse(response)
            Result.success(ByteArray(0))
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("解析配对响应消息失败: ${e.message}"))
        }
    }

    override fun validateMessage(message: ByteArray): Result<Unit, ProtocolError> {
        return try {
            val response = PairingResponse.parseFrom(message)
            response.validate()
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("验证配对响应消息失败: ${e.message}"))
        }
    }

    private fun getDeviceId(): String {
        return UUID.randomUUID().toString()
    }

    private fun generateNonce(): ByteString {
        val random = Random()
        val nonce = ByteArray(32)
        random.nextBytes(nonce)
        return ByteString.copyFrom(nonce)
    }
}

/**
 * 同步处理器
 */
class SyncHandler : ProtocolHandler {

    fun createSyncMessage(data: ClipboardData, operation: SyncOperation): SyncMessage {
        return SyncMessage.newBuilder()
            .setDeviceId(getDeviceId())
            .setOperation(operation)
            .setData(data)
            .addAllChunks(emptyList())
            .setTimestamp(System.currentTimeMillis())
            .build()
    }

    fun handleSyncMessage(message: SyncMessage): Result<SyncAck, ProtocolError> {
        message.validate()

        val ack = SyncAck.newBuilder()
            .setDataId(message.data.dataId)
            .setSuccess(true)
            .setTimestamp(System.currentTimeMillis())
            .build()

        return Result.success(ack)
    }

    override fun handleMessage(message: ByteArray): Result<ByteArray, ProtocolError> {
        return try {
            val syncMessage = SyncMessage.parseFrom(message)
            val ack = handleSyncMessage(syncMessage)
            Result.success(ack.toByteArray())
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("解析同步消息失败: ${e.message}"))
        }
    }

    override fun validateMessage(message: ByteArray): Result<Unit, ProtocolError> {
        return try {
            val syncMessage = SyncMessage.parseFrom(message)
            syncMessage.validate()
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(ProtocolError.InvalidFormat("验证同步消息失败: ${e.message}"))
        }
    }

    private fun getDeviceId(): String {
        return UUID.randomUUID().toString()
    }
}

// 扩展函数用于验证消息
fun DeviceBroadcast.validate(): Result<Unit, ProtocolError> {
    if (deviceId.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("设备ID不能为空"))
    }
    if (deviceName.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("设备名称不能为空"))
    }
    if (timestamp == 0L) {
        return Result.failure(ProtocolError.InvalidFormat("时间戳无效"))
    }
    return Result.success(Unit)
}

fun PairingResponse.validate(): Result<Unit, ProtocolError> {
    if (responderId.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("响应者ID不能为空"))
    }
    if (initiatorId.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("发起者ID不能为空"))
    }
    if (signedNonce.isEmpty) {
        return Result.failure(ProtocolError.InvalidFormat("签名的随机数不能为空"))
    }
    return Result.success(Unit)
}

fun SyncMessage.validate(): Result<Unit, ProtocolError> {
    if (deviceId.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("设备ID不能为空"))
    }
    if (!data.hasContent()) {
        return Result.failure(ProtocolError.InvalidFormat("数据内容不能为空"))
    }
    return Result.success(Unit)
}

fun ClipboardData.validate(): Result<Unit, ProtocolError> {
    if (dataId.isEmpty()) {
        return Result.failure(ProtocolError.InvalidFormat("数据ID不能为空"))
    }
    if (content.isEmpty) {
        return Result.failure(ProtocolError.InvalidFormat("数据内容不能为空"))
    }
    if (createdAt == 0L) {
        return Result.failure(ProtocolError.InvalidFormat("创建时间无效"))
    }
    return Result.success(Unit)
}