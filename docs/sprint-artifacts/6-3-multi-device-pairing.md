# Story 6.3: 实现多设备配对管理

Status: done

## Story

As a 用户,
I want 同时配对多台设备,
So that 内容可以同步到所有设备.

## Acceptance Criteria

1. **Given** 已配对至少一台设备 **When** 添加新设备配对 **Then** 新设备加入已配对列表
2. **And** 剪贴板同步到所有已配对设备
3. **And** 可独立管理每个设备（删除、暂停）
4. **And** 最多支持 5 台设备

## Tasks / Subtasks

- [x] Task 1: 最大设备数限制 (AC: 4)
  - [x] 1.1 macOS: ConnectionManager 添加 MAX_PAIRED_DEVICES = 5
  - [x] 1.2 macOS: addPairedDevice 时检查限制
  - [x] 1.3 Android: ConnectionManager 添加设备数限制
  - [x] 1.4 Android: addDeviceFromCode 时检查限制

- [x] Task 2: 暂停/恢复单个设备 (AC: 3)
  - [x] 2.1 macOS: DeviceDisplay 添加 isPaused 属性
  - [x] 2.2 macOS: ConnectionManager 添加 pauseDevice/resumeDevice 方法
  - [x] 2.3 macOS: SettingsView 添加暂停按钮
  - [x] 2.4 macOS: syncClipboard 跳过已暂停设备
  - [x] 2.5 Android: FfiDeviceInfo 扩展 isPaused 状态
  - [x] 2.6 Android: ConnectionManager 添加暂停功能
  - [x] 2.7 Android: SettingsScreen 添加暂停按钮

- [x] Task 3: UI 提示 (AC: 1, 4)
  - [x] 3.1 macOS: 达到上限时禁用添加设备按钮
  - [x] 3.2 Android: 达到上限时显示提示

## Dev Notes

### 设备管理逻辑

- 暂停状态仅在客户端维护，不影响 Rust 核心
- 暂停的设备不参与同步，但保持配对关系
- 设备数限制在添加设备时检查

### 与其他 Story 的关系

- Story 2-9: 使用已有的配对设备存储
- Story 4-8/5-9: Keychain/Keystore 存储暂停状态

## Checklist

- [x] All tasks completed
- [x] Maximum 5 devices enforced
- [x] Pause/resume works on both platforms
- [x] Story file updated to 'done'

## Implementation Summary

### macOS

**ConnectionManager.swift**:
- 添加 `maxPairedDevices = 5` 常量
- 添加 `pausedDeviceIds` 存储在 UserDefaults
- 添加 `canAddMoreDevices` 属性检查设备数量
- `pauseDevice()` / `resumeDevice()` 方法管理暂停状态
- `addPairedDevice()` 返回 Bool 并检查设备限制
- `syncClipboard()` 过滤已暂停设备

**DeviceDisplay**:
- 添加 `isPaused` 属性

**SettingsView.swift**:
- DevicesSettingsTab 显示设备数量 (N/5)
- DeviceSettingsRow 添加暂停/恢复按钮

**MenuBarView.swift**:
- MenuButton 支持 disabled 参数
- 达到设备上限时禁用 Add Device 按钮

### Android

**ConnectionManager.kt**:
- 添加 `MAX_PAIRED_DEVICES = 5` 常量
- 添加 `pausedDeviceIds` StateFlow
- 添加 `canAddMoreDevices` 属性
- `pauseDevice()` / `resumeDevice()` / `isDevicePaused()` 方法
- `addDeviceFromCode()` 检查设备限制
- `removeDevice()` 清理暂停状态

**SettingsScreen.kt**:
- 显示设备数量 (N/5)
- PairedDeviceItem 添加暂停/恢复按钮
- 暂停的设备显示 "(Paused)" 标签和透明度
