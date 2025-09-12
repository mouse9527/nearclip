# Task 0201: 定义加密算法枚举 (TDD版本)

## 任务描述

按照TDD原则定义支持的加密算法枚举，为加密功能提供类型安全的基础。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/encryption_algorithm_tests.rs
#[cfg(test)]
mod encryption_algorithm_tests {
    use super::*;
    
    #[test]
    fn test_algorithm_key_length() {
        // RED: 测试算法密钥长度
        let algo = EncryptionAlgorithm::Aes256Gcm;
        assert_eq!(algo.key_length(), 32);
        
        let algo = EncryptionAlgorithm::ChaCha20Poly1305;
        assert_eq!(algo.key_length(), 32);
    }
    
    #[test]
    fn test_algorithm_display() {
        // RED: 测试算法显示
        let algo = EncryptionAlgorithm::Aes256Gcm;
        assert_eq!(format!("{}", algo), "AES-256-GCM");
        
        let algo = EncryptionAlgorithm::ChaCha20Poly1305;
        assert_eq!(format!("{}", algo), "ChaCha20-Poly1305");
    }
    
    #[test]
    fn test_algorithm_from_str() {
        // RED: 测试从字符串解析算法
        assert_eq!("aes256gcm".parse::<EncryptionAlgorithm>().unwrap(), 
                   EncryptionAlgorithm::Aes256Gcm);
        assert_eq!("chacha20poly1305".parse::<EncryptionAlgorithm>().unwrap(), 
                   EncryptionAlgorithm::ChaCha20Poly1305);
        assert!("unknown".parse::<EncryptionAlgorithm>().is_err());
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

use std::fmt;
use std::str::FromStr;

impl EncryptionAlgorithm {
    pub fn key_length(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
        }
    }
}

impl fmt::Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES-256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
        }
    }
}

impl FromStr for EncryptionAlgorithm {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aes256gcm" | "aes-256-gcm" => Ok(EncryptionAlgorithm::Aes256Gcm),
            "chacha20poly1305" | "chacha20-poly1305" => Ok(EncryptionAlgorithm::ChaCha20Poly1305),
            _ => Err(format!("Unknown encryption algorithm: {}", s)),
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn key_length(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
            EncryptionAlgorithm::XChaCha20Poly1305 => 32,
        }
    }
    
    pub fn nonce_length(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 12,
            EncryptionAlgorithm::ChaCha20Poly1305 => 12,
            EncryptionAlgorithm::XChaCha20Poly1305 => 24,
        }
    }
    
    pub fn tag_length(&self) -> usize {
        match self {
            EncryptionAlgorithm::Aes256Gcm => 16,
            EncryptionAlgorithm::ChaCha20Poly1305 => 16,
            EncryptionAlgorithm::XChaCha20Poly1305 => 16,
        }
    }
    
    pub fn is_supported_on_platform(&self) -> bool {
        // 检查硬件加速支持
        #[cfg(target_arch = "x86_64")]
        {
            if let EncryptionAlgorithm::Aes256Gcm = self {
                return std::is_x86_feature_detected!("aes") && 
                       std::is_x86_feature_detected!("pclmulqdq");
            }
        }
        true // ChaCha20-Poly1305 在所有平台上都支持
    }
    
    pub fn recommended_for(&self, use_case: KeyUseCase) -> bool {
        match use_case {
            KeyUseCase::FileEncryption => matches!(self, EncryptionAlgorithm::Aes256Gcm),
            KeyUseCase::RealTimeCommunication => matches!(self, EncryptionAlgorithm::ChaCha20Poly1305),
            KeyUseCase::LongTermStorage => matches!(self, EncryptionAlgorithm::Aes256Gcm),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyUseCase {
    FileEncryption,
    RealTimeCommunication,
    LongTermStorage,
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的加密基础枚举：

```rust
// rust-core/domain/entities/encryption.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncryptionAlgorithm {
    // 加密算法定义
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- 无

## 后续任务

- [Task 0202: 定义加密错误类型](0202-encryption-error-types.md)
- [Task 0203: 实现加密密钥基础结构](0203-encryption-key-basics.md)