import Foundation
import Combine

/// Represents the direction of a sync operation
enum SyncDirection: String, Codable {
    case sent       // Sent to other device
    case received   // Received from other device
}

/// Represents a single sync event in history
struct SyncRecord: Identifiable, Codable, Equatable {
    let id: String
    let timestamp: Date
    let direction: SyncDirection
    let deviceId: String
    let deviceName: String
    let contentPreview: String
    let contentSize: Int
    let success: Bool
    let errorMessage: String?

    init(
        id: String = UUID().uuidString,
        timestamp: Date = Date(),
        direction: SyncDirection,
        deviceId: String,
        deviceName: String,
        contentPreview: String,
        contentSize: Int,
        success: Bool,
        errorMessage: String? = nil
    ) {
        self.id = id
        self.timestamp = timestamp
        self.direction = direction
        self.deviceId = deviceId
        self.deviceName = deviceName
        self.contentPreview = contentPreview
        self.contentSize = contentSize
        self.success = success
        self.errorMessage = errorMessage
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

/// Manager for sync history storage and retrieval
final class SyncHistoryManager: ObservableObject {
    static let shared = SyncHistoryManager()

    private static let historyKey = "syncHistory"
    private static let maxHistorySize = 50

    @Published private(set) var syncHistory: [SyncRecord] = []

    private init() {
        loadHistory()
    }

    // MARK: - Public API

    /// Record a successful send operation
    func recordSent(deviceId: String, deviceName: String, content: Data) {
        let preview = String(data: content, encoding: .utf8)?.prefix(100).description ?? "[Binary data]"

        let record = SyncRecord(
            direction: .sent,
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: preview,
            contentSize: content.count,
            success: true
        )

        addRecord(record)
    }

    /// Record a successful receive operation
    func recordReceived(deviceId: String, deviceName: String, content: Data) {
        let preview = String(data: content, encoding: .utf8)?.prefix(100).description ?? "[Binary data]"

        let record = SyncRecord(
            direction: .received,
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: preview,
            contentSize: content.count,
            success: true
        )

        addRecord(record)
    }

    /// Record a failed sync operation
    func recordError(direction: SyncDirection, deviceId: String, deviceName: String, errorMessage: String) {
        let record = SyncRecord(
            direction: direction,
            deviceId: deviceId,
            deviceName: deviceName,
            contentPreview: "",
            contentSize: 0,
            success: false,
            errorMessage: errorMessage
        )

        addRecord(record)
    }

    /// Clear all sync history
    func clearHistory() {
        syncHistory = []
        saveHistory()
        print("SyncHistoryManager: Cleared all history")
    }

    // MARK: - Private

    private func addRecord(_ record: SyncRecord) {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Add new record at the beginning
            self.syncHistory.insert(record, at: 0)

            // Trim to max size
            while self.syncHistory.count > Self.maxHistorySize {
                self.syncHistory.removeLast()
            }

            self.saveHistory()
            print("SyncHistoryManager: Added record - \(record.direction) \(record.success ? "success" : "failed")")
        }
    }

    private func loadHistory() {
        guard let data = UserDefaults.standard.data(forKey: Self.historyKey) else {
            syncHistory = []
            return
        }

        do {
            let records = try JSONDecoder().decode([SyncRecord].self, from: data)
            syncHistory = records
            print("SyncHistoryManager: Loaded \(records.count) records from storage")
        } catch {
            print("SyncHistoryManager: Failed to load history: \(error)")
            syncHistory = []
        }
    }

    private func saveHistory() {
        do {
            let data = try JSONEncoder().encode(syncHistory)
            UserDefaults.standard.set(data, forKey: Self.historyKey)
        } catch {
            print("SyncHistoryManager: Failed to save history: \(error)")
        }
    }
}
