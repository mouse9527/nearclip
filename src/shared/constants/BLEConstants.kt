package com.nearclip.shared.constants

/**
 * BLE通信共享常量定义
 * Android和Mac端共同使用
 */
object BLEConstants {

    // MARK: - BLE服务UUIDs
    const val SERVICE_UUID = "0000FE2C-0000-1000-8000-00805F9B34FB"
    const val CHARACTERISTIC_UUID = "0000FE2D-0000-1000-8000-00805F9B34FB"
    const val DESCRIPTOR_UUID = "00002902-0000-1000-8000-00805F9B34FB"

    // MARK: - 连接配置
    const val CONNECTION_TIMEOUT_MS = 10000L // 10秒连接超时
    const val SCAN_TIMEOUT_MS = 30000L // 30秒扫描超时
    const val MESSAGE_TIMEOUT_MS = 5000L // 5秒消息超时

    // MARK: - 性能目标
    const val MAX_CONNECTION_TIME_MS = 3000L // 最大连接建立时间3秒
    const val MAX_MESSAGE_DELAY_MS = 1000L // 最大消息传输延迟1秒
    const val MAX_DISCOVERY_TIME_MS = 5000L // 最大设备发现时间5秒
    const val STABILITY_TEST_DURATION_MS = 600000L // 稳定性测试10分钟
    const val MAX_DISCONNECTION_COUNT = 1 // 10分钟内最大断开次数

    // MARK: - 消息限制
    const val MAX_PAYLOAD_SIZE = 1024 // 最大载荷大小1KB
    const val MAX_MESSAGE_ID_LENGTH = 128 // 最大消息ID长度
    const val MAX_DEVICE_NAME_LENGTH = 256 // 最大设备名称长度

    // MARK: - 设备过滤
    const val NEARCLIP_DEVICE_PREFIX = "NearClip"
    const val ANDROID_DEVICE_SUFFIX = "Android"
    const val MAC_DEVICE_SUFFIX = "Mac"

    // MARK: - 重试配置
    const val MAX_RETRY_ATTEMPTS = 3 // 最大重试次数
    const val RETRY_DELAY_MS = 1000L // 重试延迟1秒
    const val RETRY_BACKOFF_MULTIPLIER = 2 // 重试退避倍数

    // MARK: - 缓存配置
    const val DEVICE_CACHE_EXPIRY_MS = 300000L // 设备缓存5分钟过期
    const val MAX_CACHED_DEVICES = 50 // 最大缓存设备数
    const val MAX_CACHED_MESSAGES = 100 // 最大缓存消息数

    // MARK: - 日志配置
    const val MAX_LOG_MESSAGE_LENGTH = 512 // 最大日志消息长度
    const val VERBOSE_LOGGING = false // 详细日志开关

    // MARK: - 测试配置
    const val TEST_MESSAGE_COUNT = 100 // 性能测试消息数量
    const val TEST_CONCURRENT_THREADS = 10 // 并发测试线程数
    const val TEST_MEMORY_MESSAGE_COUNT = 1000 // 内存测试消息数量

    // MARK: - 错误码定义
    object ErrorCodes {
        const val SUCCESS = 0
        const val ERROR_UNKNOWN = -1
        const val ERROR_BLUETOOTH_NOT_AVAILABLE = -2
        const val ERROR_BLUETOOTH_NOT_ENABLED = -3
        const val ERROR_PERMISSION_DENIED = -4
        const val ERROR_DEVICE_NOT_FOUND = -5
        const val ERROR_CONNECTION_FAILED = -6
        const val ERROR_CONNECTION_TIMEOUT = -7
        const val ERROR_CONNECTION_LOST = -8
        const val ERROR_SERVICE_DISCOVERY_FAILED = -9
        const val ERROR_CHARACTERISTIC_NOT_FOUND = -10
        const val ERROR_WRITE_FAILED = -11
        const val ERROR_READ_FAILED = -12
        const val ERROR_NOTIFICATION_FAILED = -13
        const val ERROR_INVALID_MESSAGE = -14
        const val ERROR_MESSAGE_TOO_LARGE = -15
        const val ERROR_PROTOCOL_ERROR = -16
        const val ERROR_AUTHENTICATION_FAILED = -17
        const val ERROR_INSUFFICIENT_ENCRYPTION = -18
        const val ERROR_OUT_OF_MEMORY = -19
        const val ERROR_OPERATION_CANCELLED = -20
        const val ERROR_OPERATION_IN_PROGRESS = -21
        const val ERROR_INVALID_PARAMETER = -22
        const val ERROR_RESOURCE_BUSY = -23
        const val ERROR_UNSUPPORTED_OPERATION = -24
    }

    // MARK: - 错误消息
    object ErrorMessages {
        const val SUCCESS = "操作成功"
        const val ERROR_UNKNOWN = "未知错误"
        const val ERROR_BLUETOOTH_NOT_AVAILABLE = "蓝牙不可用"
        const val ERROR_BLUETOOTH_NOT_ENABLED = "蓝牙未启用"
        const val ERROR_PERMISSION_DENIED = "权限被拒绝"
        const val ERROR_DEVICE_NOT_FOUND = "设备未找到"
        const val ERROR_CONNECTION_FAILED = "连接失败"
        const val ERROR_CONNECTION_TIMEOUT = "连接超时"
        const val ERROR_CONNECTION_LOST = "连接丢失"
        const val ERROR_SERVICE_DISCOVERY_FAILED = "服务发现失败"
        const val ERROR_CHARACTERISTIC_NOT_FOUND = "特征值未找到"
        const val ERROR_WRITE_FAILED = "写入失败"
        const val ERROR_READ_FAILED = "读取失败"
        const val ERROR_NOTIFICATION_FAILED = "通知失败"
        const val ERROR_INVALID_MESSAGE = "无效消息"
        const val ERROR_MESSAGE_TOO_LARGE = "消息过大"
        const val ERROR_PROTOCOL_ERROR = "协议错误"
        const val ERROR_AUTHENTICATION_FAILED = "身份验证失败"
        const val ERROR_INSUFFICIENT_ENCRYPTION = "加密不足"
        const val ERROR_OUT_OF_MEMORY = "内存不足"
        const val ERROR_OPERATION_CANCELLED = "操作已取消"
        const val ERROR_OPERATION_IN_PROGRESS = "操作正在进行中"
        const val ERROR_INVALID_PARAMETER = "无效参数"
        const val ERROR_RESOURCE_BUSY = "资源忙"
        const val ERROR_UNSUPPORTED_OPERATION = "不支持的操作"
    }

    // MARK: - 设备类型常量
    object DeviceType {
        const val UNSPECIFIED = "UNSPECIFIED"
        const val ANDROID = "ANDROID"
        const val MAC = "MAC"
        const val IOS = "IOS"
        const val OTHER = "OTHER"
    }

    // MARK: - 消息类型常量
    object MessageType {
        const val PING = "PING"
        const val PONG = "PONG"
        const val DATA = "DATA"
        const val ACK = "ACK"
    }

    // MARK: - 连接状态常量
    object ConnectionState {
        const val DISCONNECTED = "DISCONNECTED"
        const val CONNECTING = "CONNECTING"
        const val CONNECTED = "CONNECTED"
        const val FAILED = "FAILED"
    }
}

/**
 * BLE错误类型
 */
sealed class BLEError(
    val code: Int,
    override val message: String
) : Exception(message) {

    class BluetoothNotAvailable : BLEError(
        BLEConstants.ErrorCodes.ERROR_BLUETOOTH_NOT_AVAILABLE,
        BLEConstants.ErrorMessages.ERROR_BLUETOOTH_NOT_AVAILABLE
    )

    class BluetoothNotEnabled : BLEError(
        BLEConstants.ErrorCodes.ERROR_BLUETOOTH_NOT_ENABLED,
        BLEConstants.ErrorMessages.ERROR_BLUETOOTH_NOT_ENABLED
    )

    class PermissionDenied : BLEError(
        BLEConstants.ErrorCodes.ERROR_PERMISSION_DENIED,
        BLEConstants.ErrorMessages.ERROR_PERMISSION_DENIED
    )

    class DeviceNotFound : BLEError(
        BLEConstants.ErrorCodes.ERROR_DEVICE_NOT_FOUND,
        BLEConstants.ErrorMessages.ERROR_DEVICE_NOT_FOUND
    )

    class ConnectionFailed : BLEError(
        BLEConstants.ErrorCodes.ERROR_CONNECTION_FAILED,
        BLEConstants.ErrorMessages.ERROR_CONNECTION_FAILED
    )

    class ConnectionTimeout : BLEError(
        BLEConstants.ErrorCodes.ERROR_CONNECTION_TIMEOUT,
        BLEConstants.ErrorMessages.ERROR_CONNECTION_TIMEOUT
    )

    class ConnectionLost : BLEError(
        BLEConstants.ErrorCodes.ERROR_CONNECTION_LOST,
        BLEConstants.ErrorMessages.ERROR_CONNECTION_LOST
    )

    class InvalidMessage : BLEError(
        BLEConstants.ErrorCodes.ERROR_INVALID_MESSAGE,
        BLEConstants.ErrorMessages.ERROR_INVALID_MESSAGE
    )

    class MessageTooLarge : BLEError(
        BLEConstants.ErrorCodes.ERROR_MESSAGE_TOO_LARGE,
        BLEConstants.ErrorMessages.ERROR_MESSAGE_TOO_LARGE
    )

    class ProtocolError : BLEError(
        BLEConstants.ErrorCodes.ERROR_PROTOCOL_ERROR,
        BLEConstants.ErrorMessages.ERROR_PROTOCOL_ERROR
    )

    class UnknownError(override val message: String = BLEConstants.ErrorMessages.ERROR_UNKNOWN) : BLEError(
        BLEConstants.ErrorCodes.ERROR_UNKNOWN,
        message
    )
}