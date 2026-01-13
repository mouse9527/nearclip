//! Device pairing protocol implementation
//!
//! Handles the bidirectional pairing process between devices.

use crate::{DeviceManager, PairedDevice, DeviceError};
use nearclip_crypto::EcdhKeyPair;
use nearclip_protocol::{
    PairingMessage, PairingRequest, PairingResponse, PairingConfirm,
    PairingRejected,
};
use nearclip_protocol::pairing::DevicePlatform;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::info;

/// Transport abstraction for sending/receiving pairing messages
pub trait Transport: Send + Sync {
    /// Send a message to a device
    fn send(&self, device_id: &str, data: Vec<u8>) -> Result<(), String>;

    /// Receive a message with timeout
    fn recv_timeout(&self, timeout_ms: u64) -> Result<Vec<u8>, String>;
}

/// Pairing state for tracking active pairing sessions
#[derive(Debug, Clone)]
pub enum PairingState {
    /// No active pairing
    Idle,

    /// Waiting for response after sending request
    WaitingResponse {
        device_id: String,
        nonce: [u8; 32],
        started_at: u64,
    },

    /// Waiting for confirm after sending response
    WaitingConfirm {
        device_id: String,
        nonce: [u8; 32],
        started_at: u64,
    },

    /// Pairing completed successfully
    Completed {
        device_id: String,
        completed_at: u64,
    },

    /// Pairing failed
    Failed {
        reason: String,
        failed_at: u64,
    },
}

impl PairingState {
    /// Check if state is idle
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Check if state is waiting (for response or confirm)
    pub fn is_waiting(&self) -> bool {
        matches!(self, Self::WaitingResponse { .. } | Self::WaitingConfirm { .. })
    }

    /// Check if pairing is complete
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    /// Get the associated device ID if any
    pub fn device_id(&self) -> Option<&str> {
        match self {
            Self::Idle => None,
            Self::WaitingResponse { device_id, .. } => Some(device_id),
            Self::WaitingConfirm { device_id, .. } => Some(device_id),
            Self::Completed { device_id, .. } => Some(device_id),
            Self::Failed { .. } => None,
        }
    }
}

/// Pairing errors
#[derive(Debug, Clone)]
pub enum PairingError {
    /// Invalid data in pairing message
    InvalidData(String),

    /// Device is already paired
    AlreadyPaired,

    /// Invalid signature
    InvalidSignature,

    /// Timeout waiting for response
    Timeout,

    /// Pairing was rejected by remote device
    Rejected(String),

    /// Serialization/deserialization error
    ProtocolError(String),

    /// Transport error
    TransportError(String),

    /// Internal device error
    DeviceError(String),
}

impl std::fmt::Display for PairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairingError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            PairingError::AlreadyPaired => write!(f, "Device is already paired"),
            PairingError::InvalidSignature => write!(f, "Invalid signature"),
            PairingError::Timeout => write!(f, "Pairing timeout"),
            PairingError::Rejected(reason) => write!(f, "Pairing rejected: {}", reason),
            PairingError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            PairingError::TransportError(msg) => write!(f, "Transport error: {}", msg),
            PairingError::DeviceError(msg) => write!(f, "Device error: {}", msg),
        }
    }
}

impl std::error::Error for PairingError {}

impl From<DeviceError> for PairingError {
    fn from(err: DeviceError) -> Self {
        PairingError::DeviceError(err.to_string())
    }
}

/// Pairing manager handles the device pairing protocol
pub struct PairingManager {
    device_manager: Arc<DeviceManager>,
    state: Arc<RwLock<PairingState>>,
    transport: Arc<dyn Transport>,
    local_device_id: String,
    local_device_name: String,
    local_platform: DevicePlatform,
    local_keypair: EcdhKeyPair,
    pairing_timeout_ms: u64,
}

impl PairingManager {
    /// Create a new pairing manager
    pub fn new(
        device_manager: Arc<DeviceManager>,
        transport: Arc<dyn Transport>,
        local_device_id: String,
        local_device_name: String,
        local_platform: DevicePlatform,
        local_keypair: EcdhKeyPair,
    ) -> Self {
        Self {
            device_manager,
            state: Arc::new(RwLock::new(PairingState::Idle)),
            transport,
            local_device_id,
            local_device_name,
            local_platform,
            local_keypair,
            pairing_timeout_ms: 30_000, // 30 seconds default
        }
    }

    /// Set pairing timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.pairing_timeout_ms = timeout_ms;
        self
    }

    /// Get current pairing state
    pub async fn get_state(&self) -> PairingState {
        self.state.read().await.clone()
    }

    /// Initiate pairing as the requester
    pub async fn initiate_pairing(
        &self,
        target_device_id: &str,
    ) -> Result<PairedDevice, PairingError> {
        info!(target_device_id, "Initiating pairing");

        // Check if already paired
        if self.device_manager.is_paired(target_device_id).await {
            return Err(PairingError::AlreadyPaired);
        }

        // Generate nonce
        let nonce = self.generate_nonce();

        // Create pairing request
        let request = PairingMessage::PairingRequest(PairingRequest {
            device_id: self.local_device_id.clone(),
            device_name: self.local_device_name.clone(),
            platform: self.local_platform.clone(),
            public_key: self.local_keypair.public_key_bytes(),
            nonce,
        });

        // Serialize and send
        let request_data = rmp_serde::to_vec(&request)
            .map_err(|e| PairingError::ProtocolError(format!("Failed to serialize: {}", e)))?;

        self.transport.send(target_device_id, request_data)
            .map_err(|e| PairingError::TransportError(e))?;

        // Update state
        *self.state.write().await = PairingState::WaitingResponse {
            device_id: target_device_id.to_string(),
            nonce,
            started_at: now_millis(),
        };

        // Wait for response
        let response_data = self.transport.recv_timeout(self.pairing_timeout_ms)
            .map_err(|_| PairingError::Timeout)?;

        let response: PairingMessage = rmp_serde::from_slice(&response_data)
            .map_err(|e| PairingError::ProtocolError(format!("Failed to deserialize: {}", e)))?;

        match response {
            PairingMessage::PairingResponse(resp) => {
                // Validate response
                resp.validate().map_err(|e| PairingError::InvalidData(e.to_string()))?;

                info!(remote_device_id = %resp.device_id, "Received pairing response");

                // TODO: Verify signature (requires crypto integration)
                // TODO: Derive shared secret (requires crypto integration)

                // Send confirmation
                let confirm = PairingMessage::PairingConfirm(PairingConfirm {
                    signature: vec![], // TODO: actual signature
                });

                let confirm_data = rmp_serde::to_vec(&confirm)
                    .map_err(|e| PairingError::ProtocolError(format!("Failed to serialize: {}", e)))?;

                self.transport.send(target_device_id, confirm_data)
                    .map_err(|e| PairingError::TransportError(e))?;

                // Compute shared secret using ECDH
                let shared_secret = self.local_keypair
                    .compute_shared_secret(&resp.public_key)
                    .map_err(|e| PairingError::ProtocolError(format!("Failed to compute shared secret: {}", e)))?;

                // Create paired device
                let now = now_millis() as i64;
                let device = PairedDevice {
                    device_id: resp.device_id.clone(),
                    device_name: resp.device_name.clone(),
                    platform: to_device_platform(resp.platform),
                    public_key: resp.public_key.clone(),
                    shared_secret,
                    paired_at: now,
                    last_connected: Some(now),
                    last_seen: Some(now),
                };

                // Save to device manager
                self.device_manager.pair_device(device.clone()).await?;

                // Update state
                *self.state.write().await = PairingState::Completed {
                    device_id: resp.device_id.clone(),
                    completed_at: now_millis(),
                };

                info!(device_id = %resp.device_id, "Pairing completed");
                Ok(device)
            }
            PairingMessage::PairingRejected(rejected) => {
                *self.state.write().await = PairingState::Failed {
                    reason: rejected.reason.clone(),
                    failed_at: now_millis(),
                };
                Err(PairingError::Rejected(rejected.reason))
            }
            _ => {
                *self.state.write().await = PairingState::Failed {
                    reason: "Unexpected message type".to_string(),
                    failed_at: now_millis(),
                };
                Err(PairingError::ProtocolError("Unexpected message type".to_string()))
            }
        }
    }

    /// Handle an incoming pairing request
    pub async fn handle_incoming_request(
        &self,
        request: PairingRequest,
    ) -> Result<PairedDevice, PairingError> {
        info!(from_device_id = %request.device_id, "Handling incoming pairing request");

        // Validate request
        request.validate()
            .map_err(|e| PairingError::InvalidData(e.to_string()))?;

        // Check if already paired
        if self.device_manager.is_paired(&request.device_id).await {
            // Send rejection
            let rejected = PairingMessage::PairingRejected(PairingRejected::already_paired());
            let _ = self.send_message(&request.device_id, &rejected);
            return Err(PairingError::AlreadyPaired);
        }

        // Generate nonce for response
        let nonce = self.generate_nonce();

        // TODO: Verify signature (requires crypto integration)
        // TODO: Derive shared secret (requires crypto integration)

        // Create response
        let response = PairingMessage::PairingResponse(PairingResponse {
            device_id: self.local_device_id.clone(),
            device_name: self.local_device_name.clone(),
            platform: self.local_platform.clone(),
            public_key: self.local_keypair.public_key_bytes(),
            nonce,
            signature: vec![], // TODO: actual signature
        });

        // Send response
        self.send_message(&request.device_id, &response)?;

        // Update state
        *self.state.write().await = PairingState::WaitingConfirm {
            device_id: request.device_id.clone(),
            nonce,
            started_at: now_millis(),
        };

        // Wait for confirmation
        let confirm_data = self.transport.recv_timeout(self.pairing_timeout_ms)
            .map_err(|_| PairingError::Timeout)?;

        let confirm: PairingMessage = rmp_serde::from_slice(&confirm_data)
            .map_err(|e| PairingError::ProtocolError(format!("Failed to deserialize: {}", e)))?;

        match confirm {
            PairingMessage::PairingConfirm(_) => {
                info!("Received pairing confirmation");

                // TODO: Verify signature

                // Compute shared secret using ECDH
                let shared_secret = self.local_keypair
                    .compute_shared_secret(&request.public_key)
                    .map_err(|e| PairingError::ProtocolError(format!("Failed to compute shared secret: {}", e)))?;

                // Create paired device
                let now = now_millis() as i64;
                let device = PairedDevice {
                    device_id: request.device_id.clone(),
                    device_name: request.device_name.clone(),
                    platform: to_device_platform(request.platform),
                    public_key: request.public_key.clone(),
                    shared_secret,
                    paired_at: now,
                    last_connected: Some(now),
                    last_seen: Some(now),
                };

                // Save to device manager
                self.device_manager.pair_device(device.clone()).await?;

                // Send completion message
                let complete = PairingMessage::PairingComplete;
                let _ = self.send_message(&request.device_id, &complete);

                // Update state
                *self.state.write().await = PairingState::Completed {
                    device_id: request.device_id.clone(),
                    completed_at: now_millis(),
                };

                info!(device_id = %request.device_id, "Pairing completed (responder)");
                Ok(device)
            }
            PairingMessage::PairingRejected(rejected) => {
                *self.state.write().await = PairingState::Failed {
                    reason: rejected.reason.clone(),
                    failed_at: now_millis(),
                };
                Err(PairingError::Rejected(rejected.reason))
            }
            _ => {
                *self.state.write().await = PairingState::Failed {
                    reason: "Unexpected message type".to_string(),
                    failed_at: now_millis(),
                };
                Err(PairingError::ProtocolError("Unexpected message type".to_string()))
            }
        }
    }

    /// Reject a pairing request
    pub async fn reject_pairing(&self, device_id: &str, reason: String) {
        let rejected = PairingMessage::PairingRejected(PairingRejected::new(reason));
        let _ = self.send_message(device_id, &rejected);
        *self.state.write().await = PairingState::Idle;
    }

    /// Reset pairing state to idle
    pub async fn reset(&self) {
        *self.state.write().await = PairingState::Idle;
    }

    /// Send a pairing message
    fn send_message(&self, device_id: &str, message: &PairingMessage) -> Result<(), PairingError> {
        let data = rmp_serde::to_vec(message)
            .map_err(|e| PairingError::ProtocolError(format!("Failed to serialize: {}", e)))?;

        self.transport.send(device_id, data)
            .map_err(|e| PairingError::TransportError(e))
    }

    /// Generate a random nonce
    fn generate_nonce(&self) -> [u8; 32] {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

/// Convert protocol platform to device platform
fn to_device_platform(platform: DevicePlatform) -> crate::DevicePlatform {
    match platform {
        DevicePlatform::MacOS => crate::DevicePlatform::MacOS,
        DevicePlatform::Windows => crate::DevicePlatform::Windows,
        DevicePlatform::Linux => crate::DevicePlatform::Linux,
        DevicePlatform::Android => crate::DevicePlatform::Android,
        DevicePlatform::Ios => crate::DevicePlatform::Ios,
    }
}

/// Get current time in milliseconds since Unix epoch
fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock transport for testing
    struct MockTransport {
        sent_messages: Arc<RwLock<Vec<(String, Vec<u8>)>>>,
    }

    impl MockTransport {
        fn new() -> Self {
            Self {
                sent_messages: Arc::new(RwLock::new(Vec::new())),
            }
        }
    }

    impl Transport for MockTransport {
        fn send(&self, device_id: &str, data: Vec<u8>) -> Result<(), String> {
            // For testing, just return success
            Ok(())
        }

        fn recv_timeout(&self, _timeout_ms: u64) -> Result<Vec<u8>, String> {
            // For testing, just return timeout
            Err("Mock transport timeout".to_string())
        }
    }

    #[test]
    fn test_pairing_state_idle() {
        let state = PairingState::Idle;
        assert!(state.is_idle());
        assert!(!state.is_waiting());
        assert!(!state.is_completed());
        assert!(state.device_id().is_none());
    }

    #[test]
    fn test_pairing_state_waiting_response() {
        let state = PairingState::WaitingResponse {
            device_id: "test-device".to_string(),
            nonce: [0u8; 32],
            started_at: 1000,
        };
        assert!(!state.is_idle());
        assert!(state.is_waiting());
        assert!(!state.is_completed());
        assert_eq!(state.device_id(), Some("test-device"));
    }

    #[test]
    fn test_pairing_state_completed() {
        let state = PairingState::Completed {
            device_id: "test-device".to_string(),
            completed_at: 2000,
        };
        assert!(!state.is_idle());
        assert!(!state.is_waiting());
        assert!(state.is_completed());
        assert_eq!(state.device_id(), Some("test-device"));
    }

    #[test]
    fn test_pairing_state_failed() {
        let state = PairingState::Failed {
            reason: "Test error".to_string(),
            failed_at: 3000,
        };
        // Failed state is NOT considered idle (only Idle state is idle)
        assert!(!state.is_idle());
        assert!(!state.is_waiting());
        assert!(!state.is_completed());
        assert!(state.device_id().is_none());
    }

    #[test]
    fn test_pairing_error_display() {
        assert_eq!(
            format!("{}", PairingError::AlreadyPaired),
            "Device is already paired"
        );
        assert_eq!(
            format!("{}", PairingError::Timeout),
            "Pairing timeout"
        );
        assert_eq!(
            format!("{}", PairingError::Rejected("test".to_string())),
            "Pairing rejected: test"
        );
    }

    #[test]
    fn test_generate_nonce() {
        let manager = create_test_manager();
        let nonce1 = manager.generate_nonce();
        let nonce2 = manager.generate_nonce();

        // Nonces should be different (very high probability)
        assert_ne!(nonce1.to_vec(), nonce2.to_vec());

        // Nonce should be 32 bytes
        assert_eq!(nonce1.len(), 32);
    }

    #[tokio::test]
    async fn test_pairing_manager_get_state() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let device_manager = Arc::new(DeviceManager::new(db_path).await.unwrap());
        let transport = Arc::new(MockTransport::new());

        let manager = PairingManager::new(
            device_manager,
            transport,
            "local-device".to_string(),
            "Local Device".to_string(),
            DevicePlatform::MacOS,
            vec![1, 2, 3, 4],
        );

        let state = manager.get_state().await;
        assert!(state.is_idle());
    }

    #[tokio::test]
    async fn test_pairing_manager_reset() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let device_manager = Arc::new(DeviceManager::new(db_path).await.unwrap());
        let transport = Arc::new(MockTransport::new());

        let manager = PairingManager::new(
            device_manager,
            transport,
            "local-device".to_string(),
            "Local Device".to_string(),
            DevicePlatform::MacOS,
            vec![1, 2, 3, 4],
        );

        // Set to a non-idle state
        {
            let mut state = manager.state.write().await;
            *state = PairingState::WaitingResponse {
                device_id: "test".to_string(),
                nonce: [0u8; 32],
                started_at: 1000,
            };
        }

        manager.reset().await;
        let state = manager.get_state().await;
        assert!(state.is_idle());
    }

    fn create_test_manager() -> PairingManager {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        // Create a device manager synchronously using a runtime
        let device_manager = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                DeviceManager::new(db_path).await
            })
            .unwrap();

        let transport = Arc::new(MockTransport::new());

        PairingManager::new(
            Arc::new(device_manager),
            transport,
            "local-device".to_string(),
            "Local Device".to_string(),
            DevicePlatform::MacOS,
            vec![1, 2, 3, 4],
        )
    }
}
