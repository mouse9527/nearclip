# NearClip
NearClip: Privacy-first P2P clipboard synchronization tool for Android and macOS using BLE.

## 项目结构

```
nearclip/
├── src/                     # 源代码目录
│   ├── shared/             # 跨平台共享模块
│   │   ├── protobuf/       # Protocol Buffers 定义
│   │   ├── rust/           # Rust 核心逻辑
│   │   └── generated/      # 自动生成的代码
│   └── platform/           # 平台特定代码
│       ├── android/        # Android 应用
│       └── mac/            # macOS 应用
├── scripts/                # 构建和工具脚本
├── docs/                   # 项目文档
├── tests/                  # 跨平台集成测试
├── README.md               # 项目说明
├── LICENSE                 # 许可证
└── .gitignore             # Git 忽略文件
```

## 快速开始

### 环境要求

- Rust 1.70+
- Android Studio (Android 开发)
- Xcode (macOS 开发)
- Protocol Buffers 编译器

### 构建命令

```bash
# 构建所有平台
./scripts/build-all.sh

# 仅构建 Android
./scripts/build-android.sh

# 仅构建 macOS
./scripts/build-macos.sh

# 运行测试
./scripts/test-all.sh
```

## 开发指南

详细的开发指南请参考: [docs/development-guide.md](docs/development-guide.md)

## 架构文档

完整的架构文档请参考: [docs/architecture/](docs/architecture/)

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件