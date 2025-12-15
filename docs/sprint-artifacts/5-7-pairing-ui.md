# Story 5.7: 实现配对 UI

Status: done

## Story

As a Android 用户,
I want 通过扫描 QR 码配对设备,
So that 我可以快速安全地配对.

## Acceptance Criteria

1. **Given** 配对页面 **When** 选择显示 QR **Then** 显示本机 QR 码
2. **Given** 配对页面 **When** 选择扫描 **Then** 打开相机扫描
3. **And** 扫描成功后自动添加设备
4. **And** 支持手动输入配对码

## Tasks / Subtasks

- [x] Task 1: QR 码生成 (AC: 1) - 已在 5-2 完成
  - [x] 1.1 使用 ZXing 生成 QR 码
  - [x] 1.2 显示设备信息 QR 码

- [x] Task 2: 相机扫描 (AC: 2, 3)
  - [x] 2.1 集成 CameraX
  - [x] 2.2 集成 ML Kit Barcode Scanning
  - [x] 2.3 请求相机权限
  - [x] 2.4 扫描成功后回调

- [x] Task 3: 手动输入 (AC: 4) - 已在 5-2 完成
  - [x] 3.1 输入框验证
  - [x] 3.2 提交处理

## Dev Notes

### 依赖

```kotlin
// CameraX
implementation("androidx.camera:camera-camera2:1.3.1")
implementation("androidx.camera:camera-lifecycle:1.3.1")
implementation("androidx.camera:camera-view:1.3.1")

// ML Kit Barcode
implementation("com.google.mlkit:barcode-scanning:17.2.0")

// Accompanist Permissions
implementation("com.google.accompanist:accompanist-permissions:0.32.0")
```

### 权限

```xml
<uses-permission android:name="android.permission.CAMERA" />
<uses-feature android:name="android.hardware.camera" android:required="false" />
```

### 与其他 Story 的关系

- Story 5-2: PairingScreen 基础 UI
- Story 2-7/2-8: QR 码格式与 Rust 核心一致

## Checklist

- [x] All tasks completed
- [x] QR code displays correctly
- [x] Camera scanning works
- [x] Manual input works
- [x] Story file updated to 'done'

## Implementation Summary

### QrScanner 组件

位置: `ui/components/QrScanner.kt`

功能:
- CameraX 集成相机预览
- ML Kit Barcode Scanning 识别 QR 码
- Accompanist Permissions 处理权限请求
- 扫描成功后自动回调

权限处理:
- `PermissionRequest`: 首次请求权限
- `PermissionRationale`: 权限被拒绝后的说明
- 使用 Accompanist Permissions 简化权限状态管理

### PairingScreen 增强

- `ScanQRCodeTab` 默认显示相机扫描
- 提供 "Enter code manually" 切换到手动输入
- 扫描成功后自动调用 `addDeviceFromCode()`
- 错误处理显示在界面上
