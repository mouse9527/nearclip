# Story 4.6: 实现配对界面

Status: done

## Story

As a macOS 用户,
I want 通过界面配对新设备,
So that 可以与其他设备同步剪贴板.

## Acceptance Criteria

1. **Given** 点击 "Add Device" **When** 打开配对窗口 **Then** 显示本机 QR 码
2. **And** 显示本机配对信息（设备名、ID）
3. **And** 提供手动输入配对码选项
4. **And** 配对成功后添加到已配对设备列表

## Tasks / Subtasks

- [x] Task 1: 创建 PairingView (AC: 1, 2)
  - [x] 1.1 使用 CoreImage CIQRCodeGenerator 生成 QR 码
  - [x] 1.2 显示设备名称和配对信息
  - [x] 1.3 使用 PairingWindowController 在独立 NSWindow 中显示

- [x] Task 2: 实现手动配对 (AC: 3)
  - [x] 2.1 添加 TextField 输入对方配对码
  - [x] 2.2 JSON 解析配对码获取设备信息
  - [x] 2.3 验证并添加设备

- [x] Task 3: 集成到 AppDelegate (AC: 1, 4)
  - [x] 3.1 PairingWindowController 管理配对窗口
  - [x] 3.2 MenuBarView 的 onAddDevice 回调触发

- [x] Task 4: 设备配对管理 (AC: 4)
  - [x] 4.1 调用 ConnectionManager.nearClipManager.addPairedDevice()
  - [x] 4.2 配对成功提示和自动关闭窗口

- [x] Task 5: 构建验证 (AC: 1, 2, 3, 4)
  - [x] 5.1 运行 swift build 成功
  - [x] 5.2 配对界面功能完整

## Dev Notes

### 架构约束

1. **CoreImage**: 使用 CIQRCodeGenerator 生成 QR 码
2. **独立窗口**: 配对界面使用 NSWindow 而非 Popover
3. **配对码格式**: JSON 格式包含设备信息

### 配对码格式

```json
{
  "id": "device-uuid",
  "name": "Device Name",
  "platform": "macOS"
}
```

### 与其他 Story 的关系

- Story 2-7: Rust 端 QR 码生成
- Story 2-8: Rust 端 QR 码解析
- Story 4-8: 配对信息存储到 Keychain

### 实现的 PairingView

```swift
struct PairingView: View {
    @ObservedObject var connectionManager: ConnectionManager
    @State private var selectedTab = 0  // 0: QR Code, 1: Manual
    @State private var manualPairingCode = ""

    // Tab 1: 显示 QR 码 (CoreImage CIQRCodeGenerator)
    // Tab 2: 手动输入配对码

    private func pairWithCode() {
        // 解析 JSON 配对码
        // 创建 FfiDeviceInfo
        // 调用 addPairedDevice()
    }
}

final class PairingWindowController: NSObject {
    func showWindow(connectionManager: ConnectionManager) {
        // 创建/显示 NSWindow
    }
}
```

### 文件结构

```
Sources/NearClip/
├── PairingView.swift        # NEW - 配对界面 + 窗口控制器
├── AppDelegate.swift        # UPDATED - pairingWindowController
├── ConnectionManager.swift  # UPDATED - nearClipManager 改为 private(set)
└── ...
```

## Checklist

- [x] All tasks completed
- [x] QR code displays correctly
- [x] Manual pairing works
- [x] Device added to paired list
- [x] Build passes (swift build)
- [x] Story file updated to 'done'
