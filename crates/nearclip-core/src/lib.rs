//! NearClip Core Module
//!
//! Core coordination layer providing the main API for platform clients.
//! Orchestrates crypto, network, BLE, and sync modules.

pub mod error;
pub mod logging;

// Re-export error types for convenience
pub use error::{NearClipError, Result};

// Re-export logging
pub use logging::{flush_logs, init_logging, LogLevel};

// Re-export tracing macros for convenience
// 包含 instrument 宏用于函数级追踪
pub use tracing::{debug, error, info, instrument, trace, warn};

// Future modules:
// mod manager;   // NearClipManager main class
// mod device;    // Device management
// mod config;    // Configuration management

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_reexport() {
        let err = NearClipError::Network("test".to_string());
        assert!(err.to_string().contains("Network"));
    }

    #[test]
    fn test_result_reexport() {
        fn test_fn() -> Result<i32> {
            Ok(42)
        }
        assert!(test_fn().is_ok());
    }

    #[test]
    fn test_logging_reexport() {
        // 验证 LogLevel 和 init_logging 可以从 crate 根导入
        let level = LogLevel::Info;
        assert_eq!(level, LogLevel::Info);
        init_logging(LogLevel::Debug);
    }

    #[test]
    fn test_tracing_macros_reexport() {
        // 验证 tracing 宏可以从 crate 根导入使用
        init_logging(LogLevel::Trace);
        error!("test error from lib");
        warn!("test warn from lib");
        info!("test info from lib");
        debug!("test debug from lib");
        trace!("test trace from lib");
    }
}
