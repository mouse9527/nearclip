//! NearClip 统一错误类型
//!
//! 所有模块使用此错误类型，确保错误处理一致性。

use thiserror::Error;

/// NearClip 统一错误类型
#[derive(Debug, Clone, PartialEq, Error)]
pub enum NearClipError {
    /// 网络相关错误 (TCP, mDNS)
    #[error("Network error: {0}")]
    Network(String),

    /// 蓝牙相关错误 (BLE)
    #[error("Bluetooth error: {0}")]
    Bluetooth(String),

    /// 加密相关错误 (TLS, ECDH)
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// 设备未找到错误
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// 同步协议错误
    #[error("Sync error: {0}")]
    Sync(String),

    /// 配置错误
    #[error("Config error: {0}")]
    Config(String),

    /// IO 错误
    #[error("IO error: {0}")]
    Io(String),

    /// 未初始化错误
    #[error("Not initialized: {0}")]
    NotInitialized(String),
}

/// NearClip Result 类型别名
pub type Result<T> = std::result::Result<T, NearClipError>;

impl From<std::io::Error> for NearClipError {
    fn from(err: std::io::Error) -> Self {
        NearClipError::Io(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_error() {
        let err = NearClipError::Network("connection refused".to_string());
        assert_eq!(err.to_string(), "Network error: connection refused");
    }

    #[test]
    fn test_bluetooth_error() {
        let err = NearClipError::Bluetooth("device not available".to_string());
        assert_eq!(err.to_string(), "Bluetooth error: device not available");
    }

    #[test]
    fn test_crypto_error() {
        let err = NearClipError::Crypto("invalid key".to_string());
        assert_eq!(err.to_string(), "Crypto error: invalid key");
    }

    #[test]
    fn test_device_not_found() {
        let err = NearClipError::DeviceNotFound("abc123".to_string());
        assert_eq!(err.to_string(), "Device not found: abc123");
    }

    #[test]
    fn test_sync_error() {
        let err = NearClipError::Sync("protocol mismatch".to_string());
        assert_eq!(err.to_string(), "Sync error: protocol mismatch");
    }

    #[test]
    fn test_config_error() {
        let err = NearClipError::Config("invalid config".to_string());
        assert_eq!(err.to_string(), "Config error: invalid config");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: NearClipError = io_err.into();
        assert!(matches!(err, NearClipError::Io(ref s) if s.contains("file not found")));
    }

    #[test]
    fn test_io_error_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: NearClipError = io_err.into();
        assert!(err.to_string().contains("IO error"));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_clone() {
        let err = NearClipError::Network("test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let err1 = NearClipError::Crypto("key error".to_string());
        let err2 = NearClipError::Crypto("key error".to_string());
        let err3 = NearClipError::Crypto("different".to_string());
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_result_type_ok() {
        fn example_fn() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(example_fn().unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        fn example_fn() -> Result<i32> {
            Err(NearClipError::Network("test".to_string()))
        }
        assert!(example_fn().is_err());
    }

    #[test]
    fn test_error_debug_impl() {
        let err = NearClipError::Network("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Network"));
    }

    #[test]
    fn test_question_mark_operator() {
        fn inner() -> std::result::Result<(), std::io::Error> {
            Err(std::io::Error::other("inner error"))
        }

        fn outer() -> Result<()> {
            inner()?;
            Ok(())
        }

        assert!(outer().is_err());
    }
}
