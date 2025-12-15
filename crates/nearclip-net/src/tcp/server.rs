//! TLS TCP 服务端
//!
//! 提供 TLS 加密的 TCP 服务端实现。
//!
//! # Example
//!
//! ```no_run
//! use nearclip_net::tcp::{TcpServer, TcpServerConfig};
//! use nearclip_crypto::{TlsCertificate, TlsServerConfig};
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//! let tls_config = TlsServerConfig::new(&cert).unwrap();
//!
//! let config = TcpServerConfig::new().with_port(8765);
//! let server = TcpServer::bind(config, tls_config.config()).await?;
//!
//! println!("Server listening on {:?}", server.local_addr()?);
//! # Ok(())
//! # }
//! ```

use crate::NetError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, info, instrument, warn};

use super::TcpConnection;

/// TCP 服务端配置
///
/// 配置 TCP 服务端的绑定地址和端口。
///
/// # Example
///
/// ```
/// use nearclip_net::tcp::TcpServerConfig;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// // 使用默认配置（绑定所有接口，动态端口）
/// let config = TcpServerConfig::new();
///
/// // 指定端口
/// let config = TcpServerConfig::new().with_port(8765);
///
/// // 指定绑定地址
/// let config = TcpServerConfig::new()
///     .with_bind_addr(IpAddr::V4(Ipv4Addr::LOCALHOST))
///     .with_port(8765);
/// ```
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
            port: 0,
        }
    }
}

impl TcpServerConfig {
    /// 创建默认配置
    ///
    /// 默认绑定所有网络接口（0.0.0.0），端口动态分配。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置绑定端口
    ///
    /// 端口 0 表示由操作系统动态分配可用端口。
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// 设置绑定地址
    pub fn with_bind_addr(mut self, addr: IpAddr) -> Self {
        self.bind_addr = addr;
        self
    }

    /// 获取完整的 socket 地址
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.bind_addr, self.port)
    }
}

/// TLS 加密的 TCP 服务端
///
/// 提供 TLS 1.3 加密的 TCP 服务端功能，支持多个并发连接。
///
/// # Example
///
/// ```no_run
/// use nearclip_net::tcp::{TcpServer, TcpServerConfig};
/// use nearclip_crypto::{TlsCertificate, TlsServerConfig};
///
/// # async fn example() -> Result<(), nearclip_net::NetError> {
/// // 生成证书和 TLS 配置
/// let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
/// let tls_config = TlsServerConfig::new(&cert).unwrap();
///
/// // 创建服务端
/// let config = TcpServerConfig::new();
/// let server = TcpServer::bind(config, tls_config.config()).await?;
///
/// // 接受连接
/// loop {
///     let conn = server.accept().await?;
///     // 处理连接...
/// }
/// # Ok(())
/// # }
/// ```
pub struct TcpServer {
    listener: TcpListener,
    tls_acceptor: TlsAcceptor,
}

impl TcpServer {
    /// 创建并绑定 TCP 服务端
    ///
    /// # Arguments
    ///
    /// * `config` - 服务端配置
    /// * `tls_config` - TLS 服务端配置
    ///
    /// # Returns
    ///
    /// 绑定成功的服务端实例，或错误
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nearclip_net::tcp::{TcpServer, TcpServerConfig};
    /// use nearclip_crypto::{TlsCertificate, TlsServerConfig};
    ///
    /// # async fn example() -> Result<(), nearclip_net::NetError> {
    /// let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
    /// let tls_config = TlsServerConfig::new(&cert).unwrap();
    ///
    /// let server = TcpServer::bind(
    ///     TcpServerConfig::new().with_port(0),
    ///     tls_config.config(),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(tls_config), fields(addr = %config.socket_addr()))]
    pub async fn bind(
        config: TcpServerConfig,
        tls_config: Arc<rustls::ServerConfig>,
    ) -> Result<Self, NetError> {
        let addr = config.socket_addr();

        let listener = TcpListener::bind(addr).await.map_err(|e| {
            NetError::TcpServer(format!("Failed to bind to {}: {}", addr, e))
        })?;

        let local_addr = listener.local_addr().map_err(|e| {
            NetError::TcpServer(format!("Failed to get local address: {}", e))
        })?;

        info!("TCP server bound to {}", local_addr);

        let tls_acceptor = TlsAcceptor::from(tls_config);

        Ok(Self {
            listener,
            tls_acceptor,
        })
    }

    /// 获取实际监听地址
    ///
    /// 当使用端口 0（动态分配）时，可通过此方法获取实际分配的端口。
    ///
    /// # Returns
    ///
    /// 服务端实际监听的地址和端口
    pub fn local_addr(&self) -> Result<SocketAddr, NetError> {
        self.listener.local_addr().map_err(|e| {
            NetError::TcpServer(format!("Failed to get local address: {}", e))
        })
    }

    /// 接受新的客户端连接
    ///
    /// 此方法会阻塞直到有新连接到达，完成 TLS 握手后返回连接对象。
    ///
    /// # Returns
    ///
    /// TLS 加密的连接对象，或错误
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nearclip_net::tcp::{TcpServer, TcpServerConfig};
    /// use nearclip_crypto::{TlsCertificate, TlsServerConfig};
    ///
    /// # async fn example() -> Result<(), nearclip_net::NetError> {
    /// # let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
    /// # let tls_config = TlsServerConfig::new(&cert).unwrap();
    /// # let server = TcpServer::bind(TcpServerConfig::new(), tls_config.config()).await?;
    /// let conn = server.accept().await?;
    /// println!("New connection from {:?}", conn.peer_addr());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn accept(&self) -> Result<TcpConnection, NetError> {
        // 等待 TCP 连接
        let (tcp_stream, peer_addr) = self.listener.accept().await.map_err(|e| {
            NetError::TcpServer(format!("Failed to accept connection: {}", e))
        })?;

        debug!("TCP connection from {}, starting TLS handshake", peer_addr);

        // 执行 TLS 握手
        let tls_stream = self.tls_acceptor.accept(tcp_stream).await.map_err(|e| {
            warn!("TLS handshake failed with {}: {}", peer_addr, e);
            NetError::TlsHandshake(format!("Handshake failed with {}: {}", peer_addr, e))
        })?;

        info!("TLS connection established with {}", peer_addr);

        Ok(TcpConnection::new(tls_stream, peer_addr))
    }
}

impl std::fmt::Debug for TcpServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpServer")
            .field("local_addr", &self.listener.local_addr().ok())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_server_config_default() {
        let config = TcpServerConfig::default();
        assert_eq!(config.bind_addr, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert_eq!(config.port, 0);
    }

    #[test]
    fn test_tcp_server_config_new() {
        let config = TcpServerConfig::new();
        assert_eq!(config.bind_addr, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert_eq!(config.port, 0);
    }

    #[test]
    fn test_tcp_server_config_with_port() {
        let config = TcpServerConfig::new().with_port(8765);
        assert_eq!(config.port, 8765);
    }

    #[test]
    fn test_tcp_server_config_with_bind_addr() {
        let config = TcpServerConfig::new()
            .with_bind_addr(IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(config.bind_addr, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn test_tcp_server_config_socket_addr() {
        let config = TcpServerConfig::new()
            .with_bind_addr(IpAddr::V4(Ipv4Addr::LOCALHOST))
            .with_port(8765);
        let addr = config.socket_addr();
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(addr.port(), 8765);
    }

    #[test]
    fn test_tcp_server_config_builder_chain() {
        let config = TcpServerConfig::new()
            .with_bind_addr(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)))
            .with_port(12345);

        assert_eq!(config.bind_addr, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(config.port, 12345);
    }
}
