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

    /// TCP 服务端错误
    #[error("TCP server error: {0}")]
    TcpServer(String),

    /// TLS 握手错误
    #[error("TLS handshake error: {0}")]
    TlsHandshake(String),

    /// 连接已关闭
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    /// 连接失败
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// 连接超时
    #[error("Connection timeout: {0}")]
    ConnectionTimeout(String),
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

    #[test]
    fn test_tcp_server_error_display() {
        let error = NetError::TcpServer("bind failed".to_string());
        assert_eq!(error.to_string(), "TCP server error: bind failed");
    }

    #[test]
    fn test_tls_handshake_error_display() {
        let error = NetError::TlsHandshake("certificate invalid".to_string());
        assert_eq!(error.to_string(), "TLS handshake error: certificate invalid");
    }

    #[test]
    fn test_connection_closed_error_display() {
        let error = NetError::ConnectionClosed("peer disconnected".to_string());
        assert_eq!(error.to_string(), "Connection closed: peer disconnected");
    }

    #[test]
    fn test_connection_failed_error_display() {
        let error = NetError::ConnectionFailed("refused".to_string());
        assert_eq!(error.to_string(), "Connection failed: refused");
    }

    #[test]
    fn test_connection_timeout_error_display() {
        let error = NetError::ConnectionTimeout("10s elapsed".to_string());
        assert_eq!(error.to_string(), "Connection timeout: 10s elapsed");
    }
}
