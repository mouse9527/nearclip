//! BLE 错误类型
//!
//! 定义蓝牙低功耗模块的错误类型。

use thiserror::Error;

/// BLE 错误类型
#[derive(Debug, Clone, Error)]
pub enum BleError {
    /// BLE 初始化失败
    #[error("BLE initialization failed: {0}")]
    Initialization(String),

    /// BLE 未开启
    #[error("BLE not powered on")]
    NotPowered,

    /// 广播失败
    #[error("Advertising failed: {0}")]
    Advertising(String),

    /// 服务注册失败
    #[error("Service registration failed: {0}")]
    ServiceRegistration(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// 平台不支持
    #[error("Platform not supported")]
    PlatformNotSupported,

    /// 数据分片/重组错误
    #[error("Chunk error: {0}")]
    ChunkError(String),

    /// 数据传输错误
    #[error("Data transfer error: {0}")]
    DataTransfer(String),

    /// 连接失败
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// 操作超时
    #[error("Timeout: {0}")]
    Timeout(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ble_error_display() {
        let err = BleError::Initialization("test error".to_string());
        assert_eq!(err.to_string(), "BLE initialization failed: test error");
    }

    #[test]
    fn test_ble_error_not_powered() {
        let err = BleError::NotPowered;
        assert_eq!(err.to_string(), "BLE not powered on");
    }

    #[test]
    fn test_ble_error_advertising() {
        let err = BleError::Advertising("failed to start".to_string());
        assert_eq!(err.to_string(), "Advertising failed: failed to start");
    }

    #[test]
    fn test_ble_error_service_registration() {
        let err = BleError::ServiceRegistration("invalid uuid".to_string());
        assert_eq!(err.to_string(), "Service registration failed: invalid uuid");
    }

    #[test]
    fn test_ble_error_configuration() {
        let err = BleError::Configuration("empty device id".to_string());
        assert_eq!(err.to_string(), "Configuration error: empty device id");
    }

    #[test]
    fn test_ble_error_platform_not_supported() {
        let err = BleError::PlatformNotSupported;
        assert_eq!(err.to_string(), "Platform not supported");
    }

    #[test]
    fn test_ble_error_chunk_error() {
        let err = BleError::ChunkError("invalid header".to_string());
        assert_eq!(err.to_string(), "Chunk error: invalid header");
    }

    #[test]
    fn test_ble_error_data_transfer() {
        let err = BleError::DataTransfer("write failed".to_string());
        assert_eq!(err.to_string(), "Data transfer error: write failed");
    }

    #[test]
    fn test_ble_error_connection_failed() {
        let err = BleError::ConnectionFailed("device not found".to_string());
        assert_eq!(err.to_string(), "Connection failed: device not found");
    }

    #[test]
    fn test_ble_error_timeout() {
        let err = BleError::Timeout("operation timed out after 30s".to_string());
        assert_eq!(err.to_string(), "Timeout: operation timed out after 30s");
    }
}
