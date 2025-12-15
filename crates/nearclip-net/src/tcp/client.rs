//! TLS TCP 客户端
//!
//! 提供 TLS 加密的 TCP 客户端连接功能。
//!
//! # Example
//!
//! ```no_run
//! use nearclip_net::tcp::{TcpClient, TcpClientConfig};
//! use nearclip_crypto::{TlsCertificate, TlsClientConfig};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // 获取服务端证书（通常通过配对流程获取）
//! let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//!
//! // 创建客户端 TLS 配置（信任服务端证书）
//! let tls_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
//!
//! // 连接到服务端
//! let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
//! let config = TcpClientConfig::new(addr);
//! let mut conn = TcpClient::connect(config, tls_config.config(), "localhost").await?;
//!
//! // 使用连接
//! conn.write_all(b"Hello, server!").await?;
//! # Ok(())
//! # }
//! ```

use crate::NetError;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::TlsConnector;
use tracing::{debug, info, instrument, warn};

use super::TcpConnection;

/// 默认连接超时时间（10 秒）
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// TCP 客户端配置
///
/// 配置 TCP 客户端的连接参数。
///
/// # Example
///
/// ```
/// use nearclip_net::tcp::TcpClientConfig;
/// use std::net::SocketAddr;
/// use std::time::Duration;
///
/// let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
///
/// // 使用默认超时（10 秒）
/// let config = TcpClientConfig::new(addr);
///
/// // 自定义超时
/// let config = TcpClientConfig::new(addr)
///     .with_timeout(Duration::from_secs(5));
/// ```
#[derive(Debug, Clone)]
pub struct TcpClientConfig {
    /// 目标服务端地址
    pub target_addr: SocketAddr,
    /// 连接超时时间
    pub connect_timeout: Duration,
}

impl TcpClientConfig {
    /// 创建客户端配置
    ///
    /// 使用默认超时时间（10 秒）。
    ///
    /// # Arguments
    ///
    /// * `target_addr` - 目标服务端地址
    pub fn new(target_addr: SocketAddr) -> Self {
        Self {
            target_addr,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
        }
    }

    /// 设置连接超时
    ///
    /// # Arguments
    ///
    /// * `timeout` - 超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
}

/// TLS TCP 客户端
///
/// 提供连接到 TLS 服务端的功能。
/// 使用 TOFU (Trust On First Use) 信任模型验证服务端身份。
///
/// # Example
///
/// ```no_run
/// use nearclip_net::tcp::{TcpClient, TcpClientConfig};
/// use nearclip_crypto::{TlsCertificate, TlsClientConfig};
/// use std::net::SocketAddr;
///
/// # async fn example() -> Result<(), nearclip_net::NetError> {
/// # let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
/// let tls_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
///
/// let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
/// let config = TcpClientConfig::new(addr);
///
/// // 连接到服务端
/// let conn = TcpClient::connect(config, tls_config.config(), "localhost").await?;
/// println!("Connected to {:?}", conn.peer_addr());
/// # Ok(())
/// # }
/// ```
pub struct TcpClient;

impl TcpClient {
    /// 连接到目标服务端
    ///
    /// 建立 TCP 连接并完成 TLS 握手。
    /// TLS 配置中的证书用于验证服务端身份（TOFU 模型）。
    ///
    /// # Arguments
    ///
    /// * `config` - 客户端配置
    /// * `tls_config` - TLS 客户端配置（包含信任的服务端证书）
    /// * `server_name` - 服务端名称（用于 TLS SNI 和证书验证）
    ///
    /// # Returns
    ///
    /// TLS 加密的连接对象，或错误
    ///
    /// # Errors
    ///
    /// - `ConnectionTimeout` - 连接超时
    /// - `ConnectionFailed` - TCP 连接失败
    /// - `TlsHandshake` - TLS 握手失败（通常是证书不匹配）
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nearclip_net::tcp::{TcpClient, TcpClientConfig};
    /// use nearclip_crypto::{TlsCertificate, TlsClientConfig};
    /// use std::net::SocketAddr;
    ///
    /// # async fn example() -> Result<(), nearclip_net::NetError> {
    /// # let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
    /// let tls_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
    ///
    /// let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
    /// let config = TcpClientConfig::new(addr);
    ///
    /// let conn = TcpClient::connect(config, tls_config.config(), "localhost").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(tls_config), fields(target = %config.target_addr, timeout = ?config.connect_timeout))]
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
        let server_name_owned = rustls::pki_types::ServerName::try_from(server_name.to_string())
            .map_err(|_| NetError::TlsHandshake(format!("Invalid server name: {}", server_name)))?;

        let tls_stream = connector
            .connect(server_name_owned, tcp_stream)
            .await
            .map_err(|e| {
                warn!("TLS handshake failed with {}: {}", config.target_addr, e);
                NetError::TlsHandshake(format!("Handshake failed with {}: {}", config.target_addr, e))
            })?;

        info!("TLS connection established with {}", config.target_addr);

        // 3. 返回连接对象
        Ok(TcpConnection::new_client(tls_stream, config.target_addr))
    }
}

impl std::fmt::Debug for TcpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpClient").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_tcp_client_config_new() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8765);
        let config = TcpClientConfig::new(addr);

        assert_eq!(config.target_addr, addr);
        assert_eq!(config.connect_timeout, DEFAULT_CONNECT_TIMEOUT);
    }

    #[test]
    fn test_tcp_client_config_with_timeout() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8765);
        let timeout = Duration::from_secs(5);
        let config = TcpClientConfig::new(addr).with_timeout(timeout);

        assert_eq!(config.connect_timeout, timeout);
    }

    #[test]
    fn test_tcp_client_config_builder_chain() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 12345);
        let timeout = Duration::from_millis(500);

        let config = TcpClientConfig::new(addr).with_timeout(timeout);

        assert_eq!(config.target_addr.ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(config.target_addr.port(), 12345);
        assert_eq!(config.connect_timeout, timeout);
    }

    #[test]
    fn test_tcp_client_config_debug() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8765);
        let config = TcpClientConfig::new(addr);

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("TcpClientConfig"));
        assert!(debug_str.contains("target_addr"));
        assert!(debug_str.contains("connect_timeout"));
    }

    #[test]
    fn test_tcp_client_debug() {
        let debug_str = format!("{:?}", TcpClient);
        assert!(debug_str.contains("TcpClient"));
    }

    #[test]
    fn test_default_connect_timeout() {
        assert_eq!(DEFAULT_CONNECT_TIMEOUT, Duration::from_secs(10));
    }
}
