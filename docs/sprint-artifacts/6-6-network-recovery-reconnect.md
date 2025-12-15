# Story 6.6: 实现网络恢复自动重连

Status: done

## Story

As a 用户,
I want 网络恢复后自动重连,
So that 无需手动干预.

## Acceptance Criteria

1. **Given** 网络断开导致连接中断 **When** 网络恢复 **Then** 自动尝试重新连接
2. **And** 重连成功恢复同步
3. **And** 重连过程对用户透明
4. **And** 多次重连失败后通知用户

## Tasks / Subtasks

- [x] Task 1: 监听网络状态变化 (AC: 1)
  - [x] 1.1 macOS: 使用 NWPathMonitor 监听网络状态 (NetworkMonitor.swift)
  - [x] 1.2 Android: 使用 ConnectivityManager 监听网络状态 (NetworkMonitor.kt)

- [x] Task 2: 网络恢复时触发重连 (AC: 1, 2)
  - [x] 2.1 macOS: 网络恢复时调用 ConnectionManager.restart()
  - [x] 2.2 Android: 网络恢复时调用 restartSync()

- [x] Task 3: 重连失败处理 (AC: 4)
  - [x] 3.1 macOS: 3 次失败后显示通知
  - [x] 3.2 Android: 3 次失败后显示通知

## Dev Notes

### 网络监听

- **macOS**: 使用 Network framework 的 `NWPathMonitor`
- **Android**: 使用 `ConnectivityManager.registerNetworkCallback`

### 重连策略

- 网络恢复后延迟 1-2 秒再重连（等待网络稳定）
- 使用指数退避重试 (1s, 2s, 4s)
- 最大重试次数：3 次
- 超过重试次数后通知用户

## Checklist

- [x] All tasks completed
- [x] Network status monitoring works
- [x] Auto-reconnect triggers on network recovery
- [x] User notified after multiple failures
- [x] Story file updated to 'done'

## Implementation Summary

### macOS

**NetworkMonitor.swift**:
- 使用 `NWPathMonitor` 监听网络状态变化
- `wasDisconnected` 标记是否经历断网
- 网络恢复时触发 `onNetworkRestored` 回调
- 指数退避重试：1s, 2s, 4s
- 最大 3 次重试后触发 `onReconnectFailed`

**AppDelegate.swift**:
- 设置 `NetworkMonitor.shared` 回调
- `onNetworkRestored`: 调用 `ConnectionManager.restart()`
- `onReconnectFailed`: 显示失败通知

### Android

**NetworkMonitor.kt**:
- 使用 `ConnectivityManager.registerNetworkCallback`
- 监听 `onAvailable` 和 `onLost` 事件
- 网络恢复时触发重连
- 指数退避重试：1000ms, 2000ms, 4000ms
- 最大 3 次重试后触发回调

**NearClipService.kt**:
- 初始化 `NetworkMonitor` 并设置回调
- `onNetworkRestored`: 调用 `restartSync()`
- `onReconnectFailed`: 显示失败通知
- `isConnectedToDevices`: 检查连接状态
