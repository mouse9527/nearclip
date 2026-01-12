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

    private(set) var nearClipManager: FfiNearClipManager?
    private var isRunning = false
    private var syncInProgress = false
    private var previousStatus: ConnectionStatus = .disconnected

    /// Timer for periodic device list refresh
    private var refreshTimer: Timer?

    /// Track last auto-pair attempt time for devices to avoid spamming but allow retries
    private var lastAutoPairAttempt: [String: Date] = [:]

    /// Track last discovery notification time to throttle FFI calls and logs
    private var lastDiscoveryNotification: [String: Date] = [:]
    private let discoveryLock = NSLock()

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
        // Initialize BLE manager
        NSLog("ConnectionManager: Initializing BleManager")
        bleManager = BleManager()
        bleManager?.delegate = self
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

            // Initialize history storage
            if let manager = nearClipManager {
                let dbPath = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
                    .appendingPathComponent("NearClip")
                    .appendingPathComponent("history.db")
                    .path
                // Ensure directory exists
                try? FileManager.default.createDirectory(
                    atPath: (dbPath as NSString).deletingLastPathComponent,
                    withIntermediateDirectories: true
                )
                try manager.initHistory(dbPath: dbPath)
                SyncHistoryManager.shared.setManager(manager)
                print("History storage initialized at: \(dbPath)")
            }

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
                NSLog("ConnectionManager: Starting BLE advertising, bleManager: \(bleManager == nil ? "nil" : "exists")")
                bleManager?.startAdvertising()
                NSLog("ConnectionManager: Starting BLE scanning")
                bleManager?.startScanning()
                bleEnabled = true
                NSLog("ConnectionManager: BLE enabled")

                // Register BLE hardware bridge with FFI manager
                let bleHardwareBridge = BleHardwareBridge(bleManager: bleManager)
                manager.setBleHardware(hardware: bleHardwareBridge)
                NSLog("ConnectionManager: BLE hardware bridge registered with FFI manager")

                // Register device storage with FFI manager
                // This will load paired devices from storage automatically
                let deviceStorage = DeviceStorageImpl()
                manager.setDeviceStorage(storage: deviceStorage)
                NSLog("ConnectionManager: Device storage registered with FFI manager")
            }

            isRunning = true
            lastError = nil

            // Refresh paired devices from FFI manager (loaded by setDeviceStorage)
            refreshDeviceLists()

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

            // Start periodic refresh timer (every 10 seconds - reduced from 2s to prevent UI lag)
            refreshTimer?.invalidate()
            refreshTimer = Timer.scheduledTimer(withTimeInterval: 10.0, repeats: true) { [weak self] _ in
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
        bleEnabled = false

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
                if bleManager?.isConnected(peripheralUuid: connectedDevices[i].id) == true {
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
            $0.connectionType == .wifi && bleManager?.isConnected(peripheralUuid: $0.id) != true
        }
        for device in wifiOnlyDevices {
            print("ConnectionManager: Device \(device.name) lost (WiFi-only, no BLE)")
        }
        connectedDevices.removeAll { $0.connectionType == .wifi && bleManager?.isConnected(peripheralUuid: $0.id) != true }

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
    ///
    /// Tries WiFi first via FFI, then falls back to BLE if WiFi is not available.
    func connectDevice(_ deviceId: String) {
        guard isRunning else { return }

        do {
            try nearClipManager?.connectDevice(deviceId: deviceId)
        } catch {
            print("WiFi connection failed: \(error), trying BLE fallback")

            // Try BLE connection as fallback
            if let bleManager = bleManager {
                // Check if we have a peripheral UUID for this device
                // The device might be discoverable via BLE even if WiFi failed
                bleManager.connect(peripheralUuid: deviceId)
                print("BLE connection initiated for device: \(deviceId)")
            } else {
                print("Failed to connect device: \(error)")
                lastError = String(describing: error)
            }
        }
    }

    /// Pair a new device using Rust FFI
    ///
    /// This is the main entry point for pairing flow:
    /// 1. Rust adds device to memory
    /// 2. Rust attempts connection (WiFi + BLE)
    /// 3. On success: Rust saves to storage via FfiDeviceStorage
    /// 4. On failure: Rust removes from memory
    ///
    /// - Parameter device: Device information from QR code
    /// - Returns: true if pairing succeeded, false otherwise
    func pairDevice(_ device: DeviceDisplay) -> Bool {
        guard let manager = nearClipManager else { return false }

        let ffiDevice = FfiDeviceInfo(
            id: device.id,
            name: device.name,
            platform: platformFromString(device.platform),
            status: .disconnected
        )

        do {
            let success = try manager.pairDevice(device: ffiDevice)
            if success {
                print("ConnectionManager: Device paired successfully: \(device.name)")
                refreshDeviceLists()
            } else {
                print("ConnectionManager: Device pairing failed: \(device.name)")
            }
            return success
        } catch {
            print("ConnectionManager: Device pairing error: \(error)")
            return false
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

        // Perform FFI calls on background thread to avoid blocking Main Thread
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            // Get connected devices
            let ffiConnected = manager.getConnectedDevices()
            let connected = ffiConnected.map { device in
                DeviceDisplay(
                    id: device.id,
                    name: device.name,
                    platform: self.platformString(device.platform),
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
                    platform: self.platformString(device.platform),
                    isConnected: isConnected
                )
            }

            DispatchQueue.main.async {
                self.connectedDevices = connected
                self.pairedDevices = paired
                self.updateStatus()
            }
        }
    }

    // MARK: - Internal Callbacks (called from FFI)

    func handleDeviceConnected(_ device: FfiDeviceInfo) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Check if already connected via BLE
            let isBleConnected = self.bleManager?.isConnected(peripheralUuid: device.id) ?? false
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

            // Update paired devices status if already paired
            if let index = self.pairedDevices.firstIndex(where: { $0.id == device.id }) {
                self.pairedDevices[index] = DeviceDisplay(
                    id: device.id,
                    name: device.name,
                    platform: self.platformString(device.platform),
                    isConnected: true
                )
            } else {
                // Device connected but not in our paired list - this can happen with bidirectional pairing
                // The device will be added to paired list via refreshDeviceLists from FFI
                print("Connected device not in local paired list, will refresh from FFI: \(device.name)")
                self.refreshDeviceLists()
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
            let isBleConnected = self.bleManager?.isConnected(peripheralUuid: deviceId) ?? false

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

            // Storage removal is handled by Rust via FfiDeviceStorage

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

            // Remove from FFI manager (this will also remove from storage via FfiDeviceStorage)
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

    private var statusUpdateWorkItem: DispatchWorkItem?

    private func notifyStatusChange() {
        // Debounce UI updates to prevent Main Thread starvation during rapid state changes
        statusUpdateWorkItem?.cancel()

        let item = DispatchWorkItem { [weak self] in
            guard let self = self else { return }
            // Notify app delegate to update icon
            if let appDelegate = NSApp.delegate as? AppDelegate {
                appDelegate.updateStatusIcon(for: self.status)
            }
        }
        statusUpdateWorkItem = item

        // Wait 0.2s before updating UI (Reduced from 0.25s)
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2, execute: item)
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

    // MARK: - Device Removal

    /// Remove a paired device (sends unpair notification to remote device)
    /// Storage removal is handled by Rust via FfiDeviceStorage
    func removePairedDevice(_ deviceId: String) {
        // Unpair device via FFI (sends notification to remote device, removes from FFI and storage)
        do {
            try nearClipManager?.unpairDevice(deviceId: deviceId)
            print("ConnectionManager: Unpaired device via FFI: \(deviceId)")
        } catch {
            print("ConnectionManager: Failed to unpair device via FFI: \(deviceId), error: \(error)")
        }

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

    func onDeviceDiscovered(device: FfiDiscoveredDevice) {
        manager?.handleBleDeviceDiscovered(device)
    }

    func onDeviceLost(peripheralUuid: String) {
        manager?.handleBleDeviceLost(peripheralUuid)
    }
}

// MARK: - BleManagerDelegate

extension ConnectionManager: BleManagerDelegate {

    func bleManager(_ manager: BleManager, didDiscoverDevice peripheralUuid: String, deviceId: String?, publicKeyHash: String?, rssi: Int) {
        // Throttle discovery notifications to prevent flooding FFI and Main Thread
        let now = Date()
        var shouldSkip = false

        discoveryLock.lock()
        if let lastTime = lastDiscoveryNotification[peripheralUuid],
           now.timeIntervalSince(lastTime) < 5.0 {
            shouldSkip = true
        } else {
            lastDiscoveryNotification[peripheralUuid] = now
        }
        discoveryLock.unlock()

        if shouldSkip {
            return
        }

        // Only notify FFI layer if we have a device_id (from auto-discovery connection)
        guard let deviceId = deviceId, !deviceId.isEmpty else {
            return
        }

        // Perform FFI calls on background thread to avoid blocking BLE queue
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            NSLog("BLE: Discovered peripheral: \(peripheralUuid), deviceId: \(deviceId), rssi: \(rssi)")
            self.nearClipManager?.onBleDeviceDiscovered(
                peripheralUuid: peripheralUuid,
                deviceId: deviceId,
                publicKeyHash: publicKeyHash ?? "",
                rssi: Int32(rssi)
            )
            NSLog("ConnectionManager: Notified FFI layer of BLE device discovery: \(deviceId) -> \(peripheralUuid)")
        }
    }

    func bleManager(_ manager: BleManager, didLoseDevice peripheralUuid: String) {
        DispatchQueue.main.async { [weak self] in
            print("BLE: Lost peripheral: \(peripheralUuid)")
            // Rust layer will handle device lost logic
        }
    }

    func bleManager(_ manager: BleManager, didConnectDevice peripheralUuid: String, deviceId: String) {
        NSLog("ConnectionManager: BLE didConnectDevice - peripheralUuid: \(peripheralUuid), deviceId: \(deviceId)")

        // Perform FFI calls on background thread to avoid blocking Main Thread
        // FFI calls like onBleConnectionChanged may trigger read_characteristic which uses semaphore
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            // First, notify FFI layer about device discovery to establish device_id -> peripheral_uuid mapping
            // This is required before connection state change for BLE-only pairing to work
            self.nearClipManager?.onBleDeviceDiscovered(
                peripheralUuid: peripheralUuid,
                deviceId: deviceId,
                publicKeyHash: "",  // Will be read separately if needed
                rssi: 0
            )
            NSLog("ConnectionManager: Notified FFI layer of BLE device discovery: \(deviceId) -> \(peripheralUuid)")

            // Then notify FFI layer about BLE connection state change
            self.nearClipManager?.onBleConnectionChanged(deviceId: deviceId, connected: true)
            NSLog("ConnectionManager: Notified FFI layer of BLE connection: \(deviceId)")

            // Update UI on main thread
            DispatchQueue.main.async { [weak self] in
                guard let self = self else { return }

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
                    NSLog("ConnectionManager: Added BLE device to connectedDevices: \(deviceName) (\(deviceId))")
                } else {
                    // Device already connected via WiFi, update to .both
                    if let index = self.connectedDevices.firstIndex(where: { $0.id == deviceId }) {
                        self.connectedDevices[index].connectionType = .both
                        NSLog("ConnectionManager: Updated device to .both: \(deviceId)")
                    }
                }
            }
        }
    }

    func bleManager(_ manager: BleManager, didDisconnectDevice peripheralUuid: String, deviceId: String?) {
        print("BLE: Disconnected from peripheral: \(peripheralUuid), deviceId: \(deviceId ?? "nil")")

        guard let deviceId = deviceId else { return }

        // Perform FFI calls on background thread to avoid blocking Main Thread
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self = self else { return }

            // Notify FFI layer about BLE connection state change
            self.nearClipManager?.onBleConnectionChanged(deviceId: deviceId, connected: false)
            NSLog("ConnectionManager: Notified FFI layer of BLE disconnection: \(deviceId)")

            // Update UI on main thread
            DispatchQueue.main.async { [weak self] in
                guard let self = self else { return }

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
    }

    func bleManager(_ manager: BleManager, didReceiveData data: Data, fromPeripheral peripheralUuid: String) {
        print("BLE: Received \(data.count) bytes from peripheral: \(peripheralUuid)")

        // Forward BLE data to FFI layer for processing on background thread
        // Note: We need to map peripheralUuid to deviceId
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            self?.nearClipManager?.onBleDataReceived(deviceId: peripheralUuid, data: data)
            NSLog("ConnectionManager: Forwarded BLE data to FFI layer: \(data.count) bytes")
        }
    }

    func bleManager(_ manager: BleManager, didReceiveAck data: Data, fromPeripheral peripheralUuid: String, deviceId: String) {
        print("BLE: Received ACK from device: \(deviceId) (peripheral: \(peripheralUuid)), data: \(data.count) bytes")

        // Forward ACK to FFI layer for processing on background thread
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            self?.nearClipManager?.onBleAckReceived(deviceId: deviceId, data: data)
            NSLog("ConnectionManager: Forwarded BLE ACK to FFI layer: \(data.count) bytes from \(deviceId)")
        }
    }

    func bleManager(_ manager: BleManager, didFailWithError error: Error, forPeripheral peripheralUuid: String?) {
        print("BLE: Error for peripheral \(peripheralUuid ?? "unknown"): \(error.localizedDescription)")
        lastError = "BLE: \(error.localizedDescription)"
    }

    // MARK: - BLE Discovery Handlers (from FFI callbacks)

    func handleBleDeviceDiscovered(_ device: FfiDiscoveredDevice) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            print("BLE: FFI discovered device - peripheralUuid: \(device.peripheralUuid), name: \(device.deviceName ?? "nil")")

            // Check if this is a paired device and auto-connect
            if let publicKeyHash = device.publicKeyHash {
                // TODO: Match by public key hash with paired devices
            }

            // TEST: Auto-pair discovered device if not already paired
            // Only auto-pair if we haven't seen this device recently to avoid spamming

            // Resolve real Device ID from BleManager if possible
            // This is CRITICAL: Rust expects the real Device ID for pairing/connection,
            // not the peripheral UUID (which is random on some platforms)
            let realDeviceId = self.bleManager?.getDeviceId(for: device.peripheralUuid) ?? device.peripheralUuid
            let deviceName = device.deviceName ?? "Unknown Device"

            // Check if device is already connected via BLE or WiFi using real ID
            let isConnected = self.connectedDevices.contains(where: { $0.id == realDeviceId })
            let isPaired = self.pairedDevices.contains(where: { $0.id == realDeviceId })

            // Check if BLE is already connected for this device
            let isBleConnected = self.bleManager?.isConnected(peripheralUuid: device.peripheralUuid) == true

            if isPaired && !isConnected && !isBleConnected {
                // Already paired device discovered via BLE - auto-connect!
                // Throttle connection attempts to avoid connection storm
                let lastAttempt = self.lastAutoPairAttempt[realDeviceId]
                if let lastAttempt = lastAttempt, Date().timeIntervalSince(lastAttempt) < 5 {
                    return
                }

                print("BLE: Auto-connecting to paired device: \(realDeviceId) (UUID: \(device.peripheralUuid))")
                self.lastAutoPairAttempt[realDeviceId] = Date()

                // Connect via BLE using peripheral UUID
                self.bleManager?.connect(peripheralUuid: device.peripheralUuid)
            } else if !isPaired && !isConnected {
                // Check if we already have a pending or recent attempt for this device
                // This prevents the infinite loop of discovery -> pair -> connect -> discover -> pair...
                // Allow retry every 15 seconds
                let lastAttempt = self.lastAutoPairAttempt[realDeviceId]
                if let lastAttempt = lastAttempt, Date().timeIntervalSince(lastAttempt) < 15 {
                    return
                }

                print("TEST: Auto-pairing discovered device: \(realDeviceId) (UUID: \(device.peripheralUuid))")
                self.lastAutoPairAttempt[realDeviceId] = Date()

                let display = DeviceDisplay(
                    id: realDeviceId, // Use real ID!
                    name: deviceName,
                    platform: "Unknown",
                    isConnected: false,
                    connectionType: .ble
                )
                if self.pairDevice(display) {
                    print("TEST: Paired successfully, connecting to \(realDeviceId)...")
                    self.connectDevice(realDeviceId)
                }
            }
        }
    }

    func handleBleDeviceLost(_ peripheralUuid: String) {
        DispatchQueue.main.async { [weak self] in
            print("BLE: FFI device lost - peripheralUuid: \(peripheralUuid)")
        }
    }

    // MARK: - BLE Public API

    /// Sync clipboard content to a specific device via BLE
    func syncClipboardViaBle(_ content: Data, to deviceId: String) {
        guard let bleManager = bleManager else {
            print("BLE sync failed: BLE manager not available")
            return
        }

        let result = bleManager.writeData(peripheralUuid: deviceId, data: content)
        if result.isEmpty {
            print("BLE sync to \(deviceId): \(content.count) bytes sent")
        } else {
            print("BLE sync to \(deviceId) failed: \(result)")
        }
    }

    /// Check if a device is connected via BLE
    func isDeviceConnectedViaBle(_ deviceId: String) -> Bool {
        return bleManager?.isConnected(peripheralUuid: deviceId) ?? false
    }
}

// MARK: - FfiBleHardware Bridge

/// Bridge class that implements FfiBleHardware protocol and delegates to BleManager
final class BleHardwareBridge: FfiBleHardware {
    private weak var bleManager: BleManager?

    init(bleManager: BleManager?) {
        self.bleManager = bleManager
    }

    // ========== Scanning ==========

    func startScan() {
        bleManager?.startScanning()
    }

    func stopScan() {
        bleManager?.stopScanning()
    }

    // ========== Connection ==========

    func connect(peripheralUuid: String) {
        bleManager?.connect(peripheralUuid: peripheralUuid)
    }

    func disconnect(peripheralUuid: String) {
        bleManager?.disconnect(peripheralUuid: peripheralUuid)
    }

    // ========== GATT Operations ==========

    func readCharacteristic(peripheralUuid: String, charUuid: String) -> Data {
        guard let manager = bleManager else {
            print("readCharacteristic: BLE manager not available")
            return Data()
        }
        return manager.readCharacteristic(peripheralUuid: peripheralUuid, charUuid: charUuid)
    }

    func writeCharacteristic(peripheralUuid: String, charUuid: String, data: Data) -> String {
        guard let manager = bleManager else {
            return "BLE manager not available"
        }
        return manager.writeCharacteristic(peripheralUuid: peripheralUuid, charUuid: charUuid, data: data)
    }

    func subscribeCharacteristic(peripheralUuid: String, charUuid: String) -> String {
        guard let manager = bleManager else {
            return "BLE manager not available"
        }
        return manager.subscribeCharacteristic(peripheralUuid: peripheralUuid, charUuid: charUuid)
    }

    // ========== Advertising ==========

    func startAdvertising(serviceData: Data) {
        bleManager?.startAdvertising(serviceData: serviceData)
    }

    func stopAdvertising() {
        bleManager?.stopAdvertising()
    }

    // ========== Status Query ==========

    func isConnected(peripheralUuid: String) -> Bool {
        // peripheralUuid can be either a peripheral UUID or a device ID
        // Try device ID first, then fall back to peripheral UUID
        return bleManager?.isConnectedByDeviceId(peripheralUuid) ?? false
    }

    func getMtu(peripheralUuid: String) -> UInt32 {
        // Try to resolve device_id to peripheral_uuid first
        if let realPeripheralUuid = bleManager?.getPeripheralUuid(for: peripheralUuid) {
            return bleManager?.getMtu(peripheralUuid: realPeripheralUuid) ?? 20
        }
        return bleManager?.getMtu(peripheralUuid: peripheralUuid) ?? 20
    }
}
