# Task 0305: 实现传输协议处理 (TDD版本)

## 任务描述

按照TDD原则实现文本传输的协议处理，定义传输数据格式和协议规范。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transfer_protocol_tests.rs
#[cfg(test)]
mod transfer_protocol_tests {
    use super::*;
    
    #[test]
    fn test_protocol_header_serialization() {
        // RED: 测试协议头序列化
        let header = ProtocolHeader::new("session-123", "sender-001", "receiver-001");
        
        let serialized = header.serialize();
        let deserialized = ProtocolHeader::deserialize(&serialized).unwrap();
        
        assert_eq!(deserialized.session_id(), "session-123");
        assert_eq!(deserialized.sender_id(), "sender-001");
        assert_eq!(deserialized.receiver_id(), "receiver-001");
    }
    
    #[test]
    fn test_transfer_packet_creation() {
        // RED: 测试传输包创建
        let packet = TransferPacket::new(
            "session-123",
            1,
            b"Hello, World!".to_vec(),
            PacketType::Data,
        );
        
        assert_eq!(packet.session_id(), "session-123");
        assert_eq!(packet.sequence(), 1);
        assert_eq!(packet.payload(), b"Hello, World!");
        assert!(matches!(packet.packet_type(), PacketType::Data));
    }
    
    #[test]
    fn test_protocol_validation() {
        // RED: 测试协议验证
        let header = ProtocolHeader::new("", "sender", "receiver"); // 无效的session_id
        
        let result = header.validate();
        assert!(matches!(result, Err(ProtocolError::InvalidSessionId)));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum PacketType {
    Handshake,
    Data,
    Ack,
    Complete,
    Error,
}

#[derive(Debug)]
pub struct ProtocolHeader {
    session_id: String,
    sender_id: String,
    receiver_id: String,
}

impl ProtocolHeader {
    pub fn new(session_id: &str, sender_id: &str, receiver_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
        }
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    
    pub fn sender_id(&self) -> &str {
        &self.sender_id
    }
    
    pub fn receiver_id(&self) -> &str {
        &self.receiver_id
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        format!("{}|{}|{}", self.session_id, self.sender_id, self.receiver_id)
            .as_bytes()
            .to_vec()
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        let str_data = String::from_utf8(data.to_vec())
            .map_err(|_| ProtocolError::InvalidEncoding)?;
        
        let parts: Vec<&str> = str_data.split('|').collect();
        if parts.len() != 3 {
            return Err(ProtocolError::InvalidFormat);
        }
        
        Ok(Self {
            session_id: parts[0].to_string(),
            sender_id: parts[1].to_string(),
            receiver_id: parts[2].to_string(),
        })
    }
    
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.session_id.is_empty() {
            return Err(ProtocolError::InvalidSessionId);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TransferPacket {
    header: ProtocolHeader,
    sequence: u32,
    payload: Vec<u8>,
    packet_type: PacketType,
}

impl TransferPacket {
    pub fn new(session_id: &str, sequence: u32, payload: Vec<u8>, packet_type: PacketType) -> Self {
        Self {
            header: ProtocolHeader::new(session_id, "sender", "receiver"),
            sequence,
            payload,
            packet_type,
        }
    }
    
    pub fn session_id(&self) -> &str {
        self.header.session_id()
    }
    
    pub fn sequence(&self) -> u32 {
        self.sequence
    }
    
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    
    pub fn packet_type(&self) -> &PacketType {
        &self.packet_type
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidSessionId,
    InvalidEncoding,
    InvalidFormat,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketType {
    Handshake,
    Data,
    Ack,
    Complete,
    Error,
    Ping,
    Pong,
    KeepAlive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
}

#[derive(Debug, Clone)]
pub struct ProtocolHeader {
    protocol_version: u8,
    session_id: String,
    sender_id: String,
    receiver_id: String,
    message_id: String,
    timestamp: SystemTime,
    flags: ProtocolFlags,
}

#[derive(Debug, Clone)]
pub struct ProtocolFlags {
    pub is_encrypted: bool,
    pub is_compressed: bool,
    pub requires_ack: bool,
    pub is_retransmission: bool,
}

#[derive(Debug)]
pub struct TransferPacket {
    header: ProtocolHeader,
    sequence: u32,
    total_packets: u32,
    payload: Vec<u8>,
    packet_type: PacketType,
    compression: CompressionType,
    checksum: u32,
}

#[derive(Debug)]
pub struct ProtocolConfig {
    pub max_packet_size: usize,
    pub max_sequence_number: u32,
    pub timeout_duration: Duration,
    pub retry_attempts: u32,
    pub enable_compression: bool,
    pub enable_encryption: bool,
}

impl TransferPacket {
    // 重构后的代码，保持测试绿色
    pub fn new(session_id: &str, sequence: u32, payload: Vec<u8>, packet_type: PacketType) -> Self {
        Self::with_config(
            session_id,
            sequence,
            payload,
            packet_type,
            ProtocolConfig::default(),
        )
    }
    
    pub fn with_config(
        session_id: &str,
        sequence: u32,
        payload: Vec<u8>,
        packet_type: PacketType,
        config: ProtocolConfig,
    ) -> Self {
        let mut packet = Self {
            header: ProtocolHeader::new(session_id, "sender", "receiver"),
            sequence,
            total_packets: 1, // 简化实现
            payload,
            packet_type,
            compression: CompressionType::None,
            checksum: 0,
        };
        
        packet.checksum = packet.calculate_checksum();
        packet
    }
    
    pub fn session_id(&self) -> &str {
        &self.header.session_id
    }
    
    pub fn sequence(&self) -> u32 {
        self.sequence
    }
    
    pub fn total_packets(&self) -> u32 {
        self.total_packets
    }
    
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    
    pub fn packet_type(&self) -> &PacketType {
        &self.packet_type
    }
    
    pub fn compression(&self) -> &CompressionType {
        &self.compression
    }
    
    pub fn checksum(&self) -> u32 {
        self.checksum
    }
    
    pub fn validate(&self) -> Result<(), ProtocolError> {
        self.header.validate()?;
        
        if self.calculate_checksum() != self.checksum {
            return Err(ProtocolError::ChecksumMismatch);
        }
        
        if self.payload.len() > self.max_packet_size() {
            return Err(ProtocolError::PacketTooLarge);
        }
        
        Ok(())
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Vec::new();
        
        // 序列化头部
        buffer.extend_from_slice(&self.header.serialize()?);
        
        // 序列化包数据
        buffer.extend_from_slice(&self.sequence.to_le_bytes());
        buffer.extend_from_slice(&self.total_packets.to_le_bytes());
        buffer.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.payload);
        buffer.extend_from_slice(&self.checksum.to_le_bytes());
        
        Ok(buffer)
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        let header = ProtocolHeader::deserialize(&data[0..32])?;
        
        let mut offset = 32;
        let sequence = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let total_packets = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let payload_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        
        let payload = data[offset..offset + payload_len].to_vec();
        offset += payload_len;
        
        let checksum = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        
        Ok(Self {
            header,
            sequence,
            total_packets,
            payload,
            packet_type: PacketType::Data, // 简化实现
            compression: CompressionType::None,
            checksum,
        })
    }
    
    pub fn max_packet_size(&self) -> usize {
        1024 // 简化实现
    }
    
    fn calculate_checksum(&self) -> u32 {
        use crc32fast::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&self.payload);
        hasher.finalize()
    }
}

impl ProtocolHeader {
    pub fn new(session_id: &str, sender_id: &str, receiver_id: &str) -> Self {
        Self {
            protocol_version: 1,
            session_id: session_id.to_string(),
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
            message_id: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now(),
            flags: ProtocolFlags {
                is_encrypted: false,
                is_compressed: false,
                requires_ack: true,
                is_retransmission: false,
            },
        }
    }
    
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    
    pub fn sender_id(&self) -> &str {
        &self.sender_id
    }
    
    pub fn receiver_id(&self) -> &str {
        &self.receiver_id
    }
    
    pub fn message_id(&self) -> &str {
        &self.message_id
    }
    
    pub fn timestamp(&self) -> &SystemTime {
        &self.timestamp
    }
    
    pub fn flags(&self) -> &ProtocolFlags {
        &self.flags
    }
    
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.session_id.is_empty() || self.session_id.len() > 64 {
            return Err(ProtocolError::InvalidSessionId);
        }
        
        if self.sender_id.is_empty() || self.sender_id.len() > 64 {
            return Err(ProtocolError::InvalidSenderId);
        }
        
        if self.receiver_id.is_empty() || self.receiver_id.len() > 64 {
            return Err(ProtocolError::InvalidReceiverId);
        }
        
        Ok(())
    }
    
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Vec::with_capacity(32);
        buffer.push(self.protocol_version);
        
        // Session ID (16 bytes padded)
        let session_bytes = self.session_id.as_bytes();
        buffer.extend_from_slice(&session_bytes);
        buffer.resize(1 + 16, 0);
        
        // Sender ID (8 bytes padded)
        let sender_bytes = self.sender_id.as_bytes();
        buffer.extend_from_slice(&sender_bytes);
        buffer.resize(1 + 16 + 8, 0);
        
        // Receiver ID (8 bytes padded)
        let receiver_bytes = self.receiver_id.as_bytes();
        buffer.extend_from_slice(&receiver_bytes);
        buffer.resize(32, 0);
        
        Ok(buffer)
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        if data.len() < 32 {
            return Err(ProtocolError::InvalidHeaderLength);
        }
        
        let protocol_version = data[0];
        let session_id = String::from_utf8_lossy(&data[1..17])
            .trim_end_matches('\0')
            .to_string();
        let sender_id = String::from_utf8_lossy(&data[17..25])
            .trim_end_matches('\0')
            .to_string();
        let receiver_id = String::from_utf8_lossy(&data[25..33])
            .trim_end_matches('\0')
            .to_string();
        
        Ok(Self {
            protocol_version,
            session_id,
            sender_id,
            receiver_id,
            message_id: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now(),
            flags: ProtocolFlags::default(),
        })
    }
}

impl Default for ProtocolFlags {
    fn default() -> Self {
        Self {
            is_encrypted: false,
            is_compressed: false,
            requires_ack: true,
            is_retransmission: false,
        }
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            max_packet_size: 1024,
            max_sequence_number: u32::MAX,
            timeout_duration: Duration::from_secs(30),
            retry_attempts: 3,
            enable_compression: false,
            enable_encryption: false,
        }
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidSessionId,
    InvalidSenderId,
    InvalidReceiverId,
    InvalidHeaderLength,
    InvalidEncoding,
    InvalidFormat,
    ChecksumMismatch,
    PacketTooLarge,
    UnsupportedProtocolVersion,
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为infrastructure层的协议处理：

```rust
// rust-core/infrastructure/protocol/transfer.rs
pub struct TransferPacket {
    // 传输协议包实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [传输队列管理](0304-transfer-queue-management.md)

## 后续任务

- [Task 0306: 实现传输进度跟踪](0306-transfer-progress-tracking.md)
- [Task 0307: 实现传输状态监控](0307-transfer-status-monitor.md)
- [Task 0308: 实现传输数据压缩](0308-transfer-data-compression.md)