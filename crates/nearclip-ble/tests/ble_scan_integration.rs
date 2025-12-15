//! BLE Scanner Integration Tests
//!
//! 测试 BLE 中心模式扫描功能。
//! 标记 `#[ignore]` 的测试需要实际 BLE 硬件才能运行。

use nearclip_ble::{
    BleError, BleScanner, BleScannerConfig, DiscoveredDevice, DEFAULT_DEVICE_TIMEOUT_MS,
    DEFAULT_SCAN_TIMEOUT_MS, MAX_DEVICE_ID_LENGTH, PUBKEY_HASH_LENGTH,
};

// ==================== Config Tests ====================

#[test]
fn test_scanner_config_creation() {
    let config = BleScannerConfig::new();

    assert_eq!(config.scan_timeout_ms, DEFAULT_SCAN_TIMEOUT_MS);
    assert_eq!(config.device_timeout_ms, DEFAULT_DEVICE_TIMEOUT_MS);
    assert!(config.filter_nearclip_only);
}

#[test]
fn test_scanner_config_with_scan_timeout() {
    let config = BleScannerConfig::new().with_scan_timeout(10000);

    assert_eq!(config.scan_timeout_ms, 10000);
    assert_eq!(config.device_timeout_ms, DEFAULT_DEVICE_TIMEOUT_MS);
}

#[test]
fn test_scanner_config_with_device_timeout() {
    let config = BleScannerConfig::new().with_device_timeout(60000);

    assert_eq!(config.device_timeout_ms, 60000);
}

#[test]
fn test_scanner_config_with_filter_disabled() {
    let config = BleScannerConfig::new().with_filter_nearclip_only(false);

    assert!(!config.filter_nearclip_only);
}

#[test]
fn test_scanner_config_chained_builders() {
    let config = BleScannerConfig::new()
        .with_scan_timeout(5000)
        .with_device_timeout(15000)
        .with_filter_nearclip_only(false);

    assert_eq!(config.scan_timeout_ms, 5000);
    assert_eq!(config.device_timeout_ms, 15000);
    assert!(!config.filter_nearclip_only);
}

#[test]
fn test_scanner_config_validate_success() {
    let config = BleScannerConfig::new();
    assert!(config.validate().is_ok());
}

#[test]
fn test_scanner_config_validate_zero_device_timeout() {
    let mut config = BleScannerConfig::new();
    config.device_timeout_ms = 0;

    let result = config.validate();
    assert!(result.is_err());
    if let Err(BleError::Configuration(msg)) = result {
        assert!(msg.contains("device_timeout_ms"));
    } else {
        panic!("Expected Configuration error");
    }
}

#[test]
fn test_scanner_config_zero_scan_timeout_is_valid() {
    // 0 表示无限扫描，应该有效
    let config = BleScannerConfig::new().with_scan_timeout(0);
    assert!(config.validate().is_ok());
}

// ==================== DiscoveredDevice Tests ====================

const VALID_PUBKEY_HASH: &str = "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=";

#[test]
fn test_discovered_device_creation() {
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    assert_eq!(device.device_id, "device-001");
    assert_eq!(device.public_key_hash, VALID_PUBKEY_HASH);
    assert_eq!(device.rssi, -50);
    assert!(device.advertise_name.is_none());
    assert_eq!(device.platform_identifier, "platform-id");
}

#[test]
fn test_discovered_device_with_advertise_name() {
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    )
    .with_advertise_name("NearClip Device".to_string());

    assert_eq!(device.advertise_name, Some("NearClip Device".to_string()));
}

#[test]
fn test_discovered_device_validate_success() {
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    assert!(device.validate().is_ok());
}

#[test]
fn test_discovered_device_validate_empty_device_id() {
    let device = DiscoveredDevice::new(
        "".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    let result = device.validate();
    assert!(result.is_err());
    if let Err(BleError::Configuration(msg)) = result {
        assert!(msg.contains("device_id"));
    } else {
        panic!("Expected Configuration error");
    }
}

#[test]
fn test_discovered_device_validate_device_id_too_long() {
    let long_device_id = "a".repeat(MAX_DEVICE_ID_LENGTH + 1);
    let device = DiscoveredDevice::new(
        long_device_id,
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    let result = device.validate();
    assert!(result.is_err());
    if let Err(BleError::Configuration(msg)) = result {
        assert!(msg.contains("device_id"));
    } else {
        panic!("Expected Configuration error");
    }
}

#[test]
fn test_discovered_device_validate_invalid_pubkey_hash_length() {
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        "short".to_string(),
        -50,
        "platform-id".to_string(),
    );

    let result = device.validate();
    assert!(result.is_err());
    if let Err(BleError::Configuration(msg)) = result {
        assert!(msg.contains("public_key_hash"));
    } else {
        panic!("Expected Configuration error");
    }
}

#[test]
fn test_discovered_device_validate_invalid_base64() {
    // 44 characters but invalid Base64
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        "!!!invalid-base64-string-with-44-chars!!!!==".to_string(),
        -50,
        "platform-id".to_string(),
    );

    let result = device.validate();
    assert!(result.is_err());
    if let Err(BleError::Configuration(msg)) = result {
        assert!(msg.contains("Base64"));
    } else {
        panic!("Expected Configuration error");
    }
}

#[test]
fn test_discovered_device_device_id_max_length() {
    let max_device_id = "a".repeat(MAX_DEVICE_ID_LENGTH);
    let device = DiscoveredDevice::new(
        max_device_id.clone(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    assert!(device.validate().is_ok());
    assert_eq!(device.device_id, max_device_id);
}

#[test]
fn test_discovered_device_is_expired() {
    let device = DiscoveredDevice::new(
        "device-001".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -50,
        "platform-id".to_string(),
    );

    // 刚创建的设备不应该过期
    assert!(!device.is_expired(1000));
    assert!(!device.is_expired(30000));
}

#[test]
fn test_discovered_device_rssi_values() {
    // 测试各种 RSSI 值
    let strong = DiscoveredDevice::new(
        "strong".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -30,
        "platform-id".to_string(),
    );
    let medium = DiscoveredDevice::new(
        "medium".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -60,
        "platform-id".to_string(),
    );
    let weak = DiscoveredDevice::new(
        "weak".to_string(),
        VALID_PUBKEY_HASH.to_string(),
        -90,
        "platform-id".to_string(),
    );

    assert_eq!(strong.rssi, -30);
    assert_eq!(medium.rssi, -60);
    assert_eq!(weak.rssi, -90);
}

#[test]
fn test_pubkey_hash_length_constant() {
    // 验证 PUBKEY_HASH_LENGTH 常量值
    assert_eq!(PUBKEY_HASH_LENGTH, 44);
    assert_eq!(VALID_PUBKEY_HASH.len(), PUBKEY_HASH_LENGTH);
}

// ==================== Scanner Tests ====================

#[tokio::test]
async fn test_scanner_creation_success() {
    let config = BleScannerConfig::new();
    let scanner = BleScanner::new(config).await;

    assert!(scanner.is_ok());
}

#[tokio::test]
async fn test_scanner_creation_invalid_config() {
    let mut config = BleScannerConfig::new();
    config.device_timeout_ms = 0;

    let scanner = BleScanner::new(config).await;

    assert!(scanner.is_err());
}

#[tokio::test]
async fn test_scanner_initial_state() {
    let config = BleScannerConfig::new();
    let scanner = BleScanner::new(config).await.unwrap();

    assert!(!scanner.is_scanning().await);
    assert!(scanner.discovered_devices().await.is_empty());
}

#[tokio::test]
async fn test_scanner_config_accessor() {
    let config = BleScannerConfig::new()
        .with_scan_timeout(5000)
        .with_device_timeout(20000);

    let scanner = BleScanner::new(config).await.unwrap();

    assert_eq!(scanner.config().scan_timeout_ms, 5000);
    assert_eq!(scanner.config().device_timeout_ms, 20000);
}

#[tokio::test]
async fn test_scanner_stop_without_start() {
    let config = BleScannerConfig::new();
    let mut scanner = BleScanner::new(config).await.unwrap();

    // 未启动时停止应该成功（幂等操作）
    let result = scanner.stop().await;
    assert!(result.is_ok());
    assert!(!scanner.is_scanning().await);
}

#[tokio::test]
async fn test_scanner_double_stop() {
    let config = BleScannerConfig::new();
    let mut scanner = BleScanner::new(config).await.unwrap();

    // 连续停止应该都成功
    assert!(scanner.stop().await.is_ok());
    assert!(scanner.stop().await.is_ok());
}

#[tokio::test]
async fn test_scanner_subscribe() {
    let config = BleScannerConfig::new();
    let scanner = BleScanner::new(config).await.unwrap();

    // 订阅应该成功
    let _rx = scanner.subscribe();
    // 可以多次订阅
    let _rx2 = scanner.subscribe();
}

#[tokio::test]
async fn test_scanner_cleanup_expired_devices() {
    let config = BleScannerConfig::new();
    let scanner = BleScanner::new(config).await.unwrap();

    // 清理过期设备（即使为空也应该成功）
    scanner.cleanup_expired_devices().await;
    assert!(scanner.discovered_devices().await.is_empty());
}

#[tokio::test]
async fn test_scanner_concurrent_state_access() {
    use std::sync::Arc;
    use tokio::sync::Barrier;

    let config = BleScannerConfig::new();
    let scanner = Arc::new(BleScanner::new(config).await.unwrap());
    let barrier = Arc::new(Barrier::new(3));

    let mut handles = vec![];

    // 并发读取状态
    for _ in 0..3 {
        let s = Arc::clone(&scanner);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            b.wait().await;
            let _ = s.is_scanning().await;
            let _ = s.discovered_devices().await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

// ==================== BLE Hardware Tests (Ignored) ====================

#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_scanner_start_and_stop() {
    let config = BleScannerConfig::new().with_scan_timeout(5000);
    let mut scanner = BleScanner::new(config).await.unwrap();

    // 启动扫描
    let result = scanner.start().await;
    assert!(result.is_ok());
    assert!(scanner.is_scanning().await);

    // 停止扫描
    let result = scanner.stop().await;
    assert!(result.is_ok());
    assert!(!scanner.is_scanning().await);
}

#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_scanner_double_start() {
    let config = BleScannerConfig::new();
    let mut scanner = BleScanner::new(config).await.unwrap();

    // 第一次启动
    let result = scanner.start().await;
    assert!(result.is_ok());

    // 重复启动应该成功（幂等操作）
    let result = scanner.start().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_scanner_lifecycle() {
    let config = BleScannerConfig::new().with_scan_timeout(3000);
    let mut scanner = BleScanner::new(config).await.unwrap();

    // 完整生命周期测试
    assert!(!scanner.is_scanning().await);

    scanner.start().await.unwrap();
    assert!(scanner.is_scanning().await);

    // 等待一段时间
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    scanner.stop().await.unwrap();
    assert!(!scanner.is_scanning().await);
}

#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_scanner_device_discovery() {
    let config = BleScannerConfig::new()
        .with_scan_timeout(10000)
        .with_filter_nearclip_only(true);

    let mut scanner = BleScanner::new(config).await.unwrap();
    let mut rx = scanner.subscribe();

    scanner.start().await.unwrap();

    // 等待设备发现事件（需要附近有 NearClip 设备）
    tokio::select! {
        device = rx.recv() => {
            if let Ok(device) = device {
                assert!(!device.device_id.is_empty());
                assert_eq!(device.public_key_hash.len(), PUBKEY_HASH_LENGTH);
            }
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
            // 超时，可能没有可用设备
        }
    }

    scanner.stop().await.unwrap();
}
