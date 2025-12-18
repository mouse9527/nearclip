//! NearClip Network Module
//!
//! Network layer for device discovery and TCP/TLS communication.
//! Includes mDNS service broadcasting, discovery, and TLS-encrypted TCP communication.
//!
//! # Modules
//!
//! - [`mdns`] - mDNS service discovery and advertising
//! - [`tcp`] - TLS-encrypted TCP server and connections
//! - [`error`] - Network error types
//!
//! # mDNS Example
//!
//! ```no_run
//! use nearclip_net::{MdnsAdvertiser, MdnsServiceConfig};
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // Create service configuration
//! let config = MdnsServiceConfig::new(
//!     "device-001".to_string(),
//!     "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
//!     12345,
//! );
//!
//! // Start advertising
//! let mut advertiser = MdnsAdvertiser::new(config)?;
//! advertiser.start().await?;
//!
//! // ... service is discoverable ...
//!
//! advertiser.stop().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # TCP Server Example
//!
//! ```no_run
//! use nearclip_net::{TcpServer, TcpServerConfig};
//! use nearclip_crypto::{TlsCertificate, TlsServerConfig};
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // Generate TLS certificate
//! let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//! let tls_config = TlsServerConfig::new(&cert).unwrap();
//!
//! // Create and bind server
//! let config = TcpServerConfig::new().with_port(0);
//! let server = TcpServer::bind(config, tls_config.config()).await?;
//!
//! println!("Server listening on {:?}", server.local_addr()?);
//!
//! // Accept connections
//! let conn = server.accept().await?;
//! println!("Connected: {:?}", conn.peer_addr());
//! # Ok(())
//! # }
//! ```
//!
//! # TCP Client Example
//!
//! ```no_run
//! use nearclip_net::{TcpClient, TcpClientConfig};
//! use nearclip_crypto::{TlsCertificate, TlsClientConfig};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), nearclip_net::NetError> {
//! // Get server certificate (usually obtained through pairing)
//! # let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//! let tls_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
//!
//! // Connect to server
//! let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
//! let config = TcpClientConfig::new(addr);
//! let conn = TcpClient::connect(config, tls_config.config(), "localhost").await?;
//!
//! println!("Connected to: {:?}", conn.peer_addr());
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod mdns;
pub mod tcp;

// Re-export main types
pub use error::NetError;
pub use mdns::{
    DiscoveredDevice, DiscoveryEvent, MdnsAdvertiser, MdnsDiscovery, MdnsServiceConfig,
    SERVICE_TYPE, TXT_DEVICE_ID, TXT_PUBKEY_HASH,
};
pub use tcp::{TcpClient, TcpClientConfig, TcpConnection, TcpReadHalf, TcpServer, TcpServerConfig, TcpWriteHalf};
