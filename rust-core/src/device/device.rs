use crate::{DeviceType, DeviceStatus, DeviceCapability};

#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
    capabilities: std::collections::HashSet<DeviceCapability>,
    last_seen: std::time::SystemTime,
}

impl Device {
    pub fn new(id: String, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            status: DeviceStatus::Offline,
            capabilities: std::collections::HashSet::new(),
            last_seen: std::time::SystemTime::now(),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn device_type(&self) -> DeviceType {
        self.device_type.clone()
    }
    
    pub fn status(&self) -> DeviceStatus {
        self.status.clone()
    }
    
    pub fn set_status(&mut self, status: DeviceStatus) {
        self.status = status;
    }
    
    pub fn capabilities(&self) -> &std::collections::HashSet<DeviceCapability> {
        &self.capabilities
    }
    
    pub fn add_capability(&mut self, capability: DeviceCapability) {
        self.capabilities.insert(capability);
    }
    
    pub fn remove_capability(&mut self, capability: DeviceCapability) {
        self.capabilities.remove(&capability);
    }
    
    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.contains(&capability)
    }
    
    pub fn last_seen(&self) -> std::time::SystemTime {
        self.last_seen
    }
    
    pub fn update_last_seen(&mut self) {
        self.last_seen = std::time::SystemTime::now();
    }
}

impl Device {
    // 添加便捷方法
    pub fn is_online(&self) -> bool {
        self.status.is_connected()
    }
    
    pub fn is_offline(&self) -> bool {
        self.status.is_disconnected()
    }
    
    pub fn has_wifi(&self) -> bool {
        self.has_capability(DeviceCapability::WiFi)
    }
    
    pub fn has_ble(&self) -> bool {
        self.has_capability(DeviceCapability::BLE)
    }
    
    pub fn has_nfc(&self) -> bool {
        self.has_capability(DeviceCapability::NFC)
    }
    
    pub fn supports_qr_pairing(&self) -> bool {
        self.has_capability(DeviceCapability::QRCode)
    }
    
    pub fn supports_pin_pairing(&self) -> bool {
        self.has_capability(DeviceCapability::PinCode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        // RED: 测试设备创建
        let device = Device::new(
            "device-001".to_string(),
            "My Phone".to_string(),
            DeviceType::Phone,
        );
        
        assert_eq!(device.id(), "device-001");
        assert_eq!(device.name(), "My Phone");
        assert_eq!(device.device_type(), DeviceType::Phone);
        assert_eq!(device.status(), DeviceStatus::Offline);
    }

    #[test]
    fn test_device_status_update() {
        // RED: 测试设备状态更新
        let mut device = Device::new("id".to_string(), "name".to_string(), DeviceType::Phone);
        
        device.set_status(DeviceStatus::Online);
        assert_eq!(device.status(), DeviceStatus::Online);
        
        device.set_status(DeviceStatus::Error("Connection failed".to_string()));
        assert_eq!(device.status(), DeviceStatus::Error("Connection failed".to_string()));
    }

    #[test]
    fn test_device_capabilities() {
        // RED: 测试设备能力管理
        let mut device = Device::new("id".to_string(), "name".to_string(), DeviceType::Phone);
        
        assert!(!device.has_capability(DeviceCapability::WiFi));
        
        device.add_capability(DeviceCapability::WiFi);
        assert!(device.has_capability(DeviceCapability::WiFi));
        
        device.remove_capability(DeviceCapability::WiFi);
        assert!(!device.has_capability(DeviceCapability::WiFi));
    }
}