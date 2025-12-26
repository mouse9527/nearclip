//! Device management error types

use thiserror::Error;

/// Errors that can occur during device management operations
#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Device not found: {0}")]
    NotFound(String),

    #[error("Device already paired: {0}")]
    AlreadyPaired(String),

    #[error("Invalid device data: {0}")]
    InvalidData(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DeviceError::NotFound("test-device".to_string());
        assert_eq!(err.to_string(), "Device not found: test-device");
    }

    #[test]
    fn test_error_from_sqlite() {
        let sqlite_err = rusqlite::Error::InvalidQuery;
        let device_err: DeviceError = sqlite_err.into();
        assert!(matches!(device_err, DeviceError::Database(_)));
    }
}
