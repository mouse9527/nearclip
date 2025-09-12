/// Device type enumeration for NearClip devices
/// 
/// Represents different types of devices that can participate in NearClip
/// clipboard synchronization.
/// 
/// # Examples
/// 
/// Creating and converting device types:
/// 
/// ```
/// use nearclip_core::device::DeviceType;
/// 
/// let phone = DeviceType::Phone;
/// assert_eq!(phone.to_string(), "Phone");
/// 
/// let tablet: DeviceType = "tablet".parse().unwrap();
/// assert_eq!(tablet, DeviceType::Tablet);
/// ```
/// 
/// Parsing from strings with case insensitivity:
/// 
/// ```
/// use nearclip_core::device::DeviceType;
/// use std::str::FromStr;
/// 
/// assert_eq!(DeviceType::from_str("PHONE"), Ok(DeviceType::Phone));
/// assert_eq!(DeviceType::from_str("desktop"), Ok(DeviceType::Desktop));
/// assert!(DeviceType::from_str("invalid").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// Mobile phone device
    Phone,
    /// Tablet device  
    Tablet,
    /// Desktop computer
    Desktop,
    /// Laptop computer
    Laptop,
    /// Unknown or unrecognized device type
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Phone => write!(f, "Phone"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Laptop => write!(f, "Laptop"),
            DeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::str::FromStr for DeviceType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "phone" => Ok(DeviceType::Phone),
            "tablet" => Ok(DeviceType::Tablet),
            "desktop" => Ok(DeviceType::Desktop),
            "laptop" => Ok(DeviceType::Laptop),
            "unknown" => Ok(DeviceType::Unknown),
            _ => Err(format!("Unknown device type: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // Test constants for reuse
    const VALID_DEVICE_TYPES: &[&str] = &["phone", "tablet", "desktop", "laptop", "unknown"];
    const INVALID_DEVICE_TYPES: &[&str] = &["invalid", "smartwatch", "tv"];

    mod creation {
        use super::*;

        #[test]
        fn creates_all_device_types_successfully() {
            // Test that all device type variants can be created
            let _phone = DeviceType::Phone;
            let _tablet = DeviceType::Tablet;
            let _desktop = DeviceType::Desktop;
            let _laptop = DeviceType::Laptop;
            let _unknown = DeviceType::Unknown;
        }
    }

    mod display {
        use super::*;

        #[test]
        fn formats_all_device_types_correctly() {
            let test_cases = vec![
                (DeviceType::Phone, "Phone"),
                (DeviceType::Tablet, "Tablet"),
                (DeviceType::Desktop, "Desktop"),
                (DeviceType::Laptop, "Laptop"),
                (DeviceType::Unknown, "Unknown"),
            ];

            for (device_type, expected) in test_cases {
                assert_eq!(device_type.to_string(), expected);
            }
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn parses_valid_device_types_case_insensitively() {
            let test_cases = vec![
                ("phone", DeviceType::Phone),
                ("PHONE", DeviceType::Phone),
                ("Phone", DeviceType::Phone),
                ("tablet", DeviceType::Tablet),
                ("TABLET", DeviceType::Tablet),
                ("desktop", DeviceType::Desktop),
                ("laptop", DeviceType::Laptop),
                ("unknown", DeviceType::Unknown),
            ];

            for (input, expected) in test_cases {
                assert_eq!(
                    DeviceType::from_str(input),
                    Ok(expected),
                    "Failed to parse '{}'", input
                );
            }
        }

        #[test]
        fn returns_error_for_invalid_device_types() {
            for &invalid_type in INVALID_DEVICE_TYPES {
                let result = DeviceType::from_str(invalid_type);
                assert!(
                    result.is_err(),
                    "Expected error for invalid device type: '{}'",
                    invalid_type
                );
            }
        }

        #[test]
        fn error_message_contains_invalid_input() {
            let invalid_input = "smartwatch";
            let result = DeviceType::from_str(invalid_input);
            
            assert!(result.is_err());
            let error_message = result.unwrap_err();
            assert!(
                error_message.contains(invalid_input),
                "Error message should contain invalid input. Got: '{}'",
                error_message
            );
        }
    }

    mod traits {
        use super::*;

        #[test]
        fn supports_equality_comparison() {
            assert_eq!(DeviceType::Phone, DeviceType::Phone);
            assert_ne!(DeviceType::Phone, DeviceType::Tablet);
            assert_ne!(DeviceType::Desktop, DeviceType::Laptop);
        }

        #[test]
        fn supports_cloning() {
            let types = vec![
                DeviceType::Phone,
                DeviceType::Tablet,
                DeviceType::Desktop,
                DeviceType::Laptop,
                DeviceType::Unknown,
            ];

            for device_type in types {
                let cloned = device_type.clone();
                assert_eq!(device_type, cloned);
            }
        }

        #[test]
        fn supports_debug_formatting() {
            let phone = DeviceType::Phone;
            let debug_output = format!("{:?}", phone);
            assert!(debug_output.contains("Phone"));
        }
    }

    mod convenience {
        use super::*;

        #[test]
        fn converts_to_string_and_back() {
            let original_types = vec![
                DeviceType::Phone,
                DeviceType::Tablet,
                DeviceType::Desktop,
                DeviceType::Laptop,
                DeviceType::Unknown,
            ];

            for original in original_types {
                let string_repr = original.to_string();
                let parsed: DeviceType = string_repr.parse().expect("Failed to parse back");
                assert_eq!(original, parsed);
            }
        }

        #[test]
        fn all_predefined_valid_types_parse_successfully() {
            for &valid_type in VALID_DEVICE_TYPES {
                assert!(
                    DeviceType::from_str(valid_type).is_ok(),
                    "Valid device type '{}' should parse successfully",
                    valid_type
                );
            }
        }
    }
}