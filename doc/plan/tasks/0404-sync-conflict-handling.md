# Task 0404: 实现同步冲突处理 (TDD版本)

## 任务描述

按照TDD原则实现同步冲突处理，解决多设备同时修改剪贴板内容时的冲突问题。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/sync_conflict_tests.rs
#[cfg(test)]
mod sync_conflict_tests {
    use super::*;
    
    #[test]
    fn test_conflict_detection() {
        // RED: 测试冲突检测
        let resolver = ConflictResolver::new();
        
        let local_content = SyncContent::new("Local content", "device-001");
        let remote_content = SyncContent::new("Remote content", "device-002");
        
        let conflict = Conflict::new(local_content, remote_content);
        
        assert!(resolver.has_conflict(&conflict));
        assert!(resolver.detect_conflict_type(&conflict) == ConflictType::ContentMismatch);
    }
    
    #[test]
    fn test_timestamp_based_resolution() {
        // RED: 测试基于时间戳的冲突解决
        let resolver = ConflictResolver::with_strategy(ResolutionStrategy::NewestWins);
        
        let mut older_content = SyncContent::new("Old content", "device-001");
        older_content.timestamp = SystemTime::now() - Duration::from_secs(60);
        
        let mut newer_content = SyncContent::new("New content", "device-002");
        newer_content.timestamp = SystemTime::now();
        
        let conflict = Conflict::new(older_content, newer_content);
        let resolution = resolver.resolve_conflict(conflict).unwrap();
        
        assert_eq!(resolution.resolved_content.text(), "New content");
        assert_eq!(resolution.winner_device_id, "device-002");
    }
    
    #[test]
    fn test_manual_resolution() {
        // RED: 测试手动冲突解决
        let resolver = ConflictResolver::with_strategy(ResolutionStrategy::Manual);
        
        let conflict = Conflict::new(
            SyncContent::new("Content A", "device-001"),
            SyncContent::new("Content B", "device-002"),
        );
        
        let resolution = resolver.resolve_manually(
            conflict,
            ManualResolution::ChooseLocal,
        ).unwrap();
        
        assert_eq!(resolution.resolved_content.text(), "Content A");
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
pub struct SyncContent {
    text: String,
    device_id: String,
    timestamp: SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    ContentMismatch,
    TimestampConflict,
    VersionConflict,
}

#[derive(Debug)]
pub struct Conflict {
    local_content: SyncContent,
    remote_content: SyncContent,
    conflict_type: ConflictType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionStrategy {
    NewestWins,
    OldestWins,
    LocalWins,
    RemoteWins,
    Manual,
    Merge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ManualResolution {
    ChooseLocal,
    ChooseRemote,
    MergeContent,
}

#[derive(Debug)]
pub struct ConflictResolution {
    resolved_content: SyncContent,
    winner_device_id: String,
    resolution_strategy: ResolutionStrategy,
    resolution_time: SystemTime,
}

#[derive(Debug)]
pub struct ConflictResolver {
    strategy: ResolutionStrategy,
}

impl ConflictResolver {
    pub fn new() -> Self {
        Self {
            strategy: ResolutionStrategy::NewestWins,
        }
    }
    
    pub fn with_strategy(strategy: ResolutionStrategy) -> Self {
        Self { strategy }
    }
    
    pub fn has_conflict(&self, conflict: &Conflict) -> bool {
        conflict.local_content.text != conflict.remote_content.text
    }
    
    pub fn detect_conflict_type(&self, conflict: &Conflict) -> ConflictType {
        if conflict.local_content.text != conflict.remote_content.text {
            ConflictType::ContentMismatch
        } else {
            ConflictType::TimestampConflict
        }
    }
    
    pub fn resolve_conflict(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        match self.strategy {
            ResolutionStrategy::NewestWins => self.resolve_newest_wins(conflict),
            ResolutionStrategy::OldestWins => self.resolve_oldest_wins(conflict),
            ResolutionStrategy::LocalWins => self.resolve_local_wins(conflict),
            ResolutionStrategy::RemoteWins => self.resolve_remote_wins(conflict),
            ResolutionStrategy::Manual => Err(ConflictError::ManualResolutionRequired),
            ResolutionStrategy::Merge => self.resolve_merge(conflict),
        }
    }
    
    pub fn resolve_manually(&self, conflict: Conflict, choice: ManualResolution) -> Result<ConflictResolution, ConflictError> {
        let (winner_content, winner_id) = match choice {
            ManualResolution::ChooseLocal => (conflict.local_content.clone(), conflict.local_content.device_id.clone()),
            ManualResolution::ChooseRemote => (conflict.remote_content.clone(), conflict.remote_content.device_id.clone()),
            ManualResolution::MergeContent => {
                let merged = format!("{} | {}", conflict.local_content.text, conflict.remote_content.text);
                let merged_content = SyncContent {
                    text: merged,
                    device_id: "merged".to_string(),
                    timestamp: SystemTime::now(),
                };
                (merged_content, "merged".to_string())
            }
        };
        
        Ok(ConflictResolution {
            resolved_content: winner_content,
            winner_device_id: winner_id,
            resolution_strategy: ResolutionStrategy::Manual,
            resolution_time: SystemTime::now(),
        })
    }
    
    fn resolve_newest_wins(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        if conflict.local_content.timestamp > conflict.remote_content.timestamp {
            Ok(ConflictResolution {
                resolved_content: conflict.local_content,
                winner_device_id: conflict.local_content.device_id,
                resolution_strategy: ResolutionStrategy::NewestWins,
                resolution_time: SystemTime::now(),
            })
        } else {
            Ok(ConflictResolution {
                resolved_content: conflict.remote_content,
                winner_device_id: conflict.remote_content.device_id,
                resolution_strategy: ResolutionStrategy::NewestWins,
                resolution_time: SystemTime::now(),
            })
        }
    }
    
    fn resolve_oldest_wins(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        if conflict.local_content.timestamp < conflict.remote_content.timestamp {
            Ok(ConflictResolution {
                resolved_content: conflict.local_content,
                winner_device_id: conflict.local_content.device_id,
                resolution_strategy: ResolutionStrategy::OldestWins,
                resolution_time: SystemTime::now(),
            })
        } else {
            Ok(ConflictResolution {
                resolved_content: conflict.remote_content,
                winner_device_id: conflict.remote_content.device_id,
                resolution_strategy: ResolutionStrategy::OldestWins,
                resolution_time: SystemTime::now(),
            })
        }
    }
    
    fn resolve_local_wins(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        Ok(ConflictResolution {
            resolved_content: conflict.local_content,
            winner_device_id: conflict.local_content.device_id,
            resolution_strategy: ResolutionStrategy::LocalWins,
            resolution_time: SystemTime::now(),
        })
    }
    
    fn resolve_remote_wins(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        Ok(ConflictResolution {
            resolved_content: conflict.remote_content,
            winner_device_id: conflict.remote_content.device_id,
            resolution_strategy: ResolutionStrategy::RemoteWins,
            resolution_time: SystemTime::now(),
        })
    }
    
    fn resolve_merge(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        let merged = self.smart_merge(&conflict.local_content.text, &conflict.remote_content.text);
        let merged_content = SyncContent {
            text: merged,
            device_id: "merged".to_string(),
            timestamp: SystemTime::now(),
        };
        
        Ok(ConflictResolution {
            resolved_content: merged_content,
            winner_device_id: "merged".to_string(),
            resolution_strategy: ResolutionStrategy::Merge,
            resolution_time: SystemTime::now(),
        })
    }
    
    fn smart_merge(&self, local: &str, remote: &str) -> String {
        // 简化的智能合并逻辑
        if local.len() > remote.len() {
            local.to_string()
        } else {
            remote.to_string()
        }
    }
}

impl SyncContent {
    pub fn new(text: &str, device_id: &str) -> Self {
        Self {
            text: text.to_string(),
            device_id: device_id.to_string(),
            timestamp: SystemTime::now(),
        }
    }
    
    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
}

impl Conflict {
    pub fn new(local_content: SyncContent, remote_content: SyncContent) -> Self {
        let conflict_type = if local_content.text != remote_content.text {
            ConflictType::ContentMismatch
        } else {
            ConflictType::TimestampConflict
        };
        
        Self {
            local_content,
            remote_content,
            conflict_type,
        }
    }
}

#[derive(Debug)]
pub enum ConflictError {
    ManualResolutionRequired,
    MergeFailed,
    InvalidContent,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncContent {
    pub text: String,
    pub device_id: String,
    pub timestamp: SystemTime,
    pub content_type: ContentType,
    pub metadata: ContentMetadata,
    pub version: u32,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    File,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub size_bytes: usize,
    pub word_count: Option<usize>,
    pub line_count: Option<usize>,
    pub language: Option<String>,
    pub source_app: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictType {
    ContentMismatch,
    TimestampConflict,
    VersionConflict,
    FormatConflict,
    SizeConflict,
    MetadataConflict,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    pub local_content: SyncContent,
    pub remote_content: SyncContent,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub context: ConflictContext,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Low,    // 可以自动解决
    Medium, // 需要用户确认
    High,   // 必须手动解决
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictContext {
    pub sync_session_id: String,
    pub device_names: HashMap<String, String>,
    pub network_conditions: NetworkConditions,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConditions {
    pub latency: Duration,
    pub bandwidth: Option<u64>,
    pub reliability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub default_resolution_strategy: ResolutionStrategy,
    pub auto_resolve_low_severity: bool,
    pub prefer_local_changes: bool,
    pub notification_preference: NotificationPreference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationPreference {
    Always,
    HighSeverityOnly,
    Never,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    NewestWins,
    OldestWins,
    LocalWins,
    RemoteWins,
    Manual,
    Merge,
    SmartMerge,
    ContentBased,
    ContextAware,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ManualResolution {
    ChooseLocal,
    ChooseRemote,
    MergeContent,
    CustomContent(String),
    Defer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub conflict_id: String,
    pub resolved_content: SyncContent,
    pub winner_device_id: String,
    pub resolution_strategy: ResolutionStrategy,
    pub resolution_time: SystemTime,
    pub user_action_required: bool,
    pub confidence_score: f32,
    pub applied_transformations: Vec<String>,
}

#[derive(Debug)]
pub struct ConflictResolver {
    pub strategy: ResolutionStrategy,
    pub config: ResolverConfig,
    pub history: ConflictHistory,
    pub merge_engine: MergeEngine,
}

#[derive(Debug, Clone)]
pub struct ResolverConfig {
    pub timeout: Duration,
    pub max_merge_attempts: u32,
    pub enable_smart_features: bool,
    pub confidence_threshold: f32,
}

#[derive(Debug)]
pub struct ConflictHistory {
    pub resolutions: VecDeque<ConflictResolution>,
    pub stats: ConflictStats,
}

#[derive(Debug, Clone)]
pub struct ConflictStats {
    pub total_conflicts: u32,
    pub auto_resolved: u32,
    pub manual_resolved: u32,
    pub average_resolution_time: Duration,
    pub most_common_strategy: ResolutionStrategy,
}

#[derive(Debug)]
pub struct MergeEngine {
    pub text_diff_algorithm: DiffAlgorithm,
    pub image_merger: Option<ImageMerger>,
    pub file_merger: Option<FileMerger>,
}

#[derive(Debug, Clone)]
pub enum DiffAlgorithm {
    Myers,
    Patience,
    Histogram,
}

#[derive(Debug)]
pub enum ConflictError {
    ManualResolutionRequired,
    MergeFailed(String),
    InvalidContent(String),
    Timeout,
    ConfigurationError,
    NetworkError(String),
}

impl ConflictResolver {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_config(ResolverConfig::default())
    }
    
    pub fn with_strategy(strategy: ResolutionStrategy) -> Self {
        Self {
            strategy,
            config: ResolverConfig::default(),
            history: ConflictHistory::new(),
            merge_engine: MergeEngine::new(),
        }
    }
    
    pub fn with_config(config: ResolverConfig) -> Self {
        Self {
            strategy: ResolutionStrategy::NewestWins,
            config,
            history: ConflictHistory::new(),
            merge_engine: MergeEngine::new(),
        }
    }
    
    pub fn has_conflict(&self, conflict: &Conflict) -> bool {
        match conflict.conflict_type {
            ConflictType::ContentMismatch => conflict.local_content.text != conflict.remote_content.text,
            ConflictType::TimestampConflict => {
                let time_diff = conflict.local_content.timestamp
                    .duration_since(conflict.remote_content.timestamp)
                    .unwrap_or_else(|_| conflict.remote_content.timestamp.duration_since(conflict.local_content.timestamp).unwrap_or(Duration::ZERO));
                time_diff < Duration::from_secs(1) // 1秒内的差异视为冲突
            }
            ConflictType::VersionConflict => conflict.local_content.version != conflict.remote_content.version,
            ConflictType::FormatConflict => conflict.local_content.content_type != conflict.remote_content.content_type,
            ConflictType::SizeConflict => {
                let size_diff = (conflict.local_content.metadata.size_bytes as i32 - conflict.remote_content.metadata.size_bytes as i32).abs();
                size_diff > 100 // 100字节差异视为冲突
            }
            ConflictType::MetadataConflict => {
                conflict.local_content.metadata.source_app != conflict.remote_content.metadata.source_app ||
                conflict.local_content.metadata.language != conflict.remote_content.metadata.language
            }
        }
    }
    
    pub fn detect_conflict_type(&self, conflict: &Conflict) -> ConflictType {
        if conflict.local_content.text != conflict.remote_content.text {
            ConflictType::ContentMismatch
        } else if conflict.local_content.version != conflict.remote_content.version {
            ConflictType::VersionConflict
        } else if conflict.local_content.content_type != conflict.remote_content.content_type {
            ConflictType::FormatConflict
        } else {
            ConflictType::TimestampConflict
        }
    }
    
    pub fn resolve_conflict(&self, conflict: Conflict) -> Result<ConflictResolution, ConflictError> {
        let start_time = SystemTime::now();
        
        let resolution = match self.strategy {
            ResolutionStrategy::NewestWins => self.resolve_newest_wins(&conflict),
            ResolutionStrategy::OldestWins => self.resolve_oldest_wins(&conflict),
            ResolutionStrategy::LocalWins => self.resolve_local_wins(&conflict),
            ResolutionStrategy::RemoteWins => self.resolve_remote_wins(&conflict),
            ResolutionStrategy::Manual => Err(ConflictError::ManualResolutionRequired),
            ResolutionStrategy::Merge => self.resolve_merge(&conflict),
            ResolutionStrategy::SmartMerge => self.resolve_smart_merge(&conflict),
            ResolutionStrategy::ContentBased => self.resolve_content_based(&conflict),
            ResolutionStrategy::ContextAware => self.resolve_context_aware(&conflict),
        }?;
        
        // 记录解决历史
        self.history.record_resolution(resolution.clone());
        
        Ok(resolution)
    }
    
    pub fn resolve_manually(&self, conflict: Conflict, choice: ManualResolution) -> Result<ConflictResolution, ConflictError> {
        let (winner_content, winner_id, transformations) = match choice {
            ManualResolution::ChooseLocal => (conflict.local_content, conflict.local_content.device_id, vec!["local_wins".to_string()]),
            ManualResolution::ChooseRemote => (conflict.remote_content, conflict.remote_content.device_id, vec!["remote_wins".to_string()]),
            ManualResolution::MergeContent => {
                let merged = self.merge_engine.merge_text(
                    &conflict.local_content.text,
                    &conflict.remote_content.text,
                )?;
                let merged_content = SyncContent {
                    text: merged,
                    device_id: "merged".to_string(),
                    timestamp: SystemTime::now(),
                    content_type: conflict.local_content.content_type,
                    metadata: conflict.local_content.metadata.clone(),
                    version: conflict.local_content.version.max(conflict.remote_content.version) + 1,
                    hash: Self::calculate_hash(&merged),
                };
                (merged_content, "merged".to_string(), vec!["merged".to_string()])
            }
            ManualResolution::CustomContent(custom_text) => {
                let custom_content = SyncContent {
                    text: custom_text,
                    device_id: "custom".to_string(),
                    timestamp: SystemTime::now(),
                    content_type: ContentType::Text,
                    metadata: ContentMetadata::new(&custom_text),
                    version: 1,
                    hash: Self::calculate_hash(&custom_text),
                };
                (custom_content, "custom".to_string(), vec!["custom".to_string()])
            }
            ManualResolution::Defer => return Err(ConflictError::ManualResolutionRequired),
        };
        
        Ok(ConflictResolution {
            conflict_id: conflict.id,
            resolved_content: winner_content,
            winner_device_id: winner_id,
            resolution_strategy: ResolutionStrategy::Manual,
            resolution_time: SystemTime::now(),
            user_action_required: false,
            confidence_score: 1.0,
            applied_transformations: transformations,
        })
    }
    
    fn resolve_newest_wins(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        let (winner_content, winner_id) = if conflict.local_content.timestamp > conflict.remote_content.timestamp {
            (conflict.local_content.clone(), conflict.local_content.device_id.clone())
        } else {
            (conflict.remote_content.clone(), conflict.remote_content.device_id.clone())
        };
        
        Ok(self.create_resolution(conflict.id.clone(), winner_content, winner_id, ResolutionStrategy::NewestWins))
    }
    
    fn resolve_oldest_wins(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        let (winner_content, winner_id) = if conflict.local_content.timestamp < conflict.remote_content.timestamp {
            (conflict.local_content.clone(), conflict.local_content.device_id.clone())
        } else {
            (conflict.remote_content.clone(), conflict.remote_content.device_id.clone())
        };
        
        Ok(self.create_resolution(conflict.id.clone(), winner_content, winner_id, ResolutionStrategy::OldestWins))
    }
    
    fn resolve_local_wins(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        Ok(self.create_resolution(
            conflict.id.clone(),
            conflict.local_content.clone(),
            conflict.local_content.device_id.clone(),
            ResolutionStrategy::LocalWins,
        ))
    }
    
    fn resolve_remote_wins(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        Ok(self.create_resolution(
            conflict.id.clone(),
            conflict.remote_content.clone(),
            conflict.remote_content.device_id.clone(),
            ResolutionStrategy::RemoteWins,
        ))
    }
    
    fn resolve_merge(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        let merged_text = self.merge_engine.merge_text(
            &conflict.local_content.text,
            &conflict.remote_content.text,
        )?;
        
        let merged_content = SyncContent {
            text: merged_text,
            device_id: "merged".to_string(),
            timestamp: SystemTime::now(),
            content_type: conflict.local_content.content_type,
            metadata: conflict.local_content.metadata.clone(),
            version: conflict.local_content.version.max(conflict.remote_content.version) + 1,
            hash: Self::calculate_hash(&merged_text),
        };
        
        Ok(self.create_resolution(
            conflict.id.clone(),
            merged_content,
            "merged".to_string(),
            ResolutionStrategy::Merge,
        ))
    }
    
    fn resolve_smart_merge(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        // 智能合并，考虑多种因素
        let confidence = self.calculate_merge_confidence(conflict);
        
        if confidence < self.config.confidence_threshold {
            return Err(ConflictError::MergeFailed("Low confidence in merge result".to_string()));
        }
        
        self.resolve_merge(conflict)
    }
    
    fn resolve_content_based(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        // 基于内容特征选择解决策略
        let local_size = conflict.local_content.text.len();
        let remote_size = conflict.remote_content.text.len();
        
        if (local_size as f32 / remote_size as f32) > 2.0 {
            // 本地内容明显更长，保留本地
            self.resolve_local_wins(conflict)
        } else if (remote_size as f32 / local_size as f32) > 2.0 {
            // 远程内容明显更长，保留远程
            self.resolve_remote_wins(conflict)
        } else {
            // 大小相近，使用时间戳
            self.resolve_newest_wins(conflict)
        }
    }
    
    fn resolve_context_aware(&self, conflict: &Conflict) -> Result<ConflictResolution, ConflictError> {
        // 基于上下文信息解决冲突
        if let Some(prefer_local) = conflict.context.user_preferences.prefer_local_changes {
            if prefer_local {
                return self.resolve_local_wins(conflict);
            } else {
                return self.resolve_remote_wins(conflict);
            }
        }
        
        // 考虑网络条件
        if conflict.context.network_conditions.reliability < 0.5 {
            // 网络不可靠时，保守策略
            self.resolve_oldest_wins(conflict)
        } else {
            // 网络良好时，积极策略
            self.resolve_newest_wins(conflict)
        }
    }
    
    fn create_resolution(&self, conflict_id: String, content: SyncContent, winner_id: String, strategy: ResolutionStrategy) -> ConflictResolution {
        ConflictResolution {
            conflict_id,
            resolved_content: content,
            winner_device_id: winner_id,
            resolution_strategy: strategy,
            resolution_time: SystemTime::now(),
            user_action_required: false,
            confidence_score: 0.8,
            applied_transformations: vec![format!("{:?}", strategy).to_lowercase()],
        }
    }
    
    fn calculate_merge_confidence(&self, conflict: &Conflict) -> f32 {
        let mut confidence = 0.5;
        
        // 内容相似度
        let similarity = self.calculate_text_similarity(
            &conflict.local_content.text,
            &conflict.remote_content.text,
        );
        confidence += similarity * 0.3;
        
        // 时间差
        let time_diff = conflict.local_content.timestamp
            .duration_since(conflict.remote_content.timestamp)
            .unwrap_or_else(|_| conflict.remote_content.timestamp.duration_since(conflict.local_content.timestamp).unwrap_or(Duration::ZERO));
        
        if time_diff < Duration::from_secs(5) {
            confidence += 0.2;
        }
        
        // 内容长度比
        let size_ratio = conflict.local_content.text.len() as f32 / conflict.remote_content.text.len() as f32;
        if (0.5..=2.0).contains(&size_ratio) {
            confidence += 0.1;
        }
        
        confidence.min(1.0)
    }
    
    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        // 简化的文本相似度计算
        if text1 == text2 {
            1.0
        } else if text1.is_empty() || text2.is_empty() {
            0.0
        } else {
            let words1: std::collections::HashSet<_> = text1.split_whitespace().collect();
            let words2: std::collections::HashSet<_> = text2.split_whitespace().collect();
            
            let intersection = words1.intersection(&words2).count();
            let union = words1.union(&words2).count();
            
            if union == 0 {
                0.0
            } else {
                intersection as f32 / union as f32
            }
        }
    }
    
    fn calculate_hash(content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl SyncContent {
    pub fn new(text: &str, device_id: &str) -> Self {
        Self {
            text: text.to_string(),
            device_id: device_id.to_string(),
            timestamp: SystemTime::now(),
            content_type: ContentType::Text,
            metadata: ContentMetadata::new(text),
            version: 1,
            hash: Self::calculate_hash(text),
        }
    }
    
    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    
    fn calculate_hash(text: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl Conflict {
    pub fn new(local_content: SyncContent, remote_content: SyncContent) -> Self {
        use uuid::Uuid;
        
        let conflict_type = if local_content.text != remote_content.text {
            ConflictType::ContentMismatch
        } else if local_content.version != remote_content.version {
            ConflictType::VersionConflict
        } else {
            ConflictType::TimestampConflict
        };
        
        let severity = Self::calculate_severity(&conflict_type, &local_content, &remote_content);
        
        Self {
            id: Uuid::new_v4().to_string(),
            local_content,
            remote_content,
            conflict_type,
            severity,
            context: ConflictContext::default(),
            created_at: SystemTime::now(),
        }
    }
    
    fn calculate_severity(conflict_type: &ConflictType, local: &SyncContent, remote: &SyncContent) -> ConflictSeverity {
        match conflict_type {
            ConflictType::ContentMismatch => {
                let size_diff = (local.metadata.size_bytes as i32 - remote.metadata.size_bytes as i32).abs();
                if size_diff > 1000 {
                    ConflictSeverity::High
                } else if size_diff > 100 {
                    ConflictSeverity::Medium
                } else {
                    ConflictSeverity::Low
                }
            }
            ConflictType::VersionConflict => ConflictSeverity::Low,
            ConflictType::FormatConflict => ConflictSeverity::High,
            ConflictType::SizeConflict => ConflictSeverity::Medium,
            ConflictType::TimestampConflict => ConflictSeverity::Low,
            ConflictType::MetadataConflict => ConflictSeverity::Low,
        }
    }
}

impl ConflictHistory {
    fn new() -> Self {
        Self {
            resolutions: VecDeque::with_capacity(100),
            stats: ConflictStats::new(),
        }
    }
    
    fn record_resolution(&mut self, resolution: ConflictResolution) {
        self.resolutions.push_back(resolution);
        if self.resolutions.len() > 100 {
            self.resolutions.pop_front();
        }
        self.stats.update(&self.resolutions);
    }
}

impl ConflictStats {
    fn new() -> Self {
        Self {
            total_conflicts: 0,
            auto_resolved: 0,
            manual_resolved: 0,
            average_resolution_time: Duration::ZERO,
            most_common_strategy: ResolutionStrategy::NewestWins,
        }
    }
    
    fn update(&mut self, resolutions: &VecDeque<ConflictResolution>) {
        self.total_conflicts = resolutions.len() as u32;
        self.auto_resolved = resolutions.iter()
            .filter(|r| !r.user_action_required)
            .count() as u32;
        self.manual_resolved = self.total_conflicts - self.auto_resolved;
        
        // 更新最常用策略
        let mut strategy_counts = std::collections::HashMap::new();
        for resolution in resolutions {
            *strategy_counts.entry(resolution.resolution_strategy.clone()).or_insert(0) += 1;
        }
        
        self.most_common_strategy = strategy_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(strategy, _)| strategy)
            .unwrap_or(ResolutionStrategy::NewestWins);
    }
}

impl MergeEngine {
    fn new() -> Self {
        Self {
            text_diff_algorithm: DiffAlgorithm::Myers,
            image_merger: None,
            file_merger: None,
        }
    }
    
    fn merge_text(&self, text1: &str, text2: &str) -> Result<String, ConflictError> {
        // 简化的文本合并实现
        if text1.is_empty() {
            return Ok(text2.to_string());
        }
        if text2.is_empty() {
            return Ok(text1.to_string());
        }
        
        // 基本合并策略：选择较长的内容
        if text1.len() >= text2.len() {
            Ok(text1.to_string())
        } else {
            Ok(text2.to_string())
        }
    }
}

impl ContentMetadata {
    fn new(text: &str) -> Self {
        Self {
            size_bytes: text.len(),
            word_count: Some(text.split_whitespace().count()),
            line_count: Some(text.lines().count()),
            language: None,
            source_app: None,
        }
    }
}

impl Default for ConflictContext {
    fn default() -> Self {
        Self {
            sync_session_id: Uuid::new_v4().to_string(),
            device_names: HashMap::new(),
            network_conditions: NetworkConditions::default(),
            user_preferences: UserPreferences::default(),
        }
    }
}

impl Default for NetworkConditions {
    fn default() -> Self {
        Self {
            latency: Duration::from_millis(50),
            bandwidth: Some(1_000_000), // 1Mbps
            reliability: 0.9,
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_resolution_strategy: ResolutionStrategy::NewestWins,
            auto_resolve_low_severity: true,
            prefer_local_changes: false,
            notification_preference: NotificationPreference::HighSeverityOnly,
        }
    }
}

impl Default for ResolverConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_merge_attempts: 3,
            enable_smart_features: true,
            confidence_threshold: 0.7,
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的冲突处理：

```rust
// rust-core/domain/sync/conflict.rs
pub struct ConflictResolver {
    // 同步冲突处理实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [内容变化检测](0403-content-change-detection.md)

## 后续任务

- Task 0405: 实现同步历史记录
- Task 0406: 实现内容过滤机制
- Task 0407: 实现同步性能优化