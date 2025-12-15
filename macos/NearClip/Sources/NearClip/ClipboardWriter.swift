import AppKit
import Combine

/// Writes content to the system clipboard
/// Works with ClipboardMonitor to prevent sync loops
final class ClipboardWriter {
    static let shared = ClipboardWriter()

    private init() {}

    // MARK: - Public API

    /// Write string content to the clipboard
    /// - Parameters:
    ///   - content: The data to write (must be valid UTF-8 string)
    ///   - markAsRemote: Whether to mark as remote content (default: true)
    /// - Returns: True if write was successful
    @discardableResult
    func write(_ content: Data, markAsRemote: Bool = true) -> Bool {
        // Validate and convert data to string
        guard let string = String(data: content, encoding: .utf8) else {
            print("ClipboardWriter: Failed to convert data to UTF-8 string")
            return false
        }

        return writeString(string, markAsRemote: markAsRemote, originalData: content)
    }

    /// Write string content to the clipboard
    /// - Parameters:
    ///   - string: The string to write
    ///   - markAsRemote: Whether to mark as remote content (default: true)
    /// - Returns: True if write was successful
    @discardableResult
    func writeString(_ string: String, markAsRemote: Bool = true, originalData: Data? = nil) -> Bool {
        // Mark as remote before writing to prevent sync loop
        if markAsRemote {
            let data = originalData ?? string.data(using: .utf8)
            if let data = data {
                ClipboardMonitor.shared.markAsRemote(data)
            }
        }

        // Write to clipboard on main thread
        let result: Bool
        if Thread.isMainThread {
            result = performWrite(string)
        } else {
            result = DispatchQueue.main.sync {
                performWrite(string)
            }
        }

        if result {
            print("ClipboardWriter: Wrote \(string.count) characters to clipboard")
        } else {
            print("ClipboardWriter: Failed to write to clipboard")
        }

        return result
    }

    // MARK: - Private

    private func performWrite(_ string: String) -> Bool {
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        return pasteboard.setString(string, forType: .string)
    }
}
