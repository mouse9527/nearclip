# Task 0801: 实现桌面端剪贴板监控基础 (TDD版本)

## 任务描述

按照TDD原则实现桌面端（Windows/macOS/Linux）的剪贴板监控基础功能。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/desktop_clipboard_tests.rs
#[cfg(test)]
mod desktop_clipboard_tests {
    use super::*;
    
    #[test]
    fn test_clipboard_initialization() {
        // RED: 测试剪贴板监控器初始化
        let monitor = DesktopClipboardMonitor::new();
        
        assert!(!monitor.is_monitoring());
        assert_eq!(monitor.clipboard_content(), None);
    }
    
    #[test]
    fn test_start_monitoring() {
        // RED: 测试开始监控
        let mut monitor = DesktopClipboardMonitor::new();
        
        let result = monitor.start_monitoring();
        assert!(result.is_ok());
        assert!(monitor.is_monitoring());
    }
    
    #[test]
    fn test_stop_monitoring() {
        // RED: 测试停止监控
        let mut monitor = DesktopClipboardMonitor::new();
        monitor.start_monitoring().unwrap();
        
        let result = monitor.stop_monitoring();
        assert!(result.is_ok());
        assert!(!monitor.is_monitoring());
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug)]
pub struct DesktopClipboardMonitor {
    is_monitoring: bool,
    clipboard_content: Option<String>,
}

impl DesktopClipboardMonitor {
    pub fn new() -> Self {
        Self {
            is_monitoring: false,
            clipboard_content: None,
        }
    }
    
    pub fn is_monitoring(&self) -> bool {
        self.is_monitoring
    }
    
    pub fn clipboard_content(&self) -> Option<&str> {
        self.clipboard_content.as_deref()
    }
    
    pub fn start_monitoring(&mut self) -> Result<(), ClipboardError> {
        self.is_monitoring = true;
        Ok(())
    }
    
    pub fn stop_monitoring(&mut self) -> Result<(), ClipboardError> {
        self.is_monitoring = false;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ClipboardError {
    MonitoringFailed(String),
    AccessDenied,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct DesktopClipboardMonitor {
    is_monitoring: bool,
    clipboard_content: Option<ClipboardContent>,
    monitor_handle: Option<MonitorHandle>,
    content_format: ClipboardFormat,
    change_count: u32,
}

#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub text: Option<String>,
    pub html: Option<String>,
    pub rtf: Option<String>,
    pub files: Vec<String>,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardFormat {
    Text,
    Html,
    Rtf,
    Files,
}

impl DesktopClipboardMonitor {
    pub fn new() -> Self {
        Self {
            is_monitoring: false,
            clipboard_content: None,
            monitor_handle: None,
            content_format: ClipboardFormat::Text,
            change_count: 0,
        }
    }
    
    pub fn with_format(format: ClipboardFormat) -> Self {
        Self {
            content_format: format,
            ..Self::new()
        }
    }
    
    pub fn start_monitoring(&mut self) -> Result<(), ClipboardError> {
        if self.is_monitoring {
            return Err(ClipboardError::AlreadyMonitoring);
        }
        
        #[cfg(target_os = "windows")]
        {
            self.monitor_handle = Some(self.start_windows_monitoring()?);
        }
        
        #[cfg(target_os = "macos")]
        {
            self.monitor_handle = Some(self.start_macos_monitoring()?);
        }
        
        #[cfg(target_os = "linux")]
        {
            self.monitor_handle = Some(self.start_linux_monitoring()?);
        }
        
        self.is_monitoring = true;
        Ok(())
    }
    
    pub fn stop_monitoring(&mut self) -> Result<(), ClipboardError> {
        if !self.is_monitoring {
            return Err(ClipboardError::NotMonitoring);
        }
        
        if let Some(handle) = self.monitor_handle.take() {
            self.stop_platform_monitoring(handle)?;
        }
        
        self.is_monitoring = false;
        Ok(())
    }
    
    pub fn get_current_content(&self) -> Option<&ClipboardContent> {
        self.clipboard_content.as_ref()
    }
    
    pub fn has_text_content(&self) -> bool {
        self.clipboard_content
            .as_ref()
            .map(|content| content.text.is_some())
            .unwrap_or(false)
    }
    
    pub fn get_text_content(&self) -> Option<&str> {
        self.clipboard_content
            .as_ref()
            .and_then(|content| content.text.as_deref())
    }
    
    pub fn content_changed_since(&self, count: u32) -> bool {
        self.change_count > count
    }
    
    fn start_windows_monitoring(&mut self) -> Result<MonitorHandle, ClipboardError> {
        // Windows特定实现
        Ok(MonitorHandle::Windows)
    }
    
    fn start_macos_monitoring(&mut self) -> Result<MonitorHandle, ClipboardError> {
        // macOS特定实现
        Ok(MonitorHandle::MacOS)
    }
    
    fn start_linux_monitoring(&mut self) -> Result<MonitorHandle, ClipboardError> {
        // Linux特定实现
        Ok(MonitorHandle::Linux)
    }
    
    fn stop_platform_monitoring(&mut self, handle: MonitorHandle) -> Result<(), ClipboardError> {
        match handle {
            MonitorHandle::Windows => self.stop_windows_monitoring(),
            MonitorHandle::MacOS => self.stop_macos_monitoring(),
            MonitorHandle::Linux => self.stop_linux_monitoring(),
        }
    }
    
    fn stop_windows_monitoring(&mut self) -> Result<(), ClipboardError> {
        Ok(())
    }
    
    fn stop_macos_monitoring(&mut self) -> Result<(), ClipboardError> {
        Ok(())
    }
    
    fn stop_linux_monitoring(&mut self) -> Result<(), ClipboardError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum MonitorHandle {
    Windows,
    MacOS,
    Linux,
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为桌面端的infrastructure层实现：

```rust
// rust-core/infrastructure/desktop/clipboard_monitor.rs
pub struct DesktopClipboardMonitor {
    // 桌面端剪贴板监控实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查
- [ ] 跨平台编译无错误

## 依赖任务

- [Task 0301: 定义传输方向枚举](0301-transfer-direction-enum.md)

## 后续任务

- [Task 0802: 实现Windows剪贴板监控](0802-windows-clipboard-monitor.md)
- [Task 0803: 实现macOS剪贴板监控](0803-macos-clipboard-monitor.md)
- [Task 0804: 实现Linux剪贴板监控](0804-linux-clipboard-monitor.md)