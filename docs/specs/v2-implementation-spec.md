# NearClip v2 实现技术规范

**文档类型**: 技术规范
**版本**: 2.0
**日期**: 2025-12-26
**状态**: 执行中

---

## 1. 概述

本文档描述 NearClip v2 架构重构的技术实现细节。开发团队应严格按照本规范实现。

### 1.1 参考文档

- 架构决策: `/Users/mouse/project/mouse/nearclip/docs/architecture-v2-adr.md`
- 项目规则: `/Users/mouse/project/mouse/nearclip/docs/project_context.md`

---

## 2. Phase 1: 设备管理层

### 2.1 创建 Crate

**路径**: `crates/nearclip-device/`

**Cargo.toml**:
```toml
[package]
name = "nearclip-device"
version = "0.2.0"
edition = "2021"

[dependencies]
nearclip-crypto = { path = "../nearclip-crypto" }
thiserror = "1.0"
rusqlite = { version = "0.30", features = ["bundled"] }
tokio = { version = "1", features = ["sync", "rt"] }
tracing = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
```

### 2.2 数据库设计

**文件**: `crates/nearclip-device/src/store.rs`

```rust
use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DeviceStore {
    db: Arc<RwLock<Connection>>,
}

impl DeviceStore {
    pub fn new(db_path: PathBuf) -> Result<Self, DeviceError> {
        let conn = Connection::open(db_path)?;

        // 启用 WAL 模式
        conn.execute("PRAGMA journal_mode=WAL", [])?;
        conn.execute("PRAGMA synchronous=NORMAL", [])?;
        conn.execute("PRAGMA foreign_keys=ON", [])?;

        // 创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                platform TEXT NOT NULL,
                public_key BLOB NOT NULL,
                shared_secret BLOB NOT NULL,
                paired_at INTEGER NOT NULL,
                last_connected INTEGER,
                last_seen INTEGER
            )",
            [],
        )?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_devices_platform ON devices(platform)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_devices_last_seen ON devices(last_seen)",
            [],
        )?;

        Ok(Self {
            db: Arc::new(RwLock::new(conn)),
        })
    }

    pub async fn save_device(&self, device: &PairedDevice) -> Result<(), DeviceError> {
        let db = self.db.read().await;
        // 实现保存逻辑
        todo!()
    }

    pub async fn remove_device(&self, device_id: &str) -> Result<(), DeviceError> {
        let db = self.db.read().await;
        // 实现删除逻辑
        todo!()
    }

    pub async fn load_all_devices(&self) -> Result<Vec<PairedDevice>, DeviceError> {
        let db = self.db.read().await;
        // 实现加载逻辑
        todo!()
    }

    pub async fn get_device(&self, device_id: &str) -> Result<Option<PairedDevice>, DeviceError> {
        let db = self.db.read().await;
        // 实现查询逻辑
        todo!()
    }
}
```

### 2.3 设备模型

**文件**: `crates/nearclip-device/src/models.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedDevice {
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    pub public_key: Vec<u8>,
    pub shared_secret: Vec<u8>,
    pub paired_at: i64,
    pub last_connected: Option<i64>,
    pub last_seen: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DevicePlatform {
    MacOS,
    Windows,
    Linux,
    Android,
    Ios,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    public_key_hash: String,
    pub channel: DiscoveryChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryChannel {
    WiFi { ip: String, port: u16 },
    BLE { peripheral_id: String, rssi: i16 },
}
```

### 2.4 DeviceManager

**文件**: `crates/nearclip-device/src/manager.rs`

```rust
use crate::store::DeviceStore;
use crate::models::{PairedDevice, DiscoveredDevice, DevicePlatform};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DeviceManager {
    store: DeviceStore,
    discovered: Arc<RwLock<Vec<DiscoveredDevice>>>,
    paired: Arc<RwLock<Vec<PairedDevice>>>,
    connected: Arc<RwLock<Vec<String>>>, // device_ids
}

impl DeviceManager {
    pub fn new(db_path: std::path::PathBuf) -> Result<Self, DeviceError> {
        let store = DeviceStore::new(db_path)?;
        // 加载已配对设备
        let paired = store.load_all_devices().await?;

        Ok(Self {
            store,
            discovered: Arc::new(RwLock::new(Vec::new())),
            paired: Arc::new(RwLock::new(paired)),
            connected: Arc::new(RwLock::new(Vec::new())),
        })
    }

    // 设备发现
    pub async fn add_discovered_device(&self, device: DiscoveredDevice) {
        let mut discovered = self.discovered.write().await;
        if !discovered.iter().any(|d| d.device_id == device.device_id) {
            discovered.push(device);
        }
    }

    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        self.discovered.read().await.clone()
    }

    // 配对
    pub async fn pair_device(&self, device: PairedDevice) -> Result<(), DeviceError> {
        self.store.save_device(&device).await?;

        let mut paired = self.paired.write().await;
        if !paired.iter().any(|d| d.device_id == device.device_id) {
            paired.push(device);
        }

        Ok(())
    }

    pub async fn unpair_device(&self, device_id: &str) -> Result<(), DeviceError> {
        self.store.remove_device(device_id).await?;

        let mut paired = self.paired.write().await;
        paired.retain(|d| d.device_id != device_id);

        Ok(())
    }

    // 查询
    pub async fn get_paired_devices(&self) -> Vec<PairedDevice> {
        self.paired.read().await.clone()
    }

    pub async fn get_device(&self, device_id: &str) -> Option<PairedDevice> {
        let paired = self.paired.read().await;
        paired.iter().find(|d| d.device_id == device_id).cloned()
    }

    pub async fn is_paired(&self, device_id: &str) -> bool {
        let paired = self.paired.read().await;
        paired.iter().any(|d| d.device_id == device_id)
    }

    // 连接状态
    pub async fn mark_connected(&self, device_id: &str) {
        let mut connected = self.connected.write().await;
        if !connected.contains(&device_id.to_string()) {
            connected.push(device_id.to_string());
        }
    }

    pub async fn mark_disconnected(&self, device_id: &str) {
        let mut connected = self.connected.write().await;
        connected.retain(|id| id != device_id);
    }

    pub async fn get_connected_devices(&self) -> Vec<PairedDevice> {
        let connected = self.connected.read().await;
        let paired = self.paired.read().await;

        paired
            .iter()
            .filter(|d| connected.contains(&d.device_id))
            .cloned()
            .collect()
    }

    pub async fn is_connected(&self, device_id: &str) -> bool {
        let connected = self.connected.read().await;
        connected.contains(&device_id.to_string())
    }
}
```

### 2.5 错误类型

**文件**: `crates/nearclip-device/src/error.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Device not found: {0}")]
    NotFound(String),

    #[error("Device already paired: {0}")]
    AlreadyPaired(String),

    #[error("Invalid device data: {0}")]
    InvalidData(String),

    #[error("Storage error: {0}")]
    Storage(String),
}
```

### 2.6 测试

**文件**: `crates/nearclip-device/tests/store_test.rs`

```rust
use nearclip_device::{DeviceStore, PairedDevice, DevicePlatform};
use tempfile::tempdir;

#[tokio::test]
async fn test_save_and_load_device() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    let store = DeviceStore::new(db_path).unwrap();

    let device = PairedDevice {
        device_id: "test-device-1".to_string(),
        device_name: "Test Device".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: vec![1, 2, 3, 4],
        shared_secret: vec![5, 6, 7, 8],
        paired_at: 1703577600000,
        last_connected: None,
        last_seen: None,
    };

    store.save_device(&device).await.unwrap();

    let loaded = store.load_all_devices().await.unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].device_id, "test-device-1");
}

#[tokio::test]
async fn test_remove_device() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");

    let store = DeviceStore::new(db_path).unwrap();

    let device = PairedDevice {
        device_id: "test-device-1".to_string(),
        device_name: "Test Device".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: vec![1, 2, 3, 4],
        shared_secret: vec![5, 6, 7, 8],
        paired_at: 1703577600000,
        last_connected: None,
        last_seen: None,
    };

    store.save_device(&device).await.unwrap();
    store.remove_device("test-device-1").await.unwrap();

    let loaded = store.load_all_devices().await.unwrap();
    assert_eq!(loaded.len(), 0);
}
```

---

## 3. Phase 2: BLE 层重构

### 3.1 BleHardware Trait 简化

**文件**: `crates/nearclip-ble/src/hardware.rs`

```rust
/// 平台只需实现这个最小接口
pub trait BleHardware: Send + Sync {
    // 扫描
    fn start_scan(&self);
    fn stop_scan(&self);

    // 连接（原始 BLE 操作）
    fn connect(&self, peripheral_id: &str);
    fn disconnect(&self, peripheral_id: &str);

    // GATT 操作（返回 Result 表示成功/失败）
    fn read_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
    ) -> Result<Vec<u8>, String>;

    fn write_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
        data: &[u8],
    ) -> Result<(), String>;

    fn subscribe_characteristic(
        &self,
        peripheral_id: &str,
        char_uuid: &str,
    ) -> Result<(), String>;

    // 广播
    fn start_advertising(&self, service_data: &[u8]);
    fn stop_advertising(&self);

    // 状态查询
    fn is_connected(&self, peripheral_id: &str) -> bool;
    fn get_mtu(&self, peripheral_id: &str) -> u16;
}
```

### 3.2 BleController 重构

**文件**: `crates/nearclip-ble/src/controller.rs`

```rust
pub struct BleController {
    hardware: Arc<dyn BleHardware>,
    config: BleConfig,

    // 设备映射
    peripheral_to_device: Arc<RwLock<HashMap<String, String>>>,
    device_to_peripheral: Arc<RwLock<HashMap<String, String>>>,

    // 连接状态
    connected_peripherals: Arc<RwLock<HashSet<String>>>,

    // 数据分片（移到 Rust 层）
    chunker: DataChunker,
    reassemblers: Arc<RwLock<HashMap<String, DataReassembler>>>,

    // 配对协议
    pairing_state: Arc<RwLock<HashMap<String, PairingSession>>>,
}

impl BleController {
    pub fn new(hardware: Arc<dyn BleHardware>, config: BleConfig) -> Self {
        Self {
            hardware,
            config,
            peripheral_to_device: Arc::new(RwLock::new(HashMap::new())),
            device_to_peripheral: Arc::new(RwLock::new(HashMap::new())),
            connected_peripherals: Arc::new(RwLock::new(HashSet::new())),
            chunker: DataChunker::new(),
            reassemblers: Arc::new(RwLock::new(HashMap::new())),
            pairing_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ========== 设备发现 ==========

    pub async fn discover_devices(&self, timeout: Duration) -> Vec<DiscoveredDevice> {
        self.hardware.start_scan();

        let deadline = tokio::time::Instant::now() + timeout;
        let mut discovered = Vec::new();

        while tokio::time::Instant::now() < deadline {
            tokio::time::sleep(Duration::from_millis(200)).await;
            // 从 scan callback 收集的设备
        }

        self.hardware.stop_scan();
        discovered
    }

    // 处理扫描结果（由平台调用）
    pub async fn on_scan_result(
        &self,
        peripheral_id: String,
        rssi: i16,
        adv_data: Vec<u8>,
    ) {
        // 解析广播数据，提取设备 ID 和服务 UUID
        if let Some(device_id) = self.parse_device_id_from_adv(&adv_data) {
            let device = DiscoveredDevice {
                device_id: device_id.clone(),
                peripheral_id: peripheral_id.clone(),
                rssi,
            };

            // 更新映射
            let mut p2d = self.peripheral_to_device.write().await;
            let mut d2p = self.device_to_peripheral.write().await;
            p2d.insert(peripheral_id.clone(), device_id.clone());
            d2p.insert(device_id, peripheral_id);
        }
    }

    // ========== 连接管理 ==========

    pub async fn connect(&self, device_id: &str) -> Result<(), BleError> {
        // 获取 peripheral_id
        let peripheral_id = {
            let d2p = self.device_to_peripheral.read().await;
            d2p.get(device_id).cloned()
        };

        let peripheral_id = peripheral_id.ok_or_else(|| {
            BleError::DeviceNotFound(device_id.to_string())
        })?;

        // 连接
        self.hardware.connect(&peripheral_id);

        // 等待连接完成
        tokio::time::sleep(Duration::from_secs(5)).await;

        // 验证连接
        if self.hardware.is_connected(&peripheral_id) {
            let mut connected = self.connected_peripherals.write().await;
            connected.insert(peripheral_id);
            Ok(())
        } else {
            Err(BleError::ConnectionFailed("Connection timeout".to_string()))
        }
    }

    pub async fn disconnect(&self, device_id: &str) -> Result<(), BleError> {
        let peripheral_id = {
            let d2p = self.device_to_peripheral.read().await;
            d2p.get(device_id).cloned()
        };

        if let Some(pid) = peripheral_id {
            self.hardware.disconnect(&pid);
            let mut connected = self.connected_peripherals.write().await;
            connected.remove(&pid);
        }

        Ok(())
    }

    pub async fn is_connected(&self, device_id: &str) -> bool {
        let d2p = self.device_to_peripheral.read().await;
        if let Some(peripheral_id) = d2p.get(device_id) {
            self.hardware.is_connected(peripheral_id)
        } else {
            false
        }
    }

    // ========== 数据传输 ==========

    pub async fn send(&self, device_id: &str, data: &[u8]) -> Result<(), BleError> {
        let peripheral_id = {
            let d2p = self.device_to_peripheral.read().await;
            d2p.get(device_id).cloned().ok_or_else(|| {
                BleError::DeviceNotFound(device_id.to_string())
            })?
        };

        let mtu = self.hardware.get_mtu(&peripheral_id) as usize;
        let chunks = self.chunker.create_chunks(data, mtu);

        for chunk in chunks {
            self.hardware.write_characteristic(
                &peripheral_id,
                DATA_TX_UUID,
                &chunk,
            ).map_err(|e| BleError::SendError(e))?;

            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        Ok(())
    }

    // 处理接收到的数据（由平台调用）
    pub async fn on_data_received(
        &self,
        peripheral_id: &str,
        data: Vec<u8>,
    ) -> Option<(String, Vec<u8>)> {
        // 获取 device_id
        let device_id = {
            let p2d = self.peripheral_to_device.read().await;
            p2d.get(peripheral_id).cloned()
        };

        let device_id = device_id?;

        // 重组数据
        let reassembler = {
            let mut reassemblers = self.reassemblers.write().await;
            tokio::task::block_in_place(|| {
                reassemblers.entry(device_id.clone())
                    .or_insert_with(DataReassembler::new)
            })
        };

        if let Some(complete) = reassembler.add_chunk(data) {
            Some((device_id, complete))
        } else {
            None
        }
    }

    // ========== 处理连接状态变化 ==========

    pub async fn on_connected(&self, peripheral_id: &str) {
        // 读取设备 ID
        if let Ok(data) = self.hardware.read_characteristic(peripheral_id, DEVICE_ID_UUID) {
            if let Ok(device_id) = String::from_utf8(data) {
                let mut p2d = self.peripheral_to_device.write().await;
                let mut d2p = self.device_to_peripheral.write().await;
                p2d.insert(peripheral_id.to_string(), device_id.clone());
                d2p.insert(device_id, peripheral_id.to_string());
            }
        }

        let mut connected = self.connected_peripherals.write().await;
        connected.insert(peripheral_id.to_string());
    }

    pub async fn on_disconnected(&self, peripheral_id: &str) {
        let mut connected = self.connected_peripherals.write().await;
        connected.remove(peripheral_id);
    }
}
```

### 3.3 数据分片（移到 Rust）

**文件**: `crates/nearclip-ble/src/chunk.rs`

```rust
pub struct DataChunker {
    message_id_counter: u32,
}

const CHUNK_HEADER_SIZE: usize = 8; // [msg_id:4][seq:2][total:2]

impl DataChunker {
    pub fn new() -> Self {
        Self {
            message_id_counter: 0,
        }
    }

    pub fn create_chunks(&mut self, data: &[u8], mtu: usize) -> Vec<Vec<u8>> {
        let payload_size = mtu.saturating_sub(CHUNK_HEADER_SIZE);
        if payload_size == 0 {
            return vec![];
        }

        self.message_id_counter = self.message_id_counter.wrapping_add(1);
        let message_id = self.message_id_counter;

        let total_chunks = (data.len() + payload_size - 1) / payload_size;
        let mut chunks = Vec::new();

        for (seq, chunk) in data.chunks(payload_size).enumerate() {
            let mut packet = Vec::with_capacity(CHUNK_HEADER_SIZE + chunk.len());

            // Header
            packet.extend_from_slice(&message_id.to_le_bytes());
            packet.extend_from_slice(&(seq as u16).to_le_bytes());
            packet.extend_from_slice(&(total_chunks as u16).to_le_bytes());

            // Payload
            packet.extend_from_slice(chunk);

            chunks.push(packet);
        }

        chunks
    }
}

pub struct DataReassembler {
    chunks: HashMap<u16, Vec<u8>>,
    total_chunks: u16,
    message_id: u32,
    last_activity: Instant,
}

impl DataReassembler {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            total_chunks: 0,
            message_id: 0,
            last_activity: Instant::now(),
        }
    }

    pub fn add_chunk(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
        self.last_activity = Instant::now();

        if data.len() < CHUNK_HEADER_SIZE {
            return None;
        }

        let message_id = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let seq = u16::from_le_bytes(data[4..6].try_into().unwrap());
        let total = u16::from_le_bytes(data[6..8].try_into().unwrap());
        let payload = &data[CHUNK_HEADER_SIZE..];

        // 新消息
        if self.message_id != message_id || self.total_chunks == 0 {
            self.chunks.clear();
            self.message_id = message_id;
            self.total_chunks = total;
        }

        self.chunks.insert(seq, payload.to_vec());

        // 检查是否完整
        if self.chunks.len() == self.total_chunks as usize {
            let mut result = Vec::new();
            for i in 0..self.total_chunks {
                if let Some(chunk) = self.chunks.get(&i) {
                    result.extend_from_slice(chunk);
                }
            }
            self.chunks.clear();
            Some(result)
        } else {
            None
        }
    }

    pub fn is_timed_out(&self) -> bool {
        self.last_activity.elapsed() > Duration::from_secs(5)
    }
}
```

---

## 4. Phase 3: 配对协议实现

### 4.1 配对消息

**文件**: `crates/nearclip-protocol/src/pairing.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PairingMessage {
    PairingRequest(PairingRequest),
    PairingResponse(PairingResponse),
    PairingConfirm(PairingConfirm),
    PairingComplete,
    PairingRejected(PairingRejected),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingRequest {
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    pub public_key: Vec<u8>,
    pub nonce: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingResponse {
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    pub public_key: Vec<u8>,
    pub nonce: [u8; 32],
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingConfirm {
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingRejected {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevicePlatform {
    MacOS,
    Windows,
    Linux,
    Android,
    Ios,
}
```

### 4.2 PairingManager

**文件**: `crates/nearclip-device/src/pairing.rs`

```rust
use nearclip_crypto::CryptoEngine;
use crate::manager::DeviceManager;
use nearclip_protocol::PairingMessage;

pub struct PairingManager {
    device_manager: Arc<DeviceManager>,
    crypto: Arc<CryptoEngine>,
    transport: Arc<dyn Transport>,
}

impl PairingManager {
    pub fn new(
        device_manager: Arc<DeviceManager>,
        crypto: Arc<CryptoEngine>,
        transport: Arc<dyn Transport>,
    ) -> Self {
        Self {
            device_manager,
            crypto,
            transport,
        }
    }

    /// 作为发起方发起配对
    pub async fn initiate_pairing(
        &self,
        qr_data: &str,
    ) -> Result<DeviceInfo, PairingError> {
        // 1. 解析 QR 码
        let qr_data: QrCodeData = serde_json::from_str(qr_data)?;

        // 2. 连接到设备
        self.transport.connect(&qr_data.device_id).await?;

        // 3. 生成配对请求
        let nonce = self.generate_nonce();
        let request = PairingMessage::PairingRequest(PairingRequest {
            device_id: self.device_manager.get_device_id(),
            device_name: self.device_manager.get_device_name(),
            platform: self.device_manager.get_platform(),
            public_key: self.crypto.get_public_key(),
            nonce,
        });

        // 4. 发送请求
        let request_data = rmp_serde::to_vec(&request)?;
        self.transport.send(&qr_data.device_id, &request_data).await?;

        // 5. 等待响应
        let response_data = self.transport.recv_timeout(Duration::from_secs(30)).await?;
        let response: PairingMessage = rmp_serde::from_slice(&response_data)?;

        match response {
            PairingMessage::PairingResponse(resp) => {
                // 验证响应
                self.verify_pairing_response(&qr_data, &resp, &nonce)?;

                // 计算共享密钥
                let shared_secret = self.crypto.derive_shared_key(&resp.public_key)?;

                // 发送确认
                let confirm = PairingMessage::PairingConfirm(PairingConfirm {
                    signature: self.crypto.sign(&resp.nonce)?,
                });
                let confirm_data = rmp_serde::to_vec(&confirm)?;
                self.transport.send(&qr_data.device_id, &confirm_data).await?;

                // 保存设备
                let device = PairedDevice {
                    device_id: resp.device_id.clone(),
                    device_name: resp.device_name.clone(),
                    platform: resp.platform,
                    public_key: resp.public_key.clone(),
                    shared_secret,
                    paired_at: now_millis(),
                    last_connected: Some(now_millis()),
                    last_seen: Some(now_millis()),
                };
                self.device_manager.pair_device(device).await?;

                Ok(DeviceInfo {
                    id: resp.device_id,
                    name: resp.device_name,
                    platform: resp.platform,
                })
            }
            PairingMessage::PairingRejected(rejected) => {
                Err(PairingError::Rejected(rejected.reason))
            }
            _ => Err(PairingError::InvalidMessage),
        }
    }

    /// 作为响应方处理配对请求
    pub async fn handle_pairing_request(
        &self,
        request: PairingRequest,
    ) -> Result<(), PairingError> {
        // 1. 验证请求
        if self.device_manager.is_paired(&request.device_id).await {
            return Err(PairingError::AlreadyPaired);
        }

        // 2. 计算共享密钥
        let shared_secret = self.crypto.derive_shared_key(&request.public_key)?;

        // 3. 生成响应
        let nonce = self.generate_nonce();
        let signature = self.crypto.sign(&request.nonce)?;

        let response = PairingMessage::PairingResponse(PairingResponse {
            device_id: self.device_manager.get_device_id(),
            device_name: self.device_manager.get_device_name(),
            platform: self.device_manager.get_platform(),
            public_key: self.crypto.get_public_key(),
            nonce,
            signature,
        });

        // 4. 发送响应
        let response_data = rmp_serde::to_vec(&response)?;
        self.transport.send(&request.device_id, &response_data).await?;

        // 5. 等待确认
        let confirm_data = self.transport.recv_timeout(Duration::from_secs(30)).await?;
        let confirm: PairingMessage = rmp_serde::from_slice(&confirm_data)?;

        match confirm {
            PairingMessage::PairingConfirm(confirm) => {
                // 验证确认
                self.crypto.verify(&nonce, &confirm.signature)?;

                // 保存设备
                let device = PairedDevice {
                    device_id: request.device_id.clone(),
                    device_name: request.device_name.clone(),
                    platform: request.platform,
                    public_key: request.public_key.clone(),
                    shared_secret,
                    paired_at: now_millis(),
                    last_connected: Some(now_millis()),
                    last_seen: Some(now_millis()),
                };
                self.device_manager.pair_device(device).await?;

                // 发送完成消息
                let complete = PairingMessage::PairingComplete;
                let complete_data = rmp_serde::to_vec(&complete)?;
                self.transport.send(&request.device_id, &complete_data).await?;

                Ok(())
            }
            _ => Err(PairingError::InvalidMessage),
        }
    }

    fn generate_nonce(&self) -> [u8; 32] {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

fn now_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
```

---

## 5. 验收标准

每个 Phase 完成后，需要满足以下验收标准：

### Phase 1: 设备管理层
- [ ] `nearclip-device` crate 编译通过
- [ ] `DeviceStore` 单元测试通过
- [ ] `DeviceManager` 集成测试通过
- [ ] 数据库正确创建和初始化
- [ ] 设备 CRUD 操作正常工作

### Phase 2: BLE 层重构
- [ ] `BleHardware` trait 简化完成
- [ ] `BleController` 重构完成
- [ ] 数据分片移到 Rust 层
- [ ] 平台 BLE 代码减少 > 50%
- [ ] BLE 传输测试通过

### Phase 3: 配对协议
- [ ] 双向配对协议实现完成
- [ ] 配对握手测试通过
- [ ] 两端都能看到对方设备
- [ ] 配对拒绝流程正常

### Phase 4: 传输层统一
- [ ] WiFi/BLE 无缝切换
- [ ] 端到端加密覆盖所有通道
- [ ] 健康检查正常工作

### Phase 5: 平台适配
- [ ] macOS Swift 代码更新完成
- [ ] Android Kotlin 代码更新完成
- [ ] 跨平台集成测试通过

---

## 6. 开发规范

### 6.1 代码风格
- 遵循 `project_context.md` 中的所有规则
- 使用 `tracing` 记录日志
- 使用 `thiserror` 定义错误类型
- 异步函数使用 `async fn`

### 6.2 测试要求
- 单元测试覆盖率 > 80%
- 所有公共函数必须有测试
- 使用 `mock` 进行隔离测试
- 集成测试覆盖关键流程

### 6.3 文档要求
- 每个公共函数必须有文档注释
- 复杂逻辑需要示例代码
- 更新 `architecture-v2-adr.md` 记录决策变更

---

## 7. 下一步

完成本规范后，调用 dev 代理开始实现：

```
/bmad:bmm:agents:dev
```

参考文档：
- 架构决策: `docs/architecture-v2-adr.md`
- 本规范: `docs/specs/v2-implementation-spec.md`
- 项目规则: `docs/project_context.md`
