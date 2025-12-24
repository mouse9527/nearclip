//! NearClip 消息协议
//!
//! 定义所有网络通信使用的统一消息格式。
//!
//! # 消息类型
//!
//! | 类型 | 用途 |
//! |------|------|
//! | `ClipboardSync` | 剪贴板内容同步 |
//! | `PairingRequest` | 设备配对请求 |
//! | `PairingResponse` | 配对响应 |
//! | `Heartbeat` | 心跳保活 |
//! | `Ack` | 确认收到 |
//!
//! # 使用示例
//!
//! ```
//! use nearclip_sync::{Message, MessageType};
//!
//! // 创建剪贴板同步消息
//! let msg = Message::clipboard_sync(b"Hello, World!", "device-123".to_string());
//!
//! // 序列化
//! let bytes = msg.serialize().unwrap();
//!
//! // 反序列化
//! let decoded = Message::deserialize(&bytes).unwrap();
//! assert_eq!(decoded.msg_type, MessageType::ClipboardSync);
//! ```

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

// ============================================================
// PairingPayload - 配对请求载荷
// ============================================================

/// 设备平台类型（用于协议消息）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ProtocolPlatform {
    /// macOS 平台
    MacOS,
    /// Android 平台
    Android,
    /// 未知平台
    #[default]
    Unknown,
}

impl ProtocolPlatform {
    /// 返回平台名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ProtocolPlatform::MacOS => "macOS",
            ProtocolPlatform::Android => "Android",
            ProtocolPlatform::Unknown => "Unknown",
        }
    }
}

/// 配对请求/响应载荷
///
/// 包含设备基本信息，用于双向配对。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairingPayload {
    /// 设备唯一标识符
    pub device_id: String,
    /// 设备显示名称
    pub device_name: String,
    /// 设备平台
    pub platform: ProtocolPlatform,
}

impl PairingPayload {
    /// 创建新的配对载荷
    pub fn new(device_id: impl Into<String>, device_name: impl Into<String>, platform: ProtocolPlatform) -> Self {
        Self {
            device_id: device_id.into(),
            device_name: device_name.into(),
            platform,
        }
    }

    /// 序列化为 MessagePack 字节
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        rmp_serde::to_vec(self).map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    /// 从 MessagePack 字节反序列化
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        rmp_serde::from_slice(data).map_err(|e| ProtocolError::Deserialization(e.to_string()))
    }
}

/// 协议错误类型
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ProtocolError {
    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// 反序列化错误
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// 消息类型枚举
///
/// 标识消息的用途，用于路由和处理。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum MessageType {
    /// 剪贴板同步内容
    ///
    /// payload 包含剪贴板的原始字节内容
    ClipboardSync,

    /// 配对请求
    ///
    /// payload 包含公钥和设备信息
    PairingRequest,

    /// 配对响应
    ///
    /// payload 包含对方公钥和确认信息
    PairingResponse,

    /// 配对拒绝
    ///
    /// 当收到来自未配对设备的连接请求时发送
    /// 表示对方需要先移除本设备再重新配对
    PairingRejection,

    /// 心跳保活
    ///
    /// payload 通常为空，用于维持连接
    #[default]
    Heartbeat,

    /// 确认收到
    ///
    /// payload 可包含被确认消息的标识
    Ack,

    /// 取消配对通知
    ///
    /// 通知对方设备删除配对关系
    Unpair,
}

impl MessageType {
    /// 获取消息类型的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::ClipboardSync => "clipboard_sync",
            MessageType::PairingRequest => "pairing_request",
            MessageType::PairingResponse => "pairing_response",
            MessageType::PairingRejection => "pairing_rejection",
            MessageType::Heartbeat => "heartbeat",
            MessageType::Ack => "ack",
            MessageType::Unpair => "unpair",
        }
    }

    /// 检查是否需要确认响应
    pub fn requires_ack(&self) -> bool {
        matches!(
            self,
            MessageType::ClipboardSync | MessageType::PairingRequest | MessageType::PairingResponse
        )
    }
}

/// 统一消息结构
///
/// 所有网络通信必须使用此结构。使用 MessagePack 序列化以获得紧凑的二进制格式。
///
/// # 字段说明
///
/// - `msg_type`: 消息类型，决定如何处理 payload
/// - `payload`: 消息载荷，已使用 MessagePack 序列化
/// - `timestamp`: 消息创建时间（Unix 毫秒时间戳）
/// - `device_id`: 发送方设备的唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// 消息类型
    pub msg_type: MessageType,

    /// 原始载荷数据（剪贴板内容、公钥等）
    pub payload: Vec<u8>,

    /// Unix 毫秒时间戳
    pub timestamp: u64,

    /// 发送方设备 ID
    pub device_id: String,
}

impl Message {
    /// 创建新消息
    ///
    /// # Arguments
    ///
    /// * `msg_type` - 消息类型
    /// * `payload` - 消息载荷字节
    /// * `device_id` - 发送方设备 ID
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::{Message, MessageType};
    ///
    /// let msg = Message::new(
    ///     MessageType::Heartbeat,
    ///     Vec::new(),
    ///     "my-device".to_string(),
    /// );
    /// ```
    pub fn new(msg_type: MessageType, payload: Vec<u8>, device_id: String) -> Self {
        Self {
            msg_type,
            payload,
            timestamp: Self::timestamp_now(),
            device_id,
        }
    }

    /// 获取当前 Unix 毫秒时间戳
    ///
    /// 如果系统时间早于 Unix 纪元（极端罕见），返回 0。
    pub fn timestamp_now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    /// 创建剪贴板同步消息
    ///
    /// # Arguments
    ///
    /// * `content` - 剪贴板内容字节
    /// * `device_id` - 发送方设备 ID
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::clipboard_sync(b"copied text", "device-123".to_string());
    /// ```
    pub fn clipboard_sync(content: &[u8], device_id: String) -> Self {
        Self::new(MessageType::ClipboardSync, content.to_vec(), device_id)
    }

    /// 创建配对请求消息
    ///
    /// # Arguments
    ///
    /// * `payload` - 包含公钥和设备信息的载荷
    /// * `device_id` - 发送方设备 ID
    pub fn pairing_request(payload: Vec<u8>, device_id: String) -> Self {
        Self::new(MessageType::PairingRequest, payload, device_id)
    }

    /// 创建配对响应消息
    ///
    /// # Arguments
    ///
    /// * `payload` - 包含确认信息的载荷
    /// * `device_id` - 发送方设备 ID
    pub fn pairing_response(payload: Vec<u8>, device_id: String) -> Self {
        Self::new(MessageType::PairingResponse, payload, device_id)
    }

    /// 创建心跳消息
    ///
    /// # Arguments
    ///
    /// * `device_id` - 发送方设备 ID
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::heartbeat("device-123".to_string());
    /// assert!(msg.payload.is_empty());
    /// ```
    pub fn heartbeat(device_id: String) -> Self {
        Self::new(MessageType::Heartbeat, Vec::new(), device_id)
    }

    /// 创建确认消息
    ///
    /// # Arguments
    ///
    /// * `device_id` - 发送方设备 ID
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::ack("device-123".to_string());
    /// assert!(msg.payload.is_empty());
    /// ```
    pub fn ack(device_id: String) -> Self {
        Self::new(MessageType::Ack, Vec::new(), device_id)
    }

    /// 创建带 payload 的确认消息
    ///
    /// 用于确认特定消息（payload 可包含被确认消息的标识）
    pub fn ack_with_payload(payload: Vec<u8>, device_id: String) -> Self {
        Self::new(MessageType::Ack, payload, device_id)
    }

    /// 创建取消配对消息
    ///
    /// # Arguments
    ///
    /// * `device_id` - 发送方设备 ID
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::unpair("device-123".to_string());
    /// assert!(msg.payload.is_empty());
    /// ```
    pub fn unpair(device_id: String) -> Self {
        Self::new(MessageType::Unpair, Vec::new(), device_id)
    }

    /// 创建配对拒绝消息
    ///
    /// 当收到来自未配对设备的连接请求时使用，
    /// 通知对方需要先移除本设备再重新配对。
    ///
    /// # Arguments
    ///
    /// * `device_id` - 发送方设备 ID
    /// * `reason` - 拒绝原因（可选）
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::pairing_rejection("device-123".to_string(), Some("Device not in paired list"));
    /// ```
    pub fn pairing_rejection(device_id: String, reason: Option<&str>) -> Self {
        let payload = reason.map(|r| r.as_bytes().to_vec()).unwrap_or_default();
        Self::new(MessageType::PairingRejection, payload, device_id)
    }

    /// 序列化为 MessagePack 字节
    ///
    /// # Returns
    ///
    /// 序列化后的字节向量，或 `ProtocolError::Serialization`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::heartbeat("device-123".to_string());
    /// let bytes = msg.serialize().unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        rmp_serde::to_vec(self).map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    /// 从 MessagePack 字节反序列化
    ///
    /// # Arguments
    ///
    /// * `data` - MessagePack 序列化的字节
    ///
    /// # Returns
    ///
    /// 反序列化后的消息，或 `ProtocolError::Deserialization`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_sync::Message;
    ///
    /// let msg = Message::heartbeat("device-123".to_string());
    /// let bytes = msg.serialize().unwrap();
    /// let decoded = Message::deserialize(&bytes).unwrap();
    /// assert_eq!(msg.device_id, decoded.device_id);
    /// ```
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        rmp_serde::from_slice(data).map_err(|e| ProtocolError::Deserialization(e.to_string()))
    }

    /// 检查消息是否已过期
    ///
    /// # Arguments
    ///
    /// * `max_age_ms` - 最大消息年龄（毫秒）
    ///
    /// # Returns
    ///
    /// 如果消息时间戳早于 (当前时间 - max_age_ms)，返回 true
    pub fn is_expired(&self, max_age_ms: u64) -> bool {
        let now = Self::timestamp_now();
        now.saturating_sub(self.timestamp) > max_age_ms
    }

    /// 获取消息年龄（毫秒）
    ///
    /// # Returns
    ///
    /// 从消息创建到现在的毫秒数
    pub fn age_ms(&self) -> u64 {
        Self::timestamp_now().saturating_sub(self.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_as_str() {
        assert_eq!(MessageType::ClipboardSync.as_str(), "clipboard_sync");
        assert_eq!(MessageType::PairingRequest.as_str(), "pairing_request");
        assert_eq!(MessageType::PairingResponse.as_str(), "pairing_response");
        assert_eq!(MessageType::Heartbeat.as_str(), "heartbeat");
        assert_eq!(MessageType::Ack.as_str(), "ack");
        assert_eq!(MessageType::Unpair.as_str(), "unpair");
    }

    #[test]
    fn test_message_type_requires_ack() {
        assert!(MessageType::ClipboardSync.requires_ack());
        assert!(MessageType::PairingRequest.requires_ack());
        assert!(MessageType::PairingResponse.requires_ack());
        assert!(!MessageType::Heartbeat.requires_ack());
        assert!(!MessageType::Ack.requires_ack());
    }

    #[test]
    fn test_message_type_serialize_roundtrip() {
        let msg_type = MessageType::ClipboardSync;
        let serialized = rmp_serde::to_vec(&msg_type).unwrap();
        let deserialized: MessageType = rmp_serde::from_slice(&serialized).unwrap();
        assert_eq!(msg_type, deserialized);
    }

    #[test]
    fn test_all_message_types_serialize() {
        let types = [
            MessageType::ClipboardSync,
            MessageType::PairingRequest,
            MessageType::PairingResponse,
            MessageType::Heartbeat,
            MessageType::Ack,
            MessageType::Unpair,
        ];
        for msg_type in types {
            let serialized = rmp_serde::to_vec(&msg_type).unwrap();
            let deserialized: MessageType = rmp_serde::from_slice(&serialized).unwrap();
            assert_eq!(msg_type, deserialized);
        }
    }

    #[test]
    fn test_unpair_convenience() {
        let msg = Message::unpair("device-unpair".to_string());
        assert_eq!(msg.msg_type, MessageType::Unpair);
        assert!(msg.payload.is_empty());
        assert_eq!(msg.device_id, "device-unpair");
    }

    #[test]
    fn test_message_new() {
        let msg = Message::new(
            MessageType::ClipboardSync,
            b"test payload".to_vec(),
            "device-123".to_string(),
        );
        assert_eq!(msg.msg_type, MessageType::ClipboardSync);
        assert_eq!(msg.payload, b"test payload".to_vec());
        assert_eq!(msg.device_id, "device-123");
        assert!(msg.timestamp > 0);
    }

    #[test]
    fn test_message_roundtrip() {
        let original = Message::new(
            MessageType::ClipboardSync,
            b"hello world".to_vec(),
            "device-123".to_string(),
        );
        let serialized = original.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();

        assert_eq!(original.msg_type, deserialized.msg_type);
        assert_eq!(original.payload, deserialized.payload);
        assert_eq!(original.device_id, deserialized.device_id);
        assert_eq!(original.timestamp, deserialized.timestamp);
    }

    #[test]
    fn test_timestamp_now() {
        let ts1 = Message::timestamp_now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = Message::timestamp_now();
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_clipboard_sync_convenience() {
        let msg = Message::clipboard_sync(b"test content", "device-456".to_string());
        assert_eq!(msg.msg_type, MessageType::ClipboardSync);
        assert_eq!(msg.payload, b"test content".to_vec());
        assert_eq!(msg.device_id, "device-456");
    }

    #[test]
    fn test_pairing_request_convenience() {
        let payload = b"public_key_data".to_vec();
        let msg = Message::pairing_request(payload.clone(), "device-789".to_string());
        assert_eq!(msg.msg_type, MessageType::PairingRequest);
        assert_eq!(msg.payload, payload);
    }

    #[test]
    fn test_pairing_response_convenience() {
        let payload = b"response_data".to_vec();
        let msg = Message::pairing_response(payload.clone(), "device-abc".to_string());
        assert_eq!(msg.msg_type, MessageType::PairingResponse);
        assert_eq!(msg.payload, payload);
    }

    #[test]
    fn test_heartbeat_convenience() {
        let msg = Message::heartbeat("device-789".to_string());
        assert_eq!(msg.msg_type, MessageType::Heartbeat);
        assert!(msg.payload.is_empty());
        assert_eq!(msg.device_id, "device-789");
    }

    #[test]
    fn test_ack_convenience() {
        let msg = Message::ack("device-abc".to_string());
        assert_eq!(msg.msg_type, MessageType::Ack);
        assert!(msg.payload.is_empty());
        assert_eq!(msg.device_id, "device-abc");
    }

    #[test]
    fn test_ack_with_payload() {
        let payload = b"message_id_123".to_vec();
        let msg = Message::ack_with_payload(payload.clone(), "device-def".to_string());
        assert_eq!(msg.msg_type, MessageType::Ack);
        assert_eq!(msg.payload, payload);
    }

    #[test]
    fn test_empty_payload() {
        let msg = Message::new(MessageType::Heartbeat, Vec::new(), "device-def".to_string());
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert!(deserialized.payload.is_empty());
    }

    #[test]
    fn test_large_payload() {
        let large_payload = vec![0u8; 10000];
        let msg = Message::new(
            MessageType::ClipboardSync,
            large_payload.clone(),
            "device-large".to_string(),
        );
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.payload.len(), 10000);
        assert_eq!(deserialized.payload, large_payload);
    }

    #[test]
    fn test_binary_payload() {
        // Test with binary data including null bytes
        let binary_payload: Vec<u8> = (0u8..=255).collect();
        let msg = Message::new(
            MessageType::ClipboardSync,
            binary_payload.clone(),
            "device-bin".to_string(),
        );
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.payload, binary_payload);
    }

    #[test]
    fn test_unicode_device_id() {
        let msg = Message::heartbeat("设备-123-émoji-🎉".to_string());
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.device_id, "设备-123-émoji-🎉");
    }

    #[test]
    fn test_message_age() {
        let msg = Message::heartbeat("device-age".to_string());
        std::thread::sleep(std::time::Duration::from_millis(50));
        let age = msg.age_ms();
        assert!(age >= 50);
    }

    #[test]
    fn test_message_is_expired() {
        let msg = Message::heartbeat("device-exp".to_string());

        // Should not be expired with large max_age
        assert!(!msg.is_expired(10000));

        // Wait a bit
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Should be expired with small max_age
        assert!(msg.is_expired(50));
    }

    #[test]
    fn test_deserialize_invalid_data() {
        let invalid_data = b"not valid messagepack";
        let result = Message::deserialize(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_clone() {
        let msg = Message::clipboard_sync(b"clone test", "device-clone".to_string());
        let cloned = msg.clone();
        assert_eq!(msg, cloned);
    }

    #[test]
    fn test_message_debug_format() {
        let msg = Message::heartbeat("device-debug".to_string());
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Heartbeat"));
        assert!(debug_str.contains("device-debug"));
    }

    #[test]
    fn test_message_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(MessageType::ClipboardSync);
        set.insert(MessageType::Heartbeat);

        assert!(set.contains(&MessageType::ClipboardSync));
        assert!(set.contains(&MessageType::Heartbeat));
        assert!(!set.contains(&MessageType::Ack));
    }

    #[test]
    fn test_message_type_default() {
        let default_type = MessageType::default();
        assert_eq!(default_type, MessageType::Heartbeat);
    }

    #[test]
    fn test_protocol_error_display() {
        let ser_err = ProtocolError::Serialization("test error".to_string());
        assert_eq!(ser_err.to_string(), "Serialization error: test error");

        let de_err = ProtocolError::Deserialization("invalid data".to_string());
        assert_eq!(de_err.to_string(), "Deserialization error: invalid data");
    }

    #[test]
    fn test_protocol_error_clone_eq() {
        let err1 = ProtocolError::Serialization("test".to_string());
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_deserialize_error_type() {
        let invalid_data = b"not valid messagepack";
        let result = Message::deserialize(invalid_data);
        assert!(matches!(result, Err(ProtocolError::Deserialization(_))));
    }
}
