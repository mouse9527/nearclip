//! 消息处理模块
//!
//! 提供消息序列化、反序列化、验证等功能

use crate::error::{NearClipError, Result};
use prost::Message as ProstMessage;

/// 消息类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Discover = 1,
    PairRequest = 2,
    PairResponse = 3,
    SyncData = 4,
    Acknowledgment = 5,
    DeviceInfo = 6,
}

/// NearClip 消息结构
#[derive(Debug, Clone)]
pub struct NearClipMessage {
    pub message_id: String,
    pub message_type: MessageType,
    pub timestamp: u64,
    pub sender_id: String,
    pub data: Vec<u8>,
    pub signature: Option<Vec<u8>>,
}

impl NearClipMessage {
    /// 创建新消息
    pub fn new(
        message_id: String,
        message_type: MessageType,
        sender_id: String,
        data: Vec<u8>,
    ) -> Self {
        NearClipMessage {
            message_id,
            message_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            sender_id,
            data,
            signature: None,
        }
    }

    /// 序列化消息
    pub fn serialize(&self) -> Result<Vec<u8>> {
        // 简单的序列化实现
        // 在实际项目中会使用 Protocol Buffers
        let mut buffer = Vec::new();

        // 消息类型 (1 byte)
        buffer.push(self.message_type as u8);

        // 时间戳 (8 bytes)
        buffer.extend_from_slice(&self.timestamp.to_le_bytes());

        // 发送者ID长度和内容
        let sender_id_bytes = self.sender_id.as_bytes();
        buffer.push(sender_id_bytes.len() as u8);
        buffer.extend_from_slice(sender_id_bytes);

        // 消息ID长度和内容
        let message_id_bytes = self.message_id.as_bytes();
        buffer.extend_from_slice(&(message_id_bytes.len() as u16).to_le_bytes());
        buffer.extend_from_slice(message_id_bytes);

        // 数据长度和内容
        buffer.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.data);

        // 签名（如果有）
        if let Some(ref signature) = self.signature {
            buffer.extend_from_slice(&(signature.len() as u16).to_le_bytes());
            buffer.extend_from_slice(signature);
        } else {
            buffer.extend_from_slice(&0u16.to_le_bytes());
        }

        Ok(buffer)
    }

    /// 反序列化消息
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.is_empty() {
            return Err(NearClipError::SerializationError("Empty message data".to_string()));
        }

        let mut offset = 0;

        // 消息类型
        if offset >= data.len() {
            return Err(NearClipError::SerializationError("Invalid message format".to_string()));
        }
        let message_type = match data[offset] {
            1 => MessageType::Discover,
            2 => MessageType::PairRequest,
            3 => MessageType::PairResponse,
            4 => MessageType::SyncData,
            5 => MessageType::Acknowledgment,
            6 => MessageType::DeviceInfo,
            _ => return Err(NearClipError::SerializationError("Unknown message type".to_string())),
        };
        offset += 1;

        // 时间戳
        if offset + 8 > data.len() {
            return Err(NearClipError::SerializationError("Invalid timestamp".to_string()));
        }
        let timestamp = u64::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
            data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
        ]);
        offset += 8;

        // 发送者ID
        if offset >= data.len() {
            return Err(NearClipError::SerializationError("Invalid sender ID".to_string()));
        }
        let sender_id_len = data[offset] as usize;
        offset += 1;
        if offset + sender_id_len > data.len() {
            return Err(NearClipError::SerializationError("Sender ID too long".to_string()));
        }
        let sender_id = String::from_utf8(data[offset..offset + sender_id_len].to_vec())
            .map_err(|_| NearClipError::SerializationError("Invalid sender ID encoding".to_string()))?;
        offset += sender_id_len;

        // 消息ID
        if offset + 2 > data.len() {
            return Err(NearClipError::SerializationError("Invalid message ID".to_string()));
        }
        let message_id_len = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;
        if offset + message_id_len > data.len() {
            return Err(NearClipError::SerializationError("Message ID too long".to_string()));
        }
        let message_id = String::from_utf8(data[offset..offset + message_id_len].to_vec())
            .map_err(|_| NearClipError::SerializationError("Invalid message ID encoding".to_string()))?;
        offset += message_id_len;

        // 数据
        if offset + 4 > data.len() {
            return Err(NearClipError::SerializationError("Invalid data length".to_string()));
        }
        let data_len = u32::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        ]) as usize;
        offset += 4;
        if offset + data_len > data.len() {
            return Err(NearClipError::SerializationError("Data too long".to_string()));
        }
        let message_data = data[offset..offset + data_len].to_vec();
        offset += data_len;

        // 签名
        if offset + 2 > data.len() {
            return Err(NearClipError::SerializationError("Invalid signature length".to_string()));
        }
        let signature_len = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;
        let signature = if signature_len > 0 {
            if offset + signature_len > data.len() {
                return Err(NearClipError::SerializationError("Signature too long".to_string()));
            }
            Some(data[offset..offset + signature_len].to_vec())
        } else {
            None
        };

        Ok(NearClipMessage {
            message_id,
            message_type,
            timestamp,
            sender_id,
            data: message_data,
            signature,
        })
    }

    /// 验证消息完整性
    pub fn validate(&self) -> Result<()> {
        if self.message_id.is_empty() {
            return Err(NearClipError::InvalidParameter("Message ID cannot be empty".to_string()));
        }

        if self.sender_id.is_empty() {
            return Err(NearClipError::InvalidParameter("Sender ID cannot be empty".to_string()));
        }

        if self.timestamp == 0 {
            return Err(NearClipError::InvalidParameter("Invalid timestamp".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let message = NearClipMessage::new(
            "msg123".to_string(),
            MessageType::SyncData,
            "device1".to_string(),
            b"Hello, World!".to_vec(),
        );

        let serialized = message.serialize().unwrap();
        let deserialized = NearClipMessage::deserialize(&serialized).unwrap();

        assert_eq!(message.message_id, deserialized.message_id);
        assert_eq!(message.message_type, deserialized.message_type);
        assert_eq!(message.sender_id, deserialized.sender_id);
        assert_eq!(message.data, deserialized.data);
    }

    #[test]
    fn test_message_validation() {
        let mut message = NearClipMessage::new(
            "msg123".to_string(),
            MessageType::SyncData,
            "device1".to_string(),
            b"Hello".to_vec(),
        );

        // 正常消息应该通过验证
        assert!(message.validate().is_ok());

        // 空消息ID应该失败
        message.message_id = "".to_string();
        assert!(message.validate().is_err());

        // 空发送者ID应该失败
        message.message_id = "msg123".to_string();
        message.sender_id = "".to_string();
        assert!(message.validate().is_err());
    }
}