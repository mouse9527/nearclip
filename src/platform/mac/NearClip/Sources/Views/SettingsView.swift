import SwiftUI

struct SettingsView: View {
    @State private var autoStartEnabled = true
    @State private var notificationsEnabled = true
    @State private var syncOnWifiOnly = false
    @State private var showStatusInMenuBar = true

    var body: some View {
        VStack(spacing: 20) {
            // 标题
            Text("设置")
                .font(.headline)
                .padding(.top)

            // 设置选项
            VStack(alignment: .leading, spacing: 16) {
                // 通用设置
                VStack(alignment: .leading, spacing: 12) {
                    Text("通用")
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    Toggle("开机自动启动", isOn: $autoStartEnabled)
                    Toggle("显示通知", isOn: $notificationsEnabled)
                    Toggle("在菜单栏显示状态", isOn: $showStatusInMenuBar)
                }

                Divider()

                // 同步设置
                VStack(alignment: .leading, spacing: 12) {
                    Text("同步")
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    Toggle("仅在Wi-Fi下同步", isOn: $syncOnWifiOnly)
                }

                Divider()

                // 关于
                VStack(alignment: .leading, spacing: 8) {
                    Text("关于")
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    HStack {
                        Text("版本:")
                        Text("1.0.0")
                            .foregroundColor(.secondary)
                    }

                    HStack {
                        Text("构建:")
                        Text("2025.10.17")
                            .foregroundColor(.secondary)
                    }
                }
            }
            .padding(.horizontal)

            Spacer()

            // 底部按钮
            VStack(spacing: 12) {
                Button("检查更新") {
                    // TODO: 实现更新检查
                }
                .buttonStyle(.bordered)

                Button("退出应用") {
                    NSApplication.shared.terminate(nil)
                }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
            }
            .padding(.bottom)
        }
        .onChange(of: autoStartEnabled) { enabled in
            Logger.shared.info("自动启动设置变更: \(enabled)")
            // TODO: 实现自动启动功能
        }
        .onChange(of: notificationsEnabled) { enabled in
            Logger.shared.info("通知设置变更: \(enabled)")
            // TODO: 实现通知权限设置
        }
        .onChange(of: showStatusInMenuBar) { enabled in
            Logger.shared.info("菜单栏状态显示设置变更: \(enabled)")
            // TODO: 实现菜单栏状态显示
        }
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}