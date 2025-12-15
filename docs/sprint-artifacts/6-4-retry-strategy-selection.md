# Story 6.4: 实现重试策略选择

Status: done

## Story

As a 用户,
I want 选择同步失败后的处理方式,
So that 按我的偏好处理.

## Acceptance Criteria

1. **Given** 同步重试 3 次失败 **When** 显示策略选择 **Then** 可选择"丢弃"（放弃本次）
2. **And** 可选择"等待"（设备上线后发送）
3. **And** 可选择"继续重试"
4. **And** 可设置默认策略

## Tasks / Subtasks

- [x] Task 1: 定义重试策略枚举 (AC: 1, 2, 3)
  - [x] 1.1 macOS: 定义 SyncRetryStrategy enum in ConnectionManager.swift
  - [x] 1.2 Android: 定义 SyncRetryStrategy enum in SettingsRepository.kt

- [x] Task 2: 设置中添加默认策略选项 (AC: 4)
  - [x] 2.1 macOS: SettingsView SyncSettingsTab 添加重试策略 Picker
  - [x] 2.2 Android: SettingsScreen 添加 RetryStrategySelector

- [x] Task 3: 失败时显示策略选择 (AC: 1, 2, 3)
  - [x] 3.1 macOS: NotificationManager 失败通知包含 Retry/Wait/Discard 动作
  - [x] 3.2 Android: NotificationHelper 失败通知包含策略按钮

- [x] Task 4: 实现策略执行逻辑
  - [x] 4.1 丢弃：executeDiscardStrategy() 清除 pendingContent
  - [x] 4.2 等待：executeWaitForDeviceStrategy() 保存到 pendingContent
  - [x] 4.3 继续重试：executeContinueRetryStrategy() 触发 syncCurrentClipboard

## Dev Notes

### 重试策略

- **Discard (丢弃)**: 放弃本次同步，不再重试
- **WaitForDevice (等待)**: 保存内容，设备上线后自动发送
- **ContinueRetry (继续重试)**: 继续重试直到成功或用户取消

### 与其他 Story 的关系

- Story 3-9: 基于现有重试机制
- Story 6-2: 失败通知中集成策略选择

## Checklist

- [x] All tasks completed
- [x] Default strategy configurable in settings
- [x] Strategy selection shown on failure
- [x] All strategies work correctly
- [x] Story file updated to 'done'

## Implementation Summary

### macOS

**ConnectionManager.swift**:
- 添加 `SyncRetryStrategy` enum (discard, waitForDevice, continueRetry)
- 添加 `pendingContent` 存储待发送内容
- 添加 `defaultRetryStrategy` 属性从 UserDefaults 读取
- 添加策略执行方法：`executeDiscardStrategy()`, `executeWaitForDeviceStrategy()`, `executeContinueRetryStrategy()`
- `handleDeviceConnected` 中调用 `sendPendingContentIfNeeded()` 自动发送等待的内容

**SettingsView.swift**:
- SyncSettingsTab 添加 "On Sync Failure" 部分
- Picker 选择默认重试策略
- 显示策略描述

**NotificationManager.swift**:
- 添加 Discard 和 Wait for Device 通知动作
- 更新失败通知类别包含三个选项
- 实现动作回调

**AppDelegate.swift**:
- 设置三个通知动作的回调处理

### Android

**SettingsRepository.kt**:
- 添加 `SyncRetryStrategy` enum
- 添加 `defaultRetryStrategy` 到 NearClipSettings
- 添加 `setDefaultRetryStrategy()` 方法

**SettingsViewModel.kt**:
- 添加 `setDefaultRetryStrategy()` 方法

**SettingsScreen.kt**:
- 添加 "On Sync Failure" 设置部分
- 添加 `RetryStrategySelector` 下拉选择器

**NotificationHelper.kt**:
- 添加 Discard 和 Wait 动作常量
- 失败通知包含 Retry/Wait/Discard 三个按钮

**ConnectionManager.kt**:
- 添加 `pendingContent` 存储
- 添加 `defaultRetryStrategy` 属性
- 添加策略执行方法
- `onDeviceConnected` 中发送等待的内容

**NearClipService.kt**:
- 添加 `pendingContent` 存储
- 处理通知动作 Intent
- `onDeviceConnected` 中发送等待的内容
