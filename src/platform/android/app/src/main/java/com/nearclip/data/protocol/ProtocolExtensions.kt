package com.nearclip.data.protocol

import com.nearclip.data.network.proto.*
import com.google.protobuf.ByteString

/**
 * 协议扩展功能
 */

// 设备广播扩展
fun DeviceBroadcast.hasCapability(capability: DeviceCapability): Boolean {
    return this.capabilitiesList.contains(capability)
}

fun DeviceBroadcast.getDeviceTypeString(): String {
    return when (deviceType) {
        DeviceType.DEVICE_TYPE_ANDROID -> "Android"
        DeviceType.DEVICE_TYPE_MAC -> "macOS"
        DeviceType.DEVICE_TYPE_WINDOWS -> "Windows"
        DeviceType.DEVICE_TYPE_IOS -> "iOS"
        else -> "Unknown"
    }
}

// 扫描请求扩展
fun ScanRequest.withDeviceTypeFilter(vararg types: DeviceType): ScanRequest {
    return this.toBuilder()
        .addAllFilterTypes(types.toList())
        .build()
}

fun ScanRequest.withRequiredCapability(vararg capabilities: DeviceCapability): ScanRequest {
    return this.toBuilder()
        .addAllRequiredCapabilities(capabilities.toList())
        .build()
}

// 剪贴板数据扩展
fun ClipboardData.isExpired(): Boolean {
    if (expiresAt == 0L) {
        return false // 永不过期
    }
    return System.currentTimeMillis() > expiresAt
}

fun ClipboardData.getSize(): Int {
    return content.size()
}

fun ClipboardData.getMetadataString(): String {
    return if (metadataMap.isNotEmpty()) {
        metadataMap.entries.joinToString(", ") { "${it.key}=${it.value}" }
    } else {
        "无元数据"
    }
}

// 数据分片扩展
fun DataChunk.verifyChecksum(): Boolean {
    val calculatedChecksum = calculateChecksum(this.chunkData)
    return calculatedChecksum == this.checksum
}

private fun calculateChecksum(data: ByteString): ByteString {
    // 简单的校验和计算（实际应用中应使用更强的哈希算法）
    var sum = 0L
    for (byte in data.toByteArray()) {
        sum += byte.toLong()
    }
    return ByteString.copyFrom(sum.toString().toByteArray())
}

// 同步消息扩展
fun SyncMessage.totalSize(): Int {
    return chunksList.sumOf { it.chunkData.size() }
}

fun SyncMessage.requiresChunking(): Boolean {
    return chunksList.isNotEmpty()
}

// 配对状态扩展
fun PairingStatus.isCompleted(): Boolean {
    return this == PairingStatus.PAIRING_COMPLETED
}

fun PairingStatus.isFailed(): Boolean {
    return this == PairingStatus.PAIRING_FAILED
}

fun PairingStatus.isPending(): Boolean {
    return this == PairingStatus.PAIRING_PENDING || this == PairingStatus.PAIRING_INITIATED
}

fun PairingStatus.getStatusString(): String {
    return when (this) {
        PairingStatus.PAIRING_UNKNOWN -> "未知"
        PairingStatus.PAIRING_INITIATED -> "已发起"
        PairingStatus.PAIRING_PENDING -> "等待中"
        PairingStatus.PAIRING_CONFIRMED -> "已确认"
        PairingStatus.PAIRING_FAILED -> "失败"
        PairingStatus.PAIRING_COMPLETED -> "已完成"
    }
}

// 协议版本扩展
fun ProtocolVersion.isCompatibleWith(other: ProtocolVersion): Boolean {
    // 主版本必须相同
    if (major != other.major) {
        return false
    }
    // 次版本向后兼容
    return minor >= other.minor
}

fun ProtocolVersion.getVersionString(): String {
    return "$major.$minor.$patch${if (buildInfo.isNotEmpty()) " ($buildInfo)" else ""}"
}

// 错误消息扩展
fun ErrorMessage.withDetails(details: String): ErrorMessage {
    return this.toBuilder()
        .setDetails(details)
        .build()
}

fun ErrorCode.getErrorDescription(): String {
    return when (this) {
        ErrorCode.ERROR_NONE -> "无错误"
        ErrorCode.ERROR_INVALID_MESSAGE -> "无效消息"
        ErrorCode.ERROR_INVALID_SIGNATURE -> "无效签名"
        ErrorCode.ERROR_EXPIRED_MESSAGE -> "消息已过期"
        ErrorCode.ERROR_UNSUPPORTED_VERSION -> "不支持的版本"
        ErrorCode.ERROR_DEVICE_NOT_FOUND -> "设备未找到"
        ErrorCode.ERROR_PAIRING_FAILED -> "配对失败"
        ErrorCode.ERROR_ENCRYPTION_FAILED -> "加密失败"
        ErrorCode.ERROR_NETWORK_ERROR -> "网络错误"
        ErrorCode.ERROR_TIMEOUT -> "超时"
        ErrorCode.ERROR_QUOTA_EXCEEDED -> "配额超限"
        ErrorCode.ERROR_INTERNAL_ERROR -> "内部错误"
        else -> "未知错误"
    }
}

// 数据类型扩展
fun DataType.getDisplayName(): String {
    return when (this) {
        DataType.DATA_TYPE_TEXT -> "文本"
        DataType.DATA_TYPE_IMAGE -> "图片"
        DataType.DATA_TYPE_FILE -> "文件"
        DataType.DATA_TYPE_URL -> "链接"
        DataType.DATA_TYPE_RICH_TEXT -> "富文本"
        else -> "未知类型"
    }
}

// 同步操作扩展
fun SyncOperation.getDisplayName(): String {
    return when (this) {
        SyncOperation.SYNC_CREATE -> "创建"
        SyncOperation.SYNC_UPDATE -> "更新"
        SyncOperation.SYNC_DELETE -> "删除"
        SyncOperation.SYNC_REPLACE -> "替换"
        else -> "未知操作"
    }
}

// 能力协商扩展
fun CapabilityNegotiation.withSupportedFeature(feature: String): CapabilityNegotiation {
    return this.toBuilder()
        .addSupportedFeatures(feature)
        .build()
}

fun CapabilityNegotiation.withRequiredFeature(feature: String): CapabilityNegotiation {
    return this.toBuilder()
        .addRequiredFeatures(feature)
        .build()
}

fun CapabilityNegotiationResponse.isCompatible(): Boolean {
    return compatibility
}

fun CapabilityNegotiationResponse.getUnsupportedFeatures(): List<String> {
    return unsupportedFeaturesList
}