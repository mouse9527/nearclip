//! 网络层错误类型定义
//!
//! 提供统一的网络模块错误处理。

use thiserror::Error;

/// 网络层错误类型
#[derive(Debug, Error)]
pub enum NetError {
    /// mDNS 操作错误
    #[error("mDNS error: {0}")]
    Mdns(String),

    /// 服务注册失败
    #[error("Service registration failed: {0}")]
    ServiceRegistration(String),

    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Configuration(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdns_error_display() {
        let error = NetError::Mdns("daemon failed".to_string());
        assert_eq!(error.to_string(), "mDNS error: daemon failed");
    }

    #[test]
    fn test_service_registration_error_display() {
        let error = NetError::ServiceRegistration("invalid name".to_string());
        assert_eq!(
            error.to_string(),
            "Service registration failed: invalid name"
        );
    }

    #[test]
    fn test_configuration_error_display() {
        let error = NetError::Configuration("missing port".to_string());
        assert_eq!(error.to_string(), "Configuration error: missing port");
    }
}
