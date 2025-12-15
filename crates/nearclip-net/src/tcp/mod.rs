//! TCP 通信模块
//!
//! 提供 TLS 加密的 TCP 服务端和客户端功能。
//!
//! # Server Example
//!
//! ```no_run
//! use nearclip_net::tcp::{TcpServer, TcpServerConfig};
//! use nearclip_crypto::{TlsCertificate, TlsServerConfig};
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // 生成 TLS 证书
//! let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//! let tls_config = TlsServerConfig::new(&cert).unwrap();
//!
//! // 创建服务端
//! let config = TcpServerConfig::new().with_port(0);
//! let server = TcpServer::bind(config, tls_config.config()).await?;
//!
//! println!("Listening on: {:?}", server.local_addr());
//!
//! // 接受连接
//! let conn = server.accept().await?;
//! println!("Connected: {:?}", conn.peer_addr());
//! # Ok(())
//! # }
//! ```
//!
//! # Client Example
//!
//! ```no_run
//! use nearclip_net::tcp::{TcpClient, TcpClientConfig};
//! use nearclip_crypto::{TlsCertificate, TlsClientConfig};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // 获取服务端证书（通常通过配对流程获取）
//! # let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//! let tls_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
//!
//! // 连接到服务端
//! let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
//! let config = TcpClientConfig::new(addr);
//! let conn = TcpClient::connect(config, tls_config.config(), "localhost").await?;
//!
//! println!("Connected to: {:?}", conn.peer_addr());
//! # Ok(())
//! # }
//! ```

mod client;
mod connection;
mod server;

pub use client::{TcpClient, TcpClientConfig};
pub use connection::TcpConnection;
pub use server::{TcpServer, TcpServerConfig};
