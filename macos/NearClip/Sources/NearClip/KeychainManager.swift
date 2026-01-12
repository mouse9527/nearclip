import Foundation
import Security
import NearClipFFI

/// Manages secure storage of pairing information using macOS Keychain
/// Uses Apple's Security framework for encrypted storage
final class KeychainManager {
    static let shared = KeychainManager()

    private let serviceName = "com.nearclip.devices"
    private let legacyUserDefaultsKey = "com.nearclip.pairedDevices"  // For migration

    private init() {
        // Auto-migrate from UserDefaults on first launch
        migrateFromUserDefaultsIfNeeded()
    }

    // MARK: - Keychain Error Handling

    enum KeychainError: Error {
        case saveFailed(OSStatus)
        case loadFailed(OSStatus)
        case deleteFailed(OSStatus)
        case encodingFailed(Error)
        case decodingFailed(Error)
        case itemNotFound

        var localizedDescription: String {
            switch self {
            case .saveFailed(let status):
                return "Failed to save to Keychain: \(statusString(status))"
            case .loadFailed(let status):
                return "Failed to load from Keychain: \(statusString(status))"
            case .deleteFailed(let status):
                return "Failed to delete from Keychain: \(statusString(status))"
            case .encodingFailed(let error):
                return "Encoding failed: \(error.localizedDescription)"
            case .decodingFailed(let error):
                return "Decoding failed: \(error.localizedDescription)"
            case .itemNotFound:
                return "Item not found in Keychain"
            }
        }

        private func statusString(_ status: OSStatus) -> String {
            if let errorString = SecCopyErrorMessageString(status, nil) {
                return errorString as String
            }
            return "OSStatus: \(status)"
        }
    }

    // MARK: - Keychain Operations

    /// Save a single device to Keychain
    private func saveDeviceToKeychain(_ device: StoredDevice) throws {
        do {
            let data = try encoder.encode(device)

            let query: [String: Any] = [
                kSecClass as String: kSecClassGenericPassword,
                kSecAttrAccount as String: device.id,
                kSecAttrService as String: serviceName,
                kSecValueData as String: data,
                kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlocked
            ]

            // Delete existing item first (to handle updates)
            let deleteStatus = SecItemDelete(query as CFDictionary)
            if deleteStatus != errSecSuccess && deleteStatus != errSecItemNotFound {
                print("KeychainManager: Warning - delete returned: \(deleteStatus)")
            }

            // Add new item
            let status = SecItemAdd(query as CFDictionary, nil)

            guard status == errSecSuccess else {
                throw KeychainError.saveFailed(status)
            }

            print("KeychainManager: Saved device '\(device.name)' (\(device.id)) to Keychain")
        } catch let error as KeychainError {
            throw error
        } catch {
            throw KeychainError.encodingFailed(error)
        }
    }

    /// Load a single device from Keychain
    private func loadDeviceFromKeychain(deviceId: String) throws -> StoredDevice? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: deviceId,
            kSecAttrService as String: serviceName,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecItemNotFound {
            return nil
        }

        guard status == errSecSuccess,
              let data = result as? Data else {
            throw KeychainError.loadFailed(status)
        }

        do {
            let device = try decoder.decode(StoredDevice.self, from: data)
            return device
        } catch {
            throw KeychainError.decodingFailed(error)
        }
    }

    /// Delete a single device from Keychain
    private func deleteDeviceFromKeychain(deviceId: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: deviceId,
            kSecAttrService as String: serviceName
        ]

        let status = SecItemDelete(query as CFDictionary)

        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.deleteFailed(status)
        }

        print("KeychainManager: Deleted device '\(deviceId)' from Keychain")
    }

    /// Load all device IDs from Keychain
    private func loadAllDeviceIds() throws -> [String] {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: serviceName,
            kSecReturnAttributes as String: true,
            kSecMatchLimit as String: kSecMatchLimitAll
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        if status == errSecItemNotFound {
            return []
        }

        guard status == errSecSuccess,
              let items = result as? [[String: Any]] else {
            throw KeychainError.loadFailed(status)
        }

        return items.compactMap { item in
            item[kSecAttrAccount as String] as? String
        }
    }

    // MARK: - Public API (Using Keychain)

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

    /// Save paired devices (now stored individually in Keychain)
    func savePairedDevices(_ devices: [StoredDevice]) -> Bool {
        var allSucceeded = true

        for device in devices {
            do {
                try saveDeviceToKeychain(device)
            } catch {
                print("KeychainManager: Failed to save device '\(device.name)': \(error)")
                allSucceeded = false
            }
        }

        if allSucceeded {
            print("KeychainManager: Saved \(devices.count) paired devices to Keychain")
        }

        return allSucceeded
    }

    /// Load all paired devices from Keychain
    func loadPairedDevices() -> [StoredDevice] {
        do {
            let deviceIds = try loadAllDeviceIds()
            var devices: [StoredDevice] = []

            for deviceId in deviceIds {
                if let device = try loadDeviceFromKeychain(deviceId: deviceId) {
                    devices.append(device)
                }
            }

            print("KeychainManager: Loaded \(devices.count) paired devices from Keychain")
            return devices

        } catch {
            print("KeychainManager: Failed to load devices: \(error)")
            return []
        }
    }

    /// Add a device to Keychain
    func addPairedDevice(_ device: StoredDevice) -> Bool {
        do {
            try saveDeviceToKeychain(device)
            return true
        } catch {
            print("KeychainManager: Failed to add device: \(error)")
            return false
        }
    }

    /// Remove a device from Keychain
    func removePairedDevice(deviceId: String) -> Bool {
        do {
            try deleteDeviceFromKeychain(deviceId: deviceId)
            return true
        } catch {
            print("KeychainManager: Failed to remove device: \(error)")
            return false
        }
    }

    /// Clear all paired devices from Keychain
    func clearPairedDevices() -> Bool {
        do {
            let deviceIds = try loadAllDeviceIds()

            for deviceId in deviceIds {
                try deleteDeviceFromKeychain(deviceId: deviceId)
            }

            print("KeychainManager: Cleared all paired devices from Keychain")
            return true

        } catch {
            print("KeychainManager: Failed to clear devices: \(error)")
            return false
        }
    }

    // MARK: - Migration from UserDefaults

    /// Migrate devices from legacy UserDefaults storage to Keychain
    private func migrateFromUserDefaultsIfNeeded() {
        let defaults = UserDefaults.standard

        guard let data = defaults.data(forKey: legacyUserDefaultsKey) else {
            // No legacy data to migrate
            return
        }

        print("KeychainManager: Found legacy UserDefaults data, migrating to Keychain...")

        do {
            let devices = try decoder.decode([StoredDevice].self, from: data)

            // Save all devices to Keychain
            for device in devices {
                try saveDeviceToKeychain(device)
            }

            // Clear UserDefaults after successful migration
            defaults.removeObject(forKey: legacyUserDefaultsKey)

            print("KeychainManager: ✅ Successfully migrated \(devices.count) devices to Keychain")

        } catch {
            print("KeychainManager: ❌ Migration failed: \(error)")
            print("KeychainManager: Legacy data will remain in UserDefaults")
        }
    }
}

// MARK: - FfiDeviceStorage Implementation

/// Implements FfiDeviceStorage protocol for Rust FFI
/// This allows Rust layer to control when devices are saved/loaded/removed
final class DeviceStorageImpl: FfiDeviceStorage {
    private let keychainManager = KeychainManager.shared

    func saveDevice(device: FfiDeviceInfo) {
        let storedDevice = StoredDevice(from: device)
        if keychainManager.addPairedDevice(storedDevice) {
            print("DeviceStorageImpl: Saved device '\(device.name)' (\(device.id))")
        } else {
            print("DeviceStorageImpl: Failed to save device '\(device.name)'")
        }
    }

    func removeDevice(deviceId: String) {
        if keychainManager.removePairedDevice(deviceId: deviceId) {
            print("DeviceStorageImpl: Removed device '\(deviceId)'")
        } else {
            print("DeviceStorageImpl: Failed to remove device '\(deviceId)'")
        }
    }

    func loadAllDevices() -> [FfiDeviceInfo] {
        let storedDevices = keychainManager.loadPairedDevices()
        let ffiDevices = storedDevices.map { stored -> FfiDeviceInfo in
            FfiDeviceInfo(
                id: stored.id,
                name: stored.name,
                platform: platformFromString(stored.platform),
                status: .disconnected
            )
        }
        print("DeviceStorageImpl: Loaded \(ffiDevices.count) devices")
        return ffiDevices
    }

    private func platformFromString(_ platform: String) -> DevicePlatform {
        switch platform.lowercased() {
        case "macos":
            return .macOs
        case "android":
            return .android
        default:
            return .unknown
        }
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
