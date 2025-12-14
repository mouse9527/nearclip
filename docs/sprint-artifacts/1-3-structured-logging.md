# Story 1.3: 配置结构化日志系统

Status: Done

## Story

As a 开发者,
I want 使用 tracing 进行结构化日志记录,
So that 可以方便地调试和追踪问题.

## Acceptance Criteria

1. **Given** nearclip-core crate 已创建 **When** 集成 tracing 日志框架 **Then** 支持 error/warn/info/debug/trace 五个级别
2. **And** 日志包含时间戳和模块来源
3. **And** 提供初始化函数供各平台调用
4. **And** 测试验证日志输出格式正确

## Tasks / Subtasks

- [x] Task 1: 创建 logging.rs 模块 (AC: 1, 3) ✅
  - [x] 1.1 在 `crates/nearclip-core/src/` 创建 `logging.rs`
  - [x] 1.2 添加 `tracing` 和 `tracing-subscriber` 依赖到 nearclip-core
  - [x] 1.3 定义 `LogLevel` 枚举 (Error, Warn, Info, Debug, Trace)
  - [x] 1.4 实现 `init_logging(level: LogLevel)` 初始化函数

- [x] Task 2: 配置 tracing-subscriber (AC: 1, 2) ✅
  - [x] 2.1 使用 `fmt::Subscriber` 配置格式化输出
  - [x] 2.2 启用时间戳 (`.with_timer()`)
  - [x] 2.3 启用模块来源 (`.with_target(true)`)
  - [x] 2.4 启用行号 (`.with_line_number(true)`)
  - [x] 2.5 配置日志级别过滤

- [x] Task 3: 导出日志模块 (AC: 3) ✅
  - [x] 3.1 在 `lib.rs` 中添加 `pub mod logging;`
  - [x] 3.2 重新导出 `init_logging` 和 `LogLevel`
  - [x] 3.3 重新导出 tracing 宏 (info!, warn!, error!, debug!, trace!)

- [x] Task 4: 编写单元测试 (AC: 4) ✅
  - [x] 4.1 测试日志初始化不会 panic
  - [x] 4.2 测试各日志级别可调用
  - [x] 4.3 测试 LogLevel 枚举正确转换

- [x] Task 5: 验证构建 (AC: 1, 2, 3, 4) ✅
  - [x] 5.1 运行 `cargo build` 确保无错误
  - [x] 5.2 运行 `cargo test` 确保测试通过

## Dev Notes

### 架构约束 (来自 architecture.md)

**日志框架配置（强制遵循）：**
```rust
// 使用 tracing，统一格式
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(content))]
pub async fn sync_clipboard(device_id: &str, content: &str) -> Result<()> {
    info!(device_id, content_len = content.len(), "Syncing clipboard");
    // ...
}
```

**日志级别规范（来自 project_context.md）：**
| 级别 | 用途 | 示例 |
|------|------|------|
| `error` | 需要用户干预的问题 | 配对失败、网络不可用 |
| `warn` | 可恢复的问题 | 重试中、通道切换 |
| `info` | 重要业务事件 | 配对成功、同步完成 |
| `debug` | 开发调试信息 | 函数入口、状态变化 |
| `trace` | 详细追踪 | 网络包、BLE 帧 |

**日志规范：**
- ✅ 使用 `#[instrument]` 自动记录函数调用
- ✅ 敏感数据使用 `skip()` 排除
- ❌ 禁止使用 `println!()` 替代 tracing
- ❌ 禁止在日志中记录剪贴板明文内容

### 文件位置

**目标文件：**
- `crates/nearclip-core/src/logging.rs` - 日志模块
- `crates/nearclip-core/src/lib.rs` - 模块导出
- `crates/nearclip-core/Cargo.toml` - 添加依赖

### 技术栈版本

| 依赖 | 版本 | 用途 |
|------|------|------|
| tracing | 0.1.x (workspace) | 结构化日志宏 |
| tracing-subscriber | 0.3.x (workspace) | 日志订阅者/格式化 |

**注意：** tracing 和 tracing-subscriber 已在 Story 1-1 中配置为 workspace 依赖，无需再添加版本号。

### 从前一个 Story 继承的上下文

**Story 1-1 完成的基础设施：**
- ✅ nearclip-core crate 已创建
- ✅ workspace 配置了 tracing = "0.1" 和 tracing-subscriber = "0.3"
- ✅ Cargo.toml 使用 workspace 依赖继承

**Story 1-2 完成的错误处理：**
- ✅ NearClipError 统一错误类型已定义
- ✅ Result<T> 类型别名已导出
- ✅ 日志模块可以使用 Result 返回初始化错误

**crate 间依赖关系：**
```
nearclip-ffi → nearclip-core → nearclip-sync/net/ble → nearclip-crypto
```
日志初始化在 nearclip-core 中定义，其他 crate 直接使用 tracing 宏。

### 代码模板

**logging.rs 完整模板：**
```rust
//! NearClip 结构化日志系统
//!
//! 使用 tracing 提供统一的日志记录功能。

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// 日志级别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// 错误级别 - 需要用户干预的问题
    Error,
    /// 警告级别 - 可恢复的问题
    Warn,
    /// 信息级别 - 重要业务事件
    Info,
    /// 调试级别 - 开发调试信息
    Debug,
    /// 追踪级别 - 详细追踪
    Trace,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
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
/// 应在应用启动时调用一次。多次调用会被忽略。
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
pub fn init_logging(level: LogLevel) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.as_str()));

    let subscriber = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(subscriber)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, warn, error, debug, trace};

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
    fn test_log_macros_compile() {
        init_logging(LogLevel::Trace);
        error!("test error");
        warn!("test warn");
        info!("test info");
        debug!("test debug");
        trace!("test trace");
    }

    #[test]
    fn test_log_level_clone_eq() {
        let level = LogLevel::Info;
        let cloned = level.clone();
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_structured_logging() {
        init_logging(LogLevel::Debug);
        let device_id = "device-123";
        let content_len = 256;
        info!(device_id, content_len, "Test structured log");
    }
}
```

**lib.rs 更新模板：**
```rust
pub mod error;
pub mod logging;

// Re-export error types
pub use error::{NearClipError, Result};

// Re-export logging
pub use logging::{init_logging, LogLevel};

// Re-export tracing macros for convenience
pub use tracing::{debug, error, info, trace, warn};
```

**Cargo.toml 依赖添加：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

### References

- [Source: docs/architecture.md#Logging Patterns]
- [Source: docs/project_context.md#日志规则]
- [Source: docs/epics.md#Story 1.3]

## Dev Agent Record

### Context Reference

Story context created by create-story workflow with exhaustive artifact analysis.

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- cargo build: SUCCESS
- cargo test -p nearclip-core: 27 unit tests + 1 doc test passed

### Completion Notes List

- Created `crates/nearclip-core/src/logging.rs` with LogLevel enum and init_logging function
- LogLevel enum includes 5 variants: Error, Warn, Info, Debug, Trace with Clone, Copy, PartialEq, Eq derives
- init_logging uses tracing-subscriber with fmt layer configured for:
  - Timestamps (default timer)
  - Module targets (with_target=true)
  - Line numbers (with_line_number=true)
  - EnvFilter for RUST_LOG environment variable override
- Updated Cargo.toml to add tracing-subscriber.workspace = true dependency
- Updated workspace Cargo.toml to enable env-filter and fmt features for tracing-subscriber
- Updated lib.rs to export logging module, init_logging, LogLevel, and tracing macros
- Added 10 unit tests in logging.rs and 2 integration tests in lib.rs
- All 27 unit tests + 1 doc test pass

### Code Review Fixes (2025-12-13)

以下问题在代码审查后已修复：

| Issue | Severity | Fix |
|-------|----------|-----|
| Missing `#[instrument]` re-export | MEDIUM | Added `instrument` to tracing macro exports in lib.rs |
| `as_str()` is private | LOW | Made `LogLevel::as_str()` public with documentation |
| Missing `Default` impl | LOW | Added `#[derive(Default)]` with `Info` as default |
| `#![allow(dead_code)]` present | LOW | Removed unnecessary allow attributes from lib.rs |
| No flush mechanism | MEDIUM | Added `flush_logs()` function for log buffer flushing |

修复后测试结果：30 单元测试 + 3 文档测试全部通过

### File List

**New Files:**
- crates/nearclip-core/src/logging.rs

**Modified Files:**
- crates/nearclip-core/src/lib.rs
- crates/nearclip-core/Cargo.toml
- Cargo.toml (workspace)

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] logging.rs 文件已创建
- [x] LogLevel 枚举包含所有五个级别
- [x] init_logging 函数正确配置 tracing-subscriber
- [x] 日志输出包含时间戳和模块来源
- [x] lib.rs 正确导出 logging 模块
- [x] tracing 宏可以从 nearclip_core 导入使用
- [x] 所有单元测试通过
- [x] `cargo build` 成功
- [x] `cargo test` 成功

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-13 | Story created by create-story workflow |
| 2025-12-13 | Implementation completed - all tasks done, 28 tests passing |
| 2025-12-13 | Code review completed - 6 issues fixed, 33 tests passing |

---

Story created: 2025-12-13
Implementation completed: 2025-12-13
Code review completed: 2025-12-13
