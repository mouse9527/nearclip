//! BLE Controller
//!
//! Manages all BLE logic including scanning, connection management,
//! auto-reconnection, health checks, and pairing verification.
//!
//! This module centralizes BLE control logic that was previously
//! scattered across platform-specific code (macOS/Android).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::RwLock;
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};

use crate::error::BleError;

// ============================================================================
// Configuration
// ============================================================================

/// BLE Controller configuration
#[derive(Debug, Clone)]
pub struct BleControllerConfig {
    /// Scan timeout in milliseconds (0 = no timeout)
    pub scan_timeout_ms: u64,

    /// Time before device is considered lost (milliseconds)
    pub device_lost_timeout_ms: u64,

    /// Auto reconnect on disconnect
    pub auto_reconnect: bool,

    /// Max reconnect attempts
    pub max_reconnect_attempts: u32,

    /// Base delay for exponential backoff (milliseconds)
    pub reconnect_base_delay_ms: u64,

    /// Health check interval (milliseconds)
    pub health_check_interval_ms: u64,

    /// Connection timeout (milliseconds)
    pub connection_timeout_ms: u64,
}

impl Default for BleControllerConfig {
    fn default() -> Self {
        Self {
            scan_timeout_ms: 0,              // No timeout
            device_lost_timeout_ms: 30000,   // 30 seconds
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            reconnect_base_delay_ms: 1000,   // 1 second
            health_check_interval_ms: 30000, // 30 seconds
            connection_timeout_ms: 10000,    // 10 seconds
        }
    }
}

// ============================================================================
// Device Types
// ============================================================================

/// Discovered BLE device
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// Platform-specific peripheral UUID
    pub peripheral_uuid: String,

    /// NearClip device ID (from characteristic)
    pub device_id: String,

    /// Public key hash for pairing verification
    pub public_key_hash: String,

    /// Signal strength
    pub rssi: i32,

    /// Last seen timestamp (milliseconds since UNIX epoch)
    pub last_seen_ms: i64,
}

/// Connected BLE device
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields reserved for connection management and health check
struct ConnectedDevice {
    peripheral_uuid: String,
    device_id: String,
    public_key_hash: String,
    connected_at: SystemTime,
    last_activity: SystemTime,
}

/// Reconnection state
#[derive(Debug, Clone)]
struct ReconnectState {
    attempts: u32,
    next_attempt_at: SystemTime,
}

// ============================================================================
// Callback Trait
// ============================================================================

/// Callback interface for BLE controller events
pub trait BleControllerCallback: Send + Sync {
    /// Called when a device is discovered
    fn on_device_discovered(&self, device: DiscoveredDevice);

    /// Called when a device is lost (timeout)
    fn on_device_lost(&self, peripheral_uuid: String);

    /// Called when a device connects
    fn on_device_connected(&self, device_id: String);

    /// Called when a device disconnects
    fn on_device_disconnected(&self, device_id: String, reason: String);

    /// Called when data is received
    fn on_data_received(&self, device_id: String, data: Vec<u8>);

    /// Called when an error occurs
    fn on_error(&self, device_id: Option<String>, error: String);
}

// ============================================================================
// Hardware Interface
// ============================================================================

/// BLE hardware abstraction (platform implements this)
pub trait BleHardware: Send + Sync {
    /// Start scanning for NearClip devices
    fn start_scan(&self);

    /// Stop scanning
    fn stop_scan(&self);

    /// Connect to a peripheral
    fn connect(&self, peripheral_uuid: String);

    /// Disconnect from a peripheral
    fn disconnect(&self, peripheral_uuid: String);

    /// Write data to a peripheral
    /// Returns empty string on success, error message on failure
    fn write_data(&self, peripheral_uuid: String, data: Vec<u8>) -> String;

    /// Get MTU for a peripheral (0 = default)
    fn get_mtu(&self, peripheral_uuid: String) -> u32;

    /// Check if peripheral is connected
    fn is_connected(&self, peripheral_uuid: String) -> bool;

    /// Start advertising
    fn start_advertising(&self);

    /// Stop advertising
    fn stop_advertising(&self);

    /// Configure local device info
    fn configure(&self, device_id: String, public_key_hash: String);
}

// ============================================================================
// BLE Controller
// ============================================================================

/// BLE Controller - manages all BLE logic
///
/// Responsibilities:
/// - Scanning control and device discovery
/// - Connection lifecycle management
/// - Auto-reconnection with exponential backoff
/// - Health checks
/// - Pairing verification
pub struct BleController {
    /// Platform BLE hardware interface
    hardware: Arc<dyn BleHardware>,

    /// Discovered devices: peripheral_uuid -> DiscoveredDevice
    discovered_devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,

    /// Connected devices: peripheral_uuid -> ConnectedDevice
    connected_devices: Arc<RwLock<HashMap<String, ConnectedDevice>>>,

    /// Mapping: peripheral_uuid -> device_id
    uuid_to_device_id: Arc<RwLock<HashMap<String, String>>>,

    /// Mapping: device_id -> peripheral_uuid
    device_id_to_uuid: Arc<RwLock<HashMap<String, String>>>,

    /// Reconnection state: peripheral_uuid -> ReconnectState
    reconnect_state: Arc<RwLock<HashMap<String, ReconnectState>>>,

    /// Configuration
    config: BleControllerConfig,

    /// Event callback
    callback: Arc<dyn BleControllerCallback>,

    /// Scanning state
    is_scanning: Arc<RwLock<bool>>,
}

impl BleController {
    /// Create a new BLE controller
    pub fn new(
        hardware: Arc<dyn BleHardware>,
        config: BleControllerConfig,
        callback: Arc<dyn BleControllerCallback>,
    ) -> Self {
        Self {
            hardware,
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
            uuid_to_device_id: Arc::new(RwLock::new(HashMap::new())),
            device_id_to_uuid: Arc::new(RwLock::new(HashMap::new())),
            reconnect_state: Arc::new(RwLock::new(HashMap::new())),
            config,
            callback,
            is_scanning: Arc::new(RwLock::new(false)),
        }
    }

    // ========================================================================
    // Public API
    // ========================================================================

    /// Start scanning for devices
    pub async fn start_scan(&self) -> Result<(), BleError> {
        let mut scanning = self.is_scanning.write().await;
        if *scanning {
            return Ok(());
        }

        info!("Starting BLE scan");
        self.hardware.start_scan();
        *scanning = true;

        // Start device lost detection task
        self.spawn_device_lost_detection();

        Ok(())
    }

    /// Stop scanning
    pub async fn stop_scan(&self) {
        let mut scanning = self.is_scanning.write().await;
        if !*scanning {
            return;
        }

        info!("Stopping BLE scan");
        self.hardware.stop_scan();
        *scanning = false;
    }

    /// Connect to a device by device_id
    pub async fn connect(&self, device_id: &str) -> Result<(), BleError> {
        // Find peripheral_uuid from device_id
        let uuid_map = self.device_id_to_uuid.read().await;
        let peripheral_uuid = uuid_map.get(device_id)
            .ok_or_else(|| BleError::ConnectionFailed(format!("Device {} not found", device_id)))?
            .clone();
        drop(uuid_map);

        info!(device_id = %device_id, peripheral_uuid = %peripheral_uuid, "Connecting to device");
        self.hardware.connect(peripheral_uuid);

        Ok(())
    }

    /// Disconnect from a device
    pub async fn disconnect(&self, device_id: &str) -> Result<(), BleError> {
        let uuid_map = self.device_id_to_uuid.read().await;
        let peripheral_uuid = uuid_map.get(device_id)
            .ok_or_else(|| BleError::ConnectionFailed(format!("Device {} not found", device_id)))?
            .clone();
        drop(uuid_map);

        info!(device_id = %device_id, peripheral_uuid = %peripheral_uuid, "Disconnecting from device");
        self.hardware.disconnect(peripheral_uuid);

        Ok(())
    }

    /// Send data to a device (auto-chunking handled by transport layer)
    pub async fn send_data(&self, device_id: &str, data: &[u8]) -> Result<(), BleError> {
        let uuid_map = self.device_id_to_uuid.read().await;
        let peripheral_uuid = uuid_map.get(device_id)
            .ok_or_else(|| BleError::DataTransfer(format!("Device {} not found", device_id)))?
            .clone();
        drop(uuid_map);

        debug!(device_id = %device_id, size = data.len(), "Sending data");

        let result = self.hardware.write_data(peripheral_uuid, data.to_vec());
        if result.is_empty() {
            Ok(())
        } else {
            Err(BleError::DataTransfer(result))
        }
    }

    /// Get list of discovered devices
    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        let devices = self.discovered_devices.read().await;
        devices.values().cloned().collect()
    }

    /// Get list of connected device IDs
    pub async fn get_connected_devices(&self) -> Vec<String> {
        let devices = self.connected_devices.read().await;
        devices.values().map(|d| d.device_id.clone()).collect()
    }

    // ========================================================================
    // Platform Event Handlers (called by platform code)
    // ========================================================================

    /// Handle device discovered event from platform
    pub async fn handle_device_discovered(
        &self,
        peripheral_uuid: &str,
        device_id: &str,
        public_key_hash: &str,
        rssi: i32,
    ) {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        let device = DiscoveredDevice {
            peripheral_uuid: peripheral_uuid.to_string(),
            device_id: device_id.to_string(),
            public_key_hash: public_key_hash.to_string(),
            rssi,
            last_seen_ms: now_ms,
        };

        // Update mappings
        {
            let mut uuid_map = self.uuid_to_device_id.write().await;
            let mut device_map = self.device_id_to_uuid.write().await;
            uuid_map.insert(peripheral_uuid.to_string(), device_id.to_string());
            device_map.insert(device_id.to_string(), peripheral_uuid.to_string());
        }

        // Store discovered device
        {
            let mut devices = self.discovered_devices.write().await;
            devices.insert(peripheral_uuid.to_string(), device.clone());
        }

        info!(
            peripheral_uuid = %peripheral_uuid,
            device_id = %device_id,
            rssi = rssi,
            "Device discovered"
        );

        // Notify callback
        self.callback.on_device_discovered(device);
    }

    /// Handle connected event from platform
    pub async fn handle_connected(&self, peripheral_uuid: &str) {
        let uuid_map = self.uuid_to_device_id.read().await;
        let device_id = match uuid_map.get(peripheral_uuid) {
            Some(id) => id.clone(),
            None => {
                warn!(peripheral_uuid = %peripheral_uuid, "Connected but no device_id mapping");
                return;
            }
        };
        drop(uuid_map);

        // Get device info from discovered devices
        let discovered = self.discovered_devices.read().await;
        let device_info = discovered.get(peripheral_uuid).cloned();
        drop(discovered);

        if let Some(info) = device_info {
            let now = SystemTime::now();
            let connected_device = ConnectedDevice {
                peripheral_uuid: peripheral_uuid.to_string(),
                device_id: device_id.clone(),
                public_key_hash: info.public_key_hash,
                connected_at: now,
                last_activity: now,
            };

            // Add to connected devices
            {
                let mut devices = self.connected_devices.write().await;
                devices.insert(peripheral_uuid.to_string(), connected_device);
            }

            // Clear reconnect state
            {
                let mut reconnect = self.reconnect_state.write().await;
                reconnect.remove(peripheral_uuid);
            }

            info!(device_id = %device_id, "Device connected");
            self.callback.on_device_connected(device_id);
        }
    }

    /// Handle disconnected event from platform
    pub async fn handle_disconnected(&self, peripheral_uuid: &str, reason: &str) {
        let uuid_map = self.uuid_to_device_id.read().await;
        let device_id = match uuid_map.get(peripheral_uuid) {
            Some(id) => id.clone(),
            None => {
                warn!(peripheral_uuid = %peripheral_uuid, "Disconnected but no device_id mapping");
                return;
            }
        };
        drop(uuid_map);

        // Remove from connected devices
        {
            let mut devices = self.connected_devices.write().await;
            devices.remove(peripheral_uuid);
        }

        info!(device_id = %device_id, reason = %reason, "Device disconnected");
        self.callback.on_device_disconnected(device_id.clone(), reason.to_string());

        // Schedule reconnect if enabled
        if self.config.auto_reconnect {
            self.schedule_reconnect(peripheral_uuid).await;
        }
    }

    /// Handle data received event from platform
    pub async fn handle_data_received(&self, peripheral_uuid: &str, data: &[u8]) {
        let uuid_map = self.uuid_to_device_id.read().await;
        let device_id = match uuid_map.get(peripheral_uuid) {
            Some(id) => id.clone(),
            None => {
                warn!(peripheral_uuid = %peripheral_uuid, "Data received but no device_id mapping");
                return;
            }
        };
        drop(uuid_map);

        // Update last activity
        {
            let mut devices = self.connected_devices.write().await;
            if let Some(device) = devices.get_mut(peripheral_uuid) {
                device.last_activity = SystemTime::now();
            }
        }

        debug!(device_id = %device_id, size = data.len(), "Data received");
        self.callback.on_data_received(device_id, data.to_vec());
    }

    // ========================================================================
    // Internal Methods
    // ========================================================================

    /// Schedule reconnection attempt
    async fn schedule_reconnect(&self, peripheral_uuid: &str) {
        let mut reconnect = self.reconnect_state.write().await;

        let state = reconnect.entry(peripheral_uuid.to_string())
            .or_insert(ReconnectState {
                attempts: 0,
                next_attempt_at: SystemTime::now(),
            });

        if state.attempts >= self.config.max_reconnect_attempts {
            warn!(
                peripheral_uuid = %peripheral_uuid,
                attempts = state.attempts,
                "Max reconnect attempts reached"
            );
            reconnect.remove(peripheral_uuid);
            return;
        }

        // Calculate delay with exponential backoff
        let delay_ms = self.config.reconnect_base_delay_ms * (1 << state.attempts);
        let delay = Duration::from_millis(delay_ms);

        state.attempts += 1;
        state.next_attempt_at = SystemTime::now() + delay;

        info!(
            peripheral_uuid = %peripheral_uuid,
            attempt = state.attempts,
            delay_ms = delay_ms,
            "Scheduling reconnect"
        );

        // Spawn reconnect task
        let peripheral_uuid = peripheral_uuid.to_string();
        let hardware = self.hardware.clone();
        tokio::spawn(async move {
            sleep(delay).await;
            info!(peripheral_uuid = %peripheral_uuid, "Attempting reconnect");
            hardware.connect(peripheral_uuid);
        });
    }

    /// Spawn device lost detection task
    fn spawn_device_lost_detection(&self) {
        let discovered = self.discovered_devices.clone();
        let callback = self.callback.clone();
        let timeout_ms = self.config.device_lost_timeout_ms;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(timeout_ms / 2));

            loop {
                interval.tick().await;

                let now_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0);

                let mut devices = discovered.write().await;
                let mut lost_devices = Vec::new();

                // Find lost devices
                for (uuid, device) in devices.iter() {
                    let elapsed_ms = now_ms - device.last_seen_ms;
                    if elapsed_ms > timeout_ms as i64 {
                        lost_devices.push(uuid.clone());
                    }
                }

                // Remove and notify
                for uuid in lost_devices {
                    devices.remove(&uuid);
                    info!(peripheral_uuid = %uuid, "Device lost");
                    callback.on_device_lost(uuid);
                }
            }
        });
    }

    /// Start health check task
    pub fn start_health_check(&self) {
        let connected = self.connected_devices.clone();
        let hardware = self.hardware.clone();
        let callback = self.callback.clone();
        let check_interval = Duration::from_millis(self.config.health_check_interval_ms);
        let timeout = Duration::from_millis(self.config.connection_timeout_ms);

        tokio::spawn(async move {
            let mut interval = interval(check_interval);

            loop {
                interval.tick().await;

                let now = SystemTime::now();
                let devices = connected.read().await;

                for (peripheral_uuid, device) in devices.iter() {
                    // Check if still connected at hardware level
                    if !hardware.is_connected(peripheral_uuid.clone()) {
                        warn!(
                            device_id = %device.device_id,
                            "Device not connected at hardware level"
                        );
                        continue;
                    }

                    // Check activity timeout
                    if let Ok(elapsed) = now.duration_since(device.last_activity) {
                        if elapsed > timeout {
                            error!(
                                device_id = %device.device_id,
                                elapsed_secs = elapsed.as_secs(),
                                "Connection timeout"
                            );

                            // Force disconnect
                            hardware.disconnect(peripheral_uuid.clone());
                            callback.on_error(
                                Some(device.device_id.clone()),
                                "Connection timeout".to_string(),
                            );
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockHardware {
        scan_started: Mutex<bool>,
        connected: Mutex<Vec<String>>,
    }

    impl MockHardware {
        fn new() -> Self {
            Self {
                scan_started: Mutex::new(false),
                connected: Mutex::new(Vec::new()),
            }
        }
    }

    impl BleHardware for MockHardware {
        fn start_scan(&self) {
            *self.scan_started.lock().unwrap() = true;
        }

        fn stop_scan(&self) {
            *self.scan_started.lock().unwrap() = false;
        }

        fn connect(&self, peripheral_uuid: String) {
            self.connected.lock().unwrap().push(peripheral_uuid);
        }

        fn disconnect(&self, peripheral_uuid: String) {
            self.connected.lock().unwrap().retain(|u| u != &peripheral_uuid);
        }

        fn write_data(&self, _peripheral_uuid: String, _data: Vec<u8>) -> String {
            String::new()
        }

        fn get_mtu(&self, _peripheral_uuid: String) -> u32 {
            20
        }

        fn is_connected(&self, peripheral_uuid: String) -> bool {
            self.connected.lock().unwrap().contains(&peripheral_uuid)
        }

        fn start_advertising(&self) {}
        fn stop_advertising(&self) {}
        fn configure(&self, _device_id: String, _public_key_hash: String) {}
    }

    struct MockCallback {
        discovered: Mutex<Vec<String>>,
        connected: Mutex<Vec<String>>,
        disconnected: Mutex<Vec<String>>,
    }

    impl MockCallback {
        fn new() -> Self {
            Self {
                discovered: Mutex::new(Vec::new()),
                connected: Mutex::new(Vec::new()),
                disconnected: Mutex::new(Vec::new()),
            }
        }
    }

    impl BleControllerCallback for MockCallback {
        fn on_device_discovered(&self, device: DiscoveredDevice) {
            self.discovered.lock().unwrap().push(device.device_id);
        }

        fn on_device_lost(&self, _peripheral_uuid: String) {}

        fn on_device_connected(&self, device_id: String) {
            self.connected.lock().unwrap().push(device_id);
        }

        fn on_device_disconnected(&self, device_id: String, _reason: String) {
            self.disconnected.lock().unwrap().push(device_id);
        }

        fn on_data_received(&self, _device_id: String, _data: Vec<u8>) {}

        fn on_error(&self, _device_id: Option<String>, _error: String) {}
    }

    #[tokio::test]
    async fn test_ble_controller_scan() {
        let hardware = Arc::new(MockHardware::new());
        let callback = Arc::new(MockCallback::new());
        let config = BleControllerConfig::default();

        let controller = BleController::new(hardware.clone(), config, callback);

        // Start scan
        controller.start_scan().await.unwrap();
        assert!(*hardware.scan_started.lock().unwrap());

        // Stop scan
        controller.stop_scan().await;
        assert!(!*hardware.scan_started.lock().unwrap());
    }

    #[tokio::test]
    async fn test_ble_controller_device_discovery() {
        let hardware = Arc::new(MockHardware::new());
        let callback = Arc::new(MockCallback::new());
        let config = BleControllerConfig::default();

        let controller = BleController::new(hardware, config, callback.clone());

        // Simulate device discovery
        controller.handle_device_discovered(
            "uuid-1",
            "device-1",
            "hash-1",
            -50,
        ).await;

        // Check discovered devices
        let devices = controller.get_discovered_devices().await;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_id, "device-1");

        // Check callback
        assert_eq!(callback.discovered.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_ble_controller_connection() {
        let hardware = Arc::new(MockHardware::new());
        let callback = Arc::new(MockCallback::new());
        let config = BleControllerConfig::default();

        let controller = BleController::new(hardware.clone(), config, callback.clone());

        // Discover device first
        controller.handle_device_discovered(
            "uuid-1",
            "device-1",
            "hash-1",
            -50,
        ).await;

        // Connect
        controller.connect("device-1").await.unwrap();
        assert!(hardware.connected.lock().unwrap().contains(&"uuid-1".to_string()));

        // Simulate connection success
        controller.handle_connected("uuid-1").await;

        // Check connected devices
        let devices = controller.get_connected_devices().await;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0], "device-1");

        // Check callback
        assert_eq!(callback.connected.lock().unwrap().len(), 1);
    }
}
