//! Common test utilities for FFI testing

pub mod mock_callback;

use std::sync::Arc;
pub use mock_callback::MockCallback;
use nearclip_ffi::*;
use nearclip_core::{DevicePlatform, DeviceStatus};

/// Create a test configuration with default values
pub fn create_test_config() -> FfiNearClipConfig {
    FfiNearClipConfig {
        device_name: "Test Device".to_string(),
        device_id: uuid::Uuid::new_v4().to_string(),
        wifi_enabled: true,
        ble_enabled: true,
        auto_connect: false,
        connection_timeout_secs: 30,
        heartbeat_interval_secs: 10,
        max_retries: 3,
    }
}

/// Create a test device info
pub fn create_test_device_info(id: &str) -> FfiDeviceInfo {
    FfiDeviceInfo {
        id: id.to_string(),
        name: format!("Test Device {}", id),
        platform: DevicePlatform::MacOS,
        status: DeviceStatus::Disconnected,
    }
}

/// Create a test manager with mock callback
pub fn create_test_manager() -> Arc<FfiNearClipManager> {
    let config = create_test_config();
    let callback = Box::new(MockCallback::new());
    Arc::new(FfiNearClipManager::new(config, callback).unwrap())
}

/// Create a test manager with a specific callback
pub fn create_manager_with_callback(
    callback: MockCallback,
) -> Arc<FfiNearClipManager> {
    let config = create_test_config();
    Arc::new(FfiNearClipManager::new(config, Box::new(callback)).unwrap())
}

/// Create a test history entry
pub fn create_test_history_entry(id: i64) -> FfiSyncHistoryEntry {
    FfiSyncHistoryEntry {
        id,
        device_id: "test-device".to_string(),
        device_name: "Test Device".to_string(),
        content_preview: "Test content".to_string(),
        content_size: 12,
        direction: "sent".to_string(),
        timestamp_ms: chrono::Utc::now().timestamp_millis(),
        success: true,
        error_message: None,
    }
}
