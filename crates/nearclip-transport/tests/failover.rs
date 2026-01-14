//! Transport failover integration tests
//!
//! These tests verify that the transport manager correctly:
//! - Fails over from WiFi to BLE when WiFi fails
//! - Respects failover configuration
//! - Handles seamless transport switching

use nearclip_sync::{Channel, Message};
use nearclip_transport::{Transport, TransportManager, TransportManagerConfig};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use tokio::sync::Mutex;

/// Mock transport that can be configured to fail on demand
struct FailableTransport {
    channel: Channel,
    device_id: String,
    connected: AtomicBool,
    should_fail: AtomicBool,
    sent_count: AtomicUsize,
    sent_messages: Mutex<Vec<Message>>,
}

impl FailableTransport {
    fn new(channel: Channel, device_id: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            channel,
            device_id: device_id.into(),
            connected: AtomicBool::new(true),
            should_fail: AtomicBool::new(false),
            sent_count: AtomicUsize::new(0),
            sent_messages: Mutex::new(Vec::new()),
        })
    }

    fn set_should_fail(&self, should_fail: bool) {
        self.should_fail.store(should_fail, Ordering::SeqCst);
    }

    fn disconnect(&self) {
        self.connected.store(false, Ordering::SeqCst);
    }

    fn reconnect(&self) {
        self.connected.store(true, Ordering::SeqCst);
    }

    async fn get_sent_count(&self) -> usize {
        self.sent_count.load(Ordering::SeqCst)
    }

    async fn get_sent_messages(&self) -> Vec<Message> {
        self.sent_messages.lock().await.clone()
    }
}

#[async_trait::async_trait]
impl Transport for FailableTransport {
    async fn send(&self, msg: &Message) -> Result<(), nearclip_transport::TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(nearclip_transport::TransportError::ConnectionClosed);
        }

        if self.should_fail.load(Ordering::SeqCst) {
            return Err(nearclip_transport::TransportError::SendFailed("Configured to fail".to_string()));
        }

        self.sent_count.fetch_add(1, Ordering::SeqCst);
        self.sent_messages.lock().await.push(msg.clone());
        Ok(())
    }

    async fn recv(&self) -> Result<Message, nearclip_transport::TransportError> {
        // Not implemented for this test
        Err(nearclip_transport::TransportError::Other("Not implemented".to_string()))
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn channel(&self) -> Channel {
        self.channel
    }

    fn peer_device_id(&self) -> &str {
        &self.device_id
    }

    async fn close(&self) -> Result<(), nearclip_transport::TransportError> {
        self.disconnect();
        Ok(())
    }
}

/// Helper to create a test message
fn create_test_message(content: &str) -> Message {
    Message::clipboard_sync(content.as_bytes(), "test_device".to_string())
}

/// Test 2.1: WiFi fails, automatically falls back to BLE
#[tokio::test]
async fn test_failover_wifi_to_ble() {
    let manager = TransportManager::with_config(TransportManagerConfig {
        failover_on_error: true,
        ..Default::default()
    });

    // Create WiFi transport that will fail
    let wifi_transport = FailableTransport::new(Channel::Wifi, "device_1");
    wifi_transport.set_should_fail(true); // WiFi will fail

    // Create BLE transport that works
    let ble_transport = FailableTransport::new(Channel::Ble, "device_1");

    // Add both transports
    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // Send message - should failover to BLE
    let msg = create_test_message("test failover");
    let result = manager.send_to_device("device_1", &msg).await;

    // Verify success
    assert!(result.is_ok(), "Send should succeed via failover");

    // Verify BLE was used (WiFi failed, so BLE should have the message)
    assert_eq!(ble_transport.get_sent_count().await, 1, "BLE should have sent 1 message");
}

/// Test 2.2: When failover is disabled, don't try alternative channels
#[tokio::test]
async fn test_no_failover_when_disabled() {
    let manager = TransportManager::with_config(TransportManagerConfig {
        failover_on_error: false, // Disabled
        ..Default::default()
    });

    // Create WiFi transport that will fail
    let wifi_transport = FailableTransport::new(Channel::Wifi, "device_1");
    wifi_transport.set_should_fail(true);

    // Create BLE transport that works
    let ble_transport = FailableTransport::new(Channel::Ble, "device_1");

    // Add both transports
    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // Send message - should fail without trying BLE
    let msg = create_test_message("test no failover");
    let result = manager.send_to_device("device_1", &msg).await;

    // Verify failure (WiFi fails, failover disabled, so send should fail)
    assert!(result.is_err(), "Send should fail when failover is disabled");

    // Verify BLE was NOT used
    assert_eq!(ble_transport.get_sent_count().await, 0, "BLE should not have been used");
}

/// Test 2.3: Seamless switching when WiFi disconnects
#[tokio::test]
async fn test_seamless_switch_on_disconnect() {
    let manager = TransportManager::with_config(TransportManagerConfig {
        failover_on_error: true,
        ..Default::default()
    });

    // Create both transports, initially connected
    let wifi_transport = FailableTransport::new(Channel::Wifi, "device_1");
    let ble_transport = FailableTransport::new(Channel::Ble, "device_1");

    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // First send: WiFi should be preferred (higher priority)
    let msg1 = create_test_message("message 1");
    manager.send_to_device("device_1", &msg1).await.expect("First send should succeed");

    // Verify WiFi was used (it has higher priority)
    assert_eq!(wifi_transport.get_sent_count().await, 1, "WiFi should have sent message 1");
    assert_eq!(ble_transport.get_sent_count().await, 0, "BLE should not have been used yet");

    // WiFi disconnects
    wifi_transport.disconnect();

    // Second send: should use BLE (WiFi is disconnected)
    let msg2 = create_test_message("message 2");
    manager.send_to_device("device_1", &msg2).await.expect("Second send should succeed");

    // Verify BLE was used
    assert_eq!(wifi_transport.get_sent_count().await, 1, "WiFi count should not increase");
    assert_eq!(ble_transport.get_sent_count().await, 1, "BLE should have sent message 2");
}

/// Test 2.4: All transports fail - error is returned
#[tokio::test]
async fn test_all_transports_fail() {
    let manager = TransportManager::with_config(TransportManagerConfig {
        failover_on_error: true,
        ..Default::default()
    });

    // Create both transports configured to fail
    let wifi_transport = FailableTransport::new(Channel::Wifi, "device_1");
    wifi_transport.set_should_fail(true);

    let ble_transport = FailableTransport::new(Channel::Ble, "device_1");
    ble_transport.set_should_fail(true);

    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // Send should fail - all channels failed
    let msg = create_test_message("test all fail");
    let result = manager.send_to_device("device_1", &msg).await;

    assert!(result.is_err(), "Send should fail when all transports fail");
}

/// Test 2.5: WiFi recovers - should switch back to WiFi
#[tokio::test]
async fn test_wifi_recovery() {
    let manager = TransportManager::with_config(TransportManagerConfig {
        failover_on_error: true,
        ..Default::default()
    });

    let wifi_transport = FailableTransport::new(Channel::Wifi, "device_1");
    let ble_transport = FailableTransport::new(Channel::Ble, "device_1");

    manager.add_transport("device_1", wifi_transport.clone()).await;
    manager.add_transport("device_1", ble_transport.clone()).await;

    // 1. WiFi fails
    wifi_transport.set_should_fail(true);

    // 2. Send using BLE (WiFi is failing)
    let msg1 = create_test_message("message during WiFi failure");
    manager.send_to_device("device_1", &msg1).await.expect("Should failover to BLE");
    assert_eq!(ble_transport.get_sent_count().await, 1);

    // 3. WiFi recovers
    wifi_transport.set_should_fail(false);

    // 4. Next send should use WiFi again (higher priority)
    let msg2 = create_test_message("message after WiFi recovery");
    manager.send_to_device("device_1", &msg2).await.expect("Should use WiFi again");
    assert_eq!(wifi_transport.get_sent_count().await, 1, "WiFi should be used after recovery");
}
