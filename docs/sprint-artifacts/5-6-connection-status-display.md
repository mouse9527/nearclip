# Story 5.6: 实现连接状态显示

Status: done

## Story

As a Android 用户,
I want 查看当前连接状态,
So that 我知道同步是否可用.

## Acceptance Criteria

1. **Given** 应用运行中 **When** 查看主页 **Then** 显示连接状态
2. **And** 显示已连接设备列表
3. **And** 状态变化时实时更新 UI

## Tasks / Subtasks

- [x] Task 1: 增强 ConnectionManager (AC: 1, 2, 3)
  - [x] 1.1 暴露设备连接/断开事件
  - [x] 1.2 维护 connectedDevices StateFlow

- [x] Task 2: 绑定 Service 到 UI (AC: 3)
  - [x] 2.1 MainActivity 绑定 NearClipService
  - [x] 2.2 ServiceListener 更新 ViewModel

- [x] Task 3: 增强 HomeScreen UI (AC: 1, 2)
  - [x] 3.1 显示连接状态指示器
  - [x] 3.2 显示已连接设备卡片
  - [x] 3.3 设备连接/断开动画

## Dev Notes

### 架构约束

1. **Service Binding**: 使用 bindService() 获取 Service 实例
2. **StateFlow**: ViewModel 使用 StateFlow 暴露状态
3. **Compose**: collectAsState() 响应式更新

### 与其他 Story 的关系

- Story 5-2: 基础 UI 框架
- Story 5-3: Foreground Service 提供事件

## Checklist

- [x] All tasks completed
- [x] Connection status visible
- [x] Device list displays correctly
- [x] Real-time updates work
- [x] Story file updated to 'done'

## Implementation Summary

### MainActivity Service Binding

- `LocalNearClipService` CompositionLocal 提供 Service 访问
- `ServiceConnection` 在 onStart/onStop 绑定/解绑
- `BIND_AUTO_CREATE` 自动创建 Service

### HomeScreen 增强

新增组件:
- `LastSyncCard`: 显示最后接收的剪贴板内容和来源设备
- `ErrorCard`: 显示错误信息
- `StatusCard` 增加 "Sync Now" 按钮

动画:
- `AnimatedVisibility` 淡入淡出 + 展开收缩
- 状态变化时平滑过渡

### ConnectionManager StateFlow

已有:
- `isRunning`: 运行状态
- `pairedDevices`: 配对设备列表
- `connectedDevices`: 已连接设备列表
- `lastReceivedClipboard`: 最后接收的剪贴板
- `lastError`: 最后错误信息

所有状态通过 FfiNearClipCallback 实时更新
