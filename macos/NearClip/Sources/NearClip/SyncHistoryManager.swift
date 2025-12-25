import Foundation
import Combine

/// Represents the direction of a sync operation
enum SyncDirection: String {
    case sent       // Sent to other device
    case received   // Received from other device
}

/// View model wrapper for FfiSyncHistoryEntry
struct SyncRecord: Identifiable, Equatable {
    let id: Int64
    let timestamp: Date
    let direction: SyncDirection
    let deviceId: String
    let deviceName: String
    let contentPreview: String
    let contentSize: UInt64
    let success: Bool
    let errorMessage: String?

    init(from ffiEntry: FfiSyncHistoryEntry) {
        self.id = ffiEntry.id
        self.timestamp = Date(timeIntervalSince1970: Double(ffiEntry.timestampMs) / 1000.0)
        self.direction = ffiEntry.direction == "sent" ? .sent : .received
        self.deviceId = ffiEntry.deviceId
        self.deviceName = ffiEntry.deviceName
        self.contentPreview = ffiEntry.contentPreview
        self.contentSize = ffiEntry.contentSize
        self.success = ffiEntry.success
        self.errorMessage = ffiEntry.errorMessage
    }

    /// Format timestamp as relative time string
    func getRelativeTime() -> String {
        let now = Date()
        let diff = now.timeIntervalSince(timestamp)

        if diff < 60 {
            return "刚刚"
        } else if diff < 3600 {
            let minutes = Int(diff / 60)
            return "\(minutes) 分钟前"
        } else if diff < 86400 {
            let hours = Int(diff / 3600)
            return "\(hours) 小时前"
        } else if diff < 604800 {
            let days = Int(diff / 86400)
            return "\(days) 天前"
        } else {
            let formatter = DateFormatter()
            formatter.dateFormat = "MM-dd HH:mm"
            return formatter.string(from: timestamp)
        }
    }
}

/// Manager for sync history - delegates to FFI layer
final class SyncHistoryManager: ObservableObject {
    static let shared = SyncHistoryManager()

    @Published private(set) var syncHistory: [SyncRecord] = []

    private weak var nearClipManager: FfiNearClipManager?

    private init() {}

    // MARK: - Setup

    /// Set the FFI manager reference
    func setManager(_ manager: FfiNearClipManager) {
        self.nearClipManager = manager
        loadHistory()
    }

    // MARK: - Public API

    /// Record a successful send operation
    func recordSent(deviceId: String, deviceName: String, content: Data) {
        let preview = String(data: content, encoding: .utf8)?.prefix(100).description ?? "[Binary data]"

        let entry = FfiSyncHistoryEntry(
            id: 0,  // Will be assigned by FFI
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: preview,
            contentSize: UInt64(content.count),
            direction: "sent",
            timestampMs: Int64(Date().timeIntervalSince1970 * 1000),
            success: true,
            errorMessage: nil
        )

        addEntry(entry)
    }

    /// Record a successful receive operation
    func recordReceived(deviceId: String, deviceName: String, content: Data) {
        let preview = String(data: content, encoding: .utf8)?.prefix(100).description ?? "[Binary data]"

        let entry = FfiSyncHistoryEntry(
            id: 0,
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: preview,
            contentSize: UInt64(content.count),
            direction: "received",
            timestampMs: Int64(Date().timeIntervalSince1970 * 1000),
            success: true,
            errorMessage: nil
        )

        addEntry(entry)
    }

    /// Record a failed sync operation
    func recordError(direction: SyncDirection, deviceId: String, deviceName: String, errorMessage: String) {
        let entry = FfiSyncHistoryEntry(
            id: 0,
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: "",
            contentSize: 0,
            direction: direction == .sent ? "sent" : "received",
            timestampMs: Int64(Date().timeIntervalSince1970 * 1000),
            success: false,
            errorMessage: errorMessage
        )

        addEntry(entry)
    }

    /// Clear all sync history
    func clearHistory() {
        guard let manager = nearClipManager else {
            print("SyncHistoryManager: No manager set, cannot clear history")
            return
        }

        do {
            try manager.clearAllHistory()
            DispatchQueue.main.async {
                self.syncHistory = []
            }
            print("SyncHistoryManager: Cleared all history")
        } catch {
            print("SyncHistoryManager: Failed to clear history: \(error)")
        }
    }

    /// Reload history from FFI layer
    func loadHistory() {
        guard let manager = nearClipManager else {
            return
        }

        do {
            let entries = try manager.getRecentHistory(limit: 50)
            let records = entries.map { SyncRecord(from: $0) }
            DispatchQueue.main.async {
                self.syncHistory = records
            }
            print("SyncHistoryManager: Loaded \(records.count) records from FFI")
        } catch {
            print("SyncHistoryManager: Failed to load history: \(error)")
        }
    }

    // MARK: - Private

    private func addEntry(_ entry: FfiSyncHistoryEntry) {
        guard let manager = nearClipManager else {
            print("SyncHistoryManager: No manager set, cannot add entry")
            return
        }

        do {
            _ = try manager.addHistoryEntry(entry: entry)
            // Reload to get the updated list with proper IDs
            loadHistory()
            print("SyncHistoryManager: Added record - \(entry.direction) \(entry.success ? "success" : "failed")")
        } catch {
            print("SyncHistoryManager: Failed to add entry: \(error)")
        }
    }
}
