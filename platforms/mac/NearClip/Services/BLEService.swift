//
//  BLEService.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import Foundation
import CoreBluetooth
import os.log
import Combine

// MARK: - BLE Device Model
struct BLEDevice: Identifiable, Hashable, Codable {
    let id: String
    let name: String
    let rssi: Int
    let peripheral: CBPeripheral?
    
    init(peripheral: CBPeripheral, rssi: NSNumber) {
        self.id = peripheral.identifier.uuidString
        self.name = peripheral.name ?? "Unknown Device"
        self.rssi = rssi.intValue
        self.peripheral = peripheral
    }
    
    init(id: String, name: String, rssi: Int, peripheral: CBPeripheral? = nil) {
        self.id = id
        self.name = name
        self.rssi = rssi
        self.peripheral = peripheral
    }
    
    // 使BLEDevice符合Codable协议以支持持久化
    enum CodingKeys: String, CodingKey {
        case id, name, rssi
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(String.self, forKey: .id)
        name = try container.decode(String.self, forKey: .name)
        rssi = try container.decode(Int.self, forKey: .rssi)
        peripheral = nil // 不序列化peripheral对象
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(id, forKey: .id)
        try container.encode(name, forKey: .name)
        try container.encode(rssi, forKey: .rssi)
    }
}

// MARK: - BLE Service
class BLEService: NSObject, ObservableObject {
    // MARK: - Published Properties
    @Published var isBluetoothEnabled = false
    @Published var isScanning = false
    @Published var discoveredDevices: [BLEDevice] = []
    @Published var pairedDevices: [BLEDevice] = []
    @Published var connectedDevice: BLEDevice?
    @Published var connectionState: CBPeripheralState = .disconnected
    @Published var lastError: String?
    
    // MARK: - Private Properties
    private var centralManager: CBCentralManager!
    private var connectedPeripheral: CBPeripheral?
    private var writeCharacteristic: CBCharacteristic?
    private var readCharacteristic: CBCharacteristic?
    
    // NearClip 服务和特征 UUID
    private let nearClipServiceUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABC")
    private let writeCharacteristicUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABD")
    private let readCharacteristicUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABE")
    
    private var scanTimer: Timer?
    private let userDefaults = UserDefaults.standard
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    override init() {
        super.init()
        print("[BLEService] Initializing BLE service")
        os_log("[BLEService] Initializing BLE service", log: OSLog.default, type: .info)
        centralManager = CBCentralManager(delegate: self, queue: nil)
        print("[BLEService] CBCentralManager created with initial state: \(centralManager.state.rawValue)")
        os_log("[BLEService] CBCentralManager created with initial state: %d", log: OSLog.default, type: .info, centralManager.state.rawValue)
        setupNotificationObservers()
        loadPairedDevices()
        print("[BLEService] BLE service initialization completed")
        os_log("[BLEService] BLE service initialization completed", log: OSLog.default, type: .info)
    }
    
    // MARK: - Public Methods
    func startService() {
        os_log("[BLEService] Starting BLE service - current state: %d", log: OSLog.default, type: .info, centralManager.state.rawValue)
        if centralManager.state == .poweredOn {
            startScanning()
        } else {
            os_log("[BLEService] Bluetooth not ready, will start scanning when powered on", log: OSLog.default, type: .info)
        }
    }
    
    func stopService() {
        print("[BLEService] Stopping BLE service")
        stopScanning()
        disconnectCurrentDevice()
    }
    
    func startScanning() {
        print("[BLEService] startScanning called - current state: \(centralManager.state.rawValue)")
        guard centralManager.state == .poweredOn else {
            print("[BLEService] Cannot start scanning - Bluetooth state: \(centralManager.state.rawValue)")
            os_log("[BLEService] Cannot start scanning - Bluetooth state: %d", log: OSLog.default, type: .error, centralManager.state.rawValue)
            lastError = "蓝牙未启用"
            return
        }
        
        guard !isScanning else { 
            print("[BLEService] Already scanning, ignoring request")
            return 
        }
        
        print("[BLEService] Starting scan for NearClip devices with UUID: \(nearClipServiceUUID.uuidString)")
        os_log("[BLEService] Starting scan for NearClip devices with UUID: %@", log: OSLog.default, type: .info, nearClipServiceUUID.uuidString)
        discoveredDevices.removeAll()
        isScanning = true
        lastError = nil
        
        // 扫描所有设备（不限制服务UUID以便发现更多设备）
        print("[BLEService] Scanning for all peripherals to discover NearClip devices")
        centralManager.scanForPeripherals(
            withServices: nil,  // 扫描所有设备
            options: [CBCentralManagerScanOptionAllowDuplicatesKey: true]  // 允许重复以获取RSSI更新
        )
        
        // 设置扫描超时
        scanTimer = Timer.scheduledTimer(withTimeInterval: 30.0, repeats: false) { _ in
            os_log("[BLEService] Scan timeout reached, stopping scan", log: OSLog.default, type: .info)
            self.stopScanning()
        }
    }
    
    func stopScanning() {
        guard isScanning else { return }
        
        print("[BLEService] Stopping BLE scan")
        centralManager.stopScan()
        isScanning = false
        scanTimer?.invalidate()
        scanTimer = nil
    }
    
    func connectToDevice(_ device: BLEDevice) {
        guard let peripheral = device.peripheral else {
            lastError = "设备不可用"
            return
        }
        
        print("[BLEService] Connecting to device: \(device.name)")
        stopScanning()
        
        connectedPeripheral = peripheral
        peripheral.delegate = self
        centralManager.connect(peripheral, options: nil)
    }
    
    func disconnectCurrentDevice() {
        guard let peripheral = connectedPeripheral else { return }
        
        print("[BLEService] Disconnecting from device")
        centralManager.cancelPeripheralConnection(peripheral)
    }
    
    func pairDevice(_ device: BLEDevice) {
        // 添加到已配对设备列表
        if !pairedDevices.contains(where: { $0.id == device.id }) {
            pairedDevices.append(device)
            savePairedDevices()
            print("[BLEService] Device paired: \(device.name)")
        }
    }
    
    func unpairDevice(_ device: BLEDevice) {
        pairedDevices.removeAll { $0.id == device.id }
        savePairedDevices()
        
        // 如果当前连接的是这个设备，断开连接
        if connectedDevice?.id == device.id {
            disconnectCurrentDevice()
        }
        
        print("[BLEService] Device unpaired: \(device.name)")
    }
    
    func sendClipboardData(_ data: String) {
        guard let characteristic = writeCharacteristic,
              let peripheral = connectedPeripheral,
              peripheral.state == .connected else {
            lastError = "设备未连接"
            return
        }
        
        guard let dataToSend = data.data(using: .utf8) else {
            lastError = "数据编码失败"
            return
        }
        
        print("[BLEService] Sending clipboard data: \(data.prefix(50))...")
        peripheral.writeValue(dataToSend, for: characteristic, type: .withResponse)
    }
    
    // MARK: - Private Methods
    private func setupNotificationObservers() {
        // 监听剪贴板内容变化
        NotificationCenter.default
            .publisher(for: NSNotification.Name("ClipboardContentChanged"))
            .compactMap { $0.object as? String }
            .receive(on: DispatchQueue.main)
            .sink { [weak self] content in
                self?.sendClipboardData(content)
            }
            .store(in: &cancellables)
    }
    
    private func savePairedDevices() {
        let deviceData = pairedDevices.map { device in
            [
                "id": device.id,
                "name": device.name,
                "rssi": device.rssi
            ]
        }
        userDefaults.set(deviceData, forKey: "pairedDevices")
    }
    
    private func loadPairedDevices() {
        guard let deviceData = userDefaults.array(forKey: "pairedDevices") as? [[String: Any]] else {
            return
        }
        
        pairedDevices = deviceData.compactMap { data in
            guard let id = data["id"] as? String,
                  let name = data["name"] as? String,
                  let rssi = data["rssi"] as? Int else {
                return nil
            }
            
            return BLEDevice(
                id: id,
                name: name,
                rssi: rssi,
                peripheral: nil
            )
        }
    }
    
    // 添加模拟设备用于测试UI（因为macOS不能发现自己广播的设备）
    private func addSimulatedDevices() {
        let simulatedDevices = [
            BLEDevice(id: "sim-1", name: "NearClip-iPhone", rssi: -45),
            BLEDevice(id: "sim-2", name: "NearClip-MacBook", rssi: -60),
            BLEDevice(id: "sim-3", name: "NearClip-iPad", rssi: -75)
        ]
        
        DispatchQueue.main.async {
            self.discoveredDevices.append(contentsOf: simulatedDevices)
            print("[BLEService] Added \(simulatedDevices.count) simulated NearClip devices for testing")
        }
    }
}

// MARK: - CBCentralManagerDelegate
extension BLEService: CBCentralManagerDelegate {
    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        DispatchQueue.main.async {
            self.isBluetoothEnabled = central.state == .poweredOn
            print("[BLEService] Bluetooth state changed to: \(central.state.rawValue)")
            
            switch central.state {
            case .poweredOn:
                print("[BLEService] Bluetooth powered on, ready to scan")
                os_log("[BLEService] Bluetooth powered on, ready to scan", log: OSLog.default, type: .info)
                self.lastError = nil
                // 添加模拟设备用于测试（因为macOS不能发现自己广播的设备）
                self.addSimulatedDevices()
                // 自动开始扫描
                if !self.isScanning {
                    print("[BLEService] Auto-starting scan since not currently scanning")
                    self.startScanning()
                }
            case .poweredOff:
                print("[BLEService] Bluetooth powered off")
                os_log("[BLEService] Bluetooth powered off", log: OSLog.default, type: .error)
                self.lastError = "蓝牙已关闭"
                self.stopScanning()
            case .unauthorized:
                print("[BLEService] Bluetooth unauthorized")
                os_log("[BLEService] Bluetooth unauthorized", log: OSLog.default, type: .error)
                self.lastError = "蓝牙权限被拒绝"
            case .unsupported:
                print("[BLEService] Bluetooth unsupported")
                os_log("[BLEService] Bluetooth unsupported", log: OSLog.default, type: .error)
                self.lastError = "设备不支持蓝牙"
            case .resetting:
                print("[BLEService] Bluetooth resetting")
                os_log("[BLEService] Bluetooth resetting", log: OSLog.default, type: .info)
                self.lastError = "蓝牙正在重置"
            case .unknown:
                print("[BLEService] Bluetooth state unknown")
                os_log("[BLEService] Bluetooth state unknown", log: OSLog.default, type: .error)
                self.lastError = "蓝牙状态未知"
            @unknown default:
                print("[BLEService] Bluetooth state unknown default")
                os_log("[BLEService] Bluetooth state unknown default", log: OSLog.default, type: .error)
                self.lastError = "蓝牙状态未知"
            }
        }
    }
    
    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {
        print("[BLEService] Discovered device: \(peripheral.name ?? "Unknown") (RSSI: \(RSSI))")
        print("[BLEService] Advertisement data: \(advertisementData)")
        
        // 检查是否是NearClip设备
        var isNearClipDevice = false
        
        // 检查服务UUID
        if let serviceUUIDs = advertisementData[CBAdvertisementDataServiceUUIDsKey] as? [CBUUID] {
            isNearClipDevice = serviceUUIDs.contains(nearClipServiceUUID)
            print("[BLEService] Service UUIDs: \(serviceUUIDs), contains NearClip UUID: \(isNearClipDevice)")
        }
        
        // 检查设备名称
        if let localName = advertisementData[CBAdvertisementDataLocalNameKey] as? String {
            if localName.hasPrefix("NearClip-") {
                isNearClipDevice = true
                print("[BLEService] Found NearClip device by name: \(localName)")
            }
        }
        
        // 也检查peripheral.name
        if let peripheralName = peripheral.name, peripheralName.hasPrefix("NearClip-") {
            isNearClipDevice = true
            print("[BLEService] Found NearClip device by peripheral name: \(peripheralName)")
        }
        
        guard isNearClipDevice else {
            print("[BLEService] Ignoring non-NearClip device: \(peripheral.name ?? "Unknown")")
            return
        }
        
        os_log("[BLEService] Discovered NearClip device: %@ (%@) RSSI: %@", log: OSLog.default, type: .info, peripheral.name ?? "Unknown", peripheral.identifier.uuidString, RSSI)
        os_log("[BLEService] Advertisement data: %@", log: OSLog.default, type: .debug, String(describing: advertisementData))
        
        let device = BLEDevice(peripheral: peripheral, rssi: RSSI)
        
        DispatchQueue.main.async {
            // 避免重复添加
            if !self.discoveredDevices.contains(where: { $0.id == device.id }) {
                self.discoveredDevices.append(device)
                print("[BLEService] Added new NearClip device to list: \(device.name)")
            } else {
                // 更新 RSSI
                if let index = self.discoveredDevices.firstIndex(where: { $0.id == device.id }) {
                    self.discoveredDevices[index] = device
                    print("[BLEService] Updated RSSI for NearClip device: \(device.name)")
                }
            }
        }
    }
    
    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        print("[BLEService] Connected to peripheral: \(peripheral.name ?? "Unknown")")
        
        DispatchQueue.main.async {
            self.connectionState = .connected
            self.connectedDevice = self.discoveredDevices.first { $0.peripheral?.identifier == peripheral.identifier }
            self.lastError = nil
        }
        
        // 发现服务
        peripheral.discoverServices([nearClipServiceUUID])
    }
    
    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        print("[BLEService] Failed to connect: \(error?.localizedDescription ?? "Unknown error")")
        
        DispatchQueue.main.async {
            self.connectionState = .disconnected
            self.connectedDevice = nil
            self.lastError = "连接失败: \(error?.localizedDescription ?? "未知错误")"
        }
    }
    
    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        print("[BLEService] Disconnected from peripheral: \(peripheral.name ?? "Unknown")")
        
        DispatchQueue.main.async {
            self.connectionState = .disconnected
            self.connectedDevice = nil
            self.connectedPeripheral = nil
            self.writeCharacteristic = nil
            self.readCharacteristic = nil
            
            if let error = error {
                self.lastError = "连接断开: \(error.localizedDescription)"
            }
        }
    }
}

// MARK: - CBPeripheralDelegate
extension BLEService: CBPeripheralDelegate {
    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        guard error == nil else {
            print("[BLEService] Error discovering services: \(error!.localizedDescription)")
            return
        }
        
        guard let services = peripheral.services else { return }
        
        for service in services {
            if service.uuid == nearClipServiceUUID {
                print("[BLEService] Found NearClip service")
                peripheral.discoverCharacteristics([writeCharacteristicUUID, readCharacteristicUUID], for: service)
            }
        }
    }
    
    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        guard error == nil else {
            print("[BLEService] Error discovering characteristics: \(error!.localizedDescription)")
            return
        }
        
        guard let characteristics = service.characteristics else { return }
        
        for characteristic in characteristics {
            switch characteristic.uuid {
            case writeCharacteristicUUID:
                writeCharacteristic = characteristic
                print("[BLEService] Found write characteristic")
            case readCharacteristicUUID:
                readCharacteristic = characteristic
                peripheral.setNotifyValue(true, for: characteristic)
                print("[BLEService] Found read characteristic")
            default:
                break
            }
        }
    }
    
    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        guard error == nil else {
            print("[BLEService] Error reading characteristic: \(error!.localizedDescription)")
            return
        }
        
        guard let data = characteristic.value,
              let receivedString = String(data: data, encoding: .utf8) else {
            return
        }
        
        print("[BLEService] Received data: \(receivedString.prefix(50))...")
        
        // 通知剪贴板服务更新剪贴板内容
        DispatchQueue.main.async {
            NotificationCenter.default.post(
                name: NSNotification.Name("ClipboardDataReceived"),
                object: receivedString
            )
        }
    }
    
    func peripheral(_ peripheral: CBPeripheral, didWriteValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            print("[BLEService] Error writing characteristic: \(error.localizedDescription)")
            DispatchQueue.main.async {
                self.lastError = "发送失败: \(error.localizedDescription)"
            }
        } else {
            print("[BLEService] Successfully wrote data to characteristic")
        }
    }
}