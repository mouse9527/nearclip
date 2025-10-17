import SwiftUI

struct DeviceManagementView: View {
    @EnvironmentObject var nearClipManager: NearClipManager

    var body: some View {
        VStack(spacing: 16) {
            // 标题
            Text("设备管理")
                .font(.headline)
                .padding(.top)

            if nearClipManager.connectedDevices.isEmpty {
                // 空状态
                VStack(spacing: 12) {
                    Image(systemName: "antenna.radiowaves.left.and.right.slash")
                        .font(.largeTitle)
                        .foregroundColor(.secondary)

                    Text("暂无设备")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Text("请先在设备发现页面搜索并连接设备")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
                .padding()
            } else {
                // 设备列表
                ScrollView {
                    LazyVStack(spacing: 8) {
                        ForEach(nearClipManager.connectedDevices) { device in
                            DeviceRowView(device: device) {
                                connectToDevice(device)
                            } disconnectAction: {
                                disconnectFromDevice(device)
                            }
                        }
                    }
                    .padding(.horizontal)
                }
            }

            Spacer()
        }
    }

    private func connectToDevice(_ device: Device) {
        Task {
            do {
                try await nearClipManager.connectToDevice(device)
                Logger.shared.info("成功连接到设备: \(device.name)")
            } catch {
                Logger.shared.error("连接设备失败: \(error.localizedDescription)")
            }
        }
    }

    private func disconnectFromDevice(_ device: Device) {
        nearClipManager.disconnectFromDevice(device)
        Logger.shared.info("已断开设备连接: \(device.name)")
    }
}

// 设备行视图
struct DeviceRowView: View {
    let device: Device
    let connectAction: () -> Void
    let disconnectAction: () -> Void

    var body: some View {
        HStack {
            // 状态图标
            Image(systemName: device.statusIcon)
                .foregroundColor(device.statusColor)
                .font(.title2)

            VStack(alignment: .leading, spacing: 4) {
                // 设备名称
                Text(device.displayName)
                    .font(.subheadline)
                    .fontWeight(.medium)

                // 设备信息
                HStack {
                    Text(device.type.rawValue)
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 2)
                        .background(Color.secondary.opacity(0.2))
                        .cornerRadius(4)

                    if let batteryText = device.batteryText {
                        HStack(spacing: 2) {
                            Image(systemName: "battery.100")
                                .font(.caption)
                            Text(batteryText)
                                .font(.caption)
                        }
                        .foregroundColor(batteryColor)
                    }
                }
            }

            Spacer()

            // 连接按钮
            Button(action: {
                if device.connectionStatus == .connected {
                    disconnectAction()
                } else {
                    connectAction()
                }
            }) {
                Text(device.connectionStatus == .connected ? "断开" : "连接")
                    .font(.caption)
            }
            .buttonStyle(.bordered)
            .controlSize(.small)
        }
        .padding(.vertical, 8)
        .padding(.horizontal, 12)
        .background(Color.secondary.opacity(0.1))
        .cornerRadius(8)
    }

    private var batteryColor: Color {
        guard let level = device.batteryLevel else { return .secondary }

        if level > 50 {
            return .green
        } else if level > 20 {
            return .orange
        } else {
            return .red
        }
    }
}

struct DeviceManagementView_Previews: PreviewProvider {
    static var previews: some View {
        DeviceManagementView()
            .environmentObject(NearClipManager())
    }
}