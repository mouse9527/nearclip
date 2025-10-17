//! Error types for NearClip core library

use thiserror::Error;

/// NearClip core error types
#[derive(Error, Debug)]
pub enum NearclipError {
    /// Device discovery errors
    #[error("Device discovery failed: {message}")]
    DiscoveryFailed { message: String },

    /// Bluetooth not available
    #[error("Bluetooth is not available on this device")]
    BluetoothUnavailable,

    /// Permission denied
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    /// Connection errors
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },

    /// Connection timeout
    #[error("Connection timed out after {seconds}s")]
    ConnectionTimeout { seconds: u64 },

    /// Authentication failed
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },

    /// Sync errors
    #[error("Synchronization failed: {message}")]
    SyncFailed { message: String },

    /// Data corruption
    #[error("Data corruption detected: {message}")]
    DataCorruption { message: String },

    /// Encryption/decryption errors
    #[error("Cryptography error: {message}")]
    CryptoError { message: String },

    /// Validation errors
    #[error("Validation failed: {message}")]
    ValidationError { message: String },

    /// Internal errors
    #[error("Internal error: {message}")]
    InternalError { message: String },

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Protocol buffer errors
    #[error("Protocol buffer error: {0}")]
    ProtocolError(#[from] prost::DecodeError),

    /// JSON errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type alias for NearClip operations
pub type Result<T> = std::result::Result<T, NearclipError>;