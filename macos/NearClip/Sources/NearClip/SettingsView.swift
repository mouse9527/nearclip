import SwiftUI
import ServiceManagement

/// Settings view with multiple tabs
struct SettingsView: View {
    @ObservedObject var connectionManager: ConnectionManager
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        TabView {
            GeneralSettingsTab()
                .tabItem {
                    Label("General", systemImage: "gear")
                }

            SyncSettingsTab(connectionManager: connectionManager)
                .tabItem {
                    Label("Sync", systemImage: "arrow.triangle.2.circlepath")
                }

            DevicesSettingsTab(connectionManager: connectionManager)
                .tabItem {
                    Label("Devices", systemImage: "laptopcomputer.and.iphone")
                }

            AboutTab()
                .tabItem {
                    Label("About", systemImage: "info.circle")
                }
        }
        .frame(width: 450, height: 300)
    }
}

// MARK: - General Settings Tab

struct GeneralSettingsTab: View {
    @State private var launchAtLogin = AppDelegate.isLaunchAtLoginEnabled

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Startup")
                .font(.headline)

            Toggle("Launch at Login", isOn: $launchAtLogin)
                .onChange(of: launchAtLogin) { newValue in
                    AppDelegate.setLaunchAtLogin(newValue)
                }

            Text("NearClip will start automatically when you log in")
                .font(.caption)
                .foregroundColor(.secondary)

            Spacer()
        }
        .padding()
    }
}

// MARK: - Sync Settings Tab

struct SyncSettingsTab: View {
    @ObservedObject var connectionManager: ConnectionManager

    @AppStorage("wifiEnabled") private var wifiEnabled = true
    @AppStorage("bleEnabled") private var bleEnabled = true
    @AppStorage("autoConnect") private var autoConnect = true

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            // Connection Methods
            VStack(alignment: .leading, spacing: 8) {
                Text("Connection Methods")
                    .font(.headline)

                Toggle("WiFi Sync", isOn: $wifiEnabled)
                Text("Sync clipboard over local network")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .padding(.leading, 20)

                Toggle("Bluetooth Sync", isOn: $bleEnabled)
                Text("Sync clipboard over Bluetooth Low Energy")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .padding(.leading, 20)
            }

            Divider()

            // Behavior
            VStack(alignment: .leading, spacing: 8) {
                Text("Behavior")
                    .font(.headline)

                Toggle("Auto Connect", isOn: $autoConnect)
                Text("Automatically connect to paired devices")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .padding(.leading, 20)
            }

            Spacer()

            HStack {
                Button("Restart Service") {
                    connectionManager.restart()
                }
                Text("Apply changes by restarting the sync service")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
    }
}

// MARK: - Devices Settings Tab

struct DevicesSettingsTab: View {
    @ObservedObject var connectionManager: ConnectionManager
    @State private var selectedDeviceId: String?
    @State private var showDeleteConfirmation = false

    private var selectedDevice: DeviceDisplay? {
        connectionManager.pairedDevices.first { $0.id == selectedDeviceId }
    }

    var body: some View {
        VStack(spacing: 0) {
            // Header
            HStack {
                Text("Paired Devices")
                    .font(.headline)
                Spacer()
                Text("\(connectionManager.pairedDevices.count) devices")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            .padding()

            Divider()

            // Device list
            if connectionManager.pairedDevices.isEmpty {
                VStack(spacing: 8) {
                    Image(systemName: "laptopcomputer.and.iphone")
                        .font(.largeTitle)
                        .foregroundColor(.secondary)
                    Text("No paired devices")
                        .foregroundColor(.secondary)
                    Text("Click \"Add Device\" from the menu to pair")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .frame(maxHeight: .infinity)
            } else {
                List(connectionManager.pairedDevices, selection: $selectedDeviceId) { device in
                    DeviceSettingsRow(
                        device: device,
                        isConnected: connectionManager.connectedDevices.contains { $0.id == device.id }
                    )
                    .tag(device.id)
                }
                .listStyle(.inset)
            }

            Divider()

            // Footer with delete button
            HStack {
                Button(action: { showDeleteConfirmation = true }) {
                    Image(systemName: "minus")
                }
                .disabled(selectedDeviceId == nil)
                .help("Remove selected device")

                Spacer()
            }
            .padding(8)
        }
        .alert("Remove Device", isPresented: $showDeleteConfirmation) {
            Button("Cancel", role: .cancel) {}
            Button("Remove", role: .destructive) {
                if let device = selectedDevice {
                    removeDevice(device)
                }
            }
        } message: {
            if let device = selectedDevice {
                Text("Are you sure you want to remove \"\(device.name)\" from paired devices?")
            }
        }
    }

    private func removeDevice(_ device: DeviceDisplay) {
        // Remove from Keychain and FFI
        connectionManager.removePairedDevice(device.id)
        selectedDeviceId = nil
    }
}

struct DeviceSettingsRow: View {
    let device: DeviceDisplay
    let isConnected: Bool

    var body: some View {
        HStack {
            Image(systemName: platformIcon)
                .font(.title2)
                .foregroundColor(.secondary)
                .frame(width: 32)

            VStack(alignment: .leading, spacing: 2) {
                Text(device.name)
                    .fontWeight(.medium)

                HStack(spacing: 4) {
                    Text(device.platform)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if isConnected {
                        Circle()
                            .fill(Color.green)
                            .frame(width: 6, height: 6)
                        Text("Connected")
                            .font(.caption)
                            .foregroundColor(.green)
                    }
                }
            }

            Spacer()

            Text(device.id.prefix(8) + "...")
                .font(.system(.caption, design: .monospaced))
                .foregroundColor(.secondary)
        }
        .padding(.vertical, 4)
    }

    private var platformIcon: String {
        switch device.platform.lowercased() {
        case "macos":
            return "laptopcomputer"
        case "android":
            return "phone"
        case "ios":
            return "iphone"
        default:
            return "desktopcomputer"
        }
    }
}

// MARK: - About Tab

struct AboutTab: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "link.circle.fill")
                .font(.system(size: 64))
                .foregroundColor(.accentColor)

            Text("NearClip")
                .font(.title)
                .fontWeight(.bold)

            Text("Version 1.0.0")
                .font(.caption)
                .foregroundColor(.secondary)

            Text("Cross-device clipboard sync")
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            HStack(spacing: 16) {
                Link("GitHub", destination: URL(string: "https://github.com")!)
                    .font(.caption)

                Text("|")
                    .foregroundColor(.secondary)

                Link("Report Issue", destination: URL(string: "https://github.com")!)
                    .font(.caption)
            }
        }
        .padding()
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

// MARK: - Settings Window Controller

final class SettingsWindowController: NSObject {
    private var window: NSWindow?
    private var hostingView: NSHostingView<SettingsView>?

    func showWindow(connectionManager: ConnectionManager) {
        if let existingWindow = window {
            existingWindow.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
            return
        }

        let settingsView = SettingsView(connectionManager: connectionManager)
        let hostingView = NSHostingView(rootView: settingsView)
        self.hostingView = hostingView

        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 450, height: 300),
            styleMask: [.titled, .closable],
            backing: .buffered,
            defer: false
        )

        window.title = "NearClip Settings"
        window.contentView = hostingView
        window.center()
        window.isReleasedWhenClosed = false
        window.delegate = self

        self.window = window

        window.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    func closeWindow() {
        window?.close()
        window = nil
        hostingView = nil
    }
}

extension SettingsWindowController: NSWindowDelegate {
    func windowWillClose(_ notification: Notification) {
        window = nil
        hostingView = nil
    }
}

// MARK: - Preview

#if DEBUG
struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView(connectionManager: ConnectionManager.shared)
    }
}
#endif
