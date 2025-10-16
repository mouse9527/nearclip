//! 同步管理模块
//!
//! 提供剪贴板数据同步、冲突解决等功能

use crate::error::{NearClipError, Result};
use crate::crypto::CryptoService;
use crate::ble::BLEManager;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::SystemTime;

/// 同步数据类型
#[derive(Debug, Clone, PartialEq)]
pub enum SyncDataType {
    Text,
    Image,
    File,
    Url,
}

/// 同步数据
#[derive(Debug, Clone)]
pub struct SyncData {
    pub id: String,
    pub data_type: SyncDataType,
    pub content: Vec<u8>,
    pub timestamp: SystemTime,
    pub source_device: String,
    pub content_hash: String,
}

impl SyncData {
    /// 创建新的同步数据
    pub fn new(
        id: String,
        data_type: SyncDataType,
        content: Vec<u8>,
        source_device: String,
    ) -> Self {
        let content_hash = format!("{:x}", md5::compute(&content));

        SyncData {
            id,
            data_type,
            content,
            timestamp: SystemTime::now(),
            source_device,
            content_hash,
        }
    }

    /// 验证数据完整性
    pub fn verify_integrity(&self) -> bool {
        let current_hash = format!("{:x}", md5::compute(&self.content));
        current_hash == self.content_hash
    }
}

/// 同步管理器
pub struct SyncManager {
    crypto_service: Arc<CryptoService>,
    ble_manager: Arc<BLEManager>,
    sync_history: Arc<RwLock<Vec<SyncData>>>,
    is_running: Arc<RwLock<bool>>,
}

impl SyncManager {
    /// 创建新的同步管理器
    pub fn new(crypto_service: Arc<CryptoService>, ble_manager: Arc<BLEManager>) -> Self {
        SyncManager {
            crypto_service,
            ble_manager,
            sync_history: Arc::new(RwLock::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// 启动同步服务
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Err(NearClipError::SyncError("Sync service already running".to_string()));
        }

        *is_running = true;
        log::info!("Sync manager started");

        // 在实际实现中，这里会启动剪贴板监听和消息处理
        Ok(())
    }

    /// 停止同步服务
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        log::info!("Sync manager stopped");

        // 在实际实现中，这里会停止剪贴板监听和消息处理
        Ok(())
    }

    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// 同步数据到所有已连接设备
    pub async fn sync_to_all_devices(&self, data: &SyncData) -> Result<()> {
        if !self.is_running().await {
            return Err(NearClipError::SyncError("Sync service not running".to_string()));
        }

        // 获取所有已连接的设备
        let connected_devices = self.get_connected_devices().await;

        // 序列化数据
        let serialized_data = self.serialize_sync_data(data)?;

        // 加密数据（在实际实现中）
        let encrypted_data = self.encrypt_sync_data(&serialized_data)?;

        // 发送到每个设备
        for device_id in connected_devices {
            if let Err(e) = self.ble_manager.send_message(&device_id, &encrypted_data).await {
                log::error!("Failed to sync to device {}: {:?}", device_id, e);
            }
        }

        // 添加到历史记录
        let mut history = self.sync_history.write().await;
        history.push(data.clone());

        // 保持历史记录在合理大小
        if history.len() > 1000 {
            history.remove(0);
        }

        Ok(())
    }

    /// 处理接收到的同步数据
    pub async fn handle_received_data(&self, encrypted_data: &[u8]) -> Result<SyncData> {
        // 解密数据
        let decrypted_data = self.decrypt_sync_data(encrypted_data)?;

        // 反序列化数据
        let sync_data = self.deserialize_sync_data(&decrypted_data)?;

        // 验证数据完整性
        if !sync_data.verify_integrity() {
            return Err(NearClipError::SyncError("Data integrity check failed".to_string()));
        }

        // 检查是否是重复数据
        if self.is_duplicate_data(&sync_data).await {
            return Err(NearClipError::SyncError("Duplicate sync data".to_string()));
        }

        // 添加到历史记录
        let mut history = self.sync_history.write().await;
        history.push(sync_data.clone());

        Ok(sync_data)
    }

    /// 获取同步历史
    pub async fn get_sync_history(&self) -> Vec<SyncData> {
        self.sync_history.read().await.clone()
    }

    /// 清除同步历史
    pub async fn clear_history(&self) {
        let mut history = self.sync_history.write().await;
        history.clear();
    }

    // 私有辅助方法

    /// 获取已连接的设备列表
    async fn get_connected_devices(&self) -> Vec<String> {
        // 在实际实现中，这里会从 BLE 管理器获取已连接设备
        // 现在返回空列表作为占位符
        vec![]
    }

    /// 序列化同步数据
    fn serialize_sync_data(&self, data: &SyncData) -> Result<Vec<u8>> {
        // 简单的序列化实现
        // 在实际项目中会使用 Protocol Buffers
        let mut buffer = Vec::new();

        // 数据类型
        buffer.push(data.data_type as u8);

        // 时间戳
        let timestamp = data.timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        buffer.extend_from_slice(&timestamp.to_le_bytes());

        // 源设备
        let source_bytes = data.source_device.as_bytes();
        buffer.push(source_bytes.len() as u8);
        buffer.extend_from_slice(source_bytes);

        // 内容
        buffer.extend_from_slice(&(data.content.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&data.content);

        Ok(buffer)
    }

    /// 反序列化同步数据
    fn deserialize_sync_data(&self, data: &[u8]) -> Result<SyncData> {
        if data.is_empty() {
            return Err(NearClipError::SerializationError("Empty sync data".to_string()));
        }

        let mut offset = 0;

        // 数据类型
        let data_type = match data[offset] {
            0 => SyncDataType::Text,
            1 => SyncDataType::Image,
            2 => SyncDataType::File,
            3 => SyncDataType::Url,
            _ => return Err(NearClipError::SerializationError("Unknown data type".to_string())),
        };
        offset += 1;

        // 时间戳
        if offset + 8 > data.len() {
            return Err(NearClipError::SerializationError("Invalid timestamp".to_string()));
        }
        let timestamp_bytes = &data[offset..offset + 8];
        let timestamp_secs = u64::from_le_bytes([
            timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3],
            timestamp_bytes[4], timestamp_bytes[5], timestamp_bytes[6], timestamp_bytes[7],
        ]);
        let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp_secs);
        offset += 8;

        // 源设备
        if offset >= data.len() {
            return Err(NearClipError::SerializationError("Invalid source device".to_string()));
        }
        let source_len = data[offset] as usize;
        offset += 1;
        if offset + source_len > data.len() {
            return Err(NearClipError::SerializationError("Source device too long".to_string()));
        }
        let source_device = String::from_utf8(data[offset..offset + source_len].to_vec())
            .map_err(|_| NearClipError::SerializationError("Invalid source device encoding".to_string()))?;
        offset += source_len;

        // 内容
        if offset + 4 > data.len() {
            return Err(NearClipError::SerializationError("Invalid content length".to_string()));
        }
        let content_len = u32::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        ]) as usize;
        offset += 4;
        if offset + content_len > data.len() {
            return Err(NearClipError::SerializationError("Content too long".to_string()));
        }
        let content = data[offset..offset + content_len].to_vec();

        let sync_data = SyncData::new(
            uuid::Uuid::new_v4().to_string(),
            data_type,
            content,
            source_device,
        );

        Ok(sync_data)
    }

    /// 加密同步数据
    fn encrypt_sync_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // 生成会话密钥和 Nonce
        let session_key = self.crypto_service.generate_session_key()?;
        let nonce = self.crypto_service.generate_nonce()?;

        // 加密数据
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&session_key);
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&self.crypto_service.encrypt(data, &session_key, &nonce)?);

        Ok(encrypted_data)
    }

    /// 解密同步数据
    fn decrypt_sync_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        if encrypted_data.len() < 44 { // 32 bytes key + 12 bytes nonce + at least 1 byte data
            return Err(NearClipError::CryptoError("Invalid encrypted data format".to_string()));
        }

        let session_key = &encrypted_data[..32];
        let nonce = &encrypted_data[32..44];
        let ciphertext = &encrypted_data[44..];

        self.crypto_service.decrypt(ciphertext, session_key, nonce)
    }

    /// 检查是否是重复数据
    async fn is_duplicate_data(&self, data: &SyncData) -> bool {
        let history = self.sync_history.read().await;
        history.iter().any(|existing| {
            existing.content_hash == data.content_hash &&
            existing.source_device == data.source_device
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_data_creation() {
        let data = SyncData::new(
            "test123".to_string(),
            SyncDataType::Text,
            b"Hello, World!".to_vec(),
            "device1".to_string(),
        );

        assert_eq!(data.id, "test123");
        assert_eq!(data.data_type, SyncDataType::Text);
        assert_eq!(data.source_device, "device1");
        assert!(data.verify_integrity());
    }

    #[tokio::test]
    async fn test_sync_manager_start_stop() {
        let crypto_service = Arc::new(CryptoService::new().unwrap());
        let ble_manager = Arc::new(BLEManager::new("test-uuid".to_string()));

        let sync_manager = SyncManager::new(crypto_service, ble_manager);

        // 启动服务
        assert!(sync_manager.start().await.is_ok());
        assert!(sync_manager.is_running().await);

        // 停止服务
        assert!(sync_manager.stop().await.is_ok());
        assert!(!sync_manager.is_running().await);
    }

    #[tokio::test]
    async fn test_sync_data_serialization() {
        let crypto_service = Arc::new(CryptoService::new().unwrap());
        let ble_manager = Arc::new(BLEManager::new("test-uuid".to_string()));

        let sync_manager = SyncManager::new(crypto_service, ble_manager);

        let original_data = SyncData::new(
            "test123".to_string(),
            SyncDataType::Text,
            b"Hello, World!".to_vec(),
            "device1".to_string(),
        );

        let serialized = sync_manager.serialize_sync_data(&original_data).unwrap();
        let deserialized = sync_manager.deserialize_sync_data(&serialized).unwrap();

        assert_eq!(original_data.data_type, deserialized.data_type);
        assert_eq!(original_data.source_device, deserialized.source_device);
        assert_eq!(original_data.content, deserialized.content);
    }
}