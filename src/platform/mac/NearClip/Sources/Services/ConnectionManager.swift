import Foundation
import CoreBluetooth
import Combine

/**
 * 连接管理器 - 负责BLE设备连接和通信
 */
class ConnectionManager: NSObject, ObservableObject, CBCentralManagerDelegate, CBPeripheralDelegate {

    // MARK: - Properties

    /// 蓝牙管理器
    private var centralManager: CBCentralManager!

    /// 连接超时时间（秒）
    private let connectionTimeout: TimeInterval = 10.0

    /// 连接状态
    @Published var connectionStates: [String: ConnectionState] = [:]

    /// 活跃连接
    @Published var activeConnections: [String: CBPeripheral] = [:]

    /// 收到的消息
    @Published var receivedMessages: [TestMessage] = []

    /// 连接超时定时器
    private var connectionTimeouts: [String: Timer] = [:]

    /// Protocol Buffers消息处理器
    private let protocolHandler = ProtocolBufferMessageHandler()

    /// 连接回调
    var onConnectionStateChanged: ((String, ConnectionState) -> Void)?

    /// 消息接收回调
    var onMessageReceived: ((TestMessage) -> Void)?

    /// 连接超时回调
    var onConnectionTimeout: ((String) -> Void)?

    // MARK: - Constants

    private let serviceUUID = CBUUID(string: "0000FE2C-0000-1000-8000-00805F9B34FB")
    private let characteristicUUID = CBUUID(string: "0000FE2D-0000-1000-8000-00805F9B34FB")
    private let descriptorUUID = CBUUID(string: "00002902-0000-1000-8000-00805F9B34FB")

    // MARK: - Initialization

    override init() {
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    deinit {
        // 清理所有连接
        activeConnections.values.forEach { peripheral in
            centralManager.cancelPeripheralConnection(peripheral)
        }

        // 取消所有超时定时器
        connectionTimeouts.values.forEach { $0.invalidate() }
    }

    // MARK: - Public Methods

    /// 连接到设备
    func connect(to peripheral: CBPeripheral) {
        let deviceId = peripheral.identifier.uuidString

        guard centralManager.state == .poweredOn else {
            print("❌ 蓝牙未开启，无法连接")
            updateConnectionState(for: deviceId, state: .failed)
            return
        }

        guard activeConnections[deviceId] == nil else {
            print("⚠️ 设备已连接: \(peripheral.name ?? "未知设备")")
            return
        }

        print("🔗 开始连接设备: \(peripheral.name ?? "未知设备")")

        // 设置连接状态
        updateConnectionState(for: deviceId, state: .connecting)

        // 设置peripheral代理
        peripheral.delegate = self

        // 启动连接超时定时器
        startConnectionTimeout(for: deviceId)

        // 开始连接
        centralManager.connect(peripheral, options: nil)
    }

    /// 断开设备连接
    func disconnect(from peripheral: CBPeripheral) {
        let deviceId = peripheral.identifier.uuidString

        print("🔌 断开设备连接: \(peripheral.name ?? "未知设备")")

        // 取消超时定时器
        cancelConnectionTimeout(for: deviceId)

        // 断开连接
        centralManager.cancelPeripheralConnection(peripheral)
    }

    /// 发送消息
    func sendMessage(_ message: TestMessage, to deviceId: String) -> Bool {
        guard let peripheral = activeConnections[deviceId] else {
            print("❌ 设备未连接: \(deviceId)")
            return false
        }

        guard let connectionState = connectionStates[deviceId], connectionState == .connected else {
            print("❌ 设备连接状态不正确: \(deviceId)")
            return false
        }

        // 序列化消息
        guard let messageData = serializeMessage(message) else {
            print("❌ 消息序列化失败")
            return false
        }

        // 查找可写的特征值
        guard let characteristic = findWritableCharacteristic(in: peripheral) else {
            print("❌ 找不到可写的特征值")
            return false
        }

        print("📤 发送消息: \(message.messageId) 到设备: \(deviceId)")

        // 发送消息
        peripheral.writeValue(messageData, for: characteristic, type: .withResponse)
        return true
    }

    /// 发送Ping消息
    func sendPing(to deviceId: String) -> Bool {
        let pingMessage = TestMessage(
            messageId: "ping-\(UUID().uuidString)",
            type: .ping,
            payload: "",
            timestamp: Date().timeIntervalSince1970,
            sequenceNumber: 0
        )

        return sendMessage(pingMessage, to: deviceId)
    }

    /// 发送Pong消息
    func sendPong(to deviceId: String, originalMessageId: String) -> Bool {
        let pongMessage = TestMessage(
            messageId: "pong-\(UUID().uuidString)",
            type: .pong,
            payload: originalMessageId,
            timestamp: Date().timeIntervalSince1970,
            sequenceNumber: 0
        )

        return sendMessage(pongMessage, to: deviceId)
    }

    /// 发送数据消息
    func sendData(to deviceId: String, payload: String) -> Bool {
        let dataMessage = TestMessage(
            messageId: "data-\(UUID().uuidString)",
            type: .data,
            payload: payload,
            timestamp: Date().timeIntervalSince1970,
            sequenceNumber: 1
        )

        return sendMessage(dataMessage, to: deviceId)
    }

    /// 获取连接统计信息
    func getConnectionStats() -> ConnectionStats {
        let totalConnections = activeConnections.count
        let connectedCount = connectionStates.values.filter { $0 == .connected }.count
        let connectingCount = connectionStates.values.filter { $0 == .connecting }.count
        let failedCount = connectionStates.values.filter { $0 == .failed }.count

        return ConnectionStats(
            totalConnections: totalConnections,
            connectedCount: connectedCount,
            connectingCount: connectingCount,
            failedCount: failedCount
        )
    }

    // MARK: - CBCentralManagerDelegate

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        switch central.state {
        case .poweredOn:
            print("✅ 蓝牙管理器已就绪")
        case .poweredOff:
            print("❌ 蓝牙已关闭，断开所有连接")
            disconnectAllDevices()
        case .unauthorized:
            print("❌ 蓝牙权限未授权")
        case .unsupported:
            print("❌ 设备不支持BLE")
        case .resetting:
            print("⚠️ 蓝牙正在重置")
        case .unknown:
            print("❓ 蓝牙状态未知")
        @unknown default:
            print("❓ 未知的蓝牙状态")
        }
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        let deviceId = peripheral.identifier.uuidString

        print("✅ 设备连接成功: \(peripheral.name ?? "未知设备")")

        // 取消超时定时器
        cancelConnectionTimeout(for: deviceId)

        // 更新连接状态
        updateConnectionState(for: deviceId, state: .connected)

        // 添加到活跃连接
        activeConnections[deviceId] = peripheral

        // 发现服务
        peripheral.discoverServices([serviceUUID])
    }

    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        let deviceId = peripheral.identifier.uuidString

        print("❌ 设备连接失败: \(peripheral.name ?? "未知设备")")
        if let error = error {
            print("错误信息: \(error.localizedDescription)")
        }

        // 取消超时定时器
        cancelConnectionTimeout(for: deviceId)

        // 更新连接状态
        updateConnectionState(for: deviceId, state: .failed)
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        let deviceId = peripheral.identifier.uuidString

        print("🔌 设备已断开: \(peripheral.name ?? "未知设备")")

        // 取消超时定时器
        cancelConnectionTimeout(for: deviceId)

        // 从活跃连接中移除
        activeConnections.removeValue(forKey: deviceId)

        // 更新连接状态
        updateConnectionState(for: deviceId, state: .disconnected)

        if let error = error {
            print("断开原因: \(error.localizedDescription)")
        }
    }

    // MARK: - CBPeripheralDelegate

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error = error {
            print("❌ 服务发现失败: \(error.localizedDescription)")
            return
        }

        print("🔍 发现服务: \(peripheral.services?.count ?? 0) 个")

        peripheral.services?.forEach { service in
            print("  服务: \(service.uuid.uuidString)")

            // 发现特征值
            peripheral.discoverCharacteristics([characteristicUUID], for: service)
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error = error {
            print("❌ 特征值发现失败: \(error.localizedDescription)")
            return
        }

        print("🔍 发现特征值: \(service.characteristics?.count ?? 0) 个")

        service.characteristics?.forEach { characteristic in
            print("  特征值: \(characteristic.uuid.uuidString) (属性: \(characteristic.properties))")

            // 如果特征值支持通知，订阅它
            if characteristic.properties.contains(.notify) {
                peripheral.setNotifyValue(true, for: characteristic)
                print("  ✅ 已订阅通知")
            }
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            print("❌ 特征值更新失败: \(error.localizedDescription)")
            return
        }

        guard let data = characteristic.value else {
            print("❌ 特征值数据为空")
            return
        }

        // 解析消息
        if let message = deserializeMessage(data) {
            print("📨 收到消息: \(message.messageId) [\(message.type)]")

            // 添加到消息列表
            DispatchQueue.main.async {
                self.receivedMessages.append(message)
                // 限制消息列表大小
                if self.receivedMessages.count > 100 {
                    self.receivedMessages.removeFirst()
                }
            }

            // 触发回调
            onMessageReceived?(message)

            // 自动回复某些消息
            handleAutoReply(for: message, from: peripheral)
        } else {
            print("❌ 消息解析失败")
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didWriteValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            print("❌ 特征值写入失败: \(error.localizedDescription)")
        } else {
            print("✅ 特征值写入成功")
        }
    }

    // MARK: - Private Methods

    /// 更新连接状态
    private func updateConnectionState(for deviceId: String, state: ConnectionState) {
        connectionStates[deviceId] = state
        onConnectionStateChanged?(deviceId, state)
    }

    /// 启动连接超时定时器
    private func startConnectionTimeout(for deviceId: String) {
        connectionTimeouts[deviceId] = Timer.scheduledTimer(withTimeInterval: connectionTimeout, repeats: false) { [weak self] _ in
            self?.handleConnectionTimeout(for: deviceId)
        }
    }

    /// 取消连接超时定时器
    private func cancelConnectionTimeout(for deviceId: String) {
        connectionTimeouts[deviceId]?.invalidate()
        connectionTimeouts.removeValue(forKey: deviceId)
    }

    /// 处理连接超时
    private func handleConnectionTimeout(for deviceId: String) {
        print("⏰ 连接超时: \(deviceId)")

        updateConnectionState(for: deviceId, state: .failed)

        // 断开连接
        if let peripheral = activeConnections[deviceId] {
            centralManager.cancelPeripheralConnection(peripheral)
        }

        // 触发超时回调
        onConnectionTimeout?(deviceId)

        // 清理定时器
        cancelConnectionTimeout(for: deviceId)
    }

    /// 查找可写的特征值
    private func findWritableCharacteristic(in peripheral: CBPeripheral) -> CBCharacteristic? {
        return peripheral.services?.first { $0.uuid == serviceUUID }
            ?.characteristics?.first { $0.uuid == characteristicUUID && $0.properties.contains(.write) }
    }

    /// 序列化消息
    private func serializeMessage(_ message: TestMessage) -> Data? {
        // 验证消息格式
        guard protocolHandler.validateMessage(message) else {
            print("❌ 消息验证失败: \(message)")
            return nil
        }

        return protocolHandler.serializeMessage(message)
    }

    /// 反序列化消息
    private func deserializeMessage(_ data: Data) -> TestMessage? {
        return protocolHandler.deserializeMessage(data)
    }

    /// 自动回复消息
    private func handleAutoReply(for message: TestMessage, from peripheral: CBPeripheral) {
        let deviceId = peripheral.identifier.uuidString

        switch message.type {
        case .ping:
            print("🏓 收到Ping，自动回复Pong")
            sendPong(to: deviceId, originalMessageId: message.messageId)

        case .data:
            print("📄 收到数据消息，自动回复ACK")
            sendAck(to: deviceId, originalMessageId: message.messageId)

        case .pong, .ack:
            // 不需要回复
            break
        }
    }

    /// 发送ACK消息
    private func sendAck(to deviceId: String, originalMessageId: String) -> Bool {
        let ackMessage = TestMessage(
            messageId: "ack-\(UUID().uuidString)",
            type: .ack,
            payload: originalMessageId,
            timestamp: Date().timeIntervalSince1970,
            sequenceNumber: 2
        )

        return sendMessage(ackMessage, to: deviceId)
    }

    /// 断开所有设备
    private func disconnectAllDevices() {
        activeConnections.values.forEach { peripheral in
            centralManager.cancelPeripheralConnection(peripheral)
        }
        activeConnections.removeAll()
        connectionStates.removeAll()

        // 取消所有超时定时器
        connectionTimeouts.values.forEach { $0.invalidate() }
        connectionTimeouts.removeAll()
    }
}

// MARK: - Supporting Types

/// 连接状态
enum ConnectionState: String, CaseIterable {
    case disconnected = "已断开"
    case connecting = "连接中"
    case connected = "已连接"
    case failed = "连接失败"
}

/// 测试消息
struct TestMessage: Identifiable, Codable {
    let id = UUID()
    let messageId: String
    let type: MessageType
    let payload: String
    let timestamp: Double
    let sequenceNumber: Int

    var timeString: String {
        let date = Date(timeIntervalSince1970: timestamp)
        let formatter = DateFormatter()
        formatter.timeStyle = .medium
        return formatter.string(from: date)
    }
}

/// 消息类型
enum MessageType: String, CaseIterable, Codable {
    case ping = "PING"
    case pong = "PONG"
    case data = "DATA"
    case ack = "ACK"

    var displayName: String {
        switch self {
        case .ping: return "Ping"
        case .pong: return "Pong"
        case .data: return "数据"
        case .ack: return "确认"
        }
    }
}

/// 连接统计信息
struct ConnectionStats {
    let totalConnections: Int
    let connectedCount: Int
    let connectingCount: Int
    let failedCount: Int
}