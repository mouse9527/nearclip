//! BLE 中心模式扫描
//!
//! 实现 BLE central 模式，用于扫描发现附近的 NearClip 设备。
//!
//! # 平台支持
//!
//! 当前实现提供统一的 API 接口。平台特定的 BLE 实现将通过 feature flags 启用：
//! - macOS: CoreBluetooth CBCentralManager (计划中)
//! - Linux: BlueZ D-Bus API (计划中)
//! - Android: 通过 uniffi JNI 绑定 BluetoothLeScanner (计划中)
//!
//! 在不支持 BLE central 模式的平台上，`start()` 方法将返回
//! `BleError::PlatformNotSupported`。
//!
//! # Architecture
//!
//! ```text
//! BleScanner
//! ├── start()     - 启动扫描
//! ├── stop()      - 停止扫描
//! ├── subscribe() - 订阅设备发现事件
//! └── discovered_devices() - 获取设备列表
//! ```

use crate::error::BleError;
use crate::gatt::{MAX_DEVICE_ID_LENGTH, NEARCLIP_SERVICE_UUID, PUBKEY_HASH_LENGTH};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, instrument, warn};

/// 默认扫描超时时间（毫秒，0 表示无限）
pub const DEFAULT_SCAN_TIMEOUT_MS: u64 = 0;

/// 默认设备超时时间（毫秒）
pub const DEFAULT_DEVICE_TIMEOUT_MS: u64 = 30000;

/// 事件通道容量
const EVENT_CHANNEL_CAPACITY: usize = 32;

/// BLE 扫描配置
///
/// 用于配置 BLE 中心模式扫描的参数。
///
/// # Example
///
/// ```
/// use nearclip_ble::BleScannerConfig;
///
/// let config = BleScannerConfig::new()
///     .with_scan_timeout(10000)
///     .with_device_timeout(30000);
///
/// assert!(config.validate().is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct BleScannerConfig {
    /// 扫描超时时间（毫秒，0 表示无限）
    pub scan_timeout_ms: u64,
    /// 设备超时时间（毫秒，超过此时间未收到广播则移除）
    pub device_timeout_ms: u64,
    /// 是否只扫描 NearClip 设备
    pub filter_nearclip_only: bool,
}

impl Default for BleScannerConfig {
    fn default() -> Self {
        Self {
            scan_timeout_ms: DEFAULT_SCAN_TIMEOUT_MS,
            device_timeout_ms: DEFAULT_DEVICE_TIMEOUT_MS,
            filter_nearclip_only: true,
        }
    }
}

impl BleScannerConfig {
    /// 创建新的扫描配置（使用默认值）
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleScannerConfig;
    ///
    /// let config = BleScannerConfig::new();
    /// assert_eq!(config.scan_timeout_ms, 0); // 无限扫描
    /// assert_eq!(config.device_timeout_ms, 30000); // 30秒设备超时
    /// assert!(config.filter_nearclip_only); // 默认只扫描 NearClip 设备
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置扫描超时时间
    ///
    /// # Arguments
    ///
    /// * `timeout_ms` - 超时时间（毫秒），0 表示无限扫描
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleScannerConfig;
    ///
    /// let config = BleScannerConfig::new().with_scan_timeout(10000);
    /// assert_eq!(config.scan_timeout_ms, 10000);
    /// ```
    pub fn with_scan_timeout(mut self, timeout_ms: u64) -> Self {
        self.scan_timeout_ms = timeout_ms;
        self
    }

    /// 设置设备超时时间
    ///
    /// # Arguments
    ///
    /// * `timeout_ms` - 设备超时时间（毫秒）
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleScannerConfig;
    ///
    /// let config = BleScannerConfig::new().with_device_timeout(60000);
    /// assert_eq!(config.device_timeout_ms, 60000);
    /// ```
    pub fn with_device_timeout(mut self, timeout_ms: u64) -> Self {
        self.device_timeout_ms = timeout_ms;
        self
    }

    /// 设置是否只扫描 NearClip 设备
    ///
    /// # Arguments
    ///
    /// * `filter` - 是否过滤非 NearClip 设备
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleScannerConfig;
    ///
    /// let config = BleScannerConfig::new().with_filter_nearclip_only(false);
    /// assert!(!config.filter_nearclip_only);
    /// ```
    pub fn with_filter_nearclip_only(mut self, filter: bool) -> Self {
        self.filter_nearclip_only = filter;
        self
    }

    /// 验证配置
    ///
    /// 检查配置参数是否有效：
    /// - device_timeout_ms 必须大于 0
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError::Configuration`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleScannerConfig;
    ///
    /// let valid_config = BleScannerConfig::new();
    /// assert!(valid_config.validate().is_ok());
    ///
    /// let mut invalid_config = BleScannerConfig::new();
    /// invalid_config.device_timeout_ms = 0;
    /// assert!(invalid_config.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), BleError> {
        if self.device_timeout_ms == 0 {
            return Err(BleError::Configuration(
                "device_timeout_ms must be greater than 0".into(),
            ));
        }

        Ok(())
    }
}

/// 发现的 BLE 设备
///
/// 包含从 BLE 广播中解析的设备信息。
///
/// # Example
///
/// ```
/// use nearclip_ble::DiscoveredDevice;
///
/// let device = DiscoveredDevice::new(
///     "device-001".to_string(),
///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
///     -50,
///     "platform-id-123".to_string(),
/// );
///
/// assert_eq!(device.device_id, "device-001");
/// assert_eq!(device.rssi, -50);
/// ```
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// 设备 ID（从 GATT 特征读取）
    pub device_id: String,
    /// 公钥哈希（从 GATT 特征读取）
    pub public_key_hash: String,
    /// 信号强度 (dBm)
    pub rssi: i16,
    /// 广播名称
    pub advertise_name: Option<String>,
    /// 最后发现时间
    pub last_seen: Instant,
    /// 平台特定标识符（用于后续连接）
    pub platform_identifier: String,
}

impl DiscoveredDevice {
    /// 创建新的发现设备记录
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备 ID
    /// * `public_key_hash` - 公钥哈希（Base64 编码）
    /// * `rssi` - 信号强度 (dBm)
    /// * `platform_identifier` - 平台特定标识符
    pub fn new(
        device_id: String,
        public_key_hash: String,
        rssi: i16,
        platform_identifier: String,
    ) -> Self {
        Self {
            device_id,
            public_key_hash,
            rssi,
            advertise_name: None,
            last_seen: Instant::now(),
            platform_identifier,
        }
    }

    /// 设置广播名称
    pub fn with_advertise_name(mut self, name: String) -> Self {
        self.advertise_name = Some(name);
        self
    }

    /// 检查设备是否已过期
    ///
    /// # Arguments
    ///
    /// * `timeout_ms` - 超时时间（毫秒）
    ///
    /// # Returns
    ///
    /// 如果设备最后发现时间超过 timeout_ms，返回 true
    pub fn is_expired(&self, timeout_ms: u64) -> bool {
        self.last_seen.elapsed() > Duration::from_millis(timeout_ms)
    }

    /// 更新设备信息（RSSI 和最后发现时间）
    pub fn update(&mut self, rssi: i16) {
        self.rssi = rssi;
        self.last_seen = Instant::now();
    }

    /// 验证设备信息
    ///
    /// 检查设备信息是否有效：
    /// - device_id 不能为空且长度不超过 MAX_DEVICE_ID_LENGTH
    /// - public_key_hash 长度必须为 PUBKEY_HASH_LENGTH
    /// - public_key_hash 必须是有效的 Base64
    pub fn validate(&self) -> Result<(), BleError> {
        if self.device_id.is_empty() {
            return Err(BleError::Configuration("device_id cannot be empty".into()));
        }

        if self.device_id.len() > MAX_DEVICE_ID_LENGTH {
            return Err(BleError::Configuration(format!(
                "device_id exceeds maximum length of {} bytes",
                MAX_DEVICE_ID_LENGTH
            )));
        }

        if self.public_key_hash.len() != PUBKEY_HASH_LENGTH {
            return Err(BleError::Configuration(format!(
                "public_key_hash must be {} characters (Base64 encoded SHA-256)",
                PUBKEY_HASH_LENGTH
            )));
        }

        if STANDARD.decode(&self.public_key_hash).is_err() {
            return Err(BleError::Configuration(
                "public_key_hash must be valid Base64".into(),
            ));
        }

        Ok(())
    }
}

/// BLE 设备扫描器
///
/// 提供 BLE central 模式扫描功能，用于发现附近的 NearClip 设备。
///
/// # 设计说明
///
/// 此结构体故意不实现 `Clone`，因为：
/// - BLE 扫描是独占硬件资源的操作
/// - 每个设备应只有一个活跃的扫描器实例
/// - 如需跨任务共享，应使用 `Arc<Mutex<BleScanner>>`
///
/// # Example
///
/// ```no_run
/// use nearclip_ble::{BleScanner, BleScannerConfig, BleError};
///
/// # async fn example() -> Result<(), BleError> {
/// let config = BleScannerConfig::new()
///     .with_scan_timeout(10000);
///
/// let mut scanner = BleScanner::new(config).await?;
///
/// // 订阅设备发现事件
/// let mut rx = scanner.subscribe();
///
/// // 开始扫描
/// scanner.start().await?;
///
/// // 处理发现的设备...
///
/// // 停止扫描
/// scanner.stop().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct BleScanner {
    /// 配置
    config: BleScannerConfig,
    /// 是否正在扫描
    is_scanning: Arc<RwLock<bool>>,
    /// 发现的设备列表
    discovered_devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
    /// 设备发现事件发送器
    event_sender: broadcast::Sender<DiscoveredDevice>,
}

impl BleScanner {
    /// 创建新的扫描器
    ///
    /// # Arguments
    ///
    /// * `config` - 扫描配置
    ///
    /// # Returns
    ///
    /// 成功返回 `BleScanner`，配置无效返回 `BleError::Configuration`
    #[instrument(skip(config), fields(scan_timeout = config.scan_timeout_ms, device_timeout = config.device_timeout_ms))]
    pub async fn new(config: BleScannerConfig) -> Result<Self, BleError> {
        config.validate()?;

        let (event_sender, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);

        info!("BLE scanner created");

        Ok(Self {
            config,
            is_scanning: Arc::new(RwLock::new(false)),
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        })
    }

    /// 开始扫描
    ///
    /// 启动 BLE 中心模式扫描，过滤 NearClip Service UUID。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，平台不支持返回 `BleError::PlatformNotSupported`
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<(), BleError> {
        let is_scanning = self.is_scanning.read().await;
        if *is_scanning {
            debug!("Scanner already running");
            return Ok(());
        }
        drop(is_scanning);

        info!(
            service_uuid = %NEARCLIP_SERVICE_UUID,
            filter_nearclip = self.config.filter_nearclip_only,
            "Starting BLE scan"
        );

        // 平台特定实现
        #[cfg(target_os = "macos")]
        {
            // TODO: 实现 macOS CoreBluetooth CBCentralManager
            warn!("macOS BLE scanning not yet implemented");
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: 实现 Linux BlueZ D-Bus API
            warn!("Linux BLE scanning not yet implemented");
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "android")]
        {
            // TODO: 通过 JNI 调用 BluetoothLeScanner
            warn!("Android BLE scanning not yet implemented");
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
            warn!("BLE scanning not supported on this platform");
            return Err(BleError::PlatformNotSupported);
        }
    }

    /// 停止扫描
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<(), BleError> {
        let mut is_scanning = self.is_scanning.write().await;
        if !*is_scanning {
            debug!("Scanner not running");
            return Ok(());
        }

        *is_scanning = false;
        info!("BLE scan stopped");

        // 平台特定清理代码将在这里添加

        Ok(())
    }

    /// 是否正在扫描
    pub async fn is_scanning(&self) -> bool {
        *self.is_scanning.read().await
    }

    /// 订阅设备发现事件
    ///
    /// # Returns
    ///
    /// 返回用于接收设备发现事件的 `Receiver`
    pub fn subscribe(&self) -> broadcast::Receiver<DiscoveredDevice> {
        self.event_sender.subscribe()
    }

    /// 获取当前发现的设备列表
    ///
    /// 返回所有未过期的设备。
    pub async fn discovered_devices(&self) -> Vec<DiscoveredDevice> {
        let devices = self.discovered_devices.read().await;
        devices
            .values()
            .filter(|d| !d.is_expired(self.config.device_timeout_ms))
            .cloned()
            .collect()
    }

    /// 获取配置
    pub fn config(&self) -> &BleScannerConfig {
        &self.config
    }

    /// 清除过期设备
    ///
    /// 移除超过 device_timeout_ms 未更新的设备。
    pub async fn cleanup_expired_devices(&self) {
        let mut devices = self.discovered_devices.write().await;
        let timeout = self.config.device_timeout_ms;
        devices.retain(|_, device| !device.is_expired(timeout));
        debug!(remaining_devices = devices.len(), "Cleaned up expired devices");
    }

    /// 处理发现的设备（内部方法）
    ///
    /// 由平台特定代码调用，当发现新设备时更新状态并发送事件。
    #[allow(dead_code)]
    async fn handle_discovered_device(&self, device: DiscoveredDevice) -> Result<(), BleError> {
        // 验证设备信息
        device.validate()?;

        let device_id = device.device_id.clone();

        // 更新或添加设备
        let mut devices = self.discovered_devices.write().await;
        let is_new = !devices.contains_key(&device_id);

        if let Some(existing) = devices.get_mut(&device_id) {
            // 更新现有设备
            existing.update(device.rssi);
            debug!(device_id = %device_id, rssi = device.rssi, "Updated existing device");
        } else {
            // 添加新设备
            devices.insert(device_id.clone(), device.clone());
            debug!(device_id = %device_id, rssi = device.rssi, "Discovered new device");
        }
        drop(devices);

        // 发送事件（只对新设备或更新的设备）
        if is_new {
            let _ = self.event_sender.send(device);
        }

        Ok(())
    }
}

impl Drop for BleScanner {
    fn drop(&mut self) {
        // 使用 try_read 避免在 drop 时阻塞
        if let Ok(is_scanning) = self.is_scanning.try_read() {
            if *is_scanning {
                warn!("BLE scanner dropped while still scanning");
                // TODO(platform): 当平台特定实现完成后，在此添加清理代码：
                // - macOS: 调用 CBCentralManager.stopScan()
                // - Linux: 通过 D-Bus 停止 BlueZ 扫描
                // - Android: 通过 JNI 调用 BluetoothLeScanner.stopScan()
            }
        }
        debug!("BLE scanner dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_config_new() {
        let config = BleScannerConfig::new();

        assert_eq!(config.scan_timeout_ms, DEFAULT_SCAN_TIMEOUT_MS);
        assert_eq!(config.device_timeout_ms, DEFAULT_DEVICE_TIMEOUT_MS);
        assert!(config.filter_nearclip_only);
    }

    #[test]
    fn test_scanner_config_with_scan_timeout() {
        let config = BleScannerConfig::new().with_scan_timeout(10000);

        assert_eq!(config.scan_timeout_ms, 10000);
    }

    #[test]
    fn test_scanner_config_with_device_timeout() {
        let config = BleScannerConfig::new().with_device_timeout(60000);

        assert_eq!(config.device_timeout_ms, 60000);
    }

    #[test]
    fn test_scanner_config_with_filter() {
        let config = BleScannerConfig::new().with_filter_nearclip_only(false);

        assert!(!config.filter_nearclip_only);
    }

    #[test]
    fn test_scanner_config_validate_success() {
        let config = BleScannerConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_scanner_config_validate_zero_device_timeout() {
        let mut config = BleScannerConfig::new();
        config.device_timeout_ms = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("device_timeout_ms"));
    }

    #[test]
    fn test_discovered_device_new() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
            -50,
            "platform-id".to_string(),
        );

        assert_eq!(device.device_id, "device-001");
        assert_eq!(
            device.public_key_hash,
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE="
        );
        assert_eq!(device.rssi, -50);
        assert!(device.advertise_name.is_none());
        assert_eq!(device.platform_identifier, "platform-id");
    }

    #[test]
    fn test_discovered_device_with_advertise_name() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
            -50,
            "platform-id".to_string(),
        )
        .with_advertise_name("NearClip".to_string());

        assert_eq!(device.advertise_name, Some("NearClip".to_string()));
    }

    #[test]
    fn test_discovered_device_is_expired() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
            -50,
            "platform-id".to_string(),
        );

        // 刚创建的设备，使用合理超时不应该过期
        assert!(!device.is_expired(1000));
        assert!(!device.is_expired(30000));

        // 刚创建的设备，使用非常长的超时也不会过期
        assert!(!device.is_expired(u64::MAX));
    }

    #[test]
    fn test_discovered_device_validate_success() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
            -50,
            "platform-id".to_string(),
        );

        assert!(device.validate().is_ok());
    }

    #[test]
    fn test_discovered_device_validate_empty_device_id() {
        let device = DiscoveredDevice::new(
            "".to_string(),
            "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
            -50,
            "platform-id".to_string(),
        );

        let result = device.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_id"));
    }

    #[test]
    fn test_discovered_device_validate_invalid_pubkey_hash_length() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "short".to_string(),
            -50,
            "platform-id".to_string(),
        );

        let result = device.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("public_key_hash"));
    }

    #[test]
    fn test_discovered_device_validate_invalid_base64() {
        let device = DiscoveredDevice::new(
            "device-001".to_string(),
            "!!!invalid-base64-string-with-44-chars!!!!==".to_string(),
            -50,
            "platform-id".to_string(),
        );

        let result = device.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Base64"));
    }

    #[tokio::test]
    async fn test_scanner_new_success() {
        let config = BleScannerConfig::new();
        let scanner = BleScanner::new(config).await;

        assert!(scanner.is_ok());
    }

    #[tokio::test]
    async fn test_scanner_new_invalid_config() {
        let mut config = BleScannerConfig::new();
        config.device_timeout_ms = 0;

        let scanner = BleScanner::new(config).await;

        assert!(scanner.is_err());
    }

    #[tokio::test]
    async fn test_scanner_is_scanning_initial_state() {
        let config = BleScannerConfig::new();
        let scanner = BleScanner::new(config).await.unwrap();

        assert!(!scanner.is_scanning().await);
    }

    #[tokio::test]
    async fn test_scanner_stop_without_start() {
        let config = BleScannerConfig::new();
        let mut scanner = BleScanner::new(config).await.unwrap();

        // 未启动时停止应该成功
        let result = scanner.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scanner_config_accessor() {
        let config = BleScannerConfig::new().with_scan_timeout(5000);
        let scanner = BleScanner::new(config).await.unwrap();

        assert_eq!(scanner.config().scan_timeout_ms, 5000);
    }

    #[tokio::test]
    async fn test_scanner_discovered_devices_empty() {
        let config = BleScannerConfig::new();
        let scanner = BleScanner::new(config).await.unwrap();

        let devices = scanner.discovered_devices().await;
        assert!(devices.is_empty());
    }

    #[tokio::test]
    async fn test_scanner_subscribe() {
        let config = BleScannerConfig::new();
        let scanner = BleScanner::new(config).await.unwrap();

        // 订阅应该成功
        let _rx = scanner.subscribe();
    }
}
