# 统一项目结构

```
nearclip/
├── .github/                           # CI/CD 工作流
│   └── workflows/
│       ├── ci-android.yaml
│       └── ci-mac.yaml
├── shared/                            # 共享协议和工具
│   ├── protocol/
│   │   ├── MessageTypes.kt            # 消息类型定义
│   │   ├── Serialization.kt           # 序列化工具
│   │   └── Validation.kt              # 验证逻辑
│   ├── security/
│   │   ├── Crypto.kt                  # 加密工具
│   │   └── KeyManagement.kt           # 密钥管理
│   └── utils/
│       ├── Logger.kt                  # 日志工具
│       └── Extensions.kt              # 扩展函数
├── android/                           # Android 应用
│   ├── app/
│   │   ├── src/
│   │   │   ├── main/
│   │   │   │   ├── java/com/nearclip/
│   │   │   │   │   ├── ui/            # UI 组件
│   │   │   │   │   ├── services/      # 后端服务
│   │   │   │   │   ├── data/          # 数据层
│   │   │   │   │   └── MainActivity.kt
│   │   │   │   └── res/               # 资源文件
│   │   │   └── test/                  # 测试代码
│   │   ├── build.gradle.kts
│   │   └── proguard-rules.pro
│   ├── build.gradle.kts
│   └── gradle.properties
├── mac/                               # macOS 应用
│   ├── NearClip/
│   │   ├── Sources/
│   │   │   ├── App/
│   │   │   │   ├── ContentView.swift
│   │   │   │   └── NearClipApp.swift
│   │   │   ├── Services/              # 后端服务
│   │   │   ├── Views/                 # UI 组件
│   │   │   ├── Models/                # 数据模型
│   │   │   └── Utils/                 # 工具类
│   │   ├── Tests/
│   │   └── Package.swift
│   └── NearClip.xcodeproj/
├── docs/                              # 文档
│   ├── prd.md
│   ├── front-end-spec.md
│   ├── architecture.md
│   └── api/
├── scripts/                           # 构建和部署脚本
│   ├── build-android.sh
│   ├── build-mac.sh
│   └── test.sh
├── .env.example                       # 环境变量模板
├── .gitignore
├── README.md
└── LICENSE
```
