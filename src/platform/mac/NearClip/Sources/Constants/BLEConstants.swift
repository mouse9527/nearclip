import Foundation

/**
 * BLE通信共享常量定义
 * Mac和Android端共同使用
 */
struct BLEConstants {

    // MARK: - BLE服务UUIDs
    static let serviceUUID = "0000FE2C-0000-1000-8000-00805F9B34FB"
    static let characteristicUUID = "0000FE2D-0000-1000-8000-00805F9B34FB"
    static let descriptorUUID = "00002902-0000-1000-8000-00805F9B34FB"

    // MARK: - 连接配置
    static let connectionTimeout: TimeInterval = 10.0 // 10秒连接超时
    static let scanTimeout: TimeInterval = 30.0 // 30秒扫描超时
    static let messageTimeout: TimeInterval = 5.0 // 5秒消息超时

    // MARK: - 性能目标
    static let maxConnectionTime: TimeInterval = 3.0 // 最大连接建立时间3秒
    static let maxMessageDelay: TimeInterval = 1.0 // 最大消息传输延迟1秒
    static let maxDiscoveryTime: TimeInterval = 5.0 // 最大设备发现时间5秒
    static let stabilityTestDuration: TimeInterval = 600.0 // 稳定性测试10分钟
    static let maxDisconnectionCount = 1 // 10分钟内最大断开次数

    // MARK: - 消息限制
    static let maxPayloadSize = 1024 // 最大载荷大小1KB
    static let maxMessageIdLength = 128 // 最大消息ID长度
    static let maxDeviceNameLength = 256 // 最大设备名称长度

    // MARK: - 设备过滤
    static let nearclipDevicePrefix = "NearClip"
    static let androidDeviceSuffix = "Android"
    static let macDeviceSuffix = "Mac"

    // MARK: - 重试配置
    static let maxRetryAttempts = 3 // 最大重试次数
    static let retryDelay: TimeInterval = 1.0 // 重试延迟1秒
    static let retryBackoffMultiplier = 2 // 重试退避倍数

    // MARK: - 缓存配置
    static let deviceCacheExpiry: TimeInterval = 300.0 // 设备缓存5分钟过期
    static let maxCachedDevices = 50 // 最大缓存设备数
    static let maxCachedMessages = 100 // 最大缓存消息数

    // MARK: - 日志配置
    static let maxLogMessageLength = 512 // 最大日志消息长度
    static let verboseLogging = false // 详细日志开关

    // MARK: - 测试配置
    static let testMessageCount = 100 // 性能测试消息数量
    static let testConcurrentThreads = 10 // 并发测试线程数
    static let testMemoryMessageCount = 1000 // 内存测试消息数量

    // MARK: - 错误码定义
    struct ErrorCodes {
        static let success = 0
        static let errorUnknown = -1
        static let errorBluetoothNotAvailable = -2
        static let errorBluetoothNotEnabled = -3
        static let errorPermissionDenied = -4
        static let errorDeviceNotFound = -5
        static let errorConnectionFailed = -6
        static let errorConnectionTimeout = -7
        static let errorConnectionLost = -8
        static let errorServiceDiscoveryFailed = -9
        static let errorCharacteristicNotFound = -10
        static let errorWriteFailed = -11
        static let errorReadFailed = -12
        static let errorNotificationFailed = -13
        static let errorInvalidMessage = -14
        static let errorMessageTooLarge = -15
        static let errorProtocolError = -16
        static let errorAuthenticationFailed = -17
        static let errorInsufficientEncryption = -18
        static let errorOutOfMemory = -19
        static let errorOperationCancelled = -20
        static let errorOperationInProgress = -21
        static let errorInvalidParameter = -22
        static let errorResourceBusy = -23
        static let errorUnsupportedOperation = -24
    }

    // MARK: - 错误消息
    struct ErrorMessages {
        static let success = "操作成功"
        static let errorUnknown = "未知错误"
        static let errorBluetoothNotAvailable = "蓝牙不可用"
        static let errorBluetoothNotEnabled = "蓝牙未启用"
        static let errorPermissionDenied = "权限被拒绝"
        static let errorDeviceNotFound = "设备未找到"
        static let errorConnectionFailed = "连接失败"
        static let errorConnectionTimeout = "连接超时"
        static let errorConnectionLost = "连接丢失"
        static let errorServiceDiscoveryFailed = "服务发现失败"
        static let errorCharacteristicNotFound = "特征值未找到"
        static let errorWriteFailed = "写入失败"
        static let errorReadFailed = "读取失败"
        static let errorNotificationFailed = "通知失败"
        static let errorInvalidMessage = "无效消息"
        static let errorMessageTooLarge = "消息过大"
        static let errorProtocolError = "协议错误"
        static let errorAuthenticationFailed = "身份验证失败"
        static let errorInsufficientEncryption = "加密不足"
        static let errorOutOfMemory = "内存不足"
        static let errorOperationCancelled = "操作已取消"
        static let errorOperationInProgress = "操作正在进行中"
        static let errorInvalidParameter = "无效参数"
        static let errorResourceBusy = "资源忙"
        static let errorUnsupportedOperation = "不支持的操作"
    }

    // MARK: - 设备类型常量
    struct DeviceType {
        static let unspecified = "UNSPECIFIED"
        static let android = "ANDROID"
        static let mac = "MAC"
        static let ios = "IOS"
        static let other = "OTHER"
    }

    // MARK: - 消息类型常量
    struct MessageType {
        static let ping = "PING"
        static let pong = "PONG"
        static let data = "DATA"
        static let ack = "ACK"
    }

    // MARK: - 连接状态常量
    struct ConnectionState {
        static let disconnected = "DISCONNECTED"
        static let connecting = "CONNECTING"
        static let connected = "CONNECTED"
        static let failed = "FAILED"
    }
}

/**
 * BLE错误类型
 */
enum BLEError: Error, LocalizedError {
    case bluetoothNotAvailable
    case bluetoothNotEnabled
    case permissionDenied
    case deviceNotFound
    case connectionFailed
    case connectionTimeout
    case connectionLost
    case invalidMessage
    case messageTooLarge
    case protocolError
    case unknownError(String)

    var code: Int {
        switch self {
        case .bluetoothNotAvailable:
            return BLEConstants.ErrorCodes.errorBluetoothNotAvailable
        case .bluetoothNotEnabled:
            return BLEConstants.ErrorCodes.errorBluetoothNotEnabled
        case .permissionDenied:
            return BLEConstants.ErrorCodes.errorPermissionDenied
        case .deviceNotFound:
            return BLEConstants.ErrorCodes.errorDeviceNotFound
        case .connectionFailed:
            return BLEConstants.ErrorCodes.errorConnectionFailed
        case .connectionTimeout:
            return BLEConstants.ErrorCodes.errorConnectionTimeout
        case .connectionLost:
            return BLEConstants.ErrorCodes.errorConnectionLost
        case .invalidMessage:
            return BLEConstants.ErrorCodes.errorInvalidMessage
        case .messageTooLarge:
            return BLEConstants.ErrorCodes.errorMessageTooLarge
        case .protocolError:
            return BLEConstants.ErrorCodes.errorProtocolError
        case .unknownError:
            return BLEConstants.ErrorCodes.errorUnknown
        }
    }

    var errorDescription: String? {
        switch self {
        case .bluetoothNotAvailable:
            return BLEConstants.ErrorMessages.errorBluetoothNotAvailable
        case .bluetoothNotEnabled:
            return BLEConstants.ErrorMessages.errorBluetoothNotEnabled
        case .permissionDenied:
            return BLEConstants.ErrorMessages.errorPermissionDenied
        case .deviceNotFound:
            return BLEConstants.ErrorMessages.errorDeviceNotFound
        case .connectionFailed:
            return BLEConstants.ErrorMessages.errorConnectionFailed
        case .connectionTimeout:
            return BLEConstants.ErrorMessages.errorConnectionTimeout
        case .connectionLost:
            return BLEConstants.ErrorMessages.errorConnectionLost
        case .invalidMessage:
            return BLEConstants.ErrorMessages.errorInvalidMessage
        case .messageTooLarge:
            return BLEConstants.ErrorMessages.errorMessageTooLarge
        case .protocolError:
            return BLEConstants.ErrorMessages.errorProtocolError
        case .unknownError(let message):
            return message.isEmpty ? BLEConstants.ErrorMessages.errorUnknown : message
        }
    }
}