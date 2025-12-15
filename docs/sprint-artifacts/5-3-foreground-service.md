# Story 5.3: 实现 Foreground Service

Status: done

## Story

As a Android 用户,
I want 应用在后台持续运行,
So that 剪贴板同步不会被系统杀死.

## Acceptance Criteria

1. **Given** 应用启动 **When** 开启同步 **Then** 启动前台服务
2. **And** 通知栏显示持久通知
3. **And** 应用切到后台时服务继续运行
4. **And** 用户可以从通知停止服务

## Tasks / Subtasks

- [x] Task 1: 创建 Foreground Service (AC: 1, 3)
  - [x] 1.1 创建 NearClipService 继承 Service
  - [x] 1.2 实现 startForeground()
  - [x] 1.3 在 Service 中管理 FfiNearClipManager

- [x] Task 2: 创建通知 (AC: 2, 4)
  - [x] 2.1 创建 NotificationChannel
  - [x] 2.2 创建持久通知
  - [x] 2.3 添加停止按钮 action

- [x] Task 3: 集成到 UI (AC: 1, 4)
  - [x] 3.1 HomeScreen 开关启动/停止服务
  - [x] 3.2 处理通知按钮点击

- [x] Task 4: 更新 AndroidManifest (AC: 1)
  - [x] 4.1 声明 Service
  - [x] 4.2 foregroundServiceType="dataSync"

## Dev Notes

### 架构约束

1. **Android 8.0+**: 需要 NotificationChannel
2. **Android 12+**: 需要声明 foregroundServiceType
3. **FOREGROUND_SERVICE_DATA_SYNC**: 数据同步类型

### 与其他 Story 的关系

- Story 5-2: App Framework 提供 UI
- Story 5-4: 剪贴板监听将在 Service 中运行

## Checklist

- [x] All tasks completed
- [x] Service starts and stops correctly
- [x] Notification displays
- [x] Background operation works
- [x] Story file updated to 'done'

## Implementation Summary

### NearClipService

位置: `service/NearClipService.kt`

功能:
- 继承 `Service`，实现 `FfiNearClipCallback`
- `startForeground()` 显示持久通知
- 管理 `FfiNearClipManager` 生命周期
- `ServiceListener` 接口通知 UI 更新
- `LocalBinder` 支持本地绑定

### 通知

- Channel ID: `nearclip_sync_channel`
- 显示连接设备数量
- "Stop" 按钮停止服务
- 点击通知打开应用

### AndroidManifest

```xml
<service
    android:name=".service.NearClipService"
    android:foregroundServiceType="dataSync" />
```

### UI 集成

HomeScreen 开关触发:
- 开启: `NearClipService.startService(context)`
- 关闭: `NearClipService.stopService(context)`
