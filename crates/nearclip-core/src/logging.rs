//! NearClip 结构化日志系统
//!
//! 使用 tracing 提供统一的日志记录功能。
//!
//! # 日志级别规范
//!
//! | 级别 | 用途 |
//! |------|------|
//! | `error` | 需要用户干预的问题 |
//! | `warn` | 可恢复的问题 |
//! | `info` | 重要业务事件 |
//! | `debug` | 开发调试信息 |
//! | `trace` | 详细追踪 |

#[cfg(not(target_os = "android"))]
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[cfg(target_os = "android")]
use tracing_subscriber::{prelude::*, EnvFilter};

/// 日志级别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    /// 错误级别 - 需要用户干预的问题
    Error,
    /// 警告级别 - 可恢复的问题
    Warn,
    /// 信息级别 - 重要业务事件 (默认)
    #[default]
    Info,
    /// 调试级别 - 开发调试信息
    Debug,
    /// 追踪级别 - 详细追踪
    Trace,
}

impl LogLevel {
    /// 将日志级别转换为 tracing filter 字符串
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_core::LogLevel;
    /// assert_eq!(LogLevel::Info.as_str(), "info");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
    }
}

/// 初始化日志系统
///
/// 应在应用启动时调用一次。多次调用会被忽略（安全）。
///
/// 日志输出包含：
/// - 时间戳
/// - 日志级别
/// - 模块来源 (target)
/// - 行号
///
/// 可通过环境变量 `RUST_LOG` 覆盖日志级别。
///
/// # Arguments
///
/// * `level` - 最低日志级别
///
/// # Example
///
/// ```
/// use nearclip_core::logging::{init_logging, LogLevel};
/// init_logging(LogLevel::Info);
/// ```
#[cfg(not(target_os = "android"))]
pub fn init_logging(level: LogLevel) {
    // 支持 RUST_LOG 环境变量覆盖
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.as_str()));

    // 配置格式化输出
    let subscriber = fmt::layer()
        .with_target(true)          // 显示模块来源
        .with_line_number(true)     // 显示行号
        .with_thread_ids(false)     // 不显示线程 ID
        .with_thread_names(false);  // 不显示线程名

    // 初始化全局订阅者（多次调用安全，会被忽略）
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(subscriber)
        .try_init();
}

/// 初始化日志系统（Android 版本）
///
/// 在 Android 平台上使用 tracing-android 输出到 logcat。
#[cfg(target_os = "android")]
pub fn init_logging(level: LogLevel) {
    // 支持 RUST_LOG 环境变量覆盖
    // 默认配置：减少 mDNS 库的日志噪音
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "{},mdns_sd=warn,mdns_sd::dns_parser=warn,mdns_sd::service_daemon=warn",
            level.as_str()
        ))
    });

    // 使用 tracing-android 输出到 logcat
    let android_layer = match tracing_android::layer("NearClip") {
        Ok(layer) => layer,
        Err(_) => return, // 无法初始化 Android 日志，静默失败
    };

    // 初始化全局订阅者（多次调用安全，会被忽略）
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(android_layer)
        .try_init();
}

/// 刷新日志缓冲区
///
/// 确保所有待处理的日志都被写入输出。
/// 在应用退出前或关键操作后调用。
///
/// # Example
///
/// ```
/// use nearclip_core::logging::{init_logging, flush_logs, LogLevel};
/// init_logging(LogLevel::Info);
/// // ... 执行一些操作 ...
/// flush_logs();
/// ```
pub fn flush_logs() {
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, error, info, trace, warn};

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert_eq!(LogLevel::Warn.as_str(), "warn");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Trace.as_str(), "trace");
    }

    #[test]
    fn test_init_logging_no_panic() {
        // 初始化不应 panic
        init_logging(LogLevel::Debug);
    }

    #[test]
    fn test_init_logging_multiple_calls_safe() {
        // 多次调用应该是安全的
        init_logging(LogLevel::Info);
        init_logging(LogLevel::Debug);
        init_logging(LogLevel::Trace);
    }

    #[test]
    fn test_log_macros_compile() {
        init_logging(LogLevel::Trace);
        error!("test error");
        warn!("test warn");
        info!("test info");
        debug!("test debug");
        trace!("test trace");
    }

    #[test]
    fn test_log_level_clone() {
        let level = LogLevel::Info;
        let cloned = level;
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_log_level_copy() {
        let level = LogLevel::Debug;
        let copied: LogLevel = level; // Copy
        assert_eq!(level, copied);
    }

    #[test]
    fn test_log_level_debug_format() {
        let level = LogLevel::Warn;
        let debug_str = format!("{:?}", level);
        assert_eq!(debug_str, "Warn");
    }

    #[test]
    fn test_structured_logging() {
        init_logging(LogLevel::Debug);
        let device_id = "device-123";
        let content_len = 256;
        info!(device_id, content_len, "Test structured log");
    }

    #[test]
    fn test_all_log_levels_exist() {
        // 确保所有五个级别都已定义
        let levels = [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ];
        assert_eq!(levels.len(), 5);
    }

    #[test]
    fn test_log_level_default() {
        let default_level = LogLevel::default();
        assert_eq!(default_level, LogLevel::Info);
    }

    #[test]
    fn test_flush_logs_no_panic() {
        init_logging(LogLevel::Debug);
        info!("test message before flush");
        flush_logs();
    }

    #[test]
    fn test_as_str_is_public() {
        // 验证 as_str() 是公开的，可以从外部访问
        let level = LogLevel::Debug;
        let level_str = level.as_str();
        assert_eq!(level_str, "debug");
    }
}
