use crate::device::Device;

#[derive(Debug, Clone, PartialEq)]
pub enum DiscoveryEvent {
    DiscoveryStarted,
    DiscoveryStopped,
    DeviceDiscovered(Device),
    DeviceLost(String), // device_id
    DiscoveryError(String),
}

impl DiscoveryEvent {
    pub fn is_start_event(&self) -> bool {
        matches!(self, DiscoveryEvent::DiscoveryStarted)
    }
    
    pub fn is_stop_event(&self) -> bool {
        matches!(self, DiscoveryEvent::DiscoveryStopped)
    }
    
    pub fn is_device_event(&self) -> bool {
        matches!(self, DiscoveryEvent::DeviceDiscovered(_) | DiscoveryEvent::DeviceLost(_))
    }
    
    pub fn is_error_event(&self) -> bool {
        matches!(self, DiscoveryEvent::DiscoveryError(_))
    }
    
    pub fn get_device_id(&self) -> Option<&str> {
        match self {
            DiscoveryEvent::DeviceDiscovered(device) => Some(device.id()),
            DiscoveryEvent::DeviceLost(device_id) => Some(device_id),
            _ => None,
        }
    }
    
    pub fn get_error_message(&self) -> Option<&str> {
        match self {
            DiscoveryEvent::DiscoveryError(msg) => Some(msg),
            _ => None,
        }
    }
}

impl std::fmt::Display for DiscoveryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryEvent::DiscoveryStarted => write!(f, "Discovery started"),
            DiscoveryEvent::DiscoveryStopped => write!(f, "Discovery stopped"),
            DiscoveryEvent::DeviceDiscovered(device) => 
                write!(f, "Device discovered: {} ({})", device.name(), device.id()),
            DiscoveryEvent::DeviceLost(device_id) => 
                write!(f, "Device lost: {}", device_id),
            DiscoveryEvent::DiscoveryError(msg) => 
                write!(f, "Discovery error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{Device, DeviceType};

    #[test]
    fn test_discovery_event_creation() {
        // RED: 测试发现事件创建
        let start_event = DiscoveryEvent::DiscoveryStarted;
        let stop_event = DiscoveryEvent::DiscoveryStopped;
        
        assert!(matches!(start_event, DiscoveryEvent::DiscoveryStarted));
        assert!(matches!(stop_event, DiscoveryEvent::DiscoveryStopped));
    }

    #[test]
    fn test_device_discovered_event() {
        // RED: 测试设备发现事件
        let device = Device::new("test-id".to_string(), "Test Device".to_string(), DeviceType::Phone);
        let discovered_event = DiscoveryEvent::DeviceDiscovered(device.clone());
        
        match discovered_event {
            DiscoveryEvent::DeviceDiscovered(d) => {
                assert_eq!(d.id(), "test-id");
            }
            _ => panic!("Expected DeviceDiscovered event"),
        }
    }

    #[test]
    fn test_device_lost_event() {
        // RED: 测试设备丢失事件
        let lost_event = DiscoveryEvent::DeviceLost("test-id".to_string());
        
        match lost_event {
            DiscoveryEvent::DeviceLost(device_id) => {
                assert_eq!(device_id, "test-id");
            }
            _ => panic!("Expected DeviceLost event"),
        }
    }

    #[test]
    fn test_error_event() {
        // RED: 测试错误事件
        let error_event = DiscoveryEvent::DiscoveryError("Network error".to_string());
        
        match error_event {
            DiscoveryEvent::DiscoveryError(msg) => {
                assert_eq!(msg, "Network error");
            }
            _ => panic!("Expected DiscoveryError event"),
        }
    }
}