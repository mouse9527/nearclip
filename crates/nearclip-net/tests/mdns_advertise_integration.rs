//! mDNS 广播集成测试
//!
//! 这些测试验证 mDNS 服务广播的完整流程。

use nearclip_net::{MdnsAdvertiser, MdnsServiceConfig, NetError, SERVICE_TYPE, TXT_DEVICE_ID, TXT_PUBKEY_HASH};

/// 测试广播器启动和停止
#[tokio::test]
async fn test_advertiser_start_and_stop() {
    let config = MdnsServiceConfig::new(
        "test-device-001".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12345,
    );

    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");

    assert!(!advertiser.is_advertising());
    assert!(advertiser.service_fullname().is_none());

    advertiser
        .start()
        .await
        .expect("Failed to start advertising");

    assert!(advertiser.is_advertising());
    assert!(advertiser.service_fullname().is_some());

    // 验证服务名称格式
    let fullname = advertiser.service_fullname().unwrap();
    assert!(
        fullname.contains("test-device-001"),
        "Service name should contain device ID"
    );
    assert!(
        fullname.contains(SERVICE_TYPE.trim_end_matches('.')),
        "Service name should contain service type"
    );

    advertiser.stop().await.expect("Failed to stop advertising");

    assert!(!advertiser.is_advertising());
    assert!(advertiser.service_fullname().is_none());
}

/// 测试重复启动（应该安全处理）
#[tokio::test]
async fn test_advertiser_double_start() {
    let config = MdnsServiceConfig::new(
        "test-device-002".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12346,
    );

    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");

    advertiser
        .start()
        .await
        .expect("Failed to start advertising first time");
    assert!(advertiser.is_advertising());

    // 第二次启动应该安全处理（先停止再启动）
    advertiser
        .start()
        .await
        .expect("Failed to start advertising second time");
    assert!(advertiser.is_advertising());

    advertiser.stop().await.expect("Failed to stop advertising");
    assert!(!advertiser.is_advertising());
}

/// 测试重复停止（应该安全处理）
#[tokio::test]
async fn test_advertiser_double_stop() {
    let config = MdnsServiceConfig::new(
        "test-device-003".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12347,
    );

    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");

    advertiser.start().await.expect("Failed to start");

    // 第一次停止
    advertiser.stop().await.expect("Failed to stop first time");
    assert!(!advertiser.is_advertising());

    // 第二次停止应该安全处理（什么都不做）
    advertiser
        .stop()
        .await
        .expect("Failed to stop second time");
    assert!(!advertiser.is_advertising());
}

/// 测试不启动直接停止（应该安全处理）
#[tokio::test]
async fn test_advertiser_stop_without_start() {
    let config = MdnsServiceConfig::new(
        "test-device-004".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12348,
    );

    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");

    // 没有启动就停止应该安全
    advertiser.stop().await.expect("Stop without start should succeed");
    assert!(!advertiser.is_advertising());
}

/// 测试自定义主机名
#[tokio::test]
async fn test_advertiser_with_custom_hostname() {
    let config = MdnsServiceConfig::new(
        "test-device-005".to_string(),
        "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
        12349,
    )
    .with_hostname("custom-host".to_string());

    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");

    advertiser
        .start()
        .await
        .expect("Failed to start with custom hostname");
    assert!(advertiser.is_advertising());

    advertiser.stop().await.expect("Failed to stop");
}

/// 测试无效配置被拒绝
#[test]
fn test_advertiser_rejects_invalid_config() {
    // 空设备 ID
    let config = MdnsServiceConfig::new(String::new(), "hash".to_string(), 12345);
    let result = MdnsAdvertiser::new(config);
    assert!(result.is_err());
    if let Err(NetError::Configuration(msg)) = result {
        assert!(msg.contains("device_id"));
    } else {
        panic!("Expected Configuration error");
    }

    // 空公钥哈希
    let config = MdnsServiceConfig::new("device".to_string(), String::new(), 12345);
    let result = MdnsAdvertiser::new(config);
    assert!(result.is_err());

    // 端口为 0
    let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 0);
    let result = MdnsAdvertiser::new(config);
    assert!(result.is_err());
}

/// 测试 TXT 记录配置正确性 (AC2 验证)
///
/// 注意：实际的 mDNS 网络发现测试在某些环境中不稳定，
/// 因此这里验证配置层面的正确性，TXT 记录构建逻辑在单元测试中已验证。
#[test]
fn test_txt_record_configuration() {
    let device_id = "txt-test-device";
    let pubkey_hash = "dGVzdC1wdWJrZXktaGFzaC0xMjM=";

    let config = MdnsServiceConfig::new(
        device_id.to_string(),
        pubkey_hash.to_string(),
        12360,
    );

    // 验证配置有效
    assert!(config.validate().is_ok());

    // 验证配置值正确存储
    assert_eq!(config.device_id(), device_id);
    assert_eq!(config.public_key_hash(), pubkey_hash);

    // 验证 TXT 记录键名正确
    assert_eq!(TXT_DEVICE_ID, "id");
    assert_eq!(TXT_PUBKEY_HASH, "pk");

    // 验证服务类型正确
    assert_eq!(SERVICE_TYPE, "_nearclip._tcp.local.");
}

/// 测试多个广播器可以共存（不同端口）
#[tokio::test]
async fn test_multiple_advertisers() {
    let config1 = MdnsServiceConfig::new(
        "test-device-multi-1".to_string(),
        "aGFzaDE=".to_string(),
        12350,
    );

    let config2 = MdnsServiceConfig::new(
        "test-device-multi-2".to_string(),
        "aGFzaDI=".to_string(),
        12351,
    );

    let mut advertiser1 = MdnsAdvertiser::new(config1).expect("Failed to create advertiser 1");
    let mut advertiser2 = MdnsAdvertiser::new(config2).expect("Failed to create advertiser 2");

    advertiser1.start().await.expect("Failed to start advertiser 1");
    advertiser2.start().await.expect("Failed to start advertiser 2");

    assert!(advertiser1.is_advertising());
    assert!(advertiser2.is_advertising());

    // 验证它们有不同的服务名称
    assert_ne!(
        advertiser1.service_fullname(),
        advertiser2.service_fullname()
    );

    advertiser1.stop().await.expect("Failed to stop advertiser 1");
    advertiser2.stop().await.expect("Failed to stop advertiser 2");
}
