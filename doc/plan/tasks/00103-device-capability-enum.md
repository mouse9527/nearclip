# Task 00103: 定义设备能力枚举

## 任务描述

按照TDD原则定义设备能力枚举，表示设备支持的通信和配对方式。

## TDD开发要求

### RED阶段 - 编写失败的测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

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
```

### GREEN阶段 - 最小实现
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    WiFi,
    BLE,
    NFC,
    QRCode,
    PinCode,
}

impl ToString for DeviceCapability {
    fn to_string(&self) -> String {
        match self {
            DeviceCapability::WiFi => "WiFi".to_string(),
            DeviceCapability::BLE => "BLE".to_string(),
            DeviceCapability::NFC => "NFC".to_string(),
            DeviceCapability::QRCode => "QR Code".to_string(),
            DeviceCapability::PinCode => "PIN Code".to_string(),
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
```

### REFACTOR阶段
```rust
// 代码已经简洁，无需重构
```

## 验收标准
- [ ] 所有测试通过
- [ ] 能力定义完整
- [ ] 字符串转换正常工作

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00104: 定义设备基础结构](00104-device-basic-structure.md)