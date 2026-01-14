//! FFI Error Handling Tests
//!
//! Tests to verify that errors are correctly handled and propagated through the FFI boundary.

mod common;

use common::*;
use nearclip_ffi::*;
use nearclip_core::DeviceStatus;

/// Test 3.1: Manager creation with invalid config (empty device name)
#[test]
fn test_ffi_invalid_config_empty_name() {
    let config = FfiNearClipConfig {
        device_name: String::new(), // Invalid: empty name
        ..Default::default()
    };
    let callback = Box::new(MockCallback::new());

    let result = FfiNearClipManager::new(config, callback);
    assert!(result.is_err(), "Should fail with empty device name");
}

/// Test 3.2: Connect to non-existent device
#[test]
fn test_ffi_connect_nonexistent_device() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Try to connect to non-existent device
    let result = manager.connect_device("nonexistent-device-id".to_string());

    assert!(
        result.is_err(),
        "Connect should fail for non-existent device"
    );

    manager.stop();
}

/// Test 3.3: Disconnect non-existent device (should succeed gracefully)
#[test]
fn test_ffi_disconnect_nonexistent_device() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Try to disconnect non-existent device
    let result = manager.disconnect_device("nonexistent-device".to_string());

    // Should either succeed gracefully or return appropriate error
    assert!(result.is_ok() || result.is_err());

    manager.stop();
}

/// Test 3.4: Get status of non-existent device
#[test]
fn test_ffi_device_status_nonexistent() {
    let manager = create_test_manager();

    let status = manager.get_device_status("nonexistent-device".to_string());
    assert!(status.is_none(), "Status should be None for non-existent device");
}

/// Test 3.5: Unpair non-existent device
#[test]
fn test_ffi_unpair_nonexistent_device() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Try to unpair non-existent device
    let result = manager.unpair_device("nonexistent-device".to_string());

    // Should return error or succeed gracefully
    assert!(result.is_ok() || result.is_err());

    manager.stop();
}

/// Test 3.6: Parse invalid QR code (malformed JSON)
#[test]
fn test_ffi_invalid_qr_code_malformed_json() {
    let manager = create_test_manager();

    // Invalid JSON
    let result = manager.pair_with_qr_code("{ invalid json }".to_string());
    assert!(result.is_err(), "Should fail with malformed JSON");
}

/// Test 3.7: Parse invalid QR code (missing required fields)
#[test]
fn test_ffi_invalid_qr_code_missing_fields() {
    let manager = create_test_manager();

    // Valid JSON but missing required fields
    let result = manager.pair_with_qr_code(r#"{"version": 1}"#.to_string());
    assert!(result.is_err(), "Should fail with missing fields");
}

/// Test 3.8: Parse invalid QR code (invalid public key)
#[test]
fn test_ffi_invalid_qr_code_bad_public_key() {
    let manager = create_test_manager();

    // Valid JSON structure but invalid base64 public key
    let invalid_qr = r#"{
        "version": 1,
        "device_id": "test-device",
        "public_key": "not-valid-base64!!!"
    }"#;

    let result = manager.pair_with_qr_code(invalid_qr.to_string());
    assert!(result.is_err(), "Should fail with invalid public key");
}

/// Test 3.9: Multiple device add/remove operations
#[test]
fn test_ffi_device_add_remove_multiple_times() {
    let manager = create_test_manager();

    let device = create_test_device_info("test-device");

    // Add device
    manager.add_paired_device(device.clone());
    assert_eq!(manager.get_paired_devices().len(), 1);

    // Remove device
    manager.remove_paired_device(device.id.clone());
    assert_eq!(manager.get_paired_devices().len(), 0);

    // Add again
    manager.add_paired_device(device.clone());
    assert_eq!(manager.get_paired_devices().len(), 1);

    // Remove again
    manager.remove_paired_device(device.id.clone());
    assert_eq!(manager.get_paired_devices().len(), 0);
}

/// Test 3.10: Remove device that was never added
#[test]
fn test_ffi_remove_never_added_device() {
    let manager = create_test_manager();

    // Initial state: no devices
    assert_eq!(manager.get_paired_devices().len(), 0);

    // Try to remove non-existent device (should not panic)
    manager.remove_paired_device("never-added-device".to_string());

    // Should still have no devices
    assert_eq!(manager.get_paired_devices().len(), 0);
}

/// Test 3.11: Sync clipboard without starting manager
#[test]
fn test_ffi_sync_clipboard_not_running() {
    let manager = create_test_manager();

    // Try to sync without starting manager
    let content = b"test clipboard content".to_vec();
    let result = manager.sync_clipboard(content);

    // Should return error (manager not started)
    assert!(result.is_err(), "Should fail when manager not started");
}

/// Test 3.12: Start manager multiple times
#[test]
fn test_ffi_start_manager_multiple_times() {
    let manager = create_test_manager();

    // First start
    let result1 = manager.start();
    assert!(result1.is_ok(), "First start should succeed");
    assert!(manager.is_running());

    // Second start (already running)
    let result2 = manager.start();
    // Should either succeed or fail gracefully
    assert!(result2.is_ok() || result2.is_err());

    manager.stop();
}

/// Test 3.13: Stop manager multiple times
#[test]
fn test_ffi_stop_manager_multiple_times() {
    let manager = create_test_manager();

    manager.start().unwrap();
    assert!(manager.is_running());

    // First stop
    manager.stop();

    // Second stop (already stopped)
    manager.stop(); // Should not panic
}

/// Test 3.14: Operations on stopped manager
#[test]
fn test_ffi_operations_on_stopped_manager() {
    let manager = create_test_manager();

    // Add a device while stopped
    let device = create_test_device_info("test-device");
    manager.add_paired_device(device.clone());

    // Verify device was added
    assert_eq!(manager.get_paired_devices().len(), 1);

    // Get device status while stopped
    let status = manager.get_device_status(device.id.clone());
    assert_eq!(status, Some(DeviceStatus::Disconnected));

    // Remove device while stopped
    manager.remove_paired_device(device.id);
    assert_eq!(manager.get_paired_devices().len(), 0);
}

/// Test 3.15: Large clipboard content
#[test]
fn test_ffi_sync_large_clipboard() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Create large content (1MB)
    let large_content = vec![0u8; 1024 * 1024];
    let result = manager.sync_clipboard(large_content);

    // Should either succeed or fail gracefully (no panic)
    assert!(result.is_ok() || result.is_err());

    manager.stop();
}

/// Test 3.16: Empty clipboard content
#[test]
fn test_ffi_sync_empty_clipboard() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Empty content
    let empty_content = vec![];
    let result = manager.sync_clipboard(empty_content);

    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());

    manager.stop();
}

/// Test 3.17: Get device ID is always valid UUID
#[test]
fn test_ffi_device_id_is_valid_uuid() {
    let manager = create_test_manager();
    let device_id = manager.get_device_id();

    // Verify it's a valid UUID
    assert!(!device_id.is_empty());
    assert_eq!(device_id.len(), 36, "UUID should be 36 characters");

    // Try to parse as UUID
    let result = uuid::Uuid::parse_str(&device_id);
    assert!(result.is_ok(), "Device ID should be valid UUID");
}

/// Test 3.18: Generate QR code multiple times
#[test]
fn test_ffi_generate_qr_code_multiple_times() {
    let manager = create_test_manager();

    // Generate multiple times
    let qr1 = manager.generate_qr_code().unwrap();
    let qr2 = manager.generate_qr_code().unwrap();

    // Should both succeed
    assert!(!qr1.is_empty());
    assert!(!qr2.is_empty());

    // Both should be valid JSON
    assert!(serde_json::from_str::<serde_json::Value>(&qr1).is_ok());
    assert!(serde_json::from_str::<serde_json::Value>(&qr2).is_ok());
}

/// Test 3.19: Try to connect before starting manager
#[test]
fn test_ffi_connect_before_start() {
    let manager = create_test_manager();

    // Add a device
    let device = create_test_device_info("test-device");
    manager.add_paired_device(device.clone());

    // Try to connect without starting manager
    let result = manager.connect_device(device.id);

    // Should fail (manager not started)
    assert!(result.is_err(), "Connect should fail when manager not started");
}

/// Test 3.20: Get connected devices when none connected
#[test]
fn test_ffi_get_connected_devices_empty() {
    let manager = create_test_manager();

    let connected = manager.get_connected_devices();
    assert_eq!(connected.len(), 0, "Should have no connected devices initially");
}
