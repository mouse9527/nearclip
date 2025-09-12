//! Common utilities and helpers for integration tests
//!
//! This module provides shared utilities used across multiple integration test files.

use nearclip_core::device::DeviceType;
use std::str::FromStr;

/// Test data generator for device types
pub struct DeviceTypeTestData;

impl DeviceTypeTestData {
    /// Returns all valid device types for testing
    pub fn all_device_types() -> Vec<DeviceType> {
        vec![
            DeviceType::Phone,
            DeviceType::Tablet,
            DeviceType::Desktop,
            DeviceType::Laptop,
            DeviceType::Unknown,
        ]
    }
    
    /// Returns valid string representations for all device types
    pub fn valid_device_type_strings() -> Vec<&'static str> {
        vec!["phone", "tablet", "desktop", "laptop", "unknown"]
    }
    
    /// Returns invalid device type strings for error testing
    pub fn invalid_device_type_strings() -> Vec<&'static str> {
        vec!["invalid", "smartwatch", "tv", "console", "earbuds"]
    }
    
    /// Returns test cases for case insensitive parsing
    pub fn case_insensitive_test_cases() -> Vec<(&'static str, DeviceType)> {
        vec![
            ("phone", DeviceType::Phone),
            ("PHONE", DeviceType::Phone),
            ("Phone", DeviceType::Phone),
            ("tablet", DeviceType::Tablet),
            ("TABLET", DeviceType::Tablet),
            ("desktop", DeviceType::Desktop),
            ("laptop", DeviceType::Laptop),
            ("unknown", DeviceType::Unknown),
        ]
    }
}

/// Assertion helpers for device type tests
pub struct DeviceTypeAssertions;

impl DeviceTypeAssertions {
    /// Asserts that a device type can be converted to string and back successfully
    pub fn assert_round_trip_conversion(device_type: DeviceType) {
        let string_repr = device_type.to_string();
        let parsed: DeviceType = string_repr.parse()
            .unwrap_or_else(|_| panic!("Failed to parse device type: {}", string_repr));
        assert_eq!(device_type, parsed, "Round-trip conversion failed for {:?}", device_type);
    }
    
    /// Asserts that parsing succeeds for valid inputs
    pub fn assert_valid_parsing(input: &str, expected: DeviceType) {
        let result = DeviceType::from_str(input);
        assert!(result.is_ok(), "Expected successful parsing for '{}', got error", input);
        assert_eq!(result.unwrap(), expected, "Parsed device type doesn't match expected");
    }
    
    /// Asserts that parsing fails for invalid inputs
    pub fn assert_invalid_parsing(input: &str) {
        let result = DeviceType::from_str(input);
        assert!(result.is_err(), "Expected parsing to fail for '{}', but it succeeded", input);
    }
}

/// Performance test utilities
pub struct PerfTestUtils;

impl PerfTestUtils {
    /// Measures execution time of a function
    pub fn measure_time<F, R>(f: F) -> std::time::Duration
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        f();
        start.elapsed()
    }
    
    /// Asserts that a function completes within a time limit
    pub fn assert_completes_within<F, R>(f: F, max_duration: std::time::Duration, description: &str)
    where
        F: FnOnce() -> R,
    {
        let duration = Self::measure_time(f);
        assert!(
            duration <= max_duration,
            "{} took {:?}, expected less than {:?}",
            description, duration, max_duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_data_generators() {
        let all_types = DeviceTypeTestData::all_device_types();
        assert_eq!(all_types.len(), 5);
        
        let valid_strings = DeviceTypeTestData::valid_device_type_strings();
        assert_eq!(valid_strings.len(), 5);
        
        let invalid_strings = DeviceTypeTestData::invalid_device_type_strings();
        assert!(!invalid_strings.is_empty());
    }

    #[test]
    fn test_assertion_helpers() {
        DeviceTypeAssertions::assert_round_trip_conversion(DeviceType::Phone);
        DeviceTypeAssertions::assert_valid_parsing("tablet", DeviceType::Tablet);
        DeviceTypeAssertions::assert_invalid_parsing("invalid");
    }

    #[test]
    fn test_perf_utils() {
        let duration = PerfTestUtils::measure_time(|| {
            // Simple operation
            let _ = DeviceType::Phone.to_string();
        });
        
        // Should complete very quickly
        assert!(duration < std::time::Duration::from_millis(1));
    }
}