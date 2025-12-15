//! NearClip Core Module
//!
//! Core coordination layer providing the main API for platform clients.
//! Orchestrates crypto, network, BLE, and sync modules.
//!
//! # Overview
//!
//! `nearclip-core` 是 NearClip 的核心协调层，提供统一的 API 供平台客户端使用。
//! 它协调底层的 net、ble、crypto、sync 模块，提供简洁的高层接口。
//!
//! # 主要组件
//!
//! - [`NearClipManager`] - 核心管理器，提供 start/stop/sync 接口
//! - [`NearClipConfig`] - 配置结构
//! - [`NearClipCallback`] - 回调接口
//! - [`DeviceInfo`] - 设备信息
//!
//! # 示例
//!
//! ```
//! use nearclip_core::{
//!     NearClipManager, NearClipConfig, NoOpCallback,
//!     DeviceInfo, DevicePlatform,
//! };
//! use std::sync::Arc;
//!
//! // 创建配置
//! let config = NearClipConfig::new("My MacBook")
//!     .with_wifi_enabled(true)
//!     .with_ble_enabled(true);
//!
//! // 创建管理器
//! let callback = Arc::new(NoOpCallback);
//! let manager = NearClipManager::new(config, callback).unwrap();
//!
//! // 添加设备
//! let device = DeviceInfo::new("device-1", "iPhone")
//!     .with_platform(DevicePlatform::Unknown);
//! manager.add_paired_device(device);
//! ```

pub mod config;
pub mod device;
pub mod error;
pub mod logging;
pub mod manager;

// Re-export error types for convenience
pub use error::{NearClipError, Result};

// Re-export logging
pub use logging::{flush_logs, init_logging, LogLevel};

// Re-export device types
pub use device::{DeviceInfo, DevicePlatform, DeviceStatus};

// Re-export config types
pub use config::{
    NearClipConfig, DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_DEVICE_NAME,
    DEFAULT_HEARTBEAT_INTERVAL_SECS, DEFAULT_MAX_RETRIES,
};

// Re-export manager types
pub use manager::{NearClipCallback, NearClipManager, NoOpCallback};

// Re-export tracing macros for convenience
// 包含 instrument 宏用于函数级追踪
pub use tracing::{debug, error, info, instrument, trace, warn};

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
