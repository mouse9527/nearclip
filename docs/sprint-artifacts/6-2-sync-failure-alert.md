# Story 6.2: 实现同步失败提示

Status: done

## Story

As a 用户,
I want 同步失败时被告知,
So that 可以采取补救措施.

## Acceptance Criteria

1. **Given** 剪贴板同步失败（重试耗尽） **When** 系统检测到失败 **Then** 显示失败通知
2. **And** 通知包含失败原因
3. **And** 提供重试选项
4. **And** 可点击查看详情

## Tasks / Subtasks

- [x] Task 1: macOS 失败通知 (AC: 1, 2, 4)
  - [x] 1.1 在 ConnectionManager 中检测同步错误
  - [x] 1.2 调用 NotificationManager.showSyncFailureNotification
  - [x] 1.3 点击通知打开应用

- [x] Task 2: Android 失败通知 (AC: 1, 2, 4)
  - [x] 2.1 在 NearClipService 中检测同步错误
  - [x] 2.2 调用 NotificationHelper.showSyncFailureNotification
  - [x] 2.3 点击通知打开应用

- [x] Task 3: 重试选项 (AC: 3)
  - [x] 3.1 macOS: 通知操作按钮
  - [x] 3.2 Android: 通知操作按钮

## Dev Notes

### 失败场景

- 网络超时
- 设备断开连接
- 重试耗尽（3次重试后）
- 加密/解密失败

### 与其他 Story 的关系

- Story 6-1: 复用通知基础设施
- Story 3-9: 重试机制触发失败通知

## Checklist

- [x] All tasks completed
- [x] Failure notifications shown on both platforms
- [x] Retry action works
- [x] Story file updated to 'done'

## Implementation Summary

### macOS

**NotificationManager.swift**:
- 添加 notification category (`SYNC_FAILURE`) 和 action (`RETRY_SYNC`)
- `setupNotificationCategories()`: 注册通知类别和操作
- `showSyncFailureNotification()`: 设置 `categoryIdentifier` 启用重试按钮
- `userNotificationCenter(_:didReceive:)`: 处理重试操作回调
- `onRetryRequested` 闭包: 供外部设置重试回调

**ConnectionManager.swift**:
- `handleSyncError()`: 调用 `NotificationManager.shared.showSyncFailureNotification()`

**ClipboardMonitor.swift**:
- 添加 `syncCurrentClipboard()` 方法用于重试

**AppDelegate.swift**:
- 设置 `onRetryRequested` 回调连接到 `clipboardMonitor?.syncCurrentClipboard()`

### Android

**NotificationHelper.kt**:
- 添加 `ACTION_RETRY_SYNC` 常量
- `showSyncFailureNotification()`: 添加 Retry 操作按钮
- 使用 `NearClipService.ACTION_SYNC_NOW` 触发重试

**NearClipService.kt**:
- `onSyncError()`: 调用 `notificationHelper?.showSyncFailureNotification()`
- 已有 `ACTION_SYNC_NOW` 处理重试
