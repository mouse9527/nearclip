# Story 4.4: 实现剪贴板监听

Status: done

## Story

As a macOS 用户,
I want 系统自动监听剪贴板变化,
So that 复制后自动同步.

## Acceptance Criteria

1. **Given** 应用已启动 **When** 用户复制内容到剪贴板 **Then** 检测到剪贴板变化
2. **And** 调用 Rust 核心发送内容
3. **And** 忽略远程同步写入的内容
4. **And** 测试验证监听准确

## Tasks / Subtasks

- [x] Task 1: 创建 ClipboardMonitor 类 (AC: 1)
  - [x] 1.1 使用 NSPasteboard 监听变化
  - [x] 1.2 使用 Timer 轮询 changeCount (0.5s 间隔)
  - [x] 1.3 检测到变化时提取内容

- [x] Task 2: 实现内容提取 (AC: 1, 2)
  - [x] 2.1 提取纯文本内容 (NSPasteboard.PasteboardType.string)
  - [x] 2.2 转换为 Data 格式 (UTF-8)
  - [x] 2.3 调用 ConnectionManager.syncClipboard() via callback

- [x] Task 3: 实现远程内容过滤 (AC: 3)
  - [x] 3.1 维护最近写入的内容 SHA256 哈希集合
  - [x] 3.2 检测到变化时比对哈希
  - [x] 3.3 匹配则忽略，不回传
  - [x] 3.4 自动清理旧哈希 (最多保留 100 个)

- [x] Task 4: 集成到应用生命周期 (AC: 1, 2, 3)
  - [x] 4.1 在 AppDelegate 中启动监听
  - [x] 4.2 在应用退出时停止监听
  - [x] 4.3 使用 configure(with:) 配置 ConnectionManager

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 5.1 运行 swift build 成功
  - [x] 5.2 ClipboardMonitor 集成完成

## Dev Notes

### 架构约束

1. **NSPasteboard**: macOS 剪贴板 API
2. **轮询方式**: 使用 Timer 检查 changeCount (无原生监听事件)
3. **内容类型**: 当前仅支持纯文本 (string)
4. **CryptoKit**: 使用 SHA256 计算内容哈希

### 与其他 Story 的关系

- Story 4-3: 提供 ConnectionManager 状态管理
- Story 4-5: 将实现剪贴板写入功能
- Story 3-10: Rust 端的同步循环防护

### 实现的 ClipboardMonitor

```swift
final class ClipboardMonitor: ObservableObject {
    static let shared = ClipboardMonitor()

    @Published private(set) var isMonitoring = false
    @Published private(set) var lastContent: String?

    private let pollingInterval: TimeInterval = 0.5
    private var timer: Timer?
    private var lastChangeCount: Int = 0
    private var remoteContentHashes: Set<String> = []

    var onClipboardChange: ((Data) -> Void)?

    func startMonitoring() { ... }
    func stopMonitoring() { ... }
    func markAsRemote(_ content: Data) { ... }
    func configure(with connectionManager: ConnectionManager) { ... }
}
```

### 防止同步循环

使用 SHA256 哈希标记远程写入的内容:
- `markAsRemote(_:)`: 写入剪贴板前调用，记录哈希
- `isRemoteContent(_:)`: 检测变化时调用，比对哈希
- 匹配的哈希会被从集合中移除（一次性标记）

### 文件结构

```
Sources/NearClip/
├── ClipboardMonitor.swift   # NEW - 剪贴板监听
├── AppDelegate.swift        # UPDATED - 集成监听
└── ...
```

## Checklist

- [x] All tasks completed
- [x] Clipboard changes detected (changeCount polling)
- [x] Content sent to ConnectionManager (via callback)
- [x] Remote content filtered (SHA256 hash)
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
