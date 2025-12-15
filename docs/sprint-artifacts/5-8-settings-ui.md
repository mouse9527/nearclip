# Story 5.8: 实现设置 UI

Status: done

## Story

As a Android 用户,
I want 配置同步选项,
So that 我可以自定义同步行为.

## Acceptance Criteria

1. **Given** 设置页面 **When** 查看设置 **Then** 显示当前配置
2. **And** 可以开关 WiFi/蓝牙同步
3. **And** 可以开关自动连接
4. **And** 可以管理已配对设备
5. **And** 设置持久化保存

## Tasks / Subtasks

- [x] Task 1: 基础 UI (AC: 1, 2, 3) - 已在 5-2 完成
  - [x] 1.1 SettingsScreen 布局
  - [x] 1.2 SettingsSwitch 组件
  - [x] 1.3 SettingsItem 组件

- [x] Task 2: 设置持久化 (AC: 5)
  - [x] 2.1 创建 SettingsRepository
  - [x] 2.2 使用 DataStore 存储设置
  - [x] 2.3 ViewModel 连接 Repository

- [x] Task 3: 设备管理 (AC: 4)
  - [x] 3.1 显示已配对设备列表
  - [x] 3.2 删除设备功能
  - [x] 3.3 确认对话框

## Dev Notes

### DataStore 依赖

```kotlin
implementation("androidx.datastore:datastore-preferences:1.0.0")
```

### 与其他 Story 的关系

- Story 5-2: 基础 UI 组件
- Story 5-9: Android Keystore 存储安全数据

## Checklist

- [x] All tasks completed
- [x] Settings persist across app restarts
- [x] Device management works
- [x] Story file updated to 'done'

## Implementation Summary

### SettingsRepository

位置: `data/SettingsRepository.kt`

功能:
- 使用 DataStore Preferences 持久化设置
- `NearClipSettings` 数据类封装所有设置
- Flow 响应式暴露设置变化
- 提供 `setXxx()` 方法更新单个设置

设置项:
- `wifiEnabled`: WiFi 同步开关
- `bleEnabled`: BLE 同步开关
- `autoConnect`: 自动连接开关
- `syncNotifications`: 同步通知开关

### SettingsViewModel

位置: `SettingsViewModel.kt`

功能:
- AndroidViewModel 访问 Application Context
- StateFlow 暴露设置给 UI
- 封装 Repository 操作

### SettingsScreen 增强

新增功能:
- 使用 SettingsViewModel 管理设置状态
- 设置变化自动持久化
- "Paired Devices" section 显示已配对设备
- `PairedDeviceItem` 组件显示设备信息
- 删除设备确认对话框
