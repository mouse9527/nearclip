//! Transport traits - unified interface for WiFi and BLE transports

use async_trait::async_trait;
use nearclip_sync::{Channel, Message};
use std::sync::Arc;

use crate::TransportError;

/// Unified transport interface
///
/// This trait abstracts over different transport mechanisms (WiFi/TCP, BLE),
/// allowing upper layers to send and receive messages without caring about
/// the underlying transport.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message to the peer device
    ///
    /// # Arguments
    /// * `msg` - The message to send
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(TransportError)` if sending failed
    async fn send(&self, msg: &Message) -> Result<(), TransportError>;

    /// Receive a message from the peer device
    ///
    /// This method blocks until a message is received or the connection is closed.
    ///
    /// # Returns
    /// * `Ok(Message)` - The received message
    /// * `Err(TransportError::ConnectionClosed)` - If the connection was closed
    /// * `Err(TransportError)` - If receiving failed
    async fn recv(&self) -> Result<Message, TransportError>;

    /// Check if the transport is connected
    fn is_connected(&self) -> bool;

    /// Get the transport channel type (WiFi or BLE)
    fn channel(&self) -> Channel;

    /// Get the peer device ID
    fn peer_device_id(&self) -> &str;

    /// Close the transport connection
    async fn close(&self) -> Result<(), TransportError>;
}

/// Transport connector - used to establish outbound connections
///
/// Implementations of this trait can connect to remote devices
/// using a specific transport mechanism.
#[async_trait]
pub trait TransportConnector: Send + Sync {
    /// Connect to a device by its ID
    ///
    /// # Arguments
    /// * `device_id` - The ID of the device to connect to
    /// * `address` - Transport-specific address information
    ///
    /// # Returns
    /// * `Ok(Box<dyn Transport>)` - A connected transport
    /// * `Err(TransportError)` - If connection failed
    async fn connect(
        &self,
        device_id: &str,
        address: &str,
    ) -> Result<Arc<dyn Transport>, TransportError>;

    /// Get the channel type this connector supports
    fn channel(&self) -> Channel;
}

/// Transport listener - used to accept inbound connections
///
/// Implementations of this trait can accept incoming connections
/// from remote devices.
#[async_trait]
pub trait TransportListener: Send + Sync {
    /// Accept an incoming connection
    ///
    /// This method blocks until a connection is received.
    ///
    /// # Returns
    /// * `Ok(Box<dyn Transport>)` - An accepted transport
    /// * `Err(TransportError)` - If accepting failed
    async fn accept(&self) -> Result<Arc<dyn Transport>, TransportError>;

    /// Get the channel type this listener supports
    fn channel(&self) -> Channel;

    /// Get the local address this listener is bound to
    fn local_address(&self) -> String;
}

/// Callback interface for transport events
///
/// Implementations receive notifications about transport-level events.
pub trait TransportCallback: Send + Sync {
    /// Called when a new transport connection is established
    fn on_transport_connected(&self, device_id: &str, channel: Channel);

    /// Called when a transport connection is closed
    fn on_transport_disconnected(&self, device_id: &str, channel: Channel);

    /// Called when a message is received
    fn on_message_received(&self, device_id: &str, msg: Message);

    /// Called when a transport error occurs
    fn on_transport_error(&self, device_id: &str, error: &TransportError);
}

/// No-op callback implementation for testing
#[allow(dead_code)]
pub struct NoOpCallback;

impl TransportCallback for NoOpCallback {
    fn on_transport_connected(&self, _device_id: &str, _channel: Channel) {}
    fn on_transport_disconnected(&self, _device_id: &str, _channel: Channel) {}
    fn on_message_received(&self, _device_id: &str, _msg: Message) {}
    fn on_transport_error(&self, _device_id: &str, _error: &TransportError) {}
}
