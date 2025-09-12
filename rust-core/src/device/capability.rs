#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    WiFi,
    BLE,
    NFC,
    QRCode,
    PinCode,
}

impl std::fmt::Display for DeviceCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceCapability::WiFi => write!(f, "WiFi"),
            DeviceCapability::BLE => write!(f, "BLE"),
            DeviceCapability::NFC => write!(f, "NFC"),
            DeviceCapability::QRCode => write!(f, "QR Code"),
            DeviceCapability::PinCode => write!(f, "PIN Code"),
        }
    }
}

impl std::str::FromStr for DeviceCapability {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wifi" => Ok(DeviceCapability::WiFi),
            "ble" => Ok(DeviceCapability::BLE),
            "nfc" => Ok(DeviceCapability::NFC),
            "qr" | "qrcode" => Ok(DeviceCapability::QRCode),
            "pin" | "pincode" => Ok(DeviceCapability::PinCode),
            _ => Err(format!("Unknown device capability: {}", s)),
        }
    }
}

impl DeviceCapability {
    pub fn all() -> Vec<DeviceCapability> {
        vec![
            DeviceCapability::WiFi,
            DeviceCapability::BLE,
            DeviceCapability::NFC,
            DeviceCapability::QRCode,
            DeviceCapability::PinCode,
        ]
    }
    
    pub fn is_wireless(&self) -> bool {
        matches!(self, DeviceCapability::WiFi | DeviceCapability::BLE | DeviceCapability::NFC)
    }
    
    pub fn is_pairing_method(&self) -> bool {
        matches!(self, DeviceCapability::QRCode | DeviceCapability::PinCode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_device_capability_display() {
        // RED: 测试设备能力显示
        assert_eq!(DeviceCapability::WiFi.to_string(), "WiFi");
        assert_eq!(DeviceCapability::BLE.to_string(), "BLE");
        assert_eq!(DeviceCapability::NFC.to_string(), "NFC");
    }

    #[test]
    fn test_device_capability_from_string() {
        // RED: 测试从字符串解析设备能力
        assert_eq!(DeviceCapability::from_str("wifi"), Ok(DeviceCapability::WiFi));
        assert_eq!(DeviceCapability::from_str("ble"), Ok(DeviceCapability::BLE));
        assert!(DeviceCapability::from_str("invalid").is_err());
    }

    #[test]
    fn test_device_capabilities_all() {
        // RED: 测试所有设备能力
        let all_capabilities = DeviceCapability::all();
        assert!(all_capabilities.contains(&DeviceCapability::WiFi));
        assert!(all_capabilities.contains(&DeviceCapability::BLE));
        assert!(all_capabilities.contains(&DeviceCapability::NFC));
        assert!(all_capabilities.contains(&DeviceCapability::QRCode));
        assert!(all_capabilities.contains(&DeviceCapability::PinCode));
    }
}