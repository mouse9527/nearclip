import Foundation

/// Manages storage of pairing information using UserDefaults
/// Note: Changed from Keychain to UserDefaults to avoid password prompts
/// during development (code signature changes trigger Keychain access dialogs)
final class KeychainManager {
    static let shared = KeychainManager()

    private let defaults = UserDefaults.standard
    private let pairedDevicesKey = "com.nearclip.pairedDevices"

    private init() {}

    // MARK: - Paired Devices Storage

    /// JSON encoder with consistent date format
    private lazy var encoder: JSONEncoder = {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .secondsSince1970
        return encoder
    }()

    /// JSON decoder that handles multiple date formats
    private lazy var decoder: JSONDecoder = {
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .custom { decoder in
            let container = try decoder.singleValueContainer()
            // Try decoding as Double (seconds since 1970) first
            if let timestamp = try? container.decode(Double.self) {
                return Date(timeIntervalSince1970: timestamp)
            }
            // Try decoding as String (ISO8601 format)
            if let dateString = try? container.decode(String.self) {
                let formatter = ISO8601DateFormatter()
                if let date = formatter.date(from: dateString) {
                    return date
                }
                // Try simpler date format
                let simpleFormatter = DateFormatter()
                simpleFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ssZ"
                if let date = simpleFormatter.date(from: dateString) {
                    return date
                }
            }
            // Fallback to current date if parsing fails
            print("KeychainManager: Could not parse date, using current date")
            return Date()
        }
        return decoder
    }()

    /// Save paired devices to UserDefaults
    func savePairedDevices(_ devices: [StoredDevice]) -> Bool {
        do {
            let data = try encoder.encode(devices)
            defaults.set(data, forKey: pairedDevicesKey)
            print("KeychainManager: Saved \(devices.count) paired devices")
            return true
        } catch {
            print("KeychainManager: Failed to encode devices: \(error)")
            return false
        }
    }

    /// Load paired devices from UserDefaults
    func loadPairedDevices() -> [StoredDevice] {
        guard let data = defaults.data(forKey: pairedDevicesKey) else {
            print("KeychainManager: No paired devices data found")
            return []
        }

        do {
            let devices = try decoder.decode([StoredDevice].self, from: data)
            print("KeychainManager: Loaded \(devices.count) paired devices")
            return devices
        } catch {
            print("KeychainManager: Failed to decode devices: \(error), clearing old data")
            // Clear corrupted data and return empty
            defaults.removeObject(forKey: pairedDevicesKey)
            return []
        }
    }

    /// Add a device to paired devices
    func addPairedDevice(_ device: StoredDevice) -> Bool {
        var devices = loadPairedDevices()

        // Remove existing device with same ID
        devices.removeAll { $0.id == device.id }

        // Add new device
        devices.append(device)

        return savePairedDevices(devices)
    }

    /// Remove a device from paired devices
    func removePairedDevice(deviceId: String) -> Bool {
        var devices = loadPairedDevices()
        devices.removeAll { $0.id == deviceId }
        return savePairedDevices(devices)
    }

    /// Clear all paired devices
    func clearPairedDevices() -> Bool {
        defaults.removeObject(forKey: pairedDevicesKey)
        return true
    }
}

// MARK: - Stored Device Model

/// Device information stored in UserDefaults
struct StoredDevice: Codable, Identifiable, Equatable {
    let id: String
    let name: String
    let platform: String
    let addedAt: Date

    init(id: String, name: String, platform: String, addedAt: Date = Date()) {
        self.id = id
        self.name = name
        self.platform = platform
        self.addedAt = addedAt
    }

    /// Create from DeviceDisplay
    init(from display: DeviceDisplay) {
        self.id = display.id
        self.name = display.name
        self.platform = display.platform
        self.addedAt = Date()
    }

    /// Create from FFI device info
    init(from ffi: FfiDeviceInfo) {
        self.id = ffi.id
        self.name = ffi.name
        self.platform = platformString(ffi.platform)
        self.addedAt = Date()
    }

    /// Convert to DeviceDisplay
    func toDeviceDisplay(isConnected: Bool = false) -> DeviceDisplay {
        DeviceDisplay(
            id: id,
            name: name,
            platform: platform,
            isConnected: isConnected
        )
    }
}

// Helper function
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
