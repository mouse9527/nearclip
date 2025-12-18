import Foundation
import Security

/// Manages secure storage of pairing information in macOS Keychain
final class KeychainManager {
    static let shared = KeychainManager()

    private let service = "com.nearclip.pairing"
    private let pairedDevicesAccount = "paired-devices"

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

    /// Save paired devices to Keychain
    func savePairedDevices(_ devices: [StoredDevice]) -> Bool {
        do {
            let data = try encoder.encode(devices)
            return save(data: data, account: pairedDevicesAccount)
        } catch {
            print("KeychainManager: Failed to encode devices: \(error)")
            return false
        }
    }

    /// Load paired devices from Keychain
    func loadPairedDevices() -> [StoredDevice] {
        guard let data = load(account: pairedDevicesAccount) else {
            return []
        }

        do {
            let devices = try decoder.decode([StoredDevice].self, from: data)
            print("KeychainManager: Loaded \(devices.count) paired devices")
            return devices
        } catch {
            print("KeychainManager: Failed to decode devices: \(error), clearing old data")
            // Clear corrupted data and return empty
            _ = delete(account: pairedDevicesAccount)
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
        return delete(account: pairedDevicesAccount)
    }

    // MARK: - Generic Keychain Operations

    /// Save data to Keychain
    private func save(data: Data, account: String) -> Bool {
        // First try to delete any existing item
        _ = delete(account: account)

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlock
        ]

        let status = SecItemAdd(query as CFDictionary, nil)

        if status == errSecSuccess {
            print("KeychainManager: Saved data for account '\(account)'")
            return true
        } else {
            print("KeychainManager: Failed to save data, status: \(status)")
            return false
        }
    }

    /// Load data from Keychain
    private func load(account: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecSuccess {
            return result as? Data
        } else if status == errSecItemNotFound {
            print("KeychainManager: No data found for account '\(account)'")
            return nil
        } else {
            print("KeychainManager: Failed to load data, status: \(status)")
            return nil
        }
    }

    /// Delete data from Keychain
    @discardableResult
    private func delete(account: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account
        ]

        let status = SecItemDelete(query as CFDictionary)

        if status == errSecSuccess || status == errSecItemNotFound {
            return true
        } else {
            print("KeychainManager: Failed to delete data, status: \(status)")
            return false
        }
    }
}

// MARK: - Stored Device Model

/// Device information stored in Keychain
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
