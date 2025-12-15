//! NearClip 核心管理器模块
//!
//! 提供统一的 API 管理所有同步功能，协调底层模块。
//!
//! # 示例
//!
//! ```
//! use nearclip_core::{
//!     NearClipManager, NearClipConfig, NearClipCallback,
//!     DeviceInfo, NearClipError,
//! };
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl NearClipCallback for MyCallback {
//!     fn on_device_connected(&self, device: &DeviceInfo) {
//!         println!("Device connected: {}", device.name());
//!     }
//!     fn on_device_disconnected(&self, device_id: &str) {
//!         println!("Device disconnected: {}", device_id);
//!     }
//!     fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
//!         println!("Received {} bytes from {}", content.len(), from_device);
//!     }
//!     fn on_sync_error(&self, error: &NearClipError) {
//!         eprintln!("Sync error: {}", error);
//!     }
//! }
//!
//! let config = NearClipConfig::new("My Device");
//! let callback = Arc::new(MyCallback);
//! let manager = NearClipManager::new(config, callback).unwrap();
//!
//! assert!(!manager.is_running());
//! ```

use crate::config::NearClipConfig;
use crate::device::{DeviceInfo, DeviceStatus};
use crate::error::{NearClipError, Result};
use nearclip_sync::Channel;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

// ============================================================
// NearClipCallback - 回调接口
// ============================================================

/// NearClip 回调接口
///
/// 平台客户端实现此 trait 以接收事件通知。
///
/// # 示例
///
/// ```
/// use nearclip_core::{NearClipCallback, DeviceInfo, NearClipError};
///
/// struct LoggingCallback;
///
/// impl NearClipCallback for LoggingCallback {
///     fn on_device_connected(&self, device: &DeviceInfo) {
///         println!("Connected: {}", device.name());
///     }
///     fn on_device_disconnected(&self, device_id: &str) {
///         println!("Disconnected: {}", device_id);
///     }
///     fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
///         println!("Clipboard from {}: {} bytes", from_device, content.len());
///     }
///     fn on_sync_error(&self, error: &NearClipError) {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
pub trait NearClipCallback: Send + Sync {
    /// 设备连接成功时调用
    fn on_device_connected(&self, device: &DeviceInfo);

    /// 设备断开连接时调用
    fn on_device_disconnected(&self, device_id: &str);

    /// 收到剪贴板内容时调用
    fn on_clipboard_received(&self, content: &[u8], from_device: &str);

    /// 发生同步错误时调用
    fn on_sync_error(&self, error: &NearClipError);
}

// ============================================================
// NoOpCallback - 空回调实现
// ============================================================

/// 空回调实现
///
/// 不执行任何操作的回调，用于测试或不需要回调的场景。
#[derive(Debug, Default)]
pub struct NoOpCallback;

impl NearClipCallback for NoOpCallback {
    fn on_device_connected(&self, _device: &DeviceInfo) {}
    fn on_device_disconnected(&self, _device_id: &str) {}
    fn on_clipboard_received(&self, _content: &[u8], _from_device: &str) {}
    fn on_sync_error(&self, _error: &NearClipError) {}
}

// ============================================================
// ManagerState - 内部状态
// ============================================================

/// 管理器内部状态
#[derive(Default)]
struct ManagerState {
    /// 已配对设备列表
    paired_devices: HashMap<String, DeviceInfo>,
    /// 当前使用的通道
    current_channel: Option<Channel>,
}

// ============================================================
// NearClipManager - 核心管理器
// ============================================================

/// NearClip 核心管理器
///
/// 提供统一的 API 管理所有同步功能。
///
/// # 生命周期
///
/// 1. 创建 `NearClipManager::new(config, callback)`
/// 2. 启动 `manager.start().await`
/// 3. 同步剪贴板 `manager.sync_clipboard(content).await`
/// 4. 停止 `manager.stop().await`
///
/// # 示例
///
/// ```
/// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
/// use std::sync::Arc;
///
/// let config = NearClipConfig::new("Test Device");
/// let callback = Arc::new(NoOpCallback);
/// let manager = NearClipManager::new(config, callback).unwrap();
///
/// assert!(!manager.is_running());
/// assert_eq!(manager.get_connected_devices().len(), 0);
/// ```
pub struct NearClipManager {
    /// 配置
    config: NearClipConfig,
    /// 回调
    callback: Arc<dyn NearClipCallback>,
    /// 运行状态
    running: AtomicBool,
    /// 内部状态
    state: RwLock<ManagerState>,
}

impl NearClipManager {
    /// 创建新的管理器实例
    ///
    /// # 参数
    ///
    /// * `config` - 配置
    /// * `callback` - 回调实现
    ///
    /// # 错误
    ///
    /// 如果配置验证失败，返回错误。
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
    /// use std::sync::Arc;
    ///
    /// let config = NearClipConfig::new("Device");
    /// let callback = Arc::new(NoOpCallback);
    /// let manager = NearClipManager::new(config, callback);
    /// assert!(manager.is_ok());
    /// ```
    pub fn new(config: NearClipConfig, callback: Arc<dyn NearClipCallback>) -> Result<Self> {
        config.validate()?;

        tracing::info!(
            device_name = config.device_name(),
            wifi = config.wifi_enabled(),
            ble = config.ble_enabled(),
            "Creating NearClipManager"
        );

        Ok(Self {
            config,
            callback,
            running: AtomicBool::new(false),
            state: RwLock::new(ManagerState::default()),
        })
    }

    /// 启动服务
    ///
    /// 启动 mDNS 广播、TCP 服务器、BLE 广播等。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
    /// use std::sync::Arc;
    ///
    /// # async fn example() {
    /// let config = NearClipConfig::new("Device");
    /// let callback = Arc::new(NoOpCallback);
    /// let manager = NearClipManager::new(config, callback).unwrap();
    ///
    /// manager.start().await.unwrap();
    /// assert!(manager.is_running());
    ///
    /// manager.stop().await;
    /// assert!(!manager.is_running());
    /// # }
    /// ```
    pub async fn start(&self) -> Result<()> {
        if self.running.load(Ordering::Acquire) {
            tracing::warn!("Manager already running");
            return Ok(());
        }

        tracing::info!("Starting NearClipManager");

        // TODO: 在实际实现中启动以下服务:
        // 1. mDNS 服务广播 (如果 WiFi 启用)
        // 2. TCP 服务器 (如果 WiFi 启用)
        // 3. BLE 广播 (如果 BLE 启用)
        // 4. 通道监控
        // 5. 设备发现

        self.running.store(true, Ordering::Release);

        // 设置初始通道
        {
            let mut state = self.state.write().unwrap();
            if self.config.wifi_enabled() {
                state.current_channel = Some(Channel::Wifi);
            } else if self.config.ble_enabled() {
                state.current_channel = Some(Channel::Ble);
            }
        }

        tracing::info!("NearClipManager started");
        Ok(())
    }

    /// 停止服务
    ///
    /// 停止所有后台服务，断开所有连接。
    pub async fn stop(&self) {
        if !self.running.load(Ordering::Acquire) {
            tracing::warn!("Manager not running");
            return;
        }

        tracing::info!("Stopping NearClipManager");

        // TODO: 在实际实现中停止以下服务:
        // 1. 停止 mDNS 广播
        // 2. 关闭 TCP 服务器
        // 3. 停止 BLE 广播
        // 4. 断开所有设备连接
        // 5. 停止通道监控

        // 断开所有设备
        {
            let mut state = self.state.write().unwrap();
            for (device_id, device) in state.paired_devices.iter_mut() {
                if device.status().is_connected() {
                    device.set_status(DeviceStatus::Disconnected);
                    self.callback.on_device_disconnected(device_id);
                }
            }
            state.current_channel = None;
        }

        self.running.store(false, Ordering::Release);

        tracing::info!("NearClipManager stopped");
    }

    /// 同步剪贴板内容
    ///
    /// 将剪贴板内容发送到所有已连接设备。
    ///
    /// # 参数
    ///
    /// * `content` - 剪贴板内容
    ///
    /// # 错误
    ///
    /// - 管理器未运行
    /// - 没有可用通道
    /// - 发送失败
    pub async fn sync_clipboard(&self, content: &[u8]) -> Result<()> {
        if !self.running.load(Ordering::Acquire) {
            return Err(NearClipError::Sync("Manager not running".to_string()));
        }

        let state = self.state.read().unwrap();
        let channel = state.current_channel.ok_or_else(|| {
            NearClipError::Sync("No channel available".to_string())
        })?;

        let connected_devices: Vec<String> = state
            .paired_devices
            .iter()
            .filter(|(_, d)| d.status().is_connected())
            .map(|(id, _)| id.clone())
            .collect();
        drop(state);

        if connected_devices.is_empty() {
            tracing::debug!("No connected devices, skipping sync");
            return Ok(());
        }

        tracing::info!(
            content_size = content.len(),
            channel = ?channel,
            device_count = connected_devices.len(),
            "Syncing clipboard"
        );

        // TODO: 实际发送逻辑
        // 1. 检查 loop_guard.should_sync(content)
        // 2. 通过选定通道发送到各设备
        // 3. 等待 ACK 或处理重试

        Ok(())
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    /// 获取配置
    pub fn config(&self) -> &NearClipConfig {
        &self.config
    }

    /// 获取当前通道
    pub fn current_channel(&self) -> Option<Channel> {
        self.state.read().unwrap().current_channel
    }

    // --------------------------------------------------------
    // 设备管理
    // --------------------------------------------------------

    /// 获取已配对设备列表
    pub fn get_paired_devices(&self) -> Vec<DeviceInfo> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .values()
            .cloned()
            .collect()
    }

    /// 获取已连接设备列表
    pub fn get_connected_devices(&self) -> Vec<DeviceInfo> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .values()
            .filter(|d| d.status().is_connected())
            .cloned()
            .collect()
    }

    /// 获取设备状态
    pub fn get_device_status(&self, device_id: &str) -> Option<DeviceStatus> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .get(device_id)
            .map(|d| d.status())
    }

    /// 添加已配对设备
    ///
    /// 内部方法，用于添加新配对的设备。
    pub fn add_paired_device(&self, device: DeviceInfo) {
        let device_id = device.id().to_string();
        tracing::info!(device_id = %device_id, "Adding paired device");

        self.state
            .write()
            .unwrap()
            .paired_devices
            .insert(device_id, device);
    }

    /// 移除已配对设备
    pub fn remove_paired_device(&self, device_id: &str) -> Option<DeviceInfo> {
        tracing::info!(device_id = %device_id, "Removing paired device");

        self.state
            .write()
            .unwrap()
            .paired_devices
            .remove(device_id)
    }

    /// 连接设备
    ///
    /// 尝试连接到指定设备。
    ///
    /// # 参数
    ///
    /// * `device_id` - 设备 ID
    ///
    /// # 错误
    ///
    /// - 管理器未运行
    /// - 设备未找到
    /// - 连接失败
    pub async fn connect_device(&self, device_id: &str) -> Result<()> {
        if !self.running.load(Ordering::Acquire) {
            return Err(NearClipError::Sync("Manager not running".to_string()));
        }

        {
            let state = self.state.read().unwrap();
            if !state.paired_devices.contains_key(device_id) {
                return Err(NearClipError::DeviceNotFound(device_id.to_string()));
            }
        }

        tracing::info!(device_id = %device_id, "Connecting to device");

        // 设置为连接中
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Connecting);
            }
        }

        // TODO: 实际连接逻辑
        // 1. 根据当前通道尝试连接
        // 2. 建立 TLS 连接
        // 3. 验证设备身份

        // 模拟连接成功
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Connected);
                self.callback.on_device_connected(device);
            }
        }

        Ok(())
    }

    /// 断开设备连接
    ///
    /// # 参数
    ///
    /// * `device_id` - 设备 ID
    pub async fn disconnect_device(&self, device_id: &str) -> Result<()> {
        {
            let state = self.state.read().unwrap();
            if !state.paired_devices.contains_key(device_id) {
                return Err(NearClipError::DeviceNotFound(device_id.to_string()));
            }
        }

        tracing::info!(device_id = %device_id, "Disconnecting device");

        // TODO: 实际断开逻辑
        // 1. 关闭连接
        // 2. 清理资源

        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Disconnected);
            }
        }

        self.callback.on_device_disconnected(device_id);

        Ok(())
    }

    // --------------------------------------------------------
    // 内部方法 - 用于底层模块调用
    // --------------------------------------------------------

    /// 处理收到的剪贴板内容
    ///
    /// 由底层模块调用，当收到远程剪贴板内容时。
    pub fn handle_clipboard_received(&self, content: &[u8], from_device: &str) {
        tracing::debug!(
            content_size = content.len(),
            from_device = %from_device,
            "Received clipboard content"
        );

        // 更新设备活动时间
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(from_device) {
                device.touch();
            }
        }

        self.callback.on_clipboard_received(content, from_device);
    }

    /// 处理同步错误
    ///
    /// 由底层模块调用，当发生错误时。
    pub fn handle_sync_error(&self, error: NearClipError) {
        tracing::error!(error = %error, "Sync error occurred");
        self.callback.on_sync_error(&error);
    }

    /// 处理设备连接
    ///
    /// 由底层模块调用，当设备连接成功时。
    pub fn handle_device_connected(&self, device: DeviceInfo) {
        let device_id = device.id().to_string();
        tracing::info!(device_id = %device_id, "Device connected");

        {
            let mut state = self.state.write().unwrap();
            state.paired_devices.insert(device_id, device.clone());
        }

        self.callback.on_device_connected(&device);
    }

    /// 处理设备断开
    ///
    /// 由底层模块调用，当设备断开连接时。
    pub fn handle_device_disconnected(&self, device_id: &str) {
        tracing::info!(device_id = %device_id, "Device disconnected");

        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Disconnected);
            }
        }

        self.callback.on_device_disconnected(device_id);
    }
}

impl std::fmt::Debug for NearClipManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NearClipManager")
            .field("config", &self.config)
            .field("running", &self.is_running())
            .field("paired_devices", &self.get_paired_devices().len())
            .field("connected_devices", &self.get_connected_devices().len())
            .finish()
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::DevicePlatform;
    use std::sync::Mutex;

    // 测试回调，记录调用
    struct TestCallback {
        connected: Mutex<Vec<String>>,
        disconnected: Mutex<Vec<String>>,
        clipboard: Mutex<Vec<(Vec<u8>, String)>>,
        errors: Mutex<Vec<String>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                connected: Mutex::new(Vec::new()),
                disconnected: Mutex::new(Vec::new()),
                clipboard: Mutex::new(Vec::new()),
                errors: Mutex::new(Vec::new()),
            }
        }

        fn connected_count(&self) -> usize {
            self.connected.lock().unwrap().len()
        }

        fn disconnected_count(&self) -> usize {
            self.disconnected.lock().unwrap().len()
        }

        fn clipboard_count(&self) -> usize {
            self.clipboard.lock().unwrap().len()
        }

        fn error_count(&self) -> usize {
            self.errors.lock().unwrap().len()
        }
    }

    impl NearClipCallback for TestCallback {
        fn on_device_connected(&self, device: &DeviceInfo) {
            self.connected.lock().unwrap().push(device.id().to_string());
        }

        fn on_device_disconnected(&self, device_id: &str) {
            self.disconnected.lock().unwrap().push(device_id.to_string());
        }

        fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
            self.clipboard
                .lock()
                .unwrap()
                .push((content.to_vec(), from_device.to_string()));
        }

        fn on_sync_error(&self, error: &NearClipError) {
            self.errors.lock().unwrap().push(error.to_string());
        }
    }

    fn create_manager() -> NearClipManager {
        let config = NearClipConfig::new("Test Device");
        let callback = Arc::new(NoOpCallback);
        NearClipManager::new(config, callback).unwrap()
    }

    fn create_manager_with_callback() -> (NearClipManager, Arc<TestCallback>) {
        let config = NearClipConfig::new("Test Device");
        let callback = Arc::new(TestCallback::new());
        let manager = NearClipManager::new(config, callback.clone()).unwrap();
        (manager, callback)
    }

    // --------------------------------------------------------
    // 创建测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_new() {
        let manager = create_manager();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_manager_new_invalid_config() {
        let config = NearClipConfig::new(""); // 无效名称
        let callback = Arc::new(NoOpCallback);
        let result = NearClipManager::new(config, callback);
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_config() {
        let manager = create_manager();
        assert_eq!(manager.config().device_name(), "Test Device");
    }

    // --------------------------------------------------------
    // 生命周期测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_start_stop() {
        let manager = create_manager();

        assert!(!manager.is_running());

        manager.start().await.unwrap();
        assert!(manager.is_running());

        manager.stop().await;
        assert!(!manager.is_running());
    }

    #[tokio::test]
    async fn test_manager_start_twice() {
        let manager = create_manager();

        manager.start().await.unwrap();
        // 第二次启动不应该失败
        manager.start().await.unwrap();

        assert!(manager.is_running());
    }

    #[tokio::test]
    async fn test_manager_stop_when_not_running() {
        let manager = create_manager();
        // 未启动时停止不应该失败
        manager.stop().await;
        assert!(!manager.is_running());
    }

    // --------------------------------------------------------
    // 通道测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_current_channel_wifi() {
        let config = NearClipConfig::new("Test")
            .with_wifi_enabled(true)
            .with_ble_enabled(false);
        let callback = Arc::new(NoOpCallback);
        let manager = NearClipManager::new(config, callback).unwrap();

        manager.start().await.unwrap();
        assert_eq!(manager.current_channel(), Some(Channel::Wifi));
    }

    #[tokio::test]
    async fn test_manager_current_channel_ble() {
        let config = NearClipConfig::new("Test")
            .with_wifi_enabled(false)
            .with_ble_enabled(true);
        let callback = Arc::new(NoOpCallback);
        let manager = NearClipManager::new(config, callback).unwrap();

        manager.start().await.unwrap();
        assert_eq!(manager.current_channel(), Some(Channel::Ble));
    }

    #[test]
    fn test_manager_current_channel_not_running() {
        let manager = create_manager();
        assert_eq!(manager.current_channel(), None);
    }

    // --------------------------------------------------------
    // 设备管理测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_add_paired_device() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_platform(DevicePlatform::MacOS);
        manager.add_paired_device(device);

        assert_eq!(manager.get_paired_devices().len(), 1);
    }

    #[test]
    fn test_manager_remove_paired_device() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);
        assert_eq!(manager.get_paired_devices().len(), 1);

        let removed = manager.remove_paired_device("d1");
        assert!(removed.is_some());
        assert_eq!(manager.get_paired_devices().len(), 0);
    }

    #[test]
    fn test_manager_get_device_status() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Connected)
        );
        assert_eq!(manager.get_device_status("unknown"), None);
    }

    #[test]
    fn test_manager_get_connected_devices() {
        let manager = create_manager();

        let d1 = DeviceInfo::new("d1", "D1").with_status(DeviceStatus::Connected);
        let d2 = DeviceInfo::new("d2", "D2").with_status(DeviceStatus::Disconnected);
        let d3 = DeviceInfo::new("d3", "D3").with_status(DeviceStatus::Connected);

        manager.add_paired_device(d1);
        manager.add_paired_device(d2);
        manager.add_paired_device(d3);

        assert_eq!(manager.get_paired_devices().len(), 3);
        assert_eq!(manager.get_connected_devices().len(), 2);
    }

    // --------------------------------------------------------
    // 连接/断开测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_connect_device() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        manager.start().await.unwrap();

        manager.connect_device("d1").await.unwrap();

        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Connected)
        );
        assert_eq!(callback.connected_count(), 1);
    }

    #[tokio::test]
    async fn test_manager_connect_device_not_found() {
        let manager = create_manager();
        manager.start().await.unwrap();

        let result = manager.connect_device("unknown").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(NearClipError::DeviceNotFound(_))));
    }

    #[tokio::test]
    async fn test_manager_connect_device_not_running() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        let result = manager.connect_device("d1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_disconnect_device() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.disconnect_device("d1").await.unwrap();

        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Disconnected)
        );
        assert_eq!(callback.disconnected_count(), 1);
    }

    #[tokio::test]
    async fn test_manager_disconnect_device_not_found() {
        let manager = create_manager();

        let result = manager.disconnect_device("unknown").await;
        assert!(result.is_err());
    }

    // --------------------------------------------------------
    // 同步测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_sync_clipboard_not_running() {
        let manager = create_manager();

        let result = manager.sync_clipboard(b"test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_sync_clipboard_no_connected_devices() {
        let manager = create_manager();
        manager.start().await.unwrap();

        // 没有连接设备，应该成功但不做任何事
        let result = manager.sync_clipboard(b"test").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manager_sync_clipboard_with_devices() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.start().await.unwrap();

        let result = manager.sync_clipboard(b"test content").await;
        assert!(result.is_ok());
    }

    // --------------------------------------------------------
    // 回调测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_handle_clipboard_received() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        manager.handle_clipboard_received(b"hello", "d1");

        assert_eq!(callback.clipboard_count(), 1);
    }

    #[test]
    fn test_manager_handle_sync_error() {
        let (manager, callback) = create_manager_with_callback();

        manager.handle_sync_error(NearClipError::Network("test".to_string()));

        assert_eq!(callback.error_count(), 1);
    }

    #[test]
    fn test_manager_handle_device_connected() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.handle_device_connected(device);

        assert_eq!(callback.connected_count(), 1);
        assert_eq!(manager.get_paired_devices().len(), 1);
    }

    #[test]
    fn test_manager_handle_device_disconnected() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.handle_device_disconnected("d1");

        assert_eq!(callback.disconnected_count(), 1);
        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Disconnected)
        );
    }

    // --------------------------------------------------------
    // NoOpCallback 测试
    // --------------------------------------------------------

    #[test]
    fn test_noop_callback() {
        let callback = NoOpCallback;
        let device = DeviceInfo::new("d1", "D1");
        let error = NearClipError::Network("test".to_string());

        // 这些不应该 panic
        callback.on_device_connected(&device);
        callback.on_device_disconnected("d1");
        callback.on_clipboard_received(b"test", "d1");
        callback.on_sync_error(&error);
    }

    #[test]
    fn test_noop_callback_debug() {
        let callback = NoOpCallback;
        let debug = format!("{:?}", callback);
        assert!(debug.contains("NoOpCallback"));
    }

    // --------------------------------------------------------
    // Debug 测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_debug() {
        let manager = create_manager();
        let debug = format!("{:?}", manager);
        assert!(debug.contains("NearClipManager"));
        assert!(debug.contains("running"));
    }
}
