import Foundation
import SwiftUI

// FFI 结果枚举
enum NearclipResult: Int32 {
    case success = 0
    case invalidArgument = 1
    case internalError = 2
    case connectionFailed = 3
    case deviceNotFound = 4
}

// FFI 设备结构体
struct NearclipDevice {
    let deviceId: UnsafePointer<Int8>
    let deviceName: UnsafePointer<Int8>
    let deviceType: NearclipDeviceType
    let isConnected: Bool
    let lastSeen: TimeInterval
    let batteryLevel: UInt8
}

// FFI 设备类型
enum NearclipDeviceType: Int32 {
    case unknown = 0
    case android = 1
    case mac = 2
}

// FFI 函数声明（将在Rust实现中提供）
@_silgen_name("nearclip_start_discovery")
func nearclip_start_discovery(_ callback: @convention(c) (UnsafePointer<NearclipDevice>) -> Void) -> NearclipResult

@_silgen_name("nearclip_stop_discovery")
func nearclip_stop_discovery() -> NearclipResult

@_silgen_name("nearclip_connect_to_device")
func nearclip_connect_to_device(_ deviceId: UnsafePointer<Int8>) -> NearclipResult

@_silgen_name("nearclip_disconnect_from_device")
func nearclip_disconnect_from_device(_ deviceId: UnsafePointer<Int8>) -> NearclipResult

@_silgen_name("nearclip_set_sync_callback")
func nearclip_set_sync_callback(_ callback: @convention(c) (UnsafePointer<Int8>, UnsafePointer<Int8>, Int32) -> Void) -> NearclipResult

// FFI 错误类型
enum NearclipError: Error, LocalizedError {
    case initializationFailed(String)
    case discoveryFailed(String)
    case connectionFailed(String)
    case deviceNotFound(String)
    case invalidArgument(String)

    var errorDescription: String? {
        switch self {
        case .initializationFailed(let message):
            return "初始化失败: \(message)"
        case .discoveryFailed(let message):
            return "设备发现失败: \(message)"
        case .connectionFailed(let message):
            return "连接失败: \(message)"
        case .deviceNotFound(let message):
            return "设备未找到: \(message)"
        case .invalidArgument(let message):
            return "参数无效: \(message)"
        }
    }
}

// NearClip 管理器
class NearClipManager: ObservableObject {
    @Published var isInitialized: Bool = false
    @Published var isDiscovering: Bool = false
    @Published var connectedDevices: [Device] = []
    @Published var lastError: NearclipError?

    private var deviceCallback: ((Device) -> Void)?
    private var syncCallback: ((String, Data) -> Void)?
    private var discoveredDevices: Set<String> = []

    init() {
        initialize()
    }

    deinit {
        if isDiscovering {
            stopDeviceDiscovery()
        }
    }

    // 初始化
    private func initialize() {
        print("正在初始化 NearClip 管理器...")

        // 目前只是标记为已初始化，实际Rust集成将在后续任务中实现
        isInitialized = true

        print("NearClip 管理器初始化完成")
    }

    // 开始设备发现
    func startDeviceDiscovery(callback: @escaping (Device) -> Void) throws {
        guard isInitialized else {
            throw NearclipError.initializationFailed("NearClip 管理器未初始化")
        }

        guard !isDiscovering else {
            throw NearclipError.invalidArgument("设备发现已在进行中")
        }

        deviceCallback = callback
        isDiscovering = true
        discoveredDevices.removeAll()

        print("开始设备发现...")

        // 模拟设备发现（实际FFI调用将在Rust集成任务中实现）
        simulateDeviceDiscovery()
    }

    // 停止设备发现
    func stopDeviceDiscovery() {
        guard isDiscovering else { return }

        print("停止设备发现")
        isDiscovering = false
        deviceCallback = nil

        // 实际FFI调用: nearclip_stop_discovery()
    }

    // 连接到设备
    func connectToDevice(_ device: Device) async throws {
        guard isInitialized else {
            throw NearclipError.initializationFailed("NearClip 管理器未初始化")
        }

        print("正在连接到设备: \(device.name)")

        // 模拟连接过程
        await simulateConnection(to: device)
    }

    // 断开设备连接
    func disconnectFromDevice(_ device: Device) {
        guard let index = connectedDevices.firstIndex(where: { $0.id == device.id }) else {
            return
        }

        print("正在断开设备连接: \(device.name)")

        var updatedDevice = connectedDevices[index]
        updatedDevice.connectionStatus = .disconnected
        connectedDevices[index] = updatedDevice

        // 实际FFI调用: nearclip_disconnect_from_device()
    }

    // 设置剪贴板同步回调
    func setSyncCallback(_ callback: @escaping (String, Data) -> Void) {
        syncCallback = callback

        // 实际FFI调用: nearclip_set_sync_callback()
    }

    // 模拟设备发现（临时实现）
    private func simulateDeviceDiscovery() {
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) { [weak self] in
            guard let self = self, self.isDiscovering else { return }

            // 创建模拟设备
            let mockDevice = Device(
                id: "mock-android-device",
                name: "Mock Android 设备",
                type: .android,
                connectionStatus: .disconnected,
                lastSeen: Date(),
                capabilities: [.ble, .clipboardRead, .clipboardWrite],
                batteryLevel: 85
            )

            self.deviceCallback?(mockDevice)
            print("发现模拟设备: \(mockDevice.name)")
        }
    }

    // 模拟连接过程（临时实现）
    private func simulateConnection(to device: Device) async {
        // 添加到连接设备列表
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            var connectedDevice = device
            connectedDevice.connectionStatus = .pairing
            self.connectedDevices.append(connectedDevice)
        }

        // 模拟连接延迟
        try? await Task.sleep(nanoseconds: 2_000_000_000)

        // 更新为已连接状态
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            if let index = self.connectedDevices.firstIndex(where: { $0.id == device.id }) {
                self.connectedDevices[index].connectionStatus = .connected
                print("成功连接到设备: \(device.name)")
            }
        }
    }

    // FFI 安全封装：转换C设备结构体
    private func convertCDevice(_ cDevice: UnsafePointer<NearclipDevice>) -> Device {
        let deviceId = String(cString: cDevice.pointee.deviceId)
        let deviceName = String(cString: cDevice.pointee.deviceName)
        let deviceType: DeviceType

        switch cDevice.pointee.deviceType {
        case .android:
            deviceType = .android
        case .mac:
            deviceType = .mac
        default:
            deviceType = .unknown
        }

        let connectionStatus: ConnectionStatus = cDevice.pointee.isConnected ? .connected : .disconnected
        let lastSeen = Date(timeIntervalSince1970: cDevice.pointee.lastSeen)
        let batteryLevel = cDevice.pointee.batteryLevel != 0 ? Int(cDevice.pointee.batteryLevel) : nil

        return Device(
            id: deviceId,
            name: deviceName,
            type: deviceType,
            connectionStatus: connectionStatus,
            lastSeen: lastSeen,
            capabilities: [.ble, .clipboardRead, .clipboardWrite], // 默认能力
            batteryLevel: batteryLevel
        )
    }
}