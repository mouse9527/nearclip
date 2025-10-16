//
//  ContentView.swift
//  NearClip
//
//  NearClip 主界面视图
//

import SwiftUI
import CoreBluetooth
import OSLog

struct ContentView: View {
    @EnvironmentObject var nearClipCore: NearClipCore

    // 状态管理
    @State private var isScanning = false
    @State private var discoveredDevices: [BLEDevice] = []
    @State private var connectedDevices: [BLEDevice] = []
    @State private var selectedTab = 0

    // 系统日志
    private let logger = Logger(subsystem: "com.nearclip.NearClip", category: "ContentView")

    var body: some View {
        TabView(selection: $selectedTab) {
            // 设备标签页
            DeviceListView(
                isScanning: $isScanning,
                discoveredDevices: $discoveredDevices,
                connectedDevices: $connectedDevices,
                onStartScan: startDeviceScan,
                onStopScan: stopDeviceScan,
                onConnectDevice: connectToDevice,
                onDisconnectDevice: disconnectFromDevice
            )
            .tabItem {
                Label("设备", systemImage: "iphone.3")
            }
            .tag(0)

            // 同步标签页
            SyncView()
                .tabItem {
                    Label("同步", systemImage: "arrow.triangle.2.circlepath")
                }
                .tag(1)

            // 设置标签页
            SettingsView()
                .tabItem {
                    Label("设置", systemImage: "gear")
                }
                .tag(2)
        }
        .frame(minWidth: 600, minHeight: 400)
        .onAppear {
            loadInitialData()
        }
    }

    // MARK: - 设备管理

    private func loadInitialData() {
        logger.info("Loading initial device data...")

        Task {
            do {
                // 获取已连接的设备
                let devices = try await nearClipCore.getBLEManager().getConnectedDevices()
                await MainActor.run {
                    self.connectedDevices = devices
                }
                logger.info("Loaded \(devices.count) connected devices")
            } catch {
                logger.error("Failed to load initial device data: \(error.localizedDescription)")
            }
        }
    }

    private func startDeviceScan() {
        guard !isScanning else { return }

        logger.info("Starting device scan...")
        isScanning = true
        discoveredDevices.removeAll()

        Task {
            do {
                let devices = try await nearClipCore.getBLEManager().startScan(timeout: 30)

                await MainActor.run {
                    self.discoveredDevices = devices
                    self.isScanning = false
                }

                logger.info("Device scan completed. Found \(devices.count) devices")
            } catch {
                await MainActor.run {
                    self.isScanning = false
                }
                logger.error("Device scan failed: \(error.localizedDescription)")
            }
        }
    }

    private func stopDeviceScan() {
        guard isScanning else { return }

        logger.info("Stopping device scan...")
        nearClipCore.getBLEManager().stopScan()
        isScanning = false
    }

    private func connectToDevice(_ device: BLEDevice) {
        logger.info("Connecting to device: \(device.name)")

        Task {
            do {
                try await nearClipCore.getBLEManager().connectToDevice(device.id)

                await MainActor.run {
                    // 从发现列表移除
                    discoveredDevices.removeAll { $0.id == device.id }
                    // 添加到连接列表
                    if !connectedDevices.contains(where: { $0.id == device.id }) {
                        connectedDevices.append(device)
                    }
                }

                logger.info("Successfully connected to device: \(device.name)")
            } catch {
                logger.error("Failed to connect to device \(device.name): \(error.localizedDescription)")
            }
        }
    }

    private func disconnectFromDevice(_ device: BLEDevice) {
        logger.info("Disconnecting from device: \(device.name)")

        Task {
            do {
                try await nearClipCore.getBLEManager().disconnectFromDevice(device.id)

                await MainActor.run {
                    // 从连接列表移除
                    connectedDevices.removeAll { $0.id == device.id }
                }

                logger.info("Successfully disconnected from device: \(device.name)")
            } catch {
                logger.error("Failed to disconnect from device \(device.name): \(error.localizedDescription)")
            }
        }
    }
}

// MARK: - 设备列表视图

struct DeviceListView: View {
    @Binding var isScanning: Bool
    @Binding var discoveredDevices: [BLEDevice]
    @Binding var connectedDevices: [BLEDevice]

    let onStartScan: () -> Void
    let onStopScan: () -> Void
    let onConnectDevice: (BLEDevice) -> Void
    let onDisconnectDevice: (BLEDevice) -> Void

    var body: some View {
        NavigationSplitView {
            // 设备列表
            List {
                Section("已连接设备") {
                    ForEach(connectedDevices, id: \.id) { device in
                        DeviceRow(
                            device: device,
                            isConnected: true,
                            action: { onDisconnectDevice(device) }
                        )
                    }
                }

                Section("发现的设备") {
                    ForEach(discoveredDevices, id: \.id) { device in
                        DeviceRow(
                            device: device,
                            isConnected: false,
                            action: { onConnectDevice(device) }
                        )
                    }
                }
            }
            .navigationTitle("NearClip 设备")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button {
                        if isScanning {
                            onStopScan()
                        } else {
                            onStartScan()
                        }
                    } label: {
                        Image(systemName: isScanning ? "stop.circle.fill" : "magnifyingglass")
                    }
                }
            }
        } detail: {
            // 设备详情
            DeviceDetailView()
        }
    }
}

// MARK: - 设备行视图

struct DeviceRow: View {
    let device: BLEDevice
    let isConnected: Bool
    let action: () -> Void

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(device.name)
                    .font(.headline)
                Text(device.id)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            if isConnected {
                Label("已连接", systemImage: "checkmark.circle.fill")
                    .foregroundColor(.green)
            } else {
                Button("连接") {
                    action()
                }
                .buttonStyle(.bordered)
            }
        }
        .padding(.vertical, 4)
    }
}

// MARK: - 设备详情视图

struct DeviceDetailView: View {
    var body: some View {
        VStack {
            Image(systemName: "iphone.3")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("选择一个设备查看详情")
                .font(.title2)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

// MARK: - 同步视图

struct SyncView: View {
    var body: some View {
        VStack {
            Image(systemName: "arrow.triangle.2.circlepath")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("剪贴板同步功能")
                .font(.title2)
                .padding()

            Text("自动在连接的设备间同步剪贴板内容")
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

// MARK: - 设置视图

struct SettingsView: View {
    var body: some View {
        VStack {
            Image(systemName: "gear")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("应用设置")
                .font(.title2)
                .padding()

            Text("配置 NearClip 的各种选项")
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

// MARK: - 预览

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
            .environmentObject(NearClipCore())
            .frame(width: 800, height: 600)
    }
}