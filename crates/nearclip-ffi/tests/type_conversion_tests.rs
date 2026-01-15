//! FFI Type Conversion Tests
//!
//! Tests to verify that FFI types correctly convert to and from internal types.

mod common;

use common::*;
use nearclip_ffi::*;
use nearclip_core::{DeviceInfo, DevicePlatform, DeviceStatus, NearClipConfig};
use std::time::Duration;

/// Test 2.1: FfiDeviceInfo conversion (FFI → Core → FFI)
#[test]
fn test_ffi_device_info_conversion() {
    let ffi_device = FfiDeviceInfo {
        id: "test-id".to_string(),
        name: "Test Device".to_string(),
        platform: DevicePlatform::MacOS,
        status: DeviceStatus::Connected,
    };

    // Convert FFI → Core
    let device: DeviceInfo = ffi_device.clone().into();
    assert_eq!(device.id(), "test-id");
    assert_eq!(device.name(), "Test Device");
    assert_eq!(device.platform(), DevicePlatform::MacOS);
    assert_eq!(device.status(), DeviceStatus::Connected);

    // Convert Core → FFI
    let ffi_device2: FfiDeviceInfo = device.into();
    assert_eq!(ffi_device2.id, ffi_device.id);
    assert_eq!(ffi_device2.name, ffi_device.name);
    assert_eq!(ffi_device2.platform, ffi_device.platform);
    assert_eq!(ffi_device2.status, ffi_device.status);
}

/// Test 2.2: FfiDeviceInfo conversion with different platforms
#[test]
fn test_ffi_device_info_conversion_platforms() {
    let platforms = vec![
        DevicePlatform::MacOS,
        DevicePlatform::Android,
        DevicePlatform::Unknown,
    ];

    for platform in platforms {
        let ffi_device = FfiDeviceInfo {
            id: "test-id".to_string(),
            name: "Test Device".to_string(),
            platform,
            status: DeviceStatus::Disconnected,
        };

        let device: DeviceInfo = ffi_device.clone().into();
        assert_eq!(device.platform(), platform);

        let ffi_device2: FfiDeviceInfo = device.into();
        assert_eq!(ffi_device2.platform, platform);
    }
}

/// Test 2.3: FfiDeviceInfo conversion with different statuses
#[test]
fn test_ffi_device_info_conversion_statuses() {
    let statuses = vec![
        DeviceStatus::Connected,
        DeviceStatus::Disconnected,
        DeviceStatus::Connecting,
        DeviceStatus::Failed,
    ];

    for status in statuses {
        let ffi_device = FfiDeviceInfo {
            id: "test-id".to_string(),
            name: "Test Device".to_string(),
            platform: DevicePlatform::MacOS,
            status,
        };

        let device: DeviceInfo = ffi_device.clone().into();
        assert_eq!(device.status(), status);

        let ffi_device2: FfiDeviceInfo = device.into();
        assert_eq!(ffi_device2.status, status);
    }
}

/// Test 2.4: FfiNearClipConfig conversion with all fields
#[test]
fn test_ffi_config_conversion_full() {
    let ffi_config = FfiNearClipConfig {
        device_name: "My Device".to_string(),
        device_id: "test-id-123".to_string(),
        wifi_enabled: true,
        ble_enabled: true,
        auto_connect: true,
        connection_timeout_secs: 30,
        heartbeat_interval_secs: 10,
        max_retries: 3,
    };

    let config: NearClipConfig = ffi_config.clone().into();
    assert_eq!(config.device_name(), "My Device");
    assert_eq!(config.device_id(), Some("test-id-123"));
    assert!(config.wifi_enabled());
    assert!(config.ble_enabled());
    assert!(config.auto_connect());
    assert_eq!(config.connection_timeout(), Duration::from_secs(30));
    assert_eq!(config.heartbeat_interval(), Duration::from_secs(10));
    assert_eq!(config.max_retries(), 3);
}

/// Test 2.5: FfiNearClipConfig conversion with disabled features
#[test]
fn test_ffi_config_conversion_disabled_features() {
    let ffi_config = FfiNearClipConfig {
        device_name: "Test Device".to_string(),
        device_id: "test-id".to_string(),
        wifi_enabled: false,
        ble_enabled: false,
        auto_connect: false,
        connection_timeout_secs: 60,
        heartbeat_interval_secs: 20,
        max_retries: 5,
    };

    let config: NearClipConfig = ffi_config.into();
    assert!(!config.wifi_enabled());
    assert!(!config.ble_enabled());
    assert!(!config.auto_connect());
    assert_eq!(config.connection_timeout(), Duration::from_secs(60));
    assert_eq!(config.heartbeat_interval(), Duration::from_secs(20));
    assert_eq!(config.max_retries(), 5);
}

/// Test 2.6: FfiNearClipConfig conversion with empty device_id
#[test]
fn test_ffi_config_conversion_empty_device_id() {
    let ffi_config = FfiNearClipConfig {
        device_name: "Test Device".to_string(),
        device_id: String::new(), // Empty string should convert to None
        wifi_enabled: true,
        ble_enabled: true,
        auto_connect: false,
        connection_timeout_secs: 30,
        heartbeat_interval_secs: 10,
        max_retries: 3,
    };

    let config: NearClipConfig = ffi_config.into();
    assert_eq!(config.device_id(), None); // Empty string converts to None
}

/// Test 2.7: FfiNearClipConfig default values
#[test]
fn test_ffi_config_default() {
    let config = FfiNearClipConfig::default();

    assert_eq!(config.device_name, "NearClip Device");
    assert_eq!(config.device_id, ""); // Default is empty string
    assert!(config.wifi_enabled);
    assert!(config.ble_enabled);
    assert!(config.auto_connect);
    assert_eq!(config.connection_timeout_secs, 30);
    assert_eq!(config.heartbeat_interval_secs, 10);
    assert_eq!(config.max_retries, 3);
}

/// Test 2.8: FfiSyncHistoryEntry conversion
#[test]
fn test_ffi_history_entry_values() {
    let entry = create_test_history_entry(42);

    // Verify all fields are set correctly
    assert_eq!(entry.id, 42);
    assert_eq!(entry.device_id, "test-device");
    assert_eq!(entry.device_name, "Test Device");
    assert_eq!(entry.content_preview, "Test content");
    assert_eq!(entry.content_size, 12);
    assert_eq!(entry.direction, "sent");
    assert!(entry.success);
    assert_eq!(entry.error_message, None);
    assert!(entry.timestamp_ms > 0);
}

/// Test 2.9: FfiSyncHistoryEntry with error
#[test]
fn test_ffi_history_entry_with_error() {
    let mut entry = create_test_history_entry(1);
    entry.success = false;
    entry.error_message = Some("Connection timeout".to_string());

    assert!(!entry.success);
    assert_eq!(entry.error_message, Some("Connection timeout".to_string()));
}

/// Test 2.10: FfiSyncHistoryEntry direction values
#[test]
fn test_ffi_history_entry_directions() {
    let directions = vec!["sent", "received"];

    for direction in directions {
        let mut entry = create_test_history_entry(1);
        entry.direction = direction.to_string();
        assert_eq!(entry.direction, direction);
    }
}

/// Test 2.11: FfiDiscoveredDevice structure
#[test]
fn test_ffi_discovered_device() {
    let device = FfiDiscoveredDevice {
        peripheral_uuid: "00:11:22:33:44:55".to_string(),
        device_name: Some("Test Device".to_string()),
        rssi: -65,
        public_key_hash: Some("abc123".to_string()),
    };

    assert_eq!(device.peripheral_uuid, "00:11:22:33:44:55");
    assert_eq!(device.device_name, Some("Test Device".to_string()));
    assert_eq!(device.rssi, -65);
    assert_eq!(device.public_key_hash, Some("abc123".to_string()));
}

/// Test 2.12: FfiDiscoveredDevice with missing optional fields
#[test]
fn test_ffi_discovered_device_optional_fields() {
    let device = FfiDiscoveredDevice {
        peripheral_uuid: "00:11:22:33:44:55".to_string(),
        device_name: None,
        rssi: -80,
        public_key_hash: None,
    };

    assert_eq!(device.device_name, None);
    assert_eq!(device.public_key_hash, None);
}

/// Test 2.13: FfiBleControllerConfig default values
#[test]
fn test_ffi_ble_controller_config_default() {
    let config = FfiBleControllerConfig::default();

    assert_eq!(config.scan_timeout_ms, 20000);
    assert_eq!(config.device_lost_timeout_ms, 30000);
    assert!(config.auto_reconnect);
    assert_eq!(config.max_reconnect_attempts, 5);
    assert_eq!(config.reconnect_base_delay_ms, 1000);
    assert_eq!(config.health_check_interval_ms, 30000);
    assert_eq!(config.connection_timeout_ms, 300000);
}

/// Test 2.14: LogLevel enum values
#[test]
fn test_log_level_values() {
    let levels = vec![
        LogLevel::Error,
        LogLevel::Warn,
        LogLevel::Info,
        LogLevel::Debug,
        LogLevel::Trace,
    ];

    for level in levels {
        // Verify enum can be cloned and compared
        let level_clone = level;
        assert_eq!(level, level_clone);
    }
}

/// Test 2.15: Complex type conversion roundtrip
#[test]
fn test_complex_type_roundtrip() {
    // Create a complex scenario with multiple type conversions
    let manager = create_test_manager();

    // Add multiple devices with different types
    for i in 1..=3 {
        let device = create_test_device_info(&format!("device-{}", i));
        manager.add_paired_device(device);
    }

    // Retrieve and verify conversion
    let devices = manager.get_paired_devices();
    assert_eq!(devices.len(), 3);

    // Verify all devices exist (order may vary)
    let device_ids: Vec<String> = devices.iter().map(|d| d.id.clone()).collect();
    for i in 1..=3 {
        let expected_id = format!("device-{}", i);
        assert!(
            device_ids.contains(&expected_id),
            "Should contain device {}",
            i
        );
    }

    // Verify all devices have correct platform and status
    for device in devices.iter() {
        assert_eq!(device.platform, DevicePlatform::MacOS);
        assert_eq!(device.status, DeviceStatus::Disconnected);
    }
}
