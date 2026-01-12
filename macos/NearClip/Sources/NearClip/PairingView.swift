import SwiftUI
import CoreImage.CIFilterBuiltins

/// View for pairing with other devices
struct PairingView: View {
    @ObservedObject var connectionManager: ConnectionManager
    @Environment(\.dismiss) private var dismiss

    @State private var selectedTab = 0
    @State private var manualPairingCode = ""
    @State private var isPairing = false
    @State private var pairingError: String?
    @State private var pairingSuccess = false
    @State private var pairedDeviceName: String?
    @State private var showConnectionFailedAlert = false
    @State private var pendingDevice: (id: String, name: String, platform: String)?

    var body: some View {
        ZStack {
            VStack(spacing: 0) {
                // Header
                header

                Divider()

                // Tab picker
                Picker("", selection: $selectedTab) {
                    Text("Show QR Code").tag(0)
                    Text("Enter Code").tag(1)
                }
                .pickerStyle(.segmented)
                .padding()

                // Content based on tab
                if selectedTab == 0 {
                    qrCodeTab
                } else {
                    manualPairingTab
                }

                Spacer()

                // Footer
                footer
            }

            // Success overlay
            if let deviceName = pairedDeviceName {
                successOverlay(deviceName: deviceName)
            }
        }
        .frame(width: 400, height: 500)
        .background(Color(NSColor.windowBackgroundColor))
        .onReceive(NotificationCenter.default.publisher(for: .devicePaired)) { notification in
            if let device = notification.userInfo?["device"] as? DeviceDisplay {
                // Show success message and auto-close
                pairedDeviceName = device.name
                DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
                    dismiss()
                }
            }
        }
        .alert("连接失败", isPresented: $showConnectionFailedAlert) {
            Button("重试") {
                retryConnection()
            }
            Button("取消", role: .cancel) {
                pendingDevice = nil
            }
        } message: {
            Text("无法连接到设备，请确保两台设备在同一网络或蓝牙范围内。")
        }
    }

    // MARK: - Success Overlay

    private func successOverlay(deviceName: String) -> some View {
        VStack(spacing: 16) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 60))
                .foregroundColor(.green)

            Text("Paired Successfully!")
                .font(.title2)
                .fontWeight(.semibold)

            Text("Connected to \(deviceName)")
                .font(.body)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(NSColor.windowBackgroundColor).opacity(0.95))
    }

    // MARK: - Header

    private var header: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text("Add Device")
                    .font(.headline)
                Text("Scan this QR code from another device or enter a pairing code")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            Spacer()
            Button(action: { dismiss() }) {
                Image(systemName: "xmark.circle.fill")
                    .font(.title2)
                    .foregroundColor(.secondary)
            }
            .buttonStyle(.plain)
        }
        .padding()
    }

    // MARK: - QR Code Tab

    private var qrCodeTab: some View {
        VStack(spacing: 16) {
            // QR Code
            if let qrImage = generateQRCode() {
                Image(nsImage: qrImage)
                    .interpolation(.none)
                    .resizable()
                    .scaledToFit()
                    .frame(width: 200, height: 200)
                    .background(Color.white)
                    .cornerRadius(8)
            } else {
                Rectangle()
                    .fill(Color.gray.opacity(0.2))
                    .frame(width: 200, height: 200)
                    .cornerRadius(8)
                    .overlay(
                        Text("Failed to generate QR code")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    )
            }

            // Device info
            VStack(spacing: 8) {
                HStack {
                    Text("Device Name:")
                        .foregroundColor(.secondary)
                    Text(deviceName)
                        .fontWeight(.medium)
                }
                .font(.caption)

                HStack {
                    Text("Platform:")
                        .foregroundColor(.secondary)
                    Text("macOS")
                        .fontWeight(.medium)
                }
                .font(.caption)
            }

            // Pairing code (copyable)
            VStack(spacing: 4) {
                Text("Pairing Code")
                    .font(.caption)
                    .foregroundColor(.secondary)

                HStack {
                    Text(pairingCode)
                        .font(.system(.caption, design: .monospaced))
                        .textSelection(.enabled)
                        .padding(8)
                        .background(Color(NSColor.textBackgroundColor))
                        .cornerRadius(4)

                    Button(action: copyPairingCode) {
                        Image(systemName: "doc.on.doc")
                    }
                    .buttonStyle(.plain)
                    .help("Copy pairing code")
                }
            }
        }
        .padding()
    }

    // MARK: - Manual Pairing Tab

    private var manualPairingTab: some View {
        VStack(spacing: 16) {
            Text("Enter the pairing code from the other device")
                .font(.caption)
                .foregroundColor(.secondary)

            TextField("Pairing Code", text: $manualPairingCode)
                .textFieldStyle(.roundedBorder)
                .font(.system(.body, design: .monospaced))
                .frame(maxWidth: 300)

            if let error = pairingError {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.red)
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                }
            }

            if pairingSuccess {
                HStack {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                    Text("Device paired successfully!")
                        .font(.caption)
                        .foregroundColor(.green)
                }
            }

            Button(action: pairWithCode) {
                HStack {
                    if isPairing {
                        ProgressView()
                            .scaleEffect(0.7)
                    }
                    Text(isPairing ? "Pairing..." : "Pair Device")
                }
                .frame(width: 120)
            }
            .buttonStyle(.borderedProminent)
            .disabled(manualPairingCode.isEmpty || isPairing)
        }
        .padding()
    }

    // MARK: - Footer

    private var footer: some View {
        VStack(spacing: 8) {
            Divider()

            HStack {
                Text("Paired Devices: \(connectionManager.pairedDevices.count)")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Spacer()

                Button("Done") {
                    dismiss()
                }
                .keyboardShortcut(.defaultAction)
            }
            .padding()
        }
    }

    // MARK: - QR Code Generation

    private var deviceName: String {
        Host.current().localizedName ?? "Mac"
    }

    private var deviceId: String {
        // Use the device ID from NearClipManager (same as used for mDNS)
        connectionManager.deviceId
    }

    private var pairingCode: String {
        // Generate QR code data using Rust FFI (includes ECDH public key)
        do {
            return try connectionManager.generateQRCode()
        } catch {
            print("Failed to generate QR code: \(error)")
            // Fallback to simple JSON (without public key - INSECURE)
            let id = deviceId
            let name = deviceName
            return "{\"id\":\"\(id)\",\"name\":\"\(name)\",\"platform\":\"macOS\"}"
        }
    }

    private func generateQRCode() -> NSImage? {
        let context = CIContext()
        let filter = CIFilter.qrCodeGenerator()

        guard let data = pairingCode.data(using: .utf8) else {
            return nil
        }

        filter.setValue(data, forKey: "inputMessage")
        filter.setValue("M", forKey: "inputCorrectionLevel")

        guard let outputImage = filter.outputImage else {
            return nil
        }

        // Scale up the QR code
        let scale = 10.0
        let scaledImage = outputImage.transformed(by: CGAffineTransform(scaleX: scale, y: scale))

        guard let cgImage = context.createCGImage(scaledImage, from: scaledImage.extent) else {
            return nil
        }

        return NSImage(cgImage: cgImage, size: NSSize(width: cgImage.width, height: cgImage.height))
    }

    // MARK: - Actions

    private func copyPairingCode() {
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(pairingCode, forType: .string)
    }

    private func pairWithCode() {
        pairingError = nil
        pairingSuccess = false
        isPairing = true

        // Use Rust FFI to pair with QR code (handles JSON parsing, validation, connection + storage)
        DispatchQueue.global(qos: .userInitiated).async { [self] in
            do {
                let device = try connectionManager.pairWithQRCode(manualPairingCode)

                DispatchQueue.main.async {
                    isPairing = false
                    pairingSuccess = true
                    manualPairingCode = ""
                    pendingDevice = nil

                    // Auto-dismiss after success
                    DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                        dismiss()
                    }
                }
            } catch {
                DispatchQueue.main.async {
                    isPairing = false
                    pairingError = "Failed to pair: \(error.localizedDescription)"
                }
            }
        }
    }

    private func retryConnection() {
        guard let device = pendingDevice else { return }

        isPairing = true
        pairingError = nil

        // Create device display
        let deviceDisplay = DeviceDisplay(
            id: device.id,
            name: device.name,
            platform: device.platform,
            isConnected: false
        )

        // Retry pairing via Rust FFI
        DispatchQueue.global(qos: .userInitiated).async { [self] in
            let success = connectionManager.pairDevice(deviceDisplay)

            DispatchQueue.main.async {
                isPairing = false

                if success {
                    pairingSuccess = true
                    manualPairingCode = ""
                    pendingDevice = nil

                    // Auto-dismiss after success
                    DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                        dismiss()
                    }
                } else {
                    // Connection failed again, show retry dialog
                    showConnectionFailedAlert = true
                }
            }
        }
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

// MARK: - Pairing Window Controller

final class PairingWindowController: NSObject {
    private var window: NSWindow?
    private var hostingView: NSHostingView<PairingView>?

    func showWindow(connectionManager: ConnectionManager) {
        if let existingWindow = window {
            existingWindow.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
            return
        }

        let pairingView = PairingView(connectionManager: connectionManager)
        let hostingView = NSHostingView(rootView: pairingView)
        self.hostingView = hostingView

        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 400, height: 500),
            styleMask: [.titled, .closable, .miniaturizable],
            backing: .buffered,
            defer: false
        )

        window.title = "Add Device - NearClip"
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

extension PairingWindowController: NSWindowDelegate {
    func windowWillClose(_ notification: Notification) {
        window = nil
        hostingView = nil
    }
}

// MARK: - Preview

#if DEBUG
struct PairingView_Previews: PreviewProvider {
    static var previews: some View {
        PairingView(connectionManager: ConnectionManager.shared)
    }
}
#endif
