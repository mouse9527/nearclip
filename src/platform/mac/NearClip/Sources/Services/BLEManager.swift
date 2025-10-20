import Foundation
import CoreBluetooth
import Combine

/**
 * BLE管理器 - 主要的BLE管理类
 * 负责扫描、广播和连接管理
 */
class BLEManager: NSObject, ObservableObject, CBCentralManagerDelegate {

    // MARK: - Properties

    /// 核心蓝牙管理器
    private var centralManager: CBCentralManager!

    /// 发现的设备列表
    @Published var discoveredPeripherals: [CBPeripheral] = []

    /// 已连接的设备列表
    @Published var connectedPeripherals: [CBPeripheral] = []

    /// BLE状态
    @Published var bluetoothState: CBManagerState = .unknown

    /// 扫描状态
    @Published var isScanning: Bool = false

    /// 广播状态
    @Published var isAdvertising: Bool = false

    /// 设备发现回调
    var onDeviceDiscovered: ((CBPeripheral, [String: Any], NSNumber) -> Void)?

    /// 连接状态变化回调
    var onConnectionStateChanged: ((CBPeripheral, Bool) -> Void)?

    // MARK: - Initialization

    override init() {
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    // MARK: - Public Methods

    /// 开始扫描设备
    func startScanning() {
        guard centralManager.state == .poweredOn else {
            print("❌ 蓝牙未开启")
            return
        }

        guard !isScanning else {
            print("⚠️ 已在扫描中")
            return
        }

        print("🔍 开始扫描BLE设备...")

        // 扫描所有设备，不限制服务UUID
        centralManager.scanForPeripherals(withServices: nil, options: [
            CBCentralManagerScanOptionAllowDuplicatesKey: false
        ])

        isScanning = true
    }

    /// 停止扫描
    func stopScanning() {
        guard isScanning else { return }

        print("🛑 停止扫描")
        centralManager.stopScan()
        isScanning = false
    }

    /// 连接到设备
    func connect(to peripheral: CBPeripheral) {
        guard centralManager.state == .poweredOn else {
            print("❌ 蓝牙未开启，无法连接")
            return
        }

        guard !connectedPeripherals.contains(peripheral) else {
            print("⚠️ 设备已连接: \(peripheral.name ?? "未知设备")")
            return
        }

        print("🔗 连接到设备: \(peripheral.name ?? "未知设备")")
        peripheral.delegate = self
        centralManager.connect(peripheral, options: nil)
    }

    /// 断开设备连接
    func disconnect(from peripheral: CBPeripheral) {
        guard connectedPeripherals.contains(peripheral) else { return }

        print("🔌 断开设备连接: \(peripheral.name ?? "未知设备")")
        centralManager.cancelPeripheralConnection(peripheral)
    }

    /// 获取NearClip设备列表
    func getNearClipDevices() -> [CBPeripheral] {
        return discoveredPeripherals.filter { peripheral in
            peripheral.name?.contains("NearClip", options: .caseInsensitive) == true ||
            peripheral.name?.contains("NearClip", options: .caseInsensitive) == true
        }
    }

    /// 清除设备列表
    func clearDevices() {
        discoveredPeripherals.removeAll()
        connectedPeripherals.removeAll()
    }

    // MARK: - CBCentralManagerDelegate

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        bluetoothState = central.state

        switch central.state {
        case .poweredOn:
            print("✅ 蓝牙已开启")
        case .poweredOff:
            print("❌ 蓝牙已关闭")
            stopScanning()
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

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {

        // 忽略没有名称的设备
        guard peripheral.name != nil || (advertisementData[CBAdvertisementDataLocalNameKey] as? String) != nil else {
            return
        }

        // 检查是否已存在
        if !discoveredPeripherals.contains(where: { $0.identifier == peripheral.identifier }) {
            discoveredPeripherals.append(peripheral)

            let deviceName = peripheral.name ?? (advertisementData[CBAdvertisementDataLocalNameKey] as? String) ?? "未知设备"
            print("📱 发现设备: \(deviceName) (RSSI: \(RSSI)dBm)")

            // 触发回调
            onDeviceDiscovered?(peripheral, advertisementData, RSSI)
        }
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        print("✅ 设备连接成功: \(peripheral.name ?? "未知设备")")

        if !connectedPeripherals.contains(peripheral) {
            connectedPeripherals.append(peripheral)
        }

        // 触发连接状态变化回调
        onConnectionStateChanged?(peripheral, true)

        // 开始发现服务
        peripheral.discoverServices(nil)
    }

    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        print("❌ 设备连接失败: \(peripheral.name ?? "未知设备")")
        if let error = error {
            print("错误信息: \(error.localizedDescription)")
        }

        // 触发连接状态变化回调
        onConnectionStateChanged?(peripheral, false)
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        print("🔌 设备已断开: \(peripheral.name ?? "未知设备")")

        // 从连接列表中移除
        connectedPeripherals.removeAll { $0.identifier == peripheral.identifier }

        if let error = error {
            print("断开原因: \(error.localizedDescription)")
        }

        // 触发连接状态变化回调
        onConnectionStateChanged?(peripheral, false)
    }
}

// MARK: - CBPeripheralDelegate Extension

extension BLEManager: CBPeripheralDelegate {

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error = error {
            print("❌ 服务发现失败: \(error.localizedDescription)")
            return
        }

        print("🔍 发现服务: \(peripheral.services?.count ?? 0) 个")

        peripheral.services?.forEach { service in
            print("  服务: \(service.uuid)")

            // 发现特征值
            peripheral.discoverCharacteristics(nil, for: service)
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error = error {
            print("❌ 特征值发现失败: \(error.localizedDescription)")
            return
        }

        print("🔍 发现特征值: \(service.characteristics?.count ?? 0) 个")

        service.characteristics?.forEach { characteristic in
            print("  特征值: \(characteristic.uuid) (属性: \(characteristic.properties))")

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

        if let data = characteristic.value {
            let messageString = String(data: data, encoding: .utf8) ?? "无法解码"
            print("📨 收到消息: \(messageString)")

            // 解析消息并处理
            handleReceivedMessage(data, from: peripheral)
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

    private func handleReceivedMessage(_ data: Data, from peripheral: CBPeripheral) {
        guard let messageString = String(data: data, encoding: .utf8) else {
            print("❌ 无法解析消息数据")
            return
        }

        let parts = messageString.split(separator: "|")
        guard parts.count >= 3 else {
            print("❌ 消息格式不正确")
            return
        }

        let messageId = String(parts[0])
        let messageTypeString = String(parts[1])
        let payload = String(parts[2])

        print("📨 解析消息:")
        print("  ID: \(messageId)")
        print("  类型: \(messageTypeString)")
        print("  内容: \(payload)")

        // 根据消息类型处理
        switch messageTypeString {
        case "PING":
            print("🏓 收到Ping，发送Pong回复")
            sendPong(to: peripheral, originalMessageId: messageId)
        case "PONG":
            print("🏓 收到Pong回复")
        case "DATA":
            print("📄 收到数据消息: \(payload)")
            sendAck(to: peripheral, originalMessageId: messageId)
        case "ACK":
            print("✅ 收到确认消息")
        default:
            print("❓ 未知消息类型: \(messageTypeString)")
        }
    }

    private func sendPong(to peripheral: CBPeripheral, originalMessageId: String) {
        let pongMessage = "pong-\(UUID().uuidString)|PONG|\(originalMessageId)|\(Date().timeIntervalSince1970)|0"

        guard let data = pongMessage.data(using: .utf8) else { return }

        // 查找合适的特征值发送消息
        peripheral.services?.forEach { service in
            service.characteristics?.forEach { characteristic in
                if characteristic.properties.contains(.write) {
                    peripheral.writeValue(data, for: characteristic, type: .withResponse)
                    return
                }
            }
        }
    }

    private func sendAck(to peripheral: CBPeripheral, originalMessageId: String) {
        let ackMessage = "ack-\(UUID().uuidString)|ACK|\(originalMessageId)|\(Date().timeIntervalSince1970)|0"

        guard let data = ackMessage.data(using: .utf8) else { return }

        // 查找合适的特征值发送消息
        peripheral.services?.forEach { service in
            service.characteristics?.forEach { characteristic in
                if characteristic.properties.contains(.write) {
                    peripheral.writeValue(data, for: characteristic, type: .withResponse)
                    return
                }
            }
        }
    }
}