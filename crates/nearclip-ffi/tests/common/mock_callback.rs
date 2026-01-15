//! Mock callback implementation for FFI testing

#![allow(clippy::type_complexity)]

use std::sync::{Arc, Mutex};
use nearclip_ffi::*;

/// Mock callback implementation for testing
///
/// Tracks all callback invocations and their parameters
#[derive(Clone)]
pub struct MockCallback {
    calls: Arc<Mutex<Vec<String>>>,
    connected_devices: Arc<Mutex<Vec<FfiDeviceInfo>>>,
    disconnected_devices: Arc<Mutex<Vec<String>>>,
    unpaired_devices: Arc<Mutex<Vec<String>>>,
    rejected_pairings: Arc<Mutex<Vec<(String, String)>>>,
    received_clipboard: Arc<Mutex<Vec<(Vec<u8>, String)>>>,
    sync_errors: Arc<Mutex<Vec<String>>>,
    discovered_devices: Arc<Mutex<Vec<FfiDiscoveredDevice>>>,
    lost_devices: Arc<Mutex<Vec<String>>>,
}

impl MockCallback {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            connected_devices: Arc::new(Mutex::new(Vec::new())),
            disconnected_devices: Arc::new(Mutex::new(Vec::new())),
            unpaired_devices: Arc::new(Mutex::new(Vec::new())),
            rejected_pairings: Arc::new(Mutex::new(Vec::new())),
            received_clipboard: Arc::new(Mutex::new(Vec::new())),
            sync_errors: Arc::new(Mutex::new(Vec::new())),
            discovered_devices: Arc::new(Mutex::new(Vec::new())),
            lost_devices: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Check if a callback method was called
    pub fn was_called(&self, method: &str) -> bool {
        self.calls.lock().unwrap().contains(&method.to_string())
    }

    /// Get the number of times a callback was called
    pub fn call_count(&self, method: &str) -> usize {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|m| *m == method)
            .count()
    }

    /// Get all method calls
    pub fn get_calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }

    /// Get the last connected device
    pub fn get_last_connected_device(&self) -> Option<FfiDeviceInfo> {
        self.connected_devices.lock().unwrap().last().cloned()
    }

    /// Get all connected devices
    pub fn get_connected_devices(&self) -> Vec<FfiDeviceInfo> {
        self.connected_devices.lock().unwrap().clone()
    }

    /// Get the last disconnected device ID
    pub fn get_last_disconnected_device(&self) -> Option<String> {
        self.disconnected_devices.lock().unwrap().last().cloned()
    }

    /// Get the last received clipboard content
    pub fn get_last_clipboard_content(&self) -> Option<(Vec<u8>, String)> {
        self.received_clipboard.lock().unwrap().last().cloned()
    }

    /// Get all clipboard content received
    pub fn get_all_clipboard_content(&self) -> Vec<(Vec<u8>, String)> {
        self.received_clipboard.lock().unwrap().clone()
    }

    /// Get the last sync error
    pub fn get_last_sync_error(&self) -> Option<String> {
        self.sync_errors.lock().unwrap().last().cloned()
    }

    /// Get all sync errors
    pub fn get_all_sync_errors(&self) -> Vec<String> {
        self.sync_errors.lock().unwrap().clone()
    }

    /// Get discovered devices
    pub fn get_discovered_devices(&self) -> Vec<FfiDiscoveredDevice> {
        self.discovered_devices.lock().unwrap().clone()
    }

    /// Reset all tracked data
    pub fn reset(&self) {
        self.calls.lock().unwrap().clear();
        self.connected_devices.lock().unwrap().clear();
        self.disconnected_devices.lock().unwrap().clear();
        self.unpaired_devices.lock().unwrap().clear();
        self.rejected_pairings.lock().unwrap().clear();
        self.received_clipboard.lock().unwrap().clear();
        self.sync_errors.lock().unwrap().clear();
        self.discovered_devices.lock().unwrap().clear();
        self.lost_devices.lock().unwrap().clear();
    }
}

impl FfiNearClipCallback for MockCallback {
    fn on_device_connected(&self, device: FfiDeviceInfo) {
        self.calls
            .lock()
            .unwrap()
            .push("on_device_connected".to_string());
        self.connected_devices.lock().unwrap().push(device);
    }

    fn on_device_disconnected(&self, device_id: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_device_disconnected".to_string());
        self.disconnected_devices.lock().unwrap().push(device_id);
    }

    fn on_device_unpaired(&self, device_id: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_device_unpaired".to_string());
        self.unpaired_devices.lock().unwrap().push(device_id);
    }

    fn on_pairing_rejected(&self, device_id: String, reason: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_pairing_rejected".to_string());
        self.rejected_pairings
            .lock()
            .unwrap()
            .push((device_id, reason));
    }

    fn on_clipboard_received(&self, content: Vec<u8>, from_device: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_clipboard_received".to_string());
        self.received_clipboard
            .lock()
            .unwrap()
            .push((content, from_device));
    }

    fn on_sync_error(&self, error_message: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_sync_error".to_string());
        self.sync_errors.lock().unwrap().push(error_message);
    }

    fn on_device_discovered(&self, device: FfiDiscoveredDevice) {
        self.calls
            .lock()
            .unwrap()
            .push("on_device_discovered".to_string());
        self.discovered_devices.lock().unwrap().push(device);
    }

    fn on_device_lost(&self, peripheral_uuid: String) {
        self.calls
            .lock()
            .unwrap()
            .push("on_device_lost".to_string());
        self.lost_devices.lock().unwrap().push(peripheral_uuid);
    }
}

impl Default for MockCallback {
    fn default() -> Self {
        Self::new()
    }
}
