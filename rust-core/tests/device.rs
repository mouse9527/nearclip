//! Integration tests for device module
//!
//! Tests the device module functionality from an external user perspective.

use nearclip_core::device::DeviceType;
use std::str::FromStr;

#[test]
fn test_device_type_public_api() {
    // Test that all public methods work as expected
    let phone = DeviceType::Phone;
    
    // Test Display trait
    assert_eq!(phone.to_string(), "Phone");
    assert_eq!(format!("{}", phone), "Phone");
    
    // Test FromStr trait
    let parsed: DeviceType = "tablet".parse().unwrap();
    assert_eq!(parsed, DeviceType::Tablet);
    
    // Test Debug trait
    let debug_output = format!("{:?}", phone);
    assert!(debug_output.contains("Phone"));
}

#[test]
fn test_device_type_error_handling() {
    // Test error cases from user perspective
    let result = DeviceType::from_str("nonexistent");
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("nonexistent"));
}

#[test]
fn test_device_type_in_collections() {
    // Test usage in common collection types
    let mut devices = Vec::new();
    devices.push(DeviceType::Phone);
    devices.push(DeviceType::Desktop);
    
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0], DeviceType::Phone);
    
    // Test in Vec (most common usage)
    let device_vec: Vec<DeviceType> = vec![DeviceType::Phone, DeviceType::Tablet, DeviceType::Phone];
    assert_eq!(device_vec.len(), 3); // Vec allows duplicates
    
    // Test deduplication manually
    let mut unique_devices: Vec<DeviceType> = device_vec.clone();
    unique_devices.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
    unique_devices.dedup();
    assert_eq!(unique_devices.len(), 2); // Phone and Tablet after deduplication
}

#[test]
fn test_device_type_serialization_context() {
    // Test common serialization patterns
    let device = DeviceType::Laptop;
    let device_str = device.to_string();
    
    // Simulate JSON serialization context
    let json_like = format!("{{\"device_type\": \"{}\"}}", device_str);
    assert!(json_like.contains("Laptop"));
    
    // Test round-trip
    let round_trip: DeviceType = device_str.parse().unwrap();
    assert_eq!(device, round_trip);
}