//! 设备管理模块
//!
//! 提供设备信息管理、设备配对等功能

use crate::error::{NearClipError, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    Android,
    MacOS,
    Unknown,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub public_key: Vec<u8>,
    pub last_seen: SystemTime,
    pub is_paired: bool,
    pub rssi: Option<i32>,
}

impl Device {
    /// 创建新设备
    pub fn new(id: String, name: String, device_type: DeviceType, public_key: Vec<u8>) -> Self {
        Device {
            id,
            name,
            device_type,
            public_key,
            last_seen: SystemTime::now(),
            is_paired: false,
            rssi: None,
        }
    }

    /// 更新最后见到时间
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// 设置配对状态
    pub fn set_paired(&mut self, paired: bool) {
        self.is_paired = paired;
    }

    /// 更新 RSSI
    pub fn update_rssi(&mut self, rssi: i32) {
        self.rssi = Some(rssi);
    }
}

/// 设备管理器
pub struct DeviceManager {
    devices: std::collections::HashMap<String, Device>,
    current_device_id: String,
}

impl DeviceManager {
    /// 创建新的设备管理器
    pub fn new(current_device_id: String) -> Self {
        DeviceManager {
            devices: std::collections::HashMap::new(),
            current_device_id,
        }
    }

    /// 添加设备
    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.id.clone(), device);
    }

    /// 获取设备
    pub fn get_device(&self, device_id: &str) -> Option<&Device> {
        self.devices.get(device_id)
    }

    /// 获取所有设备
    pub fn get_all_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    /// 获取已配对设备
    pub fn get_paired_devices(&self) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|device| device.is_paired)
            .collect()
    }

    /// 删除设备
    pub fn remove_device(&mut self, device_id: &str) -> Option<Device> {
        self.devices.remove(device_id)
    }

    /// 获取当前设备ID
    pub fn get_current_device_id(&self) -> &str {
        &self.current_device_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let device = Device::new(
            "test-device".to_string(),
            "Test Device".to_string(),
            DeviceType::Android,
            vec![1, 2, 3, 4],
        );

        assert_eq!(device.id, "test-device");
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.device_type, DeviceType::Android);
        assert!(!device.is_paired);
    }

    #[test]
    fn test_device_manager() {
        let mut manager = DeviceManager::new("current-device".to_string());

        let device1 = Device::new(
            "device1".to_string(),
            "Device 1".to_string(),
            DeviceType::Android,
            vec![1, 2, 3, 4],
        );

        let device2 = Device::new(
            "device2".to_string(),
            "Device 2".to_string(),
            DeviceType::MacOS,
            vec![5, 6, 7, 8],
        );

        // 添加设备
        manager.add_device(device1.clone());
        manager.add_device(device2);

        // 验证设备数量
        assert_eq!(manager.get_all_devices().len(), 2);

        // 获取特定设备
        assert!(manager.get_device("device1").is_some());
        assert!(manager.get_device("nonexistent").is_none());

        // 获取当前设备ID
        assert_eq!(manager.get_current_device_id(), "current-device");
    }
}