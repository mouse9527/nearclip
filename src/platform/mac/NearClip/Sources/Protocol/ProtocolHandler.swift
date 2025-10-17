import Foundation

// MARK: - Protocol Errors
enum ProtocolError: LocalizedError {
    case invalidFormat(String)
    case unsupportedVersion(String)
    case signatureVerificationFailed
    case cryptographicError(String)
    case networkError(String)

    var errorDescription: String? {
        switch self {
        case .invalidFormat(let message):
            return "无效的消息格式: \(message)"
        case .unsupportedVersion(let version):
            return "不支持的协议版本: \(version)"
        case .signatureVerificationFailed:
            return "消息签名验证失败"
        case .cryptographicError(let message):
            return "加密/解密错误: \(message)"
        case .networkError(let message):
            return "网络错误: \(message)"
        }
    }
}

// MARK: - Protocol Handler Protocol
protocol ProtocolHandler {
    func handleMessage(_ message: Data) throws -> Data
    func validateMessage(_ message: Data) throws
}

// MARK: - Discovery Handler
class DiscoveryHandler: ProtocolHandler {

    func handleBroadcast(_ broadcast: DeviceBroadcast) throws {
        try broadcast.validate()
        print("收到设备广播: \(broadcast.deviceName) (\(broadcast.deviceId))")
    }

    func createScanRequest(timeoutSeconds: UInt32) -> ScanRequest {
        return ScanRequest.with {
            $0.timeoutSeconds = timeoutSeconds
        }
    }

    func handleMessage(_ message: Data) throws -> Data {
        do {
            let broadcast = try DeviceBroadcast(serializedData: message)
            try handleBroadcast(broadcast)
            return Data()
        } catch {
            throw ProtocolError.invalidFormat("解析设备广播消息失败: \(error.localizedDescription)")
        }
    }

    func validateMessage(_ message: Data) throws {
        do {
            let broadcast = try DeviceBroadcast(serializedData: message)
            try broadcast.validate()
        } catch {
            throw ProtocolError.invalidFormat("验证设备广播消息失败: \(error.localizedDescription)")
        }
    }
}

// MARK: - Pairing Handler
class PairingHandler: ProtocolHandler {

    func initiatePairing(targetId: String, deviceName: String) throws -> PairingRequest {
        return PairingRequest.with {
            $0.initiatorId = getDeviceId()
            $0.targetId = targetId
            $0.deviceName = deviceName
            $0.nonce = generateNonce()
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }
    }

    func handlePairingResponse(_ response: PairingResponse) throws {
        try response.validate()
        print("配对响应来自: \(response.responderId)")
    }

    func handleMessage(_ message: Data) throws -> Data {
        do {
            let response = try PairingResponse(serializedData: message)
            try handlePairingResponse(response)
            return Data()
        } catch {
            throw ProtocolError.invalidFormat("解析配对响应消息失败: \(error.localizedDescription)")
        }
    }

    func validateMessage(_ message: Data) throws {
        do {
            let response = try PairingResponse(serializedData: message)
            try response.validate()
        } catch {
            throw ProtocolError.invalidFormat("验证配对响应消息失败: \(error.localizedDescription)")
        }
    }

    private func getDeviceId() -> String {
        return UUID().uuidString
    }

    private func generateNonce() -> Data {
        var nonce = Data(count: 32)
        let result = nonce.withUnsafeMutableBytes { bytes in
            SecRandomCopyBytes(kSecRandomDefault, 32, bytes.baseAddress!)
        }
        guard result == errSecSuccess else {
            fatalError("生成随机数失败")
        }
        return nonce
    }
}

// MARK: - Sync Handler
class SyncHandler: ProtocolHandler {

    func createSyncMessage(data: ClipboardData, operation: SyncOperation) throws -> SyncMessage {
        return SyncMessage.with {
            $0.deviceID = getDeviceId()
            $0.operation = operation
            $0.data = data
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }
    }

    func handleSyncMessage(_ message: SyncMessage) throws -> SyncAck {
        try message.validate()

        let ack = SyncAck.with {
            $0.dataID = message.data.dataID
            $0.success = true
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        return ack
    }

    func handleMessage(_ message: Data) throws -> Data {
        do {
            let syncMessage = try SyncMessage(serializedData: message)
            let ack = try handleSyncMessage(syncMessage)
            return try ack.serializedData()
        } catch {
            throw ProtocolError.invalidFormat("解析同步消息失败: \(error.localizedDescription)")
        }
    }

    func validateMessage(_ message: Data) throws {
        do {
            let syncMessage = try SyncMessage(serializedData: message)
            try syncMessage.validate()
        } catch {
            throw ProtocolError.invalidFormat("验证同步消息失败: \(error.localizedDescription)")
        }
    }

    private func getDeviceId() -> String {
        return UUID().uuidString
    }
}

// MARK: - Message Validation Extensions
extension DeviceBroadcast {
    func validate() throws {
        if deviceId.isEmpty {
            throw ProtocolError.invalidFormat("设备ID不能为空")
        }
        if deviceName.isEmpty {
            throw ProtocolError.invalidFormat("设备名称不能为空")
        }
        if timestamp == 0 {
            throw ProtocolError.invalidFormat("时间戳无效")
        }
    }

    func hasCapability(_ capability: DeviceCapability) -> Bool {
        return capabilities.contains(capability)
    }
}

extension PairingResponse {
    func validate() throws {
        if responderId.isEmpty {
            throw ProtocolError.invalidFormat("响应者ID不能为空")
        }
        if initiatorId.isEmpty {
            throw ProtocolError.invalidFormat("发起者ID不能为空")
        }
        if signedNonce.isEmpty {
            throw ProtocolError.invalidFormat("签名的随机数不能为空")
        }
    }
}

extension SyncMessage {
    func validate() throws {
        if deviceID.isEmpty {
            throw ProtocolError.invalidFormat("设备ID不能为空")
        }
        if !data.hasContent {
            throw ProtocolError.invalidFormat("数据内容不能为空")
        }
    }

    var totalSize: Int {
        return chunks.reduce(0) { $0 + $1.chunkData.count }
    }

    var requiresChunking: Bool {
        return !chunks.isEmpty
    }
}

extension ClipboardData {
    func validate() throws {
        if dataID.isEmpty {
            throw ProtocolError.invalidFormat("数据ID不能为空")
        }
        if content.isEmpty {
            throw ProtocolError.invalidFormat("数据内容不能为空")
        }
        if createdAt == 0 {
            throw ProtocolError.invalidFormat("创建时间无效")
        }
    }

    var isExpired: Bool {
        if expiresAt == 0 {
            return false // 永不过期
        }
        let currentTime = UInt64(Date().timeIntervalSince1970 * 1000)
        return currentTime > expiresAt
    }

    var size: Int {
        return content.count
    }
}

extension DataChunk {
    func validate() throws {
        if dataID.isEmpty {
            throw ProtocolError.invalidFormat("数据ID不能为空")
        }
        if chunkIndex >= totalChunks {
            throw ProtocolError.invalidFormat("分片索引无效")
        }
        if chunkData.isEmpty {
            throw ProtocolError.invalidFormat("分片数据不能为空")
        }
        if checksum.isEmpty {
            throw ProtocolError.invalidFormat("校验和不能为空")
        }
    }

    func verifyChecksum() -> Bool {
        let hash = SHA256.hash(data: chunkData)
        return Data(hash) == checksum
    }
}

// MARK: - Pairing Status Extensions
extension PairingStatus {
    var isCompleted: Bool {
        return self == .pairingCompleted
    }

    var isFailed: Bool {
        return self == .pairingFailed
    }

    var isPending: Bool {
        return self == .pairingPending || self == .pairingInitiated
    }
}

// MARK: - Protocol Version Extensions
extension ProtocolVersion {
    func isCompatible(with other: ProtocolVersion) -> Bool {
        // 主版本必须相同
        if major != other.major {
            return false
        }
        // 次版本向后兼容
        return minor >= other.minor
    }
}

// MARK: - Error Message Extensions
extension ErrorMessage {
    static func new(code: ErrorCode, message: String) -> ErrorMessage {
        return ErrorMessage.with {
            $0.code = code
            $0.message = message
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }
    }

    func withDetails(_ details: String) -> ErrorMessage {
        var updated = self
        updated.details = details
        return updated
    }
}

// MARK: - Heartbeat Extensions
extension Heartbeat {
    static func new(deviceId: String, sequenceNumber: UInt32) -> Heartbeat {
        return Heartbeat.with {
            $0.deviceID = deviceId
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
            $0.sequenceNumber = sequenceNumber
        }
    }
}

extension HeartbeatAck {
    static func new(deviceId: String, sequenceNumber: UInt32) -> HeartbeatAck {
        return HeartbeatAck.with {
            $0.deviceID = deviceId
            $0.receivedTimestamp = UInt64(Date().timeIntervalSince1970 * 1000)
            $0.sequenceNumber = sequenceNumber
        }
    }
}

// MARK: - Capability Negotiation Extensions
extension CapabilityNegotiation {
    static func new(minVersion: ProtocolVersion, maxVersion: ProtocolVersion) -> CapabilityNegotiation {
        return CapabilityNegotiation.with {
            $0.minVersion = minVersion
            $0.maxVersion = maxVersion
        }
    }

    func withSupportedFeature(_ feature: String) -> CapabilityNegotiation {
        var updated = self
        updated.supportedFeatures.append(feature)
        return updated
    }

    func withRequiredFeature(_ feature: String) -> CapabilityNegotiation {
        var updated = self
        updated.requiredFeatures.append(feature)
        return updated
    }
}

extension CapabilityNegotiationResponse {
    static func compatible(selectedVersion: ProtocolVersion) -> CapabilityNegotiationResponse {
        return CapabilityNegotiationResponse.with {
            $0.selectedVersion = selectedVersion
            $0.compatibility = true
        }
    }

    static func incompatible(selectedVersion: ProtocolVersion) -> CapabilityNegotiationResponse {
        return CapabilityNegotiationResponse.with {
            $0.selectedVersion = selectedVersion
            $0.compatibility = false
        }
    }
}