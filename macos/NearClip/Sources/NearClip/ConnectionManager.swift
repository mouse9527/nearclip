import Foundation
import Combine
import AppKit

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
    let id: String
    let name: String
    let platform: String
    let isConnected: Bool
    let lastSeen: Date?
    var isPaused: Bool

    init(id: String, name: String, platform: String, isConnected: Bool, lastSeen: Date? = nil, isPaused: Bool = false) {
        self.id = id
        self.name = name
        self.platform = platform
        self.isConnected = isConnected
        self.lastSeen = lastSeen
        self.isPaused = isPaused
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

    private(set) var nearClipManager: FfiNearClipManager?
    private var isRunning = false
    private var syncInProgress = false
    private var previousStatus: ConnectionStatus = .disconnected

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

    /// Check if we can add more devices
    var canAddMoreDevices: Bool {
        pairedDevices.count < Self.maxPairedDevices
    }

    private init() {
        // Load paired devices from Keychain
        loadPairedDevicesFromKeychain()
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

            // Create config
            let config = FfiNearClipConfig(
                deviceName: Host.current().localizedName ?? "Mac",
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

            isRunning = true
            lastError = nil

            // Refresh device lists
            refreshDeviceLists()

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

        do {
            try nearClipManager?.syncClipboard(content: content)
            lastSyncTime = Date()
            lastError = nil
            print("Clipboard synced to \(activeDevices.count) device(s): \(content.count) bytes")

            // Clear syncing state after a brief delay
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
                self?.setSyncing(false)
            }
        } catch {
            let errorMessage = String(describing: error)
            print("Failed to sync clipboard: \(errorMessage)")
            lastError = errorMessage
            setSyncing(false)
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

            let display = DeviceDisplay(
                id: device.id,
                name: device.name,
                platform: self.platformString(device.platform),
                isConnected: true,
                lastSeen: Date()
            )

            // Update connected devices
            self.connectedDevices.removeAll { $0.id == device.id }
            self.connectedDevices.append(display)

            // Update paired devices status
            if let index = self.pairedDevices.firstIndex(where: { $0.id == device.id }) {
                self.pairedDevices[index] = DeviceDisplay(
                    id: device.id,
                    name: device.name,
                    platform: self.platformString(device.platform),
                    isConnected: true
                )
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

            // Remove from connected devices
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

            self.updateStatus()
            print("Device disconnected: \(deviceId)")
        }
    }

    func handleClipboardReceived(_ content: Data, from deviceId: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Set syncing state
            self.setSyncing(true)

            // Write to local clipboard (markAsRemote prevents sync loop)
            let success = ClipboardWriter.shared.write(content)

            if success {
                self.lastSyncTime = Date()
                self.lastError = nil
                print("Received and wrote clipboard from \(deviceId): \(content.count) bytes")

                // Show notification
                let deviceName = self.connectedDevices.first { $0.id == deviceId }?.name ?? deviceId
                let contentPreview = String(data: content, encoding: .utf8)
                NotificationManager.shared.showSyncSuccessNotification(
                    fromDevice: deviceName,
                    contentPreview: contentPreview
                )
            } else {
                self.lastError = "Failed to write clipboard"
                print("Failed to write clipboard from \(deviceId)")
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
        syncClipboard(content)
        pendingContent = nil
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
    private func loadPairedDevicesFromKeychain() {
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

    /// Remove a paired device (removes from Keychain and updates FFI)
    func removePairedDevice(_ deviceId: String) {
        // Remove from Keychain
        removePairedDeviceFromKeychain(deviceId)

        // Remove from paused list
        var paused = pausedDeviceIds
        paused.remove(deviceId)
        pausedDeviceIds = paused

        // Update local list
        pairedDevices.removeAll { $0.id == deviceId }
        connectedDevices.removeAll { $0.id == deviceId }

        // Update FFI if available
        nearClipManager?.removePairedDevice(deviceId: deviceId)

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

    func onClipboardReceived(content: Data, fromDevice: String) {
        manager?.handleClipboardReceived(content, from: fromDevice)
    }

    func onSyncError(errorMessage: String) {
        manager?.handleSyncError(errorMessage)
    }
}
