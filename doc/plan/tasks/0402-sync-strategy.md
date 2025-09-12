# Task 0402: 实现同步策略 (TDD版本)

## 任务描述

按照TDD原则实现自动同步策略，决定何时以及如何同步剪贴板内容。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/sync_strategy_tests.rs
#[cfg(test)]
mod sync_strategy_tests {
    use super::*;
    
    #[test]
    fn test_immediate_sync_strategy() {
        // RED: 测试立即同步策略
        let strategy = SyncStrategy::immediate();
        let content = "Test content";
        
        let decision = strategy.should_sync(content);
        
        assert!(matches!(decision, SyncDecision::SyncImmediately));
    }
    
    #[test]
    fn test_length_based_strategy() {
        // RED: 测试基于长度的同步策略
        let strategy = SyncStrategy::length_based(100); // 超过100字符不同步
        let short_content = "Short text";
        let long_content = "a".repeat(150);
        
        let short_decision = strategy.should_sync(short_content);
        let long_decision = strategy.should_sync(&long_content);
        
        assert!(matches!(short_decision, SyncDecision::SyncImmediately));
        assert!(matches!(long_decision, SyncDecision::Skip));
    }
    
    #[test]
    fn test_time_based_strategy() {
        // RED: 测试基于时间的同步策略
        let strategy = SyncStrategy::time_based(std::time::Duration::from_secs(5));
        
        // 第一次变化应该同步
        let decision1 = strategy.should_sync("content1");
        assert!(matches!(decision1, SyncDecision::SyncImmediately));
        
        // 短时间内的第二次变化应该跳过
        let decision2 = strategy.should_sync("content2");
        assert!(matches!(decision2, SyncDecision::Skip));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::time::{SystemTime, Duration};

#[derive(Debug, PartialEq)]
pub enum SyncDecision {
    SyncImmediately,
    Skip,
    Delay(Duration),
}

#[derive(Debug)]
pub struct SyncStrategy {
    strategy_type: StrategyType,
    last_sync_time: SystemTime,
}

#[derive(Debug)]
enum StrategyType {
    Immediate,
    LengthBased { max_length: usize },
    TimeBased { min_interval: Duration },
}

impl SyncStrategy {
    pub fn immediate() -> Self {
        Self {
            strategy_type: StrategyType::Immediate,
            last_sync_time: SystemTime::now() - Duration::from_secs(100), // 确保可以立即同步
        }
    }
    
    pub fn length_based(max_length: usize) -> Self {
        Self {
            strategy_type: StrategyType::LengthBased { max_length },
            last_sync_time: SystemTime::now(),
        }
    }
    
    pub fn time_based(min_interval: Duration) -> Self {
        Self {
            strategy_type: StrategyType::TimeBased { min_interval },
            last_sync_time: SystemTime::now() - Duration::from_secs(100),
        }
    }
    
    pub fn should_sync(&mut self, content: &str) -> SyncDecision {
        match &self.strategy_type {
            StrategyType::Immediate => {
                self.last_sync_time = SystemTime::now();
                SyncDecision::SyncImmediately
            }
            StrategyType::LengthBased { max_length } => {
                if content.len() > *max_length {
                    SyncDecision::Skip
                } else {
                    self.last_sync_time = SystemTime::now();
                    SyncDecision::SyncImmediately
                }
            }
            StrategyType::TimeBased { min_interval } => {
                if SystemTime::now().duration_since(self.last_sync_time).unwrap_or(Duration::ZERO) < *min_interval {
                    SyncDecision::Skip
                } else {
                    self.last_sync_time = SystemTime::now();
                    SyncDecision::SyncImmediately
                }
            }
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SyncDecision {
    SyncImmediately,
    Skip,
    Delay(Duration),
    Defer { reason: String, retry_after: Duration },
}

#[derive(Debug, Clone)]
pub struct SyncStrategy {
    config: SyncConfig,
    state: StrategyState,
    metrics: SyncMetrics,
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub strategy_type: StrategyType,
    pub max_content_length: usize,
    pub min_sync_interval: Duration,
    pub max_sync_interval: Duration,
    pub network_aware: bool,
    pub battery_aware: bool,
    pub content_filters: Vec<ContentFilter>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrategyType {
    Immediate,
    LengthBased,
    TimeBased,
    NetworkBased,
    BatteryBased,
    Adaptive, // 智能自适应
}

#[derive(Debug, Clone)]
pub enum ContentFilter {
    Length { min: usize, max: usize },
    Regex { pattern: String, exclude: bool },
    ContentType { allowed_types: Vec<String> },
    Keywords { words: Vec<String>, exclude: bool },
}

#[derive(Debug)]
pub struct StrategyState {
    last_sync_time: SystemTime,
    sync_count: u32,
    skip_count: u32,
    consecutive_skips: u32,
    network_status: NetworkStatus,
    battery_status: BatteryStatus,
}

#[derive(Debug, Clone)]
pub enum NetworkStatus {
    Connected { strength: f32, type_: NetworkType },
    Disconnected,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum NetworkType {
    WiFi,
    Cellular,
    Ethernet,
    Bluetooth,
}

#[derive(Debug, Clone)]
pub enum BatteryStatus {
    Good { level: f32 },
    Low { level: f32 },
    Critical { level: f32 },
    Charging { level: f32 },
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SyncMetrics {
    pub total_sync_decisions: u32,
    pub sync_count: u32,
    pub skip_count: u32,
    pub average_sync_interval: Duration,
    pub last_decision_time: SystemTime,
}

impl SyncStrategy {
    // 重构后的代码，保持测试绿色
    pub fn immediate() -> Self {
        Self::with_config(SyncConfig {
            strategy_type: StrategyType::Immediate,
            max_content_length: 10000,
            min_sync_interval: Duration::ZERO,
            max_sync_interval: Duration::from_secs(3600),
            network_aware: false,
            battery_aware: false,
            content_filters: Vec::new(),
        })
    }
    
    pub fn length_based(max_length: usize) -> Self {
        Self::with_config(SyncConfig {
            strategy_type: StrategyType::LengthBased,
            max_content_length: max_length,
            min_sync_interval: Duration::ZERO,
            max_sync_interval: Duration::from_secs(3600),
            network_aware: false,
            battery_aware: false,
            content_filters: Vec::new(),
        })
    }
    
    pub fn time_based(min_interval: Duration) -> Self {
        Self::with_config(SyncConfig {
            strategy_type: StrategyType::TimeBased,
            max_content_length: 10000,
            min_sync_interval: min_interval,
            max_sync_interval: Duration::from_secs(3600),
            network_aware: false,
            battery_aware: false,
            content_filters: Vec::new(),
        })
    }
    
    pub fn with_config(config: SyncConfig) -> Self {
        Self {
            config,
            state: StrategyState::new(),
            metrics: SyncMetrics::new(),
        }
    }
    
    pub fn should_sync(&mut self, content: &str) -> SyncDecision {
        let now = SystemTime::now();
        self.metrics.last_decision_time = now;
        self.metrics.total_sync_decisions += 1;
        
        // 内容过滤检查
        if !self.passes_content_filters(content) {
            self.state.skip_count += 1;
            self.state.consecutive_skips += 1;
            self.metrics.skip_count += 1;
            return SyncDecision::Skip;
        }
        
        // 策略特定逻辑
        match self.config.strategy_type {
            StrategyType::Immediate => self.decide_immediate(content),
            StrategyType::LengthBased => self.decide_length_based(content),
            StrategyType::TimeBased => self.decide_time_based(content),
            StrategyType::NetworkBased => self.decide_network_based(content),
            StrategyType::BatteryBased => self.decide_battery_based(content),
            StrategyType::Adaptive => self.decide_adaptive(content),
        }
    }
    
    fn decide_immediate(&mut self, content: &str) -> SyncDecision {
        self.record_sync();
        SyncDecision::SyncImmediately
    }
    
    fn decide_length_based(&mut self, content: &str) -> SyncDecision {
        if content.len() > self.config.max_content_length {
            self.state.skip_count += 1;
            self.state.consecutive_skips += 1;
            self.metrics.skip_count += 1;
            SyncDecision::Skip
        } else {
            self.record_sync();
            SyncDecision::SyncImmediately
        }
    }
    
    fn decide_time_based(&mut self, content: &str) -> SyncDecision {
        let since_last = SystemTime::now()
            .duration_since(self.state.last_sync_time)
            .unwrap_or(Duration::ZERO);
        
        if since_last < self.config.min_sync_interval {
            self.state.skip_count += 1;
            self.state.consecutive_skips += 1;
            self.metrics.skip_count += 1;
            SyncDecision::Skip
        } else {
            self.record_sync();
            SyncDecision::SyncImmediately
        }
    }
    
    fn decide_network_based(&mut self, content: &str) -> SyncDecision {
        if let NetworkStatus::Connected { strength, type_ } = &self.state.network_status {
            match type_ {
                NetworkType::WiFi if *strength > 0.5 => {
                    self.record_sync();
                    SyncDecision::SyncImmediately
                }
                NetworkType::Bluetooth if *strength > 0.3 => {
                    self.record_sync();
                    SyncDecision::SyncImmediately
                }
                _ => {
                    SyncDecision::Defer {
                        reason: "Weak network connection".to_string(),
                        retry_after: Duration::from_secs(30),
                    }
                }
            }
        } else {
            SyncDecision::Defer {
                reason: "No network connection".to_string(),
                retry_after: Duration::from_secs(60),
            }
        }
    }
    
    fn decide_battery_based(&mut self, content: &str) -> SyncDecision {
        match &self.state.battery_status {
            BatteryStatus::Good { .. } | BatteryStatus::Charging { .. } => {
                self.record_sync();
                SyncDecision::SyncImmediately
            }
            BatteryStatus::Low { level } => {
                if content.len() < 1000 { // 低电量只同步小内容
                    self.record_sync();
                    SyncDecision::SyncImmediately
                } else {
                    SyncDecision::Defer {
                        reason: format!("Low battery ({}%)", (level * 100.0) as u32),
                        retry_after: Duration::from_secs(300),
                    }
                }
            }
            BatteryStatus::Critical { .. } => {
                SyncDecision::Skip
            }
            BatteryStatus::Unknown => {
                self.record_sync();
                SyncDecision::SyncImmediately
            }
        }
    }
    
    fn decide_adaptive(&mut self, content: &str) -> SyncDecision {
        // 智能决策，结合多种因素
        let mut score = 0;
        
        // 基于时间
        let since_last = SystemTime::now()
            .duration_since(self.state.last_sync_time)
            .unwrap_or(Duration::ZERO);
        if since_last > Duration::from_secs(30) {
            score += 20;
        }
        
        // 基于内容长度
        if content.len() < 500 {
            score += 30;
        } else if content.len() > 2000 {
            score -= 10;
        }
        
        // 基于连续跳过次数
        if self.state.consecutive_skips > 5 {
            score += 40; // 强制同步避免饿死
        }
        
        // 基于网络状态
        if let NetworkStatus::Connected { strength, .. } = &self.state.network_status {
            score += (*strength * 20.0) as i32;
        }
        
        // 基于电量状态
        if let BatteryStatus::Good { .. } | BatteryStatus::Charging { .. } = &self.state.battery_status {
            score += 10;
        }
        
        if score >= 50 {
            self.record_sync();
            SyncDecision::SyncImmediately
        } else {
            self.state.skip_count += 1;
            self.state.consecutive_skips += 1;
            self.metrics.skip_count += 1;
            SyncDecision::Defer {
                reason: format!("Adaptive score too low: {}", score),
                retry_after: Duration::from_secs(10),
            }
        }
    }
    
    fn passes_content_filters(&self, content: &str) -> bool {
        for filter in &self.config.content_filters {
            if !self.passes_filter(content, filter) {
                return false;
            }
        }
        true
    }
    
    fn passes_filter(&self, content: &str, filter: &ContentFilter) -> bool {
        match filter {
            ContentFilter::Length { min, max } => {
                content.len() >= *min && content.len() <= *max
            }
            ContentFilter::Regex { pattern, exclude } => {
                use regex::Regex;
                if let Ok(re) = Regex::new(pattern) {
                    let matches = re.is_match(content);
                    if *exclude { !matches } else { matches }
                } else {
                    true // 无效的正则表达式跳过过滤
                }
            }
            ContentFilter::ContentType { allowed_types } => {
                // 简化实现，假设所有内容都是文本
                allowed_types.contains(&"text".to_string())
            }
            ContentFilter::Keywords { words, exclude } => {
                let content_lower = content.to_lowercase();
                let matches = words.iter().any(|word| content_lower.contains(&word.to_lowercase()));
                if *exclude { !matches } else { matches }
            }
        }
    }
    
    fn record_sync(&mut self) {
        self.state.last_sync_time = SystemTime::now();
        self.state.sync_count += 1;
        self.state.consecutive_skips = 0;
        self.metrics.sync_count += 1;
        
        // 更新平均同步间隔
        let total_interval = self.metrics.average_sync_interval * (self.metrics.sync_count - 1) as u32;
        let new_interval = SystemTime::now()
            .duration_since(self.state.last_sync_time)
            .unwrap_or(Duration::ZERO);
        self.metrics.average_sync_interval = (total_interval + new_interval) / self.metrics.sync_count as u32;
    }
    
    pub fn update_network_status(&mut self, status: NetworkStatus) {
        self.state.network_status = status;
    }
    
    pub fn update_battery_status(&mut self, status: BatteryStatus) {
        self.state.battery_status = status;
    }
    
    pub fn metrics(&self) -> &SyncMetrics {
        &self.metrics
    }
    
    pub fn reset_metrics(&mut self) {
        self.metrics = SyncMetrics::new();
    }
}

impl StrategyState {
    fn new() -> Self {
        Self {
            last_sync_time: SystemTime::now() - Duration::from_secs(100), // 允许立即同步
            sync_count: 0,
            skip_count: 0,
            consecutive_skips: 0,
            network_status: NetworkStatus::Unknown,
            battery_status: BatteryStatus::Unknown,
        }
    }
}

impl SyncMetrics {
    fn new() -> Self {
        Self {
            total_sync_decisions: 0,
            sync_count: 0,
            skip_count: 0,
            average_sync_interval: Duration::ZERO,
            last_decision_time: SystemTime::now(),
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的同步策略：

```rust
// rust-core/domain/sync/strategy.rs
pub struct SyncStrategy {
    // 同步策略实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [剪贴板监控器](0401-clipboard-monitoring.md)

## 后续任务

- [Task 0403: 实现内容变化检测](0403-content-change-detection.md)
- [Task 0404: 实现同步冲突处理](0404-sync-conflict-handling.md)
- [Task 0405: 实现同步历史记录](0405-sync-history.md)