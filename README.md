# NearClip

NearClip: 隐私优先的 P2P 剪贴板同步工具，支持 Android 和 macOS 设备间通过 BLE 进行安全同步。

## 🚀 特性

- **隐私优先**: 端到端加密，数据不离开本地网络
- **跨平台**: 支持 Android 和 macOS 设备间无缝同步
- **低功耗**: 基于 BLE 技术，功耗极低
- **实时同步**: 剪贴板内容自动、即时同步
- **安全连接**: 设备配对验证，确保只有授权设备可以连接

## 📁 项目结构

```
nearclip/
├── src/                          # 源代码目录
│   ├── platform/                 # 平台特定代码
│   │   ├── android/              # Android 应用 (Kotlin + Jetpack Compose)
│   │   │   └── app/              # 主应用模块
│   │   └── mac/                  # macOS 应用 (Swift + SwiftUI)
│   │       └── NearClip/         # 主应用包
│   └── shared/                   # 跨平台共享模块
│       ├── protocol/             # Protocol Buffers 定义
│       │   ├── device_discovery.proto
│       │   ├── data_sync.proto
│       │   └── error_handling.proto
│       └── rust/                 # Rust 核心逻辑库
│           ├── src/              # Rust 源代码
│           └── Cargo.toml        # Rust 项目配置
├── scripts/                      # 构建和工具脚本
│   ├── build-android.sh          # Android 构建脚本
│   ├── build-mac.sh              # macOS 构建脚本
│   └── build-rust.sh             # Rust 库构建脚本
├── docs/                         # 项目文档
│   ├── architecture/             # 架构文档
│   ├── prd/                      # 产品需求文档
│   └── stories/                  # 用户故事
├── .github/workflows/            # CI/CD 配置
├── .bmad-core/                   # BMad 工作流配置
├── README.md                     # 项目说明
├── LICENSE                       # 开源许可证
└── .gitignore                    # Git 忽略文件
```

## 🛠️ 技术栈

### 核心技术
- **共享逻辑**: Rust 1.70+ (内存安全、高性能)
- **数据序列化**: Protocol Buffers 3.x (类型安全、高效)
- **通信协议**: BLE 5.0+ (低功耗、近场通信)

### Android 平台
- **语言**: Kotlin 1.9.20
- **UI框架**: Jetpack Compose 1.5.4
- **构建工具**: Gradle 8.0
- **FFI**: JNI (Kotlin ↔ Rust)

### macOS 平台
- **语言**: Swift 5.9
- **UI框架**: SwiftUI 5.0
- **构建工具**: Swift Package Manager
- **FFI**: C ABI (Swift ↔ Rust)

## 🚀 快速开始

### 环境要求

#### 基础工具
- **Rust 1.70+**: [安装指南](https://rustup.rs/)
- **Protocol Buffers 编译器**: `protoc 3.x`
- **Git**: 版本控制

#### Android 开发
- **Android Studio**: 最新稳定版
- **Android SDK**: API 24+ (Android 7.0+)
- **JDK**: 17 或更高版本

#### macOS 开发
- **Xcode**: 14.0 或更高版本
- **macOS**: 13.0 (Ventura) 或更高版本

### 安装步骤

1. **克隆仓库**
   ```bash
   git clone https://github.com/your-org/nearclip.git
   cd nearclip
   ```

2. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **安装 Protocol Buffers**
   ```bash
   # macOS
   brew install protobuf

   # Ubuntu/Debian
   sudo apt-get install protobuf-compiler
   ```

4. **构建项目**
   ```bash
   # 构建所有平台
   ./scripts/build-rust.sh
   ./scripts/build-android.sh
   ./scripts/build-mac.sh
   ```

### 构建命令

```bash
# 构建 Rust 共享库
./scripts/build-rust.sh

# 构建 Android 应用
./scripts/build-android.sh

# 构建 macOS 应用
./scripts/build-mac.sh
```

## 🔧 开发指南

### 代码风格

项目使用严格的代码风格和格式化标准：

- **Rust**: 使用 `rustfmt` 和 `clippy`
- **Kotlin**: 遵循 Android 官方代码规范
- **Swift**: 遵循 Swift 官方代码规范

详细的开发指南请参考: [docs/development-guide.md](docs/development-guide.md)

### 调试

调试日志文件位置：`.ai/debug-log.md`

### 测试

```bash
# 运行 Rust 测试
cd src/shared/rust && cargo test

# 运行 Android 测试
cd src/platform/android && ./gradlew test

# 运行 macOS 测试
cd src/platform/mac && swift test
```

## 📚 文档

- **[架构文档](docs/architecture/)**: 详细的技术架构设计
- **[产品需求](docs/prd/)**: 产品功能规格说明
- **[用户故事](docs/stories/)**: 开发任务和验收标准
- **[API 文档](docs/api/)**: 接口文档和使用示例

## 🏗️ CI/CD

项目使用 GitHub Actions 进行持续集成和持续部署：

- **自动测试**: 每次提交触发所有平台测试
- **代码检查**: 自动运行格式化和静态分析
- **构建验证**: 验证所有平台的构建过程

配置文件：[`.github/workflows/ci.yml`](.github/workflows/ci.yml)

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🆘 支持

如果您遇到问题或有建议：

1. 查看 [FAQ](docs/faq.md)
2. 搜索 [Issues](https://github.com/your-org/nearclip/issues)
3. 创建新的 Issue 描述问题

## 🙏 致谢

感谢所有为 NearClip 项目做出贡献的开发者和社区成员。