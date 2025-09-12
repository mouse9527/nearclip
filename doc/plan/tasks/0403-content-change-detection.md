# Task 0403: 实现内容变化检测 (TDD版本)

## 任务描述

按照TDD原则实现内容变化检测算法，判断是否真的有新内容需要同步。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/content_change_detection_tests.rs
#[cfg(test)]
mod content_change_detection_tests {
    use super::*;
    
    #[test]
    fn test_simple_content_change() {
        // RED: 测试简单内容变化检测
        let detector = ContentChangeDetector::new();
        
        let result1 = detector.detect_change("Hello");
        assert!(result1.has_changed);
        
        let result2 = detector.detect_change("Hello World");
        assert!(result2.has_changed);
    }
    
    #[test]
    fn test_duplicate_content_detection() {
        // RED: 测试重复内容检测
        let detector = ContentChangeDetector::new();
        
        // 第一次设置内容
        let result1 = detector.detect_change("Same content");
        assert!(result1.has_changed);
        
        // 第二次设置相同内容
        let result2 = detector.detect_change("Same content");
        assert!(!result2.has_changed);
    }
    
    #[test]
    fn test_whitespace_ignoring() {
        // RED: 测试空白字符忽略
        let detector = ContentChangeDetector::new().ignore_whitespace(true);
        
        let result1 = detector.detect_change("Hello World");
        assert!(result1.has_changed);
        
        let result2 = detector.detect_change("Hello   World"); // 多余空格
        assert!(!result2.has_changed); // 应该视为相同内容
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug)]
pub struct ContentChangeDetector {
    last_content: String,
    ignore_whitespace: bool,
}

#[derive(Debug)]
pub struct ChangeDetectionResult {
    pub has_changed: bool,
    pub similarity_score: f32,
}

impl ContentChangeDetector {
    pub fn new() -> Self {
        Self {
            last_content: String::new(),
            ignore_whitespace: false,
        }
    }
    
    pub fn ignore_whitespace(mut self, ignore: bool) -> Self {
        self.ignore_whitespace = ignore;
        self
    }
    
    pub fn detect_change(&mut self, content: &str) -> ChangeDetectionResult {
        let normalized_content = if self.ignore_whitespace {
            content.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            content.to_string()
        };
        
        let has_changed = normalized_content != self.last_content;
        let similarity_score = if has_changed { 0.0 } else { 1.0 };
        
        if has_changed {
            self.last_content = normalized_content;
        }
        
        ChangeDetectionResult {
            has_changed,
            similarity_score,
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码难度
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ContentChangeDetector {
    config: DetectionConfig,
    state: DetectionState,
    cache: ContentCache,
}

#[derive(Debug, Clone)]
pub struct DetectionConfig {
    pub ignore_whitespace: bool,
    pub ignore_case: bool,
    pub min_change_threshold: f32,
    pub fuzzy_matching: bool,
    pub normalization: NormalizationLevel,
    pub change_timeout: std::time::Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NormalizationLevel {
    None,
    Basic,     // 空白和大小写
    Advanced,  // 标点符号、特殊字符
    Full,      // 完整标准化
}

#[derive(Debug)]
pub struct DetectionState {
    last_content: String,
    last_normalized: String,
    last_hash: u64,
    change_count: u32,
    last_change_time: SystemTime,
    content_history: Vec<String>,
}

#[derive(Debug)]
pub struct ContentCache {
    similarity_cache: HashMap<(String, String), f32>,
    hash_cache: HashMap<String, u64>,
}

#[derive(Debug)]
pub struct ChangeDetectionResult {
    pub has_changed: bool,
    pub similarity_score: f32,
    pub change_type: ChangeType,
    pub confidence: f32,
    pub normalized_content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    None,
    Append,
    Prepend,
    Replace,
    Delete,
    FormatChange,
}

impl ContentChangeDetector {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_config(DetectionConfig::default())
    }
    
    pub fn with_config(config: DetectionConfig) -> Self {
        Self {
            config,
            state: DetectionState::new(),
            cache: ContentCache::new(),
        }
    }
    
    pub fn ignore_whitespace(mut self, ignore: bool) -> Self {
        self.config.ignore_whitespace = ignore;
        self
    }
    
    pub fn detect_change(&mut self, content: &str) -> ChangeDetectionResult {
        let normalized = self.normalize_content(content);
        let content_hash = self.calculate_content_hash(&normalized);
        
        let (has_changed, similarity_score, change_type) = if self.state.last_content.is_empty() {
            // 第一次内容，总是有变化
            (true, 0.0, ChangeType::Append)
        } else if self.state.last_hash == content_hash {
            // 完全相同的内容
            (false, 1.0, ChangeType::None)
        } else {
            // 需要计算相似度
            let similarity = self.calculate_similarity(&self.state.last_normalized, &normalized);
            let change_type = self.detect_change_type(&self.state.last_normalized, &normalized);
            
            let has_changed = similarity < self.config.min_change_threshold;
            (has_changed, similarity, change_type)
        };
        
        let confidence = self.calculate_confidence(content, &normalized, has_changed);
        
        if has_changed {
            self.update_state(content.to_string(), normalized.clone(), content_hash);
        }
        
        ChangeDetectionResult {
            has_changed,
            similarity_score,
            change_type,
            confidence,
            normalized_content: normalized,
        }
    }
    
    fn normalize_content(&self, content: &str) -> String {
        let mut normalized = content.to_string();
        
        match self.config.normalization {
            NormalizationLevel::None => {}
            NormalizationLevel::Basic => {
                if self.config.ignore_whitespace {
                    normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
                }
                if self.config.ignore_case {
                    normalized = normalized.to_lowercase();
                }
            }
            NormalizationLevel::Advanced => {
                // 移除多余空格
                normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
                
                // 标准化标点符号
                normalized = normalized.replace("，", ",").replace("。", ".");
                
                if self.config.ignore_case {
                    normalized = normalized.to_lowercase();
                }
            }
            NormalizationLevel::Full => {
                // 完整标准化：移除所有格式化，只保留文本内容
                normalized = normalized
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .collect();
                normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
                normalized = normalized.to_lowercase();
            }
        }
        
        normalized.trim().to_string()
    }
    
    fn calculate_content_hash(&self, content: &str) -> u64 {
        if let Some(&cached_hash) = self.cache.hash_cache.get(content) {
            return cached_hash;
        }
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();
        
        self.cache.hash_cache.insert(content.to_string(), hash);
        hash
    }
    
    fn calculate_similarity(&self, content1: &str, content2: &str) -> f32 {
        let cache_key = (content1.to_string(), content2.to_string());
        
        if let Some(&cached_similarity) = self.cache.similarity_cache.get(&cache_key) {
            return cached_similarity;
        }
        
        let similarity = if self.config.fuzzy_matching {
            self.calculate_fuzzy_similarity(content1, content2)
        } else {
            self.calculate_exact_similarity(content1, content2)
        };
        
        self.cache.similarity_cache.insert(cache_key, similarity);
        similarity
    }
    
    fn calculate_exact_similarity(&self, content1: &str, content2: &str) -> f32 {
        if content1 == content2 {
            1.0
        } else if content1.is_empty() || content2.is_empty() {
            0.0
        } else {
            // Levenshtein距离相似度
            let distance = self.levenshtein_distance(content1, content2);
            let max_len = content1.len().max(content2.len());
            if max_len == 0 {
                1.0
            } else {
                1.0 - (distance as f32 / max_len as f32)
            }
        }
    }
    
    fn calculate_fuzzy_similarity(&self, content1: &str, content2: &str) -> f32 {
        // 使用多种算法计算相似度
        let levenshtein_sim = self.calculate_exact_similarity(content1, content2);
        
        // Jaccard相似度
        let jaccard_sim = self.jaccard_similarity(content1, content2);
        
        // Cosine相似度
        let cosine_sim = self.cosine_similarity(content1, content2);
        
        // 加权平均
        0.4 * levenshtein_sim + 0.3 * jaccard_sim + 0.3 * cosine_sim
    }
    
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        
        let len1 = s1.len();
        let len2 = s2.len();
        
        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        
        matrix[len1][len2]
    }
    
    fn jaccard_similarity(&self, s1: &str, s2: &str) -> f32 {
        let set1: std::collections::HashSet<_> = s1.chars().collect();
        let set2: std::collections::HashSet<_> = s2.chars().collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 {
            1.0
        } else {
            intersection as f32 / union as f32
        }
    }
    
    fn cosine_similarity(&self, s1: &str, s2: &str) -> f32 {
        // 将字符串转换为词袋向量
        let words1: Vec<&str> = s1.split_whitespace().collect();
        let words2: Vec<&str> = s2.split_whitespace().collect();
        
        let mut all_words = std::collections::HashSet::new();
        all_words.extend(words1.iter());
        all_words.extend(words2.iter());
        
        if all_words.is_empty() {
            return 1.0;
        }
        
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        
        for word in &all_words {
            let count1 = words1.iter().filter(|&&w| w == word).count() as f32;
            let count2 = words2.iter().filter(|&&w| w == word).count() as f32;
            vec1.push(count1);
            vec2.push(count2);
        }
        
        // 计算余弦相似度
        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let magnitude1: f32 = vec1.iter().map(|a| a * a).sum::<f32>().sqrt();
        let magnitude2: f32 = vec2.iter().map(|a| a * a).sum::<f32>().sqrt();
        
        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            0.0
        } else {
            dot_product / (magnitude1 * magnitude2)
        }
    }
    
    fn detect_change_type(&self, old_content: &str, new_content: &str) -> ChangeType {
        if old_content.is_empty() {
            return ChangeType::Append;
        }
        if new_content.is_empty() {
            return ChangeType::Delete;
        }
        
        if new_content.starts_with(old_content) {
            ChangeType::Append
        } else if old_content.starts_with(new_content) {
            ChangeType::Prepend
        } else if old_content.len() == new_content.len() {
            ChangeType::Replace
        } else {
            ChangeType::FormatChange
        }
    }
    
    fn calculate_confidence(&self, original: &str, normalized: &str, has_changed: bool) -> f32 {
        let mut confidence = 1.0;
        
        // 标准化程度影响置信度
        match self.config.normalization {
            NormalizationLevel::None => {}
            NormalizationLevel::Basic => {
                if original.len() != normalized.len() {
                    confidence *= 0.9;
                }
            }
            NormalizationLevel::Advanced | NormalizationLevel::Full => {
                confidence *= 0.8;
            }
        }
        
        // 模糊匹配降低置信度
        if self.config.fuzzy_matching {
            confidence *= 0.9;
        }
        
        // 无变化时置信度应该很高
        if !has_changed {
            confidence = 0.99;
        }
        
        confidence.min(1.0).max(0.0)
    }
    
    fn update_state(&mut self, content: String, normalized: String, hash: u64) {
        self.state.last_content = content;
        self.state.last_normalized = normalized;
        self.state.last_hash = hash;
        self.state.change_count += 1;
        self.state.last_change_time = SystemTime::now();
        
        // 更新历史记录
        self.state.content_history.push(self.state.last_content.clone());
        if self.state.content_history.len() > 10 {
            self.state.content_history.remove(0);
        }
    }
    
    pub fn get_change_history(&self) -> &[String] {
        &self.state.content_history
    }
    
    pub fn change_count(&self) -> u32 {
        self.state.change_count
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    pub fn reset(&mut self) {
        self.state = DetectionState::new();
        self.cache.clear();
    }
}

impl DetectionState {
    fn new() -> Self {
        Self {
            last_content: String::new(),
            last_normalized: String::new(),
            last_hash: 0,
            change_count: 0,
            last_change_time: SystemTime::now(),
            content_history: Vec::new(),
        }
    }
}

impl ContentCache {
    fn new() -> Self {
        Self {
            similarity_cache: HashMap::new(),
            hash_cache: HashMap::new(),
        }
    }
    
    fn clear(&mut self) {
        self.similarity_cache.clear();
        self.hash_cache.clear();
    }
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            ignore_whitespace: true,
            ignore_case: false,
            min_change_threshold: 0.95,
            fuzzy_matching: false,
            normalization: NormalizationLevel::Basic,
            change_timeout: std::time::Duration::from_secs(1),
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的内容检测：

```rust
// rust-core/domain/detection/content.rs
pub struct ContentChangeDetector {
    // 内容变化检测实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [同步策略](0402-sync-strategy.md)

## 后续任务

- [Task 0404: 实现同步冲突处理](0404-sync-conflict-handling.md)
- [Task 0405: 实现同步历史记录](0405-sync-history.md)
- [Task 0406: 实现内容过滤机制](0406-content-filter.md)