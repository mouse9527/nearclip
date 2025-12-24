import Foundation
import Combine
import AppKit

// MARK: - Notification Names

extension Notification.Name {
    static let devicePaired = Notification.Name("com.nearclip.devicePaired")
    static let requestAddDevice = Notification.Name("com.nearclip.requestAddDevice")
}

/// Retry strategy when sync fails after exhausting retries
enum SyncRetryStrategy: String, CaseIterable {
    case discard = "discard"        // Give up on this sync
    case waitForDevice = "wait"     // Queue content and send when device reconnects
    case continueRetry = "retry"    // Keep retrying indefinitely

    var displayName: String {
        switch self {
        case .discard:
            return "Discard"
        case .waitForDevice:
            return "Wait for Device"
        case .continueRetry:
            return "Continue Retrying"
        }
    }

    var description: String {
        switch self {
        case .discard:
            return "Give up on failed sync"
        case .waitForDevice:
            return "Queue and send when device reconnects"
        case .continueRetry:
            return "Keep retrying until successful"
        }
    }
}

/// Device information for display in UI
struct DeviceDisplay: Identifiable, Equatable {
    enum ConnectionType: Equatable {
        case wifi
        case ble
        case both
    }

    let id: String
    let name: String
    let platform: String
    let isConnected: Bool
    let lastSeen: Date?
    var isPaused: Bool
    var connectionType: ConnectionType

    init(id: String, name: String, platform: String, isConnected: Bool, lastSeen: Date? = nil, isPaused: Bool = false, connectionType: ConnectionType = .wifi) {
        self.id = id
        self.name = name
        self.platform = platform
        self.isConnected = isConnected
        self.lastSeen = lastSeen
        self.isPaused = isPaused
        self.connectionType = connectionType
    }
}

/// Connection status for display
enum ConnectionStatus: Equatable {
    case disconnected
    case connecting
    case connected(deviceCount: Int)
    case syncing
    case error(message: String)

    var displayText: String {
        switch self {
        case .disconnected:
            return "Not Connected"
        case .connecting:
            return "Connecting..."
        case .connected(let count):
            return count == 1 ? "1 Device Connected" : "\(count) Devices Connected"
        case .syncing:
            return "Syncing..."
        case .error(let message):
            return "Error: \(message)"
        }
    }

    var isConnected: Bool {
        switch self {
        case .connected, .syncing:
            return true
        default:
            return false
        }
    }

    var isSyncing: Bool {
        if case .syncing = self {
            return true
        }
        return false
    }

    var hasError: Bool {
        if case .error = self {
            return true
        }
        return false
    }

    /// Symbol name for menubar icon
    var symbolName: String {
        switch self {
        case .disconnected:
            return "link.badge.plus"
        case .connecting:
            return "arrow.triangle.2.circlepath"
        case .connected:
            return "link"
        case .syncing:
            return "arrow.triangle.2.circlepath"
        case .error:
            return "exclamationmark.triangle"
        }
    }

    /// Accessibility description for menubar icon
    var accessibilityDescription: String {
        switch self {
        case .disconnected:
            return "NearClip Disconnected"
        case .connecting:
            return "NearClip Connecting"
        case .connected(let count):
            return "NearClip Connected to \(count) device(s)"
        case .syncing:
            return "NearClip Syncing"
        case .error:
            return "NearClip Error"
        }
    }
}

/// Manages connection state and communicates with Rust core via FFI
final class ConnectionManager: ObservableObject {
    static let shared = ConnectionManager()

    /// Maximum number of paired devices allowed
    static let maxPairedDevices = 5

    @Published private(set) var status: ConnectionStatus = .disconnected
    @Published private(set) var connectedDevices: [DeviceDisplay] = []
    @Published private(set) var pairedDevices: [DeviceDisplay] = []
    @Published private(set) var lastError: String?
    @Published private(set) var lastSyncTime: Date?

    /// BLE connection status
    @Published private(set) var bleEnabled = false
    @Published private(set) var bleConnectedDevices: [String: BleDevice] = [:]

    private(set) var nearClipManager: FfiNearClipManager?
    private var isRunning = false
    private var syncInProgress = false
    private var previousStatus: ConnectionStatus = .disconnected

    /// Timer for periodic device list refresh
    private var refreshTimer: Timer?

    /// BLE Manager for Bluetooth communication
    private(set) var bleManager: BleManager?

    /// Pending content queue for "wait for device" strategy
    private var pendingContent: Data?

    /// Default retry strategy (loaded from UserDefaults)
    var defaultRetryStrategy: SyncRetryStrategy {
        let rawValue = UserDefaults.standard.string(forKey: "defaultRetryStrategy") ?? SyncRetryStrategy.waitForDevice.rawValue
        return SyncRetryStrategy(rawValue: rawValue) ?? .waitForDevice
    }

    /// Set of paused device IDs (stored in UserDefaults)
    private var pausedDeviceIds: Set<String> {
        get {
            Set(UserDefaults.standard.stringArray(forKey: "pausedDeviceIds") ?? [])
        }
        set {
            UserDefaults.standard.set(Array(newValue), forKey: "pausedDeviceIds")
        }
    }

    /// Device ID persistence key
    private static let deviceIdKey = "nearclip.deviceId"

    /// Get or generate device ID (persisted in UserDefaults)
    private var persistedDeviceId: String {
        get {
            UserDefaults.standard.string(forKey: Self.deviceIdKey) ?? ""
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Self.deviceIdKey)
        }
    }

    /// Check if we can add more devices
    var canAddMoreDevices: Bool {
        pairedDevices.count < Self.maxPairedDevices
    }

    /// Get the device ID used for pairing and mDNS
    /// Returns the persisted device ID, or queries the manager if available
    var deviceId: String {
        if let manager = nearClipManager {
            return manager.getDeviceId()
        }
        return persistedDeviceId
    }

    private init() {
        // Load paired devices from Keychain
        loadPairedDevicesFromKeychain()

        // Initialize BLE manager
        NSLog("ConnectionManager: Initializing BleManager")
        bleManager = BleManager()
        bleManager?.delegate = self
        bleManager?.startConnectionHealthMonitoring()
        NSLog("ConnectionManager: BleManager initialized, delegate set")
    }

    // MARK: - Public API

    /// Check if the service is currently running
    var isServiceRunning: Bool {
        return isRunning
    }

    /// Start the NearClip service
    func start() {
        guard !isRunning else { return }

        status = .connecting
        notifyStatusChange()

        do {
            // Initialize logging
            initLogging(level: .info)

            // Create config with persisted device ID
            let config = FfiNearClipConfig(
                deviceName: Host.current().localizedName ?? "Mac",
                deviceId: persistedDeviceId, // 使用持久化的设备 ID，空字符串会自动生成新的
                wifiEnabled: true,
                bleEnabled: true,
                autoConnect: true,
                connectionTimeoutSecs: 30,
                heartbeatIntervalSecs: 10,
                maxRetries: 3
            )

            // Create callback handler
            let callback = NearClipCallbackHandler(manager: self)

            // Create manager
            nearClipManager = try FfiNearClipManager(config: config, callback: callback)
            try nearClipManager?.start()

            // Save generated device ID if it was newly created
            if let manager = nearClipManager {
                let generatedId = manager.getDeviceId()
                if persistedDeviceId.isEmpty && !generatedId.isEmpty {
                    persistedDeviceId = generatedId
                    print("Saved new device ID: \(generatedId)")
                }

                // Configure and start BLE
                // Use a hash of device ID as public key hash for now
                // TODO: Get actual public key hash from FFI when available
                let publicKeyHash = generatedId.data(using: .utf8)?.sha256Hash ?? ""
                NSLog("ConnectionManager: Configuring BLE with deviceId=\(generatedId)")
                bleManager?.configure(deviceId: generatedId, publicKeyHash: publicKeyHash)
                NSLog("ConnectionManager: Starting BLE advertising")
                bleManager?.startAdvertising()
                NSLog("ConnectionManager: Starting BLE scanning")
                bleManager?.startScanning()
                bleEnabled = true
                NSLog("ConnectionManager: BLE enabled")
            }

            isRunning = true
            lastError = nil

            // Add paired devices from Keychain to FFI manager
            for device in pairedDevices {
                let ffiDevice = FfiDeviceInfo(
                    id: device.id,
                    name: device.name,
                    platform: platformFromString(device.platform),
                    status: DeviceStatus.disconnected
                )
                nearClipManager?.addPairedDevice(device: ffiDevice)
                print("Added Keychain device to FFI manager: \(device.name) (\(device.id))")
            }

            // Try to connect to paired devices after a delay to allow mDNS discovery
            DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) { [weak self] in
                guard let self = self else { return }
                let connectedCount = self.nearClipManager?.tryConnectPairedDevices() ?? 0
                print("tryConnectPairedDevices returned: \(connectedCount)")

                // Refresh device list after connection attempt
                self.refreshDeviceLists()
            }

            // Refresh device lists
            refreshDeviceLists()

            // Start periodic refresh timer (every 2 seconds)
            refreshTimer?.invalidate()
            refreshTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { [weak self] _ in
                self?.refreshDeviceLists()
            }

            print("NearClip service started")
        } catch {
            let errorMessage = String(describing: error)
            print("Failed to start NearClip: \(errorMessage)")
            lastError = errorMessage
            status = .error(message: "Failed to start")
            notifyStatusChange()
        }
    }

    /// Stop the NearClip service
    func stop() {
        // Stop refresh timer
        refreshTimer?.invalidate()
        refreshTimer = nil

        // Stop BLE
        bleManager?.stopScanning()
        bleManager?.stopAdvertising()
        bleManager?.stopConnectionHealthMonitoring()
        bleEnabled = false
        bleConnectedDevices.removeAll()

        nearClipManager?.stop()
        nearClipManager = nil
        isRunning = false
        syncInProgress = false
        status = .disconnected
        connectedDevices = []
        notifyStatusChange()
        print("NearClip service stopped")
    }

    /// Restart the NearClip service
    func restart() {
        stop()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            self?.start()
        }
    }

    /// Handle network loss - attempt to connect to paired devices via BLE
    func handleNetworkLost() {
        guard isRunning else { return }

        print("ConnectionManager: Network lost, attempting BLE fallback for paired devices")

        // Mark all WiFi-connected devices as disconnected (they will be updated if BLE is available)
        for i in connectedDevices.indices {
            if connectedDevices[i].connectionType == .wifi {
                // Check if device is also connected via BLE
                if bleConnectedDevices[connectedDevices[i].id] != nil {
                    connectedDevices[i].connectionType = .ble
                    print("ConnectionManager: Device \(connectedDevices[i].name) switched to BLE")
                }
            } else if connectedDevices[i].connectionType == .both {
                connectedDevices[i].connectionType = .ble
                print("ConnectionManager: Device \(connectedDevices[i].name) now BLE-only")
            }
        }

        // Remove devices that were WiFi-only and not connected via BLE
        let wifiOnlyDevices = connectedDevices.filter {
            $0.connectionType == .wifi && bleConnectedDevices[$0.id] == nil
        }
        for device in wifiOnlyDevices {
            print("ConnectionManager: Device \(device.name) lost (WiFi-only, no BLE)")
        }
        connectedDevices.removeAll { $0.connectionType == .wifi && bleConnectedDevices[$0.id] == nil }

        // Try to connect to paired devices via BLE if not already connected
        for device in pairedDevices {
            if bleConnectedDevices[device.id] == nil {
                // Check if device was discovered via BLE
                if bleManager?.discoveredDevices[device.id] != nil {
                    print("ConnectionManager: Attempting BLE connection to \(device.name)")
                    bleManager?.connect(deviceId: device.id)
                } else {
                    print("ConnectionManager: Device \(device.name) not discovered via BLE, scanning...")
                }
            }
        }

        // Ensure BLE scanning is active to discover devices
        if bleManager?.isScanning == false {
            bleManager?.startScanning()
        }

        updateStatus()
    }

    /// Sync clipboard content to connected devices
    func syncClipboard(_ content: Data) {
        guard isRunning else {
            print("Cannot sync: service not running")
            return
        }

        // Filter out paused devices
        let activeDevices = connectedDevices.filter { !pausedDeviceIds.contains($0.id) }

        guard !activeDevices.isEmpty else {
            print("Cannot sync: no active connected devices (all paused or none connected)")
            return
        }

        // Set syncing state
        setSyncing(true)

        // Separate devices by connection type
        let wifiDevices = activeDevices.filter { $0.connectionType == .wifi || $0.connectionType == .both }
        let bleOnlyDevices = activeDevices.filter { $0.connectionType == .ble }

        var syncedDevices: [DeviceDisplay] = []

        // Sync via WiFi (preferred)
        if !wifiDevices.isEmpty {
            do {
                try nearClipManager?.syncClipboard(content: content)
                syncedDevices.append(contentsOf: wifiDevices)
                print("Clipboard synced via WiFi to \(wifiDevices.count) device(s)")
            } catch {
                print("WiFi sync failed: \(error), will try BLE for these devices")
                // WiFi failed, try BLE for devices that support it
                for device in wifiDevices {
                    if isDeviceConnectedViaBle(device.id) {
                        syncClipboardViaBle(content, to: device.id)
                        syncedDevices.append(device)
                    }
                }
            }
        }

        // Sync via BLE for BLE-only devices
        for device in bleOnlyDevices {
            syncClipboardViaBle(content, to: device.id)
            syncedDevices.append(device)
            print("Clipboard synced via BLE to \(device.name)")
        }

        if !syncedDevices.isEmpty {
            lastSyncTime = Date()
            lastError = nil
            print("Clipboard synced to \(syncedDevices.count) device(s): \(content.count) bytes")

            // Record sync history
            for device in syncedDevices {
                SyncHistoryManager.shared.recordSent(
                    deviceId: device.id,
                    deviceName: device.name,
                    content: content
                )
            }
        } else {
            lastError = "Failed to sync to any device"
        }

        // Clear syncing state after a brief delay
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            self?.setSyncing(false)
        }
    }

    // MARK: - Device Management

    /// Connect to a specific device
    func connectDevice(_ deviceId: String) {
        guard isRunning else { return }

        do {
            try nearClipManager?.connectDevice(deviceId: deviceId)
        } catch {
            print("Failed to connect device: \(error)")
            lastError = String(describing: error)
        }
    }

    /// Disconnect from a specific device
    func disconnectDevice(_ deviceId: String) {
        guard isRunning else { return }

        do {
            try nearClipManager?.disconnectDevice(deviceId: deviceId)
        } catch {
            print("Failed to disconnect device: \(error)")
        }
    }

    /// Pause syncing for a specific device
    func pauseDevice(_ deviceId: String) {
        var paused = pausedDeviceIds
        paused.insert(deviceId)
        pausedDeviceIds = paused

        // Update the paired devices list to reflect the change
        if let index = pairedDevices.firstIndex(where: { $0.id == deviceId }) {
            var device = pairedDevices[index]
            device.isPaused = true
            pairedDevices[index] = device
        }

        print("Device paused: \(deviceId)")
    }

    /// Resume syncing for a specific device
    func resumeDevice(_ deviceId: String) {
        var paused = pausedDeviceIds
        paused.remove(deviceId)
        pausedDeviceIds = paused

        // Update the paired devices list to reflect the change
        if let index = pairedDevices.firstIndex(where: { $0.id == deviceId }) {
            var device = pairedDevices[index]
            device.isPaused = false
            pairedDevices[index] = device
        }

        print("Device resumed: \(deviceId)")
    }

    /// Check if a device is paused
    func isDevicePaused(_ deviceId: String) -> Bool {
        pausedDeviceIds.contains(deviceId)
    }

    /// Refresh the device lists from FFI
    func refreshDeviceLists() {
        guard let manager = nearClipManager else { return }

        // Get connected devices
        let ffiConnected = manager.getConnectedDevices()
        let connected = ffiConnected.map { device in
            DeviceDisplay(
                id: device.id,
                name: device.name,
                platform: platformString(device.platform),
                isConnected: true,
                lastSeen: Date()
            )
        }

        // Get paired devices
        let ffiPaired = manager.getPairedDevices()
        let paired = ffiPaired.map { device in
            let isConnected = connected.contains { $0.id == device.id }
            return DeviceDisplay(
                id: device.id,
                name: device.name,
                platform: platformString(device.platform),
                isConnected: isConnected
            )
        }

        DispatchQueue.main.async { [weak self] in
            self?.connectedDevices = connected
            self?.pairedDevices = paired
            self?.updateStatus()
        }
    }

    // MARK: - Internal Callbacks (called from FFI)

    func handleDeviceConnected(_ device: FfiDeviceInfo) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Check if already connected via BLE
            let isBleConnected = self.bleConnectedDevices[device.id] != nil
            let connectionType: DeviceDisplay.ConnectionType = isBleConnected ? .both : .wifi

            let display = DeviceDisplay(
                id: device.id,
                name: device.name,
                platform: self.platformString(device.platform),
                isConnected: true,
                lastSeen: Date(),
                connectionType: connectionType
            )

            // Update connected devices
            self.connectedDevices.removeAll { $0.id == device.id }
            self.connectedDevices.append(display)

            // Auto-add to paired devices if not already present (bidirectional pairing)
            if !self.pairedDevices.contains(where: { $0.id == device.id }) {
                print("Auto-adding connected device to paired list: \(device.name)")
                self.savePairedDeviceToKeychain(display)
                self.pairedDevices.append(display)

                // Notify that a new device was paired (for closing pairing window)
                NotificationCenter.default.post(
                    name: .devicePaired,
                    object: nil,
                    userInfo: ["device": display]
                )
            } else {
                // Update paired devices status
                if let index = self.pairedDevices.firstIndex(where: { $0.id == device.id }) {
                    self.pairedDevices[index] = DeviceDisplay(
                        id: device.id,
                        name: device.name,
                        platform: self.platformString(device.platform),
                        isConnected: true
                    )
                }
            }

            self.updateStatus()
            print("Device connected: \(device.name)")

            // Send pending content if using "wait for device" strategy
            self.sendPendingContentIfNeeded()
        }
    }

    func handleDeviceDisconnected(_ deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Check if still connected via BLE
            let isBleConnected = self.bleConnectedDevices[deviceId] != nil

            if isBleConnected {
                // WiFi disconnected but BLE still connected, update connection type
                if let index = self.connectedDevices.firstIndex(where: { $0.id == deviceId }) {
                    self.connectedDevices[index].connectionType = .ble
                }
                print("Device WiFi disconnected, still connected via BLE: \(deviceId)")
            } else {
                // Fully disconnected
                let disconnectedDevice = self.connectedDevices.first { $0.id == deviceId }
                self.connectedDevices.removeAll { $0.id == deviceId }

                // Update paired devices status
                if let index = self.pairedDevices.firstIndex(where: { $0.id == deviceId }),
                   let device = disconnectedDevice {
                    self.pairedDevices[index] = DeviceDisplay(
                        id: device.id,
                        name: device.name,
                        platform: device.platform,
                        isConnected: false
                    )
                }
                print("Device disconnected: \(deviceId)")
            }

            self.updateStatus()
        }
    }

    func handleDeviceUnpaired(_ deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            print("Device unpaired by remote: \(deviceId)")

            // Remove from connected devices
            self.connectedDevices.removeAll { $0.id == deviceId }

            // Remove from paired devices list
            self.pairedDevices.removeAll { $0.id == deviceId }

            // Remove from Keychain storage
            self.removePairedDeviceFromKeychain(deviceId)

            self.updateStatus()
        }
    }

    func handlePairingRejected(_ deviceId: String, reason: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            print("Pairing rejected by device: \(deviceId), reason: \(reason)")

            // Get device name for user-friendly message
            let deviceName = self.pairedDevices.first { $0.id == deviceId }?.name ?? deviceId

            // Remove from paired devices since the remote doesn't recognize us
            self.pairedDevices.removeAll { $0.id == deviceId }
            self.connectedDevices.removeAll { $0.id == deviceId }

            // Remove from Keychain storage
            self.removePairedDeviceFromKeychain(deviceId)

            // Remove from FFI manager
            self.nearClipManager?.removePairedDevice(deviceId: deviceId)

            // Show user notification about the rejection
            NotificationManager.shared.showPairingRejectedNotification(
                deviceName: deviceName,
                reason: reason
            )

            self.lastError = "Pairing rejected: \(reason)"
            self.updateStatus()
        }
    }

    func handleClipboardReceived(_ content: Data, from deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Set syncing state
            self.setSyncing(true)

            // Write to local clipboard (markAsRemote prevents sync loop)
            let success = ClipboardWriter.shared.write(content)

            // Get device name for history and notification
            let deviceName = self.connectedDevices.first { $0.id == deviceId }?.name ?? deviceId

            if success {
                self.lastSyncTime = Date()
                self.lastError = nil
                print("Received and wrote clipboard from \(deviceId): \(content.count) bytes")

                // Record in sync history
                SyncHistoryManager.shared.recordReceived(
                    deviceId: deviceId,
                    deviceName: deviceName,
                    content: content
                )

                // Show notification
                let contentPreview = String(data: content, encoding: .utf8)
                NotificationManager.shared.showSyncSuccessNotification(
                    fromDevice: deviceName,
                    contentPreview: contentPreview
                )
            } else {
                self.lastError = "Failed to write clipboard"
                print("Failed to write clipboard from \(deviceId)")

                // Record error in sync history
                SyncHistoryManager.shared.recordError(
                    direction: .received,
                    deviceId: deviceId,
                    deviceName: deviceName,
                    errorMessage: "Failed to write clipboard"
                )
            }

            // Clear syncing state after a brief delay
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) { [weak self] in
                self?.setSyncing(false)
            }
        }
    }

    /// Simulate receiving clipboard content (for testing)
    func simulateReceiveClipboard(_ content: String) {
        guard let data = content.data(using: .utf8) else { return }
        handleClipboardReceived(data, from: "test-device")
    }

    func handleSyncError(_ error: String) {
        DispatchQueue.main.async { [weak self] in
            self?.lastError = error
            print("Sync error: \(error)")

            // Show failure notification
            NotificationManager.shared.showSyncFailureNotification(reason: error)
        }
    }

    // MARK: - Retry Strategy Execution

    /// Execute the discard strategy - clear pending content
    func executeDiscardStrategy() {
        pendingContent = nil
        lastError = nil
        print("Retry strategy: Discarded failed sync content")
    }

    /// Execute the wait for device strategy - queue content for later
    func executeWaitForDeviceStrategy(content: Data? = nil) {
        if let newContent = content {
            pendingContent = newContent
        }
        print("Retry strategy: Content queued, waiting for device reconnection")
    }

    /// Execute the continue retry strategy - retry sync immediately
    func executeContinueRetryStrategy() {
        print("Retry strategy: Continuing retry")
        // Trigger clipboard monitor to retry
        ClipboardMonitor.shared.syncCurrentClipboard()
    }

    /// Check and send pending content when a device connects
    private func sendPendingContentIfNeeded() {
        guard let content = pendingContent else { return }
        guard !connectedDevices.isEmpty else { return }

        print("Sending pending content to reconnected device(s)")

        do {
            try nearClipManager?.syncClipboard(content: content)
            lastSyncTime = Date()
            lastError = nil
            // Only clear pending content on successful sync
            pendingContent = nil
            print("Pending content sent successfully: \(content.count) bytes")
        } catch {
            // Keep pending content for next attempt
            let errorMessage = String(describing: error)
            print("Failed to send pending content, will retry later: \(errorMessage)")
        }
    }

    /// Apply the default retry strategy for the given content
    func applyDefaultRetryStrategy(forContent content: Data) {
        switch defaultRetryStrategy {
        case .discard:
            executeDiscardStrategy()
        case .waitForDevice:
            executeWaitForDeviceStrategy(content: content)
        case .continueRetry:
            executeContinueRetryStrategy()
        }
    }

    // MARK: - Private

    private func setSyncing(_ syncing: Bool) {
        syncInProgress = syncing
        if syncing {
            previousStatus = status
            status = .syncing
        } else {
            // Restore previous status
            updateStatus()
        }
        notifyStatusChange()
    }

    private func updateStatus() {
        if !isRunning {
            status = .disconnected
        } else if syncInProgress {
            status = .syncing
        } else if connectedDevices.isEmpty {
            status = .connecting
        } else {
            status = .connected(deviceCount: connectedDevices.count)
        }
        notifyStatusChange()
    }

    private func notifyStatusChange() {
        // Notify app delegate to update icon
        DispatchQueue.main.async {
            if let appDelegate = NSApp.delegate as? AppDelegate {
                appDelegate.updateStatusIcon(for: self.status)
            }
        }
    }

    private func platformString(_ platform: DevicePlatform) -> String {
        switch platform {
        case .macOs:
            return "macOS"
        case .android:
            return "Android"
        case .unknown:
            return "Unknown"
        }
    }

    // MARK: - Keychain Integration

    /// Load paired devices from Keychain
    func loadPairedDevicesFromKeychain() {
        let storedDevices = KeychainManager.shared.loadPairedDevices()
        let paused = pausedDeviceIds
        pairedDevices = storedDevices.map { stored in
            var device = stored.toDeviceDisplay()
            device.isPaused = paused.contains(device.id)
            return device
        }
        print("ConnectionManager: Loaded \(pairedDevices.count) paired devices from Keychain")
    }

    /// Save a device to Keychain
    func savePairedDeviceToKeychain(_ device: DeviceDisplay) {
        let storedDevice = StoredDevice(from: device)
        if KeychainManager.shared.addPairedDevice(storedDevice) {
            print("ConnectionManager: Saved device '\(device.name)' to Keychain")
        }
    }

    /// Remove a device from Keychain
    func removePairedDeviceFromKeychain(_ deviceId: String) {
        if KeychainManager.shared.removePairedDevice(deviceId: deviceId) {
            print("ConnectionManager: Removed device '\(deviceId)' from Keychain")
        }
    }

    /// Add a paired device (saves to Keychain and updates FFI)
    /// Returns false if the maximum device limit has been reached
    @discardableResult
    func addPairedDevice(_ device: DeviceDisplay) -> Bool {
        // Check if device already exists (updating existing device)
        let isExisting = pairedDevices.contains { $0.id == device.id }

        // Check maximum device limit for new devices
        if !isExisting && pairedDevices.count >= Self.maxPairedDevices {
            print("ConnectionManager: Cannot add device - maximum \(Self.maxPairedDevices) devices reached")
            lastError = "Maximum \(Self.maxPairedDevices) devices reached"
            return false
        }

        // Save to Keychain
        savePairedDeviceToKeychain(device)

        // Update local list
        pairedDevices.removeAll { $0.id == device.id }
        pairedDevices.append(device)

        // Update FFI if available
        if let manager = nearClipManager {
            let ffiDevice = FfiDeviceInfo(
                id: device.id,
                name: device.name,
                platform: platformFromString(device.platform),
                status: .disconnected
            )
            manager.addPairedDevice(device: ffiDevice)
        }

        return true
    }

    /// Remove a paired device (sends unpair notification to remote device, removes from Keychain and updates FFI)
    func removePairedDevice(_ deviceId: String) {
        // Unpair device via FFI (sends notification to remote device and removes from FFI)
        do {
            try nearClipManager?.unpairDevice(deviceId: deviceId)
            print("ConnectionManager: Unpaired device via FFI: \(deviceId)")
        } catch {
            print("ConnectionManager: Failed to unpair device via FFI: \(deviceId), error: \(error)")
        }

        // Remove from Keychain
        removePairedDeviceFromKeychain(deviceId)

        // Remove from paused list
        var paused = pausedDeviceIds
        paused.remove(deviceId)
        pausedDeviceIds = paused

        // Update local list
        pairedDevices.removeAll { $0.id == deviceId }
        connectedDevices.removeAll { $0.id == deviceId }

        updateStatus()
    }

    private func platformFromString(_ string: String) -> DevicePlatform {
        switch string.lowercased() {
        case "macos":
            return .macOs
        case "android":
            return .android
        default:
            return .unknown
        }
    }
}

// MARK: - FFI Callback Handler

/// Callback handler that bridges FFI callbacks to ConnectionManager
final class NearClipCallbackHandler: FfiNearClipCallback {
    private weak var manager: ConnectionManager?

    init(manager: ConnectionManager) {
        self.manager = manager
    }

    func onDeviceConnected(device: FfiDeviceInfo) {
        manager?.handleDeviceConnected(device)
    }

    func onDeviceDisconnected(deviceId: String) {
        manager?.handleDeviceDisconnected(deviceId)
    }

    func onDeviceUnpaired(deviceId: String) {
        manager?.handleDeviceUnpaired(deviceId)
    }

    func onPairingRejected(deviceId: String, reason: String) {
        manager?.handlePairingRejected(deviceId, reason: reason)
    }

    func onClipboardReceived(content: Data, fromDevice: String) {
        manager?.handleClipboardReceived(content, from: fromDevice)
    }

    func onSyncError(errorMessage: String) {
        manager?.handleSyncError(errorMessage)
    }
}

// MARK: - BleManagerDelegate

extension ConnectionManager: BleManagerDelegate {

    func bleManager(_ manager: BleManager, didDiscoverDevice device: BleDevice) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            print("BLE: Discovered device: \(device.id)")

            // Check if this is a paired device and not already connected via BLE
            if self.pairedDevices.contains(where: { $0.id == device.id }) {
                // Only connect if not already connected via BLE
                if self.bleConnectedDevices[device.id] == nil {
                    print("BLE: Auto-connecting to paired device: \(device.id)")
                    manager.connect(deviceId: device.id)
                }
            }
        }
    }

    func bleManager(_ manager: BleManager, didLoseDevice deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            self?.bleConnectedDevices.removeValue(forKey: deviceId)
            print("BLE: Lost device: \(deviceId)")
        }
    }

    func bleManager(_ manager: BleManager, didConnectDevice deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            NSLog("ConnectionManager: BLE didConnectDevice called for: \(deviceId)")

            if let device = manager.connectedDevices[deviceId] {
                self.bleConnectedDevices[deviceId] = device
                NSLog("ConnectionManager: Added to bleConnectedDevices: \(deviceId)")
            } else {
                NSLog("ConnectionManager: Warning - device not found in manager.connectedDevices: \(deviceId)")
            }

            print("BLE: Connected to device: \(deviceId)")

            // If WiFi connection is not available, use BLE
            if !self.connectedDevices.contains(where: { $0.id == deviceId }) {
                // Get device info from paired devices if available
                let pairedDevice = self.pairedDevices.first { $0.id == deviceId }
                let deviceName = pairedDevice?.name ?? "BLE Device"
                let platform = pairedDevice?.platform ?? "Unknown"

                // Create a DeviceDisplay for the BLE-connected device
                let display = DeviceDisplay(
                    id: deviceId,
                    name: deviceName,
                    platform: platform,
                    isConnected: true,
                    lastSeen: Date(),
                    connectionType: .ble
                )

                // Add to connected devices if not already present via WiFi
                self.connectedDevices.append(display)
                self.updateStatus()
                NSLog("ConnectionManager: Added BLE device to connectedDevices: \(deviceName) (\(deviceId)), total: \(self.connectedDevices.count)")
                print("BLE: Added device to connected list: \(deviceName) (\(deviceId))")
            } else {
                // Device already connected via WiFi, update to .both
                if let index = self.connectedDevices.firstIndex(where: { $0.id == deviceId }) {
                    self.connectedDevices[index].connectionType = .both
                    NSLog("ConnectionManager: Updated device to .both: \(deviceId)")
                    print("BLE: Device already connected via WiFi, updated to .both: \(deviceId)")
                }
            }
        }
    }

    func bleManager(_ manager: BleManager, didDisconnectDevice deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            self.bleConnectedDevices.removeValue(forKey: deviceId)
            print("BLE: Disconnected from device: \(deviceId)")

            // Update connection type if device was connected via both
            if let index = self.connectedDevices.firstIndex(where: { $0.id == deviceId }) {
                if self.connectedDevices[index].connectionType == .both {
                    self.connectedDevices[index].connectionType = .wifi
                } else if self.connectedDevices[index].connectionType == .ble {
                    // BLE-only device disconnected, remove from list
                    self.connectedDevices.remove(at: index)
                    self.updateStatus()
                }
            }
        }
    }

    func bleManager(_ manager: BleManager, didReceiveData data: Data, fromDevice deviceId: String) {
        print("BLE: Received \(data.count) bytes from \(deviceId)")

        // Handle received clipboard data
        handleClipboardReceived(data, from: deviceId)
    }

    func bleManager(_ manager: BleManager, didFailWithError error: Error, forDevice deviceId: String?) {
        print("BLE: Error for device \(deviceId ?? "unknown"): \(error.localizedDescription)")
        lastError = "BLE: \(error.localizedDescription)"
    }

    // MARK: - BLE Public API

    /// Send clipboard data via BLE to a specific device
    func syncClipboardViaBle(_ content: Data, to deviceId: String) {
        bleManager?.sendData(content, to: deviceId)
    }

    /// Check if a device is connected via BLE
    func isDeviceConnectedViaBle(_ deviceId: String) -> Bool {
        return bleConnectedDevices[deviceId] != nil
    }
}
