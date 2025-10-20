import Foundation
import CoreBluetooth

/**
 * Protocol Buffers消息处理器
 * 统一Mac和Android端的消息序列化/反序列化实现
 */
class ProtocolBufferMessageHandler {

    // MARK: - Message Serialization

    /// 序列化消息为Data
    func serializeMessage(_ message: TestMessage) -> Data {
        do {
            var protoMessage = BLEMessage()
            protoMessage.messageID = message.messageId
            protoMessage.type = convertToProtoMessageType(message.type)
            protoMessage.payload = message.payload
            protoMessage.timestamp = message.timestamp
            protoMessage.sequenceNumber = Int32(message.sequenceNumber)

            return try protoMessage.serializedData()
        } catch {
            print("❌ Protocol Buffers序列化失败: \(error.localizedDescription)")
            // 回退到旧的字符串序列化方式
            return fallbackSerializeMessage(message)
        }
    }

    /// 反序列化Data为消息
    func deserializeMessage(_ data: Data) -> TestMessage? {
        do {
            let protoMessage = try BLEMessage(serializedData: data)
            return TestMessage(
                messageId: protoMessage.messageID,
                type: convertFromProtoMessageType(protoMessage.type),
                payload: protoMessage.payload,
                timestamp: protoMessage.timestamp,
                sequenceNumber: Int(protoMessage.sequenceNumber)
            )
        } catch {
            print("⚠️ Protocol Buffers解析失败，尝试回退方式: \(error.localizedDescription)")
            // 回退到旧的字符串反序列化方式
            return fallbackDeserializeMessage(data)
        }
    }

    // MARK: - Type Conversion

    /// 转换消息类型为Protocol Buffers枚举
    private func convertToProtoMessageType(_ type: MessageType) -> MessageTypeProto {
        switch type {
        case .ping:
            return .ping
        case .pong:
            return .pong
        case .data:
            return .data
        case .ack:
            return .ack
        }
    }

    /// 从Protocol Buffers枚举转换消息类型
    private func convertFromProtoMessageType(_ type: MessageTypeProto) -> MessageType {
        switch type {
        case .ping:
            return .ping
        case .pong:
            return .pong
        case .data:
            return .data
        case .ack:
            return .ack
        case .unspecified:
            return .data // 默认值
        case .UNRECOGNIZED:
            return .data // 未知类型默认为DATA
        }
    }

    // MARK: - Fallback Methods (向后兼容)

    /// 回退序列化方式（向后兼容）
    private func fallbackSerializeMessage(_ message: TestMessage) -> Data {
        let messageString = "\(message.messageId)|\(message.type.rawValue)|\(message.payload)|\(message.timestamp)|\(message.sequenceNumber)"
        return messageString.data(using: .utf8) ?? Data()
    }

    /// 回退反序列化方式（向后兼容）
    private func fallbackDeserializeMessage(_ data: Data) -> TestMessage? {
        guard let messageString = String(data: data, encoding: .utf8) else { return nil }

        let parts = messageString.split(separator: "|")
        guard parts.count >= 5 else { return nil }

        return TestMessage(
            messageId: String(parts[0]),
            type: MessageType(rawValue: String(parts[1])) ?? .data,
            payload: String(parts[2]),
            timestamp: Double(String(parts[3])) ?? Date().timeIntervalSince1970,
            sequenceNumber: Int(String(parts[4])) ?? 0
        )
    }

    // MARK: - Validation

    /// 验证消息格式
    func validateMessage(_ message: TestMessage) -> Bool {
        // 基本字段验证
        return !message.messageId.isEmpty &&
               message.timestamp > 0 &&
               message.sequenceNumber >= 0 &&
               message.payload.count <= 1024 // 限制载荷大小
    }

    /// 创建设备信息Proto消息
    func createDeviceInfoProto(from peripheral: CBPeripheral, rssi: NSNumber) -> Data {
        do {
            var protoDevice = DeviceInfo()
            protoDevice.deviceID = peripheral.identifier.uuidString
            protoDevice.deviceName = peripheral.name ?? "未知设备"
            protoDevice.rssi = Int32(truncating: rssi)
            protoDevice.isNearclipDevice = peripheral.name?.contains("NearClip") ?? false

            // 根据设备名称判断设备类型
            if peripheral.name?.contains("Android", options: .caseInsensitive) ?? false {
                protoDevice.deviceType = .android
            } else if peripheral.name?.contains("Mac", options: .caseInsensitive) ?? false ||
                      peripheral.name?.contains("iPhone", options: .caseInsensitive) ?? false {
                protoDevice.deviceType = .mac
            } else {
                protoDevice.deviceType = .other
            }

            return try protoDevice.serializedData()
        } catch {
            print("❌ 创建设备信息Proto失败: \(error.localizedDescription)")
            return Data()
        }
    }

    // MARK: - Connection State Conversion

    /// 转换连接状态为Protocol Buffers枚举
    func convertToProtoConnectionState(_ state: ConnectionState) -> ConnectionStateProto {
        switch state {
        case .disconnected:
            return .disconnected
        case .connecting:
            return .connecting
        case .connected:
            return .connected
        case .failed:
            return .failed
        }
    }

    /// 从Protocol Buffers枚举转换连接状态
    func convertFromProtoConnectionState(_ state: ConnectionStateProto) -> ConnectionState {
        switch state {
        case .disconnected:
            return .disconnected
        case .connecting:
            return .connecting
        case .connected:
            return .connected
        case .failed:
            return .failed
        case .unspecified:
            return .disconnected // 默认值
        case .UNRECOGNIZED:
            return .disconnected // 未知状态默认为断开
        }
    }
}

// 为了避免命名冲突，使用类型别名
typealias MessageTypeProto = Nearclip_Protocol_MessageType
typealias ConnectionStateProto = Nearclip_Protocol_ConnectionState