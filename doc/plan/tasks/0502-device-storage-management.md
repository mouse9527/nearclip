# Task 0502: 实现设备存储管理 (TDD版本)

## 任务描述

按照TDD原则实现设备存储管理，负责设备信息的持久化和查询。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_storage_tests.rs
#[cfg(test)]
mod device_storage_tests {
    use super::*;
    
    #[test]
    fn test_device_storage_save_and_load() {
        // RED: 测试设备保存和加载
        let storage = DeviceStorage::new();
        let device = DeviceInfo::new("device-001", "Test Device", DeviceType::Mobile);
        
        // 保存设备
        let result = storage.save_device(&device);
        assert!(result.is_ok());
        
        // 加载设备
        let loaded = storage.load_device("device-001").unwrap();
        assert_eq!(loaded.id(), "device-001");
        assert_eq!(loaded.name(), "Test Device");
    }
    
    #[test]
    fn test_device_list_retrieval() {
        // RED: 测试设备列表获取
        let storage = DeviceStorage::new();
        
        // 添加多个设备
        storage.save_device(&DeviceInfo::new("device-001", "Device 1", DeviceType::Mobile)).unwrap();
        storage.save_device(&DeviceInfo::new("device-002", "Device 2", DeviceType::Desktop)).unwrap();
        
        let devices = storage.list_devices().unwrap();
        assert_eq!(devices.len(), 2);
        
        let device_ids: Vec<&str> = devices.iter().map(|d| d.id()).collect();
        assert!(device_ids.contains(&"device-001"));
        assert!(device_ids.contains(&"device-002"));
    }
    
    #[test]
    fn test_device_update() {
        // RED: 测试设备更新
        let mut storage = DeviceStorage::new();
        let mut device = DeviceInfo::new("device-001", "Old Name", DeviceType::Mobile);
        
        // 保存原始设备
        storage.save_device(&device).unwrap();
        
        // 更新设备信息
        device.update_name("New Name");
        storage.save_device(&device).unwrap();
        
        // 验证更新
        let updated = storage.load_device("device-001").unwrap();
        assert_eq!(updated.name(), "New Name");
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::HashMap;

pub struct DeviceStorage {
    devices: HashMap<String, DeviceInfo>,
}

impl DeviceStorage {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
    
    pub fn save_device(&mut self, device: &DeviceInfo) -> Result<(), StorageError> {
        self.devices.insert(device.id().to_string(), device.clone());
        Ok(())
    }
    
    pub fn load_device(&self, device_id: &str) -> Result<DeviceInfo, StorageError> {
        self.devices
            .get(device_id)
            .cloned()
            .ok_or(StorageError::DeviceNotFound)
    }
    
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, StorageError> {
        Ok(self.devices.values().cloned().collect())
    }
    
    pub fn delete_device(&mut self, device_id: &str) -> Result<(), StorageError> {
        self.devices
            .remove(device_id)
            .ok_or(StorageError::DeviceNotFound)?;
        Ok(())
    }
    
    pub fn device_exists(&self, device_id: &str) -> bool {
        self.devices.contains_key(device_id)
    }
}

#[derive(Debug)]
pub enum StorageError {
    DeviceNotFound,
    SerializationError,
    IoError(std::io::Error),
}

// 从前面的任务导入
use super::device_info::{DeviceInfo, DeviceType};
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceStorage {
    devices: HashMap<String, StoredDevice>,
    config: StorageConfig,
    metadata: StorageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredDevice {
    info: DeviceInfo,
    last_sync: Option<SystemTime>,
    sync_count: u32,
    tags: HashSet<String>,
    custom_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_path: PathBuf,
    pub auto_backup: bool,
    pub backup_interval: Duration,
    pub max_backup_count: usize,
    pub enable_compression: bool,
    pub cache_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub version: String,
    pub created_at: SystemTime,
    pub last_modified: SystemTime,
    pub device_count: usize,
    pub total_syncs: u32,
    pub storage_stats: StorageStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_size_bytes: u64,
    pub average_device_size: f64,
    pub oldest_device_age: Duration,
    pub most_active_device: Option<String>,
}

#[derive(Debug)]
pub enum StorageError {
    DeviceNotFound,
    SerializationError(String),
    IoError(std::io::Error),
    DatabaseError(String),
    QuotaExceeded,
    VersionMismatch,
    BackupFailed,
}

#[derive(Debug, Clone)]
pub struct DeviceQuery {
    pub device_type: Option<DeviceType>,
    pub status: Option<DeviceStatus>,
    pub capabilities: Option<Vec<DeviceCapability>>,
    pub name_pattern: Option<String>,
    pub last_seen_after: Option<SystemTime>,
    pub tags: Option<Vec<String>>,
    pub trust_level_min: Option<f32>,
}

#[derive(Debug)]
pub struct DeviceFilter {
    pub query: DeviceQuery,
    pub sort_by: DeviceSortField,
    pub sort_order: SortOrder,
    pub limit: Option<usize>,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub enum DeviceSortField {
    Name,
    CreatedAt,
    LastSeen,
    TrustLevel,
    ConnectionCount,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl DeviceStorage {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Result<Self, StorageError> {
        Self::with_config(StorageConfig::default())
    }
    
    pub fn with_config(config: StorageConfig) -> Result<Self, StorageError> {
        let storage = Self {
            devices: HashMap::new(),
            config,
            metadata: StorageMetadata::new(),
        };
        
        storage.ensure_storage_directory()?;
        storage.load_from_disk()?;
        
        Ok(storage)
    }
    
    pub fn save_device(&mut self, device: &DeviceInfo) -> Result<(), StorageError> {
        // 检查配额
        self.check_quota()?;
        
        let device_id = device.id().to_string();
        let mut stored_device = if let Some(existing) = self.devices.get(&device_id) {
            existing.clone()
        } else {
            StoredDevice::new(device.clone())
        };
        
        stored_device.info = device.clone();
        stored_device.last_sync = Some(SystemTime::now());
        stored_device.sync_count += 1;
        
        self.devices.insert(device_id, stored_device);
        self.touch_metadata();
        
        // 异步保存到磁盘
        self.save_to_disk_async()?;
        
        Ok(())
    }
    
    pub fn load_device(&self, device_id: &str) -> Result<DeviceInfo, StorageError> {
        let stored = self.devices
            .get(device_id)
            .ok_or(StorageError::DeviceNotFound)?;
        Ok(stored.info.clone())
    }
    
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, StorageError> {
        Ok(self.devices.values().map(|d| d.info.clone()).collect())
    }
    
    pub fn query_devices(&self, filter: DeviceFilter) -> Result<Vec<DeviceInfo>, StorageError> {
        let mut devices: Vec<DeviceInfo> = self.devices
            .values()
            .filter(|stored| self.matches_query(&stored.info, &filter.query))
            .map(|stored| stored.info.clone())
            .collect();
        
        // 排序
        self.sort_devices(&mut devices, &filter.sort_by, &filter.sort_order);
        
        // 分页
        if let Some(limit) = filter.limit {
            let end = (filter.offset + limit).min(devices.len());
            devices = devices.into_iter().skip(filter.offset).take(end - filter.offset).collect();
        }
        
        Ok(devices)
    }
    
    pub fn delete_device(&mut self, device_id: &str) -> Result<(), StorageError> {
        self.devices
            .remove(device_id)
            .ok_or(StorageError::DeviceNotFound)?;
        
        self.touch_metadata();
        self.save_to_disk_async()?;
        Ok(())
    }
    
    pub fn device_exists(&self, device_id: &str) -> bool {
        self.devices.contains_key(device_id)
    }
    
    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }
    
    pub fn add_device_tag(&mut self, device_id: &str, tag: String) -> Result<(), StorageError> {
        if let Some(stored) = self.devices.get_mut(device_id) {
            stored.tags.insert(tag);
            self.touch_metadata();
            Ok(())
        } else {
            Err(StorageError::DeviceNotFound)
        }
    }
    
    pub fn remove_device_tag(&mut self, device_id: &str, tag: &str) -> Result<(), StorageError> {
        if let Some(stored) = self.devices.get_mut(device_id) {
            stored.tags.remove(tag);
            self.touch_metadata();
            Ok(())
        } else {
            Err(StorageError::DeviceNotFound)
        }
    }
    
    pub fn get_devices_by_tag(&self, tag: &str) -> Vec<DeviceInfo> {
        self.devices
            .values()
            .filter(|stored| stored.tags.contains(tag))
            .map(|stored| stored.info.clone())
            .collect()
    }
    
    pub fn get_device_custom_data(&self, device_id: &str, key: &str) -> Option<&serde_json::Value> {
        self.devices
            .get(device_id)
            .and_then(|stored| stored.custom_data.get(key))
    }
    
    pub fn set_device_custom_data(&mut self, device_id: &str, key: String, value: serde_json::Value) -> Result<(), StorageError> {
        if let Some(stored) = self.devices.get_mut(device_id) {
            stored.custom_data.insert(key, value);
            self.touch_metadata();
            Ok(())
        } else {
            Err(StorageError::DeviceNotFound)
        }
    }
    
    pub fn cleanup_old_devices(&mut self, older_than: Duration) -> Result<usize, StorageError> {
        let cutoff = SystemTime::now() - older_than;
        let initial_count = self.devices.len();
        
        self.devices.retain(|_, stored| {
            stored.info.last_seen() > cutoff || stored.tags.contains("persistent")
        });
        
        let removed_count = initial_count - self.devices.len();
        if removed_count > 0 {
            self.touch_metadata();
            self.save_to_disk_async()?;
        }
        
        Ok(removed_count)
    }
    
    pub fn create_backup(&self) -> Result<PathBuf, StorageError> {
        let backup_path = self.generate_backup_path();
        let backup_data = self.serialize_data()?;
        
        fs::write(&backup_path, backup_data)
            .map_err(StorageError::IoError)?;
        
        Ok(backup_path)
    }
    
    pub fn restore_from_backup(&mut self, backup_path: &Path) -> Result<(), StorageError> {
        let backup_data = fs::read(backup_path)
            .map_err(StorageError::IoError)?;
        
        let backup_storage: DeviceStorage = serde_json::from_slice(&backup_data)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        self.devices = backup_storage.devices;
        self.metadata = backup_storage.metadata;
        
        Ok(())
    }
    
    pub fn get_storage_stats(&self) -> StorageStats {
        let total_size = self.calculate_total_size();
        let device_count = self.devices.len();
        
        StorageStats {
            total_size_bytes: total_size,
            average_device_size: if device_count > 0 { total_size as f64 / device_count as f64 } else { 0.0 },
            oldest_device_age: self.get_oldest_device_age(),
            most_active_device: self.get_most_active_device(),
        }
    }
    
    // 私有辅助方法
    fn ensure_storage_directory(&self) -> Result<(), StorageError> {
        if !self.config.storage_path.exists() {
            fs::create_dir_all(&self.config.storage_path)
                .map_err(StorageError::IoError)?;
        }
        Ok(())
    }
    
    fn save_to_disk_async(&self) -> Result<(), StorageError> {
        let data = self.serialize_data()?;
        let file_path = self.config.storage_path.join("devices.json");
        
        // 简化的异步保存（实际项目中使用 tokio::spawn）
        std::thread::spawn(move || {
            let _ = fs::write(file_path, data);
        });
        
        Ok(())
    }
    
    fn load_from_disk(&mut self) -> Result<(), StorageError> {
        let file_path = self.config.storage_path.join("devices.json");
        
        if !file_path.exists() {
            return Ok(());
        }
        
        let data = fs::read(&file_path)
            .map_err(StorageError::IoError)?;
        
        let loaded: DeviceStorage = serde_json::from_slice(&data)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        self.devices = loaded.devices;
        self.metadata = loaded.metadata;
        
        Ok(())
    }
    
    fn serialize_data(&self) -> Result<Vec<u8>, StorageError> {
        serde_json::to_vec(self)
            .map_err(|e| StorageError::SerializationError(e.to_string()))
    }
    
    fn check_quota(&self) -> Result<(), StorageError> {
        let stats = self.get_storage_stats();
        // 简化的配额检查
        if stats.total_size_bytes > 100 * 1024 * 1024 { // 100MB
            return Err(StorageError::QuotaExceeded);
        }
        Ok(())
    }
    
    fn matches_query(&self, device: &DeviceInfo, query: &DeviceQuery) -> bool {
        if let Some(ref device_type) = query.device_type {
            if device.device_type() != device_type {
                return false;
            }
        }
        
        if let Some(ref status) = query.status {
            if device.status() != status {
                return false;
            }
        }
        
        if let Some(ref capabilities) = query.capabilities {
            if !device.has_capabilities(capabilities) {
                return false;
            }
        }
        
        if let Some(ref pattern) = query.name_pattern {
            if !device.name().to_lowercase().contains(&pattern.to_lowercase()) {
                return false;
            }
        }
        
        if let Some(ref after) = query.last_seen_after {
            if device.last_seen() < *after {
                return false;
            }
        }
        
        if let Some(ref min_trust) = query.trust_level_min {
            if device.trust_level() < *min_trust {
                return false;
            }
        }
        
        true
    }
    
    fn sort_devices(&self, devices: &mut Vec<DeviceInfo>, sort_by: &DeviceSortField, order: &SortOrder) {
        devices.sort_by(|a, b| {
            let comparison = match sort_by {
                DeviceSortField::Name => a.name().cmp(b.name()),
                DeviceSortField::CreatedAt => a.created_at().cmp(&b.created_at()),
                DeviceSortField::LastSeen => a.last_seen().cmp(&b.last_seen()),
                DeviceSortField::TrustLevel => {
                    a.trust_level().partial_cmp(&b.trust_level()).unwrap_or(std::cmp::Ordering::Equal)
                }
                DeviceSortField::ConnectionCount => a.connection_count().cmp(&b.connection_count()),
            };
            
            match order {
                SortOrder::Ascending => comparison,
                SortOrder::Descending => comparison.reverse(),
            }
        });
    }
    
    fn touch_metadata(&mut self) {
        self.metadata.last_modified = SystemTime::now();
        self.metadata.device_count = self.devices.len();
        self.metadata.total_syncs = self.devices.values().map(|d| d.sync_count).sum();
        self.metadata.storage_stats = self.get_storage_stats();
    }
    
    fn generate_backup_path(&self) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.config.storage_path.join(format!("backup_{}.json", timestamp))
    }
    
    fn calculate_total_size(&self) -> u64 {
        self.devices.values()
            .map(|stored| {
                serde_json::to_string(stored).unwrap_or_default().len() as u64
            })
            .sum()
    }
    
    fn get_oldest_device_age(&self) -> Duration {
        self.devices.values()
            .map(|stored| SystemTime::now().duration_since(stored.info.created_at()).unwrap_or(Duration::ZERO))
            .max()
            .unwrap_or(Duration::ZERO)
    }
    
    fn get_most_active_device(&self) -> Option<String> {
        self.devices.values()
            .max_by_key(|stored| stored.sync_count)
            .map(|stored| stored.info.id().to_string())
    }
}

impl StoredDevice {
    fn new(device: DeviceInfo) -> Self {
        Self {
            info: device,
            last_sync: None,
            sync_count: 0,
            tags: HashSet::new(),
            custom_data: HashMap::new(),
        }
    }
}

impl StorageMetadata {
    fn new() -> Self {
        let now = SystemTime::now();
        Self {
            version: "1.0".to_string(),
            created_at: now,
            last_modified: now,
            device_count: 0,
            total_syncs: 0,
            storage_stats: StorageStats {
                total_size_bytes: 0,
                average_device_size: 0.0,
                oldest_device_age: Duration::ZERO,
                most_active_device: None,
            },
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./data/devices"),
            auto_backup: true,
            backup_interval: Duration::from_secs(3600), // 1小时
            max_backup_count: 10,
            enable_compression: true,
            cache_size: 100,
        }
    }
}

// 从前面任务导入的类型
use super::device_info::{DeviceInfo, DeviceType, DeviceStatus, DeviceCapability};
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为infrastructure层的存储管理：

```rust
// rust-core/infrastructure/storage/device.rs
pub struct DeviceStorage {
    // 设备存储实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [设备信息结构](0501-device-info-structure.md)

## 后续任务

- [Task 0503: 实现设备状态管理](0503-device-state-management.md)
- [Task 0504: 实现设备信任机制](0504-device-trust-mechanism.md)
- [Task 0505: 实现设备生命周期管理](0505-device-lifecycle.md)