//! 加密服务模块
//!
//! 提供端到端加密、数字签名、密钥管理等功能

use crate::error::{NearClipError, Result};
use ring::{aead, rand, signature};
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{AES_256_GCM, LessSafeKey, Nonce, UnboundKey};
use signature::{Ed25519KeyPair, KeyPair as SignatureKeyPair, Signature};
use std::sync::Arc;

/// 加密服务 - 提供端到端加密功能
pub struct CryptoService {
    rng: SystemRandom,
    device_key_pair: Arc<Ed25519KeyPair>,
}

impl CryptoService {
    /// 创建新的加密服务实例
    pub fn new() -> Result<Self> {
        let rng = SystemRandom::new();
        let device_key_pair = Self::generate_device_keypair(&rng)?;

        Ok(CryptoService {
            rng,
            device_key_pair: Arc::new(device_key_pair),
        })
    }

    /// 从现有密钥对创建加密服务
    pub fn from_keypair(key_pair_bytes: &[u8]) -> Result<Self> {
        let device_key_pair = Ed25519KeyPair::from_pkcs8(key_pair_bytes)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;

        Ok(CryptoService {
            rng: SystemRandom::new(),
            device_key_pair: Arc::new(device_key_pair),
        })
    }

    /// 生成设备密钥对
    fn generate_device_keypair(rng: &SystemRandom) -> Result<Ed25519KeyPair> {
        let key_pair_bytes = signature::Ed25519KeyPair::generate_pkcs8(rng)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;

        Ed25519KeyPair::from_pkcs8(&key_pair_bytes)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))
    }

    /// 获取设备公钥
    pub fn get_device_public_key(&self) -> &[u8] {
        self.device_key_pair.public_key().as_ref()
    }

    /// 获取设备私钥的 PKCS8 格式
    pub fn get_device_private_key(&self) -> Vec<u8> {
        self.device_key_pair.as_ref().as_ref().to_vec()
    }

    /// 生成临时 AES 密钥用于消息加密
    pub fn generate_session_key(&self) -> Result<Vec<u8>> {
        let mut key = vec![0u8; 32]; // 256 bits for AES-256
        self.rng.fill(&mut key)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;
        Ok(key)
    }

    /// 生成随机 Nonce
    pub fn generate_nonce(&self) -> Result<Vec<u8>> {
        let mut nonce = vec![0u8; 12]; // 96 bits for AES-GCM
        self.rng.fill(&mut nonce)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;
        Ok(nonce)
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(NearClipError::InvalidParameter("Key must be 32 bytes".to_string()));
        }
        if nonce.len() != 12 {
            return Err(NearClipError::InvalidParameter("Nonce must be 12 bytes".to_string()));
        }

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());

        let mut ciphertext = plaintext.to_vec();
        less_safe_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut ciphertext)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;

        Ok(ciphertext)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(NearClipError::InvalidParameter("Key must be 32 bytes".to_string()));
        }
        if nonce.len() != 12 {
            return Err(NearClipError::InvalidParameter("Nonce must be 12 bytes".to_string()));
        }

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());

        let mut plaintext = ciphertext.to_vec();
        let decrypted_len = less_safe_key.open_in_place(nonce, aead::Aad::empty(), &mut plaintext)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;

        plaintext.truncate(decrypted_len);
        Ok(plaintext)
    }

    /// 数字签名
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.device_key_pair.sign(message).as_ref().to_vec()
    }

    /// 验证签名
    pub fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<()> {
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);

        public_key.verify(message, signature)
            .map_err(|_| NearClipError::CryptoError("Signature verification failed".to_string()))
    }

    /// 生成安全的配对码
    pub fn generate_pairing_code(&self) -> Result<String> {
        let mut code = [0u8; 4];
        self.rng.fill(&mut code)
            .map_err(|e| NearClipError::CryptoError(e.to_string()))?;

        // 转换为6位数字配对码
        let code_num = u32::from_be_bytes(code) % 1_000_000;
        Ok(format!("{:06}", code_num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let crypto = CryptoService::new().unwrap();
        let key = crypto.generate_session_key().unwrap();
        let nonce = crypto.generate_nonce().unwrap();

        let plaintext = b"Hello, NearClip!";
        let ciphertext = crypto.encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted = crypto.decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_signature_verification() {
        let crypto1 = CryptoService::new().unwrap();
        let crypto2 = CryptoService::new().unwrap();

        let message = b"Test message";
        let signature = crypto1.sign(message);

        // 验证签名应该成功
        assert!(crypto1.verify(message, &signature, crypto1.get_device_public_key()).is_ok());

        // 使用错误的公钥验证应该失败
        assert!(crypto1.verify(message, &signature, crypto2.get_device_public_key()).is_err());
    }

    #[test]
    fn test_pairing_code_generation() {
        let crypto = CryptoService::new().unwrap();
        let code1 = crypto.generate_pairing_code().unwrap();
        let code2 = crypto.generate_pairing_code().unwrap();

        assert_ne!(code1, code2);
        assert_eq!(code1.len(), 6);
        assert_eq!(code2.len(), 6);
    }
}