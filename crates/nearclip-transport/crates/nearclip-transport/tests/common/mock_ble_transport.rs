//! Mock BLE transport with encryption support for testing
//!
//! This module provides a mock BLE transport that simulates:
//! - Chunking/reassembly (like real BLE with MTU constraints)
//! - Optional end-to-end encryption (AES-256-GCM)
//! - Message send/receive
//! - Connection state management

use async_trait::async_trait;
use nearclip_ble::{Chunker, DEFAULT_BLE_MTU, Reassembler};
use nearclip_crypto::Aes256Gcm;
use nearclip_sync::{Channel, Message};
use nearclip_transport::error::TransportError;
use nearclip_transport::traits::Transport;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use std::collections::VecDeque;

/// Mock BLE transport with optional encryption
///
/// This transport simulates BLE behavior including:
/// - Chunking messages to fit BLE MTU
/// - Reassembling chunks back into messages
/// - Optional AES-256-GCM encryption/decryption
///
/// Unlike the real BLE transport, this works entirely in-memory for testing.
pub struct MockBleTransport {
    device_id: String,
    /// Optional encryption cipher
    encryption: Option<Aes256Gcm>,
    /// Messages that have been sent (for inspection)
    sent_messages: Arc<Mutex<Vec<Message>>>,
    /// Raw chunks that have been sent (for low-level inspection)
    sent_chunks: Arc<Mutex<Vec<Vec<u8>>>>,
    /// Queue of received messages
    recv_queue: Arc<Mutex<VecDeque<Message>>>,
    /// Notifier for new received messages
    recv_notify: Arc<Notify>,
    /// Connection state
    connected: AtomicBool,
    /// MTU for chunking
    mtu: usize,
    /// Reassemblers for incoming chunks (message_id -> reassembler)
    reassemblers: Arc<Mutex<HashMap<u16, Reassembler>>>,
    /// Next message ID
    next_message_id: Arc<AtomicU16>,
}

impl MockBleTransport {
    /// Create a new mock BLE transport with encryption
    ///
    /// # Arguments
    /// * `device_id` - The peer device ID
    /// * `shared_secret` - 32-byte shared secret for AES-256-GCM
    pub fn new_with_encryption(device_id: impl Into<String>, shared_secret: &[u8]) -> Result<Self, TransportError> {
        let cipher = Aes256Gcm::new(shared_secret)
            .map_err(|e| TransportError::Other(format!("Failed to initialize encryption: {}", e)))?;

        Ok(Self {
            device_id: device_id.into(),
            encryption: Some(cipher),
            sent_messages: Arc::new(Mutex::new(Vec::new())),
            sent_chunks: Arc::new(Mutex::new(Vec::new())),
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            connected: AtomicBool::new(true),
            mtu: DEFAULT_BLE_MTU,
            reassemblers: Arc::new(Mutex::new(HashMap::new())),
            next_message_id: Arc::new(AtomicU16::new(1)),
        })
    }

    /// Create a new mock BLE transport without encryption
    pub fn new_without_encryption(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            encryption: None,
            sent_messages: Arc::new(Mutex::new(Vec::new())),
            sent_chunks: Arc::new(Mutex::new(Vec::new())),
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            connected: AtomicBool::new(true),
            mtu: DEFAULT_BLE_MTU,
            reassemblers: Arc::new(Mutex::new(HashMap::new())),
            next_message_id: Arc::new(AtomicU16::new(1)),
        }
    }

    /// Create with custom MTU
    pub fn with_mtu(mut self, mtu: usize) -> Self {
        self.mtu = mtu;
        self
    }

    /// Get all sent messages
    pub async fn get_sent_messages(&self) -> Vec<Message> {
        self.sent_messages.lock().await.clone()
    }

    /// Get all sent chunks (raw bytes)
    pub async fn get_sent_chunks(&self) -> Vec<Vec<u8>> {
        self.sent_chunks.lock().await.clone()
    }

    /// Clear sent messages and chunks
    pub async fn clear_sent(&self) {
        self.sent_messages.lock().await.clear();
        self.sent_chunks.lock().await.clear();
    }

    /// Inject a message to be received
    ///
    /// This simulates receiving a message from the peer, including:
    /// - Serialization
    /// - Encryption (if enabled)
    /// - Chunking
    /// - Reassembly
    pub async fn inject_message(&self, msg: &Message) -> Result<(), TransportError> {
        // 1. Serialize
        let serialized = bincode::serialize(msg)
            .map_err(|e| TransportError::Other(format!("Serialization failed: {}", e)))?;

        // 2. Encrypt (if enabled)
        let data = if let Some(ref cipher) = self.encryption {
            cipher.encrypt(&serialized)
                .map_err(|e| TransportError::Other(format!("Encryption failed: {}", e)))?
        } else {
            serialized
        };

        // 3. Chunk
        let message_id = self.next_message_id.fetch_add(1, Ordering::SeqCst);
        let mut chunker = Chunker::new(message_id, &data, self.mtu);
        let chunks = chunker.create_all_chunks();

        // 4. Process chunks (simulating receiving them)
        let mut reassemblers = self.reassemblers.lock().await;
        for chunk in chunks {
            self.process_chunk(&chunk, &mut reassemblers).await?;
        }

        Ok(())
    }

    /// Process a received chunk
    async fn process_chunk(
        &self,
        chunk: &[u8],
        reassemblers: &mut HashMap<u16, Reassembler>,
    ) -> Result<(), TransportError> {
        use nearclip_ble::{ChunkHeader, CHUNK_HEADER_SIZE};

        if chunk.len() < CHUNK_HEADER_SIZE {
            return Err(TransportError::Other(format!(
                "Chunk too short: {} bytes",
                chunk.len()
            )));
        }

        // Parse header
        let header = ChunkHeader::from_bytes(&chunk[..CHUNK_HEADER_SIZE])
            .map_err(|e| TransportError::Other(format!("Failed to parse chunk header: {}", e)))?;

        // Get or create reassembler
        let reassembler = reassemblers
            .entry(header.message_id)
            .or_insert_with(|| Reassembler::new(header.message_id));

        // Add chunk
        reassembler.add_chunk(chunk)
            .map_err(|e| TransportError::Other(format!("Failed to add chunk: {}", e)))?;

        // Check if complete
        if reassembler.is_complete() {
            let complete_data = reassembler.get_complete_data()
                .ok_or_else(|| TransportError::Other("Failed to get complete data".to_string()))?;

            // Decrypt (if encryption enabled)
            let decrypted = if let Some(ref cipher) = self.encryption {
                cipher.decrypt(&complete_data)
                    .map_err(|e| TransportError::DecryptionFailed(format!("Decryption failed: {}", e)))?
            } else {
                complete_data
            };

            // Deserialize
            let message: Message = bincode::deserialize(&decrypted)
                .map_err(|e| TransportError::Other(format!("Deserialization failed: {}", e)))?;

            // Add to receive queue
            self.recv_queue.lock().await.push_back(message);
            self.recv_notify.notify_one();

            // Remove reassembler
            reassemblers.remove(&header.message_id);
        }

        Ok(())
    }

    /// Simulate disconnection
    pub fn disconnect(&self) {
        self.connected.store(false, Ordering::SeqCst);
        self.recv_notify.notify_waiters();
    }

    /// Simulate reconnection
    pub fn reconnect(&self) {
        self.connected.store(true, Ordering::SeqCst);
    }
}

#[async_trait]
impl Transport for MockBleTransport {
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        // 1. Store original message
        self.sent_messages.lock().await.push(msg.clone());

        // 2. Serialize
        let serialized = bincode::serialize(msg)
            .map_err(|e| TransportError::SendFailed(format!("Serialization failed: {}", e)))?;

        // 3. Encrypt (if enabled)
        let data = if let Some(ref cipher) = self.encryption {
            cipher.encrypt(&serialized)
                .map_err(|e| TransportError::SendFailed(format!("Encryption failed: {}", e)))?
        } else {
            serialized
        };

        // 4. Chunk
        let message_id = self.next_message_id.fetch_add(1, Ordering::SeqCst);
        let mut chunker = Chunker::new(message_id, &data, self.mtu);
        let chunks = chunker.create_all_chunks();

        // 5. Store chunks
        let mut sent_chunks = self.sent_chunks.lock().await;
        for chunk in chunks {
            sent_chunks.push(chunk.clone());
        }

        Ok(())
    }

    async fn recv(&self) -> Result<Message, TransportError> {
        loop {
            if !self.connected.load(Ordering::SeqCst) {
                return Err(TransportError::ConnectionClosed);
            }

            // Try to get a message from the queue
            {
                let mut queue = self.recv_queue.lock().await;
                if let Some(msg) = queue.pop_front() {
                    return Ok(msg);
                }
            }

            // Wait for notification
            self.recv_notify.notified().await;
        }
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn channel(&self) -> Channel {
        Channel::Ble
    }

    fn peer_device_id(&self) -> &str {
        &self.device_id
    }

    async fn close(&self) -> Result<(), TransportError> {
        self.connected.store(false, Ordering::SeqCst);
        self.recv_notify.notify_waiters();
        Ok(())
    }
}

/// Create a pair of connected mock BLE transports with encryption
///
/// Both transports use the same shared secret, so messages can be
/// encrypted by one and decrypted by the other.
///
/// # Arguments
/// * `device_a` - Device ID for transport A
/// * `device_b` - Device ID for transport B
/// * `shared_secret` - 32-byte shared secret for encryption
///
/// # Returns
/// A tuple of two mock BLE transports that can communicate securely
pub fn create_encrypted_pair(
    device_a: impl Into<String>,
    device_b: impl Into<String>,
    shared_secret: &[u8],
) -> Result<(Arc<MockBleTransport>, Arc<MockBleTransport>), TransportError> {
    let transport_a = Arc::new(MockBleTransport::new_with_encryption(device_a, shared_secret)?);
    let transport_b = Arc::new(MockBleTransport::new_with_encryption(device_b, shared_secret)?);

    Ok((transport_a, transport_b))
}

/// Create a pair of connected mock BLE transports without encryption
pub fn create_unencrypted_pair(
    device_a: impl Into<String>,
    device_b: impl Into<String>,
) -> (Arc<MockBleTransport>, Arc<MockBleTransport>) {
    let transport_a = Arc::new(MockBleTransport::new_without_encryption(device_a));
    let transport_b = Arc::new(MockBleTransport::new_without_encryption(device_b));

    (transport_a, transport_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nearclip_sync::Message;

    fn create_test_message(content: &str) -> Message {
        Message::clipboard_sync(content.as_bytes(), "test_device".to_string())
    }

    #[tokio::test]
    async fn test_mock_ble_without_encryption() {
        let transport = MockBleTransport::new_without_encryption("device_1");

        // Send a message
        let msg = create_test_message("hello world");
        transport.send(&msg).await.unwrap();

        // Check sent messages
        let sent = transport.get_sent_messages().await;
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].payload, msg.payload);

        // Check that chunks were created
        let chunks = transport.get_sent_chunks().await;
        assert!(!chunks.is_empty());
    }

    #[tokio::test]
    async fn test_mock_ble_with_encryption() {
        let shared_secret = [0u8; 32]; // Test key
        let transport = MockBleTransport::new_with_encryption("device_1", &shared_secret).unwrap();

        // Send a message
        let msg = create_test_message("secret message");
        transport.send(&msg).await.unwrap();

        // Check sent messages
        let sent = transport.get_sent_messages().await;
        assert_eq!(sent.len(), 1);

        // Chunks should be created and encrypted
        let chunks = transport.get_sent_chunks().await;
        assert!(!chunks.is_empty());
    }

    #[tokio::test]
    async fn test_inject_and_receive() {
        let shared_secret = [1u8; 32];
        let transport = MockBleTransport::new_with_encryption("device_1", &shared_secret).unwrap();

        // Inject a message
        let msg = create_test_message("injected message");
        transport.inject_message(&msg).await.unwrap();

        // Receive it
        let received = transport.recv().await.unwrap();
        assert_eq!(received.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_encrypted_pair_communication() {
        let shared_secret = [2u8; 32];
        let (transport_a, transport_b) =
            create_encrypted_pair("device_a", "device_b", &shared_secret).unwrap();

        // A sends to B
        let msg = create_test_message("encrypted hello");

        // A encrypts and chunks
        transport_a.send(&msg).await.unwrap();

        // Get chunks from A
        let chunks = transport_a.get_sent_chunks().await;

        // Manually feed chunks to B's reassembler (simulating BLE transfer)
        let mut reassemblers = transport_b.reassemblers.lock().await;
        for chunk in chunks {
            transport_b.process_chunk(&chunk, &mut reassemblers).await.unwrap();
        }
        drop(reassemblers);

        // B should receive the decrypted message
        let received = transport_b.recv().await.unwrap();
        assert_eq!(received.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_disconnect() {
        let transport = MockBleTransport::new_without_encryption("device_1");

        assert!(transport.is_connected());
        transport.disconnect();
        assert!(!transport.is_connected());

        // Send should fail
        let msg = create_test_message("test");
        let result = transport.send(&msg).await;
        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }
}
