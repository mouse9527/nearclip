# Task 0401: 实现剪贴板监控器 (TDD版本)

## 任务描述

按照TDD原则实现剪贴板监控器，检测系统剪贴板内容变化。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/clipboard_monitor_tests.rs
#[cfg(test)]
mod clipboard_monitor_tests {
    use super::*;
    
    #[test]
    fn test_clipboard_change_detection() {
        // RED: 测试剪贴板变化检测
        let monitor = ClipboardMonitor::new();
        let content = "Hello, World!";
        
        // 模拟剪贴板变化
        monitor.simulate_change(content);
        
        assert_eq!(monitor.current_content(), content);
        assert!(monitor.has_changes());
    }
    
    #[test]
    fn test_clipboard_content_filtering() {
        // RED: 测试剪贴板内容过滤
        let monitor = ClipboardMonitor::new().with_filter(|content| {
            content.len() <= 1000 // 只接受小于1000字符的内容
        });
        
        // 有效内容
        monitor.simulate_change("Short text");
        assert_eq!(monitor.current_content(), "Short text");
        
        // 无效内容（过长）
        monitor.simulate_change("a".repeat(2000));
        assert_eq!(monitor.current_content(), "Short text"); // 内容不变
    }
    
    #[test]
    fn test_duplicate_change_detection() {
        // RED: 测试重复变化检测
        let mut monitor = ClipboardMonitor::new();
        
        monitor.simulate_change("Same content");
        assert!(monitor.has_changes());
        
        // 再次设置相同内容
        monitor.simulate_change("Same content");
        assert!(!monitor.has_changes()); // 不应该检测为变化
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
pub struct ClipboardMonitor {
    current_content: String,
    last_content: String,
    filter: Option<Box<dyn Fn(&str) -> bool>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            current_content: String::new(),
            last_content: String::new(),
            filter: None,
        }
    }
    
    pub fn with_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&str) -> bool + 'static,
    {
        self.filter = Some(Box::new(filter));
        self
    }
    
    pub fn current_content(&self) -> &str {
        &self.current_content
    }
    
    pub fn has_changes(&self) -> bool {
        self.current_content != self.last_content
    }
    
    pub fn simulate_change(&mut self, content: &str) {
        self.last_content = self.current_content.clone();
        
        if let Some(ref filter) = self.filter {
            if filter(content) {
                self.current_content = content.to_string();
            }
        } else {
            self.current_content = content.to_string();
        }
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::time::{SystemTime, Duration};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub text: String,
    pub content_type: ClipboardContentType,
    pub timestamp: SystemTime,
    pub source_app: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardContentType {
    Text,
    Image,
    File,
    Other(String),
}

#[derive(Debug)]
pub struct ClipboardConfig {
    pub max_history_size: usize,
    pub debounce_duration: Duration,
    pub min_content_length: usize,
    pub max_content_length: usize,
    pub ignore_empty_content: bool,
    pub enabled_content_types: Vec<ClipboardContentType>,
}

#[derive(Debug)]
pub struct ClipboardMonitor {
    current_content: Option<ClipboardContent>,
    history: VecDeque<ClipboardContent>,
    config: ClipboardConfig,
    last_check_time: SystemTime,
    filters: Vec<Box<dyn Fn(&ClipboardContent) -> bool>>,
    subscribers: Vec<Box<dyn Fn(&ClipboardContent)>>,
}

impl ClipboardMonitor {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_config(ClipboardConfig::default())
    }
    
    pub fn with_config(config: ClipboardConfig) -> Self {
        Self {
            current_content: None,
            history: VecDeque::with_capacity(config.max_history_size),
            config,
            last_check_time: SystemTime::now(),
            filters: Vec::new(),
            subscribers: Vec::new(),
        }
    }
    
    pub fn current_content(&self) -> Option<&ClipboardContent> {
        self.current_content.as_ref()
    }
    
    pub fn history(&self) -> &VecDeque<ClipboardContent> {
        &self.history
    }
    
    pub fn has_changes(&self) -> bool {
        if let Some(ref current) = self.current_content {
            if let Some(back) = self.history.back() {
                current.text != back.text
            } else {
                true
            }
        } else {
            false
        }
    }
    
    pub fn simulate_change(&mut self, content: &str) {
        let new_content = ClipboardContent {
            text: content.to_string(),
            content_type: ClipboardContentType::Text,
            timestamp: SystemTime::now(),
            source_app: None,
        };
        
        if self.should_process_content(&new_content) {
            self.process_new_content(new_content);
        }
    }
    
    fn should_process_content(&self, content: &ClipboardContent) -> bool {
        // 长度检查
        if content.text.len() < self.config.min_content_length {
            return false;
        }
        
        if content.text.len() > self.config.max_content_length {
            return false;
        }
        
        // 空内容检查
        if self.config.ignore_empty_content && content.text.trim().is_empty() {
            return false;
        }
        
        // 内容类型检查
        if !self.config.enabled_content_types.contains(&content.content_type) {
            return false;
        }
        
        // 自定义过滤器
        self.filters.iter().all(|filter| filter(content))
    }
    
    fn process_new_content(&mut self, content: ClipboardContent) {
        // 更新历史记录
        if self.history.len() >= self.config.max_history_size {
            self.history.pop_front();
        }
        
        // 如果有当前内容，添加到历史
        if let Some(current) = self.current_content.take() {
            self.history.push_back(current);
        }
        
        self.current_content = Some(content.clone());
        
        // 通知订阅者
        for subscriber in &self.subscribers {
            subscriber(&content);
        }
    }
    
    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&ClipboardContent) -> bool + 'static,
    {
        self.filters.push(Box::new(filter));
    }
    
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&ClipboardContent) + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    
    pub fn get_recent_changes(&self, since: SystemTime) -> Vec<&ClipboardContent> {
        self.history
            .iter()
            .filter(|content| content.timestamp > since)
            .collect()
    }
    
    pub fn get_content_by_type(&self, content_type: &ClipboardContentType) -> Vec<&ClipboardContent> {
        self.history
            .iter()
            .filter(|content| &content.content_type == content_type)
            .collect()
    }
    
    pub fn stats(&self) -> ClipboardStats {
        ClipboardStats {
            total_changes: self.history.len(),
            last_change_time: self.current_content.as_ref().map(|c| c.timestamp),
            content_type_counts: self.get_content_type_counts(),
        }
    }
    
    fn get_content_type_counts(&self) -> std::collections::HashMap<ClipboardContentType, usize> {
        let mut counts = std::collections::HashMap::new();
        for content in &self.history {
            *counts.entry(content.content_type.clone()).or_insert(0) += 1;
        }
        counts
    }
}

#[derive(Debug)]
pub struct ClipboardStats {
    pub total_changes: usize,
    pub last_change_time: Option<SystemTime>,
    pub content_type_counts: std::collections::HashMap<ClipboardContentType, usize>,
}

impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            max_history_size: 100,
            debounce_duration: Duration::from_millis(500),
            min_content_length: 1,
            max_content_length: 10000,
            ignore_empty_content: true,
            enabled_content_types: vec![ClipboardContentType::Text],
        }
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为infrastructure层的剪贴板监控：

```rust
// rust-core/infrastructure/clipboard/monitor.rs
pub struct ClipboardMonitor {
    // 剪贴板监控器实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [传输协议处理](0305-transfer-protocol-handling.md)

## 后续任务

- [Task 0402: 实现同步策略](0402-sync-strategy.md)
- [Task 0403: 实现内容变化检测](0403-content-change-detection.md)
- [Task 0404: 实现同步冲突处理](0404-sync-conflict-handling.md)