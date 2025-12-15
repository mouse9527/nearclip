# Story 5.2: 创建 Android App Framework

Status: done

## Story

As a Android 用户,
I want 一个 Compose UI 应用框架,
So that 可以管理剪贴板同步功能.

## Acceptance Criteria

1. **Given** Android 项目已创建 **When** 运行应用 **Then** 显示主界面
2. **And** 应用使用 Jetpack Compose 构建 UI
3. **And** 集成 Kotlin FFI 绑定
4. **And** 实现基本导航结构

## Tasks / Subtasks

- [x] Task 1: 完善 Application 类 (AC: 1, 3)
  - [x] 1.1 创建 NearClipApplication
  - [x] 1.2 初始化 FFI logging

- [x] Task 2: 创建 Compose 导航 (AC: 2, 4)
  - [x] 2.1 添加 Navigation Compose 依赖
  - [x] 2.2 定义导航路由 (Home, Pairing, Settings)
  - [x] 2.3 创建 NavHost

- [x] Task 3: 实现主界面 (AC: 1, 2)
  - [x] 3.1 创建 HomeScreen
  - [x] 3.2 显示连接状态
  - [x] 3.3 显示已配对设备列表
  - [x] 3.4 添加操作按钮

- [x] Task 4: 创建 ConnectionManager (AC: 3)
  - [x] 4.1 封装 FfiNearClipManager
  - [x] 4.2 实现 StateFlow 暴露状态
  - [x] 4.3 实现 FfiNearClipCallback

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 5.1 应用框架完成
  - [x] 5.2 UI 结构正确

## Dev Notes

### 架构约束

1. **Jetpack Compose**: Material3 设计
2. **Android 8.0+**: minSdk 26
3. **MVVM**: ViewModel + StateFlow
4. **UniFFI**: 使用 Story 5-1 生成的 Kotlin 绑定

### 与其他 Story 的关系

- Story 5-1: 提供 Kotlin 绑定
- Story 5-3: 将实现前台服务
- Story 5-6: 将实现连接状态详细显示
- Story 5-7: 将实现配对 UI

## Checklist

- [x] All tasks completed
- [x] App framework created
- [x] Navigation works
- [x] FFI integration ready
- [x] Story file updated to 'done'

## Implementation Summary

### 项目结构

```
android/app/src/main/java/com/nearclip/
├── MainActivity.kt           # 主活动
├── NearClipApplication.kt    # Application 类
├── ConnectionManager.kt      # FFI 包装层
├── ffi/nearclip.kt           # UniFFI 生成的绑定
└── ui/
    ├── navigation/
    │   ├── NavRoutes.kt      # 导航路由定义
    │   └── NearClipNavHost.kt # NavHost
    ├── screens/
    │   ├── HomeScreen.kt     # 主界面
    │   ├── PairingScreen.kt  # 配对界面
    │   └── SettingsScreen.kt # 设置界面
    └── theme/
        └── Theme.kt          # Material3 主题
```

### 依赖

- Navigation Compose: 导航框架
- Material3: 设计系统
- ViewModel: MVVM 架构
- ZXing: QR Code 生成

### ConnectionManager

- 封装 `FfiNearClipManager`
- 实现 `FfiNearClipCallback` 接收事件
- 使用 `StateFlow` 暴露状态
- ViewModel 生命周期管理
