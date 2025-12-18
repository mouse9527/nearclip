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

    var body: some View {
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
        .frame(width: 400, height: 500)
        .background(Color(NSColor.windowBackgroundColor))
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
        let info: [String: Any] = [
            "id": deviceId,
            "name": deviceName,
            "platform": "macOS"
        ]

        if let data = try? JSONSerialization.data(withJSONObject: info),
           let string = String(data: data, encoding: .utf8) {
            return string
        }
        return "{}"
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

        // Parse the pairing code
        guard let data = manualPairingCode.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let id = json["id"] as? String,
              let name = json["name"] as? String else {
            pairingError = "Invalid pairing code format"
            isPairing = false
            return
        }

        let platform = json["platform"] as? String ?? "Unknown"

        // Create device display
        let device = DeviceDisplay(
            id: id,
            name: name,
            platform: platform,
            isConnected: false
        )

        // Add to paired devices (saves to Keychain and updates FFI)
        connectionManager.addPairedDevice(device)

        isPairing = false
        pairingSuccess = true
        manualPairingCode = ""

        // Auto-dismiss after success
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
            dismiss()
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
