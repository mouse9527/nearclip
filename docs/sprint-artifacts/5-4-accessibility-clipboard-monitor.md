# Story 5.4: 实现剪贴板监听

Status: done

## Story

As a Android 用户,
I want 应用监听剪贴板变化,
So that 复制的内容可以自动同步.

## Acceptance Criteria

1. **Given** 服务运行中 **When** 用户复制内容 **Then** 检测到剪贴板变化
2. **And** 自动发送到已连接设备
3. **And** 防止同步循环 (忽略远程写入的内容)
4. **And** 支持文本内容

## Tasks / Subtasks

- [x] Task 1: 创建 ClipboardMonitor (AC: 1, 3)
  - [x] 1.1 使用 ClipboardManager.OnPrimaryClipChangedListener
  - [x] 1.2 实现 sync loop prevention (hash tracking)
  - [x] 1.3 处理 Android 10+ 剪贴板限制

- [x] Task 2: 集成到 Service (AC: 1, 2)
  - [x] 2.1 在 NearClipService 中启动监听
  - [x] 2.2 剪贴板变化时调用 syncClipboard()

- [x] Task 3: 权限处理 (AC: 1)
  - [x] 3.1 前台时自动获取剪贴板访问
  - [x] 3.2 文档说明 Accessibility Service 方案

## Dev Notes

### Android 剪贴板限制

**Android 10+ (API 29+)**:
- 后台应用无法读取剪贴板
- 仅当应用在前台或有输入焦点时可读取
- 解决方案: 前台服务 + 用户主动触发

**Accessibility Service (可选)**:
- 可以在后台监听剪贴板
- 需要用户在系统设置中手动启用
- 更强大但设置复杂

### 架构约束

1. **ClipboardManager**: 系统剪贴板 API
2. **OnPrimaryClipChangedListener**: 变化监听
3. **Hash Tracking**: SHA256 防止同步循环

### 与其他 Story 的关系

- Story 5-3: 在 Foreground Service 中运行
- Story 5-5: 写入剪贴板时标记为远程

## Checklist

- [x] All tasks completed
- [x] Clipboard changes detected
- [x] Sync loop prevented
- [x] Works in foreground
- [x] Story file updated to 'done'

## Implementation Summary

### ClipboardMonitor

位置: `service/ClipboardMonitor.kt`

功能:
- 使用 `ClipboardManager.OnPrimaryClipChangedListener` 监听剪贴板变化
- SHA-256 hash tracking 防止同步循环
- `markAsRemote()` 标记远程内容
- `ignoreNextChange()` 写入时忽略下一次变化
- `syncCurrentClipboard()` 手动触发同步
- 处理 Android 10+ 剪贴板访问限制

### NearClipService 集成

- 在 `initializeManager()` 中初始化 ClipboardMonitor
- 剪贴板变化时调用 `syncClipboard()`
- `startSync()` / `stopSync()` 管理监听生命周期
- `onClipboardReceived()` 调用 `markAsRemote()` 防止循环
- `ACTION_SYNC_NOW` 支持手动同步

### Android 10+ 限制处理

由于 Android 10+ 限制后台剪贴板访问，当前实现:
1. 前台服务确保应用处于前台状态
2. 用户在应用内操作时可正常监听
3. 提供 "Sync Now" 按钮手动触发同步

### Accessibility Service (可选增强)

如需后台监听剪贴板，可实现 AccessibilityService:

```kotlin
class ClipboardAccessibilityService : AccessibilityService() {
    override fun onAccessibilityEvent(event: AccessibilityEvent) {
        if (event.eventType == AccessibilityEvent.TYPE_VIEW_TEXT_CHANGED) {
            // 检测剪贴板变化
        }
    }
}
```

需要用户在系统设置中手动启用，设置复杂度较高，作为可选增强功能。
