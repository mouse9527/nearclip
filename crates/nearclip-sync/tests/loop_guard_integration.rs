//! 循环防护集成测试
//!
//! 这些测试验证循环防护的完整流程。
//!
//! 主要测试:
//! - 本地内容应该同步
//! - 远程内容不应该回传
//! - 完整防循环流程
//! - 多设备场景

use nearclip_sync::{
    ContentFingerprint, ContentOrigin, LoopGuard, LoopGuardConfig, LoopGuardError,
    DEFAULT_EXPIRY_SECS, DEFAULT_HISTORY_SIZE,
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================================
// 常量测试
// ============================================================

#[test]
fn test_default_constants() {
    assert_eq!(DEFAULT_HISTORY_SIZE, 100);
    assert_eq!(DEFAULT_EXPIRY_SECS, 60);
}

// ============================================================
// LoopGuardConfig 测试
// ============================================================

#[test]
fn test_config_default_values() {
    let config = LoopGuardConfig::new();
    assert_eq!(config.history_size, DEFAULT_HISTORY_SIZE);
    assert_eq!(config.expiry_duration, Duration::from_secs(DEFAULT_EXPIRY_SECS));
}

#[test]
fn test_config_builder_chain() {
    let config = LoopGuardConfig::new()
        .with_history_size(50)
        .with_expiry_duration(Duration::from_secs(30));

    assert_eq!(config.history_size, 50);
    assert_eq!(config.expiry_duration, Duration::from_secs(30));
}

#[test]
fn test_config_validation_success() {
    let config = LoopGuardConfig::new();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_zero_history() {
    let config = LoopGuardConfig::new().with_history_size(0);
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(LoopGuardError::InvalidConfig(_))));
}

#[test]
fn test_config_validation_zero_expiry() {
    let config = LoopGuardConfig::new().with_expiry_duration(Duration::ZERO);
    let result = config.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(LoopGuardError::InvalidConfig(_))));
}

// ============================================================
// ContentFingerprint 测试
// ============================================================

#[test]
fn test_fingerprint_same_content() {
    let content = b"Hello, World!";
    let fp1 = ContentFingerprint::from_content(content);
    let fp2 = ContentFingerprint::from_content(content);
    assert_eq!(fp1, fp2);
}

#[test]
fn test_fingerprint_different_content() {
    let fp1 = ContentFingerprint::from_content(b"Hello");
    let fp2 = ContentFingerprint::from_content(b"World");
    assert_ne!(fp1, fp2);
}

#[test]
fn test_fingerprint_case_sensitive() {
    let fp1 = ContentFingerprint::from_content(b"Hello");
    let fp2 = ContentFingerprint::from_content(b"hello");
    assert_ne!(fp1, fp2);
}

#[test]
fn test_fingerprint_whitespace_sensitive() {
    let fp1 = ContentFingerprint::from_content(b"Hello World");
    let fp2 = ContentFingerprint::from_content(b"Hello  World");
    assert_ne!(fp1, fp2);
}

#[test]
fn test_fingerprint_empty_content() {
    let fp = ContentFingerprint::from_content(b"");
    assert_eq!(fp.as_bytes().len(), 16);
}

#[test]
fn test_fingerprint_large_content() {
    let content = vec![b'A'; 10 * 1024 * 1024]; // 10MB
    let fp = ContentFingerprint::from_content(&content);
    assert_eq!(fp.as_bytes().len(), 16);
}

#[test]
fn test_fingerprint_unicode_content() {
    let fp1 = ContentFingerprint::from_content("你好世界".as_bytes());
    let fp2 = ContentFingerprint::from_content("你好世界".as_bytes());
    let fp3 = ContentFingerprint::from_content("Hello World".as_bytes());
    assert_eq!(fp1, fp2);
    assert_ne!(fp1, fp3);
}

#[test]
fn test_fingerprint_to_hex() {
    let fp = ContentFingerprint::from_content(b"test");
    let hex = fp.to_hex();
    assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

// ============================================================
// ContentOrigin 测试
// ============================================================

#[test]
fn test_origin_local() {
    let origin = ContentOrigin::Local;
    assert!(origin.is_local());
    assert!(!origin.is_remote());
    assert_eq!(origin.device_id(), None);
}

#[test]
fn test_origin_remote() {
    let origin = ContentOrigin::Remote("device-abc".into());
    assert!(!origin.is_local());
    assert!(origin.is_remote());
    assert_eq!(origin.device_id(), Some("device-abc"));
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

// ============================================================
// LoopGuard 基本功能测试
// ============================================================

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
fn test_guard_clear() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_local(b"a");
    guard.record_remote(b"b", "device-1");
    assert_eq!(guard.history_count(), 2);

    guard.clear();
    assert_eq!(guard.history_count(), 0);
}

// ============================================================
// 核心防循环逻辑测试
// ============================================================

#[test]
fn test_local_content_should_sync() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    // 新内容应该同步
    assert!(guard.should_sync(b"new local content"));

    // 记录为本地后仍应该同步
    guard.record_local(b"new local content");
    assert!(guard.should_sync(b"new local content"));
}

#[test]
fn test_remote_content_should_not_sync() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    let content = b"content from device-1";

    // 记录为远程内容
    guard.record_remote(content, "device-1");

    // 不应该同步（防止循环）
    assert!(!guard.should_sync(content));
}

#[test]
fn test_complete_loop_prevention_flow() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    // 场景：设备 A 发送内容到设备 B
    let content = b"Clipboard from device A";

    // 步骤 1: 设备 B 收到远程内容
    guard.record_remote(content, "device-A");

    // 步骤 2: 设备 B 写入本地剪贴板
    // (这会触发本地剪贴板变化检测)

    // 步骤 3: 检测到变化后检查是否应该同步
    assert!(!guard.should_sync(content)); // 不应该同步！

    // 步骤 4: 本地新内容应该同步
    let new_content = b"New content typed by user on device B";
    assert!(guard.should_sync(new_content));
}

#[test]
fn test_new_content_after_remote() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    // 收到远程内容
    guard.record_remote(b"remote", "device-1");
    assert!(!guard.should_sync(b"remote"));

    // 用户输入新内容
    let new_content = b"user typed something new";
    assert!(guard.should_sync(new_content)); // 新内容应该同步
}

// ============================================================
// is_from_remote 测试
// ============================================================

#[test]
fn test_is_from_remote_true() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_remote(b"remote content", "device-1");
    assert!(guard.is_from_remote(b"remote content"));
}

#[test]
fn test_is_from_remote_false_for_local() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_local(b"local content");
    assert!(!guard.is_from_remote(b"local content"));
}

#[test]
fn test_is_from_remote_false_for_unknown() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    assert!(!guard.is_from_remote(b"unknown content"));
}

// ============================================================
// get_origin 测试
// ============================================================

#[test]
fn test_get_origin_local() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_local(b"local");

    let origin = guard.get_origin(b"local");
    assert!(matches!(origin, Some(ContentOrigin::Local)));
}

#[test]
fn test_get_origin_remote() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_remote(b"remote", "device-xyz");

    let origin = guard.get_origin(b"remote");
    assert!(matches!(origin, Some(ContentOrigin::Remote(ref id)) if id == "device-xyz"));
}

#[test]
fn test_get_origin_unknown() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    let origin = guard.get_origin(b"unknown");
    assert!(origin.is_none());
}

// ============================================================
// LRU 淘汰测试
// ============================================================

#[test]
fn test_lru_eviction() {
    let config = LoopGuardConfig::new().with_history_size(3);
    let guard = LoopGuard::new(config);

    guard.record_local(b"first");
    guard.record_local(b"second");
    guard.record_local(b"third");
    assert_eq!(guard.history_count(), 3);

    // 添加第 4 个，应该淘汰 "first"
    guard.record_local(b"fourth");
    assert_eq!(guard.history_count(), 3);

    assert!(guard.get_origin(b"first").is_none());
    assert!(guard.get_origin(b"second").is_some());
    assert!(guard.get_origin(b"third").is_some());
    assert!(guard.get_origin(b"fourth").is_some());
}

#[test]
fn test_lru_access_updates_order() {
    let config = LoopGuardConfig::new().with_history_size(3);
    let guard = LoopGuard::new(config);

    guard.record_local(b"a"); // oldest
    guard.record_local(b"b");
    guard.record_local(b"c");

    // 重新记录 "a"，使其成为最新
    guard.record_local(b"a");

    // 添加 "d"，应该淘汰 "b"（现在是最旧的）
    guard.record_local(b"d");

    assert!(guard.get_origin(b"b").is_none());
    assert!(guard.get_origin(b"a").is_some());
    assert!(guard.get_origin(b"c").is_some());
    assert!(guard.get_origin(b"d").is_some());
}

#[test]
fn test_lru_with_mixed_origins() {
    let config = LoopGuardConfig::new().with_history_size(3);
    let guard = LoopGuard::new(config);

    guard.record_local(b"local-1");
    guard.record_remote(b"remote-1", "device-1");
    guard.record_local(b"local-2");

    // 添加新记录，淘汰 "local-1"
    guard.record_remote(b"remote-2", "device-2");

    assert!(guard.get_origin(b"local-1").is_none());
    assert!(guard.get_origin(b"remote-1").is_some());
    assert!(guard.get_origin(b"local-2").is_some());
    assert!(guard.get_origin(b"remote-2").is_some());
}

// ============================================================
// 过期测试
// ============================================================

#[test]
fn test_expired_remote_should_sync() {
    let config = LoopGuardConfig::new()
        .with_expiry_duration(Duration::from_millis(50));
    let guard = LoopGuard::new(config);

    guard.record_remote(b"remote", "device-1");
    assert!(!guard.should_sync(b"remote"));

    // 等待过期
    thread::sleep(Duration::from_millis(60));

    // 过期后应该可以同步
    assert!(guard.should_sync(b"remote"));
}

#[test]
fn test_expired_is_not_from_remote() {
    let config = LoopGuardConfig::new()
        .with_expiry_duration(Duration::from_millis(50));
    let guard = LoopGuard::new(config);

    guard.record_remote(b"remote", "device-1");
    assert!(guard.is_from_remote(b"remote"));

    thread::sleep(Duration::from_millis(60));

    assert!(!guard.is_from_remote(b"remote"));
}

#[test]
fn test_expired_origin_is_none() {
    let config = LoopGuardConfig::new()
        .with_expiry_duration(Duration::from_millis(50));
    let guard = LoopGuard::new(config);

    guard.record_local(b"content");
    assert!(guard.get_origin(b"content").is_some());

    thread::sleep(Duration::from_millis(60));

    assert!(guard.get_origin(b"content").is_none());
}

#[test]
fn test_refresh_content_resets_expiry() {
    let config = LoopGuardConfig::new()
        .with_expiry_duration(Duration::from_millis(100));
    let guard = LoopGuard::new(config);

    guard.record_remote(b"content", "device-1");

    // 等待一半时间
    thread::sleep(Duration::from_millis(60));

    // 刷新记录
    guard.record_remote(b"content", "device-1");

    // 再等待一半时间
    thread::sleep(Duration::from_millis(60));

    // 应该仍然有效
    assert!(guard.get_origin(b"content").is_some());
    assert!(!guard.should_sync(b"content"));
}

// ============================================================
// 多设备场景测试
// ============================================================

#[test]
fn test_multiple_devices() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    guard.record_remote(b"from-1", "device-1");
    guard.record_remote(b"from-2", "device-2");
    guard.record_remote(b"from-3", "device-3");

    assert!(!guard.should_sync(b"from-1"));
    assert!(!guard.should_sync(b"from-2"));
    assert!(!guard.should_sync(b"from-3"));

    let origin1 = guard.get_origin(b"from-1");
    assert!(matches!(origin1, Some(ContentOrigin::Remote(ref id)) if id == "device-1"));

    let origin2 = guard.get_origin(b"from-2");
    assert!(matches!(origin2, Some(ContentOrigin::Remote(ref id)) if id == "device-2"));

    let origin3 = guard.get_origin(b"from-3");
    assert!(matches!(origin3, Some(ContentOrigin::Remote(ref id)) if id == "device-3"));
}

#[test]
fn test_same_content_from_different_devices() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    let content = b"shared content";

    // 先从 device-1 收到
    guard.record_remote(content, "device-1");
    let origin = guard.get_origin(content);
    assert!(matches!(origin, Some(ContentOrigin::Remote(ref id)) if id == "device-1"));

    // 再从 device-2 收到（覆盖）
    guard.record_remote(content, "device-2");
    let origin = guard.get_origin(content);
    assert!(matches!(origin, Some(ContentOrigin::Remote(ref id)) if id == "device-2"));
}

#[test]
fn test_remote_then_local_overwrite() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    let content = b"content";

    // 先记录为远程
    guard.record_remote(content, "device-1");
    assert!(!guard.should_sync(content));

    // 再记录为本地（用户可能重新复制了相同内容）
    guard.record_local(content);
    assert!(guard.should_sync(content));
}

// ============================================================
// 并发测试
// ============================================================

#[test]
fn test_concurrent_reads() {
    let guard = Arc::new(LoopGuard::new(LoopGuardConfig::new()));

    guard.record_local(b"content");

    let mut handles = vec![];
    for _ in 0..10 {
        let g = Arc::clone(&guard);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _ = g.should_sync(b"content");
                let _ = g.is_from_remote(b"content");
                let _ = g.get_origin(b"content");
                let _ = g.history_count();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_concurrent_writes() {
    let guard = Arc::new(LoopGuard::new(LoopGuardConfig::new()));

    let mut handles = vec![];
    for i in 0..10 {
        let g = Arc::clone(&guard);
        handles.push(thread::spawn(move || {
            for j in 0..100 {
                let content = format!("content-{}-{}", i, j);
                if j % 2 == 0 {
                    g.record_local(content.as_bytes());
                } else {
                    g.record_remote(content.as_bytes(), &format!("device-{}", i));
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // 应该有一些记录（可能被 LRU 淘汰了一些）
    assert!(guard.history_count() > 0);
}

#[test]
fn test_concurrent_read_write() {
    let guard = Arc::new(LoopGuard::new(LoopGuardConfig::new().with_history_size(50)));

    let mut handles = vec![];

    // 写入线程
    for i in 0..5 {
        let g = Arc::clone(&guard);
        handles.push(thread::spawn(move || {
            for j in 0..50 {
                let content = format!("content-{}-{}", i, j);
                g.record_remote(content.as_bytes(), &format!("device-{}", i));
            }
        }));
    }

    // 读取线程
    for _ in 0..5 {
        let g = Arc::clone(&guard);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _ = g.should_sync(b"test");
                let _ = g.history_count();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

// ============================================================
// 边界条件测试
// ============================================================

#[test]
fn test_empty_content() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    guard.record_local(b"");
    assert_eq!(guard.history_count(), 1);
    assert!(guard.should_sync(b""));

    guard.record_remote(b"", "device-1");
    assert!(!guard.should_sync(b""));
}

#[test]
fn test_binary_content() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    let binary: Vec<u8> = (0..=255).collect();
    guard.record_remote(&binary, "device-1");

    assert!(!guard.should_sync(&binary));
    assert!(guard.is_from_remote(&binary));
}

#[test]
fn test_history_size_one() {
    let config = LoopGuardConfig::new().with_history_size(1);
    let guard = LoopGuard::new(config);

    guard.record_local(b"first");
    assert_eq!(guard.history_count(), 1);

    guard.record_local(b"second");
    assert_eq!(guard.history_count(), 1);

    assert!(guard.get_origin(b"first").is_none());
    assert!(guard.get_origin(b"second").is_some());
}

#[test]
fn test_very_short_expiry() {
    let config = LoopGuardConfig::new()
        .with_expiry_duration(Duration::from_millis(1));
    let guard = LoopGuard::new(config);

    guard.record_remote(b"content", "device-1");

    // 短暂等待确保过期
    thread::sleep(Duration::from_millis(5));

    // 应该已过期
    assert!(guard.should_sync(b"content"));
}

// ============================================================
// Debug 输出测试
// ============================================================

#[test]
fn test_guard_debug_output() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    guard.record_local(b"test");

    let debug = format!("{:?}", guard);
    assert!(debug.contains("LoopGuard"));
    assert!(debug.contains("config"));
    assert!(debug.contains("history_count"));
}

#[test]
fn test_origin_debug_output() {
    let local = format!("{:?}", ContentOrigin::Local);
    assert!(local.contains("Local"));

    let remote = format!("{:?}", ContentOrigin::Remote("device-1".into()));
    assert!(remote.contains("Remote"));
    assert!(remote.contains("device-1"));
}

#[test]
fn test_fingerprint_display() {
    let fp = ContentFingerprint::from_content(b"test");
    let display = format!("{}", fp);
    assert_eq!(display.len(), 32);
    assert_eq!(display, fp.to_hex());
}

// ============================================================
// 实际使用场景测试
// ============================================================

#[test]
fn test_realistic_sync_scenario() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    // 场景模拟：
    // 1. 用户在设备 A 复制 "Hello"
    let content_a = b"Hello";
    guard.record_local(content_a);
    assert!(guard.should_sync(content_a)); // 应该同步到其他设备

    // 2. 设备 B 收到并写入剪贴板
    // (在设备 B 上)
    let guard_b = LoopGuard::new(LoopGuardConfig::new());
    guard_b.record_remote(content_a, "device-A");
    assert!(!guard_b.should_sync(content_a)); // 不应该回传给 A

    // 3. 用户在设备 B 复制新内容
    let content_b = b"World";
    assert!(guard_b.should_sync(content_b)); // 新内容应该同步
    guard_b.record_local(content_b);

    // 4. 设备 A 收到 "World"
    guard.record_remote(content_b, "device-B");
    assert!(!guard.should_sync(content_b)); // 不应该回传给 B
}

#[test]
fn test_rapid_content_changes() {
    let guard = LoopGuard::new(LoopGuardConfig::new());

    // 模拟快速连续复制
    for i in 0..10 {
        let content = format!("content-{}", i);
        guard.record_local(content.as_bytes());
        assert!(guard.should_sync(content.as_bytes()));
    }

    assert_eq!(guard.history_count(), 10);
}

#[test]
fn test_same_content_multiple_times() {
    let guard = LoopGuard::new(LoopGuardConfig::new());
    let content = b"repeated content";

    // 用户多次复制相同内容
    for _ in 0..5 {
        guard.record_local(content);
        assert!(guard.should_sync(content));
    }

    // 只有一条记录（因为内容相同）
    assert_eq!(guard.history_count(), 1);
}
