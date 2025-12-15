# Story 4.7: 实现设置界面

Status: done

## Story

As a macOS 用户,
I want 通过设置界面管理应用配置,
So that 可以自定义同步行为和应用偏好.

## Acceptance Criteria

1. **Given** 点击 "Settings" **When** 打开设置窗口 **Then** 显示可配置选项
2. **And** 可配置开机自启动
3. **And** 可配置 WiFi/BLE 同步开关
4. **And** 可管理已配对设备（删除）
5. **And** 设置保存到 UserDefaults

## Tasks / Subtasks

- [x] Task 1: 创建 SettingsView (AC: 1, 2, 3)
  - [x] 1.1 GeneralSettingsTab: 开机自启动 Toggle
  - [x] 1.2 SyncSettingsTab: WiFi/BLE 开关
  - [x] 1.3 使用 TabView 四个 Tab (General, Sync, Devices, About)

- [x] Task 2: 实现设备管理 (AC: 4)
  - [x] 2.1 DevicesSettingsTab 显示已配对设备列表
  - [x] 2.2 支持选择设备并删除

- [x] Task 3: 实现设置存储 (AC: 5)
  - [x] 3.1 使用 @AppStorage 绑定 UserDefaults
  - [x] 3.2 Launch at Login 使用 SMAppService

- [x] Task 4: 集成到 AppDelegate (AC: 1)
  - [x] 4.1 SettingsWindowController 管理独立 NSWindow
  - [x] 4.2 MenuBarView 的 onSettings 回调触发

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4, 5)
  - [x] 5.1 运行 swift build 成功
  - [x] 5.2 设置界面功能完整

## Dev Notes

### 架构约束

1. **@AppStorage**: SwiftUI 原生 UserDefaults 绑定
2. **TabView**: 使用 macOS 风格的设置 Tab
3. **SMAppService**: macOS 13+ 开机自启动 API

### 设置项

| 设置项 | Key | 默认值 |
|--------|-----|--------|
| Launch at Login | launchAtLogin | false |
| WiFi Sync | wifiEnabled | true |
| BLE Sync | bleEnabled | true |
| Auto Connect | autoConnect | true |

### 与其他 Story 的关系

- Story 4-2: AppDelegate 已实现 Launch at Login 基础
- Story 4-6: 配对界面已实现设备添加

### 实现的 SettingsView

```swift
struct SettingsView: View {
    TabView {
        GeneralSettingsTab()      // Launch at Login
        SyncSettingsTab()         // WiFi/BLE/AutoConnect toggles
        DevicesSettingsTab()      // Paired devices list + delete
        AboutTab()                // App info
    }
}

final class SettingsWindowController: NSObject {
    func showWindow(connectionManager: ConnectionManager) {
        // 创建/显示 NSWindow
    }
}
```

### 设置项存储

| 设置项 | Key | 存储方式 |
|--------|-----|----------|
| Launch at Login | - | SMAppService |
| WiFi Sync | wifiEnabled | @AppStorage |
| BLE Sync | bleEnabled | @AppStorage |
| Auto Connect | autoConnect | @AppStorage |

### 文件结构

```
Sources/NearClip/
├── SettingsView.swift       # NEW - 设置界面 + 窗口控制器
├── AppDelegate.swift        # UPDATED - settingsWindowController
└── ...
```

## Checklist

- [x] All tasks completed
- [x] Settings window opens correctly
- [x] Launch at login toggle works
- [x] Sync settings persist (@AppStorage)
- [x] Device management works (list + delete)
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
