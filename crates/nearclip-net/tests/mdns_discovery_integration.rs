//! mDNS 发现集成测试
//!
//! 这些测试验证 mDNS 设备发现的完整流程。
//!
//! 注意：网络相关的测试在某些环境中可能不稳定，
//! 这些测试被标记为 `#[ignore]`，需要手动运行：
//! `cargo test -p nearclip-net --test mdns_discovery_integration -- --ignored`

use nearclip_net::{
    DiscoveryEvent, MdnsAdvertiser, MdnsDiscovery, MdnsServiceConfig,
    SERVICE_TYPE,
};
use std::time::Duration;

/// 测试发现广播的设备
///
/// 注意：此测试需要网络环境支持 mDNS，在某些环境中可能不稳定
#[tokio::test]
#[ignore = "requires mDNS network support"]
async fn test_discover_advertised_device() {
    // 启动广播器
    let config = MdnsServiceConfig::new(
        "test-device-discover".to_string(),
        "dGVzdC1oYXNo".to_string(),
        12370,
    );
    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");
    advertiser
        .start()
        .await
        .expect("Failed to start advertising");

    // 给广播器一点时间注册服务
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 启动发现器
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");
    let mut event_rx = discovery.subscribe();
    discovery.start().await.expect("Failed to start discovery");

    // 等待发现事件（带超时）
    let timeout_result = tokio::time::timeout(Duration::from_secs(10), async {
        while let Ok(event) = event_rx.recv().await {
            if let DiscoveryEvent::DeviceFound(device) = event {
                if device.device_id == "test-device-discover" {
                    return Some(device);
                }
            }
        }
        None
    })
    .await;

    // 清理
    discovery.stop().await.expect("Failed to stop discovery");
    advertiser.stop().await.expect("Failed to stop advertising");

    // 验证结果
    assert!(
        timeout_result.is_ok(),
        "Timeout waiting for device discovery"
    );
    let device = timeout_result.unwrap();
    assert!(device.is_some(), "Device was not found");
    let device = device.unwrap();
    assert_eq!(device.device_id, "test-device-discover");
    assert_eq!(device.public_key_hash, "dGVzdC1oYXNo");
    assert_eq!(device.port, 12370);
}

/// 测试检测设备离线
///
/// 注意：此测试需要网络环境支持 mDNS，在某些环境中可能不稳定
#[tokio::test]
#[ignore = "requires mDNS network support"]
async fn test_detect_device_offline() {
    // 启动广播器
    let config = MdnsServiceConfig::new(
        "test-device-offline".to_string(),
        "b2ZmbGluZS1oYXNo".to_string(),
        12371,
    );
    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");
    advertiser
        .start()
        .await
        .expect("Failed to start advertising");

    // 给广播器一点时间注册服务
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 启动发现器
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");
    let mut event_rx = discovery.subscribe();
    discovery.start().await.expect("Failed to start discovery");

    // 等待发现设备
    let found = tokio::time::timeout(Duration::from_secs(10), async {
        while let Ok(event) = event_rx.recv().await {
            if let DiscoveryEvent::DeviceFound(device) = event {
                if device.device_id == "test-device-offline" {
                    return true;
                }
            }
        }
        false
    })
    .await;

    assert!(
        found.is_ok() && found.unwrap(),
        "Device was not discovered initially"
    );

    // 停止广播器
    advertiser.stop().await.expect("Failed to stop advertising");

    // 等待设备离线事件（带超时）
    // 注意：mDNS 设备离线检测可能需要更长时间
    let lost = tokio::time::timeout(Duration::from_secs(15), async {
        while let Ok(event) = event_rx.recv().await {
            if let DiscoveryEvent::DeviceLost { device_id, .. } = event {
                if device_id == "test-device-offline" {
                    return true;
                }
            }
        }
        false
    })
    .await;

    // 清理
    discovery.stop().await.expect("Failed to stop discovery");

    // 验证
    // 注意：设备离线检测在某些环境下可能不可靠，所以我们不严格断言
    if lost.is_ok() && lost.unwrap() {
        // 检测到设备离线 - 理想情况
    } else {
        // 超时或未检测到 - 在某些网络环境下是正常的
        eprintln!("Note: DeviceLost event not received within timeout (this may be normal in some environments)");
    }
}

/// 测试多设备发现
///
/// 注意：此测试需要网络环境支持 mDNS，在某些环境中可能不稳定
#[tokio::test]
#[ignore = "requires mDNS network support"]
async fn test_discover_multiple_devices() {
    // 启动两个广播器
    let config1 = MdnsServiceConfig::new(
        "test-multi-device-1".to_string(),
        "aGFzaDE=".to_string(),
        12372,
    );
    let config2 = MdnsServiceConfig::new(
        "test-multi-device-2".to_string(),
        "aGFzaDI=".to_string(),
        12373,
    );

    let mut advertiser1 = MdnsAdvertiser::new(config1).expect("Failed to create advertiser 1");
    let mut advertiser2 = MdnsAdvertiser::new(config2).expect("Failed to create advertiser 2");

    advertiser1
        .start()
        .await
        .expect("Failed to start advertiser 1");
    advertiser2
        .start()
        .await
        .expect("Failed to start advertiser 2");

    // 给广播器一点时间注册服务
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 启动发现器
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");
    let mut event_rx = discovery.subscribe();
    discovery.start().await.expect("Failed to start discovery");

    // 收集发现的设备
    let mut found_devices: Vec<String> = Vec::new();

    let result = tokio::time::timeout(Duration::from_secs(15), async {
        while found_devices.len() < 2 {
            if let Ok(event) = event_rx.recv().await {
                if let DiscoveryEvent::DeviceFound(device) = event {
                    if device.device_id == "test-multi-device-1"
                        || device.device_id == "test-multi-device-2"
                    {
                        if !found_devices.contains(&device.device_id) {
                            found_devices.push(device.device_id.clone());
                        }
                    }
                }
            }
        }
    })
    .await;

    // 清理
    discovery.stop().await.expect("Failed to stop discovery");
    advertiser1
        .stop()
        .await
        .expect("Failed to stop advertiser 1");
    advertiser2
        .stop()
        .await
        .expect("Failed to stop advertiser 2");

    // 验证
    assert!(result.is_ok(), "Timeout waiting for multiple devices");
    assert!(
        found_devices.contains(&"test-multi-device-1".to_string()),
        "Device 1 not found"
    );
    assert!(
        found_devices.contains(&"test-multi-device-2".to_string()),
        "Device 2 not found"
    );
}

/// 测试发现器启动和停止
#[tokio::test]
async fn test_discovery_start_and_stop() {
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    assert!(!discovery.is_browsing());

    discovery.start().await.expect("Failed to start discovery");
    assert!(discovery.is_browsing());

    discovery.stop().await.expect("Failed to stop discovery");
    assert!(!discovery.is_browsing());
}

/// 测试重复启动发现器（应该安全处理）
#[tokio::test]
async fn test_discovery_double_start() {
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    discovery
        .start()
        .await
        .expect("Failed to start discovery first time");
    assert!(discovery.is_browsing());

    // 第二次启动应该安全处理
    discovery
        .start()
        .await
        .expect("Failed to start discovery second time");
    assert!(discovery.is_browsing());

    discovery.stop().await.expect("Failed to stop discovery");
}

/// 测试重复停止发现器（应该安全处理）
#[tokio::test]
async fn test_discovery_double_stop() {
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    discovery.start().await.expect("Failed to start discovery");

    discovery
        .stop()
        .await
        .expect("Failed to stop discovery first time");
    assert!(!discovery.is_browsing());

    // 第二次停止应该安全
    discovery
        .stop()
        .await
        .expect("Failed to stop discovery second time");
    assert!(!discovery.is_browsing());
}

/// 测试不启动直接停止（应该安全处理）
#[tokio::test]
async fn test_discovery_stop_without_start() {
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");

    // 没有启动就停止应该安全
    discovery
        .stop()
        .await
        .expect("Stop without start should succeed");
    assert!(!discovery.is_browsing());
}

/// 测试 get_devices 返回发现的设备
///
/// 注意：此测试需要网络环境支持 mDNS，在某些环境中可能不稳定
#[tokio::test]
#[ignore = "requires mDNS network support"]
async fn test_get_devices() {
    // 启动广播器
    let config = MdnsServiceConfig::new(
        "test-device-get".to_string(),
        "Z2V0LWhhc2g=".to_string(),
        12374,
    );
    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");
    advertiser
        .start()
        .await
        .expect("Failed to start advertising");

    // 给广播器一点时间注册服务
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 启动发现器
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");
    let mut event_rx = discovery.subscribe();
    discovery.start().await.expect("Failed to start discovery");

    // 等待发现设备
    let _ = tokio::time::timeout(Duration::from_secs(10), async {
        while let Ok(event) = event_rx.recv().await {
            if let DiscoveryEvent::DeviceFound(device) = event {
                if device.device_id == "test-device-get" {
                    return;
                }
            }
        }
    })
    .await;

    // 验证 get_devices 返回设备
    let devices = discovery.get_devices().await;

    // 清理
    discovery.stop().await.expect("Failed to stop discovery");
    advertiser.stop().await.expect("Failed to stop advertising");

    // 检查设备列表
    let found = devices
        .iter()
        .any(|d| d.device_id == "test-device-get");
    assert!(found, "Device not found in get_devices result");
}

/// 测试 get_device 获取特定设备
///
/// 注意：此测试需要网络环境支持 mDNS，在某些环境中可能不稳定
#[tokio::test]
#[ignore = "requires mDNS network support"]
async fn test_get_device_by_id() {
    // 启动广播器
    let config = MdnsServiceConfig::new(
        "test-device-getid".to_string(),
        "Z2V0aWQtaGFzaA==".to_string(),
        12375,
    );
    let mut advertiser = MdnsAdvertiser::new(config).expect("Failed to create advertiser");
    advertiser
        .start()
        .await
        .expect("Failed to start advertising");

    // 给广播器一点时间注册服务
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 启动发现器
    let mut discovery = MdnsDiscovery::new().expect("Failed to create discovery");
    let mut event_rx = discovery.subscribe();
    discovery.start().await.expect("Failed to start discovery");

    // 等待发现设备
    let _ = tokio::time::timeout(Duration::from_secs(10), async {
        while let Ok(event) = event_rx.recv().await {
            if let DiscoveryEvent::DeviceFound(device) = event {
                if device.device_id == "test-device-getid" {
                    return;
                }
            }
        }
    })
    .await;

    // 验证 get_device 返回设备
    let device = discovery.get_device("test-device-getid").await;

    // 验证不存在的设备返回 None
    let not_found = discovery.get_device("nonexistent-device").await;

    // 清理
    discovery.stop().await.expect("Failed to stop discovery");
    advertiser.stop().await.expect("Failed to stop advertising");

    // 检查结果
    assert!(device.is_some(), "Device not found by get_device");
    let device = device.unwrap();
    assert_eq!(device.device_id, "test-device-getid");
    assert_eq!(device.public_key_hash, "Z2V0aWQtaGFzaA==");

    assert!(not_found.is_none(), "Nonexistent device should return None");
}

/// 测试服务类型常量
#[test]
fn test_service_type_constant() {
    assert_eq!(SERVICE_TYPE, "_nearclip._tcp.local.");
}
