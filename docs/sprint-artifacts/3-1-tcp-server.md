# Story 3.1: 实现 TCP 服务端

Status: done

## Story

As a 用户,
I want 设备能接收其他设备的连接,
So that 可以接收剪贴板内容.

## Acceptance Criteria

1. **Given** TLS 配置已完成 **When** 启动 TCP 服务端 **Then** 监听指定端口（动态分配）
2. **And** 使用 TLS 加密连接
3. **And** 支持多个并发连接
4. **And** 集成测试验证连接建立

## Tasks / Subtasks

- [x] Task 1: 扩展 NetError 错误类型 (AC: 1, 2)
  - [x] 1.1 添加 `TcpServer(String)` 错误变体
  - [x] 1.2 添加 `TlsHandshake(String)` 错误变体
  - [x] 1.3 添加 `ConnectionClosed(String)` 错误变体
  - [x] 1.4 保持与现有错误类型兼容

- [x] Task 2: 定义 TCP 服务端配置 (AC: 1)
  - [x] 2.1 创建 `crates/nearclip-net/src/tcp/mod.rs`
  - [x] 2.2 创建 `TcpServerConfig` 结构体
  - [x] 2.3 支持绑定地址配置（默认 0.0.0.0）
  - [x] 2.4 支持端口配置（默认 0 动态分配）
  - [x] 2.5 使用 Builder pattern

- [x] Task 3: 实现 TLS 服务端 (AC: 2)
  - [x] 3.1 创建 `crates/nearclip-net/src/tcp/server.rs`
  - [x] 3.2 创建 `TcpServer` 结构体
  - [x] 3.3 集成 `tokio-rustls` 和 `TlsServerConfig`
  - [x] 3.4 实现 `TcpServer::new(config, tls_config)` 构造函数
  - [x] 3.5 实现 `TcpServer::bind()` → 绑定端口
  - [x] 3.6 实现 `TcpServer::local_addr()` → 获取实际监听地址

- [x] Task 4: 实现连接接受 (AC: 1, 3)
  - [x] 4.1 实现 `TcpServer::accept()` → 接受单个连接
  - [x] 4.2 返回 `TlsStream` 封装的连接
  - [x] 4.3 完成 TLS 握手后返回
  - [x] 4.4 处理握手失败情况

- [x] Task 5: 定义连接抽象 (AC: 3)
  - [x] 5.1 创建 `crates/nearclip-net/src/tcp/connection.rs`
  - [x] 5.2 创建 `TcpConnection` 结构体封装 TLS 流
  - [x] 5.3 实现 `TcpConnection::peer_addr()` → 获取对端地址
  - [x] 5.4 实现 `TcpConnection::read()` → 读取数据
  - [x] 5.5 实现 `TcpConnection::write()` → 写入数据
  - [x] 5.6 实现 `TcpConnection::close()` → 关闭连接

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 中添加 `pub mod tcp;`
  - [x] 6.2 添加 re-exports: `TcpServer`, `TcpServerConfig`, `TcpConnection`
  - [x] 6.3 更新模块文档

- [x] Task 7: 编写单元测试 (AC: 4)
  - [x] 7.1 测试 `TcpServerConfig` builder
  - [x] 7.2 测试 `TcpServer::bind()` 成功绑定
  - [x] 7.3 测试动态端口分配（端口 0）
  - [x] 7.4 测试获取本地地址

- [x] Task 8: 编写集成测试 (AC: 4)
  - [x] 8.1 创建 `tests/tcp_server_integration.rs`
  - [x] 8.2 测试：服务端启动 → 客户端连接 → TLS 握手成功
  - [x] 8.3 测试：多客户端并发连接
  - [x] 8.4 测试：数据收发正确性

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-net` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-net` 确保测试通过
  - [x] 9.3 运行 `cargo clippy -p nearclip-net` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, NetError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**日志规则：**
- ✅ 使用 `tracing` 记录日志
- ✅ 连接建立/关闭使用 `info!` 级别
- ✅ 数据传输使用 `debug!` 级别
- ✅ 错误使用 `warn!` 或 `error!` 级别
- ❌ 禁止使用 `println!()`

**异步运行时：**
- ✅ 使用 tokio 1.x
- ✅ TCP 操作使用 `tokio::net::TcpListener`
- ✅ TLS 使用 `tokio-rustls`

### 已有代码可复用

**nearclip-crypto/src/tls_config.rs (Story 2-2):**
```rust
pub struct TlsCertificate { ... }
pub struct TlsServerConfig { ... }
pub struct TlsClientConfig { ... }

impl TlsCertificate {
    pub fn generate(subject_alt_names: &[String]) -> Result<Self, CryptoError>;
    pub fn cert_der(&self) -> Vec<u8>;
}

impl TlsServerConfig {
    pub fn new(cert: &TlsCertificate) -> Result<Self, CryptoError>;
    pub fn config(&self) -> Arc<rustls::ServerConfig>;  // 注意：实际 API 是 config() 而非 into_arc()
}
```

**nearclip-net/src/error.rs:**
```rust
#[derive(Debug, Error)]
pub enum NetError {
    #[error("mDNS error: {0}")]
    Mdns(String),
    #[error("Service registration failed: {0}")]
    ServiceRegistration(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
}
```

### 设计决策

**TcpServerConfig 结构：**
```rust
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// TCP 服务端配置
#[derive(Debug, Clone)]
pub struct TcpServerConfig {
    /// 绑定地址
    pub bind_addr: IpAddr,
    /// 绑定端口（0 表示动态分配）
    pub port: u16,
}

impl Default for TcpServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 0, // 动态分配
        }
    }
}

impl TcpServerConfig {
    pub fn new() -> Self { Self::default() }
    pub fn with_port(mut self, port: u16) -> Self { ... }
    pub fn with_bind_addr(mut self, addr: IpAddr) -> Self { ... }
    pub fn socket_addr(&self) -> SocketAddr { ... }
}
```

**TcpServer 结构：**
```rust
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

/// TLS TCP 服务端
pub struct TcpServer {
    listener: TcpListener,
    tls_acceptor: TlsAcceptor,
}

impl TcpServer {
    /// 创建并绑定服务端
    pub async fn bind(
        config: TcpServerConfig,
        tls_config: Arc<rustls::ServerConfig>,
    ) -> Result<Self, NetError>;

    /// 获取实际监听地址
    pub fn local_addr(&self) -> Result<SocketAddr, NetError>;

    /// 接受新连接
    pub async fn accept(&self) -> Result<TcpConnection, NetError>;
}
```

**TcpConnection 结构：**
```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::server::TlsStream;
use tokio::net::TcpStream;

/// TLS 加密的 TCP 连接
pub struct TcpConnection {
    stream: TlsStream<TcpStream>,
    peer_addr: SocketAddr,
}

impl TcpConnection {
    /// 获取对端地址
    pub fn peer_addr(&self) -> SocketAddr;

    /// 读取数据
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetError>;

    /// 写入数据
    pub async fn write(&mut self, data: &[u8]) -> Result<usize, NetError>;

    /// 写入所有数据
    pub async fn write_all(&mut self, data: &[u8]) -> Result<(), NetError>;

    /// 关闭连接
    pub async fn close(&mut self) -> Result<(), NetError>;
}
```

### 依赖说明

**需要添加的依赖：**
```toml
[dependencies]
tokio-rustls.workspace = true
rustls.workspace = true
```

**已有依赖（无需添加）：**
```toml
[dependencies]
tokio.workspace = true
thiserror.workspace = true
tracing.workspace = true
nearclip-crypto.workspace = true
```

### 代码模板

**tcp/mod.rs:**
```rust
//! TCP 通信模块
//!
//! 提供 TLS 加密的 TCP 服务端和客户端功能。

mod connection;
mod server;

pub use connection::TcpConnection;
pub use server::{TcpServer, TcpServerConfig};
```

**tcp/server.rs:**
```rust
//! TLS TCP 服务端
//!
//! 提供 TLS 加密的 TCP 服务端实现。

use crate::NetError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, info, instrument, warn};

use super::TcpConnection;

/// TCP 服务端配置
#[derive(Debug, Clone)]
pub struct TcpServerConfig {
    pub bind_addr: IpAddr,
    pub port: u16,
}

// ... 实现
```

### 安全考虑

**TLS 配置：**
- 使用 TLS 1.3 协议
- 使用已验证的 TlsServerConfig
- 握手失败时正确处理，不泄露信息

**并发连接：**
- 每个连接独立处理
- 使用 tokio 异步运行时
- 连接超时处理（可在后续 Story 添加）

### 文件位置

**目标文件：**
- `crates/nearclip-net/src/tcp/mod.rs` - 模块定义（新建）
- `crates/nearclip-net/src/tcp/server.rs` - TcpServer 实现（新建）
- `crates/nearclip-net/src/tcp/connection.rs` - TcpConnection 实现（新建）
- `crates/nearclip-net/src/error.rs` - 扩展错误类型（修改）
- `crates/nearclip-net/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-net/Cargo.toml` - 添加依赖（修改）
- `crates/nearclip-net/tests/tcp_server_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-crypto/src/tls_config.rs` - TLS 配置

### 与上游的关系

本 Story 依赖 Story 2-2 (TLS 1.3 配置)：
- `TlsCertificate::generate()` - 生成证书
- `TlsServerConfig::new()` - 创建服务端 TLS 配置
- `TlsServerConfig::config()` - 获取 Arc<ServerConfig>

### 与下游的关系

本 Story 为以下功能提供基础：
- Story 3.2: TCP 客户端连接
- Story 3.5: 剪贴板内容发送
- Story 3.6: 剪贴板内容接收
- Story 3.7: 通道状态监测

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-net/src/tcp/mod.rs` - TCP 模块定义（新建）
- `crates/nearclip-net/src/tcp/server.rs` - TcpServer 实现（新建）
- `crates/nearclip-net/src/tcp/connection.rs` - TcpConnection 实现（新建）
- `crates/nearclip-net/src/error.rs` - 扩展错误类型（修改）
- `crates/nearclip-net/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-net/Cargo.toml` - 添加依赖（修改）
- `crates/nearclip-net/tests/tcp_server_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] tcp/mod.rs 文件已创建
- [x] tcp/server.rs 文件已创建
- [x] tcp/connection.rs 文件已创建
- [x] TcpServer 可以绑定端口
- [x] TcpServer 可以接受 TLS 连接
- [x] TcpConnection 可以读写数据
- [x] NetError 已扩展新变体
- [x] 所有单元测试通过
- [x] 集成测试验证 TLS 连接建立
- [x] `cargo build -p nearclip-net` 成功
- [x] `cargo test -p nearclip-net` 成功
- [x] `cargo clippy -p nearclip-net` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |

---

Story created: 2025-12-15
