//! Encrypted transport wrapper
//!
//! Provides end-to-end encryption for all transport channels using AES-256-GCM.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │   Upper Layer (NearClipManager)         │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │   EncryptedTransport                    │
//! │   - Encrypts before send                │
//! │   - Decrypts after receive              │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │   Underlying Transport (WiFi/BLE)       │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```ignore
//! use nearclip_transport::{EncryptedTransport, Transport};
//! use nearclip_crypto::Aes256Gcm;
//!
//! // Wrap an existing transport with encryption
//! let shared_secret = device.shared_secret;
//! let encrypted = EncryptedTransport::new(transport, shared_secret)?;
//!
//! // Messages are automatically encrypted/decrypted
//! encrypted.send(&message).await?;
//! let received = encrypted.recv().await?;
//! ```

use async_trait::async_trait;
use nearclip_crypto::{Aes256Gcm, CipherError};
use nearclip_sync::{Channel, Message};
use std::sync::Arc;
use tracing::{debug, instrument};

use crate::error::TransportError;
use crate::traits::Transport;

/// Encrypted transport wrapper
///
/// Wraps any transport and adds AES-256-GCM encryption to all messages.
/// The encryption is transparent to upper layers - messages are encrypted
/// before sending and decrypted after receiving.
pub struct EncryptedTransport {
    /// Underlying transport (WiFi, BLE, etc.)
    inner: Arc<dyn Transport>,

    /// AES-256-GCM cipher for encryption/decryption
    cipher: Aes256Gcm,
}

impl EncryptedTransport {
    /// Create a new encrypted transport wrapper
    ///
    /// # Arguments
    ///
    /// * `inner` - The underlying transport to wrap
    /// * `shared_secret` - 32-byte shared secret (from ECDH key exchange)
    ///
    /// # Returns
    ///
    /// A new encrypted transport, or an error if the shared secret is invalid
    ///
    /// # Example
    ///
    /// ```ignore
    /// use nearclip_transport::{EncryptedTransport, Transport};
    /// use nearclip_crypto::Aes256Gcm;
    ///
    /// let shared_secret = [0u8; 32];  // From ECDH
    /// let encrypted = EncryptedTransport::new(transport, &shared_secret)?;
    /// ```
    pub fn new(inner: Arc<dyn Transport>, shared_secret: &[u8]) -> Result<Self, TransportError> {
        let cipher = Aes256Gcm::new(shared_secret)
            .map_err(|e| TransportError::Other(format!("Failed to create cipher: {}", e)))?;

        debug!(
            "Created encrypted transport for {} channel",
            inner.channel()
        );

        Ok(Self { inner, cipher })
    }

    /// Get a reference to the inner transport
    pub fn inner(&self) -> &Arc<dyn Transport> {
        &self.inner
    }

    /// Encrypt a message
    ///
    /// Serializes the message and encrypts the bytes.
    #[instrument(skip(self, msg), fields(msg_type = ?msg.msg_type, device_id = %msg.device_id))]
    fn encrypt_message(&self, msg: &Message) -> Result<Vec<u8>, TransportError> {
        // Serialize the message
        let serialized = msg.serialize()
            .map_err(|e| TransportError::Serialization(e.to_string()))?;

        // Encrypt the serialized bytes
        self.cipher.encrypt(&serialized)
            .map_err(|e| TransportError::Other(format!("Encryption failed: {}", e)))
    }

    /// Decrypt a message
    ///
    /// Decrypts the bytes and deserializes the message.
    #[instrument(skip(self, data), fields(data_len = data.len()))]
    fn decrypt_message(&self, data: &[u8]) -> Result<Message, TransportError> {
        // Decrypt the bytes
        let decrypted = self.cipher.decrypt(data)
            .map_err(|e| TransportError::Other(format!("Decryption failed: {}", e)))?;

        // Deserialize the message
        Message::deserialize(&decrypted)
            .map_err(|e| TransportError::Deserialization(e.to_string()))
    }
}

#[async_trait]
impl Transport for EncryptedTransport {
    /// Send an encrypted message
    ///
    /// The message is serialized, encrypted with AES-256-GCM, and sent
    /// through the underlying transport.
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        debug!(
            "Sending encrypted message: type={:?}, device={}",
            msg.msg_type, msg.device_id
        );

        // Encrypt the message
        let encrypted = self.encrypt_message(msg)?;

        // Send the encrypted bytes
        // We need to wrap the encrypted bytes in a Message for the underlying transport
        // The payload is the encrypted data, msg_type is Heartbeat (placeholder)
        let wrapper_msg = Message::new(
            nearclip_sync::MessageType::Heartbeat, // Placeholder, actual content is encrypted
            encrypted,
            msg.device_id.clone(),
        );

        self.inner.send(&wrapper_msg).await
    }

    /// Receive and decrypt a message
    ///
    /// Receives encrypted bytes from the underlying transport, decrypts them,
    /// and returns the original message.
    async fn recv(&self) -> Result<Message, TransportError> {
        debug!("Receiving encrypted message");

        // Receive the wrapper message
        let wrapper_msg = self.inner.recv().await?;

        // Extract and decrypt the payload
        let decrypted = self.decrypt_message(&wrapper_msg.payload)?;

        debug!(
            "Decrypted message: type={:?}, device={}",
            decrypted.msg_type, decrypted.device_id
        );

        Ok(decrypted)
    }

    /// Check if the transport is connected
    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    /// Get the transport channel type
    fn channel(&self) -> Channel {
        self.inner.channel()
    }

    /// Get the peer device ID
    fn peer_device_id(&self) -> &str {
        self.inner.peer_device_id()
    }

    /// Close the transport connection
    async fn close(&self) -> Result<(), TransportError> {
        self.inner.close().await
    }
}

/// Convert cipher errors to transport errors
impl From<CipherError> for TransportError {
    fn from(err: CipherError) -> Self {
        TransportError::Other(format!("Cipher error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{MockTransport, MockConfig};

    fn create_test_encrypted_transport() -> EncryptedTransport {
        let config = MockConfig::new()
            .with_channel(Channel::Wifi);

        let inner = Arc::new(MockTransport::new("peer-device", config));
        let shared_secret = [0u8; 32];

        EncryptedTransport::new(inner, &shared_secret).unwrap()
    }

    #[tokio::test]
    async fn test_encrypted_transport_creation() {
        let encrypted = create_test_encrypted_transport();
        assert_eq!(encrypted.channel(), Channel::Wifi);
        assert_eq!(encrypted.peer_device_id(), "peer-device");
    }

    #[tokio::test]
    async fn test_encrypted_transport_send_recv() {
        // Create a mock pair for bidirectional communication
        let (transport_a, transport_b) = crate::mock::create_mock_pair(
            "device-a",
            "device-b"
        );

        // Wrap one side with encryption
        let encrypted_a = EncryptedTransport::new(transport_a, &[0u8; 32]).unwrap();

        // Create a message to send
        let msg = Message::clipboard_sync(b"Hello, encrypted world!", "sender".to_string());

        // Send through encrypted transport
        encrypted_a.send(&msg).await.unwrap();

        // Receive from the other side (gets encrypted wrapper)
        let wrapper = transport_b.recv().await.unwrap();
        assert!(!wrapper.payload.is_empty());

        // Manually decrypt to verify
        let cipher = Aes256Gcm::new(&[0u8; 32]).unwrap();
        let decrypted = cipher.decrypt(&wrapper.payload).unwrap();
        let restored = Message::deserialize(&decrypted).unwrap();

        assert_eq!(restored.msg_type, msg.msg_type);
        assert_eq!(restored.payload, msg.payload);
        assert_eq!(restored.device_id, msg.device_id);
    }

    #[tokio::test]
    async fn test_encrypted_transport_is_connected() {
        let encrypted = create_test_encrypted_transport();
        assert!(encrypted.is_connected());
    }

    #[tokio::test]
    async fn test_encrypted_transport_channel() {
        let encrypted = create_test_encrypted_transport();
        assert_eq!(encrypted.channel(), Channel::Wifi);
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let encrypted = create_test_encrypted_transport();

        let original = Message::clipboard_sync(b"test content", "device-1".to_string());

        // Encrypt
        let encrypted_data = encrypted.encrypt_message(&original).unwrap();

        // Decrypt
        let decrypted = encrypted.decrypt_message(&encrypted_data).unwrap();

        assert_eq!(decrypted.msg_type, original.msg_type);
        assert_eq!(decrypted.payload, original.payload);
        assert_eq!(decrypted.device_id, original.device_id);
    }

    #[tokio::test]
    async fn test_invalid_shared_secret() {
        let config = MockConfig::new()
            .with_channel(Channel::Wifi);

        let inner = Arc::new(MockTransport::new("peer-device", config));

        // Invalid shared secret length
        let result = EncryptedTransport::new(inner, &[0u8; 16]);
        assert!(matches!(result, Err(TransportError::Other(_))));
    }

    #[tokio::test]
    async fn test_encrypted_transport_close() {
        let encrypted = create_test_encrypted_transport();
        encrypted.close().await.unwrap();
        assert!(!encrypted.is_connected());
    }
}
