//! ECDH P-256 密钥对生成和管理
//!
//! 提供设备配对所需的密钥对生成、导出和共享密钥计算功能。
//!
//! # Example
//!
//! ```
//! use nearclip_crypto::EcdhKeyPair;
//!
//! // 生成新密钥对
//! let keypair = EcdhKeyPair::generate();
//!
//! // 导出公钥用于交换
//! let public_bytes = keypair.public_key_bytes();
//! assert_eq!(public_bytes.len(), 65); // uncompressed format
//!
//! // 导出私钥用于安全存储
//! let private_bytes = keypair.private_key_bytes();
//! assert_eq!(private_bytes.len(), 32);
//! ```

use p256::{
    ecdh::diffie_hellman,
    elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint},
    EncodedPoint, PublicKey, SecretKey,
};
use rand_core::OsRng;
use thiserror::Error;
use tracing::{debug, instrument};

/// 加密模块错误类型
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CryptoError {
    /// 无效的私钥字节
    #[error("Invalid private key bytes: {0}")]
    InvalidPrivateKey(String),

    /// 无效的公钥字节
    #[error("Invalid public key bytes: {0}")]
    InvalidPublicKey(String),

    /// 证书生成失败
    #[error("Certificate generation failed: {0}")]
    CertificateGeneration(String),

    /// TLS 配置失败
    #[error("TLS configuration failed: {0}")]
    TlsConfiguration(String),

    /// JSON 序列化失败
    #[error("JSON serialization failed: {0}")]
    JsonSerialization(String),

    /// 二维码生成失败
    #[error("QR code generation failed: {0}")]
    QrCodeGeneration(String),

    /// 无效的配对数据
    #[error("Invalid pairing data: {0}")]
    InvalidPairingData(String),

    /// 二维码解析失败
    #[error("QR code parsing failed: {0}")]
    QrCodeParsing(String),

    /// 配对失败
    #[error("Pairing failed: {0}")]
    PairingFailed(String),

    /// 设备存储错误
    #[error("Device store error: {0}")]
    DeviceStore(String),
}

impl Default for CryptoError {
    fn default() -> Self {
        CryptoError::InvalidPrivateKey(String::new())
    }
}

/// ECDH P-256 密钥对
///
/// 用于设备配对时的密钥交换。私钥可安全存储，
/// 公钥可导出用于与其他设备交换。
///
/// # Example
///
/// ```
/// use nearclip_crypto::EcdhKeyPair;
///
/// // 生成新密钥对
/// let keypair = EcdhKeyPair::generate();
///
/// // 导出公钥用于交换
/// let public_bytes = keypair.public_key_bytes();
///
/// // 导出私钥用于安全存储
/// let private_bytes = keypair.private_key_bytes();
///
/// // 从私钥恢复
/// let restored = EcdhKeyPair::from_private_key_bytes(&private_bytes).unwrap();
/// assert_eq!(keypair.public_key_bytes(), restored.public_key_bytes());
/// ```
#[derive(Clone)]
pub struct EcdhKeyPair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl EcdhKeyPair {
    /// 生成新的 ECDH P-256 密钥对
    ///
    /// 使用操作系统的安全随机数生成器创建密钥对。
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::EcdhKeyPair;
    ///
    /// let keypair = EcdhKeyPair::generate();
    /// ```
    #[instrument]
    pub fn generate() -> Self {
        let secret_key = SecretKey::random(&mut OsRng);
        let public_key = secret_key.public_key();
        debug!("Generated new ECDH P-256 keypair");
        Self {
            secret_key,
            public_key,
        }
    }

    /// 从私钥字节恢复密钥对
    ///
    /// # Arguments
    ///
    /// * `bytes` - 32 字节的私钥标量值
    ///
    /// # Returns
    ///
    /// 恢复的密钥对，或错误如果字节无效
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::EcdhKeyPair;
    ///
    /// let original = EcdhKeyPair::generate();
    /// let private_bytes = original.private_key_bytes();
    ///
    /// let restored = EcdhKeyPair::from_private_key_bytes(&private_bytes).unwrap();
    /// assert_eq!(original.public_key_bytes(), restored.public_key_bytes());
    /// ```
    #[instrument(skip(bytes))]
    pub fn from_private_key_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        let secret_key = SecretKey::from_slice(bytes)
            .map_err(|e| CryptoError::InvalidPrivateKey(e.to_string()))?;
        let public_key = secret_key.public_key();
        debug!("Restored ECDH keypair from private key bytes");
        Ok(Self {
            secret_key,
            public_key,
        })
    }

    /// 导出私钥字节
    ///
    /// 返回 32 字节的私钥标量值。
    ///
    /// **安全警告：** 私钥必须安全存储，不可记录到日志或明文传输。
    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }

    /// 导出公钥字节（未压缩格式）
    ///
    /// 返回 65 字节的公钥（0x04 + X坐标 + Y坐标）。
    /// 这是最常用的公钥格式，兼容性最好。
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key
            .to_encoded_point(false) // uncompressed
            .as_bytes()
            .to_vec()
    }

    /// 导出公钥字节（压缩格式）
    ///
    /// 返回 33 字节的公钥（0x02 或 0x03 + X坐标）。
    /// 压缩格式节省空间，适合带宽受限场景。
    pub fn public_key_bytes_compressed(&self) -> Vec<u8> {
        self.public_key
            .to_encoded_point(true) // compressed
            .as_bytes()
            .to_vec()
    }

    /// 计算与对方公钥的共享密钥
    ///
    /// 使用 ECDH (Elliptic Curve Diffie-Hellman) 算法计算共享密钥。
    /// 双方使用各自的私钥和对方的公钥，可以得到相同的共享密钥。
    ///
    /// # Arguments
    ///
    /// * `peer_public_bytes` - 对方的公钥字节（33 字节压缩或 65 字节未压缩格式）
    ///
    /// # Returns
    ///
    /// 32 字节的共享密钥，可用于派生对称加密密钥
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::EcdhKeyPair;
    ///
    /// let alice = EcdhKeyPair::generate();
    /// let bob = EcdhKeyPair::generate();
    ///
    /// // 交换公钥后计算共享密钥
    /// let alice_shared = alice.compute_shared_secret(&bob.public_key_bytes()).unwrap();
    /// let bob_shared = bob.compute_shared_secret(&alice.public_key_bytes()).unwrap();
    ///
    /// // 双方得到相同的共享密钥
    /// assert_eq!(alice_shared, bob_shared);
    /// ```
    #[instrument(skip(self, peer_public_bytes), fields(peer_key_len = peer_public_bytes.len()))]
    pub fn compute_shared_secret(&self, peer_public_bytes: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let encoded_point = EncodedPoint::from_bytes(peer_public_bytes)
            .map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))?;

        let peer_public: Option<PublicKey> = PublicKey::from_encoded_point(&encoded_point).into();
        let peer_public = peer_public
            .ok_or_else(|| CryptoError::InvalidPublicKey("Invalid point on curve".to_string()))?;

        let shared_secret = diffie_hellman(
            self.secret_key.to_nonzero_scalar(),
            peer_public.as_affine(),
        );

        debug!("Computed shared secret successfully");
        Ok(shared_secret.raw_secret_bytes().to_vec())
    }
}

impl std::fmt::Debug for EcdhKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 安全实现：不输出私钥内容，只显示公钥前几个字节
        let public_bytes = self.public_key_bytes();
        let preview = if public_bytes.len() >= 8 {
            format!(
                "{:02x}{:02x}{:02x}{:02x}...",
                public_bytes[0], public_bytes[1], public_bytes[2], public_bytes[3]
            )
        } else {
            "invalid".to_string()
        };
        f.debug_struct("EcdhKeyPair")
            .field("public_key_preview", &preview)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let keypair = EcdhKeyPair::generate();
        assert_eq!(keypair.private_key_bytes().len(), 32);
        assert_eq!(keypair.public_key_bytes().len(), 65);
    }

    #[test]
    fn test_public_key_uncompressed_format() {
        let keypair = EcdhKeyPair::generate();
        let public_bytes = keypair.public_key_bytes();
        // 未压缩格式以 0x04 开头
        assert_eq!(public_bytes[0], 0x04);
        assert_eq!(public_bytes.len(), 65);
    }

    #[test]
    fn test_public_key_compressed_format() {
        let keypair = EcdhKeyPair::generate();
        let public_bytes = keypair.public_key_bytes_compressed();
        assert_eq!(public_bytes.len(), 33);
        // 压缩格式以 0x02 或 0x03 开头
        assert!(public_bytes[0] == 0x02 || public_bytes[0] == 0x03);
    }

    #[test]
    fn test_restore_from_private_key() {
        let original = EcdhKeyPair::generate();
        let private_bytes = original.private_key_bytes();

        let restored = EcdhKeyPair::from_private_key_bytes(&private_bytes).unwrap();

        assert_eq!(original.public_key_bytes(), restored.public_key_bytes());
        assert_eq!(original.private_key_bytes(), restored.private_key_bytes());
    }

    #[test]
    fn test_invalid_private_key_wrong_length() {
        let invalid_bytes = vec![0u8; 31]; // 应该是 32 字节
        let result = EcdhKeyPair::from_private_key_bytes(&invalid_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidPrivateKey(_))));
    }

    #[test]
    fn test_invalid_private_key_zero() {
        let zero_bytes = vec![0u8; 32]; // 全零是无效的私钥
        let result = EcdhKeyPair::from_private_key_bytes(&zero_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_secret_consistency() {
        let alice = EcdhKeyPair::generate();
        let bob = EcdhKeyPair::generate();

        let alice_shared = alice
            .compute_shared_secret(&bob.public_key_bytes())
            .unwrap();
        let bob_shared = bob
            .compute_shared_secret(&alice.public_key_bytes())
            .unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn test_shared_secret_with_compressed_key() {
        let alice = EcdhKeyPair::generate();
        let bob = EcdhKeyPair::generate();

        // 使用压缩格式公钥也能正确计算
        let alice_shared = alice
            .compute_shared_secret(&bob.public_key_bytes_compressed())
            .unwrap();
        let bob_shared = bob
            .compute_shared_secret(&alice.public_key_bytes())
            .unwrap();

        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn test_invalid_public_key_wrong_length() {
        let keypair = EcdhKeyPair::generate();
        let invalid_public = vec![0x04u8; 10]; // 错误长度

        let result = keypair.compute_shared_secret(&invalid_public);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidPublicKey(_))));
    }

    #[test]
    fn test_invalid_public_key_not_on_curve() {
        let keypair = EcdhKeyPair::generate();
        // 构造一个格式正确但不在曲线上的点
        let mut invalid_public = vec![0x04u8]; // uncompressed marker
        invalid_public.extend_from_slice(&[0xFFu8; 32]); // X coordinate
        invalid_public.extend_from_slice(&[0xFFu8; 32]); // Y coordinate

        let result = keypair.compute_shared_secret(&invalid_public);
        assert!(result.is_err());
    }

    #[test]
    fn test_debug_does_not_leak_private_key() {
        let keypair = EcdhKeyPair::generate();
        let debug_str = format!("{:?}", keypair);

        // Debug 输出不应包含完整私钥
        let private_hex = hex::encode(keypair.private_key_bytes());
        assert!(!debug_str.contains(&private_hex));

        // 应该包含 public_key_preview
        assert!(debug_str.contains("public_key_preview"));
    }

    #[test]
    fn test_keypair_clone() {
        let original = EcdhKeyPair::generate();
        let cloned = original.clone();

        assert_eq!(original.private_key_bytes(), cloned.private_key_bytes());
        assert_eq!(original.public_key_bytes(), cloned.public_key_bytes());
    }

    #[test]
    fn test_different_keypairs_different_shared_secrets() {
        let alice = EcdhKeyPair::generate();
        let bob = EcdhKeyPair::generate();
        let charlie = EcdhKeyPair::generate();

        let alice_bob = alice
            .compute_shared_secret(&bob.public_key_bytes())
            .unwrap();
        let alice_charlie = alice
            .compute_shared_secret(&charlie.public_key_bytes())
            .unwrap();

        // 与不同人的共享密钥应该不同
        assert_ne!(alice_bob, alice_charlie);
    }

    #[test]
    fn test_keypair_is_send_sync() {
        // 编译时断言：EcdhKeyPair 必须是 Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EcdhKeyPair>();
    }

    #[test]
    fn test_crypto_error_is_send_sync() {
        // 编译时断言：CryptoError 必须是 Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CryptoError>();
    }

    #[test]
    fn test_crypto_error_default() {
        let default_err = CryptoError::default();
        assert!(matches!(default_err, CryptoError::InvalidPrivateKey(_)));
    }
}
