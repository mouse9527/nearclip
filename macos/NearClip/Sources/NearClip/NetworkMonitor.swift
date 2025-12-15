import Foundation
import Network

/// Monitors network connectivity and triggers reconnection on recovery
final class NetworkMonitor {
    static let shared = NetworkMonitor()

    private let monitor = NWPathMonitor()
    private let queue = DispatchQueue(label: "com.nearclip.networkmonitor")

    private var isConnected = false
    private var wasDisconnected = false
    private var reconnectAttempts = 0
    private let maxReconnectAttempts = 3
    private var reconnectWorkItem: DispatchWorkItem?

    /// Callback when network connectivity is restored
    var onNetworkRestored: (() -> Void)?

    /// Callback when reconnection fails after max attempts
    var onReconnectFailed: (() -> Void)?

    private init() {}

    // MARK: - Public API

    /// Start monitoring network connectivity
    func startMonitoring() {
        monitor.pathUpdateHandler = { [weak self] path in
            self?.handlePathUpdate(path)
        }
        monitor.start(queue: queue)
        print("NetworkMonitor: Started monitoring")
    }

    /// Stop monitoring network connectivity
    func stopMonitoring() {
        monitor.cancel()
        reconnectWorkItem?.cancel()
        reconnectWorkItem = nil
        print("NetworkMonitor: Stopped monitoring")
    }

    /// Reset reconnection attempts counter
    func resetReconnectAttempts() {
        reconnectAttempts = 0
    }

    // MARK: - Private

    private func handlePathUpdate(_ path: NWPath) {
        let newIsConnected = path.status == .satisfied

        print("NetworkMonitor: Path update - status: \(path.status), isExpensive: \(path.isExpensive), isConstrained: \(path.isConstrained)")

        if !newIsConnected && isConnected {
            // Network went down
            wasDisconnected = true
            print("NetworkMonitor: Network disconnected")
        } else if newIsConnected && wasDisconnected {
            // Network recovered
            wasDisconnected = false
            print("NetworkMonitor: Network restored, scheduling reconnection")
            scheduleReconnect()
        }

        isConnected = newIsConnected
    }

    private func scheduleReconnect() {
        // Cancel any pending reconnect
        reconnectWorkItem?.cancel()

        // Delay reconnection to let network stabilize
        let delay = calculateReconnectDelay()

        let workItem = DispatchWorkItem { [weak self] in
            self?.attemptReconnect()
        }
        reconnectWorkItem = workItem

        DispatchQueue.main.asyncAfter(deadline: .now() + delay, execute: workItem)
        print("NetworkMonitor: Reconnection scheduled in \(delay)s (attempt \(reconnectAttempts + 1)/\(maxReconnectAttempts))")
    }

    private func calculateReconnectDelay() -> TimeInterval {
        // Exponential backoff: 1s, 2s, 4s
        let baseDelay: TimeInterval = 1.0
        return baseDelay * pow(2.0, Double(reconnectAttempts))
    }

    private func attemptReconnect() {
        reconnectAttempts += 1

        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            print("NetworkMonitor: Attempting reconnection (attempt \(self.reconnectAttempts)/\(self.maxReconnectAttempts))")

            // Trigger reconnection
            self.onNetworkRestored?()

            // Check if we need to retry
            DispatchQueue.main.asyncAfter(deadline: .now() + 5.0) { [weak self] in
                guard let self = self else { return }

                // If still not connected after callback, and we haven't exceeded max attempts
                let connectionManager = ConnectionManager.shared
                if connectionManager.connectedDevices.isEmpty && self.reconnectAttempts < self.maxReconnectAttempts {
                    // Schedule another attempt
                    self.scheduleReconnect()
                } else if connectionManager.connectedDevices.isEmpty && self.reconnectAttempts >= self.maxReconnectAttempts {
                    // Max attempts reached, notify user
                    print("NetworkMonitor: Reconnection failed after \(self.maxReconnectAttempts) attempts")
                    self.onReconnectFailed?()
                } else {
                    // Successfully reconnected
                    print("NetworkMonitor: Reconnection successful")
                    self.reconnectAttempts = 0
                }
            }
        }
    }
}
