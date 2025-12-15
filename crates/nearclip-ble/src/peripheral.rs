//! BLE 外设广播模块
//!
//! 提供 BLE Peripheral 模式广播功能，使设备可被其他设备发现。
//!
//! # 平台支持
//!
//! 当前实现提供统一的 API 接口。平台特定的 BLE 实现将通过 feature flags 启用：
//! - macOS: CoreBluetooth (计划中)
//! - Linux: BlueZ (计划中)
//! - Android: 通过 uniffi JNI 绑定 (计划中)
//!
//! 在不支持 BLE peripheral 模式的平台上，`start()` 方法将返回
//! `BleError::PlatformNotSupported`。

use crate::error::BleError;
use crate::gatt::{
    DEFAULT_ADVERTISE_NAME, MAX_ADVERTISE_NAME_LENGTH, MAX_DEVICE_ID_LENGTH, PUBKEY_HASH_LENGTH,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument, warn};

/// BLE 广播配置
///
/// 用于配置 BLE 外设广播的参数。
///
/// # Example
///
/// ```
/// use nearclip_ble::BleAdvertiserConfig;
///
/// let config = BleAdvertiserConfig::new(
///     "device-001".to_string(),
///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
/// );
///
/// assert!(config.validate().is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct BleAdvertiserConfig {
    /// 设备 ID
    pub device_id: String,
    /// 公钥哈希（Base64 编码）
    pub public_key_hash: String,
    /// 广播名称（可选，默认 "NearClip"）
    pub advertise_name: Option<String>,
}

impl BleAdvertiserConfig {
    /// 创建新的配置
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备唯一标识符
    /// * `public_key_hash` - Base64 编码的公钥哈希
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleAdvertiserConfig;
    ///
    /// let config = BleAdvertiserConfig::new(
    ///     "my-device".to_string(),
    ///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    /// );
    /// ```
    pub fn new(device_id: String, public_key_hash: String) -> Self {
        Self {
            device_id,
            public_key_hash,
            advertise_name: None,
        }
    }

    /// 设置自定义广播名称
    ///
    /// # Arguments
    ///
    /// * `name` - 自定义广播名称
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleAdvertiserConfig;
    ///
    /// let config = BleAdvertiserConfig::new(
    ///     "device-001".to_string(),
    ///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    /// ).with_name("MyDevice".to_string());
    ///
    /// assert_eq!(config.advertise_name, Some("MyDevice".to_string()));
    /// ```
    pub fn with_name(mut self, name: String) -> Self {
        self.advertise_name = Some(name);
        self
    }

    /// 验证配置
    ///
    /// 检查配置参数是否有效：
    /// - device_id 不能为空
    /// - device_id 长度不能超过 MAX_DEVICE_ID_LENGTH
    /// - public_key_hash 不能为空
    /// - advertise_name（如果设置）长度不能超过 MAX_ADVERTISE_NAME_LENGTH
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError::Configuration`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_ble::BleAdvertiserConfig;
    ///
    /// let valid_config = BleAdvertiserConfig::new(
    ///     "device-001".to_string(),
    ///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    /// );
    /// assert!(valid_config.validate().is_ok());
    ///
    /// let invalid_config = BleAdvertiserConfig::new(
    ///     "".to_string(),
    ///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
    /// );
    /// assert!(invalid_config.validate().is_err());
    /// ```
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

        if self.public_key_hash.is_empty() {
            return Err(BleError::Configuration(
                "public_key_hash cannot be empty".into(),
            ));
        }

        // 验证 public_key_hash 长度（SHA-256 的 Base64 编码应为 44 字符）
        if self.public_key_hash.len() != PUBKEY_HASH_LENGTH {
            return Err(BleError::Configuration(format!(
                "public_key_hash must be {} characters (Base64 encoded SHA-256)",
                PUBKEY_HASH_LENGTH
            )));
        }

        // 验证 public_key_hash 是有效的 Base64
        if STANDARD.decode(&self.public_key_hash).is_err() {
            return Err(BleError::Configuration(
                "public_key_hash must be valid Base64".into(),
            ));
        }

        if let Some(ref name) = self.advertise_name {
            if name.len() > MAX_ADVERTISE_NAME_LENGTH {
                return Err(BleError::Configuration(format!(
                    "advertise_name exceeds maximum length of {} bytes",
                    MAX_ADVERTISE_NAME_LENGTH
                )));
            }
        }

        Ok(())
    }

    /// 获取实际使用的广播名称
    ///
    /// 返回自定义名称或默认名称 "NearClip"
    pub fn effective_name(&self) -> &str {
        self.advertise_name
            .as_deref()
            .unwrap_or(DEFAULT_ADVERTISE_NAME)
    }
}

/// BLE 外设广播器
///
/// 提供 BLE peripheral 模式广播功能，使设备可被其他 NearClip 设备发现。
///
/// # 设计说明
///
/// 此结构体故意不实现 `Clone`，因为：
/// - BLE 广播是独占硬件资源的操作
/// - 每个设备应只有一个活跃的广播器实例
/// - 如需跨任务共享，应使用 `Arc<Mutex<BleAdvertiser>>`
///
/// # Example
///
/// ```no_run
/// use nearclip_ble::{BleAdvertiser, BleAdvertiserConfig, BleError};
///
/// # async fn example() -> Result<(), BleError> {
/// let config = BleAdvertiserConfig::new(
///     "device-001".to_string(),
///     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
/// );
///
/// let mut advertiser = BleAdvertiser::new(config).await?;
///
/// // 开始广播
/// advertiser.start().await?;
/// assert!(advertiser.is_advertising().await);
///
/// // 停止广播
/// advertiser.stop().await?;
/// assert!(!advertiser.is_advertising().await);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct BleAdvertiser {
    /// 配置
    config: BleAdvertiserConfig,
    /// 是否正在广播
    is_advertising: Arc<RwLock<bool>>,
}

impl BleAdvertiser {
    /// 创建新的广播器
    ///
    /// # Arguments
    ///
    /// * `config` - 广播配置
    ///
    /// # Returns
    ///
    /// 成功返回 `BleAdvertiser`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::Configuration` - 配置无效
    #[instrument(skip(config), fields(device_id = %config.device_id))]
    pub async fn new(config: BleAdvertiserConfig) -> Result<Self, BleError> {
        // 验证配置
        config.validate()?;

        debug!(
            device_id = %config.device_id,
            advertise_name = %config.effective_name(),
            "Creating BLE advertiser"
        );

        Ok(Self {
            config,
            is_advertising: Arc::new(RwLock::new(false)),
        })
    }

    /// 开始 BLE 广播
    ///
    /// 启动 BLE peripheral 模式广播，使设备可被其他设备发现。
    /// 广播包含 NearClip GATT 服务，包括设备 ID 和公钥哈希特征。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    ///
    /// # Errors
    ///
    /// - `BleError::PlatformNotSupported` - 当前平台不支持 BLE peripheral 模式
    /// - `BleError::NotPowered` - BLE 未开启
    /// - `BleError::Advertising` - 广播启动失败
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<(), BleError> {
        // 注意：当平台特定实现完成后，需要改为 write() 并设置 *is_advertising = true
        let is_advertising = self.is_advertising.read().await;

        if *is_advertising {
            warn!("BLE advertising already started");
            return Ok(());
        }
        drop(is_advertising);

        info!(
            device_id = %self.config.device_id,
            advertise_name = %self.config.effective_name(),
            "Starting BLE advertising"
        );

        // 平台特定实现
        #[cfg(target_os = "macos")]
        {
            // TODO: 实现 macOS CoreBluetooth peripheral
            // 当前返回 PlatformNotSupported，等待 objc2 绑定实现
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: 实现 Linux BlueZ peripheral
            // 当前返回 PlatformNotSupported，等待 zbus 绑定实现
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(target_os = "android")]
        {
            // Android 通过 uniffi JNI 绑定实现
            // 实际实现在 nearclip-ffi 和 Android 端
            return Err(BleError::PlatformNotSupported);
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
            return Err(BleError::PlatformNotSupported);
        }
    }

    /// 停止 BLE 广播
    ///
    /// 停止 BLE peripheral 模式广播。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `BleError`
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<(), BleError> {
        let is_advertising_val = *self.is_advertising.read().await;

        if !is_advertising_val {
            debug!("BLE advertising not running");
            return Ok(());
        }

        info!(
            device_id = %self.config.device_id,
            "Stopping BLE advertising"
        );

        // 平台特定实现
        // 由于 start() 在不支持的平台上返回错误，
        // 这里不会真正执行到（is_advertising 永远是 false）
        {
            let mut is_advertising = self.is_advertising.write().await;
            *is_advertising = false;
        }

        info!("BLE advertising stopped");

        Ok(())
    }

    /// 检查是否正在广播
    ///
    /// # Returns
    ///
    /// 如果正在广播返回 `true`
    pub async fn is_advertising(&self) -> bool {
        *self.is_advertising.read().await
    }

    /// 获取配置
    ///
    /// # Returns
    ///
    /// 当前广播配置的引用
    pub fn config(&self) -> &BleAdvertiserConfig {
        &self.config
    }
}

impl Drop for BleAdvertiser {
    fn drop(&mut self) {
        // 使用 try_read 避免在 drop 时阻塞
        if let Ok(is_advertising) = self.is_advertising.try_read() {
            if *is_advertising {
                warn!("BLE advertiser dropped while still advertising");
                // TODO(platform): 当平台特定实现完成后，在此添加清理代码：
                // - macOS: 调用 CBPeripheralManager.stopAdvertising()
                // - Linux: 通过 D-Bus 停止 BlueZ 广播
                // - Android: 通过 JNI 调用 BluetoothLeAdvertiser.stopAdvertising()
                // 注意：由于 drop 是同步的，可能需要使用 spawn_blocking 或类似机制
            }
        }
        debug!("BLE advertiser dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        assert_eq!(config.device_id, "device-001");
        assert_eq!(config.public_key_hash, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=");
        assert!(config.advertise_name.is_none());
    }

    #[test]
    fn test_config_with_name() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
            .with_name("MyDevice".to_string());

        assert_eq!(config.advertise_name, Some("MyDevice".to_string()));
    }

    #[test]
    fn test_config_effective_name_default() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        assert_eq!(config.effective_name(), "NearClip");
    }

    #[test]
    fn test_config_effective_name_custom() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
            .with_name("Custom".to_string());

        assert_eq!(config.effective_name(), "Custom");
    }

    #[test]
    fn test_config_validate_success() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_empty_device_id() {
        let config = BleAdvertiserConfig::new("".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("device_id cannot be empty"));
    }

    #[test]
    fn test_config_validate_empty_public_key_hash() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("public_key_hash cannot be empty"));
    }

    #[test]
    fn test_config_validate_device_id_too_long() {
        let long_id = "x".repeat(MAX_DEVICE_ID_LENGTH + 1);
        let config = BleAdvertiserConfig::new(long_id, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("device_id exceeds maximum length"));
    }

    #[test]
    fn test_config_validate_advertise_name_too_long() {
        let long_name = "x".repeat(MAX_ADVERTISE_NAME_LENGTH + 1);
        let config =
            BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
                .with_name(long_name);

        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("advertise_name exceeds maximum length"));
    }

    #[tokio::test]
    async fn test_advertiser_new_success() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        let advertiser = BleAdvertiser::new(config).await;
        assert!(advertiser.is_ok());

        let advertiser = advertiser.unwrap();
        assert!(!advertiser.is_advertising().await);
        assert_eq!(advertiser.config().device_id, "device-001");
    }

    #[tokio::test]
    async fn test_advertiser_new_invalid_config() {
        let config = BleAdvertiserConfig::new("".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        let advertiser = BleAdvertiser::new(config).await;
        assert!(advertiser.is_err());
    }

    #[tokio::test]
    async fn test_advertiser_stop_without_start() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string());

        let mut advertiser = BleAdvertiser::new(config).await.unwrap();

        // 停止未启动的广播应该成功
        let result = advertiser.stop().await;
        assert!(result.is_ok());
        assert!(!advertiser.is_advertising().await);
    }

    #[tokio::test]
    async fn test_advertiser_config_accessor() {
        let config = BleAdvertiserConfig::new("device-001".to_string(), "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string())
            .with_name("TestDevice".to_string());

        let advertiser = BleAdvertiser::new(config).await.unwrap();

        assert_eq!(advertiser.config().device_id, "device-001");
        assert_eq!(advertiser.config().public_key_hash, "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=");
        assert_eq!(
            advertiser.config().advertise_name,
            Some("TestDevice".to_string())
        );
    }
}
