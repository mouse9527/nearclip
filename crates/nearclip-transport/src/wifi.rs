//! WiFi transport implementation using TCP/TLS

use async_trait::async_trait;
use nearclip_net::tcp::{TcpReadHalf, TcpWriteHalf, TcpConnection, TcpServer, TcpClient, TcpClientConfig};
use nearclip_sync::{Channel, Message};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, warn, instrument};

use crate::error::TransportError;
use crate::traits::{Transport, TransportConnector, TransportListener};

/// Maximum message size (16 MB)
const MAX_MESSAGE_SIZE: u32 = 16 * 1024 * 1024;

/// WiFi transport using TCP/TLS
///
/// Wraps a TLS-encrypted TCP connection and implements the Transport trait.
pub struct WifiTransport {
    device_id: String,
    writer: Arc<Mutex<TcpWriteHalf>>,
    reader: Arc<Mutex<TcpReadHalf>>,
    connected: AtomicBool,
}

impl WifiTransport {
    /// Create a new WiFi transport from a TCP connection
    ///
    /// # Arguments
    /// * `device_id` - The peer device ID
    /// * `connection` - The TCP connection to wrap
    pub fn new(device_id: String, connection: TcpConnection) -> Self {
        let (reader, writer) = connection.into_split();
        Self {
            device_id,
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
            connected: AtomicBool::new(true),
        }
    }

    /// Create from pre-split connection halves
    pub fn from_split(device_id: String, reader: TcpReadHalf, writer: TcpWriteHalf) -> Self {
        Self {
            device_id,
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
            connected: AtomicBool::new(true),
        }
    }

    /// Get the writer half (for external use if needed)
    pub fn writer(&self) -> Arc<Mutex<TcpWriteHalf>> {
        self.writer.clone()
    }

    /// Get the reader half (for external use if needed)
    pub fn reader(&self) -> Arc<Mutex<TcpReadHalf>> {
        self.reader.clone()
    }
}

#[async_trait]
impl Transport for WifiTransport {
    #[instrument(skip(self, msg), fields(device_id = %self.device_id, msg_type = ?msg.msg_type))]
    async fn send(&self, msg: &Message) -> Result<(), TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        let data = msg.serialize()
            .map_err(|e| TransportError::Serialization(e.to_string()))?;

        let mut writer = self.writer.lock().await;

        // Write length prefix (4 bytes, big-endian)
        let len_bytes = (data.len() as u32).to_be_bytes();
        writer.write_all(&len_bytes).await
            .map_err(|e| {
                self.connected.store(false, Ordering::SeqCst);
                TransportError::SendFailed(e.to_string())
            })?;

        // Write message data
        writer.write_all(&data).await
            .map_err(|e| {
                self.connected.store(false, Ordering::SeqCst);
                TransportError::SendFailed(e.to_string())
            })?;

        // Flush
        writer.flush().await
            .map_err(|e| {
                self.connected.store(false, Ordering::SeqCst);
                TransportError::SendFailed(e.to_string())
            })?;

        debug!("Sent message ({} bytes) to {}", data.len(), self.device_id);
        Ok(())
    }

    #[instrument(skip(self), fields(device_id = %self.device_id))]
    async fn recv(&self) -> Result<Message, TransportError> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(TransportError::ConnectionClosed);
        }

        let mut reader = self.reader.lock().await;

        // Read length prefix (4 bytes)
        let mut len_buf = [0u8; 4];
        let mut total_read = 0;
        while total_read < 4 {
            let n = reader.read(&mut len_buf[total_read..]).await
                .map_err(|e| {
                    self.connected.store(false, Ordering::SeqCst);
                    TransportError::ReceiveFailed(e.to_string())
                })?;
            if n == 0 {
                self.connected.store(false, Ordering::SeqCst);
                return Err(TransportError::ConnectionClosed);
            }
            total_read += n;
        }

        let msg_len = u32::from_be_bytes(len_buf);
        if msg_len > MAX_MESSAGE_SIZE {
            warn!("Message too large: {} bytes", msg_len);
            return Err(TransportError::ReceiveFailed(format!(
                "Message too large: {} bytes (max {})",
                msg_len, MAX_MESSAGE_SIZE
            )));
        }

        // Read message data
        let mut data = vec![0u8; msg_len as usize];
        let mut total_read = 0;
        while total_read < msg_len as usize {
            let n = reader.read(&mut data[total_read..]).await
                .map_err(|e| {
                    self.connected.store(false, Ordering::SeqCst);
                    TransportError::ReceiveFailed(e.to_string())
                })?;
            if n == 0 {
                self.connected.store(false, Ordering::SeqCst);
                return Err(TransportError::ConnectionClosed);
            }
            total_read += n;
        }

        // Deserialize
        let msg = Message::deserialize(&data)
            .map_err(|e| TransportError::Deserialization(e.to_string()))?;

        debug!("Received message ({} bytes) from {}", data.len(), self.device_id);
        Ok(msg)
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn channel(&self) -> Channel {
        Channel::Wifi
    }

    fn peer_device_id(&self) -> &str {
        &self.device_id
    }

    async fn close(&self) -> Result<(), TransportError> {
        self.connected.store(false, Ordering::SeqCst);
        debug!("WiFi transport closed for device {}", self.device_id);
        Ok(())
    }
}

/// WiFi transport connector
///
/// Creates outbound WiFi connections to remote devices.
pub struct WifiTransportConnector {
    tls_config: Arc<rustls::ClientConfig>,
}

impl WifiTransportConnector {
    /// Create a new WiFi transport connector
    pub fn new(tls_config: Arc<rustls::ClientConfig>) -> Self {
        Self { tls_config }
    }
}

#[async_trait]
impl TransportConnector for WifiTransportConnector {
    async fn connect(
        &self,
        device_id: &str,
        address: &str,
    ) -> Result<Arc<dyn Transport>, TransportError> {
        // Parse address (expected format: "host:port")
        let addr: std::net::SocketAddr = address.parse()
            .map_err(|e| TransportError::ConnectionFailed(format!("Invalid address: {}", e)))?;

        let config = TcpClientConfig::new(addr);
        let connection = TcpClient::connect(config, self.tls_config.clone(), "nearclip.local").await
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        let transport = WifiTransport::new(device_id.to_string(), connection);
        Ok(Arc::new(transport))
    }

    fn channel(&self) -> Channel {
        Channel::Wifi
    }
}

/// WiFi transport listener
///
/// Accepts inbound WiFi connections from remote devices.
pub struct WifiTransportListener {
    server: Arc<TcpServer>,
    local_address: String,
}

impl WifiTransportListener {
    /// Create a new WiFi transport listener
    pub fn new(server: TcpServer) -> Self {
        let local_address = server.local_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        Self {
            server: Arc::new(server),
            local_address,
        }
    }

    /// Get the port this listener is bound to
    pub fn port(&self) -> u16 {
        self.server.local_addr()
            .map(|addr| addr.port())
            .unwrap_or(0)
    }
}

#[async_trait]
impl TransportListener for WifiTransportListener {
    async fn accept(&self) -> Result<Arc<dyn Transport>, TransportError> {
        let connection = self.server.accept().await
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        // Note: device_id will be set after receiving the first message (pairing request)
        // For now, use the peer address as a temporary identifier
        let peer_addr = connection.peer_addr().to_string();
        let transport = WifiTransport::new(peer_addr, connection);
        Ok(Arc::new(transport))
    }

    fn channel(&self) -> Channel {
        Channel::Wifi
    }

    fn local_address(&self) -> String {
        self.local_address.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_message_size() {
        assert_eq!(MAX_MESSAGE_SIZE, 16 * 1024 * 1024);
    }
}
