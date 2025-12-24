//! BLE transport implementation using FFI callbacks
//!
//! BLE transport works differently from WiFi - the actual BLE operations
//! are performed by platform-native code (Swift/Kotlin), and this module
//! provides the bridge between the Rust transport layer and the platform.

use async_trait::async_trait;
use nearclip_ble::{ChunkHeader, Chunker, Reassembler, DEFAULT_BLE_MTU, DEFAULT_REASSEMBLE_TIMEOUT, CHUNK_HEADER_SIZE};
use nearclip_sync::{Channel, Message};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use std::collections::VecDeque;
use tracing::{debug, warn, instrument};

use crate::error::TransportError;
use crate::traits::Transport;

/// BLE sender interface - platform must implement this
///
/// This trait is implemented by platform-native code (Swift/Kotlin)
/// to provide BLE send capability.
pub trait BleSender: Send + Sync {
    /// Send raw data over BLE to a device
    ///
    /// # Arguments
    /// * `device_id` - The target device ID
    /// * `data` - Raw bytes to send (already chunked)
    ///
    /// # Returns
    /// * `Ok(())` if send was successful
    /// * `Err(String)` with error message if failed
    fn send_ble_data(&self, device_id: &str, data: &[u8]) -> Result<(), String>;

    /// Check if BLE is connected to a device
    fn is_ble_connected(&self, device_id: &str) -> bool;

    /// Get the negotiated MTU for a device
    fn get_mtu(&self, device_id: &str) -> usize;
}

/// Process a received BLE chunk and return a complete message if reassembly is done
///
/// This is the core chunk processing logic shared by async and sync code paths.
fn process_chunk(
    data: &[u8],
    reassemblers: &mut HashMap<u16, Reassembler>,
) -> Option<Message> {
    if data.len() < CHUNK_HEADER_SIZE {
        warn!("Received BLE data too short: {} bytes", data.len());
        return None;
    }

    // Parse chunk header
    let header = match ChunkHeader::from_bytes(data) {
        Ok(h) => h,
        Err(e) => {
            warn!("Failed to parse chunk header: {}", e);
            return None;
        }
    };

    let payload = data[CHUNK_HEADER_SIZE..].to_vec();

    // Validate payload length
    if payload.len() != header.payload_length as usize {
        warn!(
            "Payload length mismatch: header says {}, actual {}",
            header.payload_length,
            payload.len()
        );
        return None;
    }

    // Get or create reassembler for this message
    let reassembler = reassemblers
        .entry(header.message_id)
        .or_insert_with(|| {
            debug!(
                message_id = header.message_id,
                total_chunks = header.total_chunks,
                "Creating new reassembler"
            );
            Reassembler::new(
                header.message_id,
                header.total_chunks,
                DEFAULT_REASSEMBLE_TIMEOUT,
            )
        });

    // Add chunk to reassembler
    if let Err(e) = reassembler.add_chunk(header, payload) {
        warn!("Failed to add chunk: {}", e);
        return None;
    }

    // Check if message is complete
    let result = if reassembler.is_complete() {
        // Remove reassembler and assemble message
        if let Some(reassembler) = reassemblers.remove(&header.message_id) {
            match reassembler.assemble() {
                Ok(data) => {
                    // Deserialize message
                    match Message::deserialize(&data) {
                        Ok(msg) => {
                            debug!(
                                message_id = header.message_id,
                                "BLE message reassembled and queued"
                            );
                            Some(msg)
                        }
                        Err(e) => {
                            warn!("Failed to deserialize BLE message: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to assemble BLE message: {}", e);
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    // Clean up expired reassemblers
    reassemblers.retain(|id, r| {
        if r.is_expired() {
            warn!(message_id = id, "Reassembler expired, dropping");
            false
        } else {
            true
        }
    });

    result
}

/// BLE transport - bridges to platform BLE via FFI callbacks
///
/// This transport uses platform-native BLE for actual data transfer.
/// Send operations call into platform code via `BleSender`.
/// Receive operations are handled by platform calling `on_data_received`.
pub struct BleTransport {
    device_id: String,
    /// Platform BLE sender
    sender: Arc<dyn BleSender>,
    /// Receive queue - platform callbacks push messages here
    recv_queue: Arc<Mutex<VecDeque<Message>>>,
    /// Notifier for new messages
    recv_notify: Arc<Notify>,
    /// Connection state
    connected: AtomicBool,
    /// Message ID counter for chunking
    message_id_counter: AtomicU16,
    /// Reassemblers for incoming chunked messages
    reassemblers: Arc<Mutex<HashMap<u16, Reassembler>>>,
}

impl BleTransport {
    /// Create a new BLE transport
    ///
    /// # Arguments
    /// * `device_id` - The peer device ID
    /// * `sender` - Platform BLE sender implementation
    pub fn new(device_id: String, sender: Arc<dyn BleSender>) -> Self {
        Self {
            device_id,
            sender,
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            connected: AtomicBool::new(true),
            message_id_counter: AtomicU16::new(0),
            reassemblers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the next message ID
    fn next_message_id(&self) -> u16 {
        self.message_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Called by platform when BLE data is received
    ///
    /// This method handles chunked data reassembly and queues
    /// complete messages for the recv() method.
    ///
    /// # Arguments
    /// * `data` - Raw bytes received from BLE (a single chunk)
    pub async fn on_data_received(&self, data: &[u8]) {
        let mut reassemblers = self.reassemblers.lock().await;
        if let Some(msg) = process_chunk(data, &mut reassemblers) {
            let mut queue = self.recv_queue.lock().await;
            queue.push_back(msg);
            self.recv_notify.notify_one();
        }
    }

    /// Called by platform when BLE data is received (sync version)
    ///
    /// This is a blocking version for use from FFI callbacks.
    pub fn on_data_received_sync(&self, data: &[u8]) {
        // Try to get a handle to the runtime
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let data = data.to_vec();
            let recv_queue = self.recv_queue.clone();
            let recv_notify = self.recv_notify.clone();
            let reassemblers = self.reassemblers.clone();

            handle.spawn(async move {
                let mut reassemblers = reassemblers.lock().await;
                if let Some(msg) = process_chunk(&data, &mut reassemblers) {
                    let mut queue = recv_queue.lock().await;
                    queue.push_back(msg);
                    recv_notify.notify_one();
                }
            });
        } else {
            // No runtime available, use blocking lock
            let mut reassemblers = self.reassemblers.blocking_lock();
            if let Some(msg) = process_chunk(data, &mut reassemblers) {
                let mut queue = self.recv_queue.blocking_lock();
                queue.push_back(msg);
                self.recv_notify.notify_one();
            }
        }
    }

    /// Called by platform when BLE connection state changes
    pub fn on_connection_state_changed(&self, connected: bool) {
        self.connected.store(connected, Ordering::SeqCst);
        if !connected {
            self.recv_notify.notify_waiters();
        }
    }
}

#[async_trait]
impl Transport for BleTransport {
    #[instrument(skip(self, msg), fields(device_id = %self.device_id, msg_type = ?msg.msg_type))]
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        if !self.sender.is_ble_connected(&self.device_id) {
            self.connected.store(false, Ordering::SeqCst);
            return Err(TransportError::ConnectionClosed);
        }

        // Serialize message
        let data = msg.serialize()
            .map_err(|e| TransportError::Serialization(e.to_string()))?;

        // Get MTU and chunk data
        let mtu = self.sender.get_mtu(&self.device_id);
        let mtu = if mtu == 0 { DEFAULT_BLE_MTU } else { mtu };
        let message_id = self.next_message_id();

        let chunks = Chunker::chunk(&data, message_id, mtu)
            .map_err(|e| TransportError::Ble(e.to_string()))?;

        debug!(
            device_id = %self.device_id,
            message_id,
            chunks = chunks.len(),
            data_len = data.len(),
            "Sending BLE message"
        );

        // Send each chunk
        for (i, chunk) in chunks.iter().enumerate() {
            self.sender.send_ble_data(&self.device_id, chunk)
                .map_err(|e| {
                    self.connected.store(false, Ordering::SeqCst);
                    TransportError::SendFailed(format!("BLE send failed at chunk {}: {}", i, e))
                })?;
        }

        debug!(
            device_id = %self.device_id,
            message_id,
            "BLE message sent successfully"
        );

        Ok(())
    }

    #[instrument(skip(self), fields(device_id = %self.device_id))]
    async fn recv(&self) -> Result<Message, TransportError> {
        loop {
            // Check connection state
            if !self.connected.load(Ordering::SeqCst) {
                return Err(TransportError::ConnectionClosed);
            }

            // Try to get a message from the queue
            {
                let mut queue = self.recv_queue.lock().await;
                if let Some(msg) = queue.pop_front() {
                    debug!(device_id = %self.device_id, "BLE message received");
                    return Ok(msg);
                }
            }

            // Wait for notification
            self.recv_notify.notified().await;
        }
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst) && self.sender.is_ble_connected(&self.device_id)
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
        debug!(device_id = %self.device_id, "BLE transport closed");
        Ok(())
    }
}

/// Mock BLE sender for testing
#[cfg(test)]
pub struct MockBleSender {
    connected: AtomicBool,
    mtu: usize,
    sent_data: std::sync::Mutex<Vec<Vec<u8>>>,
}

#[cfg(test)]
impl MockBleSender {
    pub fn new() -> Self {
        Self {
            connected: AtomicBool::new(true),
            mtu: DEFAULT_BLE_MTU,
            sent_data: std::sync::Mutex::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn with_mtu(mtu: usize) -> Self {
        Self {
            connected: AtomicBool::new(true),
            mtu,
            sent_data: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn disconnect(&self) {
        self.connected.store(false, Ordering::SeqCst);
    }

    pub fn get_sent_data(&self) -> Vec<Vec<u8>> {
        self.sent_data.lock().unwrap().clone()
    }
}

#[cfg(test)]
impl BleSender for MockBleSender {
    fn send_ble_data(&self, _device_id: &str, data: &[u8]) -> Result<(), String> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err("Not connected".to_string());
        }
        self.sent_data.lock().unwrap().push(data.to_vec());
        Ok(())
    }

    fn is_ble_connected(&self, _device_id: &str) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn get_mtu(&self, _device_id: &str) -> usize {
        self.mtu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message(content: &str) -> Message {
        Message::clipboard_sync(content.as_bytes(), "test_device".to_string())
    }

    #[tokio::test]
    async fn test_ble_transport_send() {
        let sender = Arc::new(MockBleSender::new());
        let transport = BleTransport::new("device_1".to_string(), sender.clone());

        let msg = create_test_message("hello");
        transport.send(&msg).await.unwrap();

        let sent = sender.get_sent_data();
        assert!(!sent.is_empty());
    }

    #[tokio::test]
    async fn test_ble_transport_send_disconnected() {
        let sender = Arc::new(MockBleSender::new());
        sender.disconnect();
        let transport = BleTransport::new("device_1".to_string(), sender);

        let msg = create_test_message("hello");
        let result = transport.send(&msg).await;

        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_ble_transport_channel() {
        let sender = Arc::new(MockBleSender::new());
        let transport = BleTransport::new("device_1".to_string(), sender);

        assert_eq!(transport.channel(), Channel::Ble);
    }

    #[tokio::test]
    async fn test_ble_transport_peer_device_id() {
        let sender = Arc::new(MockBleSender::new());
        let transport = BleTransport::new("device_1".to_string(), sender);

        assert_eq!(transport.peer_device_id(), "device_1");
    }

    #[tokio::test]
    async fn test_ble_transport_is_connected() {
        let sender = Arc::new(MockBleSender::new());
        let transport = BleTransport::new("device_1".to_string(), sender.clone());

        assert!(transport.is_connected());

        sender.disconnect();
        assert!(!transport.is_connected());
    }

    #[tokio::test]
    async fn test_ble_transport_close() {
        let sender = Arc::new(MockBleSender::new());
        let transport = BleTransport::new("device_1".to_string(), sender);

        assert!(transport.is_connected());

        transport.close().await.unwrap();
        assert!(!transport.is_connected());
    }

    #[tokio::test]
    async fn test_ble_transport_recv_with_injected_data() {
        let sender = Arc::new(MockBleSender::new());
        let transport = Arc::new(BleTransport::new("device_1".to_string(), sender));

        // Create a message and chunk it
        let msg = create_test_message("hello from BLE");
        let data = msg.serialize().unwrap();
        let chunks = Chunker::chunk(&data, 1, DEFAULT_BLE_MTU).unwrap();

        // Inject chunks as if received from BLE
        for chunk in chunks {
            transport.on_data_received(&chunk).await;
        }

        // Receive the message
        let received = transport.recv().await.unwrap();
        assert_eq!(received.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_ble_transport_recv_multiple_messages() {
        let sender = Arc::new(MockBleSender::new());
        let transport = Arc::new(BleTransport::new("device_1".to_string(), sender));

        // Send two messages
        let msg1 = create_test_message("message 1");
        let msg2 = create_test_message("message 2");

        let data1 = msg1.serialize().unwrap();
        let data2 = msg2.serialize().unwrap();

        let chunks1 = Chunker::chunk(&data1, 1, DEFAULT_BLE_MTU).unwrap();
        let chunks2 = Chunker::chunk(&data2, 2, DEFAULT_BLE_MTU).unwrap();

        // Inject all chunks
        for chunk in chunks1 {
            transport.on_data_received(&chunk).await;
        }
        for chunk in chunks2 {
            transport.on_data_received(&chunk).await;
        }

        // Receive both messages
        let received1 = transport.recv().await.unwrap();
        let received2 = transport.recv().await.unwrap();

        assert_eq!(received1.payload, msg1.payload);
        assert_eq!(received2.payload, msg2.payload);
    }
}
