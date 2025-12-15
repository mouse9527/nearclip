//! 配对设备存储
//!
//! 提供配对设备的持久化存储功能。
//!
//! # Example
//!
//! ```ignore
//! use nearclip_crypto::{FileDeviceStore, DeviceStore, PairedDevice};
//!
//! // 创建存储
//! let store = FileDeviceStore::new();
//!
//! // 保存设备
//! let device = PairedDevice::new("device-1".to_string(), vec![0x04; 65], &[0xAB; 32], None);
//! store.save(&device).unwrap();
//!
//! // 加载设备
//! let loaded = store.load("device-1").unwrap();
//! assert!(loaded.is_some());
//!
//! // 加载所有设备
//! let all = store.load_all().unwrap();
//! assert_eq!(all.len(), 1);
//! ```

use crate::{CryptoError, PairedDevice};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

/// 存储文件格式版本
const STORE_VERSION: u8 = 1;

/// 设备存储接口
///
/// 定义配对设备的持久化操作。不同平台可实现不同后端：
/// - `FileDeviceStore` - JSON 文件存储（默认）
/// - 未来：`KeychainDeviceStore` - macOS Keychain
/// - 未来：`KeystoreDeviceStore` - Android Keystore
///
/// # Example
///
/// ```ignore
/// use nearclip_crypto::{DeviceStore, FileDeviceStore, PairedDevice};
///
/// let store = FileDeviceStore::new();
///
/// // 保存设备
/// let device = PairedDevice::new("id".to_string(), vec![], &[0; 32], None);
/// store.save(&device)?;
///
/// // 加载设备
/// let loaded = store.load("id")?;
/// ```
pub trait DeviceStore {
    /// 保存或更新设备信息
    ///
    /// 如果设备 ID 已存在，则更新；否则添加新设备。
    fn save(&self, device: &PairedDevice) -> Result<(), CryptoError>;

    /// 按 ID 加载设备
    ///
    /// 返回 `None` 如果设备不存在。
    fn load(&self, device_id: &str) -> Result<Option<PairedDevice>, CryptoError>;

    /// 加载所有已配对设备
    fn load_all(&self) -> Result<Vec<PairedDevice>, CryptoError>;

    /// 删除设备
    ///
    /// 返回 `true` 如果设备存在并被删除，`false` 如果设备不存在。
    fn delete(&self, device_id: &str) -> Result<bool, CryptoError>;

    /// 检查设备是否存在
    fn exists(&self, device_id: &str) -> Result<bool, CryptoError>;

    /// 获取已配对设备数量
    fn count(&self) -> Result<usize, CryptoError> {
        Ok(self.load_all()?.len())
    }
}

/// 存储文件内容结构
#[derive(Debug, Serialize, Deserialize)]
struct StoreFile {
    /// 文件格式版本
    version: u8,
    /// 已配对设备列表
    devices: Vec<PairedDevice>,
}

impl StoreFile {
    /// 创建新的存储文件
    fn new() -> Self {
        Self {
            version: STORE_VERSION,
            devices: Vec::new(),
        }
    }

    /// 从 JSON 字符串解析
    fn from_json(json: &str) -> Result<Self, CryptoError> {
        serde_json::from_str(json)
            .map_err(|e| CryptoError::DeviceStore(format!("Failed to parse store file: {}", e)))
    }

    /// 转换为 JSON 字符串
    fn to_json(&self) -> Result<String, CryptoError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| CryptoError::DeviceStore(format!("Failed to serialize store file: {}", e)))
    }

    /// 查找设备索引
    fn find_index(&self, device_id: &str) -> Option<usize> {
        self.devices.iter().position(|d| d.device_id == device_id)
    }
}

/// 文件存储配置
#[derive(Debug, Clone)]
pub struct FileDeviceStoreConfig {
    /// 存储目录
    pub directory: PathBuf,
    /// 文件名
    pub filename: String,
}

impl Default for FileDeviceStoreConfig {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            filename: "paired_devices.json".to_string(),
        }
    }
}

impl FileDeviceStoreConfig {
    /// 创建新配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置存储目录
    pub fn with_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.directory = directory.as_ref().to_path_buf();
        self
    }

    /// 设置文件名
    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    /// 获取完整文件路径
    pub fn file_path(&self) -> PathBuf {
        self.directory.join(&self.filename)
    }
}

/// 基于文件的设备存储
///
/// 使用 JSON 格式存储配对设备列表。
///
/// # Example
///
/// ```ignore
/// use nearclip_crypto::{FileDeviceStore, FileDeviceStoreConfig, DeviceStore};
/// use std::path::PathBuf;
///
/// // 使用默认配置
/// let store = FileDeviceStore::new();
///
/// // 使用自定义配置
/// let config = FileDeviceStoreConfig::new()
///     .with_directory("/path/to/data")
///     .with_filename("devices.json");
/// let store = FileDeviceStore::with_config(config);
/// ```
pub struct FileDeviceStore {
    config: FileDeviceStoreConfig,
}

impl FileDeviceStore {
    /// 创建默认配置的存储
    ///
    /// 默认存储在当前目录的 `paired_devices.json` 文件。
    pub fn new() -> Self {
        Self {
            config: FileDeviceStoreConfig::default(),
        }
    }

    /// 使用自定义配置
    pub fn with_config(config: FileDeviceStoreConfig) -> Self {
        Self { config }
    }

    /// 获取存储文件路径
    pub fn file_path(&self) -> PathBuf {
        self.config.file_path()
    }

    /// 获取配置引用
    pub fn config(&self) -> &FileDeviceStoreConfig {
        &self.config
    }

    /// 读取存储文件
    #[instrument(skip(self))]
    fn read_store(&self) -> Result<StoreFile, CryptoError> {
        let path = self.file_path();

        if !path.exists() {
            debug!("Store file does not exist, returning empty store");
            return Ok(StoreFile::new());
        }

        let mut file = File::open(&path).map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to open store file: {}", e))
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to read store file: {}", e))
        })?;

        if contents.trim().is_empty() {
            debug!("Store file is empty, returning empty store");
            return Ok(StoreFile::new());
        }

        let store = StoreFile::from_json(&contents)?;
        debug!("Loaded {} devices from store", store.devices.len());
        Ok(store)
    }

    /// 写入存储文件
    #[instrument(skip(self, store))]
    fn write_store(&self, store: &StoreFile) -> Result<(), CryptoError> {
        let path = self.file_path();

        // 确保目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    CryptoError::DeviceStore(format!("Failed to create directory: {}", e))
                })?;
            }
        }

        // 使用原子写入：先写入临时文件，再重命名
        let temp_path = path.with_extension("json.tmp");
        let json = store.to_json()?;

        let mut file = File::create(&temp_path).map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to create temp file: {}", e))
        })?;

        file.write_all(json.as_bytes()).map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to write temp file: {}", e))
        })?;

        file.sync_all().map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to sync temp file: {}", e))
        })?;

        // 原子重命名
        fs::rename(&temp_path, &path).map_err(|e| {
            CryptoError::DeviceStore(format!("Failed to rename temp file: {}", e))
        })?;

        debug!("Wrote {} devices to store", store.devices.len());
        Ok(())
    }
}

impl Default for FileDeviceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceStore for FileDeviceStore {
    #[instrument(skip(self, device), fields(device_id = %device.device_id))]
    fn save(&self, device: &PairedDevice) -> Result<(), CryptoError> {
        let mut store = self.read_store()?;

        // 查找是否已存在
        if let Some(index) = store.find_index(&device.device_id) {
            // 更新现有设备
            store.devices[index] = device.clone();
            info!("Updated device: {}", device.device_id);
        } else {
            // 添加新设备
            store.devices.push(device.clone());
            info!("Added new device: {}", device.device_id);
        }

        self.write_store(&store)
    }

    #[instrument(skip(self))]
    fn load(&self, device_id: &str) -> Result<Option<PairedDevice>, CryptoError> {
        let store = self.read_store()?;

        let device = store
            .devices
            .into_iter()
            .find(|d| d.device_id == device_id);

        if device.is_some() {
            debug!("Found device: {}", device_id);
        } else {
            debug!("Device not found: {}", device_id);
        }

        Ok(device)
    }

    #[instrument(skip(self))]
    fn load_all(&self) -> Result<Vec<PairedDevice>, CryptoError> {
        let store = self.read_store()?;
        debug!("Loaded {} devices", store.devices.len());
        Ok(store.devices)
    }

    #[instrument(skip(self))]
    fn delete(&self, device_id: &str) -> Result<bool, CryptoError> {
        let mut store = self.read_store()?;

        if let Some(index) = store.find_index(device_id) {
            store.devices.remove(index);
            self.write_store(&store)?;
            info!("Deleted device: {}", device_id);
            Ok(true)
        } else {
            debug!("Device not found for deletion: {}", device_id);
            Ok(false)
        }
    }

    #[instrument(skip(self))]
    fn exists(&self, device_id: &str) -> Result<bool, CryptoError> {
        let store = self.read_store()?;
        let exists = store.find_index(device_id).is_some();
        debug!("Device {} exists: {}", device_id, exists);
        Ok(exists)
    }
}

impl std::fmt::Debug for FileDeviceStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileDeviceStore")
            .field("file_path", &self.file_path())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConnectionInfo;
    use std::env;

    /// 创建临时测试目录
    fn temp_store() -> FileDeviceStore {
        let temp_dir = env::temp_dir().join(format!(
            "nearclip_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config = FileDeviceStoreConfig::new()
            .with_directory(&temp_dir)
            .with_filename("test_devices.json");
        FileDeviceStore::with_config(config)
    }

    /// 清理测试目录
    fn cleanup(store: &FileDeviceStore) {
        let _ = fs::remove_file(store.file_path());
        if let Some(parent) = store.file_path().parent() {
            let _ = fs::remove_dir(parent);
        }
    }

    fn sample_device(id: &str) -> PairedDevice {
        PairedDevice::new(
            id.to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            Some(ConnectionInfo::new().with_ip("192.168.1.1").with_port(8080)),
        )
    }

    #[test]
    fn test_file_device_store_new() {
        let store = FileDeviceStore::new();
        assert_eq!(store.config().filename, "paired_devices.json");
    }

    #[test]
    fn test_file_device_store_with_config() {
        let config = FileDeviceStoreConfig::new()
            .with_directory("/tmp/test")
            .with_filename("devices.json");
        let store = FileDeviceStore::with_config(config);
        assert_eq!(store.file_path(), PathBuf::from("/tmp/test/devices.json"));
    }

    #[test]
    fn test_save_and_load_device() {
        let store = temp_store();
        let device = sample_device("device-1");

        // 保存
        store.save(&device).unwrap();

        // 加载
        let loaded = store.load("device-1").unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.device_id, "device-1");
        assert_eq!(loaded.public_key_bytes, device.public_key_bytes);

        cleanup(&store);
    }

    #[test]
    fn test_load_nonexistent_device() {
        let store = temp_store();

        let loaded = store.load("nonexistent").unwrap();
        assert!(loaded.is_none());

        cleanup(&store);
    }

    #[test]
    fn test_load_all_empty() {
        let store = temp_store();

        let devices = store.load_all().unwrap();
        assert!(devices.is_empty());

        cleanup(&store);
    }

    #[test]
    fn test_load_all_multiple_devices() {
        let store = temp_store();

        let device1 = sample_device("device-1");
        let device2 = sample_device("device-2");
        let device3 = sample_device("device-3");

        store.save(&device1).unwrap();
        store.save(&device2).unwrap();
        store.save(&device3).unwrap();

        let devices = store.load_all().unwrap();
        assert_eq!(devices.len(), 3);

        cleanup(&store);
    }

    #[test]
    fn test_update_device() {
        let store = temp_store();

        let mut device = sample_device("device-1");
        store.save(&device).unwrap();

        // 更新
        device.public_key_bytes = vec![0x05; 65];
        store.save(&device).unwrap();

        // 验证更新
        let loaded = store.load("device-1").unwrap().unwrap();
        assert_eq!(loaded.public_key_bytes, vec![0x05; 65]);

        // 确认只有一个设备
        let all = store.load_all().unwrap();
        assert_eq!(all.len(), 1);

        cleanup(&store);
    }

    #[test]
    fn test_delete_device() {
        let store = temp_store();
        let device = sample_device("device-1");

        store.save(&device).unwrap();
        assert!(store.exists("device-1").unwrap());

        let deleted = store.delete("device-1").unwrap();
        assert!(deleted);

        assert!(!store.exists("device-1").unwrap());
        let loaded = store.load("device-1").unwrap();
        assert!(loaded.is_none());

        cleanup(&store);
    }

    #[test]
    fn test_delete_nonexistent_device() {
        let store = temp_store();

        let deleted = store.delete("nonexistent").unwrap();
        assert!(!deleted);

        cleanup(&store);
    }

    #[test]
    fn test_exists() {
        let store = temp_store();
        let device = sample_device("device-1");

        assert!(!store.exists("device-1").unwrap());

        store.save(&device).unwrap();
        assert!(store.exists("device-1").unwrap());

        store.delete("device-1").unwrap();
        assert!(!store.exists("device-1").unwrap());

        cleanup(&store);
    }

    #[test]
    fn test_count() {
        let store = temp_store();

        assert_eq!(store.count().unwrap(), 0);

        store.save(&sample_device("device-1")).unwrap();
        assert_eq!(store.count().unwrap(), 1);

        store.save(&sample_device("device-2")).unwrap();
        assert_eq!(store.count().unwrap(), 2);

        store.delete("device-1").unwrap();
        assert_eq!(store.count().unwrap(), 1);

        cleanup(&store);
    }

    #[test]
    fn test_store_file_format() {
        let store = temp_store();
        let device = sample_device("device-1");

        store.save(&device).unwrap();

        // 读取原始文件内容
        let contents = fs::read_to_string(store.file_path()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();

        // 验证格式
        assert_eq!(parsed["version"], STORE_VERSION);
        assert!(parsed["devices"].is_array());
        assert_eq!(parsed["devices"].as_array().unwrap().len(), 1);

        cleanup(&store);
    }

    #[test]
    fn test_config_builder() {
        let config = FileDeviceStoreConfig::new()
            .with_directory("/var/data")
            .with_filename("my_devices.json");

        assert_eq!(config.directory, PathBuf::from("/var/data"));
        assert_eq!(config.filename, "my_devices.json");
        assert_eq!(config.file_path(), PathBuf::from("/var/data/my_devices.json"));
    }

    #[test]
    fn test_store_file_version() {
        let store_file = StoreFile::new();
        assert_eq!(store_file.version, STORE_VERSION);
        assert!(store_file.devices.is_empty());
    }

    #[test]
    fn test_store_file_json_roundtrip() {
        let mut store_file = StoreFile::new();
        store_file.devices.push(sample_device("device-1"));
        store_file.devices.push(sample_device("device-2"));

        let json = store_file.to_json().unwrap();
        let parsed = StoreFile::from_json(&json).unwrap();

        assert_eq!(parsed.version, store_file.version);
        assert_eq!(parsed.devices.len(), store_file.devices.len());
    }

    #[test]
    fn test_device_with_connection_info() {
        let store = temp_store();

        let device = PairedDevice::new(
            "device-with-info".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            Some(
                ConnectionInfo::new()
                    .with_ip("10.0.0.1")
                    .with_port(12345)
                    .with_mdns_name("device._nearclip._tcp.local"),
            ),
        );

        store.save(&device).unwrap();

        let loaded = store.load("device-with-info").unwrap().unwrap();
        let conn = loaded.connection_info.as_ref().unwrap();
        assert_eq!(conn.ip, Some("10.0.0.1".to_string()));
        assert_eq!(conn.port, Some(12345));
        assert_eq!(conn.mdns_name, Some("device._nearclip._tcp.local".to_string()));

        cleanup(&store);
    }

    #[test]
    fn test_device_without_connection_info() {
        let store = temp_store();

        let device = PairedDevice::new(
            "device-no-info".to_string(),
            vec![0x04; 65],
            &[0xAB; 32],
            None,
        );

        store.save(&device).unwrap();

        let loaded = store.load("device-no-info").unwrap().unwrap();
        assert!(loaded.connection_info.is_none());

        cleanup(&store);
    }

    #[test]
    fn test_debug_impl() {
        let store = FileDeviceStore::new();
        let debug_str = format!("{:?}", store);
        assert!(debug_str.contains("FileDeviceStore"));
        assert!(debug_str.contains("file_path"));
    }
}
