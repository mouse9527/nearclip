//! NearClip BLE Module
//!
//! Bluetooth Low Energy layer for device discovery and data transfer.
//! Provides fallback communication channel when WiFi is unavailable.
//!
//! # Architecture
//!
//! ```text
//! nearclip-ble
//! ├── error.rs          - BLE error types
//! ├── gatt.rs           - GATT service/characteristic UUID definitions
//! ├── peripheral.rs     - BLE peripheral mode (advertising)
//! ├── central.rs        - BLE central mode (scanning)
//! ├── chunk.rs          - Data chunking for MTU limitations
//! ├── peripheral_data.rs - Peripheral mode data receiving
//! └── central_data.rs    - Central mode data sending
//! ```
//!
//! # Peripheral Mode (Advertising)
//!
//! Use [`BleAdvertiser`] to broadcast this device for discovery:
//!
//! ```no_run
//! use nearclip_ble::{BleAdvertiser, BleAdvertiserConfig, BleError};
//!
//! # async fn example() -> Result<(), BleError> {
//! let config = BleAdvertiserConfig::new(
//!     "my-device-id".to_string(),
//!     "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE=".to_string(),
//! );
//!
//! let mut advertiser = BleAdvertiser::new(config).await?;
//! advertiser.start().await?;
//!
//! // ... device is now discoverable via BLE ...
//!
//! advertiser.stop().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Central Mode (Scanning)
//!
//! Use [`BleScanner`] to discover nearby NearClip devices:
//!
//! ```no_run
//! use nearclip_ble::{BleScanner, BleScannerConfig, BleError};
//!
//! # async fn example() -> Result<(), BleError> {
//! let config = BleScannerConfig::new()
//!     .with_scan_timeout(10000);
//!
//! let mut scanner = BleScanner::new(config).await?;
//!
//! // 订阅设备发现事件
//! let mut rx = scanner.subscribe();
//!
//! // 开始扫描（当前返回 PlatformNotSupported）
//! // scanner.start().await?;
//!
//! // 获取已发现的设备
//! let devices = scanner.discovered_devices().await;
//! # Ok(())
//! # }
//! ```

pub mod central;
pub mod central_data;
pub mod chunk;
pub mod controller;
pub mod error;
pub mod gatt;
pub mod peripheral;
pub mod peripheral_data;

// Re-exports
pub use central::{
    BleScanner, BleScannerConfig, DiscoveredDevice, DEFAULT_DEVICE_TIMEOUT_MS,
    DEFAULT_SCAN_TIMEOUT_MS,
};
pub use chunk::{ChunkHeader, Chunker, Reassembler, DEFAULT_REASSEMBLE_TIMEOUT};
pub use error::BleError;
pub use gatt::{
    ATT_HEADER_SIZE, CHUNK_HEADER_SIZE, DATA_ACK_CHARACTERISTIC_UUID,
    DATA_TRANSFER_CHARACTERISTIC_UUID, DEFAULT_ADVERTISE_NAME, DEFAULT_BLE_MTU,
    DEFAULT_CHUNK_PAYLOAD_SIZE, DEVICE_ID_CHARACTERISTIC_UUID, MAX_ADVERTISE_NAME_LENGTH,
    MAX_BLE_MTU, MAX_CHUNK_PAYLOAD_SIZE, MAX_DEVICE_ID_LENGTH, NEARCLIP_SERVICE_UUID,
    PUBKEY_HASH_CHARACTERISTIC_UUID, PUBKEY_HASH_LENGTH,
};
pub use peripheral::{BleAdvertiser, BleAdvertiserConfig};
pub use peripheral_data::{
    DataReceiverCallback, PeripheralDataConfig, PeripheralDataReceiver,
    DEFAULT_REASSEMBLE_TIMEOUT_SECS, MAX_CONCURRENT_MESSAGES,
};
pub use central_data::{
    CentralDataConfig, CentralDataSender, DataSenderCallback, SendState,
    DEFAULT_ACK_TIMEOUT_MS, DEFAULT_MTU, DEFAULT_RETRY_COUNT, DEFAULT_SEND_TIMEOUT_SECS,
};
pub use controller::{
    BleController, BleControllerCallback, BleControllerConfig, BleHardware,
    DiscoveredDevice as ControllerDiscoveredDevice,
};
