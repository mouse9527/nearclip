# 外部 API

NearClip 设计为完全本地化，不依赖外部 API 或云服务，确保用户数据的隐私和安全。

## 系统级 API 依赖

- Android Bluetooth API - 用于 BLE 通信
- Core Bluetooth Framework (macOS) - 用于 macOS BLE 通信
- Android ClipboardManager - 系统粘贴板访问
- NSPasteboard (macOS) - macOS 系统粘贴板访问

所有依赖都是操作系统原生 API，无需第三方服务。
