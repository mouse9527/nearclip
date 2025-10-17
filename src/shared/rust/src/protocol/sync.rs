include!(concat!(env!("OUT_DIR"), "/nearclip.sync.rs"));

use crate::protocol::ProtocolError;

impl ClipboardData {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.data_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("数据ID不能为空".to_string()));
        }
        if self.content.is_empty() {
            return Err(ProtocolError::InvalidFormat("数据内容不能为空".to_string()));
        }
        if self.created_at == 0 {
            return Err(ProtocolError::InvalidFormat("创建时间无效".to_string()));
        }
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false; // 永不过期
        }
        chrono::Utc::now().timestamp_millis() as u64 > self.expires_at
    }

    pub fn get_size(&self) -> usize {
        self.content.len()
    }
}

impl DataChunk {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.data_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("数据ID不能为空".to_string()));
        }
        if self.chunk_index >= self.total_chunks {
            return Err(ProtocolError::InvalidFormat("分片索引无效".to_string()));
        }
        if self.chunk_data.is_empty() {
            return Err(ProtocolError::InvalidFormat("分片数据不能为空".to_string()));
        }
        if self.checksum.is_empty() {
            return Err(ProtocolError::InvalidFormat("校验和不能为空".to_string()));
        }
        Ok(())
    }

    pub fn verify_checksum(&self) -> bool {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&self.chunk_data);
        let computed_hash = hasher.finalize();
        computed_hash.as_slice() == self.checksum
    }
}

impl SyncMessage {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.device_id.is_empty() {
            return Err(ProtocolError::InvalidFormat("设备ID不能为空".to_string()));
        }
        if let Some(ref data) = self.data {
            data.validate()?;
        }

        // 验证所有分片
        for chunk in &self.chunks {
            chunk.validate()?;
        }

        Ok(())
    }

    pub fn total_size(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.chunk_data.len()).sum()
    }

    pub fn requires_chunking(&self) -> bool {
        !self.chunks.is_empty()
    }
}

impl SyncAck {
    pub fn success(data_id: String) -> Self {
        Self {
            data_id,
            success: true,
            error_message: String::new(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn failure(data_id: String, error: String) -> Self {
        Self {
            data_id,
            success: false,
            error_message: error,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}