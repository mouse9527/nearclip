import SwiftUI

struct DeviceDiscoveryView: View {
    @EnvironmentObject var nearClipManager: NearClipManager
    @State private var isSearching = false

    var body: some View {
        VStack(spacing: 20) {
            // 标题
            Text("设备发现")
                .font(.headline)
                .padding(.top)

            // 搜索状态
            VStack(spacing: 12) {
                HStack {
                    if isSearching {
                        ProgressView()
                            .scaleEffect(0.8)
                        Text("正在搜索设备...")
                            .foregroundColor(.secondary)
                    } else {
                        Image(systemName: "magnifyingglass")
                            .foregroundColor(.secondary)
                        Text("点击开始搜索设备")
                            .foregroundColor(.secondary)
                    }
                }
                .font(.subheadline)

                // 搜索按钮
                Button(action: toggleDiscovery) {
                    HStack {
                        Image(systemName: isSearching ? "stop.circle.fill" : "play.circle.fill")
                        Text(isSearching ? "停止搜索" : "开始搜索")
                    }
                }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
            }

            // 搜索提示
            if !isSearching {
                VStack(alignment: .leading, spacing: 8) {
                    Text("搜索提示:")
                        .font(.caption)
                        .fontWeight(.semibold)

                    VStack(alignment: .leading, spacing: 4) {
                        Text("• 确保蓝牙已开启")
                        Text("• 确保设备在附近")
                        Text("• 确保设备可被发现")
                    }
                    .font(.caption)
                    .foregroundColor(.secondary)
                }
                .padding()
                .background(Color.secondary.opacity(0.1))
                .cornerRadius(8)
            }

            Spacer()
        }
        .padding()
        .onReceive(nearClipManager.$isDiscovering) { discovering in
            isSearching = discovering
        }
    }

    private func toggleDiscovery() {
        if isSearching {
            nearClipManager.stopDeviceDiscovery()
        } else {
            startDeviceDiscovery()
        }
    }

    private func startDeviceDiscovery() {
        Task {
            do {
                try nearClipManager.startDeviceDiscovery { device in
                    Logger.shared.info("发现设备: \(device.name)")
                }
            } catch {
                Logger.shared.error("启动设备发现失败: \(error.localizedDescription)")
            }
        }
    }
}

struct DeviceDiscoveryView_Previews: PreviewProvider {
    static var previews: some View {
        DeviceDiscoveryView()
            .environmentObject(NearClipManager())
    }
}