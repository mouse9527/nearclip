# Story 4.2: 创建 macOS 菜单栏应用

Status: done

## Story

As a macOS 用户,
I want 应用作为菜单栏常驻,
So that 不占用 Dock 空间且随时可用.

## Acceptance Criteria

1. **Given** Xcode 项目已创建 **When** 配置为菜单栏应用 **Then** 应用图标显示在菜单栏
2. **And** 点击图标显示下拉菜单
3. **And** 应用不显示在 Dock
4. **And** 支持开机自启动配置

## Tasks / Subtasks

- [x] Task 1: 创建 macOS App 项目结构 (AC: 1)
  - [x] 1.1 创建 `macos/NearClip` 目录
  - [x] 1.2 创建 Package.swift (Swift Package Manager)
  - [x] 1.3 配置为菜单栏应用 (NSApp.setActivationPolicy(.accessory))
  - [x] 1.4 SPM 自动生成 entitlements

- [x] Task 2: 集成 Swift 绑定 (AC: 1)
  - [x] 2.1 复制 NearClip.swift 到项目
  - [x] 2.2 创建 NearClipFFI systemLibrary 配置静态库链接
  - [x] 2.3 验证 import 成功 (build passes, 293 symbols)

- [x] Task 3: 实现菜单栏 App 结构 (AC: 1, 2)
  - [x] 3.1 创建 NearClipApp.swift (入口)
  - [x] 3.2 创建 AppDelegate 管理 NSStatusItem
  - [x] 3.3 创建菜单栏图标 (SF Symbols: link/link.badge.plus)
  - [x] 3.4 创建下拉菜单视图 (NSPopover + SwiftUI)

- [x] Task 4: 实现下拉菜单内容 (AC: 2)
  - [x] 4.1 显示连接状态 (header with status icon)
  - [x] 4.2 显示已连接设备列表 (DeviceRow component)
  - [x] 4.3 添加"添加设备"选项
  - [x] 4.4 添加"设置"选项
  - [x] 4.5 添加"退出"选项

- [x] Task 5: 隐藏 Dock 图标 (AC: 3)
  - [x] 5.1 使用 NSApp.setActivationPolicy(.accessory)
  - [x] 5.2 应用不显示在 Dock

- [x] Task 6: 实现开机自启动 (AC: 4)
  - [x] 6.1 使用 ServiceManagement 框架
  - [x] 6.2 实现 SMAppService (macOS 13+) with fallback
  - [x] 6.3 isLaunchAtLoginEnabled/setLaunchAtLogin API 就绪

- [x] Task 7: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 7.1 运行 swift build 成功
  - [x] 7.2 二进制包含 293 个 Rust FFI 符号
  - [x] 7.3 链接 AppKit, ServiceManagement, Security 框架
  - [x] 7.4 输出: .build/debug/NearClip (5.3 MB)

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **SwiftUI**: 使用 SwiftUI 构建 UI
2. **macOS 12+**: 最低支持 Monterey
3. **菜单栏应用**: NSApp.setActivationPolicy(.accessory)
4. **uniffi 集成**: 使用 Story 4-1 生成的 Swift 绑定

### 与其他 Story 的关系

- Story 4-1: 提供 Swift 绑定和静态库
- Story 4-3: 将实现连接状态显示详细逻辑
- Story 4-4: 将实现剪贴板监听
- Story 4-6: 将实现配对 UI
- Story 4-7: 将实现设置 UI

### 最终项目结构

```
macos/
└── NearClip/
    ├── Package.swift
    └── Sources/
        ├── NearClip/
        │   ├── NearClipApp.swift      # SwiftUI App 入口
        │   ├── AppDelegate.swift       # NSStatusItem 管理
        │   ├── MenuBarView.swift       # 下拉菜单 UI
        │   ├── ConnectionManager.swift # FFI 包装层
        │   ├── NearClip.swift          # uniffi 生成的绑定
        │   └── Resources/
        │       └── .gitkeep
        └── NearClipFFI/
            ├── module.modulemap
            └── include/
                └── NearClipFFI.h
```

### 构建命令

```bash
# 构建应用
cd macos/NearClip
swift build

# 运行应用
.build/debug/NearClip
```

### 菜单栏图标

使用 SF Symbols: `link` (已连接) / `link.badge.plus` (未连接)

## Checklist

- [x] All tasks completed
- [x] App builds successfully (swift build)
- [x] FFI bindings integrated (293 symbols)
- [x] Menubar app structure ready
- [x] Dropdown menu implemented
- [x] Launch at login API ready
- [x] Story file updated to 'done'
