//
//  NearClipApp.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import SwiftUI

@main
struct NearClipApp: App {
    @StateObject private var bleService = BLEService()
    @StateObject private var clipboardService = ClipboardService()
    @StateObject private var peripheralService = BLEPeripheralService()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(bleService)
                .environmentObject(clipboardService)
                .environmentObject(peripheralService)
                .frame(minWidth: 600, minHeight: 400)
                .onAppear {
                    // 启动外围设备服务用于测试
                    peripheralService.startAdvertising()
                }
        }
        .windowStyle(.hiddenTitleBar)
        .windowResizability(.contentSize)
    }
}