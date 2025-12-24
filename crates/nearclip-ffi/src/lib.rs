//! NearClip FFI Module
//!
//! Foreign Function Interface bindings using uniffi.
//! Exports NearClip API for Swift (macOS/iOS) and Kotlin (Android).
//!
//! # Overview
//!
//! This module provides the bridge between platform clients (Swift/Kotlin)
//! and the Rust core library. It uses Mozilla's uniffi to generate
//! language-specific bindings.
//!
//! # Types
//!
//! FFI types use the `Ffi` prefix to distinguish from internal types:
//! - `FfiDeviceInfo` -> `DeviceInfo`
//! - `FfiNearClipConfig` -> `NearClipConfig`
//! - `FfiNearClipManager` -> `NearClipManager`

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use nearclip_core::{
    DeviceInfo, DevicePlatform, DeviceStatus, NearClipCallback, NearClipConfig, NearClipError,
    NearClipManager,
};
use nearclip_sync::MessageType;
use nearclip_transport::{BleTransport, BleSender, Transport};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

// Include the uniffi generated scaffolding
uniffi::include_scaffolding!("nearclip");

// ============================================================
// Logging Functions (namespace level)
// ============================================================

/// Initialize the logging system
///
/// Should be called once at application startup.
/// Safe to call multiple times (subsequent calls are ignored).
pub fn init_logging(level: LogLevel) {
    let core_level = match level {
        LogLevel::Error => nearclip_core::LogLevel::Error,
        LogLevel::Warn => nearclip_core::LogLevel::Warn,
        LogLevel::Info => nearclip_core::LogLevel::Info,
        LogLevel::Debug => nearclip_core::LogLevel::Debug,
        LogLevel::Trace => nearclip_core::LogLevel::Trace,
    };
    nearclip_core::init_logging(core_level);
}

/// Flush log buffers
///
/// Ensures all pending logs are written to output.
pub fn flush_logs() {
    nearclip_core::flush_logs();
}

// ============================================================
// FFI Enums
// ============================================================

/// Log level for FFI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// ============================================================
// FFI Device Types
// ============================================================

/// Device information for FFI
#[derive(Debug, Clone)]
pub struct FfiDeviceInfo {
    pub id: String,
    pub name: String,
    pub platform: DevicePlatform,
    pub status: DeviceStatus,
}

impl From<DeviceInfo> for FfiDeviceInfo {
    fn from(device: DeviceInfo) -> Self {
        Self {
            id: device.id().to_string(),
            name: device.name().to_string(),
            platform: device.platform(),
            status: device.status(),
        }
    }
}

impl From<FfiDeviceInfo> for DeviceInfo {
    fn from(ffi: FfiDeviceInfo) -> Self {
        DeviceInfo::new(ffi.id, ffi.name)
            .with_platform(ffi.platform)
            .with_status(ffi.status)
    }
}

// ============================================================
// FFI Config Type
// ============================================================

/// Configuration for FFI
#[derive(Debug, Clone)]
pub struct FfiNearClipConfig {
    pub device_name: String,
    /// 设备唯一标识（可选，用于持久化）
    /// 如果为空字符串，则自动生成新的 ID
    pub device_id: String,
    pub wifi_enabled: bool,
    pub ble_enabled: bool,
    pub auto_connect: bool,
    pub connection_timeout_secs: u64,
    pub heartbeat_interval_secs: u64,
    pub max_retries: u32,
}

impl From<FfiNearClipConfig> for NearClipConfig {
    fn from(ffi: FfiNearClipConfig) -> Self {
        NearClipConfig::new(ffi.device_name)
            .with_device_id(ffi.device_id)
            .with_wifi_enabled(ffi.wifi_enabled)
            .with_ble_enabled(ffi.ble_enabled)
            .with_auto_connect(ffi.auto_connect)
            .with_connection_timeout(Duration::from_secs(ffi.connection_timeout_secs))
            .with_heartbeat_interval(Duration::from_secs(ffi.heartbeat_interval_secs))
            .with_max_retries(ffi.max_retries)
    }
}

impl Default for FfiNearClipConfig {
    fn default() -> Self {
        Self {
            device_name: "NearClip Device".to_string(),
            device_id: String::new(), // 空字符串表示自动生成
            wifi_enabled: true,
            ble_enabled: true,
            auto_connect: true,
            connection_timeout_secs: 30,
            heartbeat_interval_secs: 10,
            max_retries: 3,
        }
    }
}

// ============================================================
// FFI Callback Trait
// ============================================================

/// Callback interface for FFI
///
/// Platform clients implement this trait to receive events.
pub trait FfiNearClipCallback: Send + Sync {
    /// Called when a device connects
    fn on_device_connected(&self, device: FfiDeviceInfo);

    /// Called when a device disconnects
    fn on_device_disconnected(&self, device_id: String);

    /// Called when a remote device requests to unpair
    fn on_device_unpaired(&self, device_id: String);

    /// Called when a pairing request is rejected by the remote device
    ///
    /// This happens when the remote device doesn't have us in their paired list.
    /// The user should remove this device and re-pair.
    fn on_pairing_rejected(&self, device_id: String, reason: String);

    /// Called when clipboard content is received
    fn on_clipboard_received(&self, content: Vec<u8>, from_device: String);

    /// Called when a sync error occurs
    fn on_sync_error(&self, error_message: String);
}

// ============================================================
// FFI BLE Sender Trait
// ============================================================

/// BLE sender callback interface
///
/// Platform clients implement this trait to provide BLE send capability.
/// This allows the Rust transport layer to send data over BLE through
/// platform-native BLE APIs.
pub trait FfiBleSender: Send + Sync {
    /// Send raw data over BLE to a device
    ///
    /// # Arguments
    /// * `device_id` - The target device ID
    /// * `data` - Raw bytes to send
    ///
    /// # Returns
    /// Empty string on success, error message on failure
    fn send_ble_data(&self, device_id: String, data: Vec<u8>) -> String;

    /// Check if BLE is connected to a device
    fn is_ble_connected(&self, device_id: String) -> bool;

    /// Get the negotiated MTU for a device
    ///
    /// Return 0 to use the default MTU.
    fn get_mtu(&self, device_id: String) -> u32;
}

/// Bridge that adapts FfiBleSender to the transport layer's BleSender trait
struct BleSenderBridge {
    ffi_sender: Box<dyn FfiBleSender>,
}

impl BleSenderBridge {
    fn new(ffi_sender: Box<dyn FfiBleSender>) -> Self {
        Self { ffi_sender }
    }
}

impl BleSender for BleSenderBridge {
    fn send_ble_data(&self, device_id: &str, data: &[u8]) -> Result<(), String> {
        let result = self.ffi_sender
            .send_ble_data(device_id.to_string(), data.to_vec());
        if result.is_empty() {
            Ok(())
        } else {
            Err(result)
        }
    }

    fn is_ble_connected(&self, device_id: &str) -> bool {
        self.ffi_sender.is_ble_connected(device_id.to_string())
    }

    fn get_mtu(&self, device_id: &str) -> usize {
        self.ffi_sender.get_mtu(device_id.to_string()) as usize
    }
}

/// Bridge callback that converts between FFI and core callbacks
struct CallbackBridge {
    ffi_callback: Arc<dyn FfiNearClipCallback>,
}

impl CallbackBridge {
    fn new(ffi_callback: Arc<dyn FfiNearClipCallback>) -> Self {
        Self { ffi_callback }
    }
}

impl NearClipCallback for CallbackBridge {
    fn on_device_connected(&self, device: &DeviceInfo) {
        self.ffi_callback.on_device_connected(FfiDeviceInfo::from(device.clone()));
    }

    fn on_device_disconnected(&self, device_id: &str) {
        self.ffi_callback.on_device_disconnected(device_id.to_string());
    }

    fn on_device_unpaired(&self, device_id: &str) {
        self.ffi_callback.on_device_unpaired(device_id.to_string());
    }

    fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
        self.ffi_callback.on_clipboard_received(content.to_vec(), from_device.to_string());
    }

    fn on_sync_error(&self, error: &NearClipError) {
        self.ffi_callback.on_sync_error(error.to_string());
    }

    fn on_pairing_rejected(&self, device_id: &str, reason: &str) {
        self.ffi_callback.on_pairing_rejected(device_id.to_string(), reason.to_string());
    }
}

// ============================================================
// FFI Manager
// ============================================================

/// Main NearClip manager for FFI
///
/// This is the main entry point for platform clients.
pub struct FfiNearClipManager {
    inner: NearClipManager,
    runtime: tokio::runtime::Runtime,
    /// BLE sender bridge (set by platform)
    ble_sender: RwLock<Option<Arc<BleSenderBridge>>>,
    /// BLE transports per device
    ble_transports: RwLock<HashMap<String, Arc<BleTransport>>>,
    /// BLE receive tasks per device
    ble_recv_tasks: RwLock<HashMap<String, JoinHandle<()>>>,
    /// Callback for BLE message handling
    callback: Arc<dyn FfiNearClipCallback>,
}

impl FfiNearClipManager {
    /// Create a new manager instance
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the manager
    /// * `callback` - Callback for receiving events
    ///
    /// # Errors
    ///
    /// Returns error if configuration validation fails.
    pub fn new(
        config: FfiNearClipConfig,
        callback: Box<dyn FfiNearClipCallback>,
    ) -> Result<Self, NearClipError> {
        let core_config: NearClipConfig = config.into();

        // Wrap callback in Arc for sharing
        let callback: Arc<dyn FfiNearClipCallback> = callback.into();
        let bridge = Arc::new(CallbackBridge::new(callback.clone()));

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| NearClipError::Io(e.to_string()))?;

        let inner = NearClipManager::new(core_config, bridge)?;

        Ok(Self {
            inner,
            runtime,
            ble_sender: RwLock::new(None),
            ble_transports: RwLock::new(HashMap::new()),
            ble_recv_tasks: RwLock::new(HashMap::new()),
            callback,
        })
    }

    /// Start the manager
    ///
    /// Starts all background services (mDNS, TCP, BLE).
    pub fn start(&self) -> Result<(), NearClipError> {
        self.runtime.block_on(async { self.inner.start().await })
    }

    /// Stop the manager
    ///
    /// Stops all background services and disconnects devices.
    pub fn stop(&self) {
        self.runtime.block_on(async { self.inner.stop().await })
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.inner.is_running()
    }

    /// Sync clipboard content to all connected devices
    ///
    /// # Arguments
    ///
    /// * `content` - Clipboard content bytes
    pub fn sync_clipboard(&self, content: Vec<u8>) -> Result<(), NearClipError> {
        tracing::info!(content_size = content.len(), "FFI sync_clipboard called");
        let result = self.runtime.block_on(async { self.inner.sync_clipboard(&content).await });
        match &result {
            Ok(_) => tracing::info!("FFI sync_clipboard completed successfully"),
            Err(e) => tracing::error!(error = %e, "FFI sync_clipboard failed"),
        }
        result
    }

    /// Get list of paired devices
    pub fn get_paired_devices(&self) -> Vec<FfiDeviceInfo> {
        self.inner
            .get_paired_devices()
            .into_iter()
            .map(FfiDeviceInfo::from)
            .collect()
    }

    /// Get list of connected devices
    pub fn get_connected_devices(&self) -> Vec<FfiDeviceInfo> {
        tracing::info!("FFI get_connected_devices called");
        let devices = self.inner.get_connected_devices();
        tracing::info!(count = devices.len(), "FFI get_connected_devices returning");
        devices.into_iter().map(FfiDeviceInfo::from).collect()
    }

    /// Connect to a device
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of the device to connect
    pub fn connect_device(&self, device_id: String) -> Result<(), NearClipError> {
        self.runtime.block_on(async { self.inner.connect_device(&device_id).await })
    }

    /// Disconnect from a device
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of the device to disconnect
    pub fn disconnect_device(&self, device_id: String) -> Result<(), NearClipError> {
        self.runtime.block_on(async { self.inner.disconnect_device(&device_id).await })
    }

    /// Add a paired device
    ///
    /// # Arguments
    ///
    /// * `device` - Device information to add
    pub fn add_paired_device(&self, device: FfiDeviceInfo) {
        self.inner.add_paired_device(device.into());
    }

    /// Remove a paired device
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of the device to remove
    pub fn remove_paired_device(&self, device_id: String) {
        self.inner.remove_paired_device(&device_id);
    }

    /// Unpair a device (send notification and remove)
    ///
    /// Sends an unpair notification to the target device before removing it.
    /// The remote device will also remove this device from its paired list.
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of the device to unpair
    pub fn unpair_device(&self, device_id: String) -> Result<(), NearClipError> {
        self.runtime.block_on(async { self.inner.unpair_device(&device_id).await })
    }

    /// Get the status of a device
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of the device
    ///
    /// # Returns
    ///
    /// Device status if found, None otherwise.
    pub fn get_device_status(&self, device_id: String) -> Option<DeviceStatus> {
        self.inner.get_device_status(&device_id)
    }

    /// Get this device's unique ID
    ///
    /// # Returns
    ///
    /// The device ID used for mDNS advertising and message identification.
    pub fn get_device_id(&self) -> String {
        self.inner.device_id().to_string()
    }

    /// Try to connect to all discovered paired devices
    ///
    /// Scans for paired devices on the network and attempts to connect.
    ///
    /// # Returns
    ///
    /// Number of devices successfully connected.
    pub fn try_connect_paired_devices(&self) -> u32 {
        self.runtime.block_on(async { self.inner.try_connect_paired_devices().await }) as u32
    }

    // ============================================================
    // BLE Methods
    // ============================================================

    /// Set the BLE sender
    ///
    /// Platform clients call this to provide BLE send capability.
    /// This must be called before BLE communication can work.
    ///
    /// # Arguments
    ///
    /// * `sender` - Platform implementation of FfiBleSender
    pub fn set_ble_sender(&self, sender: Box<dyn FfiBleSender>) {
        let bridge = Arc::new(BleSenderBridge::new(sender));
        self.runtime.block_on(async {
            let mut ble_sender = self.ble_sender.write().await;
            *ble_sender = Some(bridge);
        });
        tracing::info!("BLE sender set");
    }

    /// Called by platform when BLE data is received
    ///
    /// Platform clients call this when they receive BLE data from a device.
    /// The data will be processed and reassembled into complete messages.
    ///
    /// # Arguments
    ///
    /// * `device_id` - The device ID that sent the data
    /// * `data` - Raw bytes received from BLE
    pub fn on_ble_data_received(&self, device_id: String, data: Vec<u8>) {
        self.runtime.block_on(async {
            let transports = self.ble_transports.read().await;
            if let Some(transport) = transports.get(&device_id) {
                transport.on_data_received(&data).await;
            } else {
                // Create a new transport if we have a sender
                let sender = self.ble_sender.read().await;
                if let Some(ref sender) = *sender {
                    drop(transports);
                    let transport = Arc::new(BleTransport::new(device_id.clone(), sender.clone()));
                    transport.on_data_received(&data).await;

                    // Start a receive task for this transport
                    let transport_for_task = transport.clone();
                    let callback = self.callback.clone();
                    let device_id_for_task = device_id.clone();

                    let recv_task = tokio::spawn(async move {
                        tracing::info!(device_id = %device_id_for_task, "BLE receive task started");
                        loop {
                            match transport_for_task.recv().await {
                                Ok(message) => {
                                    tracing::debug!(
                                        device_id = %device_id_for_task,
                                        msg_type = ?message.msg_type,
                                        "BLE message received"
                                    );

                                    match message.msg_type {
                                        MessageType::ClipboardSync => {
                                            tracing::info!(
                                                from = %message.device_id,
                                                size = message.payload.len(),
                                                "BLE clipboard received"
                                            );
                                            callback.on_clipboard_received(
                                                message.payload.clone(),
                                                message.device_id.clone(),
                                            );
                                        }
                                        MessageType::Unpair => {
                                            tracing::info!(
                                                from = %message.device_id,
                                                "BLE unpair notification received"
                                            );
                                            callback.on_device_unpaired(message.device_id.clone());
                                            break;
                                        }
                                        _ => {
                                            tracing::debug!(
                                                msg_type = ?message.msg_type,
                                                "Unhandled BLE message type"
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!(
                                        device_id = %device_id_for_task,
                                        error = %e,
                                        "BLE receive error, stopping task"
                                    );
                                    break;
                                }
                            }
                        }
                        tracing::info!(device_id = %device_id_for_task, "BLE receive task ended");
                    });

                    let mut transports = self.ble_transports.write().await;
                    transports.insert(device_id.clone(), transport);

                    let mut recv_tasks = self.ble_recv_tasks.write().await;
                    recv_tasks.insert(device_id, recv_task);
                } else {
                    tracing::warn!(
                        device_id = %device_id,
                        "Received BLE data but no BLE sender is set"
                    );
                }
            }
        });
    }

    /// Called by platform when BLE connection state changes
    ///
    /// Platform clients call this when a BLE connection is established or lost.
    ///
    /// # Arguments
    ///
    /// * `device_id` - The device ID
    /// * `connected` - Whether the device is now connected
    pub fn on_ble_connection_changed(&self, device_id: String, connected: bool) {
        tracing::info!(device_id = %device_id, connected = connected, "on_ble_connection_changed");
        self.runtime.block_on(async {
            if connected {
                // Create BLE transport if we have a sender
                let sender = self.ble_sender.read().await;
                if let Some(ref sender) = *sender {
                    let transport = Arc::new(BleTransport::new(device_id.clone(), sender.clone()));
                    transport.on_connection_state_changed(true);

                    // Start a receive task for this transport
                    let transport_for_task = transport.clone();
                    let callback = self.callback.clone();
                    let device_id_for_task = device_id.clone();

                    let recv_task = tokio::spawn(async move {
                        tracing::info!(device_id = %device_id_for_task, "BLE receive task started");
                        loop {
                            match transport_for_task.recv().await {
                                Ok(message) => {
                                    tracing::debug!(
                                        device_id = %device_id_for_task,
                                        msg_type = ?message.msg_type,
                                        "BLE message received"
                                    );

                                    match message.msg_type {
                                        MessageType::ClipboardSync => {
                                            tracing::info!(
                                                from = %message.device_id,
                                                size = message.payload.len(),
                                                "BLE clipboard received"
                                            );
                                            callback.on_clipboard_received(
                                                message.payload.clone(),
                                                message.device_id.clone(),
                                            );
                                        }
                                        MessageType::Unpair => {
                                            tracing::info!(
                                                from = %message.device_id,
                                                "BLE unpair notification received"
                                            );
                                            callback.on_device_unpaired(message.device_id.clone());
                                            break;
                                        }
                                        _ => {
                                            tracing::debug!(
                                                msg_type = ?message.msg_type,
                                                "Unhandled BLE message type"
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!(
                                        device_id = %device_id_for_task,
                                        error = %e,
                                        "BLE receive error, stopping task"
                                    );
                                    break;
                                }
                            }
                        }
                        tracing::info!(device_id = %device_id_for_task, "BLE receive task ended");
                    });

                    let mut transports = self.ble_transports.write().await;
                    transports.insert(device_id.clone(), transport.clone());

                    let mut recv_tasks = self.ble_recv_tasks.write().await;
                    recv_tasks.insert(device_id.clone(), recv_task);

                    tracing::info!(device_id = %device_id, "BLE transport created and registered");

                    // Register the BLE transport with the core manager's TransportManager
                    self.inner.add_ble_transport(&device_id, transport.clone()).await;

                    // Notify callback that device is connected
                    // Get device info from paired devices if available
                    let paired_devices = self.inner.get_paired_devices();
                    let device_info = paired_devices.iter()
                        .find(|d| d.id() == device_id)
                        .cloned()
                        .unwrap_or_else(|| {
                            DeviceInfo::new(device_id.clone(), "BLE Device".to_string())
                                .with_status(DeviceStatus::Connected)
                        });

                    let mut device_info = device_info;
                    device_info.set_status(DeviceStatus::Connected);

                    self.callback.on_device_connected(FfiDeviceInfo::from(device_info));
                } else {
                    tracing::warn!(
                        device_id = %device_id,
                        "BLE connection changed but no BLE sender is set"
                    );
                }
            } else {
                // Disconnected
                let transports = self.ble_transports.read().await;
                if let Some(transport) = transports.get(&device_id) {
                    transport.on_connection_state_changed(false);
                }
                drop(transports);

                // Stop receive task
                let mut recv_tasks = self.ble_recv_tasks.write().await;
                if let Some(task) = recv_tasks.remove(&device_id) {
                    task.abort();
                    tracing::debug!(device_id = %device_id, "BLE receive task aborted");
                }

                // Remove transport
                let mut transports = self.ble_transports.write().await;
                transports.remove(&device_id);
                tracing::info!(device_id = %device_id, "BLE transport removed");

                // Remove from core manager's TransportManager
                self.inner.remove_ble_transport(&device_id).await;

                // Notify callback
                self.callback.on_device_disconnected(device_id);
            }
        });
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use std::sync::Mutex;

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
    }

    impl FfiNearClipCallback for TestCallback {
        fn on_device_connected(&self, device: FfiDeviceInfo) {
            self.connected.lock().unwrap().push(device.id);
        }

        fn on_device_disconnected(&self, device_id: String) {
            self.disconnected.lock().unwrap().push(device_id);
        }

        fn on_device_unpaired(&self, device_id: String) {
            // Treat unpair as disconnect for test purposes
            self.disconnected.lock().unwrap().push(device_id);
        }

        fn on_clipboard_received(&self, content: Vec<u8>, from_device: String) {
            self.clipboard.lock().unwrap().push((content, from_device));
        }

        fn on_sync_error(&self, error_message: String) {
            self.errors.lock().unwrap().push(error_message);
        }

        fn on_pairing_rejected(&self, device_id: String, _reason: String) {
            // Treat rejection as a form of disconnect for test purposes
            self.disconnected.lock().unwrap().push(device_id);
        }
    }

    #[test]
    fn test_ffi_config_default() {
        let config = FfiNearClipConfig::default();
        assert_eq!(config.device_name, "NearClip Device");
        assert!(config.wifi_enabled);
        assert!(config.ble_enabled);
        assert!(config.auto_connect);
    }

    #[test]
    fn test_ffi_device_info_conversion() {
        let ffi = FfiDeviceInfo {
            id: "test-id".to_string(),
            name: "Test Device".to_string(),
            platform: DevicePlatform::MacOS,
            status: DeviceStatus::Connected,
        };

        let core: DeviceInfo = ffi.clone().into();
        assert_eq!(core.id(), "test-id");
        assert_eq!(core.name(), "Test Device");
        assert_eq!(core.platform(), DevicePlatform::MacOS);
        assert_eq!(core.status(), DeviceStatus::Connected);

        let back: FfiDeviceInfo = core.into();
        assert_eq!(back.id, ffi.id);
        assert_eq!(back.name, ffi.name);
    }

    #[test]
    fn test_ffi_config_conversion() {
        let ffi = FfiNearClipConfig {
            device_name: "My Mac".to_string(),
            device_id: "TEST-DEVICE-ID".to_string(),
            wifi_enabled: true,
            ble_enabled: false,
            auto_connect: true,
            connection_timeout_secs: 60,
            heartbeat_interval_secs: 15,
            max_retries: 5,
        };

        let core: NearClipConfig = ffi.into();
        assert_eq!(core.device_name(), "My Mac");
        assert_eq!(core.device_id(), Some("TEST-DEVICE-ID"));
        assert!(core.wifi_enabled());
        assert!(!core.ble_enabled());
        assert!(core.auto_connect());
        assert_eq!(core.connection_timeout(), Duration::from_secs(60));
        assert_eq!(core.heartbeat_interval(), Duration::from_secs(15));
        assert_eq!(core.max_retries(), 5);
    }

    #[test]
    fn test_ffi_config_empty_device_id() {
        let ffi = FfiNearClipConfig {
            device_name: "My Mac".to_string(),
            device_id: "".to_string(), // 空字符串
            ..Default::default()
        };

        let core: NearClipConfig = ffi.into();
        assert_eq!(core.device_id(), None); // 空字符串转换为 None
    }

    #[test]
    fn test_log_level_conversion() {
        // Test all log levels
        init_logging(LogLevel::Error);
        init_logging(LogLevel::Warn);
        init_logging(LogLevel::Info);
        init_logging(LogLevel::Debug);
        init_logging(LogLevel::Trace);
    }

    #[test]
    fn test_ffi_manager_creation() {
        let config = FfiNearClipConfig::default();
        let callback = Box::new(TestCallback::new());

        let manager = FfiNearClipManager::new(config, callback);
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_ffi_manager_invalid_config() {
        let config = FfiNearClipConfig {
            device_name: "".to_string(), // Invalid: empty name
            ..Default::default()
        };
        let callback = Box::new(TestCallback::new());

        let result = FfiNearClipManager::new(config, callback);
        assert!(result.is_err());
    }

    #[test]
    fn test_ffi_manager_lifecycle() {
        let config = FfiNearClipConfig::default();
        let callback = Box::new(TestCallback::new());

        let manager = FfiNearClipManager::new(config, callback).unwrap();

        assert!(!manager.is_running());

        manager.start().unwrap();
        assert!(manager.is_running());

        manager.stop();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_ffi_manager_device_management() {
        let config = FfiNearClipConfig::default();
        let callback = Box::new(TestCallback::new());
        let manager = FfiNearClipManager::new(config, callback).unwrap();

        // Add device
        let device = FfiDeviceInfo {
            id: "d1".to_string(),
            name: "Device 1".to_string(),
            platform: DevicePlatform::MacOS,
            status: DeviceStatus::Disconnected,
        };
        manager.add_paired_device(device);

        // Check device list
        let devices = manager.get_paired_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id, "d1");

        // Get device status
        let status = manager.get_device_status("d1".to_string());
        assert_eq!(status, Some(DeviceStatus::Disconnected));

        // Remove device
        manager.remove_paired_device("d1".to_string());
        let devices = manager.get_paired_devices();
        assert_eq!(devices.len(), 0);
    }

    #[test]
    fn test_flush_logs() {
        init_logging(LogLevel::Debug);
        flush_logs(); // Should not panic
    }
}
