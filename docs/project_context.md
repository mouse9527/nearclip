---
project_name: 'nearclip'
user_name: 'Mouse'
date: '2025-12-13'
sections_completed: ['technology_stack', 'rust_rules', 'ffi_rules', 'testing_rules', 'logging_rules', 'protocol_rules', 'anti_patterns']
status: 'complete'
rule_count: 45
optimized_for_llm: true
---

# Project Context for AI Agents

_This file contains critical rules and patterns that AI agents must follow when implementing code in this project. Focus on unobvious details that agents might otherwise miss._

---

## Technology Stack & Versions

### Rust Core
| 依赖 | 版本 | 用途 |
|------|------|------|
| Rust | 2021 Edition | 语言版本 |
| tokio | 1.x | 异步运行时 |
| rmp-serde | latest | MessagePack 序列化 |
| rustls | 0.23.x | TLS 1.3 加密 |
| mdns-sd | latest | 局域网设备发现 |
| tracing | 0.1.x | 结构化日志 |
| uniffi | 0.28.x | FFI 绑定生成 |
| thiserror | latest | 错误类型定义 |

### Platform Clients
| 平台 | 语言/框架 | 阶段 |
|------|----------|------|
| macOS | Swift/SwiftUI | Phase 1 |
| Android | Kotlin/Compose | Phase 1 |
| Windows | C#/WinUI | Phase 2 |
| iOS | Swift/SwiftUI | Phase 2 |

### Version Constraints
- uniffi 0.28.x 与 Rust 2021 兼容
- rustls 需要 tokio 1.x 异步支持
- Android minSdk: 26 (Android 8.0)
- macOS 最低支持: 12.0

---

## Critical Implementation Rules

### Rust 语言规则

**错误处理（强制）：**
- ✅ 所有公开函数必须返回 `Result<T, NearClipError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**统一错误类型：**
```rust
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
```

**命名约定：**
| 元素 | 约定 | 示例 |
|------|------|------|
| 模块名 | snake_case | `device_manager` |
| 函数名 | snake_case | `send_clipboard` |
| 类型名 | PascalCase | `DeviceInfo` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT` |

**异步规范：**
- 使用 `tokio` 运行时，不混用其他 runtime
- 异步函数使用 `async fn`，不使用手写 Future
- 避免在异步代码中使用阻塞调用

### FFI 接口规则

**uniffi.udl 接口定义：**
```udl
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

**FFI 导出规则：**
- ✅ 所有 FFI 函数必须在 `uniffi.udl` 中声明
- ✅ Rust snake_case 自动转换为平台命名（Swift camelCase，Kotlin camelCase）
- ✅ 可能失败的函数必须标记 `[Throws=NearClipError]`
- ✅ 异步事件使用 callback interface 模式
- ❌ 禁止直接暴露内部类型，使用 DTO

**跨语言类型映射：**
| Rust | Swift | Kotlin |
|------|-------|--------|
| `String` | `String` | `String` |
| `Vec<u8>` | `Data` | `ByteArray` |
| `Option<T>` | `T?` | `T?` |
| `Result<T, E>` | throws | throws |

### 测试规则

**测试组织：**
```
crates/nearclip-core/
├── src/
│   ├── lib.rs
│   └── device.rs      # 包含 #[cfg(test)] mod tests
└── tests/
    └── integration.rs  # 集成测试
```

**测试分类：**
| 类型 | 位置 | 用途 |
|------|------|------|
| 单元测试 | 模块内 `#[cfg(test)]` | 测试单个函数/模块 |
| 集成测试 | `crates/xxx/tests/` | 测试模块间交互 |
| FFI 测试 | 平台客户端测试目录 | 测试跨语言调用 |

**测试规范：**
- ✅ 每个公开函数至少一个测试
- ✅ 异步测试使用 `#[tokio::test]`
- ✅ 网络/BLE 测试使用 mock，不依赖真实硬件
- ✅ 测试命名：`test_<function>_<scenario>`
- ❌ 禁止测试间共享可变状态

**Mock 策略：**
- 网络层：使用 trait 抽象，测试时注入 mock
- BLE 层：模拟设备发现和连接
- 加密层：使用固定密钥对测试

### 日志规则

**使用 tracing，统一格式：**
```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(content))]
pub async fn sync_clipboard(device_id: &str, content: &str) -> Result<()> {
    info!(device_id, content_len = content.len(), "Syncing clipboard");
    // ...
}
```

**日志级别规范：**
| 级别 | 用途 | 示例 |
|------|------|------|
| `error` | 需要用户干预的问题 | 配对失败、网络不可用 |
| `warn` | 可恢复的问题 | 重试中、通道切换 |
| `info` | 重要业务事件 | 配对成功、同步完成 |
| `debug` | 开发调试信息 | 函数入口、状态变化 |
| `trace` | 详细追踪 | 网络包、BLE 帧 |

**日志规范：**
- ✅ 使用 `#[instrument]` 自动记录函数调用
- ✅ 敏感数据使用 `skip()` 排除
- ❌ 禁止使用 `println!()` 替代 tracing
- ❌ 禁止在日志中记录剪贴板明文内容

### 消息协议规则

**统一消息结构（强制）：**
```rust
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

**协议规范：**
- ✅ 所有网络消息必须遵循 `Message` 结构
- ✅ payload 使用 MessagePack 序列化
- ✅ timestamp 使用 Unix 毫秒时间戳
- ❌ 禁止自定义消息格式
- ❌ 禁止使用 JSON 序列化（性能考虑）

**通道策略：**
- WiFi 优先，延迟 < 100ms 时保持
- BLE 作为备选，自动切换
- 失败重试 3 次后提示用户

---

## Critical Don't-Miss Rules

### 禁止的反模式

**Rust 代码：**
- ❌ 使用 `unwrap()` 或 `expect()` 在库代码中
- ❌ 直接 `println!()` 替代 tracing
- ❌ 自定义消息格式不遵循 Message 结构
- ❌ 跨模块直接访问私有类型
- ❌ 在异步代码中使用 `std::thread::sleep`

**架构违规：**
- ❌ 平台客户端直接调用非 FFI 的 Rust 代码
- ❌ 绕过 nearclip-core 直接使用底层 crate
- ❌ 在 FFI 层处理业务逻辑

### 安全规则

- ✅ 所有网络通信必须使用 TLS 1.3
- ✅ 密钥存储使用平台密钥库（Keychain/Keystore）
- ❌ 禁止明文存储配对密钥
- ❌ 禁止在日志中输出敏感数据
- ❌ 禁止跳过证书验证

### 边界情况处理

- 网络断开时：缓存待发送内容，重连后自动发送
- BLE MTU 限制：自动分片，接收端重组
- 剪贴板为空：忽略，不发送空消息
- 设备离线：3 次重试后通知用户

---

## Usage Guidelines

**For AI Agents:**
- 实现代码前必须阅读本文件
- 严格遵循所有规则
- 遇到不确定时，选择更严格的选项
- 发现新模式时更新本文件

**For Humans:**
- 保持文件精简，专注于 AI Agent 需求
- 技术栈变更时更新
- 定期审查过时规则
- 移除随时间变得显而易见的规则

---

Last Updated: 2025-12-13

