//
//  DeviceDiscoveryView.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import SwiftUI

struct DeviceDiscoveryView: View {
    @EnvironmentObject var bleService: BLEService
    @State private var showingPairAlert = false
    @State private var deviceToPair: BLEDevice?
    
    var body: some View {
        VStack(spacing: 0) {
            // 扫描控制区域
            ScanControlView()
                .padding()
            
            Divider()
            
            // 设备列表
            DeviceListView()
        }
        .alert("配对设备", isPresented: $showingPairAlert) {
            Button("取消", role: .cancel) { }
            Button("配对") {
                if let device = deviceToPair {
                    bleService.pairDevice(device)
                    bleService.connectToDevice(device)
                }
            }
        } message: {
            if let device = deviceToPair {
                Text("是否要配对设备 \"\(device.name)\"？")
            }
        }
    }
    
    private func pairDevice(_ device: BLEDevice) {
        deviceToPair = device
        showingPairAlert = true
    }
}

struct ScanControlView: View {
    @EnvironmentObject var bleService: BLEService
    
    var body: some View {
        VStack(spacing: 16) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("设备发现")
                        .font(.title2)
                        .fontWeight(.semibold)
                    
                    if bleService.isScanning {
                        Text("正在扫描附近的 NearClip 设备...")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    } else {
                        Text("点击开始扫描按钮发现附近的设备")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
                
                Spacer()
                
                Button(action: {
                    if bleService.isScanning {
                        bleService.stopScanning()
                    } else {
                        bleService.startScanning()
                    }
                }) {
                    HStack(spacing: 8) {
                        if bleService.isScanning {
                            ProgressView()
                                .scaleEffect(0.8)
                            Text("停止扫描")
                        } else {
                            Image(systemName: "magnifyingglass")
                            Text("开始扫描")
                        }
                    }
                }
                .buttonStyle(.borderedProminent)
                .disabled(!bleService.isBluetoothEnabled)
            }
            
            // 错误信息显示
            if let error = bleService.lastError {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Spacer()
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 8)
                .background(.orange.opacity(0.1))
                .cornerRadius(8)
            }
        }
    }
}

struct DeviceListView: View {
    @EnvironmentObject var bleService: BLEService
    
    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // 列表头部
            HStack {
                Text("发现的设备 (\(bleService.discoveredDevices.count))")
                    .font(.headline)
                    .padding(.horizontal)
                    .padding(.top)
                
                Spacer()
            }
            
            if bleService.discoveredDevices.isEmpty {
                // 空状态
                EmptyDeviceListView()
            } else {
                // 设备列表
                List(bleService.discoveredDevices, id: \.id) { device in
                    DeviceRowView(device: device)
                }
                .listStyle(.plain)
            }
        }
    }
}

struct EmptyDeviceListView: View {
    @EnvironmentObject var bleService: BLEService
    
    var body: some View {
        VStack(spacing: 16) {
            Spacer()
            
            if bleService.isScanning {
                VStack(spacing: 12) {
                    ProgressView()
                        .scaleEffect(1.2)
                    Text("正在扫描设备...")
                        .font(.title3)
                        .foregroundColor(.secondary)
                    Text("请确保其他设备已开启 NearClip 并处于可发现状态")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
            } else if !bleService.isBluetoothEnabled {
                VStack(spacing: 12) {
                    Image(systemName: "bluetooth.slash")
                        .font(.system(size: 48))
                        .foregroundColor(.secondary)
                    Text("蓝牙未启用")
                        .font(.title3)
                        .foregroundColor(.secondary)
                    Text("请在系统设置中启用蓝牙")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            } else {
                VStack(spacing: 12) {
                    Image(systemName: "antenna.radiowaves.left.and.right")
                        .font(.system(size: 48))
                        .foregroundColor(.secondary)
                    Text("未发现设备")
                        .font(.title3)
                        .foregroundColor(.secondary)
                    Text("点击开始扫描按钮搜索附近的 NearClip 设备")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
            }
            
            Spacer()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

struct DeviceRowView: View {
    let device: BLEDevice
    @EnvironmentObject var bleService: BLEService
    @State private var showingPairAlert = false
    
    private var isAlreadyPaired: Bool {
        bleService.pairedDevices.contains { $0.id == device.id }
    }
    
    private var isConnected: Bool {
        bleService.connectedDevice?.id == device.id
    }
    
    private var signalStrengthIcon: String {
        switch device.rssi {
        case -50...0:
            return "wifi"
        case -70..<(-50):
            return "wifi.circle"
        case -90..<(-70):
            return "wifi.circle.fill"
        default:
            return "wifi.slash"
        }
    }
    
    private var signalStrengthColor: Color {
        switch device.rssi {
        case -50...0:
            return .green
        case -70..<(-50):
            return .orange
        case -90..<(-70):
            return .red
        default:
            return .gray
        }
    }
    
    var body: some View {
        HStack(spacing: 12) {
            // 设备图标
            VStack {
                Image(systemName: "iphone")
                    .font(.title2)
                    .foregroundColor(.blue)
            }
            
            // 设备信息
            VStack(alignment: .leading, spacing: 4) {
                Text(device.name)
                    .font(.headline)
                    .lineLimit(1)
                
                HStack(spacing: 8) {
                    Text(device.id.prefix(8) + "...")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    HStack(spacing: 4) {
                        Image(systemName: signalStrengthIcon)
                            .font(.caption)
                            .foregroundColor(signalStrengthColor)
                        Text("\(device.rssi) dBm")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }
            
            Spacer()
            
            // 状态和操作按钮
            HStack(spacing: 8) {
                if isConnected {
                    Text("已连接")
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(.green.opacity(0.2))
                        .foregroundColor(.green)
                        .cornerRadius(4)
                } else if isAlreadyPaired {
                    Button("连接") {
                        bleService.connectToDevice(device)
                    }
                    .buttonStyle(.bordered)
                    .controlSize(.small)
                } else {
                    Button("配对") {
                        showingPairAlert = true
                    }
                    .buttonStyle(.borderedProminent)
                    .controlSize(.small)
                }
            }
        }
        .padding(.vertical, 8)
        .alert("配对设备", isPresented: $showingPairAlert) {
            Button("取消", role: .cancel) { }
            Button("配对") {
                bleService.pairDevice(device)
                bleService.connectToDevice(device)
            }
        } message: {
            Text("是否要配对设备 \"\(device.name)\"？\n\n配对后，两台设备可以自动同步剪贴板内容。")
        }
    }
}

#Preview {
    DeviceDiscoveryView()
        .environmentObject(BLEService())
        .frame(width: 600, height: 400)
}