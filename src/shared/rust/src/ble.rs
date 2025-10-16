//! BLE 通信管理模块
//!
//! 提供设备发现、连接管理、消息传递等功能

use crate::error::{NearClipError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// BLE 设备信息
#[derive(Debug, Clone)]
pub struct BLEDevice {
    pub id: String,
    pub name: String,
    pub address: String,
    pub rssi: i32,
    pub service_data: HashMap<String, Vec<u8>>,
    pub last_seen: Instant,
}

/// BLE 连接状态
#[derive(Debug, Clone, PartialEq)]
pub enum BLEConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Bonded,
}

/// BLE 管理器 - 控制设备发现和连接
pub struct BLEManager {
    devices: Arc<Mutex<HashMap<String, BLEDevice>>>,
    connection_state: Arc<Mutex<HashMap<String, BLEConnectionState>>>,
    is_scanning: Arc<Mutex<bool>>,
    service_uuid: String,
    max_packet_size: usize,
}

impl BLEManager {
    /// 创建新的 BLE 管理器
    pub fn new(service_uuid: String) -> Self {
        BLEManager {
            devices: Arc::new(Mutex::new(HashMap::new())),
            connection_state: Arc::new(Mutex::new(HashMap::new())),
            is_scanning: Arc::new(Mutex::new(false)),
            service_uuid,
            max_packet_size: 20, // BLE 默认 MTU - 3 bytes header
        }
    }

    /// 开始设备扫描
    pub async fn start_scan(&self, timeout_seconds: u64) -> Result<Vec<BLEDevice>> {
        let mut is_scanning = self.is_scanning.lock().unwrap();
        if *is_scanning {
            return Err(NearClipError::BLEError("Already scanning".to_string()));
        }
        *is_scanning = true;
        drop(is_scanning);

        // 清除旧的设备列表
        let mut devices = self.devices.lock().unwrap();
        devices.clear();
        drop(devices);

        // 模拟设备扫描过程
        let scan_duration = Duration::from_secs(timeout_seconds);
        let start_time = Instant::now();

        while start_time.elapsed() < scan_duration {
            // 在实际实现中，这里会调用系统BLE API
            if let Some(device) = self.simulate_device_discovery().await {
                let mut devices = self.devices.lock().unwrap();
                devices.insert(device.id.clone(), device);
            }

            sleep(Duration::from_millis(100)).await;
        }

        let is_scanning = self.is_scanning.lock().unwrap();
        *is_scanning = false;

        let devices = self.devices.lock().unwrap();
        Ok(devices.values().cloned().collect())
    }

    /// 停止设备扫描
    pub fn stop_scan(&self) {
        let mut is_scanning = self.is_scanning.lock().unwrap();
        *is_scanning = false;
    }

    /// 获取发现的设备列表
    pub fn get_discovered_devices(&self) -> Vec<BLEDevice> {
        let devices = self.devices.lock().unwrap();
        devices.values().cloned().collect()
    }

    /// 连接到设备
    pub async fn connect_to_device(&self, device_id: &str) -> Result<()> {
        let devices = self.devices.lock().unwrap();
        if !devices.contains_key(device_id) {
            return Err(NearClipError::BLEError(format!("Device not found: {}", device_id)));
        }
        drop(devices);

        let mut connection_state = self.connection_state.lock().unwrap();
        connection_state.insert(device_id.to_string(), BLEConnectionState::Connecting);
        drop(connection_state);

        // 模拟连接过程
        sleep(Duration::from_millis(500)).await;

        let mut connection_state = self.connection_state.lock().unwrap();
        connection_state.insert(device_id.to_string(), BLEConnectionState::Connected);

        Ok(())
    }

    /// 断开设备连接
    pub async fn disconnect_from_device(&self, device_id: &str) -> Result<()> {
        let mut connection_state = self.connection_state.lock().unwrap();
        if connection_state.get(device_id) != Some(&BLEConnectionState::Connected) {
            return Err(NearClipError::BLEError("Device not connected".to_string()));
        }

        connection_state.insert(device_id.to_string(), BLEConnectionState::Disconnected);
        drop(connection_state);

        // 模拟断开连接过程
        sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    /// 获取设备连接状态
    pub fn get_connection_state(&self, device_id: &str) -> BLEConnectionState {
        let connection_state = self.connection_state.lock().unwrap();
        connection_state.get(device_id).cloned().unwrap_or(BLEConnectionState::Disconnected)
    }

    /// 发送消息到设备
    pub async fn send_message(&self, device_id: &str, message: &[u8]) -> Result<()> {
        if self.get_connection_state(device_id) != BLEConnectionState::Connected {
            return Err(NearClipError::BLEError("Device not connected".to_string()));
        }

        // 分片发送大消息
        let chunks = self.chunk_message(message)?;

        for chunk in chunks {
            self.send_chunk(device_id, &chunk).await?;
            sleep(Duration::from_millis(10)).await; // 避免BLE缓冲区溢出
        }

        Ok(())
    }

    /// 分片消息以适应BLE包大小限制
    fn chunk_message(&self, message: &[u8]) -> Result<Vec<Vec<u8>>> {
        if message.len() <= self.max_packet_size {
            return Ok(vec![message.to_vec()]);
        }

        let mut chunks = Vec::new();
        let mut offset = 0;

        while offset < message.len() {
            let end = (offset + self.max_packet_size).min(message.len());
            chunks.push(message[offset..end].to_vec());
            offset = end;
        }

        Ok(chunks)
    }

    /// 发送单个数据块
    async fn send_chunk(&self, device_id: &str, chunk: &[u8]) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE写入API
        println!("Sending {} bytes to device: {}", chunk.len(), device_id);

        // 模拟发送延迟
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    /// 开始广播设备信息
    pub async fn start_advertising(&self, device_info: &[u8]) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE广播API
        println!("Starting advertising with device info: {} bytes", device_info.len());

        // 模拟广播启动
        sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    /// 停止广播
    pub async fn stop_advertising(&self) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE停止广播API
        println!("Stopping advertising");

        Ok(())
    }

    /// 模拟设备发现（仅用于演示）
    async fn simulate_device_discovery(&self) -> Option<BLEDevice> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        // 10% 的概率发现一个设备
        if rng.gen_range(0..10) == 0 {
            Some(BLEDevice {
                id: format!("device-{}", rng.gen_range(1000..9999)),
                name: format!("NearClip-Device-{}", rng.gen_range(100..999)),
                address: format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256)
                ),
                rssi: rng.gen_range(-90..-30),
                service_data: HashMap::new(),
                last_seen: Instant::now(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_scan() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 启动扫描
        let devices = ble_manager.start_scan(2).await.unwrap();

        // 验证扫描结果
        assert!(!devices.is_empty());

        for device in devices {
            assert!(!device.id.is_empty());
            assert!(!device.name.is_empty());
            assert!(!device.address.is_empty());
        }
    }

    #[tokio::test]
    async fn test_device_connection() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 先扫描设备
        let devices = ble_manager.start_scan(1).await.unwrap();
        if let Some(device) = devices.first() {
            // 连接设备
            assert!(ble_manager.connect_to_device(&device.id).await.is_ok());

            // 检查连接状态
            assert_eq!(ble_manager.get_connection_state(&device.id), BLEConnectionState::Connected);

            // 断开连接
            assert!(ble_manager.disconnect_from_device(&device.id).await.is_ok());

            // 检查断开状态
            assert_eq!(ble_manager.get_connection_state(&device.id), BLEConnectionState::Disconnected);
        }
    }

    #[test]
    fn test_message_chunking() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 小消息不应分片
        let small_message = vec![1, 2, 3];
        let chunks = ble_manager.chunk_message(&small_message).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], small_message);

        // 大消息应该分片
        let large_message = vec![1; 50]; // 超过默认的20字节限制
        let chunks = ble_manager.chunk_message(&large_message).unwrap();
        assert!(chunks.len() > 1);

        // 验证重组后的消息
        let mut reassembled = Vec::new();
        for chunk in chunks {
            reassembled.extend_from_slice(&chunk);
        }
        assert_eq!(reassembled, large_message);
    }
}