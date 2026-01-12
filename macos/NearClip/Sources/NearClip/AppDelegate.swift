import AppKit
import SwiftUI
import ServiceManagement

/// AppDelegate manages the menubar status item and application lifecycle
final class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem?
    private var popover: NSPopover?
    private var connectionManager: ConnectionManager?
    private var clipboardMonitor: ClipboardMonitor?
    private var networkMonitor: NetworkMonitor?
    private var animationTimer: Timer?
    private var pairingWindowController: PairingWindowController?
    private var settingsWindowController: SettingsWindowController?

    func applicationDidFinishLaunching(_ notification: Notification) {
        setupStatusItem()
        setupPopover()
        setupConnectionManager()
        setupClipboardMonitor()
        setupNotifications()
        setupNetworkMonitor()

        // Hide dock icon (backup for Info.plist LSUIElement)
        NSApp.setActivationPolicy(.accessory)

        // Auto-start the service and clipboard monitoring
        connectionManager?.start()
        clipboardMonitor?.startMonitoring()
        networkMonitor?.startMonitoring()
    }

    func applicationWillTerminate(_ notification: Notification) {
        // Stop clipboard monitoring
        clipboardMonitor?.stopMonitoring()

        // Stop network monitoring
        networkMonitor?.stopMonitoring()

        // Stop the service before quitting
        connectionManager?.stop()
        animationTimer?.invalidate()
    }

    // MARK: - Setup

    private func setupStatusItem() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem?.button {
            button.image = NSImage(systemSymbolName: "link.badge.plus", accessibilityDescription: "NearClip")
            button.action = #selector(togglePopover)
            button.target = self
        }
    }

    private func setupPopover() {
        popover = NSPopover()
        popover?.contentSize = NSSize(width: 280, height: 360)
        popover?.behavior = .transient
        popover?.animates = true

        let menuView = MenuBarView(
            connectionManager: ConnectionManager.shared,
            onQuit: { [weak self] in self?.quitApp() },
            onSettings: { [weak self] in self?.openSettings() },
            onAddDevice: { [weak self] in self?.addDevice() }
        )
        popover?.contentViewController = NSHostingController(rootView: menuView)
    }

    private func setupConnectionManager() {
        connectionManager = ConnectionManager.shared
    }

    private func setupClipboardMonitor() {
        clipboardMonitor = ClipboardMonitor.shared

        // Configure clipboard monitor to sync via ConnectionManager
        clipboardMonitor?.configure(with: ConnectionManager.shared)
    }

    private func setupNotifications() {
        // Set default value for sync notifications if not already set
        if UserDefaults.standard.object(forKey: "syncNotificationsEnabled") == nil {
            UserDefaults.standard.set(true, forKey: "syncNotificationsEnabled")
        }

        // Initialize notification manager (requests authorization)
        let notificationManager = NotificationManager.shared

        // Set up retry strategy handlers for sync failure notifications
        notificationManager.onRetryRequested = { [weak self] in
            print("AppDelegate: Retry sync requested from notification")
            self?.connectionManager?.executeContinueRetryStrategy()
        }

        notificationManager.onDiscardRequested = { [weak self] in
            print("AppDelegate: Discard sync requested from notification")
            self?.connectionManager?.executeDiscardStrategy()
        }

        notificationManager.onWaitForDeviceRequested = { [weak self] in
            print("AppDelegate: Wait for device requested from notification")
            self?.connectionManager?.executeWaitForDeviceStrategy()
        }

        // Listen for add device request from settings
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(handleAddDeviceRequest),
            name: .requestAddDevice,
            object: nil
        )
    }

    @objc private func handleAddDeviceRequest() {
        addDevice()
    }

    private func setupNetworkMonitor() {
        networkMonitor = NetworkMonitor.shared

        // Handle network loss - switch to BLE for paired devices
        networkMonitor?.onNetworkLost = { [weak self] in
            print("AppDelegate: Network lost, attempting BLE fallback")
            self?.connectionManager?.handleNetworkLost()
        }

        // Handle network recovery - restart service to reconnect
        networkMonitor?.onNetworkRestored = { [weak self] in
            print("AppDelegate: Network restored, attempting to reconnect")
            self?.connectionManager?.restart()
        }

        // Handle reconnection failure after max attempts
        networkMonitor?.onReconnectFailed = { [weak self] in
            print("AppDelegate: Reconnection failed after multiple attempts")

            // Show notification to user
            NotificationManager.shared.showSyncFailureNotification(
                reason: "Unable to reconnect after network recovery. Please check your network connection."
            )
        }
    }

    // MARK: - Actions

    @objc private func togglePopover() {
        print("AppDelegate: Status item clicked")
        guard let button = statusItem?.button, let popover = popover else { return }

        if popover.isShown {
            popover.performClose(nil)
        } else {
            popover.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)

            // Activate app to ensure popover receives focus
            NSApp.activate(ignoringOtherApps: true)
        }
    }

    private func quitApp() {
        clipboardMonitor?.stopMonitoring()
        connectionManager?.stop()
        NSApp.terminate(nil)
    }

    private func openSettings() {
        popover?.performClose(nil)

        // Create settings window controller if needed
        if settingsWindowController == nil {
            settingsWindowController = SettingsWindowController()
        }

        // Show settings window
        settingsWindowController?.showWindow(connectionManager: ConnectionManager.shared)
    }

    private func addDevice() {
        popover?.performClose(nil)

        // Create pairing window controller if needed
        if pairingWindowController == nil {
            pairingWindowController = PairingWindowController()
        }

        // Show pairing window
        pairingWindowController?.showWindow(connectionManager: ConnectionManager.shared)
    }

    // MARK: - Status Item Updates

    /// Update the menubar icon based on connection status
    func updateStatusIcon(for status: ConnectionStatus) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self, let button = self.statusItem?.button else { return }

            // Stop any existing animation
            self.stopIconAnimation()

            // Update icon
            button.image = NSImage(
                systemSymbolName: status.symbolName,
                accessibilityDescription: status.accessibilityDescription
            )

            // Apply tint color based on status
            button.contentTintColor = self.tintColor(for: status)

            // Start animation for connecting/syncing states
            if status == .connecting || status.isSyncing {
                self.startIconAnimation()
            }
        }
    }

    /// Legacy method for backward compatibility
    func updateStatusIcon(connected: Bool) {
        let status: ConnectionStatus = connected ? .connected(deviceCount: 1) : .disconnected
        updateStatusIcon(for: status)
    }

    private func tintColor(for status: ConnectionStatus) -> NSColor? {
        switch status {
        case .disconnected:
            return .secondaryLabelColor
        case .connecting:
            return .systemOrange
        case .connected:
            return .systemGreen
        case .syncing:
            return .systemBlue
        case .error:
            return .systemRed
        }
    }

    // MARK: - Icon Animation

    private func startIconAnimation() {
        // Pulse animation for connecting/syncing
        animationTimer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { [weak self] _ in
            guard let button = self?.statusItem?.button else { return }

            NSAnimationContext.runAnimationGroup { context in
                context.duration = 0.25
                button.animator().alphaValue = 0.5
            } completionHandler: {
                NSAnimationContext.runAnimationGroup { context in
                    context.duration = 0.25
                    button.animator().alphaValue = 1.0
                }
            }
        }
    }

    private func stopIconAnimation() {
        animationTimer?.invalidate()
        animationTimer = nil

        // Reset alpha
        if let button = statusItem?.button {
            button.alphaValue = 1.0
        }
    }
}

// MARK: - Launch at Login

extension AppDelegate {
    /// Check if launch at login is enabled
    static var isLaunchAtLoginEnabled: Bool {
        if #available(macOS 13.0, *) {
            return SMAppService.mainApp.status == .enabled
        } else {
            // For macOS 12, we'd need to use the older API or helper app approach
            return UserDefaults.standard.bool(forKey: "launchAtLogin")
        }
    }

    /// Toggle launch at login setting
    static func setLaunchAtLogin(_ enabled: Bool) {
        if #available(macOS 13.0, *) {
            do {
                if enabled {
                    try SMAppService.mainApp.register()
                } else {
                    try SMAppService.mainApp.unregister()
                }
            } catch {
                print("Failed to set launch at login: \(error)")
            }
        } else {
            // For macOS 12, store preference (actual implementation would need helper app)
            UserDefaults.standard.set(enabled, forKey: "launchAtLogin")
        }
    }
}
