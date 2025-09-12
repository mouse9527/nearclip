#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStatus {
    Offline,
    Online,
    Connecting,
    Connected,
    Error(String),
}

impl DeviceStatus {
    pub fn display(&self) -> String {
        match self {
            DeviceStatus::Offline => "Offline".to_string(),
            DeviceStatus::Online => "Online".to_string(),
            DeviceStatus::Connecting => "Connecting".to_string(),
            DeviceStatus::Connected => "Connected".to_string(),
            DeviceStatus::Error(msg) => format!("Error: {}", msg),
        }
    }
    
    pub fn is_connected(&self) -> bool {
        matches!(self, DeviceStatus::Online | DeviceStatus::Connected)
    }
    
    pub fn is_connecting(&self) -> bool {
        matches!(self, DeviceStatus::Connecting)
    }
    
    pub fn is_disconnected(&self) -> bool {
        matches!(self, DeviceStatus::Offline | DeviceStatus::Error(_))
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, DeviceStatus::Error(_))
    }
    
    pub fn error_message(&self) -> Option<&str> {
        match self {
            DeviceStatus::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_status_display() {
        // RED: 测试设备状态显示
        let status = DeviceStatus::Online;
        assert_eq!(status.display(), "Online");
        
        let error_status = DeviceStatus::Error("Network failed".to_string());
        assert_eq!(error_status.display(), "Error: Network failed");
    }

    #[test]
    fn test_device_status_properties() {
        // RED: 测试设备状态属性
        assert!(DeviceStatus::Online.is_connected());
        assert!(DeviceStatus::Connecting.is_connecting());
        assert!(DeviceStatus::Offline.is_disconnected());
        assert!(DeviceStatus::Error("test".to_string()).is_error());
    }
}