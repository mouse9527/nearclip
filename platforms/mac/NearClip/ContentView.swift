//
//  ContentView.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var bleService: BLEService
    @EnvironmentObject var clipboardService: ClipboardService
    @State private var selectedTab = 0
    
    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // 顶部状态栏
                StatusBarView()
                    .padding(.horizontal)
                    .padding(.top)
                
                Divider()
                
                // 主内容区域
                TabView(selection: $selectedTab) {
                    DeviceDiscoveryView()
                        .tabItem {
                            Image(systemName: "antenna.radiowaves.left.and.right")
                            Text("设备发现")
                        }
                        .tag(0)
                    
                    PairedDevicesView()
                        .tabItem {
                            Image(systemName: "link")
                            Text("已配对设备")
                        }
                        .tag(1)
                    
                    SettingsView()
                        .tabItem {
                            Image(systemName: "gear")
                            Text("设置")
                        }
                        .tag(2)
                }
            }
        }
        .navigationTitle("NearClip")
        .onAppear {
            bleService.startService()
            clipboardService.startMonitoring()
        }
        .onDisappear {
            bleService.stopService()
            clipboardService.stopMonitoring()
        }
    }
}

struct StatusBarView: View {
    @EnvironmentObject var bleService: BLEService
    @EnvironmentObject var clipboardService: ClipboardService
    
    var body: some View {
        HStack {
            // 蓝牙状态
            HStack(spacing: 8) {
                Circle()
                    .fill(bleService.isBluetoothEnabled ? .green : .red)
                    .frame(width: 8, height: 8)
                Text(bleService.isBluetoothEnabled ? "蓝牙已启用" : "蓝牙未启用")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            // 连接状态
            if let connectedDevice = bleService.connectedDevice {
                HStack(spacing: 8) {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                    Text("已连接: \(connectedDevice.name)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            } else {
                HStack(spacing: 8) {
                    Image(systemName: "exclamationmark.circle")
                        .foregroundColor(.orange)
                    Text("未连接")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            
            Spacer()
            
            // 剪贴板状态
            HStack(spacing: 8) {
                Circle()
                    .fill(clipboardService.isMonitoring ? .green : .gray)
                    .frame(width: 8, height: 8)
                Text(clipboardService.isMonitoring ? "监听中" : "已暂停")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 8)
    }
}

struct PairedDevicesView: View {
    @EnvironmentObject var bleService: BLEService
    
    var body: some View {
        VStack {
            if bleService.pairedDevices.isEmpty {
                VStack(spacing: 16) {
                    Image(systemName: "link.circle")
                        .font(.system(size: 48))
                        .foregroundColor(.secondary)
                    Text("暂无已配对设备")
                        .font(.title2)
                        .foregroundColor(.secondary)
                    Text("请前往设备发现页面配对新设备")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                List(bleService.pairedDevices, id: \.id) { device in
                    PairedDeviceRow(device: device)
                }
            }
        }
        .padding()
    }
}

struct PairedDeviceRow: View {
    let device: BLEDevice
    @EnvironmentObject var bleService: BLEService
    
    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(device.name)
                    .font(.headline)
                Text(device.id)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            if bleService.connectedDevice?.id == device.id {
                Text("已连接")
                    .font(.caption)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(.green.opacity(0.2))
                    .foregroundColor(.green)
                    .cornerRadius(4)
            } else {
                Button("连接") {
                    bleService.connectToDevice(device)
                }
                .buttonStyle(.bordered)
            }
            
            Button("取消配对") {
                bleService.unpairDevice(device)
            }
            .buttonStyle(.bordered)
            .foregroundColor(.red)
        }
        .padding(.vertical, 4)
    }
}

struct SettingsView: View {
    @EnvironmentObject var clipboardService: ClipboardService
    @State private var autoSync = true
    @State private var showNotifications = true
    
    var body: some View {
        Form {
            Section("同步设置") {
                Toggle("自动同步剪贴板", isOn: $autoSync)
                Toggle("显示同步通知", isOn: $showNotifications)
            }
            
            Section("关于") {
                HStack {
                    Text("版本")
                    Spacer()
                    Text("1.0.0")
                        .foregroundColor(.secondary)
                }
                
                HStack {
                    Text("构建版本")
                    Spacer()
                    Text("1")
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
        .environmentObject(BLEService())
        .environmentObject(ClipboardService())
}