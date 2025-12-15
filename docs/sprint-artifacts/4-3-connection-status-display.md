# Story 4.3: 实现连接状态显示

Status: done

## Story

As a macOS 用户,
I want 看到当前连接状态,
So that 知道同步是否可用.

## Acceptance Criteria

1. **Given** 菜单栏应用已运行 **When** 查看菜单栏图标或下拉菜单 **Then** 显示当前连接状态（已连接/断开/同步中）
2. **And** 图标颜色反映状态
3. **And** 显示已连接设备列表
4. **And** 状态实时更新

## Tasks / Subtasks

- [x] Task 1: 增强 ConnectionManager 状态管理 (AC: 1, 4)
  - [x] 1.1 实现 start() 方法启动 NearClip 服务
  - [x] 1.2 实现设备连接/断开事件处理
  - [x] 1.3 使用 @Published 属性发布状态变化
  - [x] 1.4 实现 FfiNearClipCallback 桥接

- [x] Task 2: 实现菜单栏图标动态更新 (AC: 2)
  - [x] 2.1 已连接状态: link 图标 (绿色)
  - [x] 2.2 断开状态: link.badge.plus 图标 (灰色)
  - [x] 2.3 同步中状态: arrow.triangle.2.circlepath 图标 (蓝色)
  - [x] 2.4 错误状态: exclamationmark.triangle 图标 (红色)

- [x] Task 3: 实现设备列表显示 (AC: 3)
  - [x] 3.1 从 FfiNearClipManager 获取已连接设备
  - [x] 3.2 从 FfiNearClipManager 获取已配对设备
  - [x] 3.3 在 MenuBarView 中显示设备列表
  - [x] 3.4 显示设备名称、平台、状态、最后在线时间

- [x] Task 4: 实现状态实时更新 (AC: 4)
  - [x] 4.1 FfiNearClipCallback 回调触发 UI 更新
  - [x] 4.2 使用 DispatchQueue.main 确保 UI 线程更新
  - [x] 4.3 ConnectionManager 发布状态变化
  - [x] 4.4 SwiftUI 视图自动响应变化 (@ObservedObject)

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 5.1 运行 swift build 成功
  - [x] 5.2 状态枚举包含所有状态
  - [x] 5.3 图标颜色映射完整

## Dev Notes

### 架构约束

1. **SwiftUI + Combine**: 使用 @Published + ObservableObject 实现响应式更新
2. **FFI 回调**: FfiNearClipCallback 在后台线程调用，需要切换到主线程
3. **状态同步**: ConnectionManager 作为单一数据源

### 与其他 Story 的关系

- Story 4-2: 提供菜单栏应用基础结构
- Story 4-4: 将添加剪贴板监听功能
- Story 4-5: 将添加剪贴板写入功能

### 实现的状态枚举

```swift
enum ConnectionStatus: Equatable {
    case disconnected      // 未连接任何设备
    case connecting        // 正在连接
    case connected(Int)    // 已连接 N 个设备
    case syncing           // 正在同步中
    case error(String)     // 连接错误

    var symbolName: String { ... }
    var displayText: String { ... }
    var accessibilityDescription: String { ... }
}
```

### 图标映射

| 状态 | SF Symbol | 颜色 |
|------|-----------|------|
| disconnected | link.badge.plus | gray |
| connecting | arrow.triangle.2.circlepath | orange |
| connected | link | green |
| syncing | arrow.triangle.2.circlepath | blue |
| error | exclamationmark.triangle | red |

### 新增功能

1. **服务控制按钮**: 在菜单栏视图中可以启动/停止服务
2. **错误横幅**: 显示错误信息和重试按钮
3. **最后同步时间**: 显示上次同步的相对时间
4. **设备计数徽章**: 显示已连接设备数量
5. **脉冲动画**: 连接中/同步中状态图标闪烁

## Checklist

- [x] All tasks completed
- [x] Status updates work correctly
- [x] Icon changes reflect status
- [x] Device list displays properly
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
