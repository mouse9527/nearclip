# 统一项目结构

```
nearclip/
├── .github/                           # CI/CD 工作流
│   └── workflows/
│       ├── ci-android.yaml
│       ├── ci-mac.yaml
│       └── ci-rust.yaml               # Rust 库构建流水线
├── src/                               # 源代码根目录
│   ├── platform/                      # 平台特定代码
│   │   ├── android/                   # Android 应用
│   │   │   ├── app/
│   │   │   │   ├── src/
│   │   │   │   │   ├── main/
│   │   │   │   │   │   ├── java/com/nearclip/
│   │   │   │   │   │   │   ├── ui/            # UI 组件
│   │   │   │   │   │   │   ├── services/      # Rust FFI 调用层
│   │   │   │   │   │   │   ├── data/          # 数据层
│   │   │   │   │   │   │   └── MainActivity.kt
│   │   │   │   │   │   └── res/               # 资源文件
│   │   │   │   │   └── test/                  # 测试代码
│   │   │   │   ├── build.gradle.kts
│   │   │   │   └── proguard-rules.pro
│   │   │   ├── build.gradle.kts
│   │   │   └── gradle.properties
│   │   └── mac/                       # macOS 应用
│   │       ├── NearClip/
│   │       │   ├── Sources/
│   │       │   │   ├── App/
│   │       │   │   │   ├── ContentView.swift
│   │       │   │   │   └── NearClipApp.swift
│   │       │   │   ├── Services/        # Rust FFI 调用层
│   │       │   │   ├── Views/           # UI 组件
│   │       │   │   ├── Models/          # 数据模型
│   │       │   │   └── Utils/           # 工具类
│   │       │   ├── Tests/
│   │       │   └── Package.swift
│   │       └── NearClip.xcodeproj/
│   └── shared/                        # 跨平台共享代码
│       ├── protocol/                  # Protocol Buffers 定义
│       │   ├── device.proto           # 设备相关消息
│       │   ├── sync.proto             # 同步消息
│       │   ├── pairing.proto          # 配对消息
│       │   └── error.proto            # 错误消息
│       ├── rust/                      # Rust 共享逻辑库
│       │   ├── Cargo.toml             # Rust 项目配置
│       │   ├── src/
│       │   │   ├── lib.rs              # 库入口
│       │   │   ├── protocol/           # Protocol Buffers 处理
│       │   │   │   ├── mod.rs
│       │   │   │   ├── device.rs
│       │   │   │   ├── sync.rs
│       │   │   │   └── pairing.rs
│       │   │   ├── ble/                # BLE 通信逻辑
│       │   │   │   ├── mod.rs
│       │   │   │   ├── discovery.rs
│       │   │   │   ├── connection.rs
│       │   │   │   └── advertising.rs
│       │   │   ├── security/           # 安全和加密
│       │   │   │   ├── mod.rs
│       │   │   │   ├── crypto.rs
│       │   │   │   └── key_management.rs
│       │   │   ├── ffi/                # FFI 接口
│   │   │   │   ├── mod.rs
│       │   │   │   ├── android.rs      # Android JNI 绑定
│   │   │   │   └── mac.rs             # Mac C 绑定
│       │   │   └── utils/              # 工具函数
│       │   │       ├── mod.rs
│       │   │       ├── logger.rs
│       │   │       └── extensions.rs
│       │   ├── cbindgen.toml           # C 头文件生成配置
│       │   └── build.rs                # 构建脚本
│       └── generated/                  # 自动生成的文件
│           ├── java/                   # Kotlin 绑定
│           └── swift/                  # Swift 绑定
├── docs/                              # 文档
│   ├── prd.md
│   ├── front-end-spec.md
│   ├── architecture.md
│   └── api/
├── scripts/                           # 构建和部署脚本
│   ├── build-android.sh
│   ├── build-mac.sh
│   ├── build-rust.sh                   # Rust 库构建脚本
│   ├── generate-bindings.sh            # FFI 绑定生成
│   └── test.sh
├── .env.example                       # 环境变量模板
├── .gitignore
├── README.md
└── LICENSE
```

## 架构变更说明 (v1.1)

### 主要变更
1. **统一目录结构**：采用 `src/platform/*` 和 `src/shared/*` 层次结构
2. **Rust 共享逻辑**：核心业务逻辑由 Rust 实现，确保跨平台一致性
3. **Protocol Buffers**：替代 JSON，提供类型安全和性能优化
4. **FFI 集成层**：平台代码通过 FFI 调用 Rust 共享逻辑

### 关键特性
- **跨平台一致性**：Rust 确保所有平台的业务逻辑完全一致
- **类型安全**：Protocol Buffers 提供强类型的消息定义
- **高性能**：Rust 提供内存安全和执行效率
- **模块化设计**：清晰的分层架构，便于维护和扩展

### 开发流程
1. **Protocol Buffers 定义** → 2. **Rust 核心实现** → 3. **FFI 绑定生成** → 4. **平台 UI 集成
