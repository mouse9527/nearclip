//! Device models

use serde::{Deserialize, Serialize};

/// A device that has been paired with this device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairedDevice {
    /// Unique device identifier (UUID)
    pub device_id: String,
    /// Human-readable device name
    pub device_name: String,
    /// Platform/OS of the device
    pub platform: DevicePlatform,
    /// Device's public key (ECDH P-256)
    pub public_key: Vec<u8>,
    /// Shared secret derived from ECDH key exchange
    pub shared_secret: Vec<u8>,
    /// Unix timestamp (milliseconds) when pairing was completed
    pub paired_at: i64,
    /// Last connection timestamp (milliseconds)
    pub last_connected: Option<i64>,
    /// Last seen timestamp (milliseconds) via discovery
    pub last_seen: Option<i64>,
}

/// Platform/OS enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DevicePlatform {
    MacOS,
    Windows,
    Linux,
    Android,
    Ios,
}

/// A device discovered via mDNS or BLE scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    /// Unique device identifier
    pub device_id: String,
    /// Human-readable device name
    pub device_name: String,
    /// Platform/OS of the device
    pub platform: DevicePlatform,
    /// Hash of the device's public key (for verification)
    pub public_key_hash: String,
    /// Channel through which this device was discovered
    pub channel: DiscoveryChannel,
}

/// Discovery channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryChannel {
    /// Discovered via WiFi/mDNS
    WiFi { ip: String, port: u16 },
    /// Discovered via BLE
    BLE { peripheral_id: String, rssi: i16 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_serialization() {
        let platform = DevicePlatform::MacOS;
        let serialized = serde_json::to_string(&platform).unwrap();
        let deserialized: DevicePlatform = serde_json::from_str(&serialized).unwrap();
        assert_eq!(platform, deserialized);
    }

    #[test]
    fn test_paired_device_serialization() {
        let device = PairedDevice {
            device_id: "test-id".to_string(),
            device_name: "Test Device".to_string(),
            platform: DevicePlatform::MacOS,
            public_key: vec![1, 2, 3, 4],
            shared_secret: vec![5, 6, 7, 8],
            paired_at: 1703577600000,
            last_connected: Some(1703577600000),
            last_seen: Some(1703577600000),
        };

        let serialized = serde_json::to_string(&device).unwrap();
        let deserialized: PairedDevice = serde_json::from_str(&serialized).unwrap();
        assert_eq!(device, deserialized);
    }
}
