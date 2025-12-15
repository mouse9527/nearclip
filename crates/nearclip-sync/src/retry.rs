//! 发送重试机制
//!
//! 实现发送失败时的自动重试逻辑，支持多种重试策略。
//!
//! # Example
//!
//! ```
//! use nearclip_sync::{
//!     RetryConfig, RetryExecutor, FixedDelayStrategy, RetryCallback, SyncError,
//! };
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! struct MyCallback;
//! impl RetryCallback for MyCallback {
//!     fn on_retry(&self, attempt: u32, error: &SyncError, delay: Duration) {
//!         println!("Retry {} after {:?}: {}", attempt, delay, error);
//!     }
//!     fn on_exhausted(&self, total_attempts: u32, error: &SyncError) {
//!         println!("Exhausted {} attempts: {}", total_attempts, error);
//!     }
//! }
//!
//! let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));
//! let callback = Arc::new(MyCallback);
//! let executor = RetryExecutor::new(strategy).with_callback(callback);
//! ```

use crate::sender::SyncError;
use std::fmt;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

/// 默认最大重试次数
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// 默认重试延迟（2 秒）
pub const DEFAULT_RETRY_DELAY_SECS: u64 = 2;

/// 重试配置
///
/// 配置重试行为的基本参数。
///
/// # Example
///
/// ```
/// use nearclip_sync::RetryConfig;
/// use std::time::Duration;
///
/// let config = RetryConfig::new()
///     .with_max_retries(5)
///     .with_retry_delay(Duration::from_secs(3));
/// ```
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔
    pub retry_delay: Duration,
}

impl RetryConfig {
    /// 创建默认配置
    ///
    /// 默认值：
    /// - 最大重试次数：3
    /// - 重试间隔：2 秒
    pub fn new() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            retry_delay: Duration::from_secs(DEFAULT_RETRY_DELAY_SECS),
        }
    }

    /// 设置最大重试次数
    pub fn with_max_retries(mut self, count: u32) -> Self {
        self.max_retries = count;
        self
    }

    /// 设置重试间隔
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), SyncError> {
        if self.max_retries == 0 {
            return Err(SyncError::Configuration(
                "Max retries must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 重试策略 trait
///
/// 定义重试行为的策略接口。
pub trait RetryStrategy: Send + Sync {
    /// 获取下一次重试的延迟
    ///
    /// # Arguments
    ///
    /// * `attempt` - 当前尝试次数（从 1 开始）
    ///
    /// # Returns
    ///
    /// 如果应该重试，返回 Some(delay)；否则返回 None
    fn next_delay(&self, attempt: u32) -> Option<Duration>;

    /// 判断是否应该重试
    ///
    /// # Arguments
    ///
    /// * `attempt` - 当前尝试次数
    /// * `error` - 发生的错误
    ///
    /// # Returns
    ///
    /// 如果应该重试返回 true
    fn should_retry(&self, attempt: u32, error: &SyncError) -> bool;

    /// 获取最大重试次数
    fn max_retries(&self) -> u32;
}

/// 固定延迟重试策略
///
/// 每次重试使用相同的延迟时间。
///
/// # Example
///
/// ```
/// use nearclip_sync::{FixedDelayStrategy, RetryStrategy};
/// use std::time::Duration;
///
/// let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));
///
/// assert_eq!(strategy.next_delay(1), Some(Duration::from_secs(2)));
/// assert_eq!(strategy.next_delay(2), Some(Duration::from_secs(2)));
/// assert_eq!(strategy.next_delay(3), Some(Duration::from_secs(2)));
/// assert_eq!(strategy.next_delay(4), None); // 超过最大次数
/// ```
#[derive(Debug, Clone)]
pub struct FixedDelayStrategy {
    /// 最大重试次数
    max_retries: u32,
    /// 重试延迟
    delay: Duration,
}

impl FixedDelayStrategy {
    /// 创建固定延迟策略
    ///
    /// # Arguments
    ///
    /// * `max_retries` - 最大重试次数
    /// * `delay` - 每次重试的延迟
    pub fn new(max_retries: u32, delay: Duration) -> Self {
        Self { max_retries, delay }
    }

    /// 使用默认配置创建
    pub fn default_strategy() -> Self {
        Self::new(DEFAULT_MAX_RETRIES, Duration::from_secs(DEFAULT_RETRY_DELAY_SECS))
    }
}

impl RetryStrategy for FixedDelayStrategy {
    fn next_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt <= self.max_retries {
            Some(self.delay)
        } else {
            None
        }
    }

    fn should_retry(&self, attempt: u32, error: &SyncError) -> bool {
        // 不重试配置错误
        if matches!(error, SyncError::Configuration(_)) {
            return false;
        }
        attempt <= self.max_retries
    }

    fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

impl Default for FixedDelayStrategy {
    fn default() -> Self {
        Self::default_strategy()
    }
}

/// 指数退避重试策略
///
/// 每次重试的延迟按指数增长。
///
/// # Example
///
/// ```
/// use nearclip_sync::{ExponentialBackoffStrategy, RetryStrategy};
/// use std::time::Duration;
///
/// let strategy = ExponentialBackoffStrategy::new(
///     3,                          // 最大重试 3 次
///     Duration::from_millis(100), // 基础延迟 100ms
///     Duration::from_secs(10),    // 最大延迟 10s
///     2.0,                        // 倍数
/// );
///
/// // 第 1 次重试：100ms
/// // 第 2 次重试：200ms
/// // 第 3 次重试：400ms
/// ```
#[derive(Debug, Clone)]
pub struct ExponentialBackoffStrategy {
    /// 最大重试次数
    max_retries: u32,
    /// 基础延迟
    base_delay: Duration,
    /// 最大延迟
    max_delay: Duration,
    /// 延迟倍数
    multiplier: f64,
}

impl ExponentialBackoffStrategy {
    /// 创建指数退避策略
    ///
    /// # Arguments
    ///
    /// * `max_retries` - 最大重试次数
    /// * `base_delay` - 基础延迟
    /// * `max_delay` - 最大延迟上限
    /// * `multiplier` - 延迟倍数
    pub fn new(max_retries: u32, base_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self {
            max_retries,
            base_delay,
            max_delay,
            multiplier,
        }
    }

    /// 使用默认配置创建
    pub fn default_strategy() -> Self {
        Self::new(
            DEFAULT_MAX_RETRIES,
            Duration::from_secs(1),
            Duration::from_secs(30),
            2.0,
        )
    }
}

impl RetryStrategy for ExponentialBackoffStrategy {
    fn next_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt > self.max_retries {
            return None;
        }

        // 计算指数延迟: base_delay * multiplier^(attempt-1)
        let multiplied = self.multiplier.powi((attempt - 1) as i32);
        let delay_millis = self.base_delay.as_millis() as f64 * multiplied;
        let delay = Duration::from_millis(delay_millis as u64);

        // 限制在最大延迟内
        Some(delay.min(self.max_delay))
    }

    fn should_retry(&self, attempt: u32, error: &SyncError) -> bool {
        // 不重试配置错误
        if matches!(error, SyncError::Configuration(_)) {
            return false;
        }
        attempt <= self.max_retries
    }

    fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

impl Default for ExponentialBackoffStrategy {
    fn default() -> Self {
        Self::default_strategy()
    }
}

/// 重试回调 trait
///
/// 接收重试事件的通知。
pub trait RetryCallback: Send + Sync {
    /// 重试时调用
    ///
    /// # Arguments
    ///
    /// * `attempt` - 当前尝试次数
    /// * `error` - 导致重试的错误
    /// * `delay` - 等待延迟
    fn on_retry(&self, attempt: u32, error: &SyncError, delay: Duration);

    /// 重试耗尽时调用
    ///
    /// # Arguments
    ///
    /// * `total_attempts` - 总尝试次数
    /// * `final_error` - 最后一次错误
    fn on_exhausted(&self, total_attempts: u32, final_error: &SyncError);
}

/// 空回调实现（不执行任何操作）
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpRetryCallback;

impl RetryCallback for NoOpRetryCallback {
    fn on_retry(&self, _attempt: u32, _error: &SyncError, _delay: Duration) {}
    fn on_exhausted(&self, _total_attempts: u32, _final_error: &SyncError) {}
}

/// 重试执行结果
#[derive(Debug, Clone)]
pub struct RetryResult<T> {
    /// 结果值
    pub value: T,
    /// 尝试次数
    pub attempts: u32,
}

impl<T> RetryResult<T> {
    /// 创建新的重试结果
    pub fn new(value: T, attempts: u32) -> Self {
        Self { value, attempts }
    }
}

/// 重试执行器
///
/// 使用指定策略执行操作并在失败时重试。
///
/// # Example
///
/// ```
/// use nearclip_sync::{RetryExecutor, FixedDelayStrategy, SyncError};
/// use std::time::Duration;
/// use std::sync::atomic::{AtomicU32, Ordering};
///
/// # async fn example() -> Result<(), SyncError> {
/// let strategy = FixedDelayStrategy::new(3, Duration::from_millis(100));
/// let executor = RetryExecutor::new(strategy);
///
/// // 执行可能失败的操作
/// let count = AtomicU32::new(0);
/// let result = executor.execute(|| {
///     let n = count.fetch_add(1, Ordering::Relaxed);
///     async move {
///         if n < 2 {
///             Err(SyncError::SendFailed("temporary".into()))
///         } else {
///             Ok("success")
///         }
///     }
/// }).await;
/// # Ok(())
/// # }
/// ```
pub struct RetryExecutor<S: RetryStrategy> {
    strategy: S,
    callback: Option<Arc<dyn RetryCallback>>,
}

impl<S: RetryStrategy> RetryExecutor<S> {
    /// 创建新的重试执行器
    ///
    /// # Arguments
    ///
    /// * `strategy` - 重试策略
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            callback: None,
        }
    }

    /// 设置重试回调
    pub fn with_callback(mut self, callback: Arc<dyn RetryCallback>) -> Self {
        self.callback = Some(callback);
        self
    }

    /// 获取策略引用
    pub fn strategy(&self) -> &S {
        &self.strategy
    }

    /// 执行操作并在失败时重试
    ///
    /// # Arguments
    ///
    /// * `operation` - 要执行的异步操作
    ///
    /// # Returns
    ///
    /// 成功时返回 RetryResult，失败时返回最后一次错误
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<RetryResult<T>, SyncError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, SyncError>>,
    {
        let mut attempt = 0u32;

        loop {
            attempt += 1;

            match operation().await {
                Ok(value) => {
                    return Ok(RetryResult::new(value, attempt));
                }
                Err(e) => {
                    // 检查是否应该重试
                    if !self.strategy.should_retry(attempt, &e) {
                        if let Some(ref cb) = self.callback {
                            cb.on_exhausted(attempt, &e);
                        }
                        return Err(e);
                    }

                    // 获取延迟
                    let delay = match self.strategy.next_delay(attempt) {
                        Some(d) => d,
                        None => {
                            if let Some(ref cb) = self.callback {
                                cb.on_exhausted(attempt, &e);
                            }
                            return Err(e);
                        }
                    };

                    // 触发重试回调
                    if let Some(ref cb) = self.callback {
                        cb.on_retry(attempt, &e, delay);
                    }

                    // 等待延迟
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

}

impl<S: RetryStrategy + fmt::Debug> fmt::Debug for RetryExecutor<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RetryExecutor")
            .field("strategy", &self.strategy)
            .field("has_callback", &self.callback.is_some())
            .finish()
    }
}

/// 便捷函数：使用默认策略执行重试
///
/// 使用 3 次重试、2 秒固定延迟的默认策略。
///
/// # Example
///
/// ```
/// use nearclip_sync::{retry_with_default, SyncError};
///
/// # async fn example() -> Result<(), SyncError> {
/// let result = retry_with_default(|| async {
///     // 可能失败的操作
///     Ok::<_, SyncError>("success")
/// }).await?;
/// println!("Success after {} attempts", result.attempts);
/// # Ok(())
/// # }
/// ```
pub async fn retry_with_default<F, Fut, T>(operation: F) -> Result<RetryResult<T>, SyncError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, SyncError>>,
{
    let executor = RetryExecutor::new(FixedDelayStrategy::default());
    executor.execute(operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    // 测试回调
    struct TestCallback {
        retries: Mutex<Vec<(u32, String, Duration)>>,
        exhausted: Mutex<Option<(u32, String)>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                retries: Mutex::new(Vec::new()),
                exhausted: Mutex::new(None),
            }
        }

        fn retry_count(&self) -> usize {
            self.retries.lock().unwrap().len()
        }

        fn get_retries(&self) -> Vec<(u32, String, Duration)> {
            self.retries.lock().unwrap().clone()
        }

        fn was_exhausted(&self) -> bool {
            self.exhausted.lock().unwrap().is_some()
        }
    }

    impl RetryCallback for TestCallback {
        fn on_retry(&self, attempt: u32, error: &SyncError, delay: Duration) {
            self.retries
                .lock()
                .unwrap()
                .push((attempt, error.to_string(), delay));
        }

        fn on_exhausted(&self, total_attempts: u32, error: &SyncError) {
            *self.exhausted.lock().unwrap() = Some((total_attempts, error.to_string()));
        }
    }

    // RetryConfig tests
    #[test]
    fn test_config_new() {
        let config = RetryConfig::new();
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
        assert_eq!(config.retry_delay, Duration::from_secs(DEFAULT_RETRY_DELAY_SECS));
    }

    #[test]
    fn test_config_builder() {
        let config = RetryConfig::new()
            .with_max_retries(5)
            .with_retry_delay(Duration::from_secs(10));

        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_delay, Duration::from_secs(10));
    }

    #[test]
    fn test_config_validate_ok() {
        let config = RetryConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_retries() {
        let config = RetryConfig::new().with_max_retries(0);
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(SyncError::Configuration(_))));
    }

    #[test]
    fn test_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
    }

    #[test]
    fn test_config_clone() {
        let config = RetryConfig::new().with_max_retries(7);
        let cloned = config.clone();
        assert_eq!(config.max_retries, cloned.max_retries);
    }

    #[test]
    fn test_config_debug() {
        let config = RetryConfig::new();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RetryConfig"));
    }

    // FixedDelayStrategy tests
    #[test]
    fn test_fixed_delay_new() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));
        assert_eq!(strategy.max_retries(), 3);
    }

    #[test]
    fn test_fixed_delay_next_delay() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));

        assert_eq!(strategy.next_delay(1), Some(Duration::from_secs(2)));
        assert_eq!(strategy.next_delay(2), Some(Duration::from_secs(2)));
        assert_eq!(strategy.next_delay(3), Some(Duration::from_secs(2)));
        assert_eq!(strategy.next_delay(4), None);
    }

    #[test]
    fn test_fixed_delay_should_retry() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));

        // 应该重试常规错误
        assert!(strategy.should_retry(1, &SyncError::SendFailed("test".into())));
        assert!(strategy.should_retry(2, &SyncError::Timeout("test".into())));
        assert!(strategy.should_retry(3, &SyncError::ChannelUnavailable));

        // 不应该重试配置错误
        assert!(!strategy.should_retry(1, &SyncError::Configuration("test".into())));

        // 超过最大次数
        assert!(!strategy.should_retry(4, &SyncError::SendFailed("test".into())));
    }

    #[test]
    fn test_fixed_delay_default() {
        let strategy = FixedDelayStrategy::default();
        assert_eq!(strategy.max_retries(), DEFAULT_MAX_RETRIES);
    }

    #[test]
    fn test_fixed_delay_clone() {
        let strategy = FixedDelayStrategy::new(5, Duration::from_secs(3));
        let cloned = strategy.clone();
        assert_eq!(strategy.max_retries(), cloned.max_retries());
    }

    #[test]
    fn test_fixed_delay_debug() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("FixedDelayStrategy"));
    }

    // ExponentialBackoffStrategy tests
    #[test]
    fn test_exponential_new() {
        let strategy = ExponentialBackoffStrategy::new(
            3,
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        );
        assert_eq!(strategy.max_retries(), 3);
    }

    #[test]
    fn test_exponential_next_delay() {
        let strategy = ExponentialBackoffStrategy::new(
            5,
            Duration::from_millis(100),
            Duration::from_secs(10),
            2.0,
        );

        // 100ms * 2^0 = 100ms
        assert_eq!(strategy.next_delay(1), Some(Duration::from_millis(100)));
        // 100ms * 2^1 = 200ms
        assert_eq!(strategy.next_delay(2), Some(Duration::from_millis(200)));
        // 100ms * 2^2 = 400ms
        assert_eq!(strategy.next_delay(3), Some(Duration::from_millis(400)));
        // 100ms * 2^3 = 800ms
        assert_eq!(strategy.next_delay(4), Some(Duration::from_millis(800)));
        // 100ms * 2^4 = 1600ms
        assert_eq!(strategy.next_delay(5), Some(Duration::from_millis(1600)));
        // 超过最大次数
        assert_eq!(strategy.next_delay(6), None);
    }

    #[test]
    fn test_exponential_max_delay_cap() {
        let strategy = ExponentialBackoffStrategy::new(
            10,
            Duration::from_secs(1),
            Duration::from_secs(5), // 上限 5 秒
            2.0,
        );

        // 1s * 2^0 = 1s
        assert_eq!(strategy.next_delay(1), Some(Duration::from_secs(1)));
        // 1s * 2^1 = 2s
        assert_eq!(strategy.next_delay(2), Some(Duration::from_secs(2)));
        // 1s * 2^2 = 4s
        assert_eq!(strategy.next_delay(3), Some(Duration::from_secs(4)));
        // 1s * 2^3 = 8s, 但上限是 5s
        assert_eq!(strategy.next_delay(4), Some(Duration::from_secs(5)));
        // 1s * 2^4 = 16s, 但上限是 5s
        assert_eq!(strategy.next_delay(5), Some(Duration::from_secs(5)));
    }

    #[test]
    fn test_exponential_should_retry() {
        let strategy = ExponentialBackoffStrategy::default();

        assert!(strategy.should_retry(1, &SyncError::SendFailed("test".into())));
        assert!(!strategy.should_retry(1, &SyncError::Configuration("test".into())));
    }

    #[test]
    fn test_exponential_default() {
        let strategy = ExponentialBackoffStrategy::default();
        assert_eq!(strategy.max_retries(), DEFAULT_MAX_RETRIES);
    }

    #[test]
    fn test_exponential_debug() {
        let strategy = ExponentialBackoffStrategy::default();
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("ExponentialBackoffStrategy"));
    }

    // NoOpRetryCallback tests
    #[test]
    fn test_noop_callback() {
        let callback = NoOpRetryCallback;
        // 这些不应该 panic
        callback.on_retry(1, &SyncError::SendFailed("test".into()), Duration::from_secs(1));
        callback.on_exhausted(3, &SyncError::SendFailed("test".into()));
    }

    #[test]
    fn test_noop_callback_debug() {
        let callback = NoOpRetryCallback;
        let debug_str = format!("{:?}", callback);
        assert!(debug_str.contains("NoOpRetryCallback"));
    }

    // RetryResult tests
    #[test]
    fn test_retry_result() {
        let result = RetryResult::new("success", 2);
        assert_eq!(result.value, "success");
        assert_eq!(result.attempts, 2);
    }

    #[test]
    fn test_retry_result_debug() {
        let result = RetryResult::new(42, 1);
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("RetryResult"));
    }

    // RetryExecutor tests
    #[tokio::test]
    async fn test_executor_success_first_try() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
        let executor = RetryExecutor::new(strategy);

        let result = executor.execute(|| async { Ok::<_, SyncError>("success") }).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.value, "success");
        assert_eq!(result.attempts, 1);
    }

    #[tokio::test]
    async fn test_executor_success_after_retry() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
        let callback = Arc::new(TestCallback::new());
        let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

        let count = AtomicU32::new(0);
        let result = executor
            .execute(|| async {
                let n = count.fetch_add(1, Ordering::Relaxed);
                if n < 2 {
                    Err(SyncError::SendFailed("temporary".into()))
                } else {
                    Ok("success")
                }
            })
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.value, "success");
        assert_eq!(result.attempts, 3);
        assert_eq!(callback.retry_count(), 2);
        assert!(!callback.was_exhausted());
    }

    #[tokio::test]
    async fn test_executor_exhausted() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
        let callback = Arc::new(TestCallback::new());
        let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

        let result = executor
            .execute(|| async { Err::<(), _>(SyncError::SendFailed("always fail".into())) })
            .await;

        assert!(result.is_err());
        assert_eq!(callback.retry_count(), 3); // 3 retries
        assert!(callback.was_exhausted());
    }

    #[tokio::test]
    async fn test_executor_no_retry_config_error() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
        let callback = Arc::new(TestCallback::new());
        let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

        let result = executor
            .execute(|| async { Err::<(), _>(SyncError::Configuration("bad config".into())) })
            .await;

        assert!(result.is_err());
        assert_eq!(callback.retry_count(), 0); // 不重试配置错误
        assert!(callback.was_exhausted());
    }

    #[tokio::test]
    async fn test_executor_strategy_accessor() {
        let strategy = FixedDelayStrategy::new(5, Duration::from_secs(1));
        let executor = RetryExecutor::new(strategy);

        assert_eq!(executor.strategy().max_retries(), 5);
    }

    #[tokio::test]
    async fn test_executor_debug() {
        let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
        let executor = RetryExecutor::new(strategy);

        let debug_str = format!("{:?}", executor);
        assert!(debug_str.contains("RetryExecutor"));
    }

    #[tokio::test]
    async fn test_executor_callback_values() {
        let strategy = FixedDelayStrategy::new(2, Duration::from_millis(50));
        let callback = Arc::new(TestCallback::new());
        let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

        let _ = executor
            .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
            .await;

        let retries = callback.get_retries();
        assert_eq!(retries.len(), 2);
        assert_eq!(retries[0].0, 1); // 第 1 次尝试
        assert_eq!(retries[0].2, Duration::from_millis(50));
        assert_eq!(retries[1].0, 2); // 第 2 次尝试
    }

    // retry_with_default tests
    #[tokio::test]
    async fn test_retry_with_default_success() {
        let result = retry_with_default(|| async { Ok::<_, SyncError>("ok") }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().attempts, 1);
    }

    #[tokio::test]
    async fn test_retry_with_default_failure() {
        let count = AtomicU32::new(0);
        let result = retry_with_default(|| async {
            count.fetch_add(1, Ordering::Relaxed);
            Err::<(), _>(SyncError::SendFailed("fail".into()))
        })
        .await;

        assert!(result.is_err());
        // 应该尝试 1 + 3 次重试 = 4 次（但 should_retry 在第 4 次返回 false）
        // 实际是 1 初始 + 3 重试 = 4 次调用
        assert!(count.load(Ordering::Relaxed) >= 3);
    }
}
