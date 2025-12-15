# Story 3.2: 实现 TCP 客户端连接

Status: done

## Story

As a 用户,
I want 设备能连接到其他设备,
So that 可以发送剪贴板内容.

## Acceptance Criteria

1. **Given** 已知目标设备 IP 和端口 **When** 建立 TCP 客户端连接 **Then** 完成 TLS 握手
2. **And** 验证目标设备身份（公钥匹配）
3. **And** 连接失败时返回错误
4. **And** 测试验证双向通信

## Tasks / Subtasks

- [x] Task 1: 扩展 NetError 错误类型 (AC: 3)
  - [x] 1.1 添加 `ConnectionFailed(String)` 错误变体
  - [x] 1.2 添加 `ConnectionTimeout(String)` 错误变体
  - [x] 1.3 保持与现有错误类型兼容

- [x] Task 2: 定义 TCP 客户端配置 (AC: 1)
  - [x] 2.1 创建 `TcpClientConfig` 结构体
  - [x] 2.2 支持目标地址配置（SocketAddr）
  - [x] 2.3 支持连接超时配置（默认 10 秒）
  - [x] 2.4 使用 Builder pattern

- [x] Task 3: 实现 TLS 客户端连接 (AC: 1, 2)
  - [x] 3.1 创建 `crates/nearclip-net/src/tcp/client.rs`
  - [x] 3.2 创建 `TcpClient` 结构体
  - [x] 3.3 实现 `TcpClient::connect(config, tls_config)` → 连接并完成 TLS 握手
  - [x] 3.4 使用 `TlsClientConfig` 进行 TOFU 身份验证
  - [x] 3.5 连接超时处理

- [x] Task 4: 复用连接抽象 (AC: 4)
  - [x] 4.1 `TcpClient::connect()` 返回 `TcpConnection`
  - [x] 4.2 确保 `TcpConnection` 支持双向读写
  - [x] 4.3 服务端和客户端使用相同的连接抽象

- [x] Task 5: 导出模块 (AC: 1)
  - [x] 5.1 在 `tcp/mod.rs` 中添加 `mod client;`
  - [x] 5.2 添加 re-exports: `TcpClient`, `TcpClientConfig`
  - [x] 5.3 在 `lib.rs` 中添加 re-exports

- [x] Task 6: 编写单元测试 (AC: 3)
  - [x] 6.1 测试 `TcpClientConfig` builder
  - [x] 6.2 测试默认超时值
  - [x] 6.3 测试配置有效性

- [x] Task 7: 编写集成测试 (AC: 1, 2, 4)
  - [x] 7.1 创建 `tests/tcp_client_integration.rs`
  - [x] 7.2 测试：客户端连接服务端 → TLS 握手成功
  - [x] 7.3 测试：客户端发送数据 → 服务端接收
  - [x] 7.4 测试：服务端发送数据 → 客户端接收
  - [x] 7.5 测试：连接不存在的服务端 → 返回错误
  - [x] 7.6 测试：证书不匹配 → TLS 握手失败

- [x] Task 8: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 8.1 运行 `cargo build -p nearclip-net` 确保无错误
  - [x] 8.2 运行 `cargo test -p nearclip-net` 确保测试通过
  - [x] 8.3 运行 `cargo clippy -p nearclip-net` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- 所有公开函数必须返回 `Result<T, NetError>`
- 禁止在库代码中使用 `unwrap()` 或 `expect()`
- 禁止使用 `panic!()` 宏
- 使用 `thiserror` 定义错误类型

**日志规则：**
- 使用 `tracing` 记录日志
- 连接建立/关闭使用 `info!` 级别
- 数据传输使用 `debug!` 级别
- 错误使用 `warn!` 或 `error!` 级别
- 禁止使用 `println!()`

**异步运行时：**
- 使用 tokio 1.x
- TCP 操作使用 `tokio::net::TcpStream`
- TLS 使用 `tokio-rustls`

### 已有代码可复用

**nearclip-crypto/src/tls_config.rs (Story 2-2):**
```rust
pub struct TlsClientConfig { ... }

impl TlsClientConfig {
    pub fn new(trusted_cert_der: &[u8]) -> Result<Self, CryptoError>;
    pub fn config(&self) -> Arc<rustls::ClientConfig>;
}
```

**nearclip-net/src/tcp/connection.rs (Story 3-1):**
```rust
pub struct TcpConnection {
    stream: TlsStream<TcpStream>,
    peer_addr: SocketAddr,
}

impl TcpConnection {
    pub fn peer_addr(&self) -> SocketAddr;
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetError>;
    pub async fn write(&mut self, data: &[u8]) -> Result<usize, NetError>;
    pub async fn write_all(&mut self, data: &[u8]) -> Result<(), NetError>;
    pub async fn flush(&mut self) -> Result<(), NetError>;
    pub async fn close(&mut self) -> Result<(), NetError>;
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
    #[error("TCP server error: {0}")]
    TcpServer(String),
    #[error("TLS handshake error: {0}")]
    TlsHandshake(String),
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),
}
```

### 设计决策

**TcpClientConfig 结构：**
```rust
use std::net::SocketAddr;
use std::time::Duration;

/// TCP 客户端配置
#[derive(Debug, Clone)]
pub struct TcpClientConfig {
    /// 目标服务端地址
    pub target_addr: SocketAddr,
    /// 连接超时时间
    pub connect_timeout: Duration,
}

impl TcpClientConfig {
    pub fn new(target_addr: SocketAddr) -> Self;
    pub fn with_timeout(mut self, timeout: Duration) -> Self;
}
```

**TcpClient 结构：**
```rust
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// TLS TCP 客户端
pub struct TcpClient;

impl TcpClient {
    /// 连接到目标服务端
    ///
    /// 建立 TCP 连接并完成 TLS 握手。
    /// TLS 配置中的证书用于验证服务端身份（TOFU 模型）。
    pub async fn connect(
        config: TcpClientConfig,
        tls_config: Arc<rustls::ClientConfig>,
        server_name: &str,
    ) -> Result<TcpConnection, NetError>;
}
```

**客户端 TcpConnection 复用：**

当前 `TcpConnection` 使用 `TlsStream<TcpStream>` 封装服务端接受的连接。
客户端连接返回 `tokio_rustls::client::TlsStream<TcpStream>`。

为复用 `TcpConnection`，需要：
1. 修改 `TcpConnection` 支持客户端和服务端两种 TLS 流
2. 或创建通用的枚举类型封装两种流

**推荐方案：使用枚举封装**
```rust
enum TlsStreamWrapper {
    Server(tokio_rustls::server::TlsStream<TcpStream>),
    Client(tokio_rustls::client::TlsStream<TcpStream>),
}

impl TlsStreamWrapper {
    // 统一的 AsyncRead + AsyncWrite 实现
}
```

### 依赖说明

**已有依赖（无需添加）：**
```toml
[dependencies]
tokio.workspace = true
tokio-rustls.workspace = true
rustls.workspace = true
thiserror.workspace = true
tracing.workspace = true
nearclip-crypto.workspace = true
```

### 代码模板

**tcp/client.rs:**
```rust
//! TLS TCP 客户端
//!
//! 提供 TLS 加密的 TCP 客户端连接功能。

use crate::NetError;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::TlsConnector;
use tracing::{debug, info, instrument, warn};

use super::TcpConnection;

/// 默认连接超时时间
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// TCP 客户端配置
#[derive(Debug, Clone)]
pub struct TcpClientConfig {
    /// 目标服务端地址
    pub target_addr: SocketAddr,
    /// 连接超时时间
    pub connect_timeout: Duration,
}

impl TcpClientConfig {
    /// 创建客户端配置
    pub fn new(target_addr: SocketAddr) -> Self {
        Self {
            target_addr,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
        }
    }

    /// 设置连接超时
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
}

/// TLS TCP 客户端
pub struct TcpClient;

impl TcpClient {
    /// 连接到目标服务端
    #[instrument(skip(tls_config), fields(target = %config.target_addr))]
    pub async fn connect(
        config: TcpClientConfig,
        tls_config: Arc<rustls::ClientConfig>,
        server_name: &str,
    ) -> Result<TcpConnection, NetError> {
        // 1. 建立 TCP 连接（带超时）
        let tcp_stream = timeout(
            config.connect_timeout,
            TcpStream::connect(config.target_addr),
        )
        .await
        .map_err(|_| {
            NetError::ConnectionTimeout(format!(
                "Connection to {} timed out after {:?}",
                config.target_addr, config.connect_timeout
            ))
        })?
        .map_err(|e| {
            NetError::ConnectionFailed(format!(
                "Failed to connect to {}: {}",
                config.target_addr, e
            ))
        })?;

        debug!("TCP connection established to {}", config.target_addr);

        // 2. 执行 TLS 握手
        let connector = TlsConnector::from(tls_config);
        let server_name = server_name.try_into().map_err(|_| {
            NetError::TlsHandshake(format!("Invalid server name: {}", server_name))
        })?;

        let tls_stream = connector.connect(server_name, tcp_stream).await.map_err(|e| {
            warn!("TLS handshake failed with {}: {}", config.target_addr, e);
            NetError::TlsHandshake(format!("Handshake failed: {}", e))
        })?;

        info!("TLS connection established with {}", config.target_addr);

        // 3. 返回连接对象
        Ok(TcpConnection::new_client(tls_stream, config.target_addr))
    }
}
```

### 安全考虑

**TLS 身份验证：**
- 使用 TOFU 信任模型
- 客户端必须提供正确的服务端证书才能连接
- 证书不匹配时 TLS 握手失败

**超时处理：**
- 防止无限等待连接
- 默认 10 秒超时
- 可配置超时时间

### 文件位置

**目标文件：**
- `crates/nearclip-net/src/tcp/client.rs` - TcpClient 实现（新建）
- `crates/nearclip-net/src/tcp/mod.rs` - 模块导出（修改）
- `crates/nearclip-net/src/tcp/connection.rs` - 支持客户端连接（修改）
- `crates/nearclip-net/src/error.rs` - 扩展错误类型（修改）
- `crates/nearclip-net/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-net/tests/tcp_client_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-crypto/src/tls_config.rs` - TLS 配置
- `crates/nearclip-net/src/tcp/server.rs` - 服务端实现（作为测试对端）
- `crates/nearclip-net/tests/tcp_server_integration.rs` - 服务端测试参考

### 与上游的关系

本 Story 依赖：
- Story 2-2 (TLS 1.3 配置): `TlsClientConfig`
- Story 3-1 (TCP 服务端): `TcpServer`, `TcpConnection`

### 与下游的关系

本 Story 为以下功能提供基础：
- Story 3.5: 剪贴板内容发送
- Story 3.7: 通道状态监测
- Story 3.8: 通道自动切换

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-net/src/tcp/client.rs` - TcpClient 实现（新建）
- `crates/nearclip-net/src/tcp/mod.rs` - 模块导出（修改）
- `crates/nearclip-net/src/tcp/connection.rs` - 支持客户端连接（修改）
- `crates/nearclip-net/src/error.rs` - 扩展错误类型（修改）
- `crates/nearclip-net/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-net/tests/tcp_client_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [ ] tcp/client.rs 文件已创建
- [ ] TcpClient::connect() 可以连接服务端
- [ ] TLS 握手成功完成
- [ ] 证书不匹配时握手失败
- [ ] 连接超时正确处理
- [ ] TcpConnection 支持客户端连接
- [ ] NetError 已扩展新变体
- [ ] 所有单元测试通过
- [ ] 集成测试验证双向通信
- [ ] `cargo build -p nearclip-net` 成功
- [ ] `cargo test -p nearclip-net` 成功
- [ ] `cargo clippy -p nearclip-net` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |

---

Story created: 2025-12-15
