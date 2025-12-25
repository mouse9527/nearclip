import SwiftUI
import ServiceManagement

// MARK: - Settings Navigation

enum SettingsSection: String, CaseIterable, Identifiable {
    case general = "General"
    case sync = "Sync"
    case devices = "Devices"
    case history = "History"
    case debug = "Debug"
    case about = "About"

    var id: String { rawValue }

    var icon: String {
        switch self {
        case .general: return "gear"
        case .sync: return "arrow.triangle.2.circlepath"
        case .devices: return "laptopcomputer.and.iphone"
        case .history: return "clock.arrow.circlepath"
        case .debug: return "ant"
        case .about: return "info.circle"
        }
    }

    var localizedName: String {
        switch self {
        case .general: return "通用"
        case .sync: return "同步"
        case .devices: return "设备"
        case .history: return "历史"
        case .debug: return "调试"
        case .about: return "关于"
        }
    }
}

/// Settings view with sidebar navigation
struct SettingsView: View {
    @ObservedObject var connectionManager: ConnectionManager
    @Environment(\.dismiss) private var dismiss
    @State private var selectedSection: SettingsSection = .general

    var body: some View {
        HSplitView {
            // Sidebar
            VStack(spacing: 0) {
                ForEach(SettingsSection.allCases) { section in
                    SidebarItem(
                        section: section,
                        isSelected: selectedSection == section
                    ) {
                        selectedSection = section
                    }
                }
                Spacer()
            }
            .frame(width: 140)
            .background(Color(NSColor.controlBackgroundColor))

            // Content
            Group {
                switch selectedSection {
                case .general:
                    GeneralSettingsTab()
                case .sync:
                    SyncSettingsTab(connectionManager: connectionManager)
                case .devices:
                    DevicesSettingsTab(connectionManager: connectionManager)
                case .history:
                    HistorySettingsTab()
                case .debug:
                    DebugSettingsTab(connectionManager: connectionManager)
                case .about:
                    AboutTab()
                }
            }
            .frame(minWidth: 360)
        }
        .frame(width: 520, height: 400)
    }
}

struct SidebarItem: View {
    let section: SettingsSection
    let isSelected: Bool
    let action: () -> Void

    @State private var isHovering = false

    var body: some View {
        Button(action: action) {
            HStack(spacing: 8) {
                Image(systemName: section.icon)
                    .font(.system(size: 14))
                    .frame(width: 20)
                Text(section.localizedName)
                    .font(.system(size: 13))
                Spacer()
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 8)
            .background(
                RoundedRectangle(cornerRadius: 6)
                    .fill(isSelected ? Color.accentColor.opacity(0.2) : (isHovering ? Color.gray.opacity(0.1) : Color.clear))
            )
            .foregroundColor(isSelected ? .accentColor : .primary)
        }
        .buttonStyle(.plain)
        .padding(.horizontal, 8)
        .padding(.top, 4)
        .onHover { hovering in
            isHovering = hovering
        }
    }
}

// MARK: - General Settings Tab

struct GeneralSettingsTab: View {
    @State private var launchAtLogin = AppDelegate.isLaunchAtLoginEnabled
    @AppStorage("syncNotificationsEnabled") private var syncNotificationsEnabled = true

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            // Startup section
            Text("Startup")
                .font(.headline)

            Toggle("Launch at Login", isOn: $launchAtLogin)
                .onChange(of: launchAtLogin) { newValue in
                    AppDelegate.setLaunchAtLogin(newValue)
                }

            Text("NearClip will start automatically when you log in")
                .font(.caption)
                .foregroundColor(.secondary)

            Divider()

            // Notifications section
            Text("Notifications")
                .font(.headline)

            Toggle("Show Sync Notifications", isOn: $syncNotificationsEnabled)

            Text("Display a notification when clipboard is synced from another device")
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
    @AppStorage("defaultRetryStrategy") private var defaultRetryStrategy = SyncRetryStrategy.waitForDevice.rawValue

    private var selectedStrategy: Binding<SyncRetryStrategy> {
        Binding(
            get: { SyncRetryStrategy(rawValue: defaultRetryStrategy) ?? .waitForDevice },
            set: { defaultRetryStrategy = $0.rawValue }
        )
    }

    var body: some View {
        ScrollView {
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

                Divider()

                // Retry Strategy
                VStack(alignment: .leading, spacing: 8) {
                    Text("On Sync Failure")
                        .font(.headline)

                    Picker("Default Action", selection: selectedStrategy) {
                        ForEach(SyncRetryStrategy.allCases, id: \.self) { strategy in
                            Text(strategy.displayName).tag(strategy)
                        }
                    }
                    .pickerStyle(.menu)
                    .frame(width: 200)

                    Text(selectedStrategy.wrappedValue.description)
                        .font(.caption)
                        .foregroundColor(.secondary)
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
                Text("\(connectionManager.pairedDevices.count)/\(ConnectionManager.maxPairedDevices) devices")
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
                        isConnected: connectionManager.connectedDevices.contains { $0.id == device.id },
                        onTogglePause: {
                            if device.isPaused {
                                connectionManager.resumeDevice(device.id)
                            } else {
                                connectionManager.pauseDevice(device.id)
                            }
                        }
                    )
                    .tag(device.id)
                }
                .listStyle(.inset)
            }

            Divider()

            // Footer with actions
            HStack {
                Button(action: { showDeleteConfirmation = true }) {
                    Image(systemName: "minus")
                }
                .disabled(selectedDeviceId == nil)
                .help("Remove selected device")

                Button(action: requestAddDevice) {
                    Image(systemName: "plus")
                }
                .disabled(!connectionManager.canAddMoreDevices)
                .help(connectionManager.canAddMoreDevices
                    ? "Add a new device"
                    : "Maximum \(ConnectionManager.maxPairedDevices) devices reached")

                Spacer()

                Button(action: refreshDevices) {
                    Image(systemName: "arrow.clockwise")
                }
                .help("Refresh device list and try to reconnect")
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

    private func refreshDevices() {
        // Reload paired devices from Keychain
        connectionManager.loadPairedDevicesFromKeychain()

        // Try to connect to all paired devices
        _ = connectionManager.nearClipManager?.tryConnectPairedDevices()

        print("DevicesSettingsTab: Refreshed device list")
    }

    private func requestAddDevice() {
        // Post notification to open pairing window
        NotificationCenter.default.post(name: .requestAddDevice, object: nil)
    }
}

struct DeviceSettingsRow: View {
    let device: DeviceDisplay
    let isConnected: Bool
    var onTogglePause: (() -> Void)? = nil

    var body: some View {
        HStack {
            Image(systemName: platformIcon)
                .font(.title2)
                .foregroundColor(device.isPaused ? .secondary.opacity(0.5) : .secondary)
                .frame(width: 32)

            VStack(alignment: .leading, spacing: 2) {
                HStack(spacing: 4) {
                    Text(device.name)
                        .fontWeight(.medium)
                        .foregroundColor(device.isPaused ? .secondary : .primary)

                    if device.isPaused {
                        Text("(Paused)")
                            .font(.caption)
                            .foregroundColor(.orange)
                    }
                }

                HStack(spacing: 4) {
                    Text(device.platform)
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if isConnected && !device.isPaused {
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

            // Pause/Resume button
            Button(action: { onTogglePause?() }) {
                Image(systemName: device.isPaused ? "play.circle" : "pause.circle")
                    .font(.title3)
                    .foregroundColor(device.isPaused ? .green : .orange)
            }
            .buttonStyle(.plain)
            .help(device.isPaused ? "Resume sync for this device" : "Pause sync for this device")
        }
        .padding(.vertical, 4)
        .opacity(device.isPaused ? 0.7 : 1.0)
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

// MARK: - History Settings Tab

struct HistorySettingsTab: View {
    @ObservedObject private var historyManager = SyncHistoryManager.shared
    @State private var showClearConfirmation = false

    var body: some View {
        VStack(spacing: 0) {
            // Header
            HStack {
                Text("同步记录")
                    .font(.headline)
                Spacer()
                if !historyManager.syncHistory.isEmpty {
                    Button(action: { showClearConfirmation = true }) {
                        Text("清除全部")
                            .font(.caption)
                    }
                }
            }
            .padding()

            Divider()

            // History list
            if historyManager.syncHistory.isEmpty {
                VStack(spacing: 8) {
                    Image(systemName: "clock.arrow.circlepath")
                        .font(.largeTitle)
                        .foregroundColor(.secondary)
                    Text("暂无同步记录")
                        .foregroundColor(.secondary)
                    Text("同步剪贴板后记录将显示在此处")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .frame(maxHeight: .infinity)
            } else {
                ScrollView {
                    LazyVStack(spacing: 6) {
                        ForEach(historyManager.syncHistory) { record in
                            SyncHistoryRow(record: record)
                        }
                    }
                    .padding(.horizontal)
                    .padding(.vertical, 8)
                }
            }
        }
        .alert("清除同步记录", isPresented: $showClearConfirmation) {
            Button("取消", role: .cancel) {}
            Button("清除", role: .destructive) {
                historyManager.clearHistory()
            }
        } message: {
            Text("确定要清除所有同步记录吗？此操作不可撤销。")
        }
    }
}

// MARK: - Debug Settings Tab

struct DebugSettingsTab: View {
    @ObservedObject var connectionManager: ConnectionManager
    @State private var testMessage: String = "Hello from NearClip!"
    @State private var sendStatus: String = ""
    @State private var selectedChannel: TransportChannel = .auto
    @State private var logMessages: [String] = []

    enum TransportChannel: String, CaseIterable {
        case auto = "Auto"
        case wifi = "WiFi"
        case ble = "BLE"
    }

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                // Connection Status
                VStack(alignment: .leading, spacing: 8) {
                    Text("连接状态")
                        .font(.headline)

                    HStack {
                        Circle()
                            .fill(connectionManager.status.isConnected ? Color.green : Color.red)
                            .frame(width: 10, height: 10)
                        Text(connectionManager.status.displayText)
                            .font(.subheadline)
                    }

                    if !connectionManager.connectedDevices.isEmpty {
                        ForEach(connectionManager.connectedDevices) { device in
                            HStack {
                                Image(systemName: device.platform.lowercased() == "android" ? "phone" : "laptopcomputer")
                                    .foregroundColor(.secondary)
                                Text(device.name)
                                    .font(.caption)
                                Spacer()
                                Text(channelText(for: device))
                                    .font(.caption2)
                                    .foregroundColor(.blue)
                            }
                            .padding(.leading, 16)
                        }
                    }

                    // BLE Status
                    HStack {
                        Text("BLE:")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Circle()
                            .fill(connectionManager.bleEnabled ? Color.green : Color.gray)
                            .frame(width: 8, height: 8)
                        Text(connectionManager.bleEnabled ? "已启用" : "未启用")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                Divider()

                // Test Send
                VStack(alignment: .leading, spacing: 8) {
                    Text("测试发送")
                        .font(.headline)

                    TextField("输入测试消息", text: $testMessage)
                        .textFieldStyle(.roundedBorder)

                    HStack {
                        Picker("通道", selection: $selectedChannel) {
                            ForEach(TransportChannel.allCases, id: \.self) { channel in
                                Text(channel.rawValue).tag(channel)
                            }
                        }
                        .pickerStyle(.segmented)
                        .frame(width: 180)

                        Spacer()

                        Button("发送") {
                            sendTestMessage()
                        }
                        .disabled(testMessage.isEmpty || connectionManager.connectedDevices.isEmpty)
                    }

                    if !sendStatus.isEmpty {
                        Text(sendStatus)
                            .font(.caption)
                            .foregroundColor(sendStatus.contains("成功") ? .green : (sendStatus.contains("失败") ? .red : .secondary))
                    }
                }

                Divider()

                // Quick Actions
                VStack(alignment: .leading, spacing: 8) {
                    Text("快捷操作")
                        .font(.headline)

                    HStack(spacing: 12) {
                        Button("重启服务") {
                            addLog("重启服务...")
                            connectionManager.restart()
                            addLog("服务已重启")
                        }

                        Button("刷新设备") {
                            addLog("刷新设备列表...")
                            connectionManager.refreshDeviceLists()
                            addLog("设备列表已刷新: \(connectionManager.connectedDevices.count) 已连接")
                        }

                        Button("尝试重连") {
                            addLog("尝试连接配对设备...")
                            let count = connectionManager.nearClipManager?.tryConnectPairedDevices() ?? 0
                            addLog("tryConnectPairedDevices 返回: \(count)")
                        }
                    }
                }

                Divider()

                // Log
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Text("日志")
                            .font(.headline)
                        Spacer()
                        Button("清除") {
                            logMessages.removeAll()
                        }
                        .font(.caption)
                    }

                    ScrollView {
                        VStack(alignment: .leading, spacing: 2) {
                            ForEach(Array(logMessages.enumerated()), id: \.offset) { _, message in
                                Text(message)
                                    .font(.system(size: 10, design: .monospaced))
                                    .foregroundColor(.secondary)
                            }
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .frame(height: 80)
                    .padding(6)
                    .background(Color(NSColor.textBackgroundColor))
                    .cornerRadius(4)
                }
            }
            .padding()
        }
    }

    private func channelText(for device: DeviceDisplay) -> String {
        switch device.connectionType {
        case .wifi: return "WiFi"
        case .ble: return "BLE"
        case .both: return "WiFi+BLE"
        }
    }

    private func sendTestMessage() {
        guard let data = testMessage.data(using: .utf8) else {
            sendStatus = "❌ 消息编码失败"
            return
        }

        let timestamp = DateFormatter.localizedString(from: Date(), dateStyle: .none, timeStyle: .medium)
        addLog("[\(timestamp)] 发送: \(testMessage)")

        switch selectedChannel {
        case .auto:
            // Use default sync which prefers WiFi
            connectionManager.syncClipboard(data)
            sendStatus = "✅ 已通过自动通道发送"
            addLog("通过自动通道发送 \(data.count) 字节")

        case .wifi:
            do {
                try connectionManager.nearClipManager?.syncClipboard(content: data)
                sendStatus = "✅ 已通过 WiFi 发送"
                addLog("通过 WiFi 发送 \(data.count) 字节")
            } catch {
                sendStatus = "❌ WiFi 发送失败: \(error.localizedDescription)"
                addLog("WiFi 发送失败: \(error)")
            }

        case .ble:
            let bleDevices = connectionManager.connectedDevices.filter {
                connectionManager.isDeviceConnectedViaBle($0.id)
            }
            if bleDevices.isEmpty {
                sendStatus = "❌ 没有 BLE 连接的设备"
                addLog("BLE 发送失败: 没有 BLE 连接")
            } else {
                for device in bleDevices {
                    connectionManager.syncClipboardViaBle(data, to: device.id)
                    addLog("通过 BLE 发送到 \(device.name)")
                }
                sendStatus = "✅ 已通过 BLE 发送到 \(bleDevices.count) 个设备"
            }
        }
    }

    private func addLog(_ message: String) {
        let timestamp = DateFormatter.localizedString(from: Date(), dateStyle: .none, timeStyle: .medium)
        logMessages.append("[\(timestamp)] \(message)")
        // Keep only last 50 messages
        if logMessages.count > 50 {
            logMessages.removeFirst()
        }
    }
}

struct SyncHistoryRow: View {
    let record: SyncRecord

    var body: some View {
        HStack(spacing: 10) {
            // Direction icon
            Image(systemName: iconName)
                .font(.system(size: 14))
                .foregroundColor(iconColor)
                .frame(width: 20)

            VStack(alignment: .leading, spacing: 2) {
                // Device name and direction
                HStack(spacing: 4) {
                    Text(record.direction == .sent ? "发送到" : "接收自")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text(record.deviceName)
                        .font(.caption)
                        .fontWeight(.medium)
                        .lineLimit(1)
                }

                // Content preview or error message
                if !record.contentPreview.isEmpty {
                    Text(record.contentPreview)
                        .font(.caption2)
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                } else if !record.success, let error = record.errorMessage {
                    Text(error)
                        .font(.caption2)
                        .foregroundColor(.red)
                        .lineLimit(1)
                }
            }

            Spacer()

            // Timestamp
            Text(record.getRelativeTime())
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .padding(.vertical, 6)
        .padding(.horizontal, 10)
        .background(
            RoundedRectangle(cornerRadius: 6)
                .fill(record.success ? Color.clear : Color.red.opacity(0.1))
        )
    }

    private var iconName: String {
        if !record.success {
            return "exclamationmark.circle"
        }
        return record.direction == .sent ? "arrow.up.circle" : "arrow.down.circle"
    }

    private var iconColor: Color {
        if !record.success {
            return .red
        }
        return record.direction == .sent ? .blue : .green
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
            contentRect: NSRect(x: 0, y: 0, width: 520, height: 400),
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
