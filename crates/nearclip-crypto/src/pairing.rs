//! 配对数据和二维码生成
//!
//! 提供设备配对所需的数据结构和二维码生成功能。
//!
//! # Example
//!
//! ```
//! use nearclip_crypto::{EcdhKeyPair, PairingData, QrCodeGenerator};
//!
//! // 生成密钥对
//! let keypair = EcdhKeyPair::generate();
//!
//! // 创建配对数据
//! let pairing_data = PairingData::new(
//!     "my-device-id".to_string(),
//!     &keypair.public_key_bytes(),
//! );
//!
//! // 生成二维码
//! let generator = QrCodeGenerator::new();
//! let png_data = generator.generate_png(&pairing_data).unwrap();
//! assert!(!png_data.is_empty());
//! ```

use crate::{CryptoError, EcdhKeyPair};
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageBuffer, Luma};
use qrcode::{EcLevel, QrCode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, instrument, warn};
use zeroize::Zeroize;

/// 配对数据版本
pub const PAIRING_DATA_VERSION: u8 = 1;

/// 配对数据结构
///
/// 包含设备配对所需的所有信息，用于生成二维码。
///
/// # Example
///
/// ```
/// use nearclip_crypto::PairingData;
///
/// let pairing_data = PairingData::new(
///     "my-device".to_string(),
///     &[0x04; 65], // 示例公钥
/// );
/// assert_eq!(pairing_data.device_id, "my-device");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairingData {
    /// 数据格式版本（用于向后兼容）
    pub version: u8,
    /// 设备唯一标识符
    pub device_id: String,
    /// 公钥（Base64 编码）
    pub public_key: String,
    /// 连接信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<ConnectionInfo>,
}

/// 连接信息
///
/// 包含设备的网络连接信息，用于建立直接连接。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionInfo {
    /// IP 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 端口号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// mDNS 服务名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns_name: Option<String>,
}

impl ConnectionInfo {
    /// 创建新的连接信息
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::ConnectionInfo;
    ///
    /// let info = ConnectionInfo::new()
    ///     .with_ip("192.168.1.100")
    ///     .with_port(8765)
    ///     .with_mdns_name("macbook._nearclip._tcp.local");
    /// ```
    pub fn new() -> Self {
        Self {
            ip: None,
            port: None,
            mdns_name: None,
        }
    }

    /// 设置 IP 地址
    pub fn with_ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_string());
        self
    }

    /// 设置端口号
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// 设置 mDNS 服务名
    pub fn with_mdns_name(mut self, name: &str) -> Self {
        self.mdns_name = Some(name.to_string());
        self
    }

    /// 检查是否为空（没有任何连接信息）
    pub fn is_empty(&self) -> bool {
        self.ip.is_none() && self.port.is_none() && self.mdns_name.is_none()
    }
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PairingData {
    /// 创建新的配对数据
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备唯一标识符
    /// * `public_key_bytes` - 公钥字节（33 或 65 字节）
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::{EcdhKeyPair, PairingData};
    ///
    /// let keypair = EcdhKeyPair::generate();
    /// let pairing_data = PairingData::new(
    ///     "my-device".to_string(),
    ///     &keypair.public_key_bytes(),
    /// );
    /// ```
    #[instrument(skip(public_key_bytes), fields(device_id = %device_id, key_len = public_key_bytes.len()))]
    pub fn new(device_id: String, public_key_bytes: &[u8]) -> Self {
        let public_key = STANDARD.encode(public_key_bytes);
        debug!("Created pairing data for device");
        Self {
            version: PAIRING_DATA_VERSION,
            device_id,
            public_key,
            connection_info: None,
        }
    }

    /// 带连接信息的配对数据
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::{PairingData, ConnectionInfo};
    ///
    /// let info = ConnectionInfo::new()
    ///     .with_ip("192.168.1.100")
    ///     .with_port(8765);
    ///
    /// let pairing_data = PairingData::new("device".to_string(), &[0x04; 65])
    ///     .with_connection_info(info);
    /// ```
    pub fn with_connection_info(mut self, info: ConnectionInfo) -> Self {
        self.connection_info = Some(info);
        self
    }

    /// 转换为 JSON 字符串
    ///
    /// # Returns
    ///
    /// JSON 格式的配对数据字符串
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::JsonSerialization` 如果序列化失败
    #[instrument(skip(self))]
    pub fn to_json(&self) -> Result<String, CryptoError> {
        serde_json::to_string(self)
            .map_err(|e| CryptoError::JsonSerialization(e.to_string()))
    }

    /// 从 JSON 字符串解析
    ///
    /// # Arguments
    ///
    /// * `json` - JSON 格式的配对数据字符串
    ///
    /// # Returns
    ///
    /// 解析后的 PairingData
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::JsonSerialization` 如果解析失败
    #[instrument(skip(json), fields(json_len = json.len()))]
    pub fn from_json(json: &str) -> Result<Self, CryptoError> {
        let data: Self = serde_json::from_str(json)
            .map_err(|e| CryptoError::JsonSerialization(e.to_string()))?;
        debug!("Parsed pairing data from JSON");
        Ok(data)
    }

    /// 验证配对数据
    ///
    /// 检查所有必需字段是否有效：
    /// - device_id 非空
    /// - public_key 是有效的 Base64 且长度正确（33 或 65 字节解码后）
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::InvalidPairingData` 如果验证失败
    #[instrument(skip(self))]
    pub fn validate(&self) -> Result<(), CryptoError> {
        // 验证设备 ID
        if self.device_id.is_empty() {
            warn!("Device ID is empty");
            return Err(CryptoError::InvalidPairingData(
                "Device ID cannot be empty".to_string(),
            ));
        }

        // 验证公钥 Base64
        let decoded = STANDARD
            .decode(&self.public_key)
            .map_err(|e| {
                warn!("Invalid Base64 public key: {}", e);
                CryptoError::InvalidPairingData(format!("Invalid Base64 public key: {}", e))
            })?;

        // 验证公钥长度（33 字节压缩或 65 字节未压缩）
        if decoded.len() != 33 && decoded.len() != 65 {
            warn!("Invalid public key length: {}", decoded.len());
            return Err(CryptoError::InvalidPairingData(format!(
                "Invalid public key length: {} (expected 33 or 65)",
                decoded.len()
            )));
        }

        debug!("Pairing data validation passed");
        Ok(())
    }

    /// 获取解码后的公钥字节
    ///
    /// # Returns
    ///
    /// 公钥字节
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::InvalidPairingData` 如果 Base64 解码失败
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        STANDARD
            .decode(&self.public_key)
            .map_err(|e| CryptoError::InvalidPairingData(format!("Invalid Base64: {}", e)))
    }
}

/// 二维码纠错级别
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum QrCodeErrorCorrection {
    /// ~7% 恢复能力
    #[default]
    Low,
    /// ~15% 恢复能力
    Medium,
    /// ~25% 恢复能力
    Quartile,
    /// ~30% 恢复能力
    High,
}

impl QrCodeErrorCorrection {
    fn to_ec_level(self) -> EcLevel {
        match self {
            QrCodeErrorCorrection::Low => EcLevel::L,
            QrCodeErrorCorrection::Medium => EcLevel::M,
            QrCodeErrorCorrection::Quartile => EcLevel::Q,
            QrCodeErrorCorrection::High => EcLevel::H,
        }
    }
}

/// 二维码生成配置
#[derive(Debug, Clone)]
pub struct QrCodeConfig {
    /// 二维码模块大小（像素）
    pub module_size: u32,
    /// 边距（模块数）
    pub margin: u32,
    /// 纠错级别
    pub error_correction: QrCodeErrorCorrection,
}

impl Default for QrCodeConfig {
    fn default() -> Self {
        Self {
            module_size: 10,
            margin: 4,
            error_correction: QrCodeErrorCorrection::Low,
        }
    }
}

impl QrCodeConfig {
    /// 创建新配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置模块大小
    pub fn with_module_size(mut self, size: u32) -> Self {
        self.module_size = size;
        self
    }

    /// 设置边距
    pub fn with_margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    /// 设置纠错级别
    pub fn with_error_correction(mut self, level: QrCodeErrorCorrection) -> Self {
        self.error_correction = level;
        self
    }
}

/// 二维码生成器
///
/// # Example
///
/// ```
/// use nearclip_crypto::{PairingData, QrCodeGenerator, QrCodeConfig, QrCodeErrorCorrection};
///
/// let generator = QrCodeGenerator::with_config(
///     QrCodeConfig::new()
///         .with_module_size(8)
///         .with_error_correction(QrCodeErrorCorrection::Medium)
/// );
///
/// let pairing_data = PairingData::new("device".to_string(), &[0x04; 65]);
/// let png_data = generator.generate_png(&pairing_data).unwrap();
/// ```
pub struct QrCodeGenerator {
    config: QrCodeConfig,
}

impl QrCodeGenerator {
    /// 创建默认配置的生成器
    pub fn new() -> Self {
        Self {
            config: QrCodeConfig::default(),
        }
    }

    /// 使用自定义配置
    pub fn with_config(config: QrCodeConfig) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &QrCodeConfig {
        &self.config
    }

    /// 生成二维码 PNG 图片
    ///
    /// # Arguments
    ///
    /// * `data` - 配对数据
    ///
    /// # Returns
    ///
    /// PNG 格式的二维码图片字节
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::QrCodeGeneration` 如果生成失败
    #[instrument(skip(self, data), fields(device_id = %data.device_id))]
    pub fn generate_png(&self, data: &PairingData) -> Result<Vec<u8>, CryptoError> {
        // 序列化为 JSON
        let json = data.to_json()?;
        debug!("Generating QR code for {} bytes of JSON", json.len());

        // 生成二维码
        let code = QrCode::with_error_correction_level(&json, self.config.error_correction.to_ec_level())
            .map_err(|e| {
                warn!("Failed to create QR code: {}", e);
                CryptoError::QrCodeGeneration(e.to_string())
            })?;

        // 计算图片尺寸
        let qr_width = code.width();
        let total_modules = qr_width + (self.config.margin * 2) as usize;
        let img_size = (total_modules as u32) * self.config.module_size;

        // 创建图片缓冲区
        let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(img_size, img_size);

        // 填充白色背景
        for pixel in img.pixels_mut() {
            *pixel = Luma([255u8]);
        }

        // 绘制二维码模块
        let margin_px = self.config.margin * self.config.module_size;
        for (y, row) in code.to_colors().chunks(qr_width).enumerate() {
            for (x, color) in row.iter().enumerate() {
                let is_dark = matches!(color, qrcode::Color::Dark);
                if is_dark {
                    let px = margin_px + (x as u32) * self.config.module_size;
                    let py = margin_px + (y as u32) * self.config.module_size;

                    // 填充模块
                    for dy in 0..self.config.module_size {
                        for dx in 0..self.config.module_size {
                            if px + dx < img_size && py + dy < img_size {
                                img.put_pixel(px + dx, py + dy, Luma([0u8]));
                            }
                        }
                    }
                }
            }
        }

        // 编码为 PNG
        let mut png_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| {
                warn!("Failed to encode PNG: {}", e);
                CryptoError::QrCodeGeneration(format!("PNG encoding failed: {}", e))
            })?;

        debug!(
            "Generated QR code PNG: {}x{} pixels, {} bytes",
            img_size,
            img_size,
            png_data.len()
        );

        Ok(png_data)
    }

    /// 生成二维码 PNG 并保存到文件（仅用于测试/调试）
    #[cfg(test)]
    pub fn generate_to_file(
        &self,
        data: &PairingData,
        path: &std::path::Path,
    ) -> Result<(), CryptoError> {
        let png_data = self.generate_png(data)?;
        std::fs::write(path, png_data)
            .map_err(|e| CryptoError::QrCodeGeneration(format!("Failed to write file: {}", e)))
    }
}

impl Default for QrCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// 配对会话
///
/// 管理配对过程中的状态，包括本地密钥对、对方设备信息和共享密钥。
///
/// # Example
///
/// ```
/// use nearclip_crypto::{EcdhKeyPair, PairingData, PairingSession};
///
/// // 设备 B 创建配对会话
/// let keypair_b = EcdhKeyPair::generate();
/// let mut session = PairingSession::new(keypair_b);
///
/// // 设备 A 的配对数据（通常从二维码获取）
/// let keypair_a = EcdhKeyPair::generate();
/// let peer_data = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes());
///
/// // 处理对方数据，完成密钥协商
/// session.process_peer_data(&peer_data).unwrap();
///
/// // 共享密钥已计算
/// assert!(session.shared_secret().is_some());
/// ```
pub struct PairingSession {
    /// 本地密钥对
    local_keypair: EcdhKeyPair,
    /// 对方设备 ID
    peer_device_id: Option<String>,
    /// 对方公钥字节
    peer_public_key: Option<Vec<u8>>,
    /// 对方连接信息
    peer_connection_info: Option<ConnectionInfo>,
    /// 计算出的共享密钥
    shared_secret: Option<Vec<u8>>,
}

impl PairingSession {
    /// 创建新的配对会话
    ///
    /// # Arguments
    ///
    /// * `local_keypair` - 本地设备的 ECDH 密钥对
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::{EcdhKeyPair, PairingSession};
    ///
    /// let keypair = EcdhKeyPair::generate();
    /// let session = PairingSession::new(keypair);
    /// ```
    #[instrument(skip(local_keypair))]
    pub fn new(local_keypair: EcdhKeyPair) -> Self {
        debug!("Created new pairing session");
        Self {
            local_keypair,
            peer_device_id: None,
            peer_public_key: None,
            peer_connection_info: None,
            shared_secret: None,
        }
    }

    /// 处理对方设备的配对数据
    ///
    /// 验证数据、提取公钥、计算共享密钥。
    ///
    /// # Arguments
    ///
    /// * `peer_data` - 对方设备的配对数据（通常从二维码解析）
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::PairingFailed` 如果：
    /// - 配对数据验证失败
    /// - 公钥解码失败
    /// - 共享密钥计算失败
    #[instrument(skip(self, peer_data), fields(peer_device_id = %peer_data.device_id))]
    pub fn process_peer_data(&mut self, peer_data: &PairingData) -> Result<(), CryptoError> {
        // 验证配对数据
        peer_data.validate()?;

        // 提取公钥
        let public_key_bytes = peer_data.public_key_bytes()?;
        debug!("Extracted peer public key: {} bytes", public_key_bytes.len());

        // 计算共享密钥
        let shared_secret = self
            .local_keypair
            .compute_shared_secret(&public_key_bytes)
            .map_err(|e| CryptoError::PairingFailed(format!("ECDH failed: {}", e)))?;

        info!(
            "Computed shared secret with device: {}",
            peer_data.device_id
        );

        // 存储对方信息
        self.peer_device_id = Some(peer_data.device_id.clone());
        self.peer_public_key = Some(public_key_bytes);
        self.peer_connection_info = peer_data.connection_info.clone();
        self.shared_secret = Some(shared_secret);

        Ok(())
    }

    /// 获取共享密钥
    ///
    /// 只有在调用 `process_peer_data` 成功后才会返回值。
    ///
    /// # Returns
    ///
    /// 32 字节的共享密钥，如果尚未计算则返回 `None`
    pub fn shared_secret(&self) -> Option<&[u8]> {
        self.shared_secret.as_deref()
    }

    /// 获取对方设备 ID
    pub fn peer_device_id(&self) -> Option<&str> {
        self.peer_device_id.as_deref()
    }

    /// 获取对方公钥
    pub fn peer_public_key(&self) -> Option<&[u8]> {
        self.peer_public_key.as_deref()
    }

    /// 获取对方连接信息
    pub fn peer_connection_info(&self) -> Option<&ConnectionInfo> {
        self.peer_connection_info.as_ref()
    }

    /// 获取本地密钥对的引用
    pub fn local_keypair(&self) -> &EcdhKeyPair {
        &self.local_keypair
    }

    /// 完成配对，返回已配对设备信息
    ///
    /// 消费会话数据并创建 `PairedDevice` 实例。
    /// 调用后共享密钥会被清零。
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::PairingFailed` 如果：
    /// - 配对会话未完成（未调用 `process_peer_data`）
    #[instrument(skip(self))]
    pub fn complete(&mut self) -> Result<PairedDevice, CryptoError> {
        let device_id = self
            .peer_device_id
            .take()
            .ok_or_else(|| CryptoError::PairingFailed("No peer device ID".to_string()))?;

        let public_key_bytes = self
            .peer_public_key
            .take()
            .ok_or_else(|| CryptoError::PairingFailed("No peer public key".to_string()))?;

        let shared_secret = self
            .shared_secret
            .take()
            .ok_or_else(|| CryptoError::PairingFailed("No shared secret computed".to_string()))?;

        // 计算共享密钥的哈希（不存储完整密钥）
        let shared_secret_hash = Self::hash_shared_secret(&shared_secret);

        // 清零共享密钥
        let mut secret_to_clear = shared_secret;
        secret_to_clear.zeroize();

        // 获取当前时间戳
        let paired_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        info!("Pairing completed with device: {}", device_id);

        Ok(PairedDevice {
            device_id,
            public_key_bytes,
            connection_info: self.peer_connection_info.take(),
            shared_secret_hash,
            paired_at,
        })
    }

    /// 计算共享密钥的 SHA256 哈希
    fn hash_shared_secret(secret: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let result = hasher.finalize();
        STANDARD.encode(result)
    }
}

impl std::fmt::Debug for PairingSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PairingSession")
            .field("peer_device_id", &self.peer_device_id)
            .field("has_shared_secret", &self.shared_secret.is_some())
            .finish_non_exhaustive()
    }
}

impl Drop for PairingSession {
    fn drop(&mut self) {
        // 清零共享密钥以防止内存泄露
        if let Some(ref mut secret) = self.shared_secret {
            secret.zeroize();
        }
        // 清零对方公钥
        if let Some(ref mut key) = self.peer_public_key {
            key.zeroize();
        }
    }
}

/// 已配对设备信息
///
/// 存储配对成功后的设备信息，用于后续连接和通信。
/// 此结构体可序列化，用于持久化存储。
///
/// # Security
///
/// 不存储完整的共享密钥，只存储其 SHA256 哈希用于验证。
/// 实际加密通信时应重新计算共享密钥。
///
/// # Example
///
/// ```
/// use nearclip_crypto::{EcdhKeyPair, PairingData, PairingSession};
///
/// let keypair = EcdhKeyPair::generate();
/// let mut session = PairingSession::new(keypair);
///
/// let peer_keypair = EcdhKeyPair::generate();
/// let peer_data = PairingData::new("peer-device".to_string(), &peer_keypair.public_key_bytes());
///
/// session.process_peer_data(&peer_data).unwrap();
/// let paired_device = session.complete().unwrap();
///
/// assert_eq!(paired_device.device_id, "peer-device");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PairedDevice {
    /// 设备 ID
    pub device_id: String,
    /// 对方公钥（字节）
    pub public_key_bytes: Vec<u8>,
    /// 连接信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<ConnectionInfo>,
    /// 共享密钥的哈希（Base64 编码的 SHA256）
    pub shared_secret_hash: String,
    /// 配对时间（Unix 时间戳，秒）
    pub paired_at: u64,
}

impl PairedDevice {
    /// 创建新的已配对设备信息
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备 ID
    /// * `public_key_bytes` - 对方公钥字节
    /// * `shared_secret` - 共享密钥（将计算哈希）
    /// * `connection_info` - 可选的连接信息
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::PairedDevice;
    ///
    /// let device = PairedDevice::new(
    ///     "device-123".to_string(),
    ///     vec![0x04; 65],
    ///     &[0xAB; 32],
    ///     None,
    /// );
    /// assert_eq!(device.device_id, "device-123");
    /// ```
    pub fn new(
        device_id: String,
        public_key_bytes: Vec<u8>,
        shared_secret: &[u8],
        connection_info: Option<ConnectionInfo>,
    ) -> Self {
        let shared_secret_hash = Self::compute_hash(shared_secret);
        let paired_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            device_id,
            public_key_bytes,
            connection_info,
            shared_secret_hash,
            paired_at,
        }
    }

    /// 验证共享密钥是否匹配
    ///
    /// 用于验证重新计算的共享密钥与存储的哈希是否匹配。
    ///
    /// # Arguments
    ///
    /// * `shared_secret` - 要验证的共享密钥
    ///
    /// # Returns
    ///
    /// `true` 如果哈希匹配
    pub fn verify_shared_secret(&self, shared_secret: &[u8]) -> bool {
        let hash = Self::compute_hash(shared_secret);
        self.shared_secret_hash == hash
    }

    /// 转换为 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::JsonSerialization` 如果序列化失败
    pub fn to_json(&self) -> Result<String, CryptoError> {
        serde_json::to_string(self).map_err(|e| CryptoError::JsonSerialization(e.to_string()))
    }

    /// 从 JSON 字符串解析
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::JsonSerialization` 如果解析失败
    pub fn from_json(json: &str) -> Result<Self, CryptoError> {
        serde_json::from_str(json).map_err(|e| CryptoError::JsonSerialization(e.to_string()))
    }

    /// 计算共享密钥的 SHA256 哈希
    fn compute_hash(secret: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let result = hasher.finalize();
        STANDARD.encode(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EcdhKeyPair;

    #[test]
    fn test_pairing_data_new() {
        let keypair = EcdhKeyPair::generate();
        let data = PairingData::new("test-device".to_string(), &keypair.public_key_bytes());

        assert_eq!(data.device_id, "test-device");
        assert_eq!(data.version, PAIRING_DATA_VERSION);
        assert!(data.connection_info.is_none());
    }

    #[test]
    fn test_pairing_data_with_connection_info() {
        let data = PairingData::new("device".to_string(), &[0x04; 65])
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("192.168.1.100")
                    .with_port(8765),
            );

        let info = data.connection_info.as_ref().unwrap();
        assert_eq!(info.ip, Some("192.168.1.100".to_string()));
        assert_eq!(info.port, Some(8765));
    }

    #[test]
    fn test_pairing_data_json_roundtrip() {
        let keypair = EcdhKeyPair::generate();
        let original = PairingData::new("test-device".to_string(), &keypair.public_key_bytes())
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("192.168.1.100")
                    .with_port(8765)
                    .with_mdns_name("test._nearclip._tcp.local"),
            );

        let json = original.to_json().unwrap();
        let parsed = PairingData::from_json(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_pairing_data_json_format() {
        let data = PairingData::new("device".to_string(), &[0x04; 65]);
        let json = data.to_json().unwrap();

        // 验证 JSON 包含必需字段
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"device_id\""));
        assert!(json.contains("\"public_key\""));
    }

    #[test]
    fn test_pairing_data_json_skip_empty_connection_info() {
        let data = PairingData::new("device".to_string(), &[0x04; 65]);
        let json = data.to_json().unwrap();

        // connection_info 为 None 时不应出现在 JSON 中
        assert!(!json.contains("connection_info"));
    }

    #[test]
    fn test_pairing_data_validate_success() {
        let keypair = EcdhKeyPair::generate();
        let data = PairingData::new("valid-device".to_string(), &keypair.public_key_bytes());

        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_pairing_data_validate_empty_device_id() {
        let data = PairingData::new("".to_string(), &[0x04; 65]);

        let result = data.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidPairingData(_))));
    }

    #[test]
    fn test_pairing_data_validate_invalid_base64() {
        let mut data = PairingData::new("device".to_string(), &[0x04; 65]);
        data.public_key = "not-valid-base64!!!".to_string();

        let result = data.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidPairingData(_))));
    }

    #[test]
    fn test_pairing_data_validate_wrong_key_length() {
        let data = PairingData::new("device".to_string(), &[0x04; 10]); // 错误长度

        let result = data.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidPairingData(_))));
    }

    #[test]
    fn test_pairing_data_validate_compressed_key() {
        let keypair = EcdhKeyPair::generate();
        let data = PairingData::new(
            "device".to_string(),
            &keypair.public_key_bytes_compressed(), // 33 字节压缩格式
        );

        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_pairing_data_public_key_bytes() {
        let keypair = EcdhKeyPair::generate();
        let original_bytes = keypair.public_key_bytes();
        let data = PairingData::new("device".to_string(), &original_bytes);

        let decoded = data.public_key_bytes().unwrap();
        assert_eq!(original_bytes, decoded);
    }

    #[test]
    fn test_connection_info_builder() {
        let info = ConnectionInfo::new()
            .with_ip("10.0.0.1")
            .with_port(1234)
            .with_mdns_name("device._nearclip._tcp.local");

        assert_eq!(info.ip, Some("10.0.0.1".to_string()));
        assert_eq!(info.port, Some(1234));
        assert_eq!(info.mdns_name, Some("device._nearclip._tcp.local".to_string()));
    }

    #[test]
    fn test_connection_info_is_empty() {
        let empty = ConnectionInfo::new();
        assert!(empty.is_empty());

        let with_ip = ConnectionInfo::new().with_ip("127.0.0.1");
        assert!(!with_ip.is_empty());
    }

    #[test]
    fn test_qr_code_generator_default() {
        let gen = QrCodeGenerator::new();
        assert_eq!(gen.config().module_size, 10);
        assert_eq!(gen.config().margin, 4);
    }

    #[test]
    fn test_qr_code_generator_custom_config() {
        let config = QrCodeConfig::new()
            .with_module_size(5)
            .with_margin(2)
            .with_error_correction(QrCodeErrorCorrection::High);

        let gen = QrCodeGenerator::with_config(config);
        assert_eq!(gen.config().module_size, 5);
        assert_eq!(gen.config().margin, 2);
        assert_eq!(gen.config().error_correction, QrCodeErrorCorrection::High);
    }

    #[test]
    fn test_qr_code_generate_png() {
        let keypair = EcdhKeyPair::generate();
        let data = PairingData::new("test-device".to_string(), &keypair.public_key_bytes());

        let gen = QrCodeGenerator::new();
        let png_data = gen.generate_png(&data).unwrap();

        // 验证是有效的 PNG
        assert!(!png_data.is_empty());
        // PNG 文件头: 89 50 4E 47 0D 0A 1A 0A
        assert_eq!(&png_data[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn test_qr_code_generate_png_with_connection_info() {
        let data = PairingData::new("device".to_string(), &[0x04; 65])
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("192.168.1.100")
                    .with_port(8765)
                    .with_mdns_name("device._nearclip._tcp.local"),
            );

        let gen = QrCodeGenerator::new();
        let png_data = gen.generate_png(&data).unwrap();

        assert!(!png_data.is_empty());
    }

    #[test]
    fn test_qr_code_error_correction_levels() {
        let data = PairingData::new("device".to_string(), &[0x04; 65]);

        for level in [
            QrCodeErrorCorrection::Low,
            QrCodeErrorCorrection::Medium,
            QrCodeErrorCorrection::Quartile,
            QrCodeErrorCorrection::High,
        ] {
            let config = QrCodeConfig::new().with_error_correction(level);
            let gen = QrCodeGenerator::with_config(config);
            let result = gen.generate_png(&data);
            assert!(result.is_ok(), "Failed with {:?}", level);
        }
    }

    #[test]
    fn test_qr_code_different_sizes() {
        let data = PairingData::new("device".to_string(), &[0x04; 65]);

        for size in [4, 8, 12, 16] {
            let config = QrCodeConfig::new().with_module_size(size);
            let gen = QrCodeGenerator::with_config(config);
            let result = gen.generate_png(&data);
            assert!(result.is_ok(), "Failed with module_size={}", size);
        }
    }

    #[test]
    fn test_pairing_data_version_in_json() {
        let data = PairingData::new("device".to_string(), &[0x04; 65]);
        let json = data.to_json().unwrap();

        // 确保版本字段存在
        assert!(json.contains(&format!("\"version\":{}", PAIRING_DATA_VERSION)));
    }

    // ==================== PairingSession Tests ====================

    #[test]
    fn test_pairing_session_new() {
        let keypair = EcdhKeyPair::generate();
        let session = PairingSession::new(keypair);

        assert!(session.peer_device_id().is_none());
        assert!(session.peer_public_key().is_none());
        assert!(session.shared_secret().is_none());
    }

    #[test]
    fn test_pairing_session_process_peer_data() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new("peer-device".to_string(), &peer_keypair.public_key_bytes());

        let result = session.process_peer_data(&peer_data);
        assert!(result.is_ok());

        assert_eq!(session.peer_device_id(), Some("peer-device"));
        assert!(session.peer_public_key().is_some());
        assert!(session.shared_secret().is_some());
        assert_eq!(session.shared_secret().unwrap().len(), 32);
    }

    #[test]
    fn test_pairing_session_shared_secret_consistency() {
        // 设备 A
        let keypair_a = EcdhKeyPair::generate();
        let data_a = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes());

        // 设备 B
        let keypair_b = EcdhKeyPair::generate();
        let data_b = PairingData::new("device-b".to_string(), &keypair_b.public_key_bytes());

        // A 扫描 B 的二维码
        let mut session_a = PairingSession::new(keypair_a);
        session_a.process_peer_data(&data_b).unwrap();

        // B 扫描 A 的二维码
        let mut session_b = PairingSession::new(keypair_b);
        session_b.process_peer_data(&data_a).unwrap();

        // 双方应该得到相同的共享密钥
        assert_eq!(session_a.shared_secret(), session_b.shared_secret());
    }

    #[test]
    fn test_pairing_session_with_connection_info() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new("peer".to_string(), &peer_keypair.public_key_bytes())
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("192.168.1.100")
                    .with_port(8765),
            );

        session.process_peer_data(&peer_data).unwrap();

        let info = session.peer_connection_info().unwrap();
        assert_eq!(info.ip, Some("192.168.1.100".to_string()));
        assert_eq!(info.port, Some(8765));
    }

    #[test]
    fn test_pairing_session_process_invalid_data() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        // 空设备 ID
        let invalid_data = PairingData::new("".to_string(), &[0x04; 65]);
        let result = session.process_peer_data(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_pairing_session_complete() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new("peer-device".to_string(), &peer_keypair.public_key_bytes())
            .with_connection_info(ConnectionInfo::new().with_port(9000));

        session.process_peer_data(&peer_data).unwrap();

        let paired_device = session.complete().unwrap();
        assert_eq!(paired_device.device_id, "peer-device");
        assert!(!paired_device.shared_secret_hash.is_empty());
        assert!(paired_device.paired_at > 0);
    }

    #[test]
    fn test_pairing_session_complete_without_processing() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        // 未调用 process_peer_data 就尝试完成
        let result = session.complete();
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::PairingFailed(_))));
    }

    #[test]
    fn test_pairing_session_debug_does_not_leak_secrets() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new("peer".to_string(), &peer_keypair.public_key_bytes());
        session.process_peer_data(&peer_data).unwrap();

        let shared_secret = session.shared_secret().unwrap();
        let debug_str = format!("{:?}", session);

        // Debug 输出应显示是否有共享密钥（而非实际密钥值）
        assert!(debug_str.contains("has_shared_secret"));
        assert!(debug_str.contains("true")); // has_shared_secret: true

        // 不应包含实际的共享密钥字节（32 字节的十六进制或数字）
        // 共享密钥是 32 字节，转为字符串会很长
        let _secret_hex = format!("{:02x}", shared_secret[0]);
        // Debug 输出使用 finish_non_exhaustive，不会展开完整字段
        assert!(!debug_str.contains(&format!("[{}", shared_secret[0])));
    }

    // ==================== PairedDevice Tests ====================

    #[test]
    fn test_paired_device_new() {
        let device = PairedDevice::new(
            "test-device".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            None,
        );

        assert_eq!(device.device_id, "test-device");
        assert_eq!(device.public_key_bytes.len(), 65);
        assert!(!device.shared_secret_hash.is_empty());
        assert!(device.paired_at > 0);
        assert!(device.connection_info.is_none());
    }

    #[test]
    fn test_paired_device_with_connection_info() {
        let info = ConnectionInfo::new()
            .with_ip("10.0.0.1")
            .with_port(1234);

        let device = PairedDevice::new(
            "device".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            Some(info),
        );

        assert!(device.connection_info.is_some());
        let conn = device.connection_info.as_ref().unwrap();
        assert_eq!(conn.ip, Some("10.0.0.1".to_string()));
        assert_eq!(conn.port, Some(1234));
    }

    #[test]
    fn test_paired_device_verify_shared_secret() {
        let secret = [0xAB; 32];
        let device = PairedDevice::new(
            "device".to_string(),
            vec![0x04; 65],
            &secret,
            None,
        );

        // 正确的密钥应该验证通过
        assert!(device.verify_shared_secret(&secret));

        // 错误的密钥应该验证失败
        let wrong_secret = [0xCD; 32];
        assert!(!device.verify_shared_secret(&wrong_secret));
    }

    #[test]
    fn test_paired_device_json_roundtrip() {
        let original = PairedDevice::new(
            "my-device".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            Some(ConnectionInfo::new().with_ip("192.168.1.1").with_port(8080)),
        );

        let json = original.to_json().unwrap();
        let parsed = PairedDevice::from_json(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_paired_device_serialization_format() {
        let device = PairedDevice::new(
            "device".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            None,
        );

        let json = device.to_json().unwrap();

        // 验证必需字段
        assert!(json.contains("\"device_id\""));
        assert!(json.contains("\"public_key_bytes\""));
        assert!(json.contains("\"shared_secret_hash\""));
        assert!(json.contains("\"paired_at\""));
        // connection_info 为 None 时不应出现
        assert!(!json.contains("connection_info"));
    }

    #[test]
    fn test_paired_device_from_session() {
        let local_keypair = EcdhKeyPair::generate();
        let mut session = PairingSession::new(local_keypair);

        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new("peer-device".to_string(), &peer_keypair.public_key_bytes());

        session.process_peer_data(&peer_data).unwrap();
        let shared_secret = session.shared_secret().unwrap().to_vec();

        let paired_device = session.complete().unwrap();

        // 验证共享密钥哈希正确
        assert!(paired_device.verify_shared_secret(&shared_secret));
    }

    #[test]
    fn test_paired_device_hash_consistency() {
        let secret = [0x12; 32];

        let device1 = PairedDevice::new("d1".to_string(), vec![], &secret, None);
        let device2 = PairedDevice::new("d2".to_string(), vec![], &secret, None);

        // 相同的密钥应该产生相同的哈希
        assert_eq!(device1.shared_secret_hash, device2.shared_secret_hash);
    }

    #[test]
    fn test_paired_device_clone() {
        let device = PairedDevice::new(
            "device".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            Some(ConnectionInfo::new().with_port(8080)),
        );

        let cloned = device.clone();
        assert_eq!(device, cloned);
    }
}
