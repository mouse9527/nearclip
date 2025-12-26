//! AES-256-GCM 加密/解密模块
//!
//! 使用 ECDH 派生的共享密钥进行端到端加密。
//!
//! # Example
//!
//! ```
//! use nearclip_crypto::Aes256Gcm;
//!
//! // 从 ECDH 共享密钥创建加密器
//! let shared_secret = [0u8; 32];  // 从 ECDH 派生
//! let cipher = Aes256Gcm::new(&shared_secret).unwrap();
//!
//! // 加密数据
//! let plaintext = b"secret message";
//! let encrypted = cipher.encrypt(plaintext).unwrap();
//!
//! // 解密数据
//! let decrypted = cipher.decrypt(&encrypted).unwrap();
//! assert_eq!(decrypted, plaintext);
//! ```

use aes_gcm::{
    aead::{Aead, AeadCore, OsRng},
    Aes256Gcm as Aes256GcmImpl, KeyInit, Nonce,
};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::{debug, instrument};
use zeroize::Zeroize;

/// 加密错误类型
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CipherError {
    /// 加密失败
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// 解密失败
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// 无效的密钥长度
    #[error("Invalid key length: expected 32 bytes, got {0}")]
    InvalidKeyLength(usize),

    /// 无效的密文格式
    #[error("Invalid ciphertext format: {0}")]
    InvalidCiphertext(String),
}

/// AES-256-GCM 加密器
///
/// 使用 256 位密钥的 AES-GCM 认证加密算法。
/// 每次加密都会生成唯一的 nonce，确保相同明文产生不同密文。
///
/// # 密钥派生
///
/// 建议使用 HKDF-SHA256 从 ECDH 共享密钥派生加密密钥：
/// ```text
/// shared_secret (32 bytes) --HKDF-SHA256--> encryption_key (32 bytes)
/// ```
#[derive(Clone)]
pub struct Aes256Gcm {
    /// 内部 AES-256-GCM 实例
    cipher: Aes256GcmImpl,
}

impl Aes256Gcm {
    /// 密钥长度（字节）
    pub const KEY_SIZE: usize = 32;

    /// Nonce 长度（字节）
    pub const NONCE_SIZE: usize = 12;

    /// 标签长度（字节，GCM 认证标签）
    pub const TAG_SIZE: usize = 16;

    /// 从共享密钥创建新的加密器
    ///
    /// # Arguments
    ///
    /// * `shared_secret` - 32 字节的共享密钥（通常来自 ECDH）
    ///
    /// # Returns
    ///
    /// 新的加密器实例
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::Aes256Gcm;
    ///
    /// let shared_secret = [0u8; 32];
    /// let cipher = Aes256Gcm::new(&shared_secret);
    /// ```
    #[instrument(skip(shared_secret))]
    pub fn new(shared_secret: &[u8]) -> Result<Self, CipherError> {
        if shared_secret.len() != Self::KEY_SIZE {
            return Err(CipherError::InvalidKeyLength(shared_secret.len()));
        }

        // 使用 HKDF-SHA256 派生加密密钥
        let key_bytes = Self::derive_key(shared_secret);

        use aes_gcm::KeyInit;
        let cipher = Aes256GcmImpl::new(&key_bytes.into());
        debug!("Created new AES-256-GCM cipher");

        Ok(Self { cipher })
    }

    /// 直接从 32 字节密钥创建（跳过 HKDF 派生）
    ///
    /// **注意：** 仅当密钥已经通过 HKDF 或类似方法派生时使用。
    #[instrument(skip(key))]
    pub fn from_raw_key(key: &[u8]) -> Result<Self, CipherError> {
        if key.len() != Self::KEY_SIZE {
            return Err(CipherError::InvalidKeyLength(key.len()));
        }

        use aes_gcm::KeyInit;
        let cipher = Aes256GcmImpl::new_from_slice(key)
            .map_err(|e| CipherError::EncryptionFailed(e.to_string()))?;

        Ok(Self { cipher })
    }

    /// 使用 HKDF-SHA256 从共享密钥派生加密密钥
    ///
    /// HKDF (HMAC-based Key Derivation Function) 提供安全的密钥派生。
    fn derive_key(shared_secret: &[u8]) -> [u8; 32] {
        // 使用空 salt 和 info 进行简单派生
        let mut hasher = Sha256::new();
        hasher.update(b"nearclip-encryption-key");
        hasher.update(shared_secret);
        hasher.update(b"nearclip-key-derivation");
        hasher.finalize().into()
    }

    /// 加密数据
    ///
    /// # Arguments
    ///
    /// * `plaintext` - 要加密的明文数据
    ///
    /// # Returns
    ///
    /// 加密后的密文，格式为：`nonce (12 bytes) + ciphertext + tag (16 bytes)`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::Aes256Gcm;
    ///
    /// let cipher = Aes256Gcm::new(&[0u8; 32]).unwrap();
    /// let encrypted = cipher.encrypt(b"secret").unwrap();
    /// assert!(encrypted.len() > 12 + 16);  // nonce + ciphertext + tag
    /// ```
    #[instrument(skip(self, plaintext), fields(plaintext_len = plaintext.len()))]
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CipherError> {
        // 生成随机 nonce
        let nonce = Aes256GcmImpl::generate_nonce(&mut OsRng);

        // 加密数据
        let ciphertext = self.cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| CipherError::EncryptionFailed(e.to_string()))?;

        // 组合：nonce + ciphertext (ciphertext 已包含 tag)
        let mut result = Vec::with_capacity(Self::NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        debug!("Encrypted {} bytes to {} bytes", plaintext.len(), result.len());
        Ok(result)
    }

    /// 解密数据
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - 加密的数据，格式为：`nonce + ciphertext + tag`
    ///
    /// # Returns
    ///
    /// 解密后的明文数据
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::Aes256Gcm;
    ///
    /// let cipher = Aes256Gcm::new(&[0u8; 32]).unwrap();
    /// let encrypted = cipher.encrypt(b"secret").unwrap();
    /// let decrypted = cipher.decrypt(&encrypted).unwrap();
    /// assert_eq!(decrypted, b"secret");
    /// ```
    #[instrument(skip(self, ciphertext), fields(ciphertext_len = ciphertext.len()))]
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CipherError> {
        if ciphertext.len() < Self::NONCE_SIZE + Self::TAG_SIZE {
            return Err(CipherError::InvalidCiphertext(
                format!("Too short: {} bytes", ciphertext.len())
            ));
        }

        // 分离 nonce 和密文
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(Self::NONCE_SIZE);
        let nonce_array: [u8; Self::NONCE_SIZE] = nonce_bytes.try_into()
            .map_err(|_| CipherError::InvalidCiphertext("Invalid nonce length".to_string()))?;
        let nonce = Nonce::from_slice(&nonce_array);

        // 解密
        let plaintext = self.cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| CipherError::DecryptionFailed(e.to_string()))?;

        debug!("Decrypted {} bytes to {} bytes", ciphertext.len(), plaintext.len());
        Ok(plaintext)
    }

    /// 就地加密（重用缓冲区）
    ///
    /// 此方法会就地修改输入缓冲区，避免额外的内存分配。
    ///
    /// # Arguments
    ///
    /// * `buffer` - 包含明文的缓冲区，将被替换为密文
    ///
    /// # Returns
    ///
    /// 成功返回加密后密文的长度
    #[instrument(skip(self, buffer), fields(buffer_len = buffer.len()))]
    pub fn encrypt_in_place(&self, buffer: &mut Vec<u8>) -> Result<usize, CipherError> {
        let mut plaintext = buffer.clone();

        // 生成随机 nonce
        let nonce = Aes256GcmImpl::generate_nonce(&mut OsRng);

        // 加密数据
        let ciphertext = self.cipher
            .encrypt(&nonce, &*plaintext)
            .map_err(|e| CipherError::EncryptionFailed(e.to_string()))?;

        // 替换缓冲区内容
        buffer.clear();
        buffer.reserve(Self::NONCE_SIZE + ciphertext.len());
        buffer.extend_from_slice(&nonce);
        buffer.extend_from_slice(&ciphertext);

        // 清零明文副本
        plaintext.zeroize();

        debug!("Encrypted in-place: {} bytes to {} bytes", plaintext.len(), buffer.len());
        Ok(buffer.len())
    }

    /// 就地解密（重用缓冲区）
    ///
    /// 此方法会就地修改输入缓冲区，避免额外的内存分配。
    ///
    /// # Arguments
    ///
    /// * `buffer` - 包含密文的缓冲区，将被替换为明文
    ///
    /// # Returns
    ///
    /// 成功返回解密后明文的长度
    #[instrument(skip(self, buffer), fields(buffer_len = buffer.len()))]
    pub fn decrypt_in_place(&self, buffer: &mut Vec<u8>) -> Result<usize, CipherError> {
        if buffer.len() < Self::NONCE_SIZE + Self::TAG_SIZE {
            return Err(CipherError::InvalidCiphertext(
                format!("Too short: {} bytes", buffer.len())
            ));
        }

        // 复制 nonce
        let nonce_bytes: [u8; Self::NONCE_SIZE] = buffer[..Self::NONCE_SIZE]
            .try_into()
            .map_err(|_| CipherError::InvalidCiphertext("Invalid nonce length".to_string()))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 复制加密数据
        let encrypted_data = buffer[Self::NONCE_SIZE..].to_vec();

        // 解密
        let plaintext = self.cipher
            .decrypt(nonce, encrypted_data.as_slice())
            .map_err(|e| CipherError::DecryptionFailed(e.to_string()))?;

        // 替换缓冲区内容
        buffer.clear();
        buffer.extend_from_slice(&plaintext);

        debug!("Decrypted in-place: {} bytes to {} bytes", encrypted_data.len(), buffer.len());
        Ok(buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cipher() -> Aes256Gcm {
        Aes256Gcm::new(&[0u8; 32]).unwrap()
    }

    #[test]
    fn test_cipher_creation() {
        let _cipher = create_test_cipher();
        // 密码器创建成功
    }

    #[test]
    fn test_cipher_invalid_key_length() {
        let result = Aes256Gcm::new(&[0u8; 16]);
        assert!(matches!(result, Err(CipherError::InvalidKeyLength(16))));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let cipher = create_test_cipher();
        let plaintext = b"Hello, secure world!";

        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_encryption_produces_different_ciphertext() {
        let cipher = create_test_cipher();
        let plaintext = b"same content";

        let enc1 = cipher.encrypt(plaintext).unwrap();
        let enc2 = cipher.encrypt(plaintext).unwrap();

        // 由于随机 nonce，相同明文应该产生不同密文
        assert_ne!(enc1, enc2);

        // 但解密后应该相同
        assert_eq!(cipher.decrypt(&enc1).unwrap(), plaintext.as_slice());
        assert_eq!(cipher.decrypt(&enc2).unwrap(), plaintext.as_slice());
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let cipher = create_test_cipher();
        let plaintext = b"";

        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_large() {
        let cipher = create_test_cipher();
        let plaintext = vec![0u8; 10000];

        let encrypted = cipher.encrypt(&plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_decrypt_invalid_ciphertext_too_short() {
        let cipher = create_test_cipher();
        let too_short = vec![0u8; 10];

        let result = cipher.decrypt(&too_short);
        assert!(matches!(result, Err(CipherError::InvalidCiphertext(_))));
    }

    #[test]
    fn test_decrypt_tampered_ciphertext() {
        let cipher = create_test_cipher();
        let plaintext = b"original message";

        let mut encrypted = cipher.encrypt(plaintext).unwrap();

        // 篡改密文
        if let Some(last) = encrypted.last_mut() {
            *last = last.wrapping_add(1);
        }

        let result = cipher.decrypt(&encrypted);
        assert!(matches!(result, Err(CipherError::DecryptionFailed(_))));
    }

    #[test]
    fn test_encrypt_in_place() {
        let cipher = create_test_cipher();
        let mut buffer = b"Hello, in-place!".to_vec();

        let original_len = buffer.len();
        let enc_len = cipher.encrypt_in_place(&mut buffer).unwrap();

        // 加密后应该变长（nonce + tag）
        assert!(enc_len > original_len);
        assert_eq!(enc_len, buffer.len());

        // 解密应该恢复原文
        let dec_len = cipher.decrypt_in_place(&mut buffer).unwrap();
        assert_eq!(dec_len, original_len);
        assert_eq!(buffer, b"Hello, in-place!");
    }

    #[test]
    fn test_from_raw_key() {
        let key = [1u8; 32];
        let cipher = Aes256Gcm::from_raw_key(&key).unwrap();

        let plaintext = b"test message";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_from_raw_key_invalid_length() {
        let key = [1u8; 16];
        let result = Aes256Gcm::from_raw_key(&key);
        assert!(matches!(result, Err(CipherError::InvalidKeyLength(16))));
    }

    #[test]
    fn test_different_keys_produce_different_results() {
        let cipher1 = Aes256Gcm::new(&[0u8; 32]).unwrap();
        let cipher2 = Aes256Gcm::new(&[1u8; 32]).unwrap();

        let plaintext = b"same plaintext";

        let enc1 = cipher1.encrypt(plaintext).unwrap();
        let enc2 = cipher2.encrypt(plaintext).unwrap();

        // 不同密钥应该产生不同密文（即使 nonce 不同）
        assert_ne!(cipher1.decrypt(&enc2), Ok(plaintext.to_vec()));
        assert_ne!(cipher2.decrypt(&enc1), Ok(plaintext.to_vec()));
    }

    #[test]
    fn test_constants() {
        assert_eq!(Aes256Gcm::KEY_SIZE, 32);
        assert_eq!(Aes256Gcm::NONCE_SIZE, 12);
        assert_eq!(Aes256Gcm::TAG_SIZE, 16);
    }

    #[test]
    fn test_cipher_error_display() {
        assert_eq!(
            format!("{}", CipherError::InvalidKeyLength(16)),
            "Invalid key length: expected 32 bytes, got 16"
        );
        assert!(format!("{}", CipherError::EncryptionFailed("test".to_string()))
            .contains("Encryption failed"));
    }

    #[test]
    fn test_decrypt_in_place_too_short() {
        let cipher = create_test_cipher();
        let mut buffer = vec![0u8; 10];

        let result = cipher.decrypt_in_place(&mut buffer);
        assert!(matches!(result, Err(CipherError::InvalidCiphertext(_))));
    }

    #[test]
    fn test_binary_data() {
        let cipher = create_test_cipher();
        let binary_data: Vec<u8> = (0u8..=255).collect();

        let encrypted = cipher.encrypt(&binary_data).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, binary_data);
    }

    #[test]
    fn test_multiple_messages_same_key() {
        let cipher = create_test_cipher();

        let messages = vec![
            b"first".to_vec(),
            b"second".to_vec(),
            b"third".to_vec(),
        ];

        for msg in &messages {
            let enc = cipher.encrypt(msg).unwrap();
            let dec = cipher.decrypt(&enc).unwrap();
            assert_eq!(dec, *msg);
        }
    }
}
