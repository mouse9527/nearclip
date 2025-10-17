include!(concat!(env!("OUT_DIR"), "/nearclip.pairing.rs"));

use crate::protocol::ProtocolError;

impl PairingRequest {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.initiator_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("发起者ID不能为空".to_string()));
        }
        if self.target_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("目标ID不能为空".to_string()));
        }
        if self.device_name.is_empty() {
            return Err(ProtocolError::InvalidFormat("设备名称不能为空".to_string()));
        }
        if self.nonce.is_empty() {
            return Err(ProtocolError::InvalidFormat("随机数不能为空".to_string()));
        }
        if self.timestamp == 0 {
            return Err(ProtocolError::InvalidFormat("时间戳无效".to_string()));
        }
        Ok(())
    }
}

impl PairingResponse {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.responder_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("响应者ID不能为空".to_string()));
        }
        if self.initiator_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("发起者ID不能为空".to_string()));
        }
        if self.signed_nonce.is_empty() {
            return Err(ProtocolError::InvalidFormat("签名的随机数不能为空".to_string()));
        }
        Ok(())
    }
}

impl PairingConfirmation {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.session_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("会话ID不能为空".to_string()));
        }
        if self.confirmation_hash.is_empty() {
            return Err(ProtocolError::InvalidFormat("确认哈希不能为空".to_string()));
        }
        Ok(())
    }
}

impl PairingStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, PairingStatus::PairingCompleted)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, PairingStatus::PairingFailed)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, PairingStatus::PairingPending | PairingStatus::PairingInitiated)
    }
}