# Story 6.1: 实现同步成功通知

Status: done

## Story

As a 用户,
I want 同步成功时收到低调提示,
So that 知道内容已到达.

## Acceptance Criteria

1. **Given** 剪贴板内容同步成功 **When** 目标设备收到内容 **Then** 显示系统通知（可配置）
2. **And** 通知内容简洁（如"已同步: 来自 Mac"）
3. **And** 通知自动消失（3秒）
4. **And** 可在设置中关闭通知

## Tasks / Subtasks

- [x] Task 1: macOS 通知实现 (AC: 1, 2, 3)
  - [x] 1.1 请求通知权限
  - [x] 1.2 发送 UNUserNotification
  - [x] 1.3 配置自动消失时间

- [x] Task 2: Android 通知实现 (AC: 1, 2, 3)
  - [x] 2.1 创建通知渠道
  - [x] 2.2 发送同步成功通知
  - [x] 2.3 配置自动消失

- [x] Task 3: 设置集成 (AC: 4)
  - [x] 3.1 macOS 设置项：启用/禁用通知
  - [x] 3.2 Android 设置项：启用/禁用通知
  - [x] 3.3 持久化设置

## Dev Notes

### macOS 通知

使用 `UserNotifications` 框架:
```swift
import UserNotifications

// 请求权限
UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound])

// 发送通知
let content = UNMutableNotificationContent()
content.title = "NearClip"
content.body = "已同步: 来自 \(deviceName)"
let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: nil)
```

### Android 通知

使用 `NotificationCompat`:
```kotlin
val notification = NotificationCompat.Builder(context, CHANNEL_ID)
    .setSmallIcon(R.drawable.ic_sync)
    .setContentTitle("NearClip")
    .setContentText("已同步: 来自 $deviceName")
    .setAutoCancel(true)
    .setTimeoutAfter(3000)
    .build()
```

### 与其他 Story 的关系

- Story 4-5 (macOS): 剪贴板写入时触发通知
- Story 5-5 (Android): 剪贴板写入时触发通知
- Story 6-2: 失败通知使用相同的通知机制

## Checklist

- [x] All tasks completed
- [x] macOS notifications working
- [x] Android notifications working
- [x] Settings toggle works
- [x] Story file updated to 'done'

## Implementation Summary

### macOS

**NotificationManager.swift** - 单例管理器:
- `requestAuthorization()`: 请求通知权限
- `showSyncSuccessNotification(fromDevice:contentPreview:)`: 显示同步成功通知
- `showSyncFailureNotification(toDevice:reason:)`: 显示同步失败通知（供 Story 6-2 使用）
- 使用 `UNUserNotificationCenter` 和 `UNMutableNotificationContent`
- 设置为 `UNUserNotificationCenterDelegate` 以支持前台通知显示

**ConnectionManager.swift** - 集成:
- 在 `handleClipboardReceived` 中调用 `NotificationManager.shared.showSyncSuccessNotification()`

**SettingsView.swift** - 设置:
- 添加 `syncNotificationsEnabled` 开关 (使用 @AppStorage)

**AppDelegate.swift** - 初始化:
- `setupNotifications()` 初始化 NotificationManager 并设置默认值

### Android

**NotificationHelper.kt** - 通知助手:
- 创建独立的通知渠道 `SYNC_NOTIFICATION_CHANNEL_ID`
- `showSyncSuccessNotification(fromDevice:contentPreview:)`: 显示成功通知
- `showSyncFailureNotification(toDevice:reason:)`: 显示失败通知
- 使用 `NotificationCompat.Builder` 和 `setTimeoutAfter(3000)` 自动消失

**NearClipService.kt** - 集成:
- 添加 `notificationHelper` 字段
- 在 `onClipboardReceived` 中调用 `notificationHelper?.showSyncSuccessNotification()`

**SettingsRepository.kt / SettingsScreen.kt** - 设置:
- 已有 `syncNotifications` 设置项和 UI 开关
