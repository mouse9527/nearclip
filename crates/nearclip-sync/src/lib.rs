//! NearClip Sync Module
//!
//! Synchronization protocol implementation including message format,
//! channel selection strategy, and retry logic.
//!
//! # Message Protocol
//!
//! All network communication uses the [`Message`] struct with [`MessageType`]
//! to ensure consistent formatting across WiFi and BLE channels.
//!
//! ```
//! use nearclip_sync::{Message, MessageType};
//!
//! // Create a clipboard sync message
//! let msg = Message::clipboard_sync(b"Hello!", "device-123".to_string());
//!
//! // Serialize to MessagePack bytes
//! let bytes = msg.serialize().unwrap();
//!
//! // Deserialize back
//! let decoded = Message::deserialize(&bytes).unwrap();
//! assert_eq!(decoded.msg_type, MessageType::ClipboardSync);
//! ```
//!
//! # Channel Selection
//!
//! The [`channel`] module provides abstractions for selecting communication channels.
//! WiFi is preferred over BLE for its higher speed and lower latency.
//!
//! ```
//! use nearclip_sync::{Channel, ChannelStatus, ChannelInfo, PriorityChannelSelector, ChannelSelector};
//!
//! let channels = vec![
//!     ChannelInfo::new(Channel::Wifi, ChannelStatus::Available),
//!     ChannelInfo::new(Channel::Ble, ChannelStatus::Available),
//! ];
//!
//! let selector = PriorityChannelSelector;
//! let selected = selector.select(&channels);
//! assert_eq!(selected, Some(Channel::Wifi)); // WiFi preferred
//! ```
//!
//! # Clipboard Sending
//!
//! The [`sender`] module implements clipboard content sending with automatic
//! channel selection and ACK handling.
//!
//! ```no_run
//! use nearclip_sync::{ClipboardSender, ClipboardSenderConfig, ClipboardSendCallback, SyncError};
//! use std::sync::Arc;
//!
//! struct MyCallback;
//! impl ClipboardSendCallback for MyCallback {
//!     fn on_send_success(&self, device_id: &str) {}
//!     fn on_send_failure(&self, device_id: &str, error: SyncError) {}
//!     fn on_ack_received(&self, device_id: &str) {}
//! }
//!
//! # async fn example() -> Result<(), SyncError> {
//! let config = ClipboardSenderConfig::new()
//!     .with_device_id("my-device");
//! let callback = Arc::new(MyCallback);
//! let sender = ClipboardSender::new(config, callback)?;
//! # Ok(())
//! # }
//! ```

pub mod channel;
pub mod protocol;
pub mod receiver;
pub mod sender;

// Re-export protocol types
pub use protocol::{Message, MessageType, ProtocolError};

// Re-export channel types
pub use channel::{
    BleOnlyChannelSelector, Channel, ChannelInfo, ChannelSelector, ChannelStatus,
    PriorityChannelSelector, WifiOnlyChannelSelector,
};

// Re-export sender types
pub use sender::{
    ClipboardSendCallback, ClipboardSender, ClipboardSenderConfig, SendStatus, SyncError,
    DEFAULT_ACK_TIMEOUT_SECS, DEFAULT_RETRY_COUNT,
};

// Re-export receiver types
pub use receiver::{
    ClipboardReceiveCallback, ClipboardReceiver, ClipboardReceiverConfig, ReceiveResult,
    DEFAULT_MAX_MESSAGE_SIZE, DEFAULT_MESSAGE_TIMEOUT_SECS,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_reexport() {
        // Verify Message can be used from crate root
        let msg = Message::heartbeat("test-device".to_string());
        assert_eq!(msg.msg_type, MessageType::Heartbeat);
    }

    #[test]
    fn test_message_type_reexport() {
        // Verify MessageType can be used from crate root
        let msg_type = MessageType::ClipboardSync;
        assert!(msg_type.requires_ack());
    }
}
