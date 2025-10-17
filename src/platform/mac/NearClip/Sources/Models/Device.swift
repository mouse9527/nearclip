import Foundation
import SwiftUI

// 设备类型枚举
enum DeviceType: String, Codable, CaseIterable {
    case unknown = "UNKNOWN"
    case android = "ANDROID"
    case mac = "MAC"
}

// 连接状态枚举
enum ConnectionStatus: String, Codable, CaseIterable {
    case unknown = "UNKNOWN"
    case disconnected = "DISCONNECTED"
    case connected = "CONNECTED"
    case pairing = "PAIRING"
    case error = "ERROR"
}

// 能力枚举
enum Capability: String, Codable, CaseIterable {
    case unknown = "UNKNOWN"
    case ble = "BLE"
    case wifiDirect = "WIFI_DIRECT"
    case clipboardRead = "CLIPBOARD_READ"
    case clipboardWrite = "CLIPBOARD_WRITE"
}

// 设备模型
struct Device: Codable, Identifiable, Equatable {
    let id: String
    let name: String
    let type: DeviceType
    var connectionStatus: ConnectionStatus
    let lastSeen: Date
    let capabilities: [Capability]
    let alias: String?
    let batteryLevel: Int?

    // SwiftUI Identifiable
    var idForSwiftUI: String { id }

    // 便利初始化器
    init(
        id: String,
        name: String,
        type: DeviceType = .unknown,
        connectionStatus: ConnectionStatus = .disconnected,
        lastSeen: Date = Date(),
        capabilities: [Capability] = [],
        alias: String? = nil,
        batteryLevel: Int? = nil
    ) {
        self.id = id
        self.name = name
        self.type = type
        self.connectionStatus = connectionStatus
        self.lastSeen = lastSeen
        self.capabilities = capabilities
        self.alias = alias
        self.batteryLevel = batteryLevel
    }

    // 显示名称
    var displayName: String {
        alias ?? name
    }

    // 状态图标
    var statusIcon: String {
        switch connectionStatus {
        case .connected:
            return "checkmark.circle.fill"
        case .pairing:
            return "progress.indicator"
        case .disconnected:
            return "circle"
        case .error:
            return "xmark.circle.fill"
        case .unknown:
            return "questionmark.circle"
        }
    }

    // 状态颜色
    var statusColor: Color {
        switch connectionStatus {
        case .connected:
            return .green
        case .pairing:
            return .blue
        case .disconnected:
            return .gray
        case .error:
            return .red
        case .unknown:
            return .secondary
        }
    }

    // 电池显示文本
    var batteryText: String? {
        guard let level = batteryLevel else { return nil }
        return "\(level)%"
    }

    // 是否支持剪贴板操作
    var supportsClipboard: Bool {
        capabilities.contains(.clipboardRead) || capabilities.contains(.clipboardWrite)
    }
}

// 应用状态
class AppState: ObservableObject {
    @Published var isConnected: Bool = false
    @Published var connectedDevices: [Device] = []
    @Published var isDiscovering: Bool = false
    @Published var lastSyncTime: Date?

    var deviceCount: Int {
        connectedDevices.count
    }

    func addDevice(_ device: Device) {
        if let index = connectedDevices.firstIndex(where: { $0.id == device.id }) {
            connectedDevices[index] = device
        } else {
            connectedDevices.append(device)
        }
    }

    func removeDevice(_ deviceId: String) {
        connectedDevices.removeAll { $0.id == deviceId }
    }

    func updateDeviceStatus(_ deviceId: String, status: ConnectionStatus) {
        if let index = connectedDevices.firstIndex(where: { $0.id == deviceId }) {
            connectedDevices[index].connectionStatus = status
        }
    }
}