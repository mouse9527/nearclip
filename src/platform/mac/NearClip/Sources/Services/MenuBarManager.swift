import SwiftUI
import AppKit

// 菜单栏管理器
class MenuBarManager: ObservableObject {
    @Published var connectionStatus: ConnectionStatus = .disconnected
    @Published var isMonitoring: Bool = false
    @Published var deviceCount: Int = 0

    private var statusItem: NSStatusItem?
    private var popover: NSPopover?
    private var contentViewController: NSHostingController<ContentView>?

    init() {
        setupMenuBar()
    }

    deinit {
        cleanup()
    }

    // 设置菜单栏
    private func setupMenuBar() {
        // 创建状态栏项
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)

        if let button = statusItem?.button {
            button.image = NSImage(systemSymbolName: "antenna.radiowaves.left.and.right", accessibilityDescription: "NearClip")
            button.target = self
            button.action = #selector(statusItemClicked)
        }

        updateStatusIcon()

        print("菜单栏设置完成")
    }

    // 清理资源
    private func cleanup() {
        if popover?.isShown == true {
            popover?.performClose(nil)
        }
        statusItem = nil
        popover = nil
        contentViewController = nil
    }

    // 状态栏按钮点击事件
    @objc private func statusItemClicked() {
        togglePopover()
    }

    // 切换弹出菜单显示状态
    private func togglePopover() {
        if let popover = popover {
            if popover.isShown {
                hidePopover()
            } else {
                showPopover()
            }
        } else {
            showPopover()
        }
    }

    // 显示弹出菜单
    private func showPopover() {
        if popover == nil {
            popover = NSPopover()
            popover?.contentSize = NSSize(width: 320, height: 480)
            popover?.behavior = .transient
            popover?.animates = true

            let contentView = ContentView()
                .environmentObject(self)
            contentViewController = NSHostingController(rootView: contentView)
            popover?.contentViewController = contentViewController
        }

        guard let button = statusItem?.button else { return }

        popover?.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)

        // 使应用成为前台应用以接收键盘事件
        NSApp.activate(ignoringOtherApps: true)
    }

    // 隐藏弹出菜单
    func hidePopover() {
        popover?.performClose(nil)
    }

    // 更新状态图标
    func updateStatusIcon() {
        guard let button = statusItem?.button else { return }

        let iconName: String
        let iconColor: NSColor

        switch connectionStatus {
        case .connected:
            iconName = "antenna.radiowaves.left.and.right"
            iconColor = .systemGreen
        case .pairing:
            iconName = "antenna.radiowaves.left.and.right.badge.exclamationmark"
            iconColor = .systemBlue
        case .disconnected:
            iconName = "antenna.radiowaves.left.and.right.slash"
            iconColor = .systemGray
        case .error:
            iconName = "antenna.radiowaves.left.and.right.badge.xmark"
            iconColor = .systemRed
        case .unknown:
            iconName = "antenna.radiowaves.left.and.right.questionmark"
            iconColor = .systemOrange
        }

        if let image = NSImage(systemSymbolName: iconName, accessibilityDescription: "NearClip") {
            image.isTemplate = true
            button.image = image
            button.contentTintColor = iconColor
        }

        // 如果有设备连接，显示设备数量
        if deviceCount > 0 {
            let badgeText = "\(deviceCount)"
            let badge = createBadge(text: badgeText)
            if let existingBadge = button.subviews.first(where: { $0.tag == 999 }) {
                existingBadge.removeFromSuperview()
            }
            badge.tag = 999
            button.addSubview(badge)

            NSLayoutConstraint.activate([
                badge.trailingAnchor.constraint(equalTo: button.trailingAnchor, constant: -2),
                badge.bottomAnchor.constraint(equalTo: button.bottomAnchor, constant: -2)
            ])
        }
    }

    // 创建数字徽章
    private func createBadge(text: String) -> NSTextField {
        let badge = NSTextField()
        badge.stringValue = text
        badge.isEditable = false
        badge.isBordered = false
        badge.backgroundColor = NSColor.systemRed
        badge.textColor = NSColor.white
        badge.font = NSFont.systemFont(ofSize: 8, weight: .bold)
        badge.alignment = .center
        badge.wantsLayer = true
        badge.layer?.cornerRadius = 6
        badge.translatesAutoresizingMaskIntoConstraints = false

        // 设置固定大小
        badge.widthAnchor.constraint(equalToConstant: 12).isActive = true
        badge.heightAnchor.constraint(equalToConstant: 12).isActive = true

        return badge
    }

    // 设置连接状态
    func setConnectionStatus(_ status: ConnectionStatus) {
        DispatchQueue.main.async { [weak self] in
            self?.connectionStatus = status
            self?.updateStatusIcon()
        }
    }

    // 设置设备数量
    func setDeviceCount(_ count: Int) {
        DispatchQueue.main.async { [weak self] in
            self?.deviceCount = count
            self?.updateStatusIcon()
        }
    }

    // 设置监控状态
    func setMonitoring(_ monitoring: Bool) {
        DispatchQueue.main.async { [weak self] in
            self?.isMonitoring = monitoring
        }
    }

    // 显示通知
    func showNotification(title: String, message: String) {
        let notification = NSUserNotification()
        notification.title = title
        notification.informativeText = message
        notification.soundName = NSUserNotificationDefaultSoundName

        NSUserNotificationCenter.default.deliver(notification)
    }
}

// 主内容视图
struct ContentView: View {
    @EnvironmentObject var menuBarManager: MenuBarManager
    @StateObject private var nearClipManager = NearClipManager()
    @State private var selectedTab = 0

    var body: some View {
        TabView(selection: $selectedTab) {
            // 设备发现页面
            DeviceDiscoveryView()
                .environmentObject(nearClipManager)
                .tabItem {
                    Image(systemName: "magnifyingglass")
                    Text("发现")
                }
                .tag(0)

            // 设备管理页面
            DeviceManagementView()
                .environmentObject(nearClipManager)
                .tabItem {
                    Image(systemName: "list.bullet")
                    Text("设备")
                }
                .tag(1)

            // 设置页面
            SettingsView()
                .tabItem {
                    Image(systemName: "gear")
                    Text("设置")
                }
                .tag(2)
        }
        .frame(width: 320, height: 480)
        .onAppear {
            // 设置回调以更新菜单栏状态
            nearClipManager.$connectedDevices
                .receive(on: DispatchQueue.main)
                .sink { devices in
                    menuBarManager.setDeviceCount(devices.filter { $0.connectionStatus == .connected }.count)
                }
                .store(in: &cancellables)

            nearClipManager.$isDiscovering
                .receive(on: DispatchQueue.main)
                .sink { isDiscovering in
                    menuBarManager.setMonitoring(isDiscovering)
                }
                .store(in: &cancellables)
        }
    }

    @State private var cancellables = Set<AnyCancellable>()
}

// Combine import
import Combine