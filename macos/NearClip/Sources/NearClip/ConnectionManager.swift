import Foundation
import Combine
import AppKit

/// Device information for display in UI
struct DeviceDisplay: Identifiable, Equatable {
    let id: String
    let name: String
    let platform: String
    let isConnected: Bool
    let lastSeen: Date?

    init(id: String, name: String, platform: String, isConnected: Bool, lastSeen: Date? = nil) {
        self.id = id
        self.name = name
        self.platform = platform
        self.isConnected = isConnected
        self.lastSeen = lastSeen
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

    @Published private(set) var status: ConnectionStatus = .disconnected
    @Published private(set) var connectedDevices: [DeviceDisplay] = []
    @Published private(set) var pairedDevices: [DeviceDisplay] = []
    @Published private(set) var lastError: String?
    @Published private(set) var lastSyncTime: Date?

    private(set) var nearClipManager: FfiNearClipManager?
    private var isRunning = false
    private var syncInProgress = false
    private var previousStatus: ConnectionStatus = .disconnected

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

        guard !connectedDevices.isEmpty else {
            print("Cannot sync: no connected devices")
            return
        }

        // Set syncing state
        setSyncing(true)

        do {
            try nearClipManager?.syncClipboard(content: content)
            lastSyncTime = Date()
            lastError = nil
            print("Clipboard synced: \(content.count) bytes")

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
        pairedDevices = storedDevices.map { $0.toDeviceDisplay() }
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
    func addPairedDevice(_ device: DeviceDisplay) {
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
    }

    /// Remove a paired device (removes from Keychain and updates FFI)
    func removePairedDevice(_ deviceId: String) {
        // Remove from Keychain
        removePairedDeviceFromKeychain(deviceId)

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
