# Story 4.5: 实现剪贴板写入

Status: done

## Story

As a macOS 用户,
I want 接收到远程剪贴板内容时自动写入本地剪贴板,
So that 可以直接粘贴使用.

## Acceptance Criteria

1. **Given** 收到远程剪贴板内容 **When** 内容有效 **Then** 写入本地剪贴板
2. **And** 写入前标记为远程内容（防止回传）
3. **And** 写入成功后更新 UI 状态
4. **And** 测试验证写入功能

## Tasks / Subtasks

- [x] Task 1: 创建 ClipboardWriter 类 (AC: 1, 2)
  - [x] 1.1 使用 NSPasteboard.general 写入内容
  - [x] 1.2 写入前调用 ClipboardMonitor.markAsRemote()
  - [x] 1.3 支持纯文本写入 (UTF-8 String)

- [x] Task 2: 实现内容接收处理 (AC: 1, 3)
  - [x] 2.1 在 ConnectionManager 中更新 handleClipboardReceived() 方法
  - [x] 2.2 解析接收的 Data 为字符串
  - [x] 2.3 调用 ClipboardWriter 写入

- [x] Task 3: 实现 UI 反馈 (AC: 3)
  - [x] 3.1 写入成功后更新 lastSyncTime
  - [x] 3.2 syncing 状态动画反馈

- [x] Task 4: 集成模拟接收 (AC: 1, 2, 3, 4)
  - [x] 4.1 添加 Debug 测试按钮模拟接收内容
  - [x] 4.2 simulateReceiveClipboard() 方法

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 5.1 运行 swift build 成功
  - [x] 5.2 写入功能集成完成

## Dev Notes

### 架构约束

1. **NSPasteboard**: 使用 setString(_:forType:) 写入
2. **同步循环防护**: 必须在写入前调用 markAsRemote()
3. **线程安全**: 剪贴板操作需在主线程执行

### 与其他 Story 的关系

- Story 4-4: 提供 ClipboardMonitor 和 markAsRemote() 方法
- Story 3-10: Rust 端的同步循环防护（双重保护）

### 预期流程

```
远程设备 → Rust FFI → ConnectionManager.receiveClipboard()
                            ↓
                    ClipboardMonitor.markAsRemote()
                            ↓
                    ClipboardWriter.write()
                            ↓
                    NSPasteboard.general.setString()
```

### 实现的 ClipboardWriter

```swift
final class ClipboardWriter {
    static let shared = ClipboardWriter()

    @discardableResult
    func write(_ content: Data, markAsRemote: Bool = true) -> Bool {
        guard let string = String(data: content, encoding: .utf8) else {
            return false
        }
        return writeString(string, markAsRemote: markAsRemote, originalData: content)
    }

    @discardableResult
    func writeString(_ string: String, markAsRemote: Bool = true, originalData: Data? = nil) -> Bool {
        // Mark as remote before writing to prevent sync loop
        if markAsRemote {
            let data = originalData ?? string.data(using: .utf8)
            if let data = data {
                ClipboardMonitor.shared.markAsRemote(data)
            }
        }
        // Write to NSPasteboard.general
        return performWrite(string)
    }
}
```

### 文件结构

```
Sources/NearClip/
├── ClipboardWriter.swift    # NEW - 剪贴板写入
├── ClipboardMonitor.swift   # 剪贴板监听
├── ConnectionManager.swift  # UPDATED - handleClipboardReceived
├── MenuBarView.swift        # UPDATED - Debug 测试按钮
└── ...
```

## Checklist

- [x] All tasks completed
- [x] Clipboard write works (NSPasteboard)
- [x] Remote content marked (no sync loop)
- [x] UI updated after write
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
