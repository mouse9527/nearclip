//! BLE 广播集成测试
//!
//! 这些测试验证 BLE 设备广播的完整流程。
//!
//! 注意：BLE 相关的测试在某些环境中可能不稳定，
//! 需要 BLE 硬件支持的测试被标记为 `#[ignore]`，需要手动运行：
//! `cargo test -p nearclip-ble --test ble_advertise_integration -- --ignored`

use nearclip_ble::{
    BleAdvertiser, BleAdvertiserConfig, BleError, DEFAULT_ADVERTISE_NAME,
    DEVICE_ID_CHARACTERISTIC_UUID, MAX_ADVERTISE_NAME_LENGTH, MAX_DEVICE_ID_LENGTH,
    NEARCLIP_SERVICE_UUID, PUBKEY_HASH_CHARACTERISTIC_UUID,
};

// ============================================================
// BleAdvertiserConfig 测试
// ============================================================

#[test]
fn test_advertiser_config_creation() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    assert_eq!(config.device_id, "test-device");
    assert_eq!(config.public_key_hash, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=");
    assert!(config.advertise_name.is_none());
    assert!(config.validate().is_ok());
}

#[test]
fn test_advertiser_config_with_custom_name() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
        .with_name("CustomName".to_string());

    assert_eq!(config.advertise_name, Some("CustomName".to_string()));
    assert_eq!(config.effective_name(), "CustomName");
    assert!(config.validate().is_ok());
}

#[test]
fn test_advertiser_config_default_name() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    assert_eq!(config.effective_name(), DEFAULT_ADVERTISE_NAME);
    assert_eq!(config.effective_name(), "NearClip");
}

#[test]
fn test_advertiser_config_empty_device_id() {
    let config = BleAdvertiserConfig::new("".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let result = config.validate();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, BleError::Configuration(_)));
    assert!(err.to_string().contains("device_id cannot be empty"));
}

#[test]
fn test_advertiser_config_empty_public_key_hash() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "".to_string());

    let result = config.validate();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, BleError::Configuration(_)));
    assert!(err.to_string().contains("public_key_hash cannot be empty"));
}

#[test]
fn test_advertiser_config_device_id_max_length() {
    // 测试边界值 - 刚好在限制内
    let max_id = "x".repeat(MAX_DEVICE_ID_LENGTH);
    let config = BleAdvertiserConfig::new(max_id, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());
    assert!(config.validate().is_ok());

    // 测试超出限制
    let too_long_id = "x".repeat(MAX_DEVICE_ID_LENGTH + 1);
    let config = BleAdvertiserConfig::new(too_long_id, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());
    assert!(config.validate().is_err());
}

#[test]
fn test_advertiser_config_name_max_length() {
    // 测试边界值 - 刚好在限制内
    let max_name = "x".repeat(MAX_ADVERTISE_NAME_LENGTH);
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
        .with_name(max_name);
    assert!(config.validate().is_ok());

    // 测试超出限制
    let too_long_name = "x".repeat(MAX_ADVERTISE_NAME_LENGTH + 1);
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
        .with_name(too_long_name);
    assert!(config.validate().is_err());
}

#[test]
fn test_advertiser_config_invalid_public_key_hash_length() {
    // 测试错误长度的 public_key_hash（应为 44 字符）
    let short_hash = "dGVzdA=="; // 8 字符
    let config = BleAdvertiserConfig::new("test-device".to_string(), short_hash.to_string());

    let result = config.validate();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, BleError::Configuration(_)));
    assert!(err.to_string().contains("must be 44 characters"));
}

#[test]
fn test_advertiser_config_invalid_base64() {
    // 测试无效的 Base64 格式（正确长度但无效字符）
    let invalid_base64 = "!!!invalid-base64-string-with-44-chars!!!!==";
    assert_eq!(invalid_base64.len(), 44); // 确保长度正确

    let config = BleAdvertiserConfig::new("test-device".to_string(), invalid_base64.to_string());

    let result = config.validate();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(matches!(err, BleError::Configuration(_)));
    assert!(err.to_string().contains("valid Base64"));
}

// ============================================================
// GATT 常量测试
// ============================================================

#[test]
fn test_gatt_uuid_constants() {
    // 验证 UUID 是有效的 128-bit UUID
    assert_eq!(NEARCLIP_SERVICE_UUID.as_bytes().len(), 16);
    assert_eq!(DEVICE_ID_CHARACTERISTIC_UUID.as_bytes().len(), 16);
    assert_eq!(PUBKEY_HASH_CHARACTERISTIC_UUID.as_bytes().len(), 16);
}

#[test]
fn test_gatt_uuids_are_unique() {
    // 确保所有 UUID 都是唯一的
    assert_ne!(NEARCLIP_SERVICE_UUID, DEVICE_ID_CHARACTERISTIC_UUID);
    assert_ne!(NEARCLIP_SERVICE_UUID, PUBKEY_HASH_CHARACTERISTIC_UUID);
    assert_ne!(
        DEVICE_ID_CHARACTERISTIC_UUID,
        PUBKEY_HASH_CHARACTERISTIC_UUID
    );
}

#[test]
fn test_gatt_uuid_string_format() {
    // 验证 UUID 字符串格式正确
    let service_uuid = NEARCLIP_SERVICE_UUID.to_string();
    assert!(service_uuid.contains("-"));
    assert_eq!(service_uuid.len(), 36); // UUID 字符串长度 (含连字符)

    // 验证是我们定义的自定义 UUID
    assert!(service_uuid.starts_with("4e454152")); // "NEAR" in hex
}

// ============================================================
// BleError 测试
// ============================================================

#[test]
fn test_ble_error_variants() {
    let init_err = BleError::Initialization("test".to_string());
    assert!(init_err.to_string().contains("BLE initialization failed"));

    let not_powered = BleError::NotPowered;
    assert!(not_powered.to_string().contains("not powered on"));

    let adv_err = BleError::Advertising("failed".to_string());
    assert!(adv_err.to_string().contains("Advertising failed"));

    let service_err = BleError::ServiceRegistration("invalid".to_string());
    assert!(service_err.to_string().contains("Service registration failed"));

    let config_err = BleError::Configuration("bad config".to_string());
    assert!(config_err.to_string().contains("Configuration error"));

    let platform_err = BleError::PlatformNotSupported;
    assert!(platform_err.to_string().contains("Platform not supported"));
}

// ============================================================
// BleAdvertiser 测试
// ============================================================

#[tokio::test]
async fn test_advertiser_creation_success() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let advertiser = BleAdvertiser::new(config).await;
    assert!(advertiser.is_ok());

    let advertiser = advertiser.unwrap();
    assert!(!advertiser.is_advertising().await);
    assert_eq!(advertiser.config().device_id, "test-device");
}

#[tokio::test]
async fn test_advertiser_creation_invalid_config() {
    let config = BleAdvertiserConfig::new("".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let advertiser = BleAdvertiser::new(config).await;
    assert!(advertiser.is_err());

    let err = advertiser.unwrap_err();
    assert!(matches!(err, BleError::Configuration(_)));
}

#[tokio::test]
async fn test_advertiser_stop_without_start() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let mut advertiser = BleAdvertiser::new(config).await.unwrap();

    // 停止未启动的广播应该成功（幂等操作）
    let result = advertiser.stop().await;
    assert!(result.is_ok());
    assert!(!advertiser.is_advertising().await);
}

#[tokio::test]
async fn test_advertiser_double_stop() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let mut advertiser = BleAdvertiser::new(config).await.unwrap();

    // 多次停止应该安全
    assert!(advertiser.stop().await.is_ok());
    assert!(advertiser.stop().await.is_ok());
    assert!(!advertiser.is_advertising().await);
}

#[tokio::test]
async fn test_advertiser_config_accessor() {
    let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
        .with_name("TestDevice".to_string());

    let advertiser = BleAdvertiser::new(config).await.unwrap();

    assert_eq!(advertiser.config().device_id, "device-001");
    assert_eq!(advertiser.config().public_key_hash, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=");
    assert_eq!(
        advertiser.config().advertise_name,
        Some("TestDevice".to_string())
    );
    assert_eq!(advertiser.config().effective_name(), "TestDevice");
}

// ============================================================
// BLE 硬件相关测试 (需要手动运行)
// ============================================================

/// 测试启动和停止广播
///
/// 此测试需要 BLE 硬件支持
#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_advertiser_start_and_stop() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let mut advertiser = BleAdvertiser::new(config).await.unwrap();

    assert!(!advertiser.is_advertising().await);

    // 启动广播
    let start_result = advertiser.start().await;

    // 在支持的平台上应该成功
    if start_result.is_ok() {
        assert!(advertiser.is_advertising().await);

        // 停止广播
        advertiser.stop().await.unwrap();
        assert!(!advertiser.is_advertising().await);
    } else {
        // 在不支持的平台上返回 PlatformNotSupported
        let err = start_result.unwrap_err();
        assert!(matches!(err, BleError::PlatformNotSupported));
    }
}

/// 测试重复启动广播
///
/// 此测试需要 BLE 硬件支持
#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_advertiser_double_start() {
    let config = BleAdvertiserConfig::new("test-device".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let mut advertiser = BleAdvertiser::new(config).await.unwrap();

    // 第一次启动
    let first_result = advertiser.start().await;

    if first_result.is_ok() {
        // 第二次启动应该安全（幂等操作）
        let second_result = advertiser.start().await;
        assert!(second_result.is_ok());
        assert!(advertiser.is_advertising().await);

        advertiser.stop().await.unwrap();
    }
}

/// 测试广播器生命周期
///
/// 此测试需要 BLE 硬件支持
#[tokio::test]
#[ignore = "requires BLE hardware"]
async fn test_advertiser_lifecycle() {
    let config = BleAdvertiserConfig::new("lifecycle-test".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

    let mut advertiser = BleAdvertiser::new(config).await.unwrap();

    // 启动 -> 停止 -> 启动 -> 停止
    if advertiser.start().await.is_ok() {
        assert!(advertiser.is_advertising().await);

        advertiser.stop().await.unwrap();
        assert!(!advertiser.is_advertising().await);

        advertiser.start().await.unwrap();
        assert!(advertiser.is_advertising().await);

        advertiser.stop().await.unwrap();
        assert!(!advertiser.is_advertising().await);
    }
}

// ============================================================
// 并发访问测试
// ============================================================

/// 测试多任务并发访问 is_advertising 状态
#[tokio::test]
async fn test_advertiser_concurrent_state_access() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let config = BleAdvertiserConfig::new(
        "concurrent-test".to_string(),
        "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    );

    let advertiser = Arc::new(Mutex::new(BleAdvertiser::new(config).await.unwrap()));

    // 启动多个并发任务读取状态
    let mut handles = vec![];

    for _ in 0..10 {
        let adv = Arc::clone(&advertiser);
        handles.push(tokio::spawn(async move {
            let guard = adv.lock().await;
            guard.is_advertising().await
        }));
    }

    // 所有任务应该成功完成且返回 false（未广播）
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(!result);
    }
}

/// 测试多任务并发调用 stop（幂等性）
#[tokio::test]
async fn test_advertiser_concurrent_stop_calls() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let config = BleAdvertiserConfig::new(
        "concurrent-stop-test".to_string(),
        "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    );

    let advertiser = Arc::new(Mutex::new(BleAdvertiser::new(config).await.unwrap()));

    // 并发调用多次 stop - 应该都成功（幂等操作）
    let mut handles = vec![];

    for _ in 0..5 {
        let adv = Arc::clone(&advertiser);
        handles.push(tokio::spawn(async move {
            let mut guard = adv.lock().await;
            guard.stop().await
        }));
    }

    // 所有 stop 调用应该成功
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // 最终状态应该是未广播
    let guard = advertiser.lock().await;
    assert!(!guard.is_advertising().await);
}
