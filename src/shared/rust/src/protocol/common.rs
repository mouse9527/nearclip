include!(concat!(env!("OUT_DIR"), "/nearclip.common.rs"));

use crate::protocol::ProtocolError;

impl ErrorMessage {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code: code as i32,
            message,
            details: String::new(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = details;
        self
    }
}

impl Heartbeat {
    pub fn new(device_id: String, sequence_number: u32) -> Self {
        Self {
            device_id,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            sequence_number,
        }
    }
}

impl HeartbeatAck {
    pub fn new(device_id: String, sequence_number: u32) -> Self {
        Self {
            device_id,
            received_timestamp: chrono::Utc::now().timestamp_millis() as u64,
            sequence_number,
        }
    }
}

impl ProtocolVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            build_info: String::new(),
        }
    }

    pub fn with_build_info(mut self, build_info: String) -> Self {
        self.build_info = build_info;
        self
    }

    pub fn is_compatible_with(&self, other: &ProtocolVersion) -> bool {
        // 主版本必须相同
        if self.major != other.major {
            return false;
        }
        // 次版本向后兼容
        self.minor >= other.minor
    }
}

impl CapabilityNegotiation {
    pub fn new(min_version: ProtocolVersion, max_version: ProtocolVersion) -> Self {
        Self {
            min_version: Some(min_version),
            max_version: Some(max_version),
            supported_features: vec![],
            required_features: vec![],
        }
    }

    pub fn with_supported_feature(mut self, feature: String) -> Self {
        self.supported_features.push(feature);
        self
    }

    pub fn with_required_feature(mut self, feature: String) -> Self {
        self.required_features.push(feature);
        self
    }
}

impl CapabilityNegotiationResponse {
    pub fn compatible(selected_version: ProtocolVersion) -> Self {
        Self {
            selected_version: Some(selected_version),
            supported_features: vec![],
            unsupported_features: vec![],
            compatibility: true,
        }
    }

    pub fn incompatible(selected_version: ProtocolVersion) -> Self {
        Self {
            selected_version: Some(selected_version),
            supported_features: vec![],
            unsupported_features: vec![],
            compatibility: false,
        }
    }
}