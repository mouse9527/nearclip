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
    var publicKeyHash: String?
    var rssi: Int
    var lastSeen: Date

    static func == (lhs: BleDevice, rhs: BleDevice) -> Bool {
        lhs.id == rhs.id
    }
}

// MARK: - BLE Connection State

enum BleConnectionState: Equatable {
    case disconnected
    case connecting
    case connected
    case ready  // Services discovered and ready for data transfer
}

// MARK: - BLE Manager Delegate

protocol BleManagerDelegate: AnyObject {
    func bleManager(_ manager: BleManager, didDiscoverDevice device: BleDevice)
    func bleManager(_ manager: BleManager, didLoseDevice deviceId: String)
    func bleManager(_ manager: BleManager, didConnectDevice deviceId: String)
    func bleManager(_ manager: BleManager, didDisconnectDevice deviceId: String)
    func bleManager(_ manager: BleManager, didReceiveData data: Data, fromDevice deviceId: String)
    func bleManager(_ manager: BleManager, didFailWithError error: Error, forDevice deviceId: String?)
}

// MARK: - BLE Manager

/// Manages Bluetooth Low Energy communication for NearClip
/// Supports both Central (scanner) and Peripheral (advertiser) modes
final class BleManager: NSObject, ObservableObject {

    // MARK: - Properties

    weak var delegate: BleManagerDelegate?

    @Published private(set) var centralState: CBManagerState = .unknown
    @Published private(set) var peripheralState: CBManagerState = .unknown
    @Published private(set) var isScanning = false
    @Published private(set) var isAdvertising = false

    // Flags to track desired state (for deferred start when Bluetooth becomes ready)
    private var wantsToScan = false
    private var wantsToAdvertise = false
    @Published private(set) var discoveredDevices: [String: BleDevice] = [:]
    @Published private(set) var connectedDevices: [String: BleDevice] = [:]

    private var centralManager: CBCentralManager!
    private var peripheralManager: CBPeripheralManager!

    // Peripheral mode properties
    private var advertisedService: CBMutableService?
    private var deviceIdCharacteristic: CBMutableCharacteristic?
    private var publicKeyHashCharacteristic: CBMutableCharacteristic?
    private var dataTransferCharacteristic: CBMutableCharacteristic?
    private var dataAckCharacteristic: CBMutableCharacteristic?

    // Central mode properties
    private var pendingPeripherals: [UUID: CBPeripheral] = [:]
    private var peripheralDeviceIds: [UUID: String] = [:]
    private var reconnectPeripherals: [UUID: CBPeripheral] = [:]  // Devices to auto-reconnect

    // Data transfer
    private var reassemblers: [String: DataReassembler] = [:]
    private var chunker = DataChunker()
    private var mtu: Int = 20  // Default BLE MTU payload size

    // Write queue for flow control
    private var writeQueue: [String: [Data]] = [:]
    private var isWriting: [String: Bool] = [:]
    private let writeDelayMs: UInt64 = 5_000_000  // 5ms in nanoseconds

    // Central -> Device ID mapping (for peripheral mode)
    private var centralDeviceIds: [UUID: String] = [:]

    // Configuration
    private var localDeviceId: String = ""
    private var localPublicKeyHash: String = ""
    private var autoReconnect: Bool = true

    // Reconnection settings
    private var reconnectAttempts: [UUID: Int] = [:]
    private let maxReconnectAttempts = 5
    private let baseReconnectDelay: TimeInterval = 1.0
    private let maxReconnectDelay: TimeInterval = 30.0

    // Connection health monitoring
    private var connectionHealthTimer: Timer?
    private var lastActivityTimes: [String: Date] = [:]
    private let connectionHealthInterval: TimeInterval = 30.0
    private let connectionTimeout: TimeInterval = 60.0

    // Power optimization
    private var scanPauseTimer: Timer?
    private let scanPauseInterval: TimeInterval = 60.0  // Pause scanning after 60s if connected
    private var shouldPauseScanWhenConnected: Bool = true

    // MARK: - Initialization

    override init() {
        super.init()

        // Initialize managers on a background queue
        let bleQueue = DispatchQueue(label: "com.nearclip.ble", qos: .userInitiated)
        centralManager = CBCentralManager(delegate: self, queue: bleQueue)
        peripheralManager = CBPeripheralManager(delegate: self, queue: bleQueue)
    }

    // MARK: - Configuration

    /// Configure the BLE manager with device information
    func configure(deviceId: String, publicKeyHash: String) {
        self.localDeviceId = deviceId
        self.localPublicKeyHash = publicKeyHash
        NSLog("BLE: Configured with deviceId=\(deviceId)")
        os_log("Configured with deviceId=%{public}@", log: bleLog, type: .info, deviceId)
    }

    // MARK: - Central Mode (Scanner)

    /// Start scanning for NearClip devices
    func startScanning() {
        wantsToScan = true

        // Use centralManager.state directly instead of the async-updated centralState
        guard centralManager.state == .poweredOn else {
            NSLog("BLE: Cannot scan yet - Bluetooth not powered on (state: \(centralManager.state.rawValue)), will start when ready")
            os_log("Cannot scan yet - Bluetooth not powered on (state: %d), will start when ready", log: bleLog, type: .info, centralManager.state.rawValue)
            return
        }

        guard !isScanning else { return }

        NSLog("BLE: Starting scan for NearClip devices (service: \(BleUUID.service.uuidString))")
        os_log("Starting scan for NearClip devices", log: bleLog, type: .info)
        centralManager.scanForPeripherals(
            withServices: [BleUUID.service],
            options: [
                CBCentralManagerScanOptionAllowDuplicatesKey: false
            ]
        )

        isScanning = true
    }

    /// Stop scanning for devices
    func stopScanning() {
        wantsToScan = false
        guard isScanning else { return }

        os_log("Stopping scan", log: bleLog, type: .info)
        centralManager.stopScan()

        DispatchQueue.main.async {
            self.isScanning = false
        }
    }

    /// Connect to a discovered device
    func connect(deviceId: String) {
        guard let device = discoveredDevices[deviceId] else {
            os_log("Device not found: %{public}@", log: bleLog, type: .error, deviceId)
            return
        }

        os_log("Connecting to device: %{public}@", log: bleLog, type: .info, deviceId)
        centralManager.connect(device.peripheral, options: nil)
    }

    /// Disconnect from a device
    func disconnect(deviceId: String) {
        if let device = connectedDevices[deviceId] {
            os_log("Disconnecting from device: %{public}@", log: bleLog, type: .info, deviceId)
            centralManager.cancelPeripheralConnection(device.peripheral)
        }
    }

    // MARK: - Peripheral Mode (Advertiser)

    /// Start advertising as a NearClip device
    func startAdvertising() {
        wantsToAdvertise = true

        // Use peripheralManager.state directly instead of the async-updated peripheralState
        guard peripheralManager.state == .poweredOn else {
            NSLog("BLE: Cannot advertise yet - Bluetooth not powered on (state: \(peripheralManager.state.rawValue)), will start when ready")
            os_log("Cannot advertise yet - Bluetooth not powered on (state: %d), will start when ready", log: bleLog, type: .info, peripheralManager.state.rawValue)
            return
        }

        guard !isAdvertising else { return }
        guard !localDeviceId.isEmpty else {
            NSLog("BLE: Cannot advertise - device ID not configured")
            os_log("Cannot advertise - device ID not configured", log: bleLog, type: .error)
            return
        }

        // Create service and characteristics if not already created
        if advertisedService == nil {
            setupGattService()
        }

        // Start advertising
        let advertisementData: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [BleUUID.service],
            CBAdvertisementDataLocalNameKey: "NearClip"
        ]

        os_log("Starting advertisement", log: bleLog, type: .info)
        peripheralManager.startAdvertising(advertisementData)

        DispatchQueue.main.async {
            self.isAdvertising = true
        }
    }

    /// Stop advertising
    func stopAdvertising() {
        wantsToAdvertise = false
        guard isAdvertising else { return }

        os_log("Stopping advertisement", log: bleLog, type: .info)
        peripheralManager.stopAdvertising()

        DispatchQueue.main.async {
            self.isAdvertising = false
        }
    }

    // MARK: - Data Transfer

    /// Send data to a connected device (Central mode - we are the central, sending to peripheral)
    func sendData(_ data: Data, to deviceId: String) {
        guard let device = connectedDevices[deviceId] else {
            NSLog("BLE: Cannot send - device not connected: \(deviceId)")
            os_log("Cannot send - device not connected: %{public}@", log: bleLog, type: .error, deviceId)
            return
        }

        // Find the data transfer characteristic
        guard let service = device.peripheral.services?.first(where: { $0.uuid == BleUUID.service }),
              let characteristic = service.characteristics?.first(where: { $0.uuid == BleUUID.dataTransfer }) else {
            NSLog("BLE: Cannot send - data transfer characteristic not found")
            os_log("Cannot send - data transfer characteristic not found", log: bleLog, type: .error)
            return
        }

        // Create chunks with headers
        let chunks = chunker.createChunks(from: data, maxPayloadSize: mtu)

        NSLog("BLE: Sending \(data.count) bytes in \(chunks.count) chunks to \(deviceId)")
        os_log("Sending %d bytes in %d chunks to %{public}@", log: bleLog, type: .info, data.count, chunks.count, deviceId)

        // Add to write queue
        if writeQueue[deviceId] == nil {
            writeQueue[deviceId] = []
        }
        writeQueue[deviceId]?.append(contentsOf: chunks)

        // Start processing queue if not already writing
        if isWriting[deviceId] != true {
            processWriteQueue(deviceId: deviceId, peripheral: device.peripheral, characteristic: characteristic)
        }
    }

    /// Process write queue with flow control
    private func processWriteQueue(deviceId: String, peripheral: CBPeripheral, characteristic: CBCharacteristic) {
        guard var queue = writeQueue[deviceId], !queue.isEmpty else {
            isWriting[deviceId] = false
            NSLog("BLE: Write queue empty for \(deviceId)")
            return
        }

        isWriting[deviceId] = true

        // Get next chunk
        let chunk = queue.removeFirst()
        writeQueue[deviceId] = queue

        // Write chunk
        peripheral.writeValue(chunk, for: characteristic, type: .withoutResponse)

        // Update activity time
        updateActivity(for: deviceId)

        // Schedule next write with delay for flow control
        let sendQueue = DispatchQueue(label: "com.nearclip.ble.send.\(deviceId)")
        sendQueue.asyncAfter(deadline: .now() + .nanoseconds(Int(writeDelayMs))) { [weak self] in
            self?.processWriteQueue(deviceId: deviceId, peripheral: peripheral, characteristic: characteristic)
        }
    }

    /// Update MTU size (called after MTU negotiation)
    func setMtu(_ newMtu: Int) {
        self.mtu = max(20, newMtu - 3)  // Subtract ATT header
        os_log("MTU updated to %d", log: bleLog, type: .info, mtu)
    }

    // MARK: - Connection Health Monitoring

    /// Start monitoring connection health
    func startConnectionHealthMonitoring() {
        stopConnectionHealthMonitoring()

        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            self.connectionHealthTimer = Timer.scheduledTimer(withTimeInterval: self.connectionHealthInterval, repeats: true) { [weak self] _ in
                self?.checkConnectionHealth()
            }
        }
        os_log("Connection health monitoring started", log: bleLog, type: .info)
    }

    /// Stop monitoring connection health
    func stopConnectionHealthMonitoring() {
        connectionHealthTimer?.invalidate()
        connectionHealthTimer = nil
    }

    private func checkConnectionHealth() {
        let now = Date()

        for (deviceId, device) in connectedDevices {
            // Check if peripheral is still connected at the system level
            // If it's connected, update activity time and skip timeout check
            if device.peripheral.state == .connected {
                // Peripheral is still connected, update activity time
                lastActivityTimes[deviceId] = now
                continue
            }

            let lastActivity = lastActivityTimes[deviceId] ?? device.lastSeen

            if now.timeIntervalSince(lastActivity) > connectionTimeout {
                os_log("Connection timeout for device %{public}@, disconnecting", log: bleLog, type: .default, deviceId)
                NSLog("BLE: Connection timeout for device \(deviceId), disconnecting")

                // Force disconnect and trigger reconnection
                centralManager.cancelPeripheralConnection(device.peripheral)
            }
        }
    }

    /// Update last activity time for a device (thread-safe, updates on main thread)
    func updateActivity(for deviceId: String) {
        if Thread.isMainThread {
            lastActivityTimes[deviceId] = Date()
        } else {
            DispatchQueue.main.async {
                self.lastActivityTimes[deviceId] = Date()
            }
        }
    }

    /// Reset reconnection attempts for a device
    func resetReconnectAttempts(for peripheralId: UUID) {
        reconnectAttempts.removeValue(forKey: peripheralId)
    }

    // MARK: - Power Optimization

    /// Enable or disable smart scan pausing when connected
    func setScanPauseWhenConnected(_ enabled: Bool) {
        shouldPauseScanWhenConnected = enabled
        if !enabled {
            cancelScanPauseTimer()
        }
    }

    private func scheduleScanPause() {
        guard shouldPauseScanWhenConnected else { return }
        guard !connectedDevices.isEmpty else { return }

        cancelScanPauseTimer()

        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            self.scanPauseTimer = Timer.scheduledTimer(withTimeInterval: self.scanPauseInterval, repeats: false) { [weak self] _ in
                self?.pauseScanningForPowerSaving()
            }
        }
    }

    private func cancelScanPauseTimer() {
        scanPauseTimer?.invalidate()
        scanPauseTimer = nil
    }

    private func pauseScanningForPowerSaving() {
        guard isScanning && !connectedDevices.isEmpty else { return }

        os_log("Pausing scan for power saving (connected devices: %d)", log: bleLog, type: .info, connectedDevices.count)
        NSLog("BLE: Pausing scan for power saving (connected devices: \(connectedDevices.count))")

        centralManager.stopScan()
        // Keep wantsToScan = true so we resume when a device disconnects
        DispatchQueue.main.async {
            self.isScanning = false
        }
    }

    /// Resume scanning if needed (called when a device disconnects)
    private func resumeScanningIfNeeded() {
        guard wantsToScan && !isScanning else { return }
        guard centralManager.state == .poweredOn else { return }

        os_log("Resuming scan after device disconnect", log: bleLog, type: .info)
        NSLog("BLE: Resuming scan after device disconnect")

        centralManager.scanForPeripherals(
            withServices: [BleUUID.service],
            options: [
                CBCentralManagerScanOptionAllowDuplicatesKey: false
            ]
        )
        isScanning = true
    }

    // MARK: - Private Methods

    private func setupGattService() {
        // Device ID characteristic (Read)
        deviceIdCharacteristic = CBMutableCharacteristic(
            type: BleUUID.deviceId,
            properties: .read,
            value: localDeviceId.data(using: .utf8),
            permissions: .readable
        )

        // Public Key Hash characteristic (Read)
        publicKeyHashCharacteristic = CBMutableCharacteristic(
            type: BleUUID.publicKeyHash,
            properties: .read,
            value: localPublicKeyHash.data(using: .utf8),
            permissions: .readable
        )

        // Data Transfer characteristic (Write Without Response)
        dataTransferCharacteristic = CBMutableCharacteristic(
            type: BleUUID.dataTransfer,
            properties: .writeWithoutResponse,
            value: nil,
            permissions: .writeable
        )

        // Data ACK characteristic (Read + Notify)
        dataAckCharacteristic = CBMutableCharacteristic(
            type: BleUUID.dataAck,
            properties: [.read, .notify],
            value: nil,
            permissions: .readable
        )

        // Create service
        advertisedService = CBMutableService(type: BleUUID.service, primary: true)
        advertisedService?.characteristics = [
            deviceIdCharacteristic!,
            publicKeyHashCharacteristic!,
            dataTransferCharacteristic!,
            dataAckCharacteristic!
        ]

        // Add service to peripheral manager
        peripheralManager.add(advertisedService!)
        os_log("GATT service configured", log: bleLog, type: .info)
    }

    private func readDeviceInfo(from peripheral: CBPeripheral) {
        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            return
        }

        // Discover characteristics
        peripheral.discoverCharacteristics(
            [BleUUID.deviceId, BleUUID.publicKeyHash, BleUUID.dataTransfer, BleUUID.dataAck],
            for: service
        )
    }
}

// MARK: - CBCentralManagerDelegate

extension BleManager: CBCentralManagerDelegate {

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        let newState = central.state
        NSLog("BLE: Central state updated: \(newState.rawValue) (\(stateDescription(newState)))")
        DispatchQueue.main.async {
            self.centralState = newState
        }

        os_log("Central state updated: %d", log: bleLog, type: .info, newState.rawValue)

        if newState == .poweredOn && wantsToScan && !isScanning {
            // Start scanning now that Bluetooth is ready
            NSLog("BLE: Bluetooth ready, starting deferred scan")
            os_log("Bluetooth ready, starting deferred scan", log: bleLog, type: .info)
            // Use the actual state from central, not the async-updated centralState
            centralManager.scanForPeripherals(
                withServices: [BleUUID.service],
                options: [
                    CBCentralManagerScanOptionAllowDuplicatesKey: false
                ]
            )
            isScanning = true
            NSLog("BLE: Scan started from state update callback")
            os_log("Scan started from state update callback", log: bleLog, type: .info)
        }
    }

    private func stateDescription(_ state: CBManagerState) -> String {
        switch state {
        case .unknown: return "unknown"
        case .resetting: return "resetting"
        case .unsupported: return "unsupported"
        case .unauthorized: return "unauthorized"
        case .poweredOff: return "poweredOff"
        case .poweredOn: return "poweredOn"
        @unknown default: return "unknown(\(state.rawValue))"
        }
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {
        let peripheralId = peripheral.identifier

        NSLog("BLE: Discovered peripheral: \(peripheral.name ?? "Unknown") (\(peripheralId.uuidString)) RSSI: \(RSSI.intValue)")

        // Skip if already connected or connecting
        if peripheral.state == .connected || peripheral.state == .connecting {
            NSLog("BLE: Skipping - already connected/connecting")
            return
        }

        // Skip if we already have this peripheral in pending list
        if pendingPeripherals[peripheralId] != nil {
            NSLog("BLE: Skipping - already in pending list")
            return
        }

        // Skip if we already know this device's ID and it's connected
        if let existingDeviceId = peripheralDeviceIds[peripheralId] {
            // Check on main thread synchronously to avoid race condition
            var isConnected = false
            if Thread.isMainThread {
                isConnected = connectedDevices[existingDeviceId] != nil
            } else {
                DispatchQueue.main.sync {
                    isConnected = self.connectedDevices[existingDeviceId] != nil
                }
            }
            if isConnected {
                NSLog("BLE: Skipping - device already connected")
                return
            }
        }

        // Store peripheral for later connection
        pendingPeripherals[peripheralId] = peripheral

        NSLog("BLE: Connecting to peripheral: \(peripheral.name ?? "Unknown")")
        os_log("Discovered peripheral: %{public}@ RSSI: %d", log: bleLog, type: .info, peripheral.name ?? "Unknown", RSSI.intValue)

        // Connect to read device info
        central.connect(peripheral, options: nil)
    }

    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        NSLog("BLE: Connected to peripheral: \(peripheral.identifier.uuidString)")
        os_log("Connected to peripheral: %{public}@", log: bleLog, type: .info, peripheral.identifier.uuidString)

        // Clear from reconnect list and reset attempts on successful connection
        reconnectPeripherals.removeValue(forKey: peripheral.identifier)
        reconnectAttempts.removeValue(forKey: peripheral.identifier)

        peripheral.delegate = self
        peripheral.discoverServices([BleUUID.service])
    }

    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        os_log("Failed to connect: %{public}@", log: bleLog, type: .error, error?.localizedDescription ?? "Unknown error")

        pendingPeripherals.removeValue(forKey: peripheral.identifier)

        if let deviceId = peripheralDeviceIds[peripheral.identifier] {
            delegate?.bleManager(self, didFailWithError: error ?? NSError(domain: "BleManager", code: -1), forDevice: deviceId)
        }
    }

    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        let peripheralId = peripheral.identifier

        if let deviceId = peripheralDeviceIds[peripheralId] {
            os_log("Disconnected from device: %{public}@", log: bleLog, type: .info, deviceId)

            DispatchQueue.main.async {
                self.connectedDevices.removeValue(forKey: deviceId)
                self.lastActivityTimes.removeValue(forKey: deviceId)
                self.writeQueue.removeValue(forKey: deviceId)
                self.isWriting.removeValue(forKey: deviceId)
            }

            delegate?.bleManager(self, didDisconnectDevice: deviceId)
            peripheralDeviceIds.removeValue(forKey: peripheralId)

            // Resume scanning if no devices connected
            if connectedDevices.isEmpty {
                cancelScanPauseTimer()
                resumeScanningIfNeeded()
            }

            // Auto-reconnect if enabled and not exceeded max attempts
            if autoReconnect {
                let attempts = reconnectAttempts[peripheralId] ?? 0
                if attempts < maxReconnectAttempts {
                    reconnectPeripherals[peripheralId] = peripheral
                    let delay = calculateReconnectDelay(attempts: attempts)
                    reconnectAttempts[peripheralId] = attempts + 1

                    os_log("Scheduling reconnect attempt %d/%d in %.1fs for %{public}@",
                           log: bleLog, type: .info, attempts + 1, maxReconnectAttempts, delay, deviceId)
                    NSLog("BLE: Scheduling reconnect attempt \(attempts + 1)/\(maxReconnectAttempts) in \(delay)s for \(deviceId)")

                    DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
                        self?.attemptReconnect(peripheral)
                    }
                } else {
                    os_log("Max reconnect attempts reached for %{public}@, giving up", log: bleLog, type: .default, deviceId)
                    NSLog("BLE: Max reconnect attempts reached for \(deviceId), giving up")
                    reconnectAttempts.removeValue(forKey: peripheralId)
                    reconnectPeripherals.removeValue(forKey: peripheralId)
                }
            }
        }

        pendingPeripherals.removeValue(forKey: peripheralId)
    }

    /// Calculate reconnect delay with exponential backoff
    private func calculateReconnectDelay(attempts: Int) -> TimeInterval {
        let delay = baseReconnectDelay * pow(2.0, Double(attempts))
        return min(delay, maxReconnectDelay)
    }

    private func attemptReconnect(_ peripheral: CBPeripheral) {
        guard centralManager.state == .poweredOn else {
            os_log("Cannot reconnect - Bluetooth not powered on", log: bleLog, type: .default)
            return
        }
        guard reconnectPeripherals[peripheral.identifier] != nil else {
            os_log("Reconnect cancelled for %{public}@", log: bleLog, type: .info, peripheral.identifier.uuidString)
            return
        }

        os_log("Attempting to reconnect to %{public}@", log: bleLog, type: .info, peripheral.identifier.uuidString)
        NSLog("BLE: Attempting to reconnect to \(peripheral.identifier.uuidString)")
        centralManager.connect(peripheral, options: nil)
    }
}

// MARK: - CBPeripheralDelegate

extension BleManager: CBPeripheralDelegate {

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error = error {
            NSLog("BLE: Error discovering services: \(error.localizedDescription)")
            os_log("Error discovering services: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let service = peripheral.services?.first(where: { $0.uuid == BleUUID.service }) else {
            NSLog("BLE: NearClip service not found on peripheral")
            os_log("NearClip service not found", log: bleLog, type: .error)
            return
        }

        NSLog("BLE: Found NearClip service, discovering characteristics...")
        // Discover characteristics
        peripheral.discoverCharacteristics(
            [BleUUID.deviceId, BleUUID.publicKeyHash, BleUUID.dataTransfer, BleUUID.dataAck],
            for: service
        )
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error = error {
            NSLog("BLE: Error discovering characteristics: \(error.localizedDescription)")
            os_log("Error discovering characteristics: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let characteristics = service.characteristics else { return }

        NSLog("BLE: Discovered \(characteristics.count) characteristics")
        // Read device ID and public key hash
        for characteristic in characteristics {
            if characteristic.uuid == BleUUID.deviceId || characteristic.uuid == BleUUID.publicKeyHash {
                NSLog("BLE: Reading characteristic: \(characteristic.uuid.uuidString)")
                peripheral.readValue(for: characteristic)
            }

            // Subscribe to ACK notifications
            if characteristic.uuid == BleUUID.dataAck {
                peripheral.setNotifyValue(true, for: characteristic)
            }
        }
    }

    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            NSLog("BLE: Error reading characteristic \(characteristic.uuid.uuidString): \(error.localizedDescription)")
            os_log("Error reading characteristic: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }

        guard let data = characteristic.value else {
            NSLog("BLE: Characteristic \(characteristic.uuid.uuidString) has no value")
            return
        }

        NSLog("BLE: Read characteristic \(characteristic.uuid.uuidString): \(data.count) bytes")

        switch characteristic.uuid {
        case BleUUID.deviceId:
            if let deviceId = String(data: data, encoding: .utf8) {
                handleDeviceIdRead(deviceId, for: peripheral)
            }

        case BleUUID.publicKeyHash:
            if let hash = String(data: data, encoding: .utf8) {
                handlePublicKeyHashRead(hash, for: peripheral)
            }

        case BleUUID.dataAck:
            handleAckReceived(data, for: peripheral)

        default:
            break
        }
    }

    private func handleDeviceIdRead(_ deviceId: String, for peripheral: CBPeripheral) {
        let peripheralId = peripheral.identifier
        peripheralDeviceIds[peripheralId] = deviceId

        NSLog("BLE: Device ID read from peripheral \(peripheralId.uuidString): \(deviceId)")

        // Create or update device
        let device = BleDevice(
            id: deviceId,
            peripheral: peripheral,
            publicKeyHash: nil,
            rssi: 0,
            lastSeen: Date()
        )

        // Update discoveredDevices on BLE queue (used for scanning)
        discoveredDevices[deviceId] = device

        os_log("Device ID read: %{public}@", log: bleLog, type: .info, deviceId)
        NSLog("BLE: Calling delegate didDiscoverDevice and didConnectDevice for: \(deviceId)")

        // Schedule scan pause for power saving
        scheduleScanPause()

        // Update connectedDevices and lastActivityTimes on main thread (for health check)
        // Also notify delegate on main thread
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Update dictionaries on main thread where health check runs
            self.connectedDevices[deviceId] = device
            self.lastActivityTimes[deviceId] = Date()

            NSLog("BLE: Added device to connectedDevices on main thread: \(deviceId)")
            self.delegate?.bleManager(self, didDiscoverDevice: device)
            self.delegate?.bleManager(self, didConnectDevice: deviceId)
        }
    }

    private func handlePublicKeyHashRead(_ hash: String, for peripheral: CBPeripheral) {
        guard let deviceId = peripheralDeviceIds[peripheral.identifier] else { return }

        DispatchQueue.main.async {
            self.discoveredDevices[deviceId]?.publicKeyHash = hash
            self.connectedDevices[deviceId]?.publicKeyHash = hash
        }

        os_log("Public key hash read for %{public}@: %{public}@", log: bleLog, type: .info, deviceId, hash)
    }

    private func handleAckReceived(_ data: Data, for peripheral: CBPeripheral) {
        guard let deviceId = peripheralDeviceIds[peripheral.identifier] else { return }
        os_log("ACK received from %{public}@", log: bleLog, type: .info, deviceId)
        // TODO: Handle ACK for reliable transfer
    }

    func peripheral(_ peripheral: CBPeripheral, didModifyServices invalidatedServices: [CBService]) {
        os_log("Services modified for peripheral %{public}@", log: bleLog, type: .info, peripheral.identifier.uuidString)

        // Check if our service was invalidated
        let nearClipServiceInvalidated = invalidatedServices.contains { $0.uuid == BleUUID.service }

        if nearClipServiceInvalidated {
            os_log("NearClip service invalidated, rediscovering services", log: bleLog, type: .info)
            // Rediscover services
            peripheral.discoverServices([BleUUID.service])
        }
    }
}

// MARK: - CBPeripheralManagerDelegate

extension BleManager: CBPeripheralManagerDelegate {

    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        NSLog("BLE: Peripheral state updated: \(peripheral.state.rawValue) (\(stateDescription(peripheral.state)))")
        DispatchQueue.main.async {
            self.peripheralState = peripheral.state
        }

        os_log("Peripheral state updated: %d", log: bleLog, type: .info, peripheral.state.rawValue)

        if peripheral.state == .poweredOn && wantsToAdvertise && !isAdvertising {
            // Start advertising now that Bluetooth is ready
            NSLog("BLE: Bluetooth ready, starting deferred advertisement")
            os_log("Bluetooth ready, starting deferred advertisement", log: bleLog, type: .info)
            startAdvertising()
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error = error {
            NSLog("BLE: Error adding service: \(error.localizedDescription)")
            os_log("Error adding service: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            return
        }
        NSLog("BLE: Service added successfully")
        os_log("Service added successfully", log: bleLog, type: .info)
    }

    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error = error {
            NSLog("BLE: Error starting advertising: \(error.localizedDescription)")
            os_log("Error starting advertising: %{public}@", log: bleLog, type: .error, error.localizedDescription)
            DispatchQueue.main.async {
                self.isAdvertising = false
            }
            return
        }
        NSLog("BLE: Advertising started successfully")
        os_log("Advertising started successfully", log: bleLog, type: .info)
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveRead request: CBATTRequest) {
        NSLog("BLE: Received read request for characteristic: \(request.characteristic.uuid.uuidString)")
        // Handle read requests for our characteristics
        switch request.characteristic.uuid {
        case BleUUID.deviceId:
            if let data = localDeviceId.data(using: .utf8) {
                request.value = data
                peripheral.respond(to: request, withResult: .success)
                NSLog("BLE: Responded with deviceId: \(localDeviceId)")
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
        NSLog("BLE: Received \(requests.count) write request(s)")
        for request in requests {
            if request.characteristic.uuid == BleUUID.dataTransfer,
               let data = request.value {
                // Handle incoming data
                NSLog("BLE: Received data transfer: \(data.count) bytes")
                handleIncomingData(data, from: request.central)
            }
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didSubscribeTo characteristic: CBCharacteristic) {
        NSLog("BLE: Central subscribed to \(characteristic.uuid.uuidString)")
        os_log("Central subscribed to %{public}@", log: bleLog, type: .info, characteristic.uuid.uuidString)
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didUnsubscribeFrom characteristic: CBCharacteristic) {
        os_log("Central unsubscribed from %{public}@", log: bleLog, type: .info, characteristic.uuid.uuidString)
    }

    private func handleIncomingData(_ data: Data, from central: CBCentral) {
        let centralId = central.identifier.uuidString

        // Parse chunk header
        guard let (messageId, sequence, total, payload) = DataChunker.parseChunk(data) else {
            NSLog("BLE: Invalid chunk received from \(centralId), size: \(data.count)")
            os_log("Invalid chunk received from %{public}@", log: bleLog, type: .error, centralId)
            return
        }

        NSLog("BLE: Received chunk \(sequence + 1)/\(total) (msgId: \(messageId)) from \(centralId), payload: \(payload.count) bytes")

        // Get or create reassembler for this central
        if reassemblers[centralId] == nil {
            reassemblers[centralId] = DataReassembler()
        }

        guard let reassembler = reassemblers[centralId] else { return }

        // Check for timeout and reset if needed
        if reassembler.isTimedOut {
            NSLog("BLE: Reassembler timed out for \(centralId), resetting")
            reassembler.reset()
        }

        // Add chunk to reassembler
        if let completeData = reassembler.addChunk(payload, sequence: Int(sequence), total: Int(total), messageId: messageId) {
            NSLog("BLE: Complete message received from \(centralId): \(completeData.count) bytes")
            os_log("Complete message received from %{public}@: %d bytes", log: bleLog, type: .info, centralId, completeData.count)

            // Get device ID from central mapping, or use central ID as fallback
            let deviceId = centralDeviceIds[central.identifier] ?? centralId

            // Notify delegate
            delegate?.bleManager(self, didReceiveData: completeData, fromDevice: deviceId)

            // Send ACK for complete message
            sendAck(to: central, messageId: messageId)
        }
    }

    private func sendAck(to central: CBCentral, messageId: UInt32 = 0) {
        guard let characteristic = dataAckCharacteristic else { return }

        // ACK format: [messageId: 4 bytes]
        var ackData = Data()
        var msgId = messageId.littleEndian
        ackData.append(Data(bytes: &msgId, count: 4))

        peripheralManager.updateValue(ackData, for: characteristic, onSubscribedCentrals: [central])
        NSLog("BLE: Sent ACK for message \(messageId) to central")
    }
}

// MARK: - Data Reassembler

/// Header size for chunked data transfer
/// Format: [messageId: 4 bytes][sequence: 2 bytes][total: 2 bytes]
private let kChunkHeaderSize = 8

/// Reassembles chunked data received over BLE
class DataReassembler {
    private var chunks: [Int: Data] = [:]
    private var totalChunks: Int = 0
    private var messageId: UInt32 = 0
    private var lastActivityTime: Date = Date()

    /// Timeout for incomplete messages (30 seconds)
    private let timeout: TimeInterval = 30.0

    /// Check if the reassembler has timed out
    var isTimedOut: Bool {
        return Date().timeIntervalSince(lastActivityTime) > timeout
    }

    /// Reset the reassembler
    func reset() {
        chunks.removeAll()
        totalChunks = 0
        messageId = 0
    }

    func addChunk(_ data: Data, sequence: Int, total: Int, messageId: UInt32) -> Data? {
        lastActivityTime = Date()

        if self.messageId != messageId {
            // New message, reset
            self.chunks.removeAll()
            self.messageId = messageId
            self.totalChunks = total
        }

        chunks[sequence] = data

        // Check if complete
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

/// Chunks data for BLE transmission with header information
class DataChunker {
    private var messageIdCounter: UInt32 = 0

    /// Create chunks from data with headers
    /// Format: [messageId: 4 bytes][sequence: 2 bytes][total: 2 bytes][payload]
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

            // Build chunk with header
            var chunk = Data()

            // Message ID (4 bytes, little endian)
            var msgId = messageId.littleEndian
            chunk.append(Data(bytes: &msgId, count: 4))

            // Sequence number (2 bytes, little endian)
            var seq = sequence.littleEndian
            chunk.append(Data(bytes: &seq, count: 2))

            // Total chunks (2 bytes, little endian)
            var total = UInt16(totalChunks).littleEndian
            chunk.append(Data(bytes: &total, count: 2))

            // Payload
            chunk.append(payload)

            chunks.append(chunk)
            offset += chunkPayloadSize
            sequence += 1
        }

        // Handle empty data case
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

    /// Parse a chunk and extract header information
    static func parseChunk(_ data: Data) -> (messageId: UInt32, sequence: UInt16, total: UInt16, payload: Data)? {
        guard data.count >= kChunkHeaderSize else { return nil }

        let messageId = data.subdata(in: 0..<4).withUnsafeBytes { $0.load(as: UInt32.self).littleEndian }
        let sequence = data.subdata(in: 4..<6).withUnsafeBytes { $0.load(as: UInt16.self).littleEndian }
        let total = data.subdata(in: 6..<8).withUnsafeBytes { $0.load(as: UInt16.self).littleEndian }
        let payload = data.subdata(in: kChunkHeaderSize..<data.count)

        return (messageId, sequence, total, payload)
    }
}
