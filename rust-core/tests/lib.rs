//! Integration tests for nearclip-core library
//!
//! This module contains integration tests that verify the interaction
//! between different components of the nearclip-core library.

pub mod device;
pub mod common;

use nearclip_core::device::DeviceType;

/// Test that the library can be imported and basic functionality works
#[test]
fn test_library_imports_successfully() {
    // Test that we can import and use the library
    let device_type = DeviceType::Phone;
    assert_eq!(device_type.to_string(), "Phone");
}

/// Test cross-module functionality
#[test]
fn test_device_type_cross_module_usage() {
    // Test that DeviceType can be used in various contexts
    let device_from_str: DeviceType = "desktop".parse().unwrap();
    let device_direct = DeviceType::Desktop;
    
    assert_eq!(device_from_str, device_direct);
    
    // Test in collection contexts
    let devices = vec![DeviceType::Phone, DeviceType::Tablet, DeviceType::Laptop];
    assert_eq!(devices.len(), 3);
    assert!(devices.contains(&DeviceType::Phone));
}