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
    func bleManager(_ manager: BleManager, didReceiveAck data: Data, fromPeripheral peripheralUuid: String, deviceId: String)
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

    private var characteristicReadSemaphores: [String: DispatchSemaphore] = [:]
    private var characteristicReadResults: [String: Data] = [:]

    // Central mode (Peripheral server) tracking
    private var connectedCentrals: [UUID: CBCentral] = [:]

    // MTU cache
    private var mtuCache: [UUID: Int] = [:]
    private let defaultMtu: Int = 20

    // Configuration
    private var localDeviceId: String = ""
    private var localPublicKeyHash: String = ""

    // Central -> Device ID mapping (for peripheral mode)
    private var centralDeviceIds: [UUID: String] = [:]

    // Logging throttle
    private var lastDiscoveryLog: [String: Date] = [:]
    // Callback throttle
    private var lastDiscoveryCallback: [String: Date] = [:]

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
        NSLog("BLE: Configured with deviceId=%@, publicKeyHash length=%d", deviceId, publicKeyHash.count)

        // If we're powered on, we can setup GATT now
        if peripheralManager.state == .poweredOn {
            if advertisedService == nil {
                 NSLog("BLE: configure called and powered on, setting up GATT service")
                 setupGattService()
            }
        }
    }

    // MARK: - Central Mode (Scanner)

    /// Start scanning for NearClip devices
    func startScanning() {
        guard centralManager.state == .poweredOn else {
            os_log("Cannot scan - Bluetooth not powered on", log: bleLog, type: .info)
            NSLog("BLE: Cannot scan - Bluetooth not powered on (state: %d)", centralManager.state.rawValue)
            return
        }
        guard !isScanning else {
            NSLog("BLE: Already scanning, skipping startScanning")
            return
        }

        os_log("Starting scan for NearClip devices", log: bleLog, type: .info)
        NSLog("BLE: Starting scan (filtering in callback for cross-platform compatibility)")
        // Scan without service filter for cross-platform compatibility with Android
        // Filter by service UUID in didDiscover callback instead
        centralManager.scanForPeripherals(
            withServices: nil,
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

    /// Check if connected to a peripheral or central
    func isConnected(peripheralUuid: String) -> Bool {
        guard let uuid = UUID(uuidString: peripheralUuid) else { return false }
        // Check both Central mode (we connected to a peripheral) and Peripheral mode (central connected to us)
        return connectedPeripherals.contains(uuid) || connectedCentrals[uuid] != nil
    }

    /// Get MTU for a peripheral
    func getMtu(peripheralUuid: String) -> UInt32 {
        var targetUuid: UUID?

        // Try to find the CoreBluetooth UUID from the Device ID first (check if input is a Device ID)
        if let mappedKey = peripheralDeviceIds.first(where: { $0.value == peripheralUuid })?.key {
            targetUuid = mappedKey
        } else {
            // Not a mapped Device ID, try to parse as UUID directly (might be raw Peripheral UUID)
            targetUuid = UUID(uuidString: peripheralUuid)
        }

        guard let uuid = targetUuid else { return UInt32(defaultMtu) }
        return UInt32(mtuCache[uuid] ?? defaultMtu)
    }

    /// Get the Device ID for a peripheral UUID if known
    func getDeviceId(for peripheralUuid: String) -> String? {
        guard let uuid = UUID(uuidString: peripheralUuid) else { return nil }
        return peripheralDeviceIds[uuid]
    }

    /// Get the Peripheral UUID for a device ID if known
    /// This is the reverse lookup of getDeviceId
    func getPeripheralUuid(for deviceId: String) -> String? {
        for (uuid, id) in peripheralDeviceIds {
            if id == deviceId {
                return uuid.uuidString
            }
        }
        return nil
    }

    /// Check if a device is connected by device ID
    /// This allows checking connection status using either peripheral UUID or device ID
    func isConnectedByDeviceId(_ deviceId: String) -> Bool {
        // Try to find the peripheral UUID for this device ID (Central mode)
        if let peripheralUuid = getPeripheralUuid(for: deviceId) {
            return isConnected(peripheralUuid: peripheralUuid)
        }
        // Try to find the central UUID for this device ID (Peripheral mode)
        for (centralUuid, id) in centralDeviceIds {
            if id == deviceId {
                return connectedCentrals[centralUuid] != nil
            }
        }
        // Fall back to trying it as a peripheral UUID directly
        return isConnected(peripheralUuid: deviceId)
    }

    // MARK: - Peripheral Mode (Advertiser)

    /// Start advertising as a NearClip device
    // Store pending advertisement data to start after service is added
    private var pendingAdvertisementData: [String: Any]?
    // Track whether GATT service has been successfully added
    private var serviceAddedSuccessfully = false

    func startAdvertising(serviceData: Data? = nil) {
        NSLog("BLE: startAdvertising called, peripheral state: %d, isAdvertising: %d, deviceId: %@, serviceAdded: %d",
              peripheralManager.state.rawValue, isAdvertising ? 1 : 0, localDeviceId.isEmpty ? "empty" : "set", serviceAddedSuccessfully ? 1 : 0)

        guard peripheralManager.state == .poweredOn else {
            os_log("Cannot advertise - Bluetooth not powered on", log: bleLog, type: .info)
            NSLog("BLE: Cannot advertise - Bluetooth not powered on (state: %d)", peripheralManager.state.rawValue)
            return
        }
        guard !isAdvertising else {
            print("BLE: Already advertising, skipping")
            return
        }
        guard !localDeviceId.isEmpty else {
            os_log("Cannot advertise - device ID not configured", log: bleLog, type: .error)
            print("BLE: Cannot advertise - device ID not configured")
            return
        }

        // Setup GATT service first if not already added
        if advertisedService == nil {
            NSLog("BLE: Setting up GATT service before advertising")
            setupGattService()
        }

        // Prepare advertisement data
        var advertisementData: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [BleUUID.service],
            CBAdvertisementDataLocalNameKey: "NearClip"
        ]

        // Add service data if provided
        if let serviceData = serviceData {
            advertisementData[CBAdvertisementDataServiceDataKey] = [BleUUID.service: serviceData]
        }

        // Store for later use when service is added
        pendingAdvertisementData = advertisementData

        // Check if service was already added successfully
        if serviceAddedSuccessfully {
            os_log("Service already added, starting advertisement", log: bleLog, type: .info)
            NSLog("BLE: Service already added, starting advertisement immediately")
            startAdvertisingNow(advertisementData)
        } else {
            os_log("Waiting for service to be added before advertising", log: bleLog, type: .info)
            NSLog("BLE: Waiting for service to be added before advertising")
        }
    }

    private func startAdvertisingNow(_ advertisementData: [String: Any]) {
        os_log("Starting advertisement with data: %{public}@", log: bleLog, type: .info, advertisementData.description)
        NSLog("BLE: Starting advertisement with UUID: %@", BleUUID.service.uuidString)
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

    /// Write data to a peripheral or connected central
    /// Data should already be chunked by Rust layer if needed
    func writeData(peripheralUuid: String, data: Data) -> String {
        var targetUuid: UUID?

        // Try to find the CoreBluetooth UUID from the Device ID first (check if input is a Device ID)
        if let mappedKey = peripheralDeviceIds.first(where: { $0.value == peripheralUuid })?.key {
            targetUuid = mappedKey
        } else {
            // Not a mapped Device ID, try to parse as UUID directly (might be raw Peripheral UUID)
            targetUuid = UUID(uuidString: peripheralUuid)
        }

        guard let uuid = targetUuid else {
             return "Invalid UUID or Device ID not found: \(peripheralUuid)"
        }

        // Case 1: Writing to a connected Peripheral (Client role)
        if let peripheral = peripherals[uuid] {
            guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }),
                  let characteristic = service.characteristics?.first(where: { $0.uuid == BleUUID.dataTransfer }) else {
                return "Data transfer characteristic not found"
            }

            os_log("Sending %d bytes to peripheral %{public}@", log: bleLog, type: .info, data.count, peripheralUuid)
            peripheral.writeValue(data, for: characteristic, type: .withResponse)

            return "" // Success
        }

        // Case 2: Sending to a connected Central (Server role)
        if let central = connectedCentrals[uuid] {
             guard let characteristic = dataTransferCharacteristic else {
                 return "Data transfer characteristic not configured"
             }

             os_log("Sending %d bytes to central %{public}@", log: bleLog, type: .info, data.count, peripheralUuid)

             let sent = peripheralManager.updateValue(data, for: characteristic, onSubscribedCentrals: [central])
             if !sent {
                 os_log("Failed to send data to central %{public}@ - queue full", log: bleLog, type: .error, peripheralUuid)
                 return "Failed to send data - transmission queue full"
             }

             return "" // Success
        }

        return "Device not found: \(peripheralUuid)"
    }

    // MARK: - GATT Operations

    /// Read a characteristic value from a peripheral
    func readCharacteristic(peripheralUuid: String, charUuid: String) -> Data {
        let isMainThread = Thread.isMainThread
        NSLog("BLE: readCharacteristic called for %@ on thread: %@ (isMain: %@)", charUuid, Thread.current.description, isMainThread ? "YES" : "NO")

        if isMainThread {
            NSLog("BLE: ⚠️ WARNING: readCharacteristic blocking Main Thread!")
        }

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

        let key = "\(peripheralUuid)-\(charUuid)"
        let semaphore = DispatchSemaphore(value: 0)

        // Store semaphore
        characteristicReadSemaphores[key] = semaphore
        characteristicReadResults[key] = nil // Clear previous result

        os_log("Reading characteristic %{public}@ for peripheral %{public}@", log: bleLog, type: .info, charUuid, peripheralUuid)
        peripheral.readValue(for: characteristic) // Trigger asynchronous read

        // Wait for the delegate to signal that the value has been read
        let timeout = DispatchTime.now() + .seconds(5) // 5 seconds timeout
        if semaphore.wait(timeout: timeout) == .timedOut {
            os_log("Read characteristic timeout for %{public}@ on %{public}@", log: bleLog, type: .error, charUuid, peripheralUuid)
            characteristicReadSemaphores.removeValue(forKey: key)
            return Data()
        }

        // Retrieve result
        characteristicReadSemaphores.removeValue(forKey: key)
        return characteristicReadResults.removeValue(forKey: key) ?? Data()
    }

    /// Write to a characteristic on a peripheral
    func writeCharacteristic(peripheralUuid: String, charUuid: String, data: Data) -> String {
        var targetUuid: UUID?

        // Try to find the CoreBluetooth UUID from the Device ID first (check if input is a Device ID)
        if let mappedKey = peripheralDeviceIds.first(where: { $0.value == peripheralUuid })?.key {
            targetUuid = mappedKey
        } else {
            // Not a mapped Device ID, try to parse as UUID directly (might be raw Peripheral UUID)
            targetUuid = UUID(uuidString: peripheralUuid)
        }

        guard let uuid = targetUuid,
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
        var targetUuid: UUID?

        // Try to find the CoreBluetooth UUID from the Device ID first (check if input is a Device ID)
        if let mappedKey = peripheralDeviceIds.first(where: { $0.value == peripheralUuid })?.key {
            targetUuid = mappedKey
        } else {
            // Not a mapped Device ID, try to parse as UUID directly (might be raw Peripheral UUID)
            targetUuid = UUID(uuidString: peripheralUuid)
        }

        guard let uuid = targetUuid,
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
            properties: [.writeWithoutResponse, .notify],
            value: nil,
            permissions: [.writeable, .readable]
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
        NSLog("BLE: Central state updated to %d", central.state.rawValue)

        // Auto-start scanning when Bluetooth becomes available
        if central.state == .poweredOn && !isScanning {
            NSLog("BLE: Auto-starting scanning after poweredOn")
            startScanning()
        }
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {
        let peripheralUuid = peripheral.identifier.uuidString

        // Get service UUIDs from advertisement data
        let serviceUUIDs = advertisementData[CBAdvertisementDataServiceUUIDsKey] as? [CBUUID] ?? []

        // Check if this is a NearClip device
        let isNearClip = serviceUUIDs.contains(BleUUID.service)
        let name = peripheral.name ?? advertisementData[CBAdvertisementDataLocalNameKey] as? String ?? "Unknown"
        let hasNameMatch = name.localizedCaseInsensitiveContains("NearClip")

        guard isNearClip || hasNameMatch else {
            // Ignore non-NearClip devices
            return
        }

        // Throttle discovery logging (max once per 5 seconds per device)
        let now = Date()
        let lastLog = lastDiscoveryLog[peripheralUuid]
        let shouldLog = lastLog == nil || now.timeIntervalSince(lastLog!) > 5.0

        if shouldLog {
            NSLog("BLE: Discovered NearClip device: %@ (name: %@), RSSI: %d", peripheralUuid, name, RSSI.intValue)
            lastDiscoveryLog[peripheralUuid] = now
        }

        // Store peripheral reference
        peripherals[peripheral.identifier] = peripheral

        if shouldLog {
            os_log("Discovered NearClip peripheral: %{public}@ RSSI: %d", log: bleLog, type: .info, peripheralUuid, RSSI.intValue)
        }

        // Check if we already know this device's ID
        if let knownDeviceId = peripheralDeviceIds[peripheral.identifier] {
            // Already know this device, notify with known info
            // Throttle duplicate delegate calls (max once per second)
            let now = Date()
            if let lastCallback = lastDiscoveryCallback[peripheralUuid], now.timeIntervalSince(lastCallback) < 1.0 {
                return
            }
            lastDiscoveryCallback[peripheralUuid] = now

            if shouldLog {
                NSLog("BLE: Device %@ already known with deviceId: %@", peripheralUuid, knownDeviceId)
            }
            delegate?.bleManager(self, didDiscoverDevice: peripheralUuid, deviceId: knownDeviceId, publicKeyHash: nil, rssi: RSSI.intValue)
            return
        }

        // Notify delegate about discovery with unknown device ID
        // Rust layer will decide whether to connect for discovery
        delegate?.bleManager(self, didDiscoverDevice: peripheralUuid, deviceId: nil, publicKeyHash: nil, rssi: RSSI.intValue)
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        let peripheralUuid = peripheral.identifier.uuidString

        os_log("Connected to peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)
        NSLog("BLE: Connected to %@", peripheralUuid)

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
        NSLog("BLE: Failed to connect to %@: %@", peripheralUuid, error?.localizedDescription ?? "Unknown error")

        delegate?.bleManager(self, didFailWithError: error ?? NSError(domain: "BleManager", code: -1), forPeripheral: peripheralUuid)
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        let peripheralUuid = peripheral.identifier.uuidString
        let deviceId = peripheralDeviceIds[peripheral.identifier]

        os_log("Disconnected from peripheral: %{public}@", log: bleLog, type: .info, peripheralUuid)
        NSLog("BLE: Disconnected from %@", peripheralUuid)

        connectedPeripherals.remove(peripheral.identifier)
        peripheralDeviceIds.removeValue(forKey: peripheral.identifier)
        mtuCache.removeValue(forKey: peripheral.identifier)

        delegate?.bleManager(self, didDisconnectDevice: peripheralUuid, deviceId: deviceId)
    }
}

// MARK: - CBPeripheralDelegate

extension BleManager: CBPeripheralDelegate {

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        NSLog("BLE: didDiscoverServices called for %@", peripheral.identifier.uuidString)

        if let error = error {
            os_log("Error discovering services: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            NSLog("BLE: Error discovering services: %@", error.localizedDescription)
            return
        }

        NSLog("BLE: Services count: %d", peripheral.services?.count ?? 0)
        if let services = peripheral.services {
            for svc in services {
                NSLog("BLE: Found service: %@", svc.uuid.uuidString)
            }
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            os_log("NearClip service not found", log: bleLog, type: .error)
            NSLog("BLE: NearClip service not found on %@", peripheral.identifier.uuidString)
            return
        }

        NSLog("BLE: Discovering characteristics for service %@", service.uuid.uuidString)
        peripheral.discoverCharacteristics(
            [BleUUID.deviceId, BleUUID.publicKeyHash, BleUUID.dataTransfer, BleUUID.dataAck],
            for: service
        )
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        NSLog("BLE: didDiscoverCharacteristicsFor called for %@, service %@", peripheral.identifier.uuidString, service.uuid.uuidString)

        if let error = error {
            os_log("Error discovering characteristics: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            NSLog("BLE: Error discovering characteristics: %@", error.localizedDescription)
            return
        }

        guard let characteristics = service.characteristics else {
            NSLog("BLE: No characteristics found for service %@", service.uuid.uuidString)
            return
        }

        NSLog("BLE: Found %d characteristics", characteristics.count)
        for char in characteristics {
            NSLog("BLE: Characteristic: %@", char.uuid.uuidString)
        }

        for characteristic in characteristics {
            if characteristic.uuid == BleUUID.deviceId || characteristic.uuid == BleUUID.publicKeyHash {
                NSLog("BLE: Reading characteristic %@", characteristic.uuid.uuidString)
                peripheral.readValue(for: characteristic)
            }
            if characteristic.uuid == BleUUID.dataAck || characteristic.uuid == BleUUID.dataTransfer {
                peripheral.setNotifyValue(true, for: characteristic)
            }
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        let peripheralUuid = peripheral.identifier.uuidString
        let key = "\(peripheralUuid)-\(characteristic.uuid.uuidString)"

        if let error = error {
            os_log("Error reading characteristic %{public}@ for peripheral %{public}@: %{public}@", log: bleLog, type: .error, characteristic.uuid.uuidString, peripheralUuid, error.localizedDescription)
            if let semaphore = characteristicReadSemaphores[key] {
                semaphore.signal() // Release semaphore even on error
            }
            return
        }

        guard let data = characteristic.value else {
            os_log("Characteristic value is nil for %{public}@ on peripheral %{public}@", log: bleLog, type: .error, characteristic.uuid.uuidString, peripheralUuid)
            if let semaphore = characteristicReadSemaphores[key] {
                semaphore.signal() // Release semaphore even if data is nil
            }
            return
        }

        // Store result for synchronous read
        characteristicReadResults[key] = data
        if let semaphore = characteristicReadSemaphores[key] {
            semaphore.signal()
        }

        switch characteristic.uuid {
        case BleUUID.deviceId:
            if let deviceId = String(data: data, encoding: .utf8) {
                peripheralDeviceIds[peripheral.identifier] = deviceId
                os_log("Device ID read: %{public}@", log: bleLog, type: .info, deviceId)
                NSLog("BLE: Device ID read: %@ from %@", deviceId, peripheralUuid)

                // Notify delegate about connection with device ID
                delegate?.bleManager(self, didConnectDevice: peripheralUuid, deviceId: deviceId)
            }

        case BleUUID.publicKeyHash:
            // Public key hash read - can be used for verification
            if let hash = String(data: data, encoding: .utf8) {
                os_log("Public key hash read: %{public}@", log: bleLog, type: .info, hash)
            }

        case BleUUID.dataAck:
            // ACK received from peripheral - forward to delegate for FFI processing
            if let deviceId = peripheralDeviceIds[peripheral.identifier] {
                os_log("ACK received from %{public}@, data: %{public}d bytes", log: bleLog, type: .info, deviceId, data.count)
                delegate?.bleManager(self, didReceiveAck: data, fromPeripheral: peripheralUuid, deviceId: deviceId)
            } else {
                os_log("ACK received but no device ID mapping for %{public}@", log: bleLog, type: .error, peripheralUuid)
            }

        case BleUUID.dataTransfer:
            // Forward raw data to delegate - Rust layer handles reassembly
            os_log("Data chunk received from peripheral: %d bytes", log: bleLog, type: .debug, data.count)
            delegate?.bleManager(self, didReceiveData: data, fromPeripheral: peripheralUuid)

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
        NSLog("BLE: Peripheral state updated to %d", peripheral.state.rawValue)

        // Auto-start advertising when Bluetooth becomes available
        if peripheral.state == .poweredOn && !localDeviceId.isEmpty && !isAdvertising {
            NSLog("BLE: Auto-starting advertising after poweredOn")
            startAdvertising()
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error = error {
            os_log("Error adding service: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            NSLog("BLE: Error adding service: %@", error.localizedDescription)
            serviceAddedSuccessfully = false
            return
        }
        os_log("Service added successfully", log: bleLog, type: .info)
        NSLog("BLE: Service added successfully, checking if advertising is pending")
        serviceAddedSuccessfully = true

        // If advertising was waiting for service to be added, start it now
        if let pendingAdData = pendingAdvertisementData {
            NSLog("BLE: Starting pending advertisement now that service is added")
            startAdvertisingNow(pendingAdData)
            pendingAdvertisementData = nil
        }
    }

    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error = error {
            os_log("Error starting advertising: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            NSLog("BLE: Error starting advertising: %@", error.localizedDescription)
            DispatchQueue.main.async {
                self.isAdvertising = false
            }
            return
        }
        os_log("Advertising started successfully", log: bleLog, type: .info)
        NSLog("BLE: Advertising started successfully with UUID: %@", BleUUID.service.uuidString)
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
            let centralId = request.central.identifier.uuidString
            if let data = request.value {
                if request.characteristic.uuid == BleUUID.dataTransfer {
                    os_log("Data chunk received from central: %d bytes", log: bleLog, type: .debug, data.count)
                    delegate?.bleManager(self, didReceiveData: data, fromPeripheral: centralId)
                } else if request.characteristic.uuid == BleUUID.dataAck {
                    os_log("ACK received from central: %d bytes", log: bleLog, type: .debug, data.count)
                    // Try to get device ID for better tracking, fallback to central ID
                    let deviceId = peripheralDeviceIds[request.central.identifier] ?? centralId
                    delegate?.bleManager(self, didReceiveAck: data, fromPeripheral: centralId, deviceId: deviceId)
                }
            }
            // Always respond to write requests to complete the GATT transaction
            peripheral.respond(to: request, withResult: .success)
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didSubscribeTo characteristic: CBCharacteristic) {
        let centralId = central.identifier.uuidString
        os_log("Central subscribed: %{public}@", log: bleLog, type: .info, centralId)

        // Store the central connection
        connectedCentrals[central.identifier] = central

        // Notify delegate about the connection
        // We don't have a device ID yet - it will be discovered after reading characteristics
        // Use empty string as placeholder
        delegate?.bleManager(self, didConnectDevice: centralId, deviceId: "")
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didUnsubscribeFrom characteristic: CBCharacteristic) {
        let centralId = central.identifier.uuidString
        os_log("Central unsubscribed: %{public}@", log: bleLog, type: .info, centralId)

        // Remove from connected centrals
        connectedCentrals.removeValue(forKey: central.identifier)

        // Notify delegate about disconnection
        delegate?.bleManager(self, didDisconnectDevice: centralId, deviceId: nil)
    }

    private func sendAck(to central: CBCentral, messageId: UInt16) {
        guard let characteristic = dataAckCharacteristic else { return }

        var ackData = Data()
        var msgId = messageId.littleEndian
        ackData.append(Data(bytes: &msgId, count: 2))

        peripheralManager.updateValue(ackData, for: characteristic, onSubscribedCentrals: [central])
    }
}

