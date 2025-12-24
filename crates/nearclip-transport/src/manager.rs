//! Transport manager - manages connections and channel selection

use nearclip_sync::{Channel, ChannelInfo, ChannelSelector, ChannelStatus, Message, PriorityChannelSelector};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::error::TransportError;
use crate::traits::{Transport, TransportCallback, TransportConnector, TransportListener};

/// Transport manager configuration
#[derive(Debug, Clone)]
pub struct TransportManagerConfig {
    /// Whether to automatically select the best channel
    pub auto_select_channel: bool,
    /// Whether to attempt failover on send failure
    pub failover_on_error: bool,
}

impl Default for TransportManagerConfig {
    fn default() -> Self {
        Self {
            auto_select_channel: true,
            failover_on_error: true,
        }
    }
}

/// Manages transport connections for multiple devices
///
/// The TransportManager maintains connections to multiple devices,
/// potentially using multiple transport channels (WiFi, BLE) per device.
/// It automatically selects the best available channel for each operation.
pub struct TransportManager {
    /// Device connections: device_id -> list of transports
    connections: RwLock<HashMap<String, Vec<Arc<dyn Transport>>>>,

    /// Channel selector for choosing the best transport
    channel_selector: Box<dyn ChannelSelector>,

    /// Transport connectors (for outbound connections)
    connectors: RwLock<Vec<Arc<dyn TransportConnector>>>,

    /// Transport listeners (for inbound connections)
    listeners: RwLock<Vec<Arc<dyn TransportListener>>>,

    /// Callback for transport events
    callback: Option<Arc<dyn TransportCallback>>,

    /// Configuration
    config: TransportManagerConfig,
}

impl TransportManager {
    /// Create a new transport manager with default channel selector
    pub fn new() -> Self {
        Self::with_selector(Box::new(PriorityChannelSelector))
    }

    /// Create a new transport manager with a custom channel selector
    pub fn with_selector(selector: Box<dyn ChannelSelector>) -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            channel_selector: selector,
            connectors: RwLock::new(Vec::new()),
            listeners: RwLock::new(Vec::new()),
            callback: None,
            config: TransportManagerConfig::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(config: TransportManagerConfig) -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            channel_selector: Box::new(PriorityChannelSelector),
            connectors: RwLock::new(Vec::new()),
            listeners: RwLock::new(Vec::new()),
            callback: None,
            config,
        }
    }

    /// Set the callback for transport events
    pub fn set_callback(&mut self, callback: Arc<dyn TransportCallback>) {
        self.callback = Some(callback);
    }

    /// Add a transport connector
    pub async fn add_connector(&self, connector: Arc<dyn TransportConnector>) {
        self.connectors.write().await.push(connector);
    }

    /// Add a transport listener
    pub async fn add_listener(&self, listener: Arc<dyn TransportListener>) {
        self.listeners.write().await.push(listener);
    }

    /// Add a transport for a device
    ///
    /// A device can have multiple transports (e.g., WiFi and BLE).
    pub async fn add_transport(&self, device_id: &str, transport: Arc<dyn Transport>) {
        let mut connections = self.connections.write().await;
        let transports = connections.entry(device_id.to_string()).or_insert_with(Vec::new);

        // Check if we already have a transport for this channel
        let channel = transport.channel();
        if let Some(existing) = transports.iter().find(|t| t.channel() == channel) {
            if existing.is_connected() {
                warn!(
                    "Device {} already has a connected {} transport, replacing",
                    device_id, channel
                );
            }
            // Remove the old transport
            transports.retain(|t| t.channel() != channel);
        }

        transports.push(transport);
        info!("Added {} transport for device {}", channel, device_id);

        // Notify callback
        if let Some(ref callback) = self.callback {
            callback.on_transport_connected(device_id, channel);
        }
    }

    /// Remove a transport for a device
    pub async fn remove_transport(&self, device_id: &str, channel: Channel) {
        let mut connections = self.connections.write().await;
        if let Some(transports) = connections.get_mut(device_id) {
            transports.retain(|t| t.channel() != channel);
            info!("Removed {} transport for device {}", channel, device_id);

            // Notify callback
            if let Some(ref callback) = self.callback {
                callback.on_transport_disconnected(device_id, channel);
            }

            // Remove device entry if no transports left
            if transports.is_empty() {
                connections.remove(device_id);
            }
        }
    }

    /// Remove all transports for a device
    pub async fn remove_device(&self, device_id: &str) {
        let mut connections = self.connections.write().await;
        if let Some(transports) = connections.remove(device_id) {
            for transport in transports {
                if let Some(ref callback) = self.callback {
                    callback.on_transport_disconnected(device_id, transport.channel());
                }
            }
            info!("Removed all transports for device {}", device_id);
        }
    }

    /// Get the best transport for a device
    ///
    /// Uses the channel selector to choose the best available transport.
    pub async fn get_best_transport(&self, device_id: &str) -> Result<Arc<dyn Transport>, TransportError> {
        let connections = self.connections.read().await;
        let transports = connections.get(device_id)
            .ok_or_else(|| TransportError::NotConnected(device_id.to_string()))?;

        if transports.is_empty() {
            return Err(TransportError::NotConnected(device_id.to_string()));
        }

        // Build channel info list
        let channel_infos: Vec<ChannelInfo> = transports.iter()
            .map(|t| ChannelInfo::new(
                t.channel(),
                if t.is_connected() {
                    ChannelStatus::Available
                } else {
                    ChannelStatus::Unavailable
                }
            ))
            .collect();

        // Select best channel
        let best_channel = self.channel_selector.select(&channel_infos)
            .ok_or_else(|| TransportError::NoAvailableChannel(device_id.to_string()))?;

        // Find the transport for that channel
        transports.iter()
            .find(|t| t.channel() == best_channel && t.is_connected())
            .cloned()
            .ok_or_else(|| TransportError::NoAvailableChannel(device_id.to_string()))
    }

    /// Get all transports for a device
    pub async fn get_transports(&self, device_id: &str) -> Vec<Arc<dyn Transport>> {
        let connections = self.connections.read().await;
        connections.get(device_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Send a message to a device (auto-selects best channel)
    pub async fn send_to_device(&self, device_id: &str, msg: &Message) -> Result<(), TransportError> {
        let transport = self.get_best_transport(device_id).await?;
        let result = transport.send(msg).await;

        // Handle failover if enabled
        if result.is_err() && self.config.failover_on_error {
            // Try other channels
            let transports = self.get_transports(device_id).await;
            for t in transports {
                if t.channel() != transport.channel() && t.is_connected() {
                    debug!("Attempting failover to {} for device {}", t.channel(), device_id);
                    if let Ok(()) = t.send(msg).await {
                        return Ok(());
                    }
                }
            }
        }

        result
    }

    /// Broadcast a message to all connected devices
    ///
    /// Returns a list of (device_id, result) pairs.
    pub async fn broadcast(&self, msg: &Message) -> Vec<(String, Result<(), TransportError>)> {
        // Collect device IDs first to avoid holding the lock during send
        let connections = self.connections.read().await;
        let device_ids: Vec<String> = connections.keys().cloned().collect();
        drop(connections);

        let mut results = Vec::new();
        for device_id in device_ids {
            let result = self.send_to_device(&device_id, msg).await;
            results.push((device_id, result));
        }

        results
    }

    /// Connect to a device using available connectors
    ///
    /// Tries connectors in priority order until one succeeds.
    pub async fn connect(&self, device_id: &str, address: &str) -> Result<(), TransportError> {
        let connectors = self.connectors.read().await;

        // Sort by channel priority (WiFi first)
        let mut sorted_connectors: Vec<_> = connectors.iter().cloned().collect();
        sorted_connectors.sort_by_key(|c| std::cmp::Reverse(c.channel().priority()));

        for connector in sorted_connectors {
            debug!("Trying {} connector for device {}", connector.channel(), device_id);
            match connector.connect(device_id, address).await {
                Ok(transport) => {
                    self.add_transport(device_id, transport).await;
                    return Ok(());
                }
                Err(e) => {
                    debug!("Connector {} failed: {}", connector.channel(), e);
                    continue;
                }
            }
        }

        Err(TransportError::ConnectionFailed(format!(
            "All connectors failed for device {}",
            device_id
        )))
    }

    /// Get list of connected device IDs
    pub async fn connected_devices(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.iter()
            .filter(|(_, transports)| transports.iter().any(|t| t.is_connected()))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get connection info for a device
    pub async fn device_channels(&self, device_id: &str) -> Vec<(Channel, bool)> {
        let connections = self.connections.read().await;
        connections.get(device_id)
            .map(|transports| {
                transports.iter()
                    .map(|t| (t.channel(), t.is_connected()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if a device is connected via any channel
    pub async fn is_device_connected(&self, device_id: &str) -> bool {
        let connections = self.connections.read().await;
        connections.get(device_id)
            .map(|transports| transports.iter().any(|t| t.is_connected()))
            .unwrap_or(false)
    }

    /// Check if a device is connected via a specific channel
    pub async fn is_device_connected_via(&self, device_id: &str, channel: Channel) -> bool {
        let connections = self.connections.read().await;
        connections.get(device_id)
            .map(|transports| {
                transports.iter()
                    .any(|t| t.channel() == channel && t.is_connected())
            })
            .unwrap_or(false)
    }

    /// Get total number of connected devices
    pub async fn connected_count(&self) -> usize {
        self.connected_devices().await.len()
    }

    /// Close all connections
    pub async fn close_all(&self) {
        let mut connections = self.connections.write().await;
        for (device_id, transports) in connections.drain() {
            for transport in transports {
                let _ = transport.close().await;
                if let Some(ref callback) = self.callback {
                    callback.on_transport_disconnected(&device_id, transport.channel());
                }
            }
        }
        info!("Closed all transport connections");
    }
}

impl Default for TransportManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{MockTransport, MockConfig};

    fn create_test_message(content: &str) -> Message {
        Message::clipboard_sync(content.as_bytes(), "test".to_string())
    }

    #[tokio::test]
    async fn test_add_transport() {
        let manager = TransportManager::new();
        let transport = Arc::new(MockTransport::with_defaults("device_1"));

        manager.add_transport("device_1", transport).await;

        assert!(manager.is_device_connected("device_1").await);
        assert_eq!(manager.connected_count().await, 1);
    }

    #[tokio::test]
    async fn test_remove_transport() {
        let manager = TransportManager::new();
        let transport = Arc::new(MockTransport::with_defaults("device_1"));

        manager.add_transport("device_1", transport).await;
        manager.remove_transport("device_1", Channel::Wifi).await;

        assert!(!manager.is_device_connected("device_1").await);
    }

    #[tokio::test]
    async fn test_send_to_device() {
        let manager = TransportManager::new();
        let transport = Arc::new(MockTransport::with_defaults("device_1"));
        let transport_clone = transport.clone();

        manager.add_transport("device_1", transport).await;

        let msg = create_test_message("hello");
        manager.send_to_device("device_1", &msg).await.unwrap();

        let sent = transport_clone.get_sent_messages().await;
        assert_eq!(sent.len(), 1);
    }

    #[tokio::test]
    async fn test_get_best_transport_wifi_priority() {
        let manager = TransportManager::new();

        // Add BLE transport first
        let ble_transport = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Ble)
        ));
        manager.add_transport("device_1", ble_transport).await;

        // Add WiFi transport
        let wifi_transport = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Wifi)
        ));
        manager.add_transport("device_1", wifi_transport).await;

        // Should select WiFi (higher priority)
        let best = manager.get_best_transport("device_1").await.unwrap();
        assert_eq!(best.channel(), Channel::Wifi);
    }

    #[tokio::test]
    async fn test_fallback_to_ble() {
        let manager = TransportManager::new();

        // Add disconnected WiFi transport
        let wifi_transport = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Wifi)
        ));
        wifi_transport.disconnect();
        manager.add_transport("device_1", wifi_transport).await;

        // Add connected BLE transport
        let ble_transport = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Ble)
        ));
        manager.add_transport("device_1", ble_transport).await;

        // Should select BLE (WiFi is disconnected)
        let best = manager.get_best_transport("device_1").await.unwrap();
        assert_eq!(best.channel(), Channel::Ble);
    }

    #[tokio::test]
    async fn test_broadcast() {
        let manager = TransportManager::new();

        let transport_1 = Arc::new(MockTransport::with_defaults("device_1"));
        let transport_2 = Arc::new(MockTransport::with_defaults("device_2"));

        manager.add_transport("device_1", transport_1.clone()).await;
        manager.add_transport("device_2", transport_2.clone()).await;

        let msg = create_test_message("broadcast");
        let results = manager.broadcast(&msg).await;

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|(_, r)| r.is_ok()));
    }

    #[tokio::test]
    async fn test_device_channels() {
        let manager = TransportManager::new();

        let wifi = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Wifi)
        ));
        let ble = Arc::new(MockTransport::new(
            "device_1",
            MockConfig::new().with_channel(Channel::Ble)
        ));

        manager.add_transport("device_1", wifi).await;
        manager.add_transport("device_1", ble).await;

        let channels = manager.device_channels("device_1").await;
        assert_eq!(channels.len(), 2);
        assert!(channels.iter().any(|(c, _)| *c == Channel::Wifi));
        assert!(channels.iter().any(|(c, _)| *c == Channel::Ble));
    }

    #[tokio::test]
    async fn test_close_all() {
        let manager = TransportManager::new();

        let transport_1 = Arc::new(MockTransport::with_defaults("device_1"));
        let transport_2 = Arc::new(MockTransport::with_defaults("device_2"));

        manager.add_transport("device_1", transport_1.clone()).await;
        manager.add_transport("device_2", transport_2.clone()).await;

        manager.close_all().await;

        assert_eq!(manager.connected_count().await, 0);
    }
}
