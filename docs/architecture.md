---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
status: completed
completedAt: '2025-12-13'
inputDocuments:
  - docs/prd.md
  - docs/analysis/research/technical-clipboard-sync-research-2025-12-12.md
workflowType: 'architecture'
lastStep: 8
project_name: 'nearclip'
user_name: 'Mouse'
date: '2025-12-13'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**
26 条功能需求，涵盖设备发现、剪贴板同步、通信通道、安全隐私、平台集成和用户反馈。核心挑战在于实现 WiFi + BLE 双通道的无缝切换，以及跨平台剪贴板监听的一致性。

**Non-Functional Requirements:**
性能要求严格（WiFi 同步 < 3s），安装包需极度精简（移动端 < 5MB），这对 Rust 核心的体积优化和 FFI 层的效率提出较高要求。

**Scale & Complexity:**
- Primary domain: 跨平台系统级应用
- Complexity level: 高
- Estimated architectural components: ~15

### Technical Constraints & Dependencies

| 约束 | 影响 |
|------|------|
| Rust 核心 + 原生 UI | 需要 FFI 绑定（uniffi、jni-rs、P/Invoke） |
| 无云服务器 | 完全 P2P，需处理 NAT、防火墙等网络问题 |
| TLS 1.3 强制 | 需自管理证书/密钥 |
| 平台特定 API | macOS NSPasteboard、Android AccessibilityService、iOS Shortcuts |
| AI Coding 依赖 | 所有技术栈需 AI 辅助，影响迭代速度 |

### Cross-Cutting Concerns Identified

1. **加密与安全**：TLS 会话管理、密钥交换、本地密钥存储
2. **设备状态机**：发现→配对→连接→同步→断开→重连
3. **通道切换**：WiFi ↔ BLE 自动切换逻辑
4. **错误处理**：网络超时、蓝牙断开、同步失败的统一处理
5. **日志与调试**：跨平台统一日志格式，便于问题排查

## Starter Template Evaluation

### Primary Technology Domain

**跨平台 Rust 库 + 原生客户端** - 系统级应用，非传统 Web/Mobile 框架项目。

### Starter Options Considered

| 方案 | 说明 | 结论 |
|------|------|------|
| 无现成模板 | Rust + uniffi + 原生 UI 没有标准 CLI 模板 | 需手动设计 |
| dotlottie-rs 参考 | uniffi 跨平台实现 | 可借鉴结构 |
| Mozilla Application Services | 官方大型 uniffi 项目 | 可借鉴模式 |

### Selected Approach: Custom Rust Workspace

**选择理由：**
- 无现成模板适用于此架构
- 参考 Mozilla/LottieFiles 等成熟项目结构
- 模块化设计便于 AI Coding 逐步实现

### Project Structure

```
nearclip/
├── Cargo.toml                 # Workspace root
├── crates/
│   ├── nearclip-core/         # 核心协调逻辑
│   ├── nearclip-net/          # mDNS + TCP/TLS
│   ├── nearclip-ble/          # BLE 通信
│   ├── nearclip-crypto/       # 加密模块
│   ├── nearclip-sync/         # 同步逻辑
│   └── nearclip-ffi/          # uniffi 绑定层
├── clients/
│   ├── macos/                 # Swift/SwiftUI 项目
│   ├── android/               # Kotlin/Compose 项目
│   ├── windows/               # C#/WinUI 项目 (Phase 2)
│   └── ios/                   # Swift/SwiftUI 项目 (Phase 2)
└── docs/
```

### Initialization Commands

```bash
# Step 1: Create workspace
mkdir nearclip && cd nearclip
cargo init --name nearclip

# Step 2: Create crates
mkdir -p crates/{nearclip-core,nearclip-net,nearclip-ble,nearclip-crypto,nearclip-sync,nearclip-ffi}
for crate in core net ble crypto sync ffi; do
  cargo init --lib crates/nearclip-$crate
done

# Step 3: Create client directories
mkdir -p clients/{macos,android,windows,ios}
```

### Architectural Decisions Provided by Structure

| 决策领域 | 选择 |
|---------|------|
| **Language & Runtime** | Rust 2021 Edition |
| **FFI Tooling** | uniffi 0.28 (Mozilla) |
| **Build System** | Cargo workspace + uniffi-bindgen |
| **Code Organization** | Multi-crate monorepo |
| **Platform Clients** | 独立原生项目 (Xcode/Android Studio) |

**Note:** 项目初始化应作为第一个实施 Story。

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- 异步运行时: tokio
- TLS 库: rustls
- FFI 绑定: uniffi
- 序列化格式: MessagePack

**Important Decisions (Shape Architecture):**
- 设备配对: 二维码 + ECDH
- 通道策略: WiFi 优先
- 日志框架: tracing

**Deferred Decisions (Post-MVP):**
- 剪贴板历史存储方案
- 图片同步压缩策略

### Rust Core Dependencies

| 依赖 | 选择 | 版本 | 理由 |
|------|------|------|------|
| **异步运行时** | tokio | 1.x | 网络编程标准，生态丰富 |
| **序列化** | rmp-serde (MessagePack) | latest | 二进制紧凑，跨平台支持好 |
| **TLS** | rustls | 0.23.x | 纯 Rust，跨平台一致 |
| **mDNS** | mdns-sd | latest | 纯 Rust，活跃维护 |
| **日志** | tracing | 0.1.x | 结构化日志，异步友好 |
| **FFI** | uniffi | 0.28.x | Mozilla 官方，支持 Swift/Kotlin |

### Authentication & Security

| 决策 | 选择 | 理由 |
|------|------|------|
| **配对机制** | 二维码 + ECDH | 用户友好，安全性高 |
| **密钥存储** | 平台密钥库 | macOS Keychain / Android Keystore |
| **传输加密** | TLS 1.3 (rustls) | 端到端加密，无需证书 |
| **信任模型** | 首次配对信任 (TOFU) | 简单有效 |

### Communication Patterns

| 决策 | 选择 | 理由 |
|------|------|------|
| **同步模式** | Push 模式 | 复制即推送，延迟最低 |
| **通道选择** | WiFi 优先，BLE 备选 | 性能优先，功耗可控 |
| **BLE 传输** | 分片传输 | 支持长文本，需重组逻辑 |
| **消息格式** | MessagePack | 紧凑二进制，跨语言 |
| **失败处理** | 重试3次 → 用户选择 | 平衡可靠性与用户体验 |

### Platform Integration

| 平台 | 集成方式 | UI 形态 |
|------|---------|---------|
| **macOS** | 菜单栏应用 | 图标 + 下拉菜单 |
| **Android** | 无障碍服务 + 前台服务 | 持久通知 + 设置页 |
| **Windows** | 系统托盘 (Phase 2) | 图标 + 菜单 |
| **iOS** | 快捷指令 (Phase 2) | 分享扩展 |

### Decision Impact Analysis

**Implementation Sequence:**
1. nearclip-crypto (TLS/加密基础)
2. nearclip-net (mDNS + TCP)
3. nearclip-ble (蓝牙通信)
4. nearclip-sync (同步逻辑)
5. nearclip-core (协调层)
6. nearclip-ffi (绑定生成)
7. macOS 客户端
8. Android 客户端

**Cross-Component Dependencies:**
- nearclip-net 和 nearclip-ble 都依赖 nearclip-crypto
- nearclip-sync 依赖 nearclip-net 和 nearclip-ble
- nearclip-core 协调所有模块
- nearclip-ffi 暴露 nearclip-core 给平台客户端

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:** 7 areas where AI agents could make different choices

### Naming Patterns

**Rust Naming Conventions:**

| 元素 | 约定 | 示例 |
|------|------|------|
| 模块名 | snake_case | `nearclip_net`, `device_manager` |
| 函数名 | snake_case | `send_clipboard`, `get_paired_devices` |
| 类型名 | PascalCase | `DeviceInfo`, `ClipboardContent` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT`, `BLE_MTU_SIZE` |
| FFI 导出 | snake_case (uniffi 自动转换) | `pair_device` → Swift: `pairDevice` |

**Platform Client Naming:**
- Swift: camelCase 函数，PascalCase 类型（uniffi 自动处理）
- Kotlin: camelCase 函数，PascalCase 类型（uniffi 自动处理）

### Message Protocol Patterns

```rust
// 所有网络消息必须遵循此结构
#[derive(Serialize, Deserialize)]
struct Message {
    msg_type: MessageType,    // 消息类型枚举
    payload: Vec<u8>,         // MessagePack 序列化的 payload
    timestamp: u64,           // Unix 毫秒时间戳
    device_id: String,        // 发送方设备 ID
}

enum MessageType {
    ClipboardSync,    // 剪贴板同步
    PairingRequest,   // 配对请求
    PairingResponse,  // 配对响应
    Heartbeat,        // 心跳
    Ack,              // 确认
}
```

### Error Handling Patterns

```rust
// 统一错误类型 - 使用 thiserror
#[derive(Debug, thiserror::Error)]
pub enum NearClipError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Bluetooth error: {0}")]
    Bluetooth(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),
}

// 规则：所有公开函数返回 Result<T, NearClipError>
// 规则：禁止在库代码中使用 panic!() 或 unwrap()
```

### Logging Patterns

```rust
// 使用 tracing，统一格式
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(content))]
pub async fn sync_clipboard(device_id: &str, content: &str) -> Result<()> {
    info!(device_id, content_len = content.len(), "Syncing clipboard");
    // ...
}

// 日志级别规范：
// - error: 需要用户干预的问题
// - warn: 可恢复的问题（如重试）
// - info: 重要业务事件（配对成功、同步完成）
// - debug: 开发调试信息
// - trace: 详细追踪（网络包、BLE 帧）
```

### Test Patterns

```
# 单元测试：模块内 #[cfg(test)]
# 集成测试：crates/xxx/tests/ 目录

crates/nearclip-core/
├── src/
│   ├── lib.rs
│   └── device.rs      # 包含 #[cfg(test)] mod tests
└── tests/
    └── integration.rs  # 集成测试
```

### FFI Interface Patterns

```udl
// uniffi.udl - 接口定义规范
namespace nearclip {
    // 所有可能失败的函数必须标记 [Throws=NearClipError]
    [Throws=NearClipError]
    void pair_device(string qr_code);

    [Throws=NearClipError]
    sequence<DeviceInfo> get_paired_devices();

    // 回调接口用于异步事件通知
    callback interface ClipboardCallback {
        void on_clipboard_received(string content, string from_device);
        void on_sync_error(string error_message);
    };
};
```

### Enforcement Guidelines

**All AI Agents MUST:**
1. 使用 `Result<T, NearClipError>` 而非 panic
2. 使用 `tracing` 宏记录日志，遵循级别规范
3. 遵循 MessagePack 消息结构定义
4. FFI 函数必须在 uniffi.udl 中声明
5. 单元测试放模块内，集成测试放 tests/ 目录

**Anti-Patterns (禁止):**
- ❌ 使用 `unwrap()` 或 `expect()` 在库代码中
- ❌ 直接 `println!()` 替代 tracing
- ❌ 自定义消息格式不遵循 Message 结构
- ❌ 跨模块直接访问私有类型

## Project Structure & Boundaries

### Complete Project Directory Structure

```
nearclip/
├── README.md
├── LICENSE
├── Cargo.toml                          # Workspace 根配置
├── Cargo.lock
├── .gitignore
├── .github/
│   └── workflows/
│       ├── ci.yml                      # Rust CI
│       ├── build-macos.yml             # macOS 构建
│       └── build-android.yml           # Android 构建
│
├── crates/
│   ├── nearclip-core/                  # 核心协调模块
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs                  # 模块入口
│   │   │   ├── manager.rs              # NearClipManager 主类
│   │   │   ├── device.rs               # 设备管理
│   │   │   ├── config.rs               # 配置管理
│   │   │   └── error.rs                # NearClipError 定义
│   │   └── tests/
│   │       └── integration.rs
│   │
│   ├── nearclip-net/                   # 网络模块
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── mdns/
│   │   │   │   ├── mod.rs              # mDNS 发现
│   │   │   │   ├── discovery.rs        # 设备发现
│   │   │   │   └── advertise.rs        # 服务广播
│   │   │   ├── tcp/
│   │   │   │   ├── mod.rs              # TCP 连接
│   │   │   │   ├── server.rs           # TCP 服务端
│   │   │   │   └── client.rs           # TCP 客户端
│   │   │   └── tls.rs                  # TLS 封装
│   │   └── tests/
│   │
│   ├── nearclip-ble/                   # 蓝牙模块
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── peripheral.rs           # BLE 外设模式
│   │   │   ├── central.rs              # BLE 中心模式
│   │   │   ├── gatt.rs                 # GATT 服务定义
│   │   │   └── chunking.rs             # 分片传输
│   │   └── tests/
│   │
│   ├── nearclip-crypto/                # 加密模块
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── keypair.rs              # ECDH 密钥对
│   │   │   ├── tls_config.rs           # TLS 配置
│   │   │   ├── qrcode.rs               # 二维码生成/解析
│   │   │   └── keystore.rs             # 密钥存储抽象
│   │   └── tests/
│   │
│   ├── nearclip-sync/                  # 同步模块
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── protocol.rs             # Message 定义
│   │   │   ├── sender.rs               # 发送逻辑
│   │   │   ├── receiver.rs             # 接收逻辑
│   │   │   └── channel.rs              # 通道选择策略
│   │   └── tests/
│   │
│   └── nearclip-ffi/                   # FFI 绑定
│       ├── Cargo.toml
│       ├── build.rs                    # uniffi 构建脚本
│       ├── src/
│       │   ├── lib.rs                  # FFI 导出
│       │   └── uniffi.udl              # 接口定义
│       └── uniffi.toml                 # uniffi 配置
│
├── clients/
│   ├── macos/                          # macOS 客户端
│   │   ├── NearClip.xcodeproj/
│   │   ├── NearClip/
│   │   │   ├── NearClipApp.swift       # App 入口
│   │   │   ├── MenuBarView.swift       # 菜单栏 UI
│   │   │   ├── PairingView.swift       # 配对界面
│   │   │   ├── SettingsView.swift      # 设置界面
│   │   │   ├── ClipboardMonitor.swift  # 剪贴板监听
│   │   │   ├── NearClipBridge.swift    # FFI 桥接
│   │   │   └── Assets.xcassets/
│   │   └── NearClipTests/
│   │
│   ├── android/                        # Android 客户端
│   │   ├── build.gradle.kts
│   │   ├── settings.gradle.kts
│   │   ├── app/
│   │   │   ├── build.gradle.kts
│   │   │   ├── src/main/
│   │   │   │   ├── AndroidManifest.xml
│   │   │   │   ├── kotlin/com/nearclip/
│   │   │   │   │   ├── NearClipApp.kt
│   │   │   │   │   ├── MainActivity.kt
│   │   │   │   │   ├── ui/
│   │   │   │   │   │   ├── PairingScreen.kt
│   │   │   │   │   │   ├── SettingsScreen.kt
│   │   │   │   │   │   └── theme/
│   │   │   │   │   ├── service/
│   │   │   │   │   │   ├── ClipboardService.kt
│   │   │   │   │   │   └── SyncForegroundService.kt
│   │   │   │   │   └── bridge/
│   │   │   │   │       └── NearClipBridge.kt
│   │   │   │   └── res/
│   │   │   └── src/test/
│   │   └── nearclip-rs/
│   │       └── build.gradle.kts
│   │
│   ├── windows/                        # Phase 2
│   │   └── .gitkeep
│   │
│   └── ios/                            # Phase 2
│       └── .gitkeep
│
├── docs/
│   ├── architecture.md
│   ├── prd.md
│   └── analysis/
│
└── scripts/
    ├── build-all.sh
    ├── build-macos.sh
    └── build-android.sh
```

### Architectural Boundaries

**Crate Dependency Graph:**

```
nearclip-ffi (FFI 导出层)
       │
       ▼
nearclip-core (核心协调)
       │
   ┌───┼───┐
   ▼   ▼   ▼
 sync  net  ble
   │   │   │
   └───┼───┘
       ▼
   crypto
```

**Platform Client Layers:**

| 层 | 职责 | 依赖 |
|---|------|------|
| UI 层 | 界面渲染、用户交互 | Bridge 层 |
| Bridge 层 | FFI 调用封装、类型转换 | nearclip-ffi |
| 服务层 | 后台运行、系统集成 | Bridge 层 |

### Requirements to Structure Mapping

| PRD 需求 | Crate/模块 | 文件 |
|---------|-----------|------|
| FR-1.1 mDNS 发现 | nearclip-net | mdns/discovery.rs |
| FR-1.2 BLE 发现 | nearclip-ble | central.rs |
| FR-1.3 二维码配对 | nearclip-crypto | qrcode.rs |
| FR-2.1 剪贴板监听 | 平台客户端 | ClipboardMonitor.swift / ClipboardService.kt |
| FR-3.1 TCP/TLS | nearclip-net | tcp/, tls.rs |
| FR-3.2 BLE 传输 | nearclip-ble | peripheral.rs, chunking.rs |
| FR-4.1 TLS 加密 | nearclip-crypto | tls_config.rs |

### Data Flow

```
[剪贴板变化] → [平台监听] → [FFI: sync_clipboard()]
      ↓
[nearclip-sync: 构建 Message]
      ↓
[nearclip-crypto: 加密]
      ↓
[nearclip-net/ble: 发送] ──→ [远程设备]
      ↓
[接收 → 解密 → 解析]
      ↓
[FFI callback: on_clipboard_received()]
      ↓
[平台客户端写入剪贴板]
```

### Integration Points

**Internal (Crate 间):**
- nearclip-core 通过 trait 抽象调用 net/ble/crypto
- nearclip-sync 使用 nearclip-crypto 加解密
- 所有 crate 通过 NearClipError 统一错误类型

**External (平台集成):**
- macOS: NSPasteboard, NSStatusItem
- Android: AccessibilityService, ForegroundService
- FFI: uniffi 生成的 Swift/Kotlin 绑定

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
- tokio + rustls: 兼容，rustls 支持 tokio 异步
- uniffi + Rust 2021: uniffi 0.28 完全支持
- MessagePack + uniffi: 通过 Vec<u8> 传递序列化数据
- tracing + tokio: 原生支持异步追踪

**Pattern Consistency:**
- 命名约定一致：Rust snake_case，uniffi 自动转换到平台规范
- 错误处理统一：NearClipError + Result 模式
- 日志格式统一：全局 tracing 规范
- 消息格式统一：Message 结构定义

**Structure Alignment:**
- Crate 依赖关系清晰，无循环依赖
- FFI 层通过 nearclip-ffi 单点暴露
- 平台客户端独立，仅依赖 FFI 生成的绑定

### Requirements Coverage Validation ✅

**Functional Requirements Coverage:**

| FR 类别 | 状态 | 架构支持 |
|---------|------|---------|
| FR-1: 设备发现配对 | ✅ | nearclip-net, nearclip-ble, nearclip-crypto |
| FR-2: 剪贴板同步 | ✅ | nearclip-sync, 平台客户端 |
| FR-3: 通信通道 | ✅ | nearclip-net, nearclip-ble |
| FR-4: 安全隐私 | ✅ | nearclip-crypto |
| FR-5: 平台集成 | ✅ | 平台客户端结构 |
| FR-6: 用户反馈 | ✅ | 平台客户端 UI |

**Non-Functional Requirements Coverage:**

| NFR | 状态 | 架构支持 |
|-----|------|---------|
| 性能 (< 3s) | ✅ | tokio 异步 + Push 模式 |
| 安装包大小 | ⚠️ | 需实现时验证优化 |
| 可靠性 | ✅ | 重试策略 + 通道切换 |
| 兼容性 | ✅ | 平台版本已定义 |
| 可维护性 | ✅ | 模块化 + 日志 |

### Implementation Readiness Validation ✅

**Decision Completeness:**
- ✅ 所有关键依赖有版本号
- ✅ 实现模式有代码示例
- ✅ 一致性规则可执行
- ✅ 禁止模式明确列出

**Structure Completeness:**
- ✅ 完整目录树已定义
- ✅ 所有 crate 结构明确
- ✅ 平台客户端结构明确
- ✅ 集成点已映射

### Gap Analysis Results

**Critical Gaps:** 无

**Important Gaps (可在实现中解决):**
- BLE GATT UUID 定义 - 实现时确定
- 二维码内容格式 - 实现时定义编码规范

**Nice-to-Have Gaps:**
- CI/CD 流水线详细配置
- 代码签名和分发流程
- 性能测试基准

### Architecture Completeness Checklist

**✅ Requirements Analysis**
- [x] 项目上下文深入分析
- [x] 规模与复杂度评估
- [x] 技术约束识别
- [x] 跨切关注点映射

**✅ Architectural Decisions**
- [x] 关键决策带版本号文档化
- [x] 技术栈完全指定
- [x] 集成模式定义
- [x] 性能考量已处理

**✅ Implementation Patterns**
- [x] 命名约定已建立
- [x] 结构模式已定义
- [x] 通信模式已指定
- [x] 流程模式已文档化

**✅ Project Structure**
- [x] 完整目录结构已定义
- [x] 组件边界已建立
- [x] 集成点已映射
- [x] 需求到结构映射完成

### Architecture Readiness Assessment

**Overall Status:** ✅ READY FOR IMPLEMENTATION

**Confidence Level:** 高

**Key Strengths:**
- 模块化清晰，边界明确
- 跨平台一致性通过 Rust 核心保证
- 安全性内建（TLS 1.3）
- AI Coding 友好的模式定义

**Areas for Future Enhancement:**
- 性能优化（安装包大小）
- 更多平台支持（Windows, iOS）
- 高级功能（图片同步、历史记录）

### Implementation Handoff

**AI Agent Guidelines:**
1. 严格遵循架构决策
2. 使用一致的实现模式
3. 尊重项目结构和边界
4. 遇到架构问题参考本文档

**First Implementation Priority:**
```bash
# 1. 初始化 Rust workspace
cargo init --name nearclip
# 2. 创建 crate 结构
# 3. 从 nearclip-crypto 开始实现
```

## Architecture Completion Summary

### Workflow Completion

**Architecture Decision Workflow:** COMPLETED ✅
**Total Steps Completed:** 8
**Date Completed:** 2025-12-13
**Document Location:** docs/architecture.md

### Final Architecture Deliverables

**Complete Architecture Document**
- 所有架构决策带版本号文档化
- AI Agent 一致性实现模式
- 完整项目结构（6 个 Rust crate + 4 个平台客户端）
- 需求到架构的完整映射
- 验证确认一致性和完整性

**Implementation Ready Foundation**
- 12+ 架构决策
- 7 个实现模式类别
- 10+ 架构组件
- 26 个功能需求完全支持

**AI Agent Implementation Guide**
- 技术栈带版本验证
- 防止实现冲突的一致性规则
- 项目结构清晰边界
- 集成模式和通信标准

### Development Sequence

1. **初始化 Rust workspace** - 创建 Cargo.toml 和 crate 结构
2. **nearclip-crypto** - TLS/加密基础
3. **nearclip-net** - mDNS + TCP 网络
4. **nearclip-ble** - 蓝牙通信
5. **nearclip-sync** - 同步协议
6. **nearclip-core** - 核心协调
7. **nearclip-ffi** - uniffi 绑定
8. **macOS 客户端** - Swift/SwiftUI
9. **Android 客户端** - Kotlin/Compose

### Project Success Factors

**Clear Decision Framework**
所有技术选择通过协作讨论确定，有明确理由

**Consistency Guarantee**
实现模式和规则确保多个 AI Agent 生成兼容一致的代码

**Complete Coverage**
所有项目需求都有架构支持，从业务需求到技术实现的清晰映射

**Solid Foundation**
Rust 核心 + uniffi + 原生客户端架构遵循业界最佳实践

---

**Architecture Status:** ✅ READY FOR IMPLEMENTATION

**Next Phase:** 开始按架构决策和模式进行实现

**Document Maintenance:** 实现过程中如有重大技术决策变更，更新本文档

