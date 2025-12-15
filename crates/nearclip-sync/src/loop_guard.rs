//! 同步循环防护模块
//!
//! 防止剪贴板内容在设备间无限循环同步。
//!
//! # 工作原理
//!
//! 当收到远程剪贴板内容时，`LoopGuard` 会记录该内容的指纹和来源。
//! 当本地剪贴板变化时，通过检查指纹来判断是否应该同步：
//! - 如果内容来自远程设备（刚写入本地），则不应该再次同步
//! - 如果内容是本地产生的新内容，则应该同步
//!
//! # 示例
//!
//! ```
//! use nearclip_sync::{LoopGuard, LoopGuardConfig, ContentOrigin};
//!
//! // 创建默认配置的循环防护
//! let guard = LoopGuard::new(LoopGuardConfig::new());
//!
//! // 收到远程内容时记录
//! let content = b"Hello from remote";
//! guard.record_remote(content, "device-123");
//!
//! // 本地检测到变化时检查
//! assert!(!guard.should_sync(content)); // 不应该同步（来自远程）
//!
//! // 本地新内容应该同步
//! let local_content = b"Local content";
//! assert!(guard.should_sync(local_content)); // 应该同步
//! ```

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// 默认历史记录大小
pub const DEFAULT_HISTORY_SIZE: usize = 100;

/// 默认过期时间（秒）
pub const DEFAULT_EXPIRY_SECS: u64 = 60;

// ============================================================
// ContentOrigin - 内容来源
// ============================================================

/// 内容来源枚举
///
/// 标识剪贴板内容是来自本地还是远程设备。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentOrigin {
    /// 本地产生的内容
    Local,
    /// 远程设备发送的内容，包含设备 ID
    Remote(String),
}

impl ContentOrigin {
    /// 检查是否为本地来源
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::ContentOrigin;
    ///
    /// assert!(ContentOrigin::Local.is_local());
    /// assert!(!ContentOrigin::Remote("device-1".into()).is_local());
    /// ```
    pub fn is_local(&self) -> bool {
        matches!(self, ContentOrigin::Local)
    }

    /// 检查是否为远程来源
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::ContentOrigin;
    ///
    /// assert!(!ContentOrigin::Local.is_remote());
    /// assert!(ContentOrigin::Remote("device-1".into()).is_remote());
    /// ```
    pub fn is_remote(&self) -> bool {
        matches!(self, ContentOrigin::Remote(_))
    }

    /// 获取远程设备 ID（如果是远程来源）
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::ContentOrigin;
    ///
    /// assert_eq!(ContentOrigin::Local.device_id(), None);
    /// assert_eq!(ContentOrigin::Remote("device-1".into()).device_id(), Some("device-1"));
    /// ```
    pub fn device_id(&self) -> Option<&str> {
        match self {
            ContentOrigin::Local => None,
            ContentOrigin::Remote(id) => Some(id),
        }
    }
}

// ============================================================
// ContentFingerprint - 内容指纹
// ============================================================

/// 内容指纹
///
/// 使用 SHA256 哈希的前 16 字节作为内容的唯一标识。
/// 这个大小在保持足够唯一性的同时，也节省了内存。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentFingerprint {
    hash: [u8; 16],
}

impl ContentFingerprint {
    /// 从内容计算指纹
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::ContentFingerprint;
    ///
    /// let fp1 = ContentFingerprint::from_content(b"Hello");
    /// let fp2 = ContentFingerprint::from_content(b"Hello");
    /// let fp3 = ContentFingerprint::from_content(b"World");
    ///
    /// assert_eq!(fp1, fp2); // 相同内容相同指纹
    /// assert_ne!(fp1, fp3); // 不同内容不同指纹
    /// ```
    pub fn from_content(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();

        let mut hash = [0u8; 16];
        hash.copy_from_slice(&result[..16]);

        Self { hash }
    }

    /// 获取原始哈希字节
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.hash
    }

    /// 转换为十六进制字符串
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::ContentFingerprint;
    ///
    /// let fp = ContentFingerprint::from_content(b"test");
    /// let hex = fp.to_hex();
    /// assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    /// ```
    pub fn to_hex(&self) -> String {
        self.hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}

impl std::fmt::Display for ContentFingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

// ============================================================
// HistoryEntry - 历史记录条目
// ============================================================

/// 历史记录条目
///
/// 存储内容指纹、来源和时间戳。
#[derive(Debug, Clone)]
struct HistoryEntry {
    /// 内容来源
    origin: ContentOrigin,
    /// 记录时间
    timestamp: Instant,
}

impl HistoryEntry {
    fn new(origin: ContentOrigin) -> Self {
        Self {
            origin,
            timestamp: Instant::now(),
        }
    }

    /// 检查是否已过期
    fn is_expired(&self, expiry: Duration) -> bool {
        self.timestamp.elapsed() > expiry
    }
}

// ============================================================
// LoopGuardConfig - 循环防护配置
// ============================================================

/// 循环防护配置
///
/// 使用 builder 模式配置历史记录大小和过期时间。
///
/// # 示例
///
/// ```
/// use nearclip_sync::LoopGuardConfig;
/// use std::time::Duration;
///
/// let config = LoopGuardConfig::new()
///     .with_history_size(200)
///     .with_expiry_duration(Duration::from_secs(120));
///
/// assert_eq!(config.history_size, 200);
/// assert_eq!(config.expiry_duration, Duration::from_secs(120));
/// ```
#[derive(Debug, Clone)]
pub struct LoopGuardConfig {
    /// 历史记录最大数量（默认 100）
    pub history_size: usize,
    /// 记录过期时间（默认 60 秒）
    pub expiry_duration: Duration,
}

impl Default for LoopGuardConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl LoopGuardConfig {
    /// 创建默认配置
    ///
    /// - 历史大小: 100
    /// - 过期时间: 60 秒
    pub fn new() -> Self {
        Self {
            history_size: DEFAULT_HISTORY_SIZE,
            expiry_duration: Duration::from_secs(DEFAULT_EXPIRY_SECS),
        }
    }

    /// 设置历史记录大小
    ///
    /// # 参数
    ///
    /// * `size` - 历史记录最大数量
    pub fn with_history_size(mut self, size: usize) -> Self {
        self.history_size = size;
        self
    }

    /// 设置过期时间
    ///
    /// # 参数
    ///
    /// * `duration` - 记录过期时间
    pub fn with_expiry_duration(mut self, duration: Duration) -> Self {
        self.expiry_duration = duration;
        self
    }

    /// 验证配置
    ///
    /// # 错误
    ///
    /// - 历史大小为 0
    /// - 过期时间为 0
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::LoopGuardConfig;
    ///
    /// let config = LoopGuardConfig::new();
    /// assert!(config.validate().is_ok());
    ///
    /// let invalid = LoopGuardConfig::new().with_history_size(0);
    /// assert!(invalid.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), LoopGuardError> {
        if self.history_size == 0 {
            return Err(LoopGuardError::InvalidConfig(
                "history_size must be greater than 0".into(),
            ));
        }
        if self.expiry_duration.is_zero() {
            return Err(LoopGuardError::InvalidConfig(
                "expiry_duration must be greater than 0".into(),
            ));
        }
        Ok(())
    }
}

// ============================================================
// LoopGuardError - 错误类型
// ============================================================

/// 循环防护错误
#[derive(Debug, Clone, thiserror::Error)]
pub enum LoopGuardError {
    /// 无效配置
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

// ============================================================
// LoopGuard - 循环防护核心
// ============================================================

/// 循环防护
///
/// 通过记录内容指纹和来源，防止剪贴板内容在设备间无限循环同步。
///
/// # 线程安全
///
/// `LoopGuard` 使用 `RwLock` 保护内部状态，可以安全地在多线程环境中使用。
///
/// # 示例
///
/// ```
/// use nearclip_sync::{LoopGuard, LoopGuardConfig};
///
/// let guard = LoopGuard::new(LoopGuardConfig::new());
///
/// // 模拟收到远程内容
/// let remote_content = b"Hello from device-2";
/// guard.record_remote(remote_content, "device-2");
///
/// // 写入本地剪贴板后，检测到变化
/// // should_sync 返回 false，因为这是远程内容
/// assert!(!guard.should_sync(remote_content));
///
/// // 用户复制的新内容应该同步
/// let new_content = b"User typed this";
/// assert!(guard.should_sync(new_content));
/// ```
pub struct LoopGuard {
    config: LoopGuardConfig,
    history: RwLock<HashMap<ContentFingerprint, HistoryEntry>>,
    /// 插入顺序追踪（用于 LRU 淘汰）
    insertion_order: RwLock<Vec<ContentFingerprint>>,
}

impl LoopGuard {
    /// 创建新的循环防护实例
    ///
    /// # 参数
    ///
    /// * `config` - 循环防护配置
    pub fn new(config: LoopGuardConfig) -> Self {
        Self {
            config,
            history: RwLock::new(HashMap::new()),
            insertion_order: RwLock::new(Vec::new()),
        }
    }

    /// 记录本地内容
    ///
    /// 当本地产生新的剪贴板内容时调用。
    ///
    /// # 参数
    ///
    /// * `content` - 剪贴板内容
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    /// guard.record_local(b"Local content");
    /// ```
    pub fn record_local(&self, content: &[u8]) {
        self.record(content, ContentOrigin::Local);
    }

    /// 记录远程内容
    ///
    /// 当收到远程设备发送的剪贴板内容时调用。
    /// 在写入本地剪贴板之前调用此方法。
    ///
    /// # 参数
    ///
    /// * `content` - 剪贴板内容
    /// * `device_id` - 发送设备的 ID
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    /// guard.record_remote(b"Remote content", "device-123");
    /// ```
    pub fn record_remote(&self, content: &[u8], device_id: &str) {
        self.record(content, ContentOrigin::Remote(device_id.to_string()));
    }

    /// 内部记录方法
    fn record(&self, content: &[u8], origin: ContentOrigin) {
        let fingerprint = ContentFingerprint::from_content(content);
        let entry = HistoryEntry::new(origin);

        // 清理过期记录
        self.cleanup_expired();

        let mut history = self.history.write().unwrap();
        let mut order = self.insertion_order.write().unwrap();

        // 如果已存在，更新并移动到末尾
        if history.contains_key(&fingerprint) {
            order.retain(|fp| fp != &fingerprint);
        }

        // 插入新记录
        history.insert(fingerprint, entry);
        order.push(fingerprint);

        // LRU 淘汰
        while order.len() > self.config.history_size {
            if let Some(oldest) = order.first().copied() {
                order.remove(0);
                history.remove(&oldest);
            }
        }

        tracing::trace!(
            fingerprint = %fingerprint,
            history_size = history.len(),
            "Recorded content"
        );
    }

    /// 判断内容是否应该同步
    ///
    /// 检查给定内容是否应该发送到其他设备：
    /// - 如果内容来自远程设备（且未过期），返回 `false`
    /// - 如果内容是本地新产生的或不在历史中，返回 `true`
    ///
    /// # 参数
    ///
    /// * `content` - 要检查的剪贴板内容
    ///
    /// # 返回
    ///
    /// - `true`: 应该同步到其他设备
    /// - `false`: 不应该同步（防止循环）
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    ///
    /// // 远程内容不应该同步
    /// guard.record_remote(b"Remote", "device-1");
    /// assert!(!guard.should_sync(b"Remote"));
    ///
    /// // 新内容应该同步
    /// assert!(guard.should_sync(b"New local content"));
    /// ```
    pub fn should_sync(&self, content: &[u8]) -> bool {
        let fingerprint = ContentFingerprint::from_content(content);

        // 清理过期记录
        self.cleanup_expired();

        let history = self.history.read().unwrap();

        match history.get(&fingerprint) {
            Some(entry) => {
                // 如果记录已过期，应该同步
                if entry.is_expired(self.config.expiry_duration) {
                    tracing::trace!(
                        fingerprint = %fingerprint,
                        "Content expired, should sync"
                    );
                    return true;
                }

                // 如果来自远程，不应该同步
                if entry.origin.is_remote() {
                    tracing::trace!(
                        fingerprint = %fingerprint,
                        device_id = ?entry.origin.device_id(),
                        "Content from remote, should not sync"
                    );
                    return false;
                }

                // 本地内容，应该同步
                tracing::trace!(
                    fingerprint = %fingerprint,
                    "Content from local, should sync"
                );
                true
            }
            None => {
                // 不在历史中，是新内容，应该同步
                tracing::trace!(
                    fingerprint = %fingerprint,
                    "Content not in history, should sync"
                );
                true
            }
        }
    }

    /// 判断内容是否来自远程
    ///
    /// # 参数
    ///
    /// * `content` - 要检查的内容
    ///
    /// # 返回
    ///
    /// - `true`: 内容来自远程设备
    /// - `false`: 内容来自本地或不在历史中
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    ///
    /// guard.record_remote(b"Remote", "device-1");
    /// assert!(guard.is_from_remote(b"Remote"));
    ///
    /// guard.record_local(b"Local");
    /// assert!(!guard.is_from_remote(b"Local"));
    ///
    /// assert!(!guard.is_from_remote(b"Unknown"));
    /// ```
    pub fn is_from_remote(&self, content: &[u8]) -> bool {
        let fingerprint = ContentFingerprint::from_content(content);

        self.cleanup_expired();

        let history = self.history.read().unwrap();

        match history.get(&fingerprint) {
            Some(entry) => {
                !entry.is_expired(self.config.expiry_duration) && entry.origin.is_remote()
            }
            None => false,
        }
    }

    /// 获取内容来源
    ///
    /// # 参数
    ///
    /// * `content` - 要查询的内容
    ///
    /// # 返回
    ///
    /// 如果内容在历史中且未过期，返回其来源；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig, ContentOrigin};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    ///
    /// guard.record_remote(b"Remote", "device-1");
    /// let origin = guard.get_origin(b"Remote");
    /// assert!(matches!(origin, Some(ContentOrigin::Remote(_))));
    ///
    /// guard.record_local(b"Local");
    /// let origin = guard.get_origin(b"Local");
    /// assert!(matches!(origin, Some(ContentOrigin::Local)));
    /// ```
    pub fn get_origin(&self, content: &[u8]) -> Option<ContentOrigin> {
        let fingerprint = ContentFingerprint::from_content(content);

        self.cleanup_expired();

        let history = self.history.read().unwrap();

        history.get(&fingerprint).and_then(|entry| {
            if entry.is_expired(self.config.expiry_duration) {
                None
            } else {
                Some(entry.origin.clone())
            }
        })
    }

    /// 清空所有历史记录
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_sync::{LoopGuard, LoopGuardConfig};
    ///
    /// let guard = LoopGuard::new(LoopGuardConfig::new());
    /// guard.record_local(b"content");
    /// assert_eq!(guard.history_count(), 1);
    ///
    /// guard.clear();
    /// assert_eq!(guard.history_count(), 0);
    /// ```
    pub fn clear(&self) {
        let mut history = self.history.write().unwrap();
        let mut order = self.insertion_order.write().unwrap();

        history.clear();
        order.clear();

        tracing::debug!("Loop guard history cleared");
    }

    /// 获取当前历史记录数量
    ///
    /// 注意：返回的数量可能包含已过期但尚未清理的记录。
    pub fn history_count(&self) -> usize {
        self.history.read().unwrap().len()
    }

    /// 清理过期记录
    fn cleanup_expired(&self) {
        let mut history = self.history.write().unwrap();
        let mut order = self.insertion_order.write().unwrap();

        let expiry = self.config.expiry_duration;
        let before_count = history.len();

        // 收集过期的指纹
        let expired: Vec<ContentFingerprint> = history
            .iter()
            .filter(|(_, entry)| entry.is_expired(expiry))
            .map(|(fp, _)| *fp)
            .collect();

        // 移除过期记录
        for fp in &expired {
            history.remove(fp);
            order.retain(|f| f != fp);
        }

        let removed = before_count - history.len();
        if removed > 0 {
            tracing::trace!(
                removed = removed,
                remaining = history.len(),
                "Cleaned up expired entries"
            );
        }
    }
}

impl std::fmt::Debug for LoopGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoopGuard")
            .field("config", &self.config)
            .field("history_count", &self.history_count())
            .finish()
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------
    // ContentOrigin 测试
    // --------------------------------------------------------

    #[test]
    fn test_origin_local() {
        let origin = ContentOrigin::Local;
        assert!(origin.is_local());
        assert!(!origin.is_remote());
        assert_eq!(origin.device_id(), None);
    }

    #[test]
    fn test_origin_remote() {
        let origin = ContentOrigin::Remote("device-1".into());
        assert!(!origin.is_local());
        assert!(origin.is_remote());
        assert_eq!(origin.device_id(), Some("device-1"));
    }

    #[test]
    fn test_origin_equality() {
        assert_eq!(ContentOrigin::Local, ContentOrigin::Local);
        assert_eq!(
            ContentOrigin::Remote("a".into()),
            ContentOrigin::Remote("a".into())
        );
        assert_ne!(ContentOrigin::Local, ContentOrigin::Remote("a".into()));
        assert_ne!(
            ContentOrigin::Remote("a".into()),
            ContentOrigin::Remote("b".into())
        );
    }

    // --------------------------------------------------------
    // ContentFingerprint 测试
    // --------------------------------------------------------

    #[test]
    fn test_fingerprint_same_content() {
        let fp1 = ContentFingerprint::from_content(b"Hello, World!");
        let fp2 = ContentFingerprint::from_content(b"Hello, World!");
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_fingerprint_different_content() {
        let fp1 = ContentFingerprint::from_content(b"Hello");
        let fp2 = ContentFingerprint::from_content(b"World");
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn test_fingerprint_empty_content() {
        let fp = ContentFingerprint::from_content(b"");
        assert_eq!(fp.as_bytes().len(), 16);
    }

    #[test]
    fn test_fingerprint_large_content() {
        let content = vec![0u8; 1024 * 1024]; // 1MB
        let fp = ContentFingerprint::from_content(&content);
        assert_eq!(fp.as_bytes().len(), 16);
    }

    #[test]
    fn test_fingerprint_to_hex() {
        let fp = ContentFingerprint::from_content(b"test");
        let hex = fp.to_hex();
        assert_eq!(hex.len(), 32);
        // 验证是有效的十六进制
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_fingerprint_display() {
        let fp = ContentFingerprint::from_content(b"test");
        let display = format!("{}", fp);
        assert_eq!(display, fp.to_hex());
    }

    #[test]
    fn test_fingerprint_hash() {
        use std::collections::HashSet;

        let fp1 = ContentFingerprint::from_content(b"a");
        let fp2 = ContentFingerprint::from_content(b"b");
        let fp3 = ContentFingerprint::from_content(b"a");

        let mut set = HashSet::new();
        set.insert(fp1);
        set.insert(fp2);
        set.insert(fp3);

        assert_eq!(set.len(), 2); // fp1 和 fp3 相同
    }

    // --------------------------------------------------------
    // LoopGuardConfig 测试
    // --------------------------------------------------------

    #[test]
    fn test_config_default() {
        let config = LoopGuardConfig::new();
        assert_eq!(config.history_size, DEFAULT_HISTORY_SIZE);
        assert_eq!(
            config.expiry_duration,
            Duration::from_secs(DEFAULT_EXPIRY_SECS)
        );
    }

    #[test]
    fn test_config_builder() {
        let config = LoopGuardConfig::new()
            .with_history_size(50)
            .with_expiry_duration(Duration::from_secs(30));

        assert_eq!(config.history_size, 50);
        assert_eq!(config.expiry_duration, Duration::from_secs(30));
    }

    #[test]
    fn test_config_validate_success() {
        let config = LoopGuardConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_zero_history() {
        let config = LoopGuardConfig::new().with_history_size(0);
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(LoopGuardError::InvalidConfig(_))));
    }

    #[test]
    fn test_config_validate_zero_expiry() {
        let config = LoopGuardConfig::new().with_expiry_duration(Duration::ZERO);
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(LoopGuardError::InvalidConfig(_))));
    }

    // --------------------------------------------------------
    // LoopGuard 基本测试
    // --------------------------------------------------------

    #[test]
    fn test_guard_creation() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        assert_eq!(guard.history_count(), 0);
    }

    #[test]
    fn test_guard_record_local() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_local(b"local content");
        assert_eq!(guard.history_count(), 1);
    }

    #[test]
    fn test_guard_record_remote() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_remote(b"remote content", "device-1");
        assert_eq!(guard.history_count(), 1);
    }

    #[test]
    fn test_guard_should_sync_new_content() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        assert!(guard.should_sync(b"new content"));
    }

    #[test]
    fn test_guard_should_sync_local_content() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_local(b"local");
        assert!(guard.should_sync(b"local")); // 本地内容应该同步
    }

    #[test]
    fn test_guard_should_not_sync_remote_content() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_remote(b"remote", "device-1");
        assert!(!guard.should_sync(b"remote")); // 远程内容不应该同步
    }

    #[test]
    fn test_guard_is_from_remote() {
        let guard = LoopGuard::new(LoopGuardConfig::new());

        guard.record_remote(b"remote", "device-1");
        assert!(guard.is_from_remote(b"remote"));

        guard.record_local(b"local");
        assert!(!guard.is_from_remote(b"local"));

        assert!(!guard.is_from_remote(b"unknown"));
    }

    #[test]
    fn test_guard_get_origin() {
        let guard = LoopGuard::new(LoopGuardConfig::new());

        guard.record_remote(b"remote", "device-1");
        let origin = guard.get_origin(b"remote");
        assert!(matches!(origin, Some(ContentOrigin::Remote(ref id)) if id == "device-1"));

        guard.record_local(b"local");
        let origin = guard.get_origin(b"local");
        assert!(matches!(origin, Some(ContentOrigin::Local)));

        let origin = guard.get_origin(b"unknown");
        assert!(origin.is_none());
    }

    #[test]
    fn test_guard_clear() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_local(b"a");
        guard.record_remote(b"b", "device-1");
        assert_eq!(guard.history_count(), 2);

        guard.clear();
        assert_eq!(guard.history_count(), 0);
    }

    // --------------------------------------------------------
    // LRU 淘汰测试
    // --------------------------------------------------------

    #[test]
    fn test_guard_lru_eviction() {
        let config = LoopGuardConfig::new().with_history_size(3);
        let guard = LoopGuard::new(config);

        guard.record_local(b"a");
        guard.record_local(b"b");
        guard.record_local(b"c");
        assert_eq!(guard.history_count(), 3);

        // 添加第 4 个，应该淘汰最旧的 "a"
        guard.record_local(b"d");
        assert_eq!(guard.history_count(), 3);

        // "a" 应该不在历史中了
        assert!(guard.get_origin(b"a").is_none());
        // "b", "c", "d" 应该还在
        assert!(guard.get_origin(b"b").is_some());
        assert!(guard.get_origin(b"c").is_some());
        assert!(guard.get_origin(b"d").is_some());
    }

    #[test]
    fn test_guard_update_existing_moves_to_end() {
        let config = LoopGuardConfig::new().with_history_size(3);
        let guard = LoopGuard::new(config);

        guard.record_local(b"a");
        guard.record_local(b"b");
        guard.record_local(b"c");

        // 更新 "a"，应该移动到末尾
        guard.record_local(b"a");
        assert_eq!(guard.history_count(), 3);

        // 添加 "d"，应该淘汰 "b"（现在最旧）
        guard.record_local(b"d");
        assert_eq!(guard.history_count(), 3);

        assert!(guard.get_origin(b"b").is_none());
        assert!(guard.get_origin(b"a").is_some());
        assert!(guard.get_origin(b"c").is_some());
        assert!(guard.get_origin(b"d").is_some());
    }

    // --------------------------------------------------------
    // 过期测试
    // --------------------------------------------------------

    #[test]
    fn test_guard_expired_content_should_sync() {
        let config = LoopGuardConfig::new().with_expiry_duration(Duration::from_millis(50));
        let guard = LoopGuard::new(config);

        guard.record_remote(b"remote", "device-1");
        assert!(!guard.should_sync(b"remote"));

        // 等待过期
        std::thread::sleep(Duration::from_millis(60));

        // 过期后应该可以同步
        assert!(guard.should_sync(b"remote"));
    }

    #[test]
    fn test_guard_expired_is_not_from_remote() {
        let config = LoopGuardConfig::new().with_expiry_duration(Duration::from_millis(50));
        let guard = LoopGuard::new(config);

        guard.record_remote(b"remote", "device-1");
        assert!(guard.is_from_remote(b"remote"));

        std::thread::sleep(Duration::from_millis(60));

        assert!(!guard.is_from_remote(b"remote"));
    }

    #[test]
    fn test_guard_expired_origin_is_none() {
        let config = LoopGuardConfig::new().with_expiry_duration(Duration::from_millis(50));
        let guard = LoopGuard::new(config);

        guard.record_local(b"content");
        assert!(guard.get_origin(b"content").is_some());

        std::thread::sleep(Duration::from_millis(60));

        assert!(guard.get_origin(b"content").is_none());
    }

    // --------------------------------------------------------
    // 多设备测试
    // --------------------------------------------------------

    #[test]
    fn test_guard_multiple_devices() {
        let guard = LoopGuard::new(LoopGuardConfig::new());

        guard.record_remote(b"content-1", "device-1");
        guard.record_remote(b"content-2", "device-2");

        assert!(!guard.should_sync(b"content-1"));
        assert!(!guard.should_sync(b"content-2"));

        let origin1 = guard.get_origin(b"content-1");
        assert!(matches!(origin1, Some(ContentOrigin::Remote(ref id)) if id == "device-1"));

        let origin2 = guard.get_origin(b"content-2");
        assert!(matches!(origin2, Some(ContentOrigin::Remote(ref id)) if id == "device-2"));
    }

    #[test]
    fn test_guard_same_content_different_origin() {
        let guard = LoopGuard::new(LoopGuardConfig::new());

        // 先记录为远程
        guard.record_remote(b"content", "device-1");
        assert!(!guard.should_sync(b"content"));

        // 再记录为本地（覆盖）
        guard.record_local(b"content");
        assert!(guard.should_sync(b"content"));
    }

    // --------------------------------------------------------
    // Debug 测试
    // --------------------------------------------------------

    #[test]
    fn test_guard_debug() {
        let guard = LoopGuard::new(LoopGuardConfig::new());
        guard.record_local(b"test");
        let debug = format!("{:?}", guard);
        assert!(debug.contains("LoopGuard"));
        assert!(debug.contains("history_count"));
    }
}
