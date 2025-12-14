# Story 2.3: 实现 mDNS 服务广播

Status: done

## Story

As a 用户,
I want 设备在局域网广播自己的存在,
So that 其他设备可以发现我.

## Acceptance Criteria

1. **Given** nearclip-net crate 已创建 **When** 启动 mDNS 服务广播 **Then** 使用 `_nearclip._tcp.local` 服务类型
2. **And** 广播包含设备 ID 和公钥哈希（作为 TXT 记录）
3. **And** 可以停止广播
4. **And** 测试验证服务注册成功

## Tasks / Subtasks

- [x] Task 1: 创建 mDNS 模块结构 (AC: 1)
  - [x] 1.1 创建 `crates/nearclip-net/src/mdns/mod.rs` 模块入口
  - [x] 1.2 创建 `crates/nearclip-net/src/mdns/advertise.rs` 广播实现
  - [x] 1.3 在 `lib.rs` 中添加 `pub mod mdns;`
  - [x] 1.4 定义 `NetError` 错误类型（Mdns 变体）

- [x] Task 2: 定义服务广播配置 (AC: 1, 2)
  - [x] 2.1 创建 `MdnsServiceConfig` 结构体，包含 device_id、public_key_hash、port
  - [x] 2.2 定义服务类型常量 `SERVICE_TYPE = "_nearclip._tcp.local."`
  - [x] 2.3 定义 TXT 记录键名常量 `TXT_DEVICE_ID = "id"`, `TXT_PUBKEY_HASH = "pk"`

- [x] Task 3: 实现 MdnsAdvertiser (AC: 1, 2, 3)
  - [x] 3.1 创建 `MdnsAdvertiser` 结构体，封装 mdns-sd ServiceDaemon
  - [x] 3.2 实现 `MdnsAdvertiser::new(config: MdnsServiceConfig)` 构造函数
  - [x] 3.3 实现 `start()` 方法注册服务
  - [x] 3.4 实现 `stop()` 方法注销服务
  - [x] 3.5 使用 tokio 异步处理 ServiceDaemon 事件

- [x] Task 4: 实现 TXT 记录构建 (AC: 2)
  - [x] 4.1 将 device_id 添加为 `id=<device_id>` TXT 记录
  - [x] 4.2 将 public_key_hash 添加为 `pk=<hash>` TXT 记录
  - [x] 4.3 确保 TXT 记录总长度 < 255 字节（DNS 限制）

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `mdns/mod.rs` 导出 `MdnsAdvertiser`, `MdnsServiceConfig`
  - [x] 5.2 在 `lib.rs` 重新导出主要类型

- [x] Task 6: 编写单元测试 (AC: 4)
  - [x] 6.1 测试 `MdnsServiceConfig` 创建成功
  - [x] 6.2 测试 `MdnsAdvertiser` 创建成功
  - [x] 6.3 测试 TXT 记录构建正确
  - [x] 6.4 测试服务名称格式正确

- [x] Task 7: 编写集成测试 (AC: 4)
  - [x] 7.1 创建 `tests/mdns_advertise_integration.rs`
  - [x] 7.2 测试服务注册成功（监听 ServiceDaemon 事件）
  - [x] 7.3 测试服务注销成功
  - [x] 7.4 测试重复注册处理

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-net` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-net` 确保测试通过
  - [x] 8.3 运行 `cargo clippy -p nearclip-net` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, NetError>` 或 `Result<T, NearClipError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**日志规则：**
- ✅ 使用 `tracing` 记录日志
- ✅ 服务注册成功使用 `info!` 级别
- ✅ 服务注册失败使用 `error!` 级别
- ❌ 禁止使用 `println!()`

**网络模块在架构中的位置：**
```
nearclip-ffi → nearclip-core → nearclip-sync → nearclip-net → nearclip-crypto
                                                    ↓
                                               nearclip-ble
```
nearclip-net 依赖 nearclip-crypto（用于公钥哈希计算）。

### Previous Story (2-2) Intelligence

**关键学习：**
- rustls 0.23 需要显式指定 CryptoProvider
- 使用 `time` crate 设置证书有效期
- 测试文件放在 `tests/` 目录下进行集成测试
- zeroize 用于敏感数据清理

**代码复用：**
- 可使用 `nearclip-crypto` 的公钥导出功能计算公钥哈希

### mdns-sd API 使用

**服务注册（广播）：**
```rust
use mdns_sd::{ServiceDaemon, ServiceInfo};

// 创建 ServiceDaemon
let mdns = ServiceDaemon::new().expect("Failed to create daemon");

// 定义服务
let service_type = "_nearclip._tcp.local.";
let instance_name = "device-uuid";  // 设备 ID
let host_name = "hostname.local.";
let port = 12345;

let properties = [
    ("id", device_id),      // 设备 ID
    ("pk", pubkey_hash),    // 公钥哈希（base64）
];

let service_info = ServiceInfo::new(
    service_type,
    instance_name,
    host_name,
    (),  // 自动解析 IP
    port,
    &properties[..],
)?;

// 注册服务
mdns.register(service_info)?;

// 监听事件
let receiver = mdns.browse(service_type)?;
while let Ok(event) = receiver.recv() {
    match event {
        ServiceEvent::ServiceRegistered(name) => {
            println!("Registered: {}", name);
        }
        // ...
    }
}

// 注销服务
mdns.unregister(&full_name)?;
```

**mdns-sd 异步处理：**
```rust
use tokio::sync::mpsc;

// 在 tokio runtime 中运行
async fn run_mdns() {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse(SERVICE_TYPE)?;

    loop {
        tokio::select! {
            event = tokio::task::spawn_blocking(move || receiver.recv()) => {
                // 处理事件
            }
            _ = shutdown_signal => break,
        }
    }
}
```

### 服务类型设计

**mDNS 服务类型格式：**
- `_nearclip._tcp.local.` - NearClip 专用服务类型
- `_tcp` 表示使用 TCP 协议
- `.local.` 表示本地网络

**实例名称：**
- 使用设备 ID（UUID 格式）作为实例名称
- 完整服务名：`<device-id>._nearclip._tcp.local.`

**TXT 记录设计：**
| 键 | 值 | 说明 |
|----|-----|------|
| `id` | `abc123...` | 设备 ID（用于识别） |
| `pk` | `base64...` | 公钥 SHA-256 哈希的 Base64 编码（32 字节 → 44 字符） |

### 文件位置

**目标文件：**
- `crates/nearclip-net/src/mdns/mod.rs` - mDNS 模块入口（新建）
- `crates/nearclip-net/src/mdns/advertise.rs` - 服务广播实现（新建）
- `crates/nearclip-net/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-net/src/error.rs` - 错误类型定义（新建）

### 依赖变更

**nearclip-net/Cargo.toml（已有）：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
tokio.workspace = true
mdns-sd.workspace = true
nearclip-crypto.workspace = true
```

**可能需要添加：**
```toml
[dependencies]
base64.workspace = true  # 用于公钥哈希编码
sha2 = "0.10"            # 用于计算公钥哈希（如果不使用 crypto crate）
```

### 代码模板

**NetError 定义（error.rs）：**
```rust
use thiserror::Error;

/// 网络层错误类型
#[derive(Debug, Error)]
pub enum NetError {
    #[error("mDNS error: {0}")]
    Mdns(String),

    #[error("Service registration failed: {0}")]
    ServiceRegistration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

**MdnsServiceConfig（advertise.rs）：**
```rust
/// mDNS 服务配置
#[derive(Debug, Clone)]
pub struct MdnsServiceConfig {
    /// 设备唯一标识符
    pub device_id: String,
    /// 公钥哈希（Base64 编码）
    pub public_key_hash: String,
    /// 服务监听端口
    pub port: u16,
    /// 可选的主机名
    pub hostname: Option<String>,
}

impl MdnsServiceConfig {
    pub fn new(device_id: String, public_key_hash: String, port: u16) -> Self {
        Self {
            device_id,
            public_key_hash,
            port,
            hostname: None,
        }
    }
}
```

**MdnsAdvertiser（advertise.rs）：**
```rust
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, instrument};

use crate::error::NetError;

/// 服务类型常量
pub const SERVICE_TYPE: &str = "_nearclip._tcp.local.";
/// TXT 记录：设备 ID
pub const TXT_DEVICE_ID: &str = "id";
/// TXT 记录：公钥哈希
pub const TXT_PUBKEY_HASH: &str = "pk";

/// mDNS 服务广播器
pub struct MdnsAdvertiser {
    daemon: Arc<Mutex<ServiceDaemon>>,
    config: MdnsServiceConfig,
    service_fullname: Option<String>,
}

impl MdnsAdvertiser {
    /// 创建新的广播器
    #[instrument(skip(config))]
    pub fn new(config: MdnsServiceConfig) -> Result<Self, NetError> {
        let daemon = ServiceDaemon::new()
            .map_err(|e| NetError::Mdns(e.to_string()))?;

        debug!(device_id = %config.device_id, "Created mDNS advertiser");

        Ok(Self {
            daemon: Arc::new(Mutex::new(daemon)),
            config,
            service_fullname: None,
        })
    }

    /// 启动服务广播
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<(), NetError> {
        let daemon = self.daemon.lock().await;

        let hostname = self.config.hostname
            .clone()
            .unwrap_or_else(|| format!("{}.local.", self.config.device_id));

        let properties = [
            (TXT_DEVICE_ID, self.config.device_id.as_str()),
            (TXT_PUBKEY_HASH, self.config.public_key_hash.as_str()),
        ];

        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &self.config.device_id,
            &hostname,
            (),
            self.config.port,
            &properties[..],
        ).map_err(|e| NetError::ServiceRegistration(e.to_string()))?;

        let fullname = service_info.get_fullname().to_string();

        daemon.register(service_info)
            .map_err(|e| NetError::ServiceRegistration(e.to_string()))?;

        self.service_fullname = Some(fullname.clone());

        info!(
            service_name = %fullname,
            port = self.config.port,
            "mDNS service registered"
        );

        Ok(())
    }

    /// 停止服务广播
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<(), NetError> {
        if let Some(fullname) = self.service_fullname.take() {
            let daemon = self.daemon.lock().await;

            daemon.unregister(&fullname)
                .map_err(|e| NetError::Mdns(e.to_string()))?;

            info!(service_name = %fullname, "mDNS service unregistered");
        }

        Ok(())
    }

    /// 检查是否正在广播
    pub fn is_advertising(&self) -> bool {
        self.service_fullname.is_some()
    }
}
```

**lib.rs 更新模板：**
```rust
//! NearClip Network Module
//!
//! Network layer for device discovery and TCP/TLS communication.
//! Includes mDNS service broadcasting and discovery.

pub mod error;
pub mod mdns;

// Re-export main types
pub use error::NetError;
pub use mdns::{MdnsAdvertiser, MdnsServiceConfig, SERVICE_TYPE};

// Future modules:
// mod tcp;      // TCP server and client
// mod tls;      // TLS wrapper for secure connections
```

### 集成测试模板

**tests/mdns_advertise_integration.rs:**
```rust
//! mDNS 广播集成测试

use nearclip_net::{MdnsAdvertiser, MdnsServiceConfig};

#[tokio::test]
async fn test_advertiser_start_and_stop() {
    let config = MdnsServiceConfig::new(
        "test-device-001".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),  // Base64 encoded test hash
        12345,
    );

    let mut advertiser = MdnsAdvertiser::new(config)
        .expect("Failed to create advertiser");

    assert!(!advertiser.is_advertising());

    advertiser.start().await.expect("Failed to start advertising");
    assert!(advertiser.is_advertising());

    advertiser.stop().await.expect("Failed to stop advertising");
    assert!(!advertiser.is_advertising());
}

#[tokio::test]
async fn test_advertiser_double_start() {
    let config = MdnsServiceConfig::new(
        "test-device-002".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12346,
    );

    let mut advertiser = MdnsAdvertiser::new(config)
        .expect("Failed to create advertiser");

    advertiser.start().await.expect("Failed to start advertising");
    // 第二次 start 应该安全处理（更新或忽略）
    let result = advertiser.start().await;
    // 具体行为取决于实现决策

    advertiser.stop().await.expect("Failed to stop advertising");
}
```

### References

- [Source: docs/architecture.md#Project Structure - nearclip-net 结构]
- [Source: docs/architecture.md#Rust Core Dependencies - mdns-sd latest]
- [Source: docs/epics.md#Story 2.3 - 验收标准]
- [Source: docs/project_context.md#Technology Stack - 版本约束]
- [Source: docs/sprint-artifacts/2-2-tls-1-3-config.md - 错误处理模式]
- [mdns-sd 官方文档](https://docs.rs/mdns-sd)
- [mDNS RFC 6762](https://www.rfc-editor.org/rfc/rfc6762)
- [DNS-SD RFC 6763](https://www.rfc-editor.org/rfc/rfc6763)

## Dev Agent Record

### Context Reference

Story context created by create-story workflow with exhaustive artifact analysis.

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

无调试问题

### Completion Notes List

1. **NetError 类型定义**: 创建了包含 Mdns、ServiceRegistration、Io、Configuration 四个变体的错误类型
2. **MdnsServiceConfig**: 实现了完整的配置结构，包含验证逻辑和 TXT 属性构建
3. **MdnsAdvertiser**: 使用 Arc<Mutex<ServiceDaemon>> 封装，支持 tokio 异步操作
4. **TXT 记录**: 使用 `id` 和 `pk` 键名，验证总长度不超过 255 字节
5. **测试覆盖**: 16 个单元测试 + 7 个集成测试 + 4 个文档测试 = 27 个测试全部通过
6. **无需额外依赖**: 当前实现不需要 base64 或 sha2，公钥哈希由调用方提供

### File List

**Created:**
- `crates/nearclip-net/src/error.rs` - 网络错误类型（~50 行）
- `crates/nearclip-net/src/mdns/mod.rs` - mDNS 模块入口（~10 行）
- `crates/nearclip-net/src/mdns/advertise.rs` - 服务广播实现（~330 行）
- `crates/nearclip-net/tests/mdns_advertise_integration.rs` - 集成测试（~120 行）

**Modified:**
- `crates/nearclip-net/src/lib.rs` - 导出模块和类型

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] error.rs 文件已创建，定义 NetError
- [x] mdns/mod.rs 模块入口已创建
- [x] mdns/advertise.rs 广播实现已创建
- [x] MdnsServiceConfig 结构体完整
- [x] MdnsAdvertiser 实现 start/stop 方法
- [x] 使用 `_nearclip._tcp.local.` 服务类型
- [x] TXT 记录包含 device_id 和 public_key_hash
- [x] 所有单元测试通过
- [x] 集成测试验证服务注册/注销
- [x] `cargo build -p nearclip-net` 成功
- [x] `cargo test -p nearclip-net` 成功 (27 tests)
- [x] `cargo clippy -p nearclip-net` 无警告

---

## Code Review Record

### Review Date: 2025-12-14

**Reviewer:** Claude Opus 4.5 (code-review workflow)

**Issues Found:** 4 High, 4 Medium, 3 Low

**Issues Fixed:**
1. **H1** [FIXED]: 添加 Drop trait 实现，防止服务泄漏
2. **H2** [FIXED]: 修复 TXT 记录长度验证（单条记录 ≤ 255 字节）
3. **H3** [FIXED]: 添加主机名 DNS 格式验证
4. **H4** [FIXED]: 添加 TXT 记录配置验证测试
5. **M2** [DEFERRED]: 错误类型保留上下文 - 依赖 mdns-sd crate 限制
6. **L2** [FIXED]: 主机名边界条件处理

**Test Results After Fix:**
- Unit tests: 20 passed
- Integration tests: 8 passed
- Doc tests: 4 passed
- Total: 32 tests (增加 5 个验证测试)
- Workspace total: 152 tests passed

## Change Log

| Date | Change |
|------|--------|
| 2025-12-14 | Story created by create-story workflow |
| 2025-12-14 | Implementation completed - all 8 tasks done, 27 tests passing |
| 2025-12-14 | Code review completed - 5 issues fixed, 1 deferred, 32 tests passing |

---

Story created: 2025-12-14
Implementation completed: 2025-12-14
