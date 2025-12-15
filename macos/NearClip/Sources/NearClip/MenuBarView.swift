import SwiftUI

/// Main view displayed in the menubar popover
struct MenuBarView: View {
    @ObservedObject var connectionManager: ConnectionManager

    let onQuit: () -> Void
    let onSettings: () -> Void
    let onAddDevice: () -> Void

    var body: some View {
        VStack(spacing: 0) {
            // Header with status
            headerSection

            Divider()

            // Error banner if present
            if let error = connectionManager.lastError {
                errorBanner(error)
                Divider()
            }

            // Connected devices
            if !connectionManager.connectedDevices.isEmpty {
                connectedDevicesSection
                Divider()
            }

            // Paired devices (not connected)
            let disconnectedPaired = connectionManager.pairedDevices.filter { paired in
                !connectionManager.connectedDevices.contains { $0.id == paired.id }
            }
            if !disconnectedPaired.isEmpty {
                pairedDevicesSection(devices: disconnectedPaired)
                Divider()
            }

            // Last sync info
            if let lastSync = connectionManager.lastSyncTime {
                lastSyncSection(lastSync)
                Divider()
            }

            // Actions
            actionsSection
        }
        .frame(width: 280)
        .background(Color(NSColor.windowBackgroundColor))
    }

    // MARK: - Header

    private var headerSection: some View {
        HStack {
            statusIcon
            VStack(alignment: .leading, spacing: 2) {
                Text("NearClip")
                    .font(.headline)
                Text(connectionManager.status.displayText)
                    .font(.caption)
                    .foregroundColor(statusTextColor)
            }
            Spacer()

            // Service control button
            serviceControlButton
        }
        .padding()
    }

    private var statusIcon: some View {
        ZStack {
            Circle()
                .fill(statusColor.opacity(0.2))
                .frame(width: 36, height: 36)

            Image(systemName: connectionManager.status.symbolName)
                .font(.system(size: 18))
                .foregroundColor(statusColor)
                .rotationEffect(shouldAnimateIcon ? .degrees(360) : .degrees(0))
                .animation(
                    shouldAnimateIcon
                        ? Animation.linear(duration: 1.0).repeatForever(autoreverses: false)
                        : .default,
                    value: shouldAnimateIcon
                )
        }
    }

    private var shouldAnimateIcon: Bool {
        connectionManager.status == .connecting || connectionManager.status.isSyncing
    }

    private var statusColor: Color {
        switch connectionManager.status {
        case .disconnected:
            return .gray
        case .connecting:
            return .orange
        case .connected:
            return .green
        case .syncing:
            return .blue
        case .error:
            return .red
        }
    }

    private var statusTextColor: Color {
        if connectionManager.status.hasError {
            return .red
        }
        return .secondary
    }

    private var serviceControlButton: some View {
        Button(action: toggleService) {
            Image(systemName: connectionManager.isServiceRunning ? "stop.circle" : "play.circle")
                .font(.system(size: 18))
                .foregroundColor(connectionManager.isServiceRunning ? .red : .green)
        }
        .buttonStyle(.plain)
        .help(connectionManager.isServiceRunning ? "Stop Service" : "Start Service")
    }

    private func toggleService() {
        if connectionManager.isServiceRunning {
            connectionManager.stop()
        } else {
            connectionManager.start()
        }
    }

    // MARK: - Error Banner

    private func errorBanner(_ error: String) -> some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.red)

            Text(error)
                .font(.caption)
                .lineLimit(2)

            Spacer()

            Button("Retry") {
                connectionManager.restart()
            }
            .buttonStyle(.plain)
            .font(.caption)
            .foregroundColor(.accentColor)
        }
        .padding(.horizontal)
        .padding(.vertical, 8)
        .background(Color.red.opacity(0.1))
    }

    // MARK: - Connected Devices

    private var connectedDevicesSection: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack {
                Text("Connected")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Spacer()

                Text("\(connectionManager.connectedDevices.count)")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(Color.green.opacity(0.2))
                    .cornerRadius(4)
            }
            .padding(.horizontal)
            .padding(.top, 8)
            .padding(.bottom, 4)

            ForEach(connectionManager.connectedDevices) { device in
                DeviceRow(device: device, isConnected: true) {
                    connectionManager.disconnectDevice(device.id)
                }
            }
        }
    }

    // MARK: - Paired Devices

    private func pairedDevicesSection(devices: [DeviceDisplay]) -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("Paired Devices")
                .font(.caption)
                .foregroundColor(.secondary)
                .padding(.horizontal)
                .padding(.top, 8)
                .padding(.bottom, 4)

            ForEach(devices) { device in
                DeviceRow(device: device, isConnected: false) {
                    connectionManager.connectDevice(device.id)
                }
            }
        }
    }

    // MARK: - Last Sync

    private func lastSyncSection(_ date: Date) -> some View {
        HStack {
            Image(systemName: "clock")
                .font(.caption)
                .foregroundColor(.secondary)

            Text("Last sync: \(relativeTimeString(date))")
                .font(.caption)
                .foregroundColor(.secondary)

            Spacer()
        }
        .padding(.horizontal)
        .padding(.vertical, 8)
    }

    private func relativeTimeString(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: date, relativeTo: Date())
    }

    // MARK: - Actions

    private var actionsSection: some View {
        VStack(spacing: 0) {
            MenuButton(title: "Add Device", systemImage: "plus.circle") {
                onAddDevice()
            }

            MenuButton(title: "Settings", systemImage: "gearshape") {
                onSettings()
            }

            #if DEBUG
            Divider()
                .padding(.vertical, 4)

            testSection
            #endif

            Divider()
                .padding(.vertical, 4)

            MenuButton(title: "Quit NearClip", systemImage: "power") {
                onQuit()
            }
        }
        .padding(.vertical, 4)
    }

    // MARK: - Debug Test Section

    #if DEBUG
    @State private var testClipboardText = "Test clipboard from NearClip"

    private var testSection: some View {
        VStack(spacing: 0) {
            Text("Debug")
                .font(.caption)
                .foregroundColor(.secondary)
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
                .padding(.bottom, 4)

            MenuButton(title: "Simulate Receive", systemImage: "arrow.down.doc") {
                connectionManager.simulateReceiveClipboard(testClipboardText)
            }
        }
    }
    #endif
}

// MARK: - Device Row

struct DeviceRow: View {
    let device: DeviceDisplay
    let isConnected: Bool
    let action: () -> Void

    @State private var isHovering = false

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: platformIcon)
                    .font(.system(size: 16))
                    .foregroundColor(.secondary)
                    .frame(width: 24)

                VStack(alignment: .leading, spacing: 1) {
                    Text(device.name)
                        .font(.system(size: 13))

                    HStack(spacing: 4) {
                        Text(device.platform)
                            .font(.caption2)
                            .foregroundColor(.secondary)

                        if let lastSeen = device.lastSeen, isConnected {
                            Text("·")
                                .foregroundColor(.secondary)
                            Text(relativeTime(lastSeen))
                                .font(.caption2)
                                .foregroundColor(.secondary)
                        }
                    }
                }

                Spacer()

                if isConnected {
                    Circle()
                        .fill(Color.green)
                        .frame(width: 8, height: 8)
                } else {
                    Image(systemName: "arrow.right.circle")
                        .font(.system(size: 14))
                        .foregroundColor(.secondary)
                }
            }
            .padding(.horizontal)
            .padding(.vertical, 8)
            .background(isHovering ? Color.accentColor.opacity(0.1) : Color.clear)
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .onHover { hovering in
            isHovering = hovering
        }
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

    private func relativeTime(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

// MARK: - Menu Button

struct MenuButton: View {
    let title: String
    let systemImage: String
    let action: () -> Void

    @State private var isHovering = false

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: systemImage)
                    .font(.system(size: 14))
                    .frame(width: 24)
                Text(title)
                    .font(.system(size: 13))
                Spacer()
            }
            .padding(.horizontal)
            .padding(.vertical, 6)
            .background(isHovering ? Color.accentColor.opacity(0.1) : Color.clear)
            .cornerRadius(4)
        }
        .buttonStyle(.plain)
        .padding(.horizontal, 4)
        .onHover { hovering in
            isHovering = hovering
        }
    }
}

// MARK: - Preview

#if DEBUG
struct MenuBarView_Previews: PreviewProvider {
    static var previews: some View {
        MenuBarView(
            connectionManager: ConnectionManager.shared,
            onQuit: {},
            onSettings: {},
            onAddDevice: {}
        )
        .frame(height: 400)
    }
}
#endif
