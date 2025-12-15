import Foundation
import UserNotifications

/// Manages system notifications for NearClip
final class NotificationManager: NSObject {
    static let shared = NotificationManager()

    private var isAuthorized = false

    // Notification categories and actions
    private static let syncFailureCategoryId = "SYNC_FAILURE"
    private static let retryActionId = "RETRY_SYNC"

    // Callback for retry action
    var onRetryRequested: (() -> Void)?

    private override init() {
        super.init()
        setupNotificationCategories()
        requestAuthorization()
    }

    // MARK: - Setup

    private func setupNotificationCategories() {
        // Define retry action
        let retryAction = UNNotificationAction(
            identifier: Self.retryActionId,
            title: "Retry",
            options: [.foreground]
        )

        // Define sync failure category with retry action
        let syncFailureCategory = UNNotificationCategory(
            identifier: Self.syncFailureCategoryId,
            actions: [retryAction],
            intentIdentifiers: [],
            options: []
        )

        // Register categories
        UNUserNotificationCenter.current().setNotificationCategories([syncFailureCategory])
    }

    // MARK: - Authorization

    /// Request notification authorization from user
    func requestAuthorization() {
        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound]) { [weak self] granted, error in
            DispatchQueue.main.async {
                self?.isAuthorized = granted
                if granted {
                    print("NotificationManager: Authorization granted")
                } else if let error = error {
                    print("NotificationManager: Authorization denied - \(error.localizedDescription)")
                } else {
                    print("NotificationManager: Authorization denied")
                }
            }
        }

        // Set delegate to handle foreground notifications
        UNUserNotificationCenter.current().delegate = self
    }

    /// Check current authorization status
    func checkAuthorization(completion: @escaping (Bool) -> Void) {
        UNUserNotificationCenter.current().getNotificationSettings { settings in
            DispatchQueue.main.async {
                let authorized = settings.authorizationStatus == .authorized
                completion(authorized)
            }
        }
    }

    // MARK: - Sync Notifications

    /// Show a notification when clipboard sync succeeds
    /// - Parameters:
    ///   - fromDevice: Name of the device that sent the clipboard content
    ///   - contentPreview: Optional preview of the content (first few characters)
    func showSyncSuccessNotification(fromDevice: String, contentPreview: String? = nil) {
        // Check if notifications are enabled in settings
        guard UserDefaults.standard.bool(forKey: "syncNotificationsEnabled") else {
            print("NotificationManager: Sync notifications disabled in settings")
            return
        }

        guard isAuthorized else {
            print("NotificationManager: Not authorized to show notifications")
            return
        }

        let content = UNMutableNotificationContent()
        content.title = "NearClip"

        if let preview = contentPreview, !preview.isEmpty {
            let truncated = preview.count > 50 ? String(preview.prefix(50)) + "..." : preview
            content.body = "Synced from \(fromDevice): \"\(truncated)\""
        } else {
            content.body = "Synced from \(fromDevice)"
        }

        content.sound = nil // Silent notification

        // Create request with unique identifier
        let request = UNNotificationRequest(
            identifier: "sync-success-\(UUID().uuidString)",
            content: content,
            trigger: nil // Deliver immediately
        )

        UNUserNotificationCenter.current().add(request) { error in
            if let error = error {
                print("NotificationManager: Failed to show notification - \(error.localizedDescription)")
            } else {
                print("NotificationManager: Showed sync success notification")
            }
        }
    }

    /// Show a notification when clipboard sync fails
    /// - Parameters:
    ///   - toDevice: Name of the device we tried to sync to (optional)
    ///   - reason: The failure reason
    func showSyncFailureNotification(toDevice: String? = nil, reason: String) {
        // Check if notifications are enabled in settings
        guard UserDefaults.standard.bool(forKey: "syncNotificationsEnabled") else {
            return
        }

        guard isAuthorized else { return }

        let content = UNMutableNotificationContent()
        content.title = "NearClip Sync Failed"

        if let device = toDevice {
            content.body = "Failed to sync to \(device): \(reason)"
        } else {
            content.body = "Sync failed: \(reason)"
        }

        content.sound = .default // Use default sound for errors
        content.categoryIdentifier = Self.syncFailureCategoryId // Enable retry action

        let request = UNNotificationRequest(
            identifier: "sync-failure-\(UUID().uuidString)",
            content: content,
            trigger: nil
        )

        UNUserNotificationCenter.current().add(request) { error in
            if let error = error {
                print("NotificationManager: Failed to show failure notification - \(error.localizedDescription)")
            } else {
                print("NotificationManager: Showed sync failure notification")
            }
        }
    }

    // MARK: - Utility

    /// Remove all pending and delivered notifications
    func clearAllNotifications() {
        UNUserNotificationCenter.current().removeAllPendingNotificationRequests()
        UNUserNotificationCenter.current().removeAllDeliveredNotifications()
    }
}

// MARK: - UNUserNotificationCenterDelegate

extension NotificationManager: UNUserNotificationCenterDelegate {
    /// Show notifications even when app is in foreground
    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        // Show banner and list even when app is active
        completionHandler([.banner, .list])
    }

    /// Handle notification tap and actions
    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        switch response.actionIdentifier {
        case Self.retryActionId:
            // User tapped Retry action
            print("NotificationManager: Retry action triggered")
            DispatchQueue.main.async { [weak self] in
                self?.onRetryRequested?()
            }

        case UNNotificationDefaultActionIdentifier:
            // User tapped the notification itself - bring app to front
            print("NotificationManager: Notification tapped")
            DispatchQueue.main.async {
                NSApp.activate(ignoringOtherApps: true)
            }

        default:
            break
        }

        completionHandler()
    }
}
