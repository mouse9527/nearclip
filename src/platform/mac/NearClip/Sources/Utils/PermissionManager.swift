import Foundation
import AppKit
import CoreBluetooth

// 权限管理器
class PermissionManager: ObservableObject {
    @Published var bluetoothPermission: PermissionStatus = .unknown
    @Published var pasteboardPermission: PermissionStatus = .unknown

    enum PermissionStatus {
        case unknown
        case granted
        case denied
        case notRequested
    }

    static let shared = PermissionManager()

    private init() {
        checkCurrentPermissions()
    }

    // 检查当前权限状态
    private func checkCurrentPermissions() {
        checkBluetoothPermission()
        checkPasteboardPermission()
    }

    // 检查蓝牙权限
    private func checkBluetoothPermission() {
        // 在实际应用中，需要检查CBCentralManager的授权状态
        // 这里使用模拟检查
        bluetoothPermission = .notRequested
    }

    // 检查剪贴板权限
    private func checkPasteboardPermission() {
        // 剪贴板权限通常不需要特别授权，除非有沙箱限制
        pasteboardPermission = .granted
    }

    // 请求蓝牙权限
    func requestBluetoothPermission() async -> PermissionStatus {
        return await withCheckedContinuation { continuation in
            // 在实际应用中，这里会创建CBCentralManager并触发权限请求
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                self.bluetoothPermission = .granted
                continuation.resume(returning: .granted)
            }
        }
    }

    // 请求剪贴板权限
    func requestPasteboardPermission() async -> PermissionStatus {
        // 剪贴板权限通常在沙箱环境中自动处理
        return .granted
    }

    // 显示权限拒绝对话框
    func showPermissionDeniedDialog(for permission: PermissionType) {
        let alert = NSAlert()
        alert.messageText = "权限被拒绝"
        alert.informativeText = permission.deniedMessage
        alert.alertStyle = .warning
        alert.addButton(withTitle: "打开系统设置")
        alert.addButton(withTitle: "稍后")

        let response = alert.runModal()

        if response == .alertFirstButtonReturn {
            // 打开系统设置
            if let settingsUrl = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Bluetooth") {
                NSWorkspace.shared.open(settingsUrl)
            }
        }
    }

    // 权限类型
    enum PermissionType {
        case bluetooth
        case pasteboard

        var deniedMessage: String {
            switch self {
            case .bluetooth:
                return "NearClip需要蓝牙权限来发现和连接附近设备。请在系统设置中允许蓝牙访问。"
            case .pasteboard:
                return "NearClip需要剪贴板权限来同步剪贴板内容。请在系统设置中允许剪贴板访问。"
            }
        }
    }
}