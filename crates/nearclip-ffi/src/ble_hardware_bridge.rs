//! BLE Hardware Bridge
//!
//! Adapts FFI BleHardware interface to Rust BleHardware trait

use std::sync::Arc;
use nearclip_ble::BleHardware;

// FfiBleHardware trait is defined in lib.rs
use super::FfiBleHardware;

/// Bridge that adapts FfiBleHardware to BleHardware trait
pub struct BleHardwareBridge {
    ffi_hardware: Arc<dyn FfiBleHardware>,
}

impl BleHardwareBridge {
    pub fn new(ffi_hardware: Arc<dyn FfiBleHardware>) -> Self {
        Self { ffi_hardware }
    }
}

impl BleHardware for BleHardwareBridge {
    fn start_scan(&self) {
        self.ffi_hardware.start_scan();
    }

    fn stop_scan(&self) {
        self.ffi_hardware.stop_scan();
    }

    fn connect(&self, peripheral_uuid: String) {
        self.ffi_hardware.connect(peripheral_uuid);
    }

    fn disconnect(&self, peripheral_uuid: String) {
        self.ffi_hardware.disconnect(peripheral_uuid);
    }

    fn write_data(&self, peripheral_uuid: String, data: Vec<u8>) -> String {
        self.ffi_hardware.write_data(peripheral_uuid, data)
    }

    fn get_mtu(&self, peripheral_uuid: String) -> u32 {
        self.ffi_hardware.get_mtu(peripheral_uuid)
    }

    fn is_connected(&self, peripheral_uuid: String) -> bool {
        self.ffi_hardware.is_connected(peripheral_uuid)
    }

    fn start_advertising(&self) {
        self.ffi_hardware.start_advertising();
    }

    fn stop_advertising(&self) {
        self.ffi_hardware.stop_advertising();
    }

    fn configure(&self, device_id: String, public_key_hash: String) {
        self.ffi_hardware.configure(device_id, public_key_hash);
    }
}
