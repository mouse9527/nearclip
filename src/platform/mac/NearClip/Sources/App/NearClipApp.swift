import SwiftUI
import AppKit

@main
struct NearClipApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        Settings {
            EmptyView()
        }
    }
}

class AppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem?
    var popover: NSPopover?

    func applicationDidFinishLaunching(_ notification: Notification) {
        // 设置应用为后台应用（不显示在Dock中）
        NSApp.setActivationPolicy(.accessory)

        // 恢复应用状态
        restoreApplicationState()

        // 初始化菜单栏
        setupMenuBar()

        print("NearClip 应用已启动")
    }

    func applicationWillTerminate(_ notification: Notification) {
        // 清理资源
        cleanup()
        print("NearClip 应用即将退出")
    }

    func applicationWillUnhide(_ notification: Notification) {
        // 应用从后台回到前台
        print("NearClip 应用回到前台")
    }

    func applicationWillResignActive(_ notification: Notification) {
        // 应用即将进入后台
        print("NearClip 应用即将进入后台")
    }

    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // 用户点击Dock图标时显示菜单
        showPopover()
        return true
    }

    private func cleanup() {
        // 状态保存和恢复前的清理
        if popover?.isShown == true {
            popover?.performClose(nil)
        }
        statusItem = nil
        popover = nil

        // 保存应用状态（如果需要）
        saveApplicationState()
    }

    private func saveApplicationState() {
        // 实现状态保存逻辑
        let userDefaults = UserDefaults.standard
        userDefaults.set(Date(), forKey: "lastClosedDate")
        userDefaults.synchronize()
    }

    private func restoreApplicationState() {
        // 实现状态恢复逻辑
        let userDefaults = UserDefaults.standard
        if let lastClosedDate = userDefaults.object(forKey: "lastClosedDate") as? Date {
            print("上次关闭时间: \(lastClosedDate)")
        }
    }

    private func setupMenuBar() {
        // 创建菜单栏图标
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)

        if let button = statusItem?.button {
            // 使用系统图标
            button.image = NSImage(systemSymbolName: "antenna.radiowaves.left.and.right", accessibilityDescription: "NearClip")
            button.action = #selector(menuBarButtonClicked)
            button.target = self
        }
    }

    @objc private func menuBarButtonClicked() {
        // 切换弹出菜单显示状态
        if let popover = popover {
            if popover.isShown {
                popover.performClose(nil)
            } else {
                showPopover()
            }
        } else {
            showPopover()
        }
    }

    private func showPopover() {
        if popover == nil {
            popover = NSPopover()
            popover?.contentSize = NSSize(width: 320, height: 480)
            popover?.behavior = .transient
            popover?.contentViewController = NSHostingController(rootView: ContentView())
        }

        if let button = statusItem?.button {
            popover?.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
        }
    }
}

struct ContentView: View {
    var body: some View {
        NavigationView {
            VStack {
                Text("NearClip")
                    .font(.title)
                    .padding()

                Text("剪贴板同步工具")
                    .font(.subheadline)
                    .foregroundColor(.secondary)

                Divider()

                VStack(spacing: 20) {
                    HStack {
                        Image(systemName: "antenna.radiowaves.left.and.right")
                            .foregroundColor(.blue)
                        Text("设备发现")
                        Spacer()
                    }

                    HStack {
                        Image(systemName: "arrow.left.arrow.right")
                            .foregroundColor(.green)
                        Text("剪贴板同步")
                        Spacer()
                    }

                    HStack {
                        Image(systemName: "gear")
                            .foregroundColor(.gray)
                        Text("设置")
                        Spacer()
                    }
                }
                .padding()

                Spacer()

                Button("退出") {
                    NSApplication.shared.terminate(nil)
                }
                .buttonStyle(.bordered)
                .padding()
            }
            .frame(width: 280, height: 400)
        }
    }
}