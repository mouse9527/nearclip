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

pub mod protocol;

// Re-export protocol types for convenience
pub use protocol::{Message, MessageType, ProtocolError};

// Future modules:
// mod sender;     // Clipboard content sending logic
// mod receiver;   // Clipboard content receiving logic
// mod channel;    // Channel selection and switching

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
