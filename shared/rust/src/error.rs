use thiserror::Error;

/// NearClip 统一错误类型
#[derive(Error, Debug)]
pub enum NearClipError {
    #[error("加密操作失败: {0}")]
    CryptoError(String),

    #[error("BLE 操作失败: {0}")]
    BLEError(String),

    #[error("设备连接失败: {0}")]
    DeviceError(String),

    #[error("同步操作失败: {0}")]
    SyncError(String),

    #[error("序列化失败: {0}")]
    SerializationError(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("操作超时")]
    Timeout,

    #[error("参数无效: {0}")]
    InvalidParameter(String),

    #[error("内部错误: {0}")]
    InternalError(String),
}

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, NearClipError>;

impl From<ring::error::Unspecified> for NearClipError {
    fn from(err: ring::error::Unspecified) -> Self {
        NearClipError::CryptoError(format!("Ring crypto error: {:?}", err))
    }
}

impl From<prost::DecodeError> for NearClipError {
    fn from(err: prost::DecodeError) -> Self {
        NearClipError::SerializationError(format!("Protobuf decode error: {}", err))
    }
}

impl From<prost::EncodeError> for NearClipError {
    fn from(err: prost::EncodeError) -> Self {
        NearClipError::SerializationError(format!("Protobuf encode error: {}", err))
    }
}

/// 错误码定义
pub mod error_codes {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = -1;
    pub const CRYPTO_ERROR: i32 = -2;
    pub const BLE_ERROR: i32 = -3;
    pub const DEVICE_ERROR: i32 = -4;
    pub const SYNC_ERROR: i32 = -5;
    pub const TIMEOUT_ERROR: i32 = -6;
    pub const INVALID_PARAMETER: i32 = -7;
}

/// 从错误码转换为错误消息
pub fn error_message_from_code(code: i32) -> &'static str {
    match code {
        error_codes::SUCCESS => "操作成功",
        error_codes::GENERAL_ERROR => "通用错误",
        error_codes::CRYPTO_ERROR => "加密操作失败",
        error_codes::BLE_ERROR => "BLE 操作失败",
        error_codes::DEVICE_ERROR => "设备操作失败",
        error_codes::SYNC_ERROR => "同步操作失败",
        error_codes::TIMEOUT_ERROR => "操作超时",
        error_codes::INVALID_PARAMETER => "参数无效",
        _ => "未知错误",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let nearclip_err: NearClipError = io_err.into();
        assert!(matches!(nearclip_err, NearClipError::IoError(_)));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(error_message_from_code(0), "操作成功");
        assert_eq!(error_message_from_code(-2), "加密操作失败");
        assert_eq!(error_message_from_code(999), "未知错误");
    }
}