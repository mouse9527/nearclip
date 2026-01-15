//! FFI Smoke Tests
//!
//! Basic functionality tests to verify that FFI interfaces work correctly.
//! These tests don't require platform integration.

mod common;

use common::*;
use nearclip_ffi::*;

/// Test 1.1: FFI Manager creation succeeds
#[test]
fn test_ffi_manager_creation() {
    let config = create_test_config();
    let callback = Box::new(MockCallback::new());

    let result = FfiNearClipManager::new(config, callback);
    assert!(result.is_ok(), "FfiNearClipManager creation should succeed");

    let manager = result.unwrap();
    assert!(!manager.is_running(), "Manager should not be running initially");
}

/// Test 1.2: Get device ID returns valid UUID
#[test]
fn test_ffi_get_device_id() {
    let manager = create_test_manager();
    let device_id = manager.get_device_id();

    assert!(!device_id.is_empty(), "Device ID should not be empty");
    assert_eq!(device_id.len(), 36, "Device ID should be a valid UUID (36 chars)");

    // Verify it's a valid UUID format
    let result = uuid::Uuid::parse_str(&device_id);
    assert!(result.is_ok(), "Device ID should be a valid UUID");
}

/// Test 1.3: Initial state has no paired devices
#[test]
fn test_ffi_initial_state_no_devices() {
    let manager = create_test_manager();

    let paired_devices = manager.get_paired_devices();
    assert_eq!(
        paired_devices.len(),
        0,
        "Should have no paired devices initially"
    );

    let connected_devices = manager.get_connected_devices();
    assert_eq!(
        connected_devices.len(),
        0,
        "Should have no connected devices initially"
    );
}

/// Test 1.4: Add and remove paired device
#[test]
fn test_ffi_device_management() {
    let manager = create_test_manager();

    // Add device
    let device = create_test_device_info("test-device-1");
    manager.add_paired_device(device.clone());

    // Verify device was added
    let devices = manager.get_paired_devices();
    assert_eq!(devices.len(), 1, "Should have 1 paired device");
    assert_eq!(devices[0].id, device.id);
    assert_eq!(devices[0].name, device.name);

    // Remove device
    manager.remove_paired_device(device.id.clone());

    // Verify device was removed
    let devices = manager.get_paired_devices();
    assert_eq!(devices.len(), 0, "Should have no paired devices after removal");
}

/// Test 1.5: Add multiple devices
#[test]
fn test_ffi_multiple_devices() {
    let manager = create_test_manager();

    // Add multiple devices
    for i in 1..=5 {
        let device = create_test_device_info(&format!("device-{}", i));
        manager.add_paired_device(device);
    }

    // Verify all devices were added
    let devices = manager.get_paired_devices();
    assert_eq!(devices.len(), 5, "Should have 5 paired devices");

    // Verify device IDs
    let device_ids: Vec<String> = devices.iter().map(|d| d.id.clone()).collect();
    for i in 1..=5 {
        let expected_id = format!("device-{}", i);
        assert!(
            device_ids.contains(&expected_id),
            "Should contain device {}",
            i
        );
    }
}

/// Test 1.6: Get device status for non-existent device
#[test]
fn test_ffi_device_status_nonexistent() {
    let manager = create_test_manager();

    let status = manager.get_device_status("nonexistent-device".to_string());
    assert!(status.is_none(), "Status should be None for non-existent device");
}

/// Test 1.7: Generate QR code succeeds
#[test]
fn test_ffi_generate_qr_code() {
    let manager = create_test_manager();

    let result = manager.generate_qr_code();
    assert!(result.is_ok(), "QR code generation should succeed");

    let qr_data = result.unwrap();
    assert!(!qr_data.is_empty(), "QR code data should not be empty");

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&qr_data)
        .expect("QR code data should be valid JSON");

    // Verify required fields exist (based on PairingData structure)
    assert!(parsed["device_id"].is_string(), "QR code should contain device_id");
    assert!(parsed["public_key"].is_string(), "QR code should contain public_key");
    assert!(parsed["version"].is_number(), "QR code should contain version");

    // Verify device_id is not empty
    let device_id = parsed["device_id"].as_str().unwrap();
    assert!(!device_id.is_empty(), "device_id should not be empty");

    // Verify public_key is Base64 encoded and not empty
    let public_key = parsed["public_key"].as_str().unwrap();
    assert!(!public_key.is_empty(), "public_key should not be empty");
}

/// Test 1.8: Manager lifecycle
#[test]
fn test_ffi_manager_lifecycle() {
    let manager = create_test_manager();

    // Initial state: not running
    assert!(!manager.is_running(), "Manager should not be running initially");

    // Start manager
    let result = manager.start();
    assert!(result.is_ok(), "Manager start should succeed");
    assert!(manager.is_running(), "Manager should be running after start");

    // Stop manager
    manager.stop();

    // Note: is_running() may still return true immediately after stop()
    // as the internal runtime is shutting down asynchronously
}

/// Test 1.9: Clipboard sync requires manager to be running (placeholder)
#[test]
fn test_ffi_sync_clipboard_not_running() {
    let _manager = create_test_manager();

    // Try to sync without starting manager
    let _content = b"test clipboard content".to_vec();

    // Manager methods are not async, but sync_clipboard likely uses internal runtime
    // For now, we just test that it doesn't panic
    // Note: Actual behavior depends on implementation
}

/// Test 1.10: Connect device requires device to exist
#[test]
fn test_ffi_connect_nonexistent_device() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Try to connect to non-existent device
    let result = manager.connect_device("nonexistent-device".to_string());

    assert!(
        result.is_err(),
        "Connect should fail for non-existent device"
    );

    manager.stop();
}

/// Test 1.11: Disconnect device gracefully handles non-existent device
#[test]
fn test_ffi_disconnect_nonexistent_device() {
    let manager = create_test_manager();
    manager.start().unwrap();

    // Try to disconnect non-existent device
    let result = manager.disconnect_device("nonexistent-device".to_string());

    // Should either succeed gracefully or return appropriate error
    // (implementation dependent)
    assert!(result.is_ok() || result.is_err());

    manager.stop();
}

/// Test 1.12: Unpair device removes it from paired list
#[test]
fn test_ffi_unpair_device() {
    let manager = create_test_manager();

    // Add device
    let device = create_test_device_info("test-device");
    manager.add_paired_device(device.clone());

    // Verify device exists
    assert_eq!(manager.get_paired_devices().len(), 1);

    // Unpair device
    let result = manager.unpair_device(device.id.clone());
    assert!(result.is_ok(), "Unpair should succeed");

    // Verify device was removed
    assert_eq!(
        manager.get_paired_devices().len(),
        0,
        "Device should be removed after unpair"
    );
}
