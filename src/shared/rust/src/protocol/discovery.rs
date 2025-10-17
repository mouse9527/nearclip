include!(concat!(env!("OUT_DIR"), "/nearclip.discovery.rs"));

use crate::protocol::ProtocolError;

impl DeviceBroadcast {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.device_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("设备ID不能为空".to_string()));
        }
        if self.device_name.is_empty() {
            return Err(ProtocolError::InvalidFormat("设备名称不能为空".to_string()));
        }
        if self.timestamp == 0 {
            return Err(ProtocolError::InvalidFormat("时间戳无效".to_string()));
        }
        Ok(())
    }

    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

impl ScanRequest {
    pub fn new(timeout_seconds: u32) -> Self {
        Self {
            timeout_seconds,
            filter_types: vec![],
            required_capabilities: vec![],
        }
    }

    pub fn with_device_type_filter(mut self, device_type: DeviceType) -> Self {
        self.filter_types.push(device_type);
        self
    }

    pub fn with_required_capability(mut self, capability: DeviceCapability) -> Self {
        self.required_capabilities.push(capability);
        self
    }
}

impl ScanResponse {
    pub fn filter_by_type(&self, device_type: DeviceType) -> Vec<&DeviceBroadcast> {
        self.devices.iter()
            .filter(|device| device.device_type == device_type)
            .collect()
    }

    pub fn filter_by_capability(&self, capability: DeviceCapability) -> Vec<&DeviceBroadcast> {
        self.devices.iter()
            .filter(|device| device.has_capability(capability))
            .collect()
    }
}