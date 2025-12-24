import Foundation
import Network

/// Monitors network connectivity and triggers reconnection on recovery
final class NetworkMonitor {
    static let shared = NetworkMonitor()

    // MARK: - Constants

    /// Maximum number of reconnection attempts before giving up
    private static let maxReconnectAttempts = 3

    /// Base delay for exponential backoff (seconds)
    private static let baseReconnectDelay: TimeInterval = 1.0

    /// Delay to verify connection status after reconnection attempt (seconds)
    private static let connectionVerifyDelay: TimeInterval = 5.0

    // MARK: - Properties

    private var monitor: NWPathMonitor?
    private let queue = DispatchQueue(label: "com.nearclip.networkmonitor")

    private var isConnected = false
    private var wasDisconnected = false
    private var reconnectAttempts = 0
    private var reconnectWorkItem: DispatchWorkItem?
    private var isMonitoring = false

    /// Callback when network connectivity is restored
    var onNetworkRestored: (() -> Void)?

    /// Callback when network connectivity is lost
    var onNetworkLost: (() -> Void)?

    /// Callback when reconnection fails after max attempts
    var onReconnectFailed: (() -> Void)?

    private init() {}

    // MARK: - Public API

    /// Start monitoring network connectivity
    func startMonitoring() {
        guard !isMonitoring else {
            print("NetworkMonitor: Already monitoring")
            return
        }

        // Create a new monitor instance (NWPathMonitor cannot be restarted after cancel)
        let newMonitor = NWPathMonitor()
        newMonitor.pathUpdateHandler = { [weak self] path in
            self?.handlePathUpdate(path)
        }
        newMonitor.start(queue: queue)
        monitor = newMonitor
        isMonitoring = true
        print("NetworkMonitor: Started monitoring")
    }

    /// Stop monitoring network connectivity
    func stopMonitoring() {
        guard isMonitoring else { return }

        monitor?.cancel()
        monitor = nil
        reconnectWorkItem?.cancel()
        reconnectWorkItem = nil
        isMonitoring = false
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

            // Notify about network loss (for BLE fallback)
            DispatchQueue.main.async { [weak self] in
                self?.onNetworkLost?()
            }
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
        print("NetworkMonitor: Reconnection scheduled in \(delay)s (attempt \(reconnectAttempts + 1)/\(Self.maxReconnectAttempts))")
    }

    private func calculateReconnectDelay() -> TimeInterval {
        // Exponential backoff: 1s, 2s, 4s
        return Self.baseReconnectDelay * pow(2.0, Double(reconnectAttempts))
    }

    private func attemptReconnect() {
        reconnectAttempts += 1

        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            print("NetworkMonitor: Attempting reconnection (attempt \(self.reconnectAttempts)/\(Self.maxReconnectAttempts))")

            // Trigger reconnection
            self.onNetworkRestored?()

            // Check if we need to retry after a delay to allow connection to establish
            DispatchQueue.main.asyncAfter(deadline: .now() + Self.connectionVerifyDelay) { [weak self] in
                guard let self = self else { return }

                // If still not connected after callback, and we haven't exceeded max attempts
                let connectionManager = ConnectionManager.shared
                if connectionManager.connectedDevices.isEmpty && self.reconnectAttempts < Self.maxReconnectAttempts {
                    // Schedule another attempt
                    self.scheduleReconnect()
                } else if connectionManager.connectedDevices.isEmpty && self.reconnectAttempts >= Self.maxReconnectAttempts {
                    // Max attempts reached, notify user
                    print("NetworkMonitor: Reconnection failed after \(Self.maxReconnectAttempts) attempts")
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
