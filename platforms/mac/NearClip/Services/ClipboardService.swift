//
//  ClipboardService.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import Foundation
import AppKit
import Combine

class ClipboardService: ObservableObject {
    // MARK: - Published Properties
    @Published var isMonitoring = false
    @Published var lastClipboardContent = ""
    @Published var syncCount = 0
    @Published var lastSyncTime: Date?
    
    // MARK: - Private Properties
    private var monitoringTimer: Timer?
    private var lastChangeCount: Int = 0
    private let pasteboard = NSPasteboard.general
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    init() {
        setupNotificationObservers()
        lastChangeCount = pasteboard.changeCount
        updateLastClipboardContent()
    }
    
    deinit {
        stopMonitoring()
    }
    
    // MARK: - Public Methods
    func startMonitoring() {
        guard !isMonitoring else { return }
        
        print("[ClipboardService] Starting clipboard monitoring")
        isMonitoring = true
        lastChangeCount = pasteboard.changeCount
        
        // 每0.5秒检查一次剪贴板变化
        monitoringTimer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { _ in
            self.checkClipboardChanges()
        }
    }
    
    func stopMonitoring() {
        guard isMonitoring else { return }
        
        print("[ClipboardService] Stopping clipboard monitoring")
        isMonitoring = false
        monitoringTimer?.invalidate()
        monitoringTimer = nil
    }
    
    func updateClipboard(with content: String) {
        // 暂停监听以避免循环触发
        let wasMonitoring = isMonitoring
        if wasMonitoring {
            stopMonitoring()
        }
        
        // 更新剪贴板内容
        pasteboard.clearContents()
        pasteboard.setString(content, forType: .string)
        
        // 更新本地状态
        DispatchQueue.main.async {
            self.lastClipboardContent = content
            self.lastChangeCount = self.pasteboard.changeCount
            self.lastSyncTime = Date()
            self.syncCount += 1
        }
        
        print("[ClipboardService] Clipboard updated with received content: \(content.prefix(50))...")
        
        // 延迟恢复监听
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            if wasMonitoring {
                self.startMonitoring()
            }
        }
    }
    
    func getCurrentClipboardContent() -> String? {
        return pasteboard.string(forType: .string)
    }
    
    // MARK: - Private Methods
    private func setupNotificationObservers() {
        // 监听从 BLE 服务接收到的剪贴板数据
        NotificationCenter.default
            .publisher(for: NSNotification.Name("ClipboardDataReceived"))
            .compactMap { $0.object as? String }
            .receive(on: DispatchQueue.main)
            .sink { [weak self] content in
                self?.updateClipboard(with: content)
            }
            .store(in: &cancellables)
    }
    
    private func checkClipboardChanges() {
        let currentChangeCount = pasteboard.changeCount
        
        // 检查剪贴板是否有变化
        guard currentChangeCount != lastChangeCount else { return }
        
        lastChangeCount = currentChangeCount
        
        guard let currentContent = pasteboard.string(forType: .string),
              !currentContent.isEmpty,
              currentContent != lastClipboardContent else {
            return
        }
        
        print("[ClipboardService] Clipboard content changed: \(currentContent.prefix(50))...")
        
        DispatchQueue.main.async {
            self.lastClipboardContent = currentContent
            self.lastSyncTime = Date()
        }
        
        // 通知 BLE 服务发送数据
        NotificationCenter.default.post(
            name: NSNotification.Name("ClipboardContentChanged"),
            object: currentContent
        )
    }
    
    private func updateLastClipboardContent() {
        if let content = pasteboard.string(forType: .string) {
            lastClipboardContent = content
        }
    }
}

// MARK: - Extensions
extension ClipboardService {
    var formattedLastSyncTime: String {
        guard let lastSyncTime = lastSyncTime else {
            return "从未同步"
        }
        
        let formatter = DateFormatter()
        formatter.dateStyle = .none
        formatter.timeStyle = .medium
        return formatter.string(from: lastSyncTime)
    }
    
    var clipboardPreview: String {
        if lastClipboardContent.isEmpty {
            return "剪贴板为空"
        }
        
        let preview = lastClipboardContent.prefix(100)
        return preview.count < lastClipboardContent.count ? "\(preview)..." : String(preview)
    }
}