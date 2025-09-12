# Task 0202: 实现密钥生成器 (TDD版本)

## 任务描述

按照TDD原则实现加密密钥生成器，提供安全的随机密钥生成功能。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/key_generator_tests.rs
#[cfg(test)]
mod key_generator_tests {
    use super::*;
    
    #[test]
    fn test_aes_key_generation() {
        // RED: 测试AES密钥生成
        let generator = KeyGenerator::new();
        
        let key = generator.generate_aes256_key().unwrap();
        
        assert_eq!(key.key_bytes().len(), 32);
        assert_eq!(key.algorithm(), &EncryptionAlgorithm::Aes256Gcm);
        assert!(!key.key_bytes().iter().all(|&b| b == 0)); // 不是全零
    }
    
    #[test]
    fn test_key_uniqueness() {
        // RED: 测试密钥唯一性
        let generator = KeyGenerator::new();
        
        let key1 = generator.generate_aes256_key().unwrap();
        let key2 = generator.generate_aes256_key().unwrap();
        
        // 两次生成的密钥应该不同
        assert_ne!(key1.key_bytes(), key2.key_bytes());
        assert_ne!(key1.key_id(), key2.key_id());
    }
    
    #[test]
    fn test_key_generation_performance() {
        // RED: 测试密钥生成性能
        let generator = KeyGenerator::new();
        
        let start = SystemTime::now();
        for _ in 0..100 {
            generator.generate_aes256_key().unwrap();
        }
        let duration = start.elapsed().unwrap();
        
        // 100个密钥应该在100ms内生成完成
        assert!(duration.as_millis() < 100);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不多
use rand::RngCore;

pub struct KeyGenerator {
    rng: Box<dyn RngCore>,
}

impl KeyGenerator {
    pub fn new() -> Self {
        Self {
            rng: Box::new(rand::thread_rng()),
        }
    }
    
    pub fn generate_aes256_key(&mut self) -> Result<EncryptionKey, EncryptionError> {
        let mut key_bytes = vec![0u8; 32];
        self.rng.fill_bytes(&mut key_bytes);
        
        EncryptionKey::new(key_bytes)
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub struct KeyGenerator {
    rng: ChaCha20Rng,
}

impl KeyGenerator {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_entropy(&[])
    }
    
    pub fn with_entropy(entropy: &[u8]) -> Self {
        let seed = if entropy.is_empty() {
            // 使用系统随机性作为种子
            let mut seed = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut seed);
            seed
        } else {
            // 使用提供的熵作为种子
            let mut seed = [0u8; 32];
            seed.iter_mut().zip(entropy.iter()).for_each(|(s, &e)| *s = e);
            seed
        };
        
        Self {
            rng: ChaCha20Rng::from_seed(seed),
        }
    }
    
    pub fn generate_aes256_key(&mut self) -> Result<EncryptionKey, EncryptionKey> {
        let mut key_bytes = [0u8; 32];
        self.rng.fill_bytes(&mut key_bytes);
        
        Ok(EncryptionKey::new(key_bytes.to_vec())?)
    }
    
    pub fn generate_key_pair(&mut self) -> Result<(EncryptionKey, EncryptionKey), EncryptionError> {
        // 生成密钥对（暂时返回相同的对称密钥）
        let private_key = self.generate_aes256_key()?;
        let public_key = self.generate_aes256_key()?;
        
        Ok((private_key, public_key))
    }
    
    pub fn reseed(&mut self, entropy: &[u8]) {
        let mut seed = [0u8; 32];
        seed.iter_mut().zip(entropy.iter()).for_each(|(s, &e)| *s = e);
        self.rng = ChaCha20Rng::from_seed(seed);
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为infrastructure层的具体实现：

```rust
// rust-core/infrastructure/security/key_generator.rs
pub struct KeyGenerator {
    // 密钥生成器具体实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查
- [ ] 性能测试通过

## 依赖任务

- [加密密钥结构](0201-encryption-key-structure.md)

## 后续任务

- [Task 0203: 实现安全上下文基础结构](0203-security-context.md)
- [Task 0204: 实现配对会话管理](0204-pairing-session.md)