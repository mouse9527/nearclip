//! Mock transport for testing and debug mode

use async_trait::async_trait;
use nearclip_sync::{Channel, Message};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Notify};
use tracing::debug;

use crate::error::TransportError;
use crate::traits::Transport;

/// Configuration for mock transport behavior
#[derive(Debug, Clone)]
pub struct MockConfig {
    /// Simulated latency for send operations
    pub latency: Duration,
    /// Simulated packet drop rate (0.0 - 1.0)
    pub drop_rate: f32,
    /// Simulated error on send (if Some, all sends will fail with this error)
    pub error_on_send: Option<TransportError>,
    /// Channel type to simulate
    pub channel: Channel,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            latency: Duration::ZERO,
            drop_rate: 0.0,
            error_on_send: None,
            channel: Channel::Wifi,
        }
    }
}

impl MockConfig {
    /// Create a new mock config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set simulated latency
    pub fn with_latency(mut self, latency: Duration) -> Self {
        self.latency = latency;
        self
    }

    /// Set simulated drop rate
    pub fn with_drop_rate(mut self, rate: f32) -> Self {
        self.drop_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Set simulated error
    pub fn with_error(mut self, error: TransportError) -> Self {
        self.error_on_send = Some(error);
        self
    }

    /// Set channel type
    pub fn with_channel(mut self, channel: Channel) -> Self {
        self.channel = channel;
        self
    }
}

/// Mock transport for testing
///
/// This transport stores sent messages in a queue and allows injecting
/// received messages for testing purposes.
pub struct MockTransport {
    device_id: String,
    /// Messages that have been sent (for inspection in tests)
    sent_messages: Arc<Mutex<Vec<Message>>>,
    /// Messages to be received (injected by tests)
    recv_queue: Arc<Mutex<VecDeque<Message>>>,
    /// Notifier for new messages
    recv_notify: Arc<Notify>,
    /// Configuration
    config: MockConfig,
    /// Connection state
    connected: AtomicBool,
    /// Peer transport (for bidirectional communication)
    peer: Option<Arc<MockTransport>>,
}

impl MockTransport {
    /// Create a new mock transport
    pub fn new(device_id: impl Into<String>, config: MockConfig) -> Self {
        Self {
            device_id: device_id.into(),
            sent_messages: Arc::new(Mutex::new(Vec::new())),
            recv_queue: Arc::new(Mutex::new(VecDeque::new())),
            recv_notify: Arc::new(Notify::new()),
            config,
            connected: AtomicBool::new(true),
            peer: None,
        }
    }

    /// Create with default config
    pub fn with_defaults(device_id: impl Into<String>) -> Self {
        Self::new(device_id, MockConfig::default())
    }

    /// Inject a message to be received
    ///
    /// This is used in tests to simulate receiving a message from the peer.
    pub async fn inject_message(&self, msg: Message) {
        let mut queue = self.recv_queue.lock().await;
        queue.push_back(msg);
        self.recv_notify.notify_one();
    }

    /// Inject a message synchronously (for use in non-async contexts)
    pub fn inject_message_sync(&self, msg: Message) {
        let mut queue = self.recv_queue.blocking_lock();
        queue.push_back(msg);
        self.recv_notify.notify_one();
    }

    /// Get all sent messages
    pub async fn get_sent_messages(&self) -> Vec<Message> {
        self.sent_messages.lock().await.clone()
    }

    /// Get sent messages synchronously
    pub fn get_sent_messages_sync(&self) -> Vec<Message> {
        self.sent_messages.blocking_lock().clone()
    }

    /// Clear sent messages
    pub async fn clear_sent(&self) {
        self.sent_messages.lock().await.clear();
    }

    /// Clear sent messages synchronously
    pub fn clear_sent_sync(&self) {
        self.sent_messages.blocking_lock().clear();
    }

    /// Get the number of sent messages
    pub async fn sent_count(&self) -> usize {
        self.sent_messages.lock().await.len()
    }

    /// Get the number of pending received messages
    pub async fn recv_queue_len(&self) -> usize {
        self.recv_queue.lock().await.len()
    }

    /// Set the peer transport for bidirectional communication
    pub fn set_peer(&mut self, peer: Arc<MockTransport>) {
        self.peer = Some(peer);
    }

    /// Simulate connection loss
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
impl Transport for MockTransport {
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        // Simulate latency
        if self.config.latency > Duration::ZERO {
            tokio::time::sleep(self.config.latency).await;
        }

        // Simulate error
        if let Some(ref err) = self.config.error_on_send {
            return Err(err.clone());
        }

        // Simulate packet drop
        if self.config.drop_rate > 0.0 {
            let random: f32 = rand_float();
            if random < self.config.drop_rate {
                debug!("Mock transport: dropping message (simulated)");
                return Ok(()); // Silently drop
            }
        }

        // Store sent message
        self.sent_messages.lock().await.push(msg.clone());

        // If we have a peer, deliver the message to them
        if let Some(ref peer) = self.peer {
            peer.inject_message(msg.clone()).await;
        }

        debug!("Mock transport: sent message to {}", self.device_id);
        Ok(())
    }

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
                    debug!("Mock transport: received message from {}", self.device_id);
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
        self.config.channel
    }

    fn peer_device_id(&self) -> &str {
        &self.device_id
    }

    async fn close(&self) -> Result<(), TransportError> {
        self.connected.store(false, Ordering::SeqCst);
        self.recv_notify.notify_waiters();
        debug!("Mock transport closed for device {}", self.device_id);
        Ok(())
    }
}

/// Create a pair of connected mock transports
///
/// Messages sent on one transport will be received by the other.
/// This is useful for testing bidirectional communication.
///
/// # Arguments
/// * `device_a` - Device ID for the first transport
/// * `device_b` - Device ID for the second transport
///
/// # Returns
/// A tuple of two connected mock transports
pub fn create_mock_pair(
    device_a: impl Into<String>,
    device_b: impl Into<String>,
) -> (Arc<dyn Transport>, Arc<dyn Transport>) {
    create_mock_pair_with_config(device_a, device_b, MockConfig::default(), MockConfig::default())
}

/// Create a pair of connected mock transports with custom configs
pub fn create_mock_pair_with_config(
    device_a: impl Into<String>,
    device_b: impl Into<String>,
    config_a: MockConfig,
    config_b: MockConfig,
) -> (Arc<dyn Transport>, Arc<dyn Transport>) {
    let device_a = device_a.into();
    let device_b = device_b.into();

    // Create shared message queues
    let queue_a_to_b: Arc<Mutex<VecDeque<Message>>> = Arc::new(Mutex::new(VecDeque::new()));
    let queue_b_to_a: Arc<Mutex<VecDeque<Message>>> = Arc::new(Mutex::new(VecDeque::new()));
    let notify_a = Arc::new(Notify::new());
    let notify_b = Arc::new(Notify::new());

    let transport_a = MockTransportPair {
        device_id: device_a,
        sent_messages: Arc::new(Mutex::new(Vec::new())),
        send_queue: queue_a_to_b.clone(),
        recv_queue: queue_b_to_a.clone(),
        send_notify: notify_b.clone(),
        recv_notify: notify_a.clone(),
        config: config_a,
        connected: AtomicBool::new(true),
    };

    let transport_b = MockTransportPair {
        device_id: device_b,
        sent_messages: Arc::new(Mutex::new(Vec::new())),
        send_queue: queue_b_to_a,
        recv_queue: queue_a_to_b,
        send_notify: notify_a,
        recv_notify: notify_b,
        config: config_b,
        connected: AtomicBool::new(true),
    };

    (Arc::new(transport_a), Arc::new(transport_b))
}

/// A mock transport that is part of a connected pair
struct MockTransportPair {
    device_id: String,
    sent_messages: Arc<Mutex<Vec<Message>>>,
    send_queue: Arc<Mutex<VecDeque<Message>>>,
    recv_queue: Arc<Mutex<VecDeque<Message>>>,
    send_notify: Arc<Notify>,
    recv_notify: Arc<Notify>,
    config: MockConfig,
    connected: AtomicBool,
}

#[async_trait]
impl Transport for MockTransportPair {
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        // Simulate latency
        if self.config.latency > Duration::ZERO {
            tokio::time::sleep(self.config.latency).await;
        }

        // Simulate error
        if let Some(ref err) = self.config.error_on_send {
            return Err(err.clone());
        }

        // Simulate packet drop
        if self.config.drop_rate > 0.0 {
            let random: f32 = rand_float();
            if random < self.config.drop_rate {
                return Ok(());
            }
        }

        // Store sent message locally
        self.sent_messages.lock().await.push(msg.clone());

        // Put message in send queue for peer to receive
        self.send_queue.lock().await.push_back(msg.clone());

        // Notify peer
        self.send_notify.notify_one();

        Ok(())
    }

    async fn recv(&self) -> Result<Message, TransportError> {
        loop {
            if !self.connected.load(Ordering::SeqCst) {
                return Err(TransportError::ConnectionClosed);
            }

            // Check recv queue
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
        self.config.channel
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

/// Simple random float generator (0.0 - 1.0)
fn rand_float() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 1000) as f32 / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message(content: &str) -> Message {
        Message::clipboard_sync(content.as_bytes(), "test_device".to_string())
    }

    #[tokio::test]
    async fn test_mock_transport_send_receive() {
        let transport = MockTransport::with_defaults("device_1");

        // Inject a message
        let msg = create_test_message("hello");
        transport.inject_message(msg.clone()).await;

        // Receive it
        let received = transport.recv().await.unwrap();
        assert_eq!(received.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_mock_transport_sent_messages() {
        let transport = MockTransport::with_defaults("device_1");

        let msg = create_test_message("test");
        transport.send(&msg).await.unwrap();

        let sent = transport.get_sent_messages().await;
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].payload, msg.payload);
    }

    #[tokio::test]
    async fn test_mock_transport_disconnect() {
        let transport = MockTransport::with_defaults("device_1");

        assert!(transport.is_connected());
        transport.disconnect();
        assert!(!transport.is_connected());

        // Send should fail
        let msg = create_test_message("test");
        let result = transport.send(&msg).await;
        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_mock_transport_with_latency() {
        let config = MockConfig::new().with_latency(Duration::from_millis(50));
        let transport = MockTransport::new("device_1", config);

        let msg = create_test_message("test");
        let start = std::time::Instant::now();
        transport.send(&msg).await.unwrap();
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_mock_transport_with_error() {
        let config = MockConfig::new()
            .with_error(TransportError::SendFailed("simulated error".to_string()));
        let transport = MockTransport::new("device_1", config);

        let msg = create_test_message("test");
        let result = transport.send(&msg).await;
        assert!(matches!(result, Err(TransportError::SendFailed(_))));
    }

    #[tokio::test]
    async fn test_mock_pair_communication() {
        let (transport_a, transport_b) = create_mock_pair("device_a", "device_b");

        // Send from A
        let msg = create_test_message("hello from A");
        transport_a.send(&msg).await.unwrap();

        // Receive on B
        let received = transport_b.recv().await.unwrap();
        assert_eq!(received.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_mock_pair_bidirectional() {
        let (transport_a, transport_b) = create_mock_pair("device_a", "device_b");

        // Send from A to B
        let msg_a = create_test_message("from A");
        transport_a.send(&msg_a).await.unwrap();

        // Send from B to A
        let msg_b = create_test_message("from B");
        transport_b.send(&msg_b).await.unwrap();

        // Receive on B
        let received_b = transport_b.recv().await.unwrap();
        assert_eq!(received_b.payload, msg_a.payload);

        // Receive on A
        let received_a = transport_a.recv().await.unwrap();
        assert_eq!(received_a.payload, msg_b.payload);
    }

    #[test]
    fn test_mock_config_builder() {
        let config = MockConfig::new()
            .with_latency(Duration::from_millis(100))
            .with_drop_rate(0.5)
            .with_channel(Channel::Ble);

        assert_eq!(config.latency, Duration::from_millis(100));
        assert_eq!(config.drop_rate, 0.5);
        assert_eq!(config.channel, Channel::Ble);
    }
}
