# Task 0201: 定义加密密钥结构 (TDD版本)

## 任务描述

按照TDD原则定义NearClip的加密密钥基础结构，为安全通信建立基础。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/encryption_key_tests.rs
#[cfg(test)]
mod encryption_key_tests {
    use super::*;
    
    #[test]
    fn test_encryption_key_creation() {
        // RED: 测试加密密钥创建
        let key_bytes = vec![1u8; 32]; // AES-256需要32字节
        let key = EncryptionKey::new(key_bytes);
        
        assert_eq!(key.key_bytes().len(), 32);
        assert_eq!(key.algorithm(), EncryptionAlgorithm::Aes256Gcm);
        assert!(key.created_at() <= SystemTime::now());
    }
    
    #[test]
    fn test_encryption_key_validation() {
        // RED: 测试密钥验证
        // 无效密钥长度
        let invalid_bytes = vec![1u8; 16]; // 太短
        let result = EncryptionKey::new(invalid_bytes);
        assert!(matches!(result, Err(EncryptionError::InvalidKeyLength(_))));
        
        // 有效密钥长度
        let valid_bytes = vec![1u8; 32];
        let result = EncryptionKey::new(valid_bytes);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_key_serialization() {
        // RED: 测试密钥序列化
        let key_bytes = vec![42u8; 32];
        let key = EncryptionKey::new(key_bytes.clone()).unwrap();
        
        let serialized = key.to_bytes();
        assert_eq!(serialized, key_bytes);
        
        // 反序列化
        let deserialized = EncryptionKey::from_bytes(&serialized);
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap().key_bytes(), key_bytes);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
}

#[derive(Debug)]
pub struct EncryptionKey {
    key_bytes: Vec<u8>,
    algorithm: EncryptionAlgorithm,
    created_at: SystemTime,
}

impl EncryptionKey {
    pub fn new(key_bytes: Vec<u8>) -> Result<Self, EncryptionError> {
        if key_bytes.len() != 32 {
            return Err(EncryptionError::InvalidKeyLength(key_bytes.len()));
        }
        
        Ok(Self {
            key_bytes,
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            created_at: SystemTime::now(),
        })
    }
    
    pub fn key_bytes(&self) -> &[u8] {
        &self.key_bytes
    }
    
    pub fn algorithm(&self) -> &EncryptionAlgorithm {
        &self.algorithm
    }
    
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key_bytes.clone()
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EncryptionError> {
        Self::new(bytes.to_vec())
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct EncryptionKey {
    key_bytes: Vec<u8>,
    algorithm: EncryptionAlgorithm,
    created_at: SystemTime,
    key_id: String,
    metadata: KeyMetadata,
}

#[derive(Debug, Clone)]
pub struct KeyMetadata {
    pub key_type: KeyType,
    pub expires_at: Option<SystemTime>,
    pub usage_count: u32,
    pub is_revoked: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Symmetric,
    AsymmetricPublic,
    AsymmetricPrivate,
}

impl EncryptionKey {
    // 重构后的代码，保持测试绿色
    pub fn new(key_bytes: Vec<u8>) -> Result<Self, EncryptionError> {
        Self::with_algorithm(key_bytes, EncryptionAlgorithm::Aes256Gcm)
    }
    
    pub fn with_algorithm(key_bytes: Vec<u8>, algorithm: EncryptionAlgorithm) -> Result<Self, EncryptionError> {
        Self::validate_key_length(&key_bytes, &algorithm)?;
        
        let key_id = uuid::Uuid::new_v4().to_string();
        
        Ok(Self {
            key_bytes,
            algorithm,
            created_at: SystemTime::now(),
            key_id,
            metadata: KeyMetadata {
                key_type: KeyType::Symmetric,
                expires_at: None,
                usage_count: 0,
                is_revoked: false,
            },
        })
    }
    
    fn validate_key_length(key_bytes: &[u8], algorithm: &EncryptionAlgorithm) -> Result<(), EncryptionError> {
        let required_length = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => 32,
        };
        
        if key_bytes.len() != required_length {
            return Err(EncryptionError::InvalidKeyLength {
                expected: required_length,
                actual: key_bytes.len(),
            });
        }
        
        Ok(())
    }
    
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.metadata.expires_at {
            SystemTime::now() > expires_at
        } else {
            false
        }
    }
    
    pub fn increment_usage(&mut self) {
        self.metadata.usage_count += 1;
    }
    
    pub fn revoke(&mut self) {
        self.metadata.is_revoked = true;
    }
    
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.metadata.is_revoked
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的核心实体，不依赖外部实现：

```rust
// rust-core/domain/entities/encryption.rs
#[derive(Debug)]
pub struct EncryptionKey {
    // 加密密钥核心属性
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [设备抽象层](../tasks/0101-device-abstraction-layer.md)

## 后续任务

- [Task 0202: 实现密钥生成器](../tasks/0202-key-generator.md)
- [Task 0203: 实现安全上下文基础结构](../tasks/0203-security-context.md)
- [Task 0204: 实现密钥序列化](../tasks/0204-key-serialization.md)