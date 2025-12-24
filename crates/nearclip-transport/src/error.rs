//! Transport error types

use std::io;
use thiserror::Error;

/// Errors that can occur during transport operations
#[derive(Error, Debug)]
pub enum TransportError {
    /// Device is not connected
    #[error("device not connected: {0}")]
    NotConnected(String),

    /// No available transport channel
    #[error("no available transport channel for device: {0}")]
    NoAvailableChannel(String),

    /// Connection failed
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// Send failed
    #[error("send failed: {0}")]
    SendFailed(String),

    /// Receive failed
    #[error("receive failed: {0}")]
    ReceiveFailed(String),

    /// Connection closed
    #[error("connection closed")]
    ConnectionClosed,

    /// Timeout
    #[error("operation timed out")]
    Timeout,

    /// Message serialization error
    #[error("serialization error: {0}")]
    Serialization(String),

    /// Message deserialization error
    #[error("deserialization error: {0}")]
    Deserialization(String),

    /// IO error
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// Network error (from nearclip-net)
    #[error("network error: {0}")]
    Network(String),

    /// BLE error
    #[error("ble error: {0}")]
    Ble(String),

    /// Invalid state
    #[error("invalid state: {0}")]
    InvalidState(String),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl TransportError {
    /// Check if this error indicates the connection is closed
    pub fn is_connection_closed(&self) -> bool {
        matches!(self, TransportError::ConnectionClosed)
    }

    /// Check if this error is recoverable (can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            TransportError::Timeout | TransportError::SendFailed(_)
        )
    }
}

impl From<nearclip_net::NetError> for TransportError {
    fn from(err: nearclip_net::NetError) -> Self {
        TransportError::Network(err.to_string())
    }
}

impl From<nearclip_sync::SyncError> for TransportError {
    fn from(err: nearclip_sync::SyncError) -> Self {
        TransportError::Serialization(err.to_string())
    }
}

impl Clone for TransportError {
    fn clone(&self) -> Self {
        match self {
            TransportError::NotConnected(s) => TransportError::NotConnected(s.clone()),
            TransportError::NoAvailableChannel(s) => TransportError::NoAvailableChannel(s.clone()),
            TransportError::ConnectionFailed(s) => TransportError::ConnectionFailed(s.clone()),
            TransportError::SendFailed(s) => TransportError::SendFailed(s.clone()),
            TransportError::ReceiveFailed(s) => TransportError::ReceiveFailed(s.clone()),
            TransportError::ConnectionClosed => TransportError::ConnectionClosed,
            TransportError::Timeout => TransportError::Timeout,
            TransportError::Serialization(s) => TransportError::Serialization(s.clone()),
            TransportError::Deserialization(s) => TransportError::Deserialization(s.clone()),
            TransportError::Io(e) => TransportError::Io(io::Error::new(e.kind(), e.to_string())),
            TransportError::Network(s) => TransportError::Network(s.clone()),
            TransportError::Ble(s) => TransportError::Ble(s.clone()),
            TransportError::InvalidState(s) => TransportError::InvalidState(s.clone()),
            TransportError::Other(s) => TransportError::Other(s.clone()),
        }
    }
}
