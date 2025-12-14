//! NearClip Network Module
//!
//! Network layer for device discovery and TCP/TLS communication.
//! Includes mDNS service broadcasting and discovery.
//!
//! # Modules
//!
//! - [`mdns`] - mDNS service discovery and advertising
//! - [`error`] - Network error types
//!
//! # Example
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

pub mod error;
pub mod mdns;

// Re-export main types
pub use error::NetError;
pub use mdns::{
    DiscoveredDevice, DiscoveryEvent, MdnsAdvertiser, MdnsDiscovery, MdnsServiceConfig,
    SERVICE_TYPE, TXT_DEVICE_ID, TXT_PUBKEY_HASH,
};

// Future modules:
// mod tcp;      // TCP server and client
// mod tls;      // TLS wrapper for secure connections
