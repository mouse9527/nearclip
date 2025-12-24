//! Unified Transport Layer for NearClip
//!
//! This crate provides a unified abstraction over different transport mechanisms
//! (WiFi/TCP and BLE), allowing upper layers to send messages without caring
//! about the underlying transport.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │ Application Layer (NearClipManager)     │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │ TransportManager                        │
//! │ - Manages connections per device        │
//! │ - Auto-selects best channel             │
//! │ - Handles failover                      │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │ Transport Trait                         │
//! └───────┬─────────────┬───────────────────┘
//!         │             │
//! ┌───────▼───────┐ ┌───▼───────┐ ┌─────────────┐
//! │ WifiTransport │ │BleTransport│ │MockTransport│
//! └───────────────┘ └───────────┘ └─────────────┘
//! ```
//!
//! # Example
//!
//! ```ignore
//! use nearclip_transport::{TransportManager, Transport};
//!
//! // Send to a device (auto-selects best channel)
//! let msg = Message::clipboard_sync(content, device_id);
//! manager.send_to_device("device_123", &msg).await?;
//!
//! // Broadcast to all connected devices
//! manager.broadcast(&msg).await;
//! ```

mod error;
mod traits;
mod wifi;
mod ble;
mod mock;
mod manager;

pub use error::TransportError;
pub use traits::{Transport, TransportConnector, TransportListener, TransportCallback};
pub use wifi::{WifiTransport, WifiTransportConnector, WifiTransportListener};
pub use ble::{BleTransport, BleSender};
pub use mock::{MockTransport, MockConfig, create_mock_pair};
pub use manager::TransportManager;

// Re-export Channel from nearclip-sync for convenience
pub use nearclip_sync::Channel;
