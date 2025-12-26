import Foundation
import CoreBluetooth
import Combine
import CryptoKit
import os.log

// MARK: - Logging

private let bleLog = OSLog(subsystem: "com.nearclip", category: "BLE")

// MARK: - Data Extension for SHA256

extension Data {
    /// Returns SHA256 hash as a Base64 encoded string
    var sha256Hash: String {
        let hash = SHA256.hash(data: self)
        return Data(hash).base64EncodedString()
    }
}

// MARK: - BLE Constants

/// NearClip BLE Service and Characteristic UUIDs
/// Must match the UUIDs defined in nearclip-ble/src/gatt.rs
enum BleUUID {
    static let service = CBUUID(string: "4E454152-434C-4950-0000-000000000001")
    static let deviceId = CBUUID(string: "4E454152-434C-4950-0000-000000000002")
    static let publicKeyHash = CBUUID(string: "4E454152-434C-4950-0000-000000000003")
    static let dataTransfer = CBUUID(string: "4E454152-434C-4950-0000-000000000004")
    static let dataAck = CBUUID(string: "4E454152-434C-4950-0000-000000000005")
}

// MARK: - BLE Device

/// Represents a discovered BLE device
struct BleDevice: Identifiable, Equatable {
    let id: String          // Device ID from characteristic
    let peripheral: CBPeripheral
    let peripheralUuid: String
    var publicKeyHash: String?
    var rssi: Int
    var lastSeen: Date

    static func == (lhs: BleDevice, rhs: BleDevice) -> Bool {
        lhs.id == rhs.id
    }
}

// MARK: - BLE Manager Delegate

protocol BleManagerDelegate: AnyObject {
    func bleManager(_ manager: BleManager, didDiscoverDevice peripheralUuid: String, deviceId: String?, publicKeyHash: String?, rssi: Int)
    func bleManager(_ manager: BleManager, didLoseDevice peripheralUuid: String)
    func bleManager(_ manager: BleManager, didConnectDevice peripheralUuid: String, deviceId: String)
    func bleManager(_ manager: BleManager, didDisconnectDevice peripheralUuid: String, deviceId: String?)
    func bleManager(_ manager: BleManager, didReceiveData data: Data, fromPeripheral peripheralUuid: String)
    func bleManager(_ manager: BleManager, didFailWithError error: Error, forPeripheral peripheralUuid: String?)
}

// MARK: - BLE Manager

/// Simplified BLE Manager - Hardware abstraction layer only
/// Business logic (reconnection, health monitoring, etc.) is handled by Rust BleController
final class BleManager: NSObject, ObservableObject {

    // MARK: - Properties

    weak var delegate: BleManagerDelegate?

    @Published private(set) var centralState: CBManagerState = .unknown
    @Published private(set) var peripheralState: CBManagerState = .unknown
    @Published private(set) var isScanning = false
    @Published private(set) var isAdvertising = false

    private var centralManager: CBCentralManager!
    private var peripheralManager: CBPeripheralManager!
    private let bleQueue = DispatchQueue(label: "com.nearclip.ble", qos: .userInitiated)

    // Peripheral mode properties
    private var advertisedService: CBMutableService?
    private var deviceIdCharacteristic: CBMutableCharacteristic?
    private var publicKeyHashCharacteristic: CBMutableCharacteristic?
    private var dataTransferCharacteristic: CBMutableCharacteristic?
    private var dataAckCharacteristic: CBMutableCharacteristic?

    // Peripheral tracking
    private var peripherals: [UUID: CBPeripheral] = [:]
    private var peripheralDeviceIds: [UUID: String] = [:]
    private var connectedPeripherals: Set<UUID> = []

    // Data transfer
    private var reassemblers: [String: DataReassembler] = [:]
    private var chunker = DataChunker()
    private var mtuCache: [UUID: Int] = [:]
    private let defaultMtu: Int = 20

    // Configuration
    private var localDeviceId: String = ""
    private var localPublicKeyHash: String = ""

    // Central -> Device ID mapping (for peripheral mode)
    private var centralDeviceIds: [UUID: String] = [:]

    // MARK: - Initialization

    override init() {
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: bleQueue)
        peripheralManager = CBPeripheralManager(delegate: self, queue: bleQueue)
    }

    // MARK: - Configuration

    /// Configure the BLE manager with device information
    func configure(deviceId: String, publicKeyHash: String) {
        self.localDeviceId = deviceId
        self.localPublicKeyHash = publicKeyHash
        os_log("Configured with deviceId=%{public}@", log: bleLog, type: .info, deviceId)
    }

    // MARK: - Central Mode (Scanner)

    /// Start scanning for NearClip devices
    func startScanning() {
        guard centralManager.state == .poweredOn else {
            os_log("Cannot scan - Bluetooth not powered on", log: bleLog, type: .info)
            return
        }
        guard !isScanning else { return }

        os_log("Starting scan for NearClip devices", log: bleLog, type: .info)
        centralManager.scanForPeripherals(
            withServices: [BleUUID.service],
            options: [CBCentralManagerScanOptionAllowDuplicatesKey: false]
        )

        DispatchQueue.main.async {
            self.isScanning = true
        }
    }

    /// Stop scanning for devices
    func stopScanning() {
        guard isScanning else { return }

        os_log("Stopping scan", log: bleLog, type: .info)
        centralManager.stopScan()

        DispatchQueue.main.async {
            self.isScanning = false
        }
    }

    /// Connect to a peripheral by UUID
    func connect(peripheralUuid: String) {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            os_log("Peripheral not found: %{public}@", log: bleLog, type: .error, peripheralUuid)
            return
        }

        os_log("Connecting to peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)
        centralManager.connect(peripheral, options: nil)
    }

    /// Disconnect from a peripheral by UUID
    func disconnect(peripheralUuid: String) {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            return
        }

        os_log("Disconnecting from peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)
        centralManager.cancelPeripheralConnection(peripheral)
    }

    /// Check if connected to a peripheral
    func isConnected(peripheralUuid: String) -> Bool {
        guard let uuid = UUID(uuidString: peripheralUuid) else { return false }
        return connectedPeripherals.contains(uuid)
    }

    /// Get MTU for a peripheral
    func getMtu(peripheralUuid: String) -> UInt32 {
        guard let uuid = UUID(uuidString: peripheralUuid) else { return UInt32(defaultMtu) }
        return UInt32(mtuCache[uuid] ?? defaultMtu)
    }

    // MARK: - Peripheral Mode (Advertiser)

    /// Start advertising as a NearClip device
    func startAdvertising(serviceData: Data? = nil) {
        guard peripheralManager.state == .poweredOn else {
            os_log("Cannot advertise - Bluetooth not powered on", log: bleLog, type: .info)
            return
        }
        guard !isAdvertising else { return }
        guard !localDeviceId.isEmpty else {
            os_log("Cannot advertise - device ID not configured", log: bleLog, type: .error)
            return
        }

        if advertisedService == nil {
            setupGattService()
        }

        var advertisementData: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [BleUUID.service],
            CBAdvertisementDataLocalNameKey: "NearClip"
        ]

        // Add service data if provided
        if let serviceData = serviceData {
            advertisementData[CBAdvertisementDataServiceDataKey] = [BleUUID.service: serviceData]
        }

        os_log("Starting advertisement", log: bleLog, type: .info)
        peripheralManager.startAdvertising(advertisementData)

        DispatchQueue.main.async {
            self.isAdvertising = true
        }
    }

    /// Stop advertising
    func stopAdvertising() {
        guard isAdvertising else { return }

        os_log("Stopping advertisement", log: bleLog, type: .info)
        peripheralManager.stopAdvertising()

        DispatchQueue.main.async {
            self.isAdvertising = false
        }
    }

    // MARK: - Data Transfer

    /// Write data to a peripheral
    func writeData(peripheralUuid: String, data: Data) -> String {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            return "Peripheral not found: \(peripheralUuid)"
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }),
              let characteristic = service.characteristics?.first(where: { $0.uuid == BleUUID.dataTransfer }) else {
            return "Data transfer characteristic not found"
        }

        let mtu = mtuCache[uuid] ?? defaultMtu
        let chunks = chunker.createChunks(from: data, maxPayloadSize: mtu)

        os_log("Sending %d bytes in %d chunks to %{public}@", log: bleLog, type: .info, data.count, chunks.count, peripheralUuid)

        for chunk in chunks {
            peripheral.writeValue(chunk, for: characteristic, type: .withoutResponse)
        }

        return "" // Success
    }

    // MARK: - GATT Operations

    /// Read a characteristic value from a peripheral
    func readCharacteristic(peripheralUuid: String, charUuid: String) -> Data {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            os_log("Peripheral not found for read: %{public}@", log: bleLog, type: .error, peripheralUuid)
            return Data()
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            os_log("Service not found for read: %{public}@", log: bleLog, type: .error, peripheralUuid)
            return Data()
        }

        let cbuuid = CBUUID(string: charUuid)
        guard let characteristic = service.characteristics?.first(where: { $0.uuid == cbuuid }) else {
            os_log("Characteristic not found for read: %{public}@", log: bleLog, type: .error, charUuid)
            return Data()
        }

        // Return cached value if available
        return characteristic.value ?? Data()
    }

    /// Write to a characteristic on a peripheral
    func writeCharacteristic(peripheralUuid: String, charUuid: String, data: Data) -> String {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            return "Peripheral not found: \(peripheralUuid)"
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            return "Service not found for peripheral: \(peripheralUuid)"
        }

        let cbuuid = CBUUID(string: charUuid)
        guard let characteristic = service.characteristics?.first(where: { $0.uuid == cbuuid }) else {
            return "Characteristic not found: \(charUuid)"
        }

        // Determine write type based on characteristic properties
        let writeType: CBCharacteristicWriteType
        if characteristic.properties.contains(.write) {
            writeType = .withResponse
        } else if characteristic.properties.contains(.writeWithoutResponse) {
            writeType = .withoutResponse
        } else {
            return "Characteristic does not support writing"
        }

        peripheral.writeValue(data, for: characteristic, type: writeType)
        return "" // Success
    }

    /// Subscribe to notifications/indications from a characteristic
    func subscribeCharacteristic(peripheralUuid: String, charUuid: String) -> String {
        guard let uuid = UUID(uuidString: peripheralUuid),
              let peripheral = peripherals[uuid] else {
            return "Peripheral not found: \(peripheralUuid)"
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            return "Service not found for peripheral: \(peripheralUuid)"
        }

        let cbuuid = CBUUID(string: charUuid)
        guard let characteristic = service.characteristics?.first(where: { $0.uuid == cbuuid }) else {
            return "Characteristic not found: \(charUuid)"
        }

        // Check if characteristic supports notify or indicate
        if characteristic.properties.contains(.notify) || characteristic.properties.contains(.indicate) {
            peripheral.setNotifyValue(true, for: characteristic)
            return "" // Success
        } else {
            return "Characteristic does not support notifications"
        }
    }

    // MARK: - Private Methods

    private func setupGattService() {
        deviceIdCharacteristic = CBMutableCharacteristic(
            type: BleUUID.deviceId,
            properties: .read,
            value: localDeviceId.data(using: .utf8),
            permissions: .readable
        )

        publicKeyHashCharacteristic = CBMutableCharacteristic(
            type: BleUUID.publicKeyHash,
            properties: .read,
            value: localPublicKeyHash.data(using: .utf8),
            permissions: .readable
        )

        dataTransferCharacteristic = CBMutableCharacteristic(
            type: BleUUID.dataTransfer,
            properties: .writeWithoutResponse,
            value: nil,
            permissions: .writeable
        )

        dataAckCharacteristic = CBMutableCharacteristic(
            type: BleUUID.dataAck,
            properties: [.read, .notify],
            value: nil,
            permissions: .readable
        )

        advertisedService = CBMutableService(type: BleUUID.service, primary: true)
        advertisedService?.characteristics = [
            deviceIdCharacteristic!,
            publicKeyHashCharacteristic!,
            dataTransferCharacteristic!,
            dataAckCharacteristic!
        ]

        peripheralManager.add(advertisedService!)
        os_log("GATT service configured", log: bleLog, type: .info)
    }
}

// MARK: - CBCentralManagerDelegate

extension BleManager: CBCentralManagerDelegate {

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        DispatchQueue.main.async {
            self.centralState = central.state
        }
        os_log("Central state updated: %d", log: bleLog, type: .info, central.state.rawValue)
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {
        let peripheralUuid = peripheral.identifier.uuidString

        // Store peripheral reference
        peripherals[peripheral.identifier] = peripheral

        os_log("Discovered peripheral: %{public}@ RSSI: %d", log: bleLog, type: .info, peripheralUuid, RSSI.intValue)

        // Notify delegate - Rust layer will decide whether to connect
        delegate?.bleManager(self, didDiscoverDevice: peripheralUuid, deviceId: nil, publicKeyHash: nil, rssi: RSSI.intValue)
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        let peripheralUuid = peripheral.identifier.uuidString
        os_log("Connected to peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)

        connectedPeripherals.insert(peripheral.identifier)
        peripheral.delegate = self
        peripheral.discoverServices([BleUUID.service])

        // Update MTU
        let mtu = peripheral.maximumWriteValueLength(for: .withoutResponse)
        mtuCache[peripheral.identifier] = max(20, mtu - 3)
    }

    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        let peripheralUuid = peripheral.identifier.uuidString
        os_log("Failed to connect: %{public}@", log: bleLog, type: .error, error?.localizedDescription ?? "Unknown error")

        delegate?.bleManager(self, didFailWithError: error ?? NSError(domain: "BleManager", code: -1), forPeripheral: peripheralUuid)
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        let peripheralUuid = peripheral.identifier.uuidString
        let deviceId = peripheralDeviceIds[peripheral.identifier]

        os_log("Disconnected from peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)

        connectedPeripherals.remove(peripheral.identifier)
        peripheralDeviceIds.removeValue(forKey: peripheral.identifier)
        mtuCache.removeValue(forKey: peripheral.identifier)

        delegate?.bleManager(self, didDisconnectDevice: peripheralUuid, deviceId: deviceId)
    }
}

// MARK: - CBPeripheralDelegate

extension BleManager: CBPeripheralDelegate {

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error = error {
            os_log("Error discovering services: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            os_log("NearClip service not found", log: bleLog, type: .error)
            return
        }

        peripheral.discoverCharacteristics(
            [BleUUID.deviceId, BleUUID.publicKeyHash, BleUUID.dataTransfer, BleUUID.dataAck],
            for: service
        )
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error = error {
            os_log("Error discovering characteristics: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let characteristics = service.characteristics else { return }

        for characteristic in characteristics {
            if characteristic.uuid == BleUUID.deviceId || characteristic.uuid == BleUUID.publicKeyHash {
                peripheral.readValue(for: characteristic)
            }
            if characteristic.uuid == BleUUID.dataAck {
                peripheral.setNotifyValue(true, for: characteristic)
            }
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            os_log("Error reading characteristic: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let data = characteristic.value else { return }
        let peripheralUuid = peripheral.identifier.uuidString

        switch characteristic.uuid {
        case BleUUID.deviceId:
            if let deviceId = String(data: data, encoding: .utf8) {
                peripheralDeviceIds[peripheral.identifier] = deviceId
                os_log("Device ID read: %{public}@", log: bleLog, type: .info, deviceId)
                delegate?.bleManager(self, didConnectDevice: peripheralUuid, deviceId: deviceId)
            }

        case BleUUID.publicKeyHash:
            // Public key hash read - can be used for verification
            if let hash = String(data: data, encoding: .utf8) {
                os_log("Public key hash read: %{public}@", log: bleLog, type: .info, hash)
            }

        case BleUUID.dataAck:
            os_log("ACK received", log: bleLog, type: .info)

        default:
            break
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didModifyServices invalidatedServices: [CBService]) {
        let nearClipServiceInvalidated = invalidatedServices.contains { $0.uuid == BleUUID.service }
        if nearClipServiceInvalidated {
            peripheral.discoverServices([BleUUID.service])
        }
    }
}

// MARK: - CBPeripheralManagerDelegate

extension BleManager: CBPeripheralManagerDelegate {

    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        DispatchQueue.main.async {
            self.peripheralState = peripheral.state
        }
        os_log("Peripheral state updated: %d", log: bleLog, type: .info, peripheral.state.rawValue)
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error = error {
            os_log("Error adding service: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }
        os_log("Service added successfully", log: bleLog, type: .info)
    }

    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error = error {
            os_log("Error starting advertising: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            DispatchQueue.main.async {
                self.isAdvertising = false
            }
            return
        }
        os_log("Advertising started successfully", log: bleLog, type: .info)
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveRead request: CBATTRequest) {
        switch request.characteristic.uuid {
        case BleUUID.deviceId:
            if let data = localDeviceId.data(using: .utf8) {
                request.value = data
                peripheral.respond(to: request, withResult: .success)
            } else {
                peripheral.respond(to: request, withResult: .unlikelyError)
            }

        case BleUUID.publicKeyHash:
            if let data = localPublicKeyHash.data(using: .utf8) {
                request.value = data
                peripheral.respond(to: request, withResult: .success)
            } else {
                peripheral.respond(to: request, withResult: .unlikelyError)
            }

        default:
            peripheral.respond(to: request, withResult: .attributeNotFound)
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveWrite requests: [CBATTRequest]) {
        for request in requests {
            if request.characteristic.uuid == BleUUID.dataTransfer,
               let data = request.value {
                handleIncomingData(data, from: request.central)
            }
        }
    }

    private func handleIncomingData(_ data: Data, from central: CBCentral) {
        let centralId = central.identifier.uuidString

        guard let (messageId, sequence, total, payload) = DataChunker.parseChunk(data) else {
            os_log("Invalid chunk received", log: bleLog, type: .error)
            return
        }

        if reassemblers[centralId] == nil {
            reassemblers[centralId] = DataReassembler()
        }

        guard let reassembler = reassemblers[centralId] else { return }

        if reassembler.isTimedOut {
            reassembler.reset()
        }

        if let completeData = reassembler.addChunk(payload, sequence: Int(sequence), total: Int(total), messageId: messageId) {
            os_log("Complete message received: %d bytes", log: bleLog, type: .info, completeData.count)
            delegate?.bleManager(self, didReceiveData: completeData, fromPeripheral: centralId)
            sendAck(to: central, messageId: messageId)
        }
    }

    private func sendAck(to central: CBCentral, messageId: UInt32) {
        guard let characteristic = dataAckCharacteristic else { return }

        var ackData = Data()
        var msgId = messageId.littleEndian
        ackData.append(Data(bytes: &msgId, count: 4))

        peripheralManager.updateValue(ackData, for: characteristic, onSubscribedCentrals: [central])
    }
}

// MARK: - Data Reassembler

private let kChunkHeaderSize = 8

class DataReassembler {
    private var chunks: [Int: Data] = [:]
    private var totalChunks: Int = 0
    private var messageId: UInt32 = 0
    private var lastActivityTime: Date = Date()
    private let timeout: TimeInterval = 30.0

    var isTimedOut: Bool {
        return Date().timeIntervalSince(lastActivityTime) > timeout
    }

    func reset() {
        chunks.removeAll()
        totalChunks = 0
        messageId = 0
    }

    func addChunk(_ data: Data, sequence: Int, total: Int, messageId: UInt32) -> Data? {
        lastActivityTime = Date()

        if self.messageId != messageId {
            self.chunks.removeAll()
            self.messageId = messageId
            self.totalChunks = total
        }

        chunks[sequence] = data

        if chunks.count == totalChunks {
            var completeData = Data()
            for i in 0..<totalChunks {
                if let chunk = chunks[i] {
                    completeData.append(chunk)
                }
            }
            chunks.removeAll()
            return completeData
        }

        return nil
    }
}

// MARK: - Data Chunker

class DataChunker {
    private var messageIdCounter: UInt32 = 0

    func createChunks(from data: Data, maxPayloadSize: Int) -> [Data] {
        let payloadSize = max(1, maxPayloadSize - kChunkHeaderSize)
        var chunks: [Data] = []

        let totalChunks = (data.count + payloadSize - 1) / payloadSize
        messageIdCounter = messageIdCounter &+ 1
        let messageId = messageIdCounter

        var offset = 0
        var sequence: UInt16 = 0

        while offset < data.count {
            let chunkPayloadSize = min(payloadSize, data.count - offset)
            let payload = data.subdata(in: offset..<(offset + chunkPayloadSize))

            var chunk = Data()
            var msgId = messageId.littleEndian
            chunk.append(Data(bytes: &msgId, count: 4))
            var seq = sequence.littleEndian
            chunk.append(Data(bytes: &seq, count: 2))
            var total = UInt16(totalChunks).littleEndian
            chunk.append(Data(bytes: &total, count: 2))
            chunk.append(payload)

            chunks.append(chunk)
            offset += chunkPayloadSize
            sequence += 1
        }

        if chunks.isEmpty {
            var chunk = Data()
            var msgId = messageId.littleEndian
            chunk.append(Data(bytes: &msgId, count: 4))
            var seq: UInt16 = 0
            chunk.append(Data(bytes: &seq, count: 2))
            var total: UInt16 = 1
            chunk.append(Data(bytes: &total, count: 2))
            chunks.append(chunk)
        }

        return chunks
    }

    static func parseChunk(_ data: Data) -> (messageId: UInt32, sequence: UInt16, total: UInt16, payload: Data)? {
        guard data.count >= kChunkHeaderSize else { return nil }

        let messageId = data.subdata(in: 0..<4).withUnsafeBytes { $0.load(as: UInt32.self).littleEndian }
        let sequence = data.subdata(in: 4..<6).withUnsafeBytes { $0.load(as: UInt16.self).littleEndian }
        let total = data.subdata(in: 6..<8).withUnsafeBytes { $0.load(as: UInt16.self).littleEndian }
        let payload = data.subdata(in: kChunkHeaderSize..<data.count)

        return (messageId, sequence, total, payload)
    }
}
