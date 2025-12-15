//! BLE 数据分片模块
//!
//! 处理 BLE MTU 限制下的数据分片和重组。
//!
//! # 概述
//!
//! BLE 的 ATT MTU 限制了单次传输的数据量。本模块提供：
//! - `Chunker`: 将大数据分片为符合 MTU 的小块
//! - `Reassembler`: 将收到的分片重组为完整数据
//!
//! # 分片格式
//!
//! 每个分片由头部 (8 bytes) + payload 组成：
//!
//! ```text
//! +----------------+----------------+----------------+----------------+
//! | message_id (2) | sequence (2)   | total (2)      | length (2)     |
//! +----------------+----------------+----------------+----------------+
//! |                          payload (variable)                       |
//! +------------------------------------------------------------------+
//! ```
//!
//! # Example
//!
//! ```
//! use nearclip_ble::chunk::{Chunker, Reassembler, DEFAULT_REASSEMBLE_TIMEOUT};
//!
//! // 分片数据
//! let data = b"Hello, World! This is a test message.";
//! let mtu = 23; // 最小 MTU
//! let message_id = 1;
//!
//! let chunks = Chunker::chunk(data, message_id, mtu).unwrap();
//! assert!(chunks.len() > 1); // 数据被分片
//!
//! // 重组数据
//! use nearclip_ble::chunk::ChunkHeader;
//!
//! let header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
//! let mut reassembler = Reassembler::new(
//!     header.message_id,
//!     header.total_chunks,
//!     DEFAULT_REASSEMBLE_TIMEOUT,
//! );
//!
//! for chunk in &chunks {
//!     let header = ChunkHeader::from_bytes(chunk).unwrap();
//!     let payload = chunk[8..].to_vec();
//!     reassembler.add_chunk(header, payload).unwrap();
//! }
//!
//! assert!(reassembler.is_complete());
//! let assembled = reassembler.assemble().unwrap();
//! assert_eq!(assembled, data);
//! ```

use crate::gatt::{ATT_HEADER_SIZE, CHUNK_HEADER_SIZE};
use crate::BleError;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, trace, warn};

/// 默认重组超时时间 (30 秒)
pub const DEFAULT_REASSEMBLE_TIMEOUT: Duration = Duration::from_secs(30);

/// 分片头部
///
/// 每个数据分片的头部信息，用于标识分片属于哪个消息以及在消息中的位置。
///
/// # 字段
///
/// - `message_id`: 消息唯一标识符 (2 bytes)
/// - `sequence_number`: 当前分片序号，从 0 开始 (2 bytes)
/// - `total_chunks`: 总分片数 (2 bytes)
/// - `payload_length`: 当前分片的 payload 长度 (2 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHeader {
    /// 消息 ID（用于识别属于同一消息的分片）
    pub message_id: u16,
    /// 当前分片序号（从 0 开始）
    pub sequence_number: u16,
    /// 总分片数
    pub total_chunks: u16,
    /// 当前分片 payload 长度
    pub payload_length: u16,
}

impl ChunkHeader {
    /// 从字节数组解析头部
    ///
    /// # Arguments
    ///
    /// * `bytes` - 包含头部数据的字节数组，至少需要 8 字节
    ///
    /// # Returns
    ///
    /// 成功返回 `ChunkHeader`，失败返回 `BleError::ChunkError`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::chunk::ChunkHeader;
    ///
    /// let bytes = [0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x0A, 0x00];
    /// let header = ChunkHeader::from_bytes(&bytes).unwrap();
    ///
    /// assert_eq!(header.message_id, 1);
    /// assert_eq!(header.sequence_number, 2);
    /// assert_eq!(header.total_chunks, 5);
    /// assert_eq!(header.payload_length, 10);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BleError> {
        if bytes.len() < CHUNK_HEADER_SIZE {
            return Err(BleError::ChunkError(format!(
                "Header too short: {} bytes, expected at least {}",
                bytes.len(),
                CHUNK_HEADER_SIZE
            )));
        }

        Ok(Self {
            message_id: u16::from_le_bytes([bytes[0], bytes[1]]),
            sequence_number: u16::from_le_bytes([bytes[2], bytes[3]]),
            total_chunks: u16::from_le_bytes([bytes[4], bytes[5]]),
            payload_length: u16::from_le_bytes([bytes[6], bytes[7]]),
        })
    }

    /// 序列化为字节数组
    ///
    /// # Returns
    ///
    /// 8 字节的头部数据
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::chunk::ChunkHeader;
    ///
    /// let header = ChunkHeader {
    ///     message_id: 1,
    ///     sequence_number: 0,
    ///     total_chunks: 3,
    ///     payload_length: 12,
    /// };
    ///
    /// let bytes = header.to_bytes();
    /// assert_eq!(bytes.len(), 8);
    /// ```
    pub fn to_bytes(&self) -> [u8; CHUNK_HEADER_SIZE] {
        let mut bytes = [0u8; CHUNK_HEADER_SIZE];
        bytes[0..2].copy_from_slice(&self.message_id.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.sequence_number.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.total_chunks.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.payload_length.to_le_bytes());
        bytes
    }

    /// 创建新的头部
    pub fn new(
        message_id: u16,
        sequence_number: u16,
        total_chunks: u16,
        payload_length: u16,
    ) -> Self {
        Self {
            message_id,
            sequence_number,
            total_chunks,
            payload_length,
        }
    }
}

/// 数据分片器
///
/// 将大数据块分割为符合 BLE MTU 限制的小分片。
pub struct Chunker;

impl Chunker {
    /// 将数据分片
    ///
    /// 根据指定的 MTU 大小，将数据分割为多个分片。每个分片包含：
    /// - 8 字节头部
    /// - 可变长度 payload
    ///
    /// # Arguments
    ///
    /// * `data` - 要分片的原始数据
    /// * `message_id` - 消息唯一标识符
    /// * `mtu` - BLE MTU 大小（包含 ATT 头部）
    ///
    /// # Returns
    ///
    /// 分片数据列表，每个元素是完整的分片（头部 + payload）
    ///
    /// # Errors
    ///
    /// - `BleError::ChunkError` - MTU 太小或数据为空
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::chunk::Chunker;
    ///
    /// let data = b"Hello, World!";
    /// let chunks = Chunker::chunk(data, 1, 23).unwrap();
    ///
    /// // 每个分片都是完整的（头部 + payload）
    /// for chunk in &chunks {
    ///     assert!(chunk.len() >= 8); // 至少有头部
    /// }
    /// ```
    pub fn chunk(data: &[u8], message_id: u16, mtu: usize) -> Result<Vec<Vec<u8>>, BleError> {
        // 计算有效 payload 大小
        // MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE
        let effective_mtu = mtu.saturating_sub(ATT_HEADER_SIZE);
        if effective_mtu <= CHUNK_HEADER_SIZE {
            return Err(BleError::ChunkError(format!(
                "MTU too small: {} (effective: {}), minimum needed: {}",
                mtu,
                effective_mtu,
                ATT_HEADER_SIZE + CHUNK_HEADER_SIZE + 1
            )));
        }

        let max_payload_size = effective_mtu - CHUNK_HEADER_SIZE;

        // 空数据特殊处理：发送一个空 payload 的分片
        if data.is_empty() {
            let header = ChunkHeader::new(message_id, 0, 1, 0);
            let mut chunk = Vec::with_capacity(CHUNK_HEADER_SIZE);
            chunk.extend_from_slice(&header.to_bytes());
            return Ok(vec![chunk]);
        }

        // 计算需要的分片数
        let total_chunks = data.len().div_ceil(max_payload_size);
        if total_chunks > u16::MAX as usize {
            return Err(BleError::ChunkError(format!(
                "Data too large: {} bytes would require {} chunks (max: {})",
                data.len(),
                total_chunks,
                u16::MAX
            )));
        }

        let total_chunks = total_chunks as u16;
        let mut chunks = Vec::with_capacity(total_chunks as usize);

        debug!(
            message_id,
            data_len = data.len(),
            total_chunks,
            max_payload_size,
            "Chunking data"
        );

        for (i, chunk_data) in data.chunks(max_payload_size).enumerate() {
            let header = ChunkHeader::new(
                message_id,
                i as u16,
                total_chunks,
                chunk_data.len() as u16,
            );

            let mut chunk = Vec::with_capacity(CHUNK_HEADER_SIZE + chunk_data.len());
            chunk.extend_from_slice(&header.to_bytes());
            chunk.extend_from_slice(chunk_data);

            trace!(
                message_id,
                sequence = i,
                payload_len = chunk_data.len(),
                "Created chunk"
            );

            chunks.push(chunk);
        }

        Ok(chunks)
    }
}

/// 数据重组器
///
/// 收集分片并在所有分片到齐后重组为完整数据。
///
/// # 特性
///
/// - 支持乱序接收分片
/// - 自动去重（相同序号的分片只保留第一个）
/// - 支持超时检测
///
/// # Example
///
/// ```
/// use nearclip_ble::chunk::{Reassembler, ChunkHeader, DEFAULT_REASSEMBLE_TIMEOUT};
///
/// let mut reassembler = Reassembler::new(1, 3, DEFAULT_REASSEMBLE_TIMEOUT);
///
/// // 可以乱序添加分片
/// reassembler.add_chunk(
///     ChunkHeader::new(1, 2, 3, 5),
///     b"World".to_vec(),
/// ).unwrap();
///
/// reassembler.add_chunk(
///     ChunkHeader::new(1, 0, 3, 6),
///     b"Hello,".to_vec(),
/// ).unwrap();
///
/// reassembler.add_chunk(
///     ChunkHeader::new(1, 1, 3, 1),
///     b" ".to_vec(),
/// ).unwrap();
///
/// assert!(reassembler.is_complete());
/// let data = reassembler.assemble().unwrap();
/// assert_eq!(data, b"Hello, World");
/// ```
#[derive(Debug)]
pub struct Reassembler {
    /// 消息 ID
    message_id: u16,
    /// 预期分片总数
    total_chunks: u16,
    /// 已接收的分片 (sequence_number -> payload)
    chunks: HashMap<u16, Vec<u8>>,
    /// 创建时间（用于超时检测）
    created_at: Instant,
    /// 超时时间
    timeout: Duration,
}

impl Reassembler {
    /// 创建新的重组器
    ///
    /// # Arguments
    ///
    /// * `message_id` - 消息 ID
    /// * `total_chunks` - 预期分片总数
    /// * `timeout` - 超时时间
    pub fn new(message_id: u16, total_chunks: u16, timeout: Duration) -> Self {
        debug!(
            message_id,
            total_chunks,
            timeout_secs = timeout.as_secs(),
            "Created reassembler"
        );

        Self {
            message_id,
            total_chunks,
            chunks: HashMap::with_capacity(total_chunks as usize),
            created_at: Instant::now(),
            timeout,
        }
    }

    /// 添加分片
    ///
    /// # Arguments
    ///
    /// * `header` - 分片头部
    /// * `payload` - 分片数据
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::ChunkError` - message_id 不匹配或 sequence 无效
    pub fn add_chunk(&mut self, header: ChunkHeader, payload: Vec<u8>) -> Result<(), BleError> {
        // 验证 message_id
        if header.message_id != self.message_id {
            return Err(BleError::ChunkError(format!(
                "Message ID mismatch: expected {}, got {}",
                self.message_id, header.message_id
            )));
        }

        // 验证 total_chunks
        if header.total_chunks != self.total_chunks {
            return Err(BleError::ChunkError(format!(
                "Total chunks mismatch: expected {}, got {}",
                self.total_chunks, header.total_chunks
            )));
        }

        // 验证 sequence_number
        if header.sequence_number >= self.total_chunks {
            return Err(BleError::ChunkError(format!(
                "Invalid sequence number: {} (total: {})",
                header.sequence_number, self.total_chunks
            )));
        }

        // 验证 payload_length
        if header.payload_length as usize != payload.len() {
            return Err(BleError::ChunkError(format!(
                "Payload length mismatch: header says {}, actual {}",
                header.payload_length,
                payload.len()
            )));
        }

        // 检查是否重复
        if self.chunks.contains_key(&header.sequence_number) {
            warn!(
                message_id = self.message_id,
                sequence = header.sequence_number,
                "Duplicate chunk received, ignoring"
            );
            return Ok(());
        }

        trace!(
            message_id = self.message_id,
            sequence = header.sequence_number,
            payload_len = payload.len(),
            "Added chunk"
        );

        self.chunks.insert(header.sequence_number, payload);

        Ok(())
    }

    /// 检查是否已收到所有分片
    pub fn is_complete(&self) -> bool {
        self.chunks.len() == self.total_chunks as usize
    }

    /// 检查是否已超时
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.timeout
    }

    /// 获取已接收的分片数
    pub fn received_count(&self) -> usize {
        self.chunks.len()
    }

    /// 获取消息 ID
    pub fn message_id(&self) -> u16 {
        self.message_id
    }

    /// 重组完整数据
    ///
    /// 将所有分片按序号排序并拼接为完整数据。
    ///
    /// # Returns
    ///
    /// 成功返回完整数据，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::ChunkError` - 分片不完整
    pub fn assemble(self) -> Result<Vec<u8>, BleError> {
        if !self.is_complete() {
            return Err(BleError::ChunkError(format!(
                "Incomplete message: received {}/{} chunks",
                self.chunks.len(),
                self.total_chunks
            )));
        }

        // 计算总大小
        let total_size: usize = self.chunks.values().map(|v| v.len()).sum();
        let mut result = Vec::with_capacity(total_size);

        // 按序号排序并拼接
        for i in 0..self.total_chunks {
            if let Some(payload) = self.chunks.get(&i) {
                result.extend_from_slice(payload);
            } else {
                return Err(BleError::ChunkError(format!(
                    "Missing chunk at sequence {}",
                    i
                )));
            }
        }

        debug!(
            message_id = self.message_id,
            total_size,
            chunks = self.total_chunks,
            "Assembled message"
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // ChunkHeader Tests
    // ========================================

    #[test]
    fn test_chunk_header_new() {
        let header = ChunkHeader::new(100, 5, 10, 128);

        assert_eq!(header.message_id, 100);
        assert_eq!(header.sequence_number, 5);
        assert_eq!(header.total_chunks, 10);
        assert_eq!(header.payload_length, 128);
    }

    #[test]
    fn test_chunk_header_to_bytes() {
        let header = ChunkHeader::new(1, 2, 5, 10);
        let bytes = header.to_bytes();

        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0..2], [0x01, 0x00]); // message_id = 1
        assert_eq!(bytes[2..4], [0x02, 0x00]); // sequence = 2
        assert_eq!(bytes[4..6], [0x05, 0x00]); // total = 5
        assert_eq!(bytes[6..8], [0x0A, 0x00]); // length = 10
    }

    #[test]
    fn test_chunk_header_from_bytes() {
        let bytes = [0x64, 0x00, 0x03, 0x00, 0x0A, 0x00, 0x14, 0x00];
        let header = ChunkHeader::from_bytes(&bytes).unwrap();

        assert_eq!(header.message_id, 100);
        assert_eq!(header.sequence_number, 3);
        assert_eq!(header.total_chunks, 10);
        assert_eq!(header.payload_length, 20);
    }

    #[test]
    fn test_chunk_header_from_bytes_too_short() {
        let bytes = [0x01, 0x00, 0x02, 0x00]; // Only 4 bytes
        let result = ChunkHeader::from_bytes(&bytes);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Header too short"));
    }

    #[test]
    fn test_chunk_header_roundtrip() {
        let original = ChunkHeader::new(12345, 67, 100, 200);
        let bytes = original.to_bytes();
        let parsed = ChunkHeader::from_bytes(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    // ========================================
    // Chunker Tests
    // ========================================

    #[test]
    fn test_chunker_small_data_single_chunk() {
        // 数据小于 payload 大小，应该只产生一个分片
        let data = b"Hi";
        let mtu = 23; // payload = 23 - 3 - 8 = 12 bytes
        let chunks = Chunker::chunk(data, 1, mtu).unwrap();

        assert_eq!(chunks.len(), 1);

        let header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        assert_eq!(header.message_id, 1);
        assert_eq!(header.sequence_number, 0);
        assert_eq!(header.total_chunks, 1);
        assert_eq!(header.payload_length, 2);

        let payload = &chunks[0][8..];
        assert_eq!(payload, b"Hi");
    }

    #[test]
    fn test_chunker_exact_mtu_data() {
        // 数据正好等于 payload 大小
        let data = b"123456789012"; // 12 bytes
        let mtu = 23; // payload = 12 bytes
        let chunks = Chunker::chunk(data, 1, mtu).unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 8 + 12); // header + payload
    }

    #[test]
    fn test_chunker_multi_chunk() {
        // 数据需要多个分片
        let data = b"Hello, World! This is a test."; // 29 bytes
        let mtu = 23; // payload = 12 bytes per chunk
        let chunks = Chunker::chunk(data, 42, mtu).unwrap();

        // 29 bytes / 12 bytes per chunk = 3 chunks (ceil)
        assert_eq!(chunks.len(), 3);

        // 验证每个分片的头部
        for (i, chunk) in chunks.iter().enumerate() {
            let header = ChunkHeader::from_bytes(chunk).unwrap();
            assert_eq!(header.message_id, 42);
            assert_eq!(header.sequence_number, i as u16);
            assert_eq!(header.total_chunks, 3);
        }

        // 验证最后一个分片的 payload 长度
        let last_header = ChunkHeader::from_bytes(&chunks[2]).unwrap();
        assert_eq!(last_header.payload_length, 5); // 29 - 12 - 12 = 5
    }

    #[test]
    fn test_chunker_empty_data() {
        let data: &[u8] = b"";
        let chunks = Chunker::chunk(data, 1, 23).unwrap();

        assert_eq!(chunks.len(), 1);

        let header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        assert_eq!(header.payload_length, 0);
        assert_eq!(chunks[0].len(), 8); // Only header
    }

    #[test]
    fn test_chunker_mtu_too_small() {
        let data = b"test";
        let result = Chunker::chunk(data, 1, 10); // Too small

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MTU too small"));
    }

    #[test]
    fn test_chunker_large_mtu() {
        // 使用较大的 MTU
        let data = b"Hello, World!";
        let mtu = 512; // Large MTU
        let chunks = Chunker::chunk(data, 1, mtu).unwrap();

        assert_eq!(chunks.len(), 1); // Should fit in one chunk
    }

    // ========================================
    // Reassembler Tests
    // ========================================

    #[test]
    fn test_reassembler_single_chunk() {
        let mut reassembler = Reassembler::new(1, 1, DEFAULT_REASSEMBLE_TIMEOUT);

        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 1, 5), b"Hello".to_vec())
            .unwrap();

        assert!(reassembler.is_complete());

        let data = reassembler.assemble().unwrap();
        assert_eq!(data, b"Hello");
    }

    #[test]
    fn test_reassembler_in_order() {
        let mut reassembler = Reassembler::new(1, 3, DEFAULT_REASSEMBLE_TIMEOUT);

        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 3, 6), b"Hello,".to_vec())
            .unwrap();
        reassembler
            .add_chunk(ChunkHeader::new(1, 1, 3, 1), b" ".to_vec())
            .unwrap();
        reassembler
            .add_chunk(ChunkHeader::new(1, 2, 3, 6), b"World!".to_vec())
            .unwrap();

        assert!(reassembler.is_complete());

        let data = reassembler.assemble().unwrap();
        assert_eq!(data, b"Hello, World!");
    }

    #[test]
    fn test_reassembler_out_of_order() {
        let mut reassembler = Reassembler::new(1, 3, DEFAULT_REASSEMBLE_TIMEOUT);

        // 乱序添加
        reassembler
            .add_chunk(ChunkHeader::new(1, 2, 3, 6), b"World!".to_vec())
            .unwrap();
        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 3, 6), b"Hello,".to_vec())
            .unwrap();
        reassembler
            .add_chunk(ChunkHeader::new(1, 1, 3, 1), b" ".to_vec())
            .unwrap();

        assert!(reassembler.is_complete());

        let data = reassembler.assemble().unwrap();
        assert_eq!(data, b"Hello, World!");
    }

    #[test]
    fn test_reassembler_duplicate_chunk() {
        let mut reassembler = Reassembler::new(1, 2, DEFAULT_REASSEMBLE_TIMEOUT);

        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 2, 5), b"Hello".to_vec())
            .unwrap();

        // 添加重复的分片（应被忽略）
        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 2, 5), b"Hello".to_vec())
            .unwrap();

        assert!(!reassembler.is_complete()); // 仍然缺少分片 1
        assert_eq!(reassembler.received_count(), 1);
    }

    #[test]
    fn test_reassembler_wrong_message_id() {
        let mut reassembler = Reassembler::new(1, 2, DEFAULT_REASSEMBLE_TIMEOUT);

        let result =
            reassembler.add_chunk(ChunkHeader::new(2, 0, 2, 5), b"Hello".to_vec()); // Wrong ID

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Message ID mismatch"));
    }

    #[test]
    fn test_reassembler_invalid_sequence() {
        let mut reassembler = Reassembler::new(1, 2, DEFAULT_REASSEMBLE_TIMEOUT);

        let result =
            reassembler.add_chunk(ChunkHeader::new(1, 5, 2, 5), b"Hello".to_vec()); // Invalid seq

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid sequence number"));
    }

    #[test]
    fn test_reassembler_payload_length_mismatch() {
        let mut reassembler = Reassembler::new(1, 2, DEFAULT_REASSEMBLE_TIMEOUT);

        let result =
            reassembler.add_chunk(ChunkHeader::new(1, 0, 2, 10), b"Hello".to_vec()); // Wrong len

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Payload length mismatch"));
    }

    #[test]
    fn test_reassembler_incomplete_assemble() {
        let mut reassembler = Reassembler::new(1, 3, DEFAULT_REASSEMBLE_TIMEOUT);

        reassembler
            .add_chunk(ChunkHeader::new(1, 0, 3, 5), b"Hello".to_vec())
            .unwrap();
        // Missing chunks 1 and 2

        assert!(!reassembler.is_complete());

        let result = reassembler.assemble();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Incomplete"));
    }

    #[test]
    fn test_reassembler_expired() {
        let reassembler = Reassembler::new(1, 2, Duration::from_millis(1));

        std::thread::sleep(Duration::from_millis(10));

        assert!(reassembler.is_expired());
    }

    #[test]
    fn test_reassembler_not_expired() {
        let reassembler = Reassembler::new(1, 2, Duration::from_secs(60));

        assert!(!reassembler.is_expired());
    }

    // ========================================
    // Chunker + Reassembler Integration Tests
    // ========================================

    #[test]
    fn test_chunk_and_reassemble_small_data() {
        let original_data = b"Hello";
        let message_id = 123;
        let mtu = 23;

        // 分片
        let chunks = Chunker::chunk(original_data, message_id, mtu).unwrap();

        // 重组
        let first_header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        let mut reassembler = Reassembler::new(
            first_header.message_id,
            first_header.total_chunks,
            DEFAULT_REASSEMBLE_TIMEOUT,
        );

        for chunk in &chunks {
            let header = ChunkHeader::from_bytes(chunk).unwrap();
            let payload = chunk[8..].to_vec();
            reassembler.add_chunk(header, payload).unwrap();
        }

        let assembled = reassembler.assemble().unwrap();
        assert_eq!(assembled, original_data);
    }

    #[test]
    fn test_chunk_and_reassemble_large_data() {
        // 创建较大的数据
        let original_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let message_id = 456;
        let mtu = 23;

        // 分片
        let chunks = Chunker::chunk(&original_data, message_id, mtu).unwrap();
        assert!(chunks.len() > 10); // 应该有很多分片

        // 重组
        let first_header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        let mut reassembler = Reassembler::new(
            first_header.message_id,
            first_header.total_chunks,
            DEFAULT_REASSEMBLE_TIMEOUT,
        );

        for chunk in &chunks {
            let header = ChunkHeader::from_bytes(chunk).unwrap();
            let payload = chunk[8..].to_vec();
            reassembler.add_chunk(header, payload).unwrap();
        }

        let assembled = reassembler.assemble().unwrap();
        assert_eq!(assembled, original_data);
    }

    #[test]
    fn test_chunk_and_reassemble_out_of_order() {
        let original_data = b"The quick brown fox jumps over the lazy dog";
        let message_id = 789;
        let mtu = 23;

        // 分片
        let chunks = Chunker::chunk(original_data, message_id, mtu).unwrap();

        // 乱序重组
        let first_header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        let mut reassembler = Reassembler::new(
            first_header.message_id,
            first_header.total_chunks,
            DEFAULT_REASSEMBLE_TIMEOUT,
        );

        // 逆序添加分片
        for chunk in chunks.iter().rev() {
            let header = ChunkHeader::from_bytes(chunk).unwrap();
            let payload = chunk[8..].to_vec();
            reassembler.add_chunk(header, payload).unwrap();
        }

        let assembled = reassembler.assemble().unwrap();
        assert_eq!(assembled, original_data);
    }

    #[test]
    fn test_chunk_and_reassemble_with_larger_mtu() {
        let original_data = b"Test data for larger MTU";
        let message_id = 100;
        let mtu = 512; // Large MTU

        // 分片
        let chunks = Chunker::chunk(original_data, message_id, mtu).unwrap();
        assert_eq!(chunks.len(), 1); // Should fit in one chunk

        // 重组
        let header = ChunkHeader::from_bytes(&chunks[0]).unwrap();
        let mut reassembler =
            Reassembler::new(header.message_id, header.total_chunks, DEFAULT_REASSEMBLE_TIMEOUT);

        let payload = chunks[0][8..].to_vec();
        reassembler.add_chunk(header, payload).unwrap();

        let assembled = reassembler.assemble().unwrap();
        assert_eq!(assembled, original_data);
    }
}
