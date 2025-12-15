import AppKit
import CryptoKit
import Combine

/// Monitors the system clipboard for changes and triggers sync
final class ClipboardMonitor: ObservableObject {
    static let shared = ClipboardMonitor()

    /// Whether monitoring is currently active
    @Published private(set) var isMonitoring = false

    /// Last detected clipboard content (for debugging)
    @Published private(set) var lastContent: String?

    /// Polling interval in seconds
    private let pollingInterval: TimeInterval = 0.5

    /// Timer for polling clipboard changes
    private var timer: Timer?

    /// Last known change count from NSPasteboard
    private var lastChangeCount: Int = 0

    /// Set of content hashes that came from remote devices
    /// Used to prevent sync loops
    private var remoteContentHashes: Set<String> = []

    /// Maximum number of remote hashes to keep (to prevent memory growth)
    private let maxRemoteHashes = 100

    /// Callback when clipboard content should be synced
    var onClipboardChange: ((Data) -> Void)?

    private init() {
        // Initialize with current change count to avoid syncing existing content
        lastChangeCount = NSPasteboard.general.changeCount
    }

    // MARK: - Public API

    /// Start monitoring clipboard changes
    func startMonitoring() {
        guard !isMonitoring else { return }

        // Update to current change count
        lastChangeCount = NSPasteboard.general.changeCount

        // Start polling timer
        timer = Timer.scheduledTimer(withTimeInterval: pollingInterval, repeats: true) { [weak self] _ in
            self?.checkClipboard()
        }

        // Add to common run loop mode to ensure it fires during UI interactions
        if let timer = timer {
            RunLoop.current.add(timer, forMode: .common)
        }

        isMonitoring = true
        print("Clipboard monitoring started")
    }

    /// Stop monitoring clipboard changes
    func stopMonitoring() {
        timer?.invalidate()
        timer = nil
        isMonitoring = false
        print("Clipboard monitoring stopped")
    }

    /// Mark content as coming from a remote device
    /// This prevents it from being synced back when written to clipboard
    func markAsRemote(_ content: Data) {
        let hash = contentHash(content)
        remoteContentHashes.insert(hash)

        // Prune old hashes if needed
        if remoteContentHashes.count > maxRemoteHashes {
            // Remove oldest entries (simple approach: remove random ones)
            while remoteContentHashes.count > maxRemoteHashes / 2 {
                if let first = remoteContentHashes.first {
                    remoteContentHashes.remove(first)
                }
            }
        }
    }

    /// Clear all remote content hashes
    func clearRemoteHashes() {
        remoteContentHashes.removeAll()
    }

    // MARK: - Private

    private func checkClipboard() {
        let pasteboard = NSPasteboard.general
        let currentChangeCount = pasteboard.changeCount

        // Check if clipboard has changed
        guard currentChangeCount != lastChangeCount else { return }

        lastChangeCount = currentChangeCount

        // Extract text content
        guard let stringContent = pasteboard.string(forType: .string) else {
            // No string content, could be other types (images, files, etc.)
            // For now, we only support text
            return
        }

        // Convert to data
        guard let data = stringContent.data(using: .utf8) else {
            print("Failed to convert clipboard string to data")
            return
        }

        // Check if this is remote content (to prevent sync loops)
        if isRemoteContent(data) {
            print("Clipboard content is from remote device, skipping sync")
            // Remove from remote hashes since we've now seen it
            let hash = contentHash(data)
            remoteContentHashes.remove(hash)
            return
        }

        // Update last content for debugging
        DispatchQueue.main.async { [weak self] in
            self?.lastContent = stringContent.prefix(100).description
        }

        // Trigger sync
        print("Clipboard changed: \(data.count) bytes")
        onClipboardChange?(data)
    }

    private func isRemoteContent(_ content: Data) -> Bool {
        let hash = contentHash(content)
        return remoteContentHashes.contains(hash)
    }

    private func contentHash(_ content: Data) -> String {
        let digest = SHA256.hash(data: content)
        return digest.map { String(format: "%02x", $0) }.joined()
    }
}

// MARK: - Integration with ConnectionManager

extension ClipboardMonitor {
    /// Configure the monitor to work with ConnectionManager
    func configure(with connectionManager: ConnectionManager) {
        onClipboardChange = { [weak connectionManager] data in
            connectionManager?.syncClipboard(data)
        }
    }
}
