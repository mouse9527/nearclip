# NearClip

跨253565e3_20251217141413设备本地剪贴板同步工具 - 在 macOS 和 Android 设备之间无缝同步剪贴板内容。

## 功能特性

- **即时同步** - 剪贴板内容在设备间实时同步
- **多通道传输** - 支持 Wi-Fi (TCP/mDNS) 和蓝牙 (BLE) 双通道
- **自动切换** - 网络不稳定时自动在 Wi-Fi 和 BLE 之间切换
- **端到端加密** - 使用 ECDH + TLS 1.3 保护数据安全
- **QR 码配对** - 扫描二维码快速配对设备
- **多设备支持** - 最多同时配对 5 台设备
- **网络恢复** - 网络断开后自动重连
- **智能重试** - 可配置的同步失败重试策略

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                     Platform Clients                         │
├─────────────────────────────┬───────────────────────────────┤
│     macOS (Swift/SwiftUI)   │      Android (Kotlin/Jetpack) │
│     - MenuBar App           │      - Foreground Service     │
│     - Keychain Storage      │      - EncryptedPreferences   │
└─────────────────────────────┴───────────────────────────────┘
                              │
                      ┌───────┴───────┐
                      │  UniFFI (FFI) │
                      └───────┬───────┘
                              │
┌─────────────────────────────┴───────────────────────────────┐
│                      Rust Core                               │
├──────────┬──────────┬──────────┬──────────┬─────────────────┤
│nearclip- │nearclip- │nearclip- │nearclip- │   nearclip-     │
│  core    │   net    │   ble    │  crypto  │     sync        │
│          │          │          │          │                 │
│ Manager  │ TCP/mDNS │ BLE GATT │ ECDH     │ ChannelMonitor  │
│ Config   │ TLS 1.3  │ Scan     │ TLS Cert │ ChannelSwitcher │
│ Device   │ Client   │ Advertise│ QR Code  │ RetryPolicy     │
│ Callback │ Server   │ Transfer │          │ LoopGuard       │
└──────────┴──────────┴──────────┴──────────┴─────────────────┘
```

## 项目结构

```
nearclip/
├── crates/                      # Rust 核心库
│   ├── nearclip-core/          # 核心协调层 & API
│   ├── nearclip-net/           # 网络层 (TCP/mDNS/TLS)
│   ├── nearclip-ble/           # 蓝牙层 (BLE GATT)
│   ├── nearclip-crypto/        # 加密层 (ECDH/TLS证书/QR码)
│   ├── nearclip-sync/          # 同步层 (通道监控/切换/重试)
│   └── nearclip-ffi/           # FFI 绑定 (UniFFI)
├── macos/                       # macOS 客户端
│   └── NearClip/               # Swift Package
└── android/                     # Android 客户端
    └── app/                    # Kotlin/Jetpack Compose
```

## 技术栈

### Rust Core
- **异步运行时**: Tokio
- **加密**: p256 (ECDH), rustls (TLS 1.3), rcgen (证书生成)
- **网络**: mdns-sd (mDNS 服务发现)
- **序列化**: serde, MessagePack
- **FFI**: UniFFI

### macOS
- **语言**: Swift 5.9+
- **UI**: SwiftUI
- **框架**: Network.framework, CoreBluetooth, UserNotifications
- **存储**: Keychain Services

### Android
- **语言**: Kotlin
- **UI**: Jetpack Compose + Material 3
- **存储**: EncryptedSharedPreferences
- **服务**: Foreground Service

## 安装

### 下载

从 [Releases](https://github.com/mouse9527/nearclip/releases) 页面下载最新版本：

- **macOS**: `NearClip-x.x.x-macos.dmg`
- **Android**: `NearClip-x.x.x-android.apk`

### macOS 安装

1. 下载 DMG 文件
2. 打开 DMG，将 NearClip.app 拖到 Applications 文件夹
3. 首次打开时，由于应用未签名，macOS 会阻止运行。请按以下步骤操作：
   - 右键点击 NearClip.app，选择 "打开"
   - 在弹出的对话框中点击 "打开"
   - 或者在 系统设置 > 隐私与安全性 中点击 "仍要打开"

### Android 安装

1. 下载 APK 文件
2. 在设备上启用 "允许安装未知来源应用"
3. 打开 APK 文件进行安装

## 从源码构建

### 系统要求

- **macOS**: macOS 12.0+ (Monterey)
- **Android**: Android 8.0+ (API 26)
- **Rust**: 1.75+

### 构建 Rust Core

```bash
# 克隆仓库
git clone https://github.com/user/nearclip.git
cd nearclip

# 构建所有 crates
cargo build --release

# 运行测试
cargo test
```

### 构建 macOS 客户端

```bash
cd macos/NearClip

# 使用 Swift Package Manager 构建
swift build

# 或使用脚本生成 Swift 绑定并构建
../scripts/build-swift.sh
```

### 构建 Android 客户端

```bash
cd android

# 生成 Kotlin 绑定
cargo run -p nearclip-ffi --bin uniffi-bindgen generate \
    --library ../target/release/libnearclip_ffi.dylib \
    --language kotlin \
    --out-dir app/src/main/java

# 使用 Gradle 构建
./gradlew assembleDebug
```

## 使用指南

### 配对设备

1. 在两台设备上安装 NearClip
2. 在一台设备上点击 "添加设备" 生成 QR 码
3. 在另一台设备上扫描 QR 码完成配对

### 同步设置

| 设置 | 说明 |
|------|------|
| Wi-Fi 同步 | 通过本地网络同步 (更快) |
| 蓝牙同步 | 无需网络连接 (备用通道) |
| 自动连接 | 启动时自动连接已配对设备 |
| 同步通知 | 显示同步成功/失败通知 |
| 重试策略 | 同步失败时的处理方式 |

### 重试策略选项

- **放弃 (Discard)**: 放弃本次同步
- **等待设备 (Wait for Device)**: 排队等待设备重连后发送
- **继续重试 (Continue Retry)**: 持续重试直到成功

## API 参考

### NearClipManager

核心管理器，提供所有主要功能：

```rust
use nearclip_core::{NearClipManager, NearClipConfig};

// 创建配置
let config = NearClipConfig::new("My Device")
    .with_wifi_enabled(true)
    .with_ble_enabled(true)
    .with_auto_connect(true);

// 创建管理器
let manager = NearClipManager::new(config, callback)?;

// 启动服务
manager.start()?;

// 同步剪贴板
manager.sync_clipboard(content)?;

// 获取已连接设备
let devices = manager.get_connected_devices();

// 停止服务
manager.stop();
```

### 回调接口

```rust
pub trait NearClipCallback: Send + Sync {
    fn on_device_connected(&self, device: DeviceInfo);
    fn on_device_disconnected(&self, device_id: &str);
    fn on_clipboard_received(&self, content: Vec<u8>, from_device: &str);
    fn on_sync_error(&self, error: &str);
}
```

## 安全性

- **设备配对**: 使用 ECDH 密钥交换建立共享密钥
- **数据传输**: TLS 1.3 加密所有网络流量
- **本地存储**:
  - macOS: Keychain Services (kSecAttrAccessibleAfterFirstUnlock)
  - Android: EncryptedSharedPreferences (AES-256-GCM)
- **QR 码**: 包含设备公钥，扫描后完成密钥交换

## 开发状态

| Epic | 状态 | 说明 |
|------|------|------|
| Epic 1: 项目基础设施 | ✅ 完成 | Rust workspace, 错误类型, 日志, 消息协议 |
| Epic 2: 设备发现与配对 | ✅ 完成 | ECDH, TLS, mDNS, BLE, QR码, 持久化 |
| Epic 3: 剪贴板同步 | ✅ 完成 | TCP/BLE 传输, 通道监控/切换, 重试, 防循环 |
| Epic 4: macOS 客户端 | ✅ 完成 | MenuBar App, UI, Keychain |
| Epic 5: Android 客户端 | ✅ 完成 | Foreground Service, UI, Keystore |
| Epic 6: 增强体验 | ✅ 完成 | 通知, 多设备, 重试策略, 网络恢复 |

## 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- [UniFFI](https://github.com/mozilla/uniffi-rs) - Rust FFI 绑定生成
- [mdns-sd](https://github.com/keepsimple1/mdns-sd) - mDNS 服务发现
- [rustls](https://github.com/rustls/rustls) - TLS 实现
