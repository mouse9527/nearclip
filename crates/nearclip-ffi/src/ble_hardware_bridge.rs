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
    // ========== Scanning ==========

    fn start_scan(&self) {
        self.ffi_hardware.start_scan();
    }

    fn stop_scan(&self) {
        self.ffi_hardware.stop_scan();
    }

    // ========== Connection ==========

    fn connect(&self, peripheral_id: &str) {
        self.ffi_hardware.connect(peripheral_id.to_string());
    }

    fn disconnect(&self, peripheral_id: &str) {
        self.ffi_hardware.disconnect(peripheral_id.to_string());
    }

    // ========== GATT Operations ==========

    fn read_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
    ) -> Result<Vec<u8>, String> {
        let data = self.ffi_hardware.read_characteristic(
            peripheral_id.to_string(),
            char_uuid.to_string(),
        );
        // Empty vec indicates error - but we can't distinguish from empty data
        // For now, assume empty vec is valid data (might be intentional)
        // Real error handling would require checking an error string method
        Ok(data)
    }

    fn write_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
        data: &[u8],
    ) -> Result<(), String> {
        let error = self.ffi_hardware.write_characteristic(
            peripheral_id.to_string(),
            char_uuid.to_string(),
            data.to_vec(),
        );
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }

    fn subscribe_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
    ) -> Result<(), String> {
        let error = self.ffi_hardware.subscribe_characteristic(
            peripheral_id.to_string(),
            char_uuid.to_string(),
        );
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }

    // ========== Advertising ==========

    fn start_advertising(&self, service_data: &[u8]) {
        self.ffi_hardware.start_advertising(service_data.to_vec());
    }

    fn stop_advertising(&self) {
        self.ffi_hardware.stop_advertising();
    }

    // ========== Status Query ==========

    fn is_connected(&self, peripheral_id: &str) -> bool {
        self.ffi_hardware.is_connected(peripheral_id.to_string())
    }

    fn get_mtu(&self, peripheral_id: &str) -> u16 {
        self.ffi_hardware.get_mtu(peripheral_id.to_string()) as u16
    }
}
