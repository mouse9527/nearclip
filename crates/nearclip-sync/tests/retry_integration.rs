//! 重试机制集成测试
//!
//! 这些测试验证重试机制的完整流程。
//!
//! 主要测试:
//! - 成功无需重试
//! - 重试后成功
//! - 重试耗尽失败
//! - 回调正确触发

use nearclip_sync::{
    retry_with_default, ExponentialBackoffStrategy, FixedDelayStrategy, NoOpRetryCallback,
    RetryCallback, RetryConfig, RetryExecutor, RetryResult, RetryStrategy, SyncError,
    DEFAULT_MAX_RETRIES, DEFAULT_RETRY_DELAY_SECS,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// 测试回调实现
// ============================================================

struct TestCallback {
    retries: Arc<Mutex<Vec<(u32, String, Duration)>>>,
    exhausted: Arc<Mutex<Option<(u32, String)>>>,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            retries: Arc::new(Mutex::new(Vec::new())),
            exhausted: Arc::new(Mutex::new(None)),
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

    fn get_exhausted(&self) -> Option<(u32, String)> {
        self.exhausted.lock().unwrap().clone()
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

// ============================================================
// RetryConfig 测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = RetryConfig::new();
    assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
    assert_eq!(
        config.retry_delay,
        Duration::from_secs(DEFAULT_RETRY_DELAY_SECS)
    );
}

#[test]
fn test_config_builder_chain() {
    let config = RetryConfig::new()
        .with_max_retries(5)
        .with_retry_delay(Duration::from_secs(10));

    assert_eq!(config.max_retries, 5);
    assert_eq!(config.retry_delay, Duration::from_secs(10));
}

#[test]
fn test_config_validation_success() {
    let config = RetryConfig::new();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_zero_retries() {
    let config = RetryConfig::new().with_max_retries(0);
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(SyncError::Configuration(_))));
}

// ============================================================
// FixedDelayStrategy 测试
// ============================================================

#[test]
fn test_fixed_delay_strategy_default() {
    let strategy = FixedDelayStrategy::default();
    assert_eq!(strategy.max_retries(), DEFAULT_MAX_RETRIES);
}

#[test]
fn test_fixed_delay_strategy_next_delay() {
    let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));

    // 应该在最大次数内返回相同延迟
    assert_eq!(strategy.next_delay(1), Some(Duration::from_secs(2)));
    assert_eq!(strategy.next_delay(2), Some(Duration::from_secs(2)));
    assert_eq!(strategy.next_delay(3), Some(Duration::from_secs(2)));

    // 超过最大次数返回 None
    assert_eq!(strategy.next_delay(4), None);
}

#[test]
fn test_fixed_delay_strategy_should_retry() {
    let strategy = FixedDelayStrategy::new(3, Duration::from_secs(2));

    // 应该重试常规错误
    assert!(strategy.should_retry(1, &SyncError::SendFailed("test".into())));
    assert!(strategy.should_retry(2, &SyncError::Timeout("test".into())));
    assert!(strategy.should_retry(3, &SyncError::ChannelUnavailable));

    // 不应该重试配置错误
    assert!(!strategy.should_retry(1, &SyncError::Configuration("test".into())));

    // 超过最大次数不重试
    assert!(!strategy.should_retry(4, &SyncError::SendFailed("test".into())));
}

// ============================================================
// ExponentialBackoffStrategy 测试
// ============================================================

#[test]
fn test_exponential_strategy_default() {
    let strategy = ExponentialBackoffStrategy::default();
    assert_eq!(strategy.max_retries(), DEFAULT_MAX_RETRIES);
}

#[test]
fn test_exponential_strategy_next_delay() {
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
}

#[test]
fn test_exponential_strategy_max_delay_cap() {
    let strategy = ExponentialBackoffStrategy::new(
        10,
        Duration::from_secs(1),
        Duration::from_secs(5), // 上限 5 秒
        2.0,
    );

    // 1s * 2^3 = 8s, 但上限是 5s
    assert_eq!(strategy.next_delay(4), Some(Duration::from_secs(5)));
    // 1s * 2^4 = 16s, 但上限是 5s
    assert_eq!(strategy.next_delay(5), Some(Duration::from_secs(5)));
}

#[test]
fn test_exponential_strategy_should_retry() {
    let strategy = ExponentialBackoffStrategy::default();

    assert!(strategy.should_retry(1, &SyncError::SendFailed("test".into())));
    assert!(!strategy.should_retry(1, &SyncError::Configuration("test".into())));
}

// ============================================================
// NoOpRetryCallback 测试
// ============================================================

#[test]
fn test_noop_callback() {
    let callback = NoOpRetryCallback;
    // 这些不应该 panic
    callback.on_retry(
        1,
        &SyncError::SendFailed("test".into()),
        Duration::from_secs(1),
    );
    callback.on_exhausted(3, &SyncError::SendFailed("test".into()));
}

// ============================================================
// RetryResult 测试
// ============================================================

#[test]
fn test_retry_result() {
    let result = RetryResult::new("success", 2);
    assert_eq!(result.value, "success");
    assert_eq!(result.attempts, 2);
}

// ============================================================
// RetryExecutor 测试 - 成功场景
// ============================================================

#[tokio::test]
async fn test_executor_success_first_try() {
    let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
    let executor = RetryExecutor::new(strategy);

    let result = executor
        .execute(|| async { Ok::<_, SyncError>("success") })
        .await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "success");
    assert_eq!(result.attempts, 1);
}

#[tokio::test]
async fn test_executor_success_after_one_retry() {
    let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let count = AtomicU32::new(0);
    let result = executor
        .execute(|| {
            let n = count.fetch_add(1, Ordering::Relaxed);
            async move {
                if n < 1 {
                    Err(SyncError::SendFailed("temporary".into()))
                } else {
                    Ok("success")
                }
            }
        })
        .await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "success");
    assert_eq!(result.attempts, 2);
    assert_eq!(callback.retry_count(), 1);
    assert!(!callback.was_exhausted());
}

#[tokio::test]
async fn test_executor_success_after_multiple_retries() {
    let strategy = FixedDelayStrategy::new(5, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let count = AtomicU32::new(0);
    let result = executor
        .execute(|| {
            let n = count.fetch_add(1, Ordering::Relaxed);
            async move {
                if n < 3 {
                    Err(SyncError::SendFailed("temporary".into()))
                } else {
                    Ok("success")
                }
            }
        })
        .await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "success");
    assert_eq!(result.attempts, 4); // 1 initial + 3 retries
    assert_eq!(callback.retry_count(), 3);
    assert!(!callback.was_exhausted());
}

// ============================================================
// RetryExecutor 测试 - 失败场景
// ============================================================

#[tokio::test]
async fn test_executor_exhausted_after_max_retries() {
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
async fn test_executor_no_retry_for_config_error() {
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
async fn test_executor_correct_error_passed_to_exhausted() {
    let strategy = FixedDelayStrategy::new(2, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let _ = executor
        .execute(|| async { Err::<(), _>(SyncError::Timeout("timeout error".into())) })
        .await;

    let exhausted = callback.get_exhausted().unwrap();
    assert!(exhausted.1.contains("timeout error"));
}

// ============================================================
// RetryExecutor 测试 - 回调验证
// ============================================================

#[tokio::test]
async fn test_executor_callback_receives_correct_attempt_numbers() {
    let strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let _ = executor
        .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
        .await;

    let retries = callback.get_retries();
    assert_eq!(retries.len(), 3);
    assert_eq!(retries[0].0, 1); // 第 1 次尝试后
    assert_eq!(retries[1].0, 2); // 第 2 次尝试后
    assert_eq!(retries[2].0, 3); // 第 3 次尝试后
}

#[tokio::test]
async fn test_executor_callback_receives_correct_delay() {
    let strategy = FixedDelayStrategy::new(2, Duration::from_millis(50));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let _ = executor
        .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
        .await;

    let retries = callback.get_retries();
    assert_eq!(retries[0].2, Duration::from_millis(50));
    assert_eq!(retries[1].2, Duration::from_millis(50));
}

#[tokio::test]
async fn test_executor_exponential_callback_receives_increasing_delay() {
    let strategy = ExponentialBackoffStrategy::new(
        3,
        Duration::from_millis(10),
        Duration::from_secs(1),
        2.0,
    );
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let _ = executor
        .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
        .await;

    let retries = callback.get_retries();
    assert_eq!(retries[0].2, Duration::from_millis(10)); // 10ms
    assert_eq!(retries[1].2, Duration::from_millis(20)); // 20ms
    assert_eq!(retries[2].2, Duration::from_millis(40)); // 40ms
}

// ============================================================
// retry_with_default 测试
// ============================================================

#[tokio::test]
async fn test_retry_with_default_success() {
    let result = retry_with_default(|| async { Ok::<_, SyncError>("ok") }).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().attempts, 1);
}

#[tokio::test]
async fn test_retry_with_default_success_after_retry() {
    let count = AtomicU32::new(0);
    let result = retry_with_default(|| {
        let n = count.fetch_add(1, Ordering::Relaxed);
        async move {
            if n < 2 {
                Err(SyncError::SendFailed("fail".into()))
            } else {
                Ok("success")
            }
        }
    })
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().attempts, 3);
}

#[tokio::test]
async fn test_retry_with_default_failure() {
    let result =
        retry_with_default(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) }).await;

    assert!(result.is_err());
}

// ============================================================
// 策略比较测试
// ============================================================

#[tokio::test]
async fn test_fixed_vs_exponential_retry_timing() {
    // 使用固定延迟策略
    let fixed_strategy = FixedDelayStrategy::new(3, Duration::from_millis(10));
    let fixed_executor = RetryExecutor::new(fixed_strategy);

    let start_fixed = std::time::Instant::now();
    let _ = fixed_executor
        .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
        .await;
    let fixed_duration = start_fixed.elapsed();

    // 使用指数退避策略
    let exp_strategy = ExponentialBackoffStrategy::new(
        3,
        Duration::from_millis(10),
        Duration::from_secs(1),
        2.0,
    );
    let exp_executor = RetryExecutor::new(exp_strategy);

    let start_exp = std::time::Instant::now();
    let _ = exp_executor
        .execute(|| async { Err::<(), _>(SyncError::SendFailed("fail".into())) })
        .await;
    let exp_duration = start_exp.elapsed();

    // 固定延迟: 10ms + 10ms + 10ms = 30ms
    // 指数退避: 10ms + 20ms + 40ms = 70ms
    // 指数退避应该比固定延迟花更长时间
    assert!(exp_duration > fixed_duration);
}

// ============================================================
// 边界条件测试
// ============================================================

#[tokio::test]
async fn test_single_retry_strategy() {
    let strategy = FixedDelayStrategy::new(1, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let count = AtomicU32::new(0);
    let result = executor
        .execute(|| {
            let n = count.fetch_add(1, Ordering::Relaxed);
            async move {
                if n < 1 {
                    Err(SyncError::SendFailed("fail".into()))
                } else {
                    Ok("success")
                }
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().attempts, 2);
    assert_eq!(callback.retry_count(), 1);
}

#[tokio::test]
async fn test_different_error_types() {
    let strategy = FixedDelayStrategy::new(5, Duration::from_millis(10));
    let callback = Arc::new(TestCallback::new());
    let executor = RetryExecutor::new(strategy).with_callback(callback.clone());

    let count = AtomicU32::new(0);
    let result = executor
        .execute(|| {
            let n = count.fetch_add(1, Ordering::Relaxed);
            async move {
                match n {
                    0 => Err(SyncError::SendFailed("send failed".into())),
                    1 => Err(SyncError::Timeout("timeout".into())),
                    2 => Err(SyncError::ChannelUnavailable),
                    _ => Ok("success"),
                }
            }
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().attempts, 4);

    let retries = callback.get_retries();
    assert!(retries[0].1.contains("send failed"));
    assert!(retries[1].1.contains("timeout"));
    assert!(retries[2].1.contains("No channel available"));
}
