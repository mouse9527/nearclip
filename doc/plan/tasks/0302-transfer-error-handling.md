# Task 0302: 实现传输错误处理 (TDD版本)

## 任务描述

按照TDD原则实现文本传输的错误处理机制，为传输过程中的异常情况提供统一的错误处理。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_error_tests.rs
#[cfg(test)]
mod transfer_error_tests {
    use super::*;
    
    #[test]
    fn test_connection_error() {
        // RED: 测试连接错误
        let error = TransferError::ConnectionFailed("Device not reachable".to_string());
        
        assert!(matches!(error, TransferError::ConnectionFailed(_)));
        assert_eq!(error.to_string(), "Connection failed: Device not reachable");
        assert_eq!(error.code(), ErrorCode::ConnectionFailed);
    }
    
    #[test]
    fn test_validation_error() {
        // RED: 测试验证错误
        let error = TransferError::ValidationError("Invalid session ID".to_string());
        
        assert!(matches!(error, TransferError::ValidationError(_)));
        assert_eq!(error.code(), ErrorCode::ValidationError);
    }
    
    #[test]
    fn test_timeout_error() {
        // RED: 测试超时错误
        let error = TransferError::Timeout("Transfer timed out".to_string());
        
        assert!(matches!(error, TransferError::Timeout(_)));
        assert_eq!(error.to_string(), "Timeout: Transfer timed out");
        assert_eq!(error.code(), ErrorCode::Timeout);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    ConnectionFailed,
    ValidationError,
    Timeout,
    AuthenticationFailed,
    NetworkError,
}

#[derive(Debug)]
pub enum TransferError {
    ConnectionFailed(String),
    ValidationError(String),
    Timeout(String),
}

impl TransferError {
    pub fn code(&self) -> ErrorCode {
        match self {
            TransferError::ConnectionFailed(_) => ErrorCode::ConnectionFailed,
            TransferError::ValidationError(_) => ErrorCode::ValidationError,
            TransferError::Timeout(_) => ErrorCode::Timeout,
        }
    }
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            TransferError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            TransferError::Timeout(msg) => write!(f, "Timeout: {}", msg),
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    ConnectionFailed,
    ValidationError,
    Timeout,
    AuthenticationFailed,
    NetworkError,
    ProtocolError,
    Cancellation,
}

#[derive(Debug)]
pub struct TransferError {
    code: ErrorCode,
    message: String,
    context: ErrorContext,
    retry_strategy: RetryStrategy,
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub session_id: Option<String>,
    pub device_id: Option<String>,
    pub timestamp: std::time::SystemTime,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RetryStrategy {
    NoRetry,
    Immediate,
    Delayed(Duration),
    ExponentialBackoff { initial_delay: Duration, max_retries: u32 },
}

impl TransferError {
    // 重构后的代码，保持测试绿色
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message,
            context: ErrorContext {
                session_id: None,
                device_id: None,
                timestamp: std::time::SystemTime::now(),
                stack_trace: None,
            },
            retry_strategy: Self::default_retry_strategy(&code),
        }
    }
    
    pub fn with_context(mut self, session_id: Option<String>, device_id: Option<String>) -> Self {
        self.context.session_id = session_id;
        self.context.device_id = device_id;
        self
    }
    
    pub fn code(&self) -> &ErrorCode {
        &self.code
    }
    
    pub fn message(&self) -> &str {
        &self.message
    }
    
    pub fn context(&self) -> &ErrorContext {
        &self.context
    }
    
    pub fn is_retriable(&self) -> bool {
        !matches!(self.retry_strategy, RetryStrategy::NoRetry)
    }
    
    pub fn should_retry_now(&self) -> bool {
        matches!(self.retry_strategy, RetryStrategy::Immediate)
    }
    
    fn default_retry_strategy(code: &ErrorCode) -> RetryStrategy {
        match code {
            ErrorCode::ConnectionFailed => RetryStrategy::Delayed(Duration::from_secs(1)),
            ErrorCode::Timeout => RetryStrategy::Immediate,
            ErrorCode::NetworkError => RetryStrategy::ExponentialBackoff {
                initial_delay: Duration::from_millis(100),
                max_retries: 3,
            },
            _ => RetryStrategy::NoRetry,
        }
    }
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl std::error::Error for TransferError {}

// 便捷的构造函数
impl TransferError {
    pub fn connection_failed(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::ConnectionFailed, msg.into())
    }
    
    pub fn validation_error(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationError, msg.into())
    }
    
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::Timeout, msg.into())
    }
    
    pub fn authentication_failed(msg: impl Into<String>) -> Self {
        Self::new(ErrorCode::AuthenticationFailed, msg.into())
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的错误类型，不依赖具体实现：

```rust
// rust-core/domain/errors/transfer.rs
#[derive(Debug)]
pub enum TransferError {
    // 传输错误变体
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [文本传输会话](0301-transfer-session.md)

## 后续任务

- [Task 0303: 实现传输事件系统](0303-transfer-event-system.md)
- [Task 0304: 实现传输队列管理](0304-transfer-queue-management.md)
- [Task 0305: 实现传输协议处理](0305-transfer-protocol-handling.md)