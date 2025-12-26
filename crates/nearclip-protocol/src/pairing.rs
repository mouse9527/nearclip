//! Pairing protocol messages
//!
//! Defines the message types used during the device pairing process.
//!
//! # Pairing Flow
//!
//! ```text
//! Initiator                  Responder
//!    |                           |
//!    |--- PairingRequest --------->|
//!    |                           |
//!    |<------ PairingResponse -----|
//!    |                           |
//!    |------ PairingConfirm ------>|
//!    |                           |
//!    |<------- PairingComplete ----|
//! ```
//!
//! If pairing is rejected at any point, a `PairingRejected` message is sent.

use serde::{Deserialize, Serialize};

/// All pairing protocol messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PairingMessage {
    /// Initiator sends pairing request
    PairingRequest(PairingRequest),

    /// Responder accepts and sends their info
    PairingResponse(PairingResponse),

    /// Initiator confirms the pairing
    PairingConfirm(PairingConfirm),

    /// Pairing completed successfully
    PairingComplete,

    /// Pairing was rejected
    PairingRejected(PairingRejected),
}

/// Initial pairing request from initiator
///
/// Sent when a user scans a QR code and initiates pairing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairingRequest {
    /// Unique device identifier
    pub device_id: String,

    /// Human-readable device name
    pub device_name: String,

    /// Platform/OS of the device
    pub platform: DevicePlatform,

    /// Public key for ECDH key exchange
    pub public_key: Vec<u8>,

    /// Random nonce for freshness verification
    pub nonce: [u8; 32],
}

/// Response to pairing request from responder
///
/// Sent when the user accepts the pairing request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairingResponse {
    /// Responder's device ID
    pub device_id: String,

    /// Responder's device name
    pub device_name: String,

    /// Responder's platform
    pub platform: DevicePlatform,

    /// Responder's public key
    pub public_key: Vec<u8>,

    /// Random nonce for freshness verification
    pub nonce: [u8; 32],

    /// Signature of initiator's nonce (proves identity)
    pub signature: Vec<u8>,
}

/// Final confirmation from initiator
///
/// Sent after receiving PairingResponse to complete the handshake.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairingConfirm {
    /// Signature of responder's nonce
    pub signature: Vec<u8>,
}

/// Sent when pairing is rejected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairingRejected {
    /// Human-readable reason for rejection
    pub reason: String,
}

/// Device platform/OS type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DevicePlatform {
    MacOS,
    Windows,
    Linux,
    Android,
    Ios,
}

impl PairingRequest {
    /// Create a new pairing request
    pub fn new(
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
    ) -> Self {
        Self {
            device_id,
            device_name,
            platform,
            public_key,
            nonce,
        }
    }

    /// Validate the request
    pub fn validate(&self) -> Result<(), PairingError> {
        if self.device_id.is_empty() {
            return Err(PairingError::InvalidData("device_id is empty".to_string()));
        }
        if self.device_name.is_empty() {
            return Err(PairingError::InvalidData("device_name is empty".to_string()));
        }
        if self.public_key.is_empty() {
            return Err(PairingError::InvalidData("public_key is empty".to_string()));
        }
        Ok(())
    }
}

impl PairingResponse {
    /// Create a new pairing response
    pub fn new(
        device_id: String,
        device_name: String,
        platform: DevicePlatform,
        public_key: Vec<u8>,
        nonce: [u8; 32],
        signature: Vec<u8>,
    ) -> Self {
        Self {
            device_id,
            device_name,
            platform,
            public_key,
            nonce,
            signature,
        }
    }

    /// Validate the response
    pub fn validate(&self) -> Result<(), PairingError> {
        if self.device_id.is_empty() {
            return Err(PairingError::InvalidData("device_id is empty".to_string()));
        }
        if self.device_name.is_empty() {
            return Err(PairingError::InvalidData("device_name is empty".to_string()));
        }
        if self.public_key.is_empty() {
            return Err(PairingError::InvalidData("public_key is empty".to_string()));
        }
        if self.signature.is_empty() {
            return Err(PairingError::InvalidData("signature is empty".to_string()));
        }
        Ok(())
    }
}

impl PairingRejected {
    /// Create a new rejection message
    pub fn new(reason: String) -> Self {
        Self { reason }
    }

    /// Common rejection reasons
    pub fn user_declined() -> Self {
        Self::new("User declined the pairing request".to_string())
    }

    pub fn timeout() -> Self {
        Self::new("Pairing timed out".to_string())
    }

    pub fn already_paired() -> Self {
        Self::new("Device is already paired".to_string())
    }

    pub fn incompatible_version() -> Self {
        Self::new("Incompatible protocol version".to_string())
    }
}

/// Pairing protocol errors
#[derive(Debug, Clone, PartialEq)]
pub enum PairingError {
    InvalidData(String),
    AlreadyPaired,
    InvalidSignature,
    InvalidNonce,
    Timeout,
    Rejected(String),
    SerializationError,
    DeserializationError,
}

impl std::fmt::Display for PairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairingError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            PairingError::AlreadyPaired => write!(f, "Device is already paired"),
            PairingError::InvalidSignature => write!(f, "Invalid signature"),
            PairingError::InvalidNonce => write!(f, "Invalid nonce"),
            PairingError::Timeout => write!(f, "Pairing timeout"),
            PairingError::Rejected(reason) => write!(f, "Pairing rejected: {}", reason),
            PairingError::SerializationError => write!(f, "Failed to serialize message"),
            PairingError::DeserializationError => write!(f, "Failed to deserialize message"),
        }
    }
}

impl std::error::Error for PairingError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_nonce() -> [u8; 32] {
        [42u8; 32]
    }

    fn create_test_request() -> PairingRequest {
        PairingRequest {
            device_id: "test-device".to_string(),
            device_name: "Test Device".to_string(),
            platform: DevicePlatform::MacOS,
            public_key: vec![1, 2, 3, 4],
            nonce: create_test_nonce(),
        }
    }

    #[test]
    fn test_pairing_request_validation() {
        let request = create_test_request();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_pairing_request_empty_device_id() {
        let mut request = create_test_request();
        request.device_id = String::new();
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_pairing_request_empty_device_name() {
        let mut request = create_test_request();
        request.device_name = String::new();
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_pairing_request_empty_public_key() {
        let mut request = create_test_request();
        request.public_key = Vec::new();
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_pairing_response_validation() {
        let response = PairingResponse {
            device_id: "test-device".to_string(),
            device_name: "Test Device".to_string(),
            platform: DevicePlatform::Android,
            public_key: vec![5, 6, 7, 8],
            nonce: create_test_nonce(),
            signature: vec![9, 10, 11, 12],
        };
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_pairing_response_empty_signature() {
        let mut response = PairingResponse {
            device_id: "test-device".to_string(),
            device_name: "Test Device".to_string(),
            platform: DevicePlatform::Android,
            public_key: vec![5, 6, 7, 8],
            nonce: create_test_nonce(),
            signature: vec![9, 10, 11, 12],
        };
        response.signature = Vec::new();
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_pairing_rejected_common_reasons() {
        let _ = PairingRejected::user_declined();
        let _ = PairingRejected::timeout();
        let _ = PairingRejected::already_paired();
        let _ = PairingRejected::incompatible_version();
    }

    #[test]
    fn test_pairing_message_serialization() {
        let request = create_test_request();
        let message = PairingMessage::PairingRequest(request);

        // Test JSON serialization
        let json = serde_json::to_string(&message).unwrap();
        let _deserialized: PairingMessage = serde_json::from_str(&json).unwrap();

        // Test MessagePack serialization
        let msgpack = rmp_serde::to_vec(&message).unwrap();
        let _deserialized: PairingMessage = rmp_serde::from_slice(&msgpack).unwrap();
    }

    #[test]
    fn test_device_platform_equality() {
        assert_eq!(DevicePlatform::MacOS, DevicePlatform::MacOS);
        assert_ne!(DevicePlatform::MacOS, DevicePlatform::Android);
    }

    #[test]
    fn test_pairing_error_display() {
        assert_eq!(
            format!("{}", PairingError::AlreadyPaired),
            "Device is already paired"
        );
        assert_eq!(
            format!("{}", PairingError::Rejected("test".to_string())),
            "Pairing rejected: test"
        );
    }

    #[test]
    fn test_complete_pairing_flow_messages() {
        let nonce = create_test_nonce();

        let request = PairingMessage::PairingRequest(PairingRequest {
            device_id: "initiator".to_string(),
            device_name: "Initiator".to_string(),
            platform: DevicePlatform::MacOS,
            public_key: vec![1, 2, 3, 4],
            nonce,
        });

        let response = PairingMessage::PairingResponse(PairingResponse {
            device_id: "responder".to_string(),
            device_name: "Responder".to_string(),
            platform: DevicePlatform::Android,
            public_key: vec![5, 6, 7, 8],
            nonce,
            signature: vec![9, 10],
        });

        let confirm = PairingMessage::PairingConfirm(PairingConfirm {
            signature: vec![11, 12],
        });

        let complete = PairingMessage::PairingComplete;

        // Verify all can be serialized/deserialized
        for msg in [request, response, confirm, complete] {
            let serialized = rmp_serde::to_vec(&msg).unwrap();
            let deserialized: PairingMessage = rmp_serde::from_slice(&serialized).unwrap();
            assert_eq!(msg, deserialized);
        }
    }
}
