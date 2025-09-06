# NearClip macOS 应用

NearClip 的 macOS 原生应用，基于 SwiftUI 和 Core Bluetooth 构建。

## 功能特性

- 🔍 **设备发现**: 自动扫描附近的 NearClip 设备
- 🔗 **设备配对**: 安全的设备配对和连接管理
- 📋 **剪贴板同步**: 实时监听和同步剪贴板内容
- 🎨 **现代界面**: 基于 SwiftUI 的原生 macOS 界面
- ⚡ **低功耗**: 优化的蓝牙低功耗通信

## 系统要求

- macOS 13.0 或更高版本
- 支持蓝牙 4.0 或更高版本
- Xcode 15.0 或更高版本（开发）

## 构建和运行

### 使用构建脚本

```bash
# 仅构建
./build.sh

# 构建并运行
./build.sh --run
```

### 使用 Xcode

1. 打开 `NearClip.xcodeproj`
2. 选择 NearClip scheme
3. 点击 Run 按钮或按 Cmd+R

### 使用命令行

```bash
# 构建
xcodebuild build -project NearClip.xcodeproj -scheme NearClip

# 运行
open build/Debug/NearClip.app
```

## 项目结构

```
NearClip/
├── NearClipApp.swift          # 应用入口
├── ContentView.swift          # 主界面
├── Services/
│   ├── BLEService.swift       # 蓝牙服务
│   └── ClipboardService.swift # 剪贴板服务
├── Views/
│   └── DeviceDiscoveryView.swift # 设备发现界面
├── Assets.xcassets/           # 应用资源
└── NearClip.entitlements      # 应用权限
```

## 使用说明

### 首次使用

1. **启动应用**: 双击 NearClip.app 启动应用
2. **授权蓝牙**: 系统会请求蓝牙权限，请点击"允许"
3. **扫描设备**: 点击"开始扫描"按钮搜索附近的设备
4. **配对设备**: 找到目标设备后点击"配对"按钮

### 日常使用

- **自动连接**: 应用会自动连接到已配对的设备
- **剪贴板同步**: 复制内容后会自动同步到配对设备
- **状态监控**: 顶部状态栏显示连接和同步状态

## 权限说明

应用需要以下权限：

- **蓝牙权限**: 用于设备发现和通信
- **网络权限**: 用于本地网络通信（备用方案）
- **文件访问权限**: 用于读写用户选择的文件

## 故障排除

### 蓝牙问题

- 确保蓝牙已启用
- 检查系统偏好设置中的蓝牙权限
- 重启蓝牙服务：系统偏好设置 > 蓝牙 > 关闭/开启

### 连接问题

- 确保两台设备距离足够近（< 10米）
- 检查设备是否已正确配对
- 尝试重新扫描和连接

### 剪贴板问题

- 检查应用是否有剪贴板访问权限
- 确保剪贴板监听功能已启用
- 重启应用以重置剪贴板监听

## 开发说明

### 架构设计

- **MVVM 模式**: 使用 SwiftUI 的 ObservableObject
- **服务分离**: BLE 和剪贴板功能独立服务
- **响应式编程**: 使用 Combine 框架处理异步事件

### 关键组件

1. **BLEService**: 处理蓝牙设备发现、连接和通信
2. **ClipboardService**: 监听剪贴板变化并处理同步
3. **DeviceDiscoveryView**: 设备发现和配对界面
4. **ContentView**: 主界面和状态显示

### 调试技巧

- 使用 Console.app 查看应用日志
- 在 Xcode 中设置断点调试
- 检查蓝牙调试日志：`log stream --predicate 'subsystem contains "bluetooth"'`

## 许可证

本项目采用 MIT 许可证，详见 [LICENSE](../../LICENSE) 文件。