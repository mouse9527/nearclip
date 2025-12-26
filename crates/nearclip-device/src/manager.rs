//! Device manager for handling device discovery, pairing, and connection state

use crate::{DeviceError, PairedDevice, DiscoveredDevice};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Device manager handles all device-related operations
pub struct DeviceManager {
    store: crate::DeviceStore,
    discovered: Arc<RwLock<Vec<DiscoveredDevice>>>,
    paired: Arc<RwLock<Vec<PairedDevice>>>,
    connected: Arc<RwLock<Vec<String>>>, // device_ids
}

impl DeviceManager {
    /// Create a new device manager
    ///
    /// # Arguments
    /// * `db_path` - Path to the SQLite database file
    pub async fn new(db_path: PathBuf) -> Result<Self, DeviceError> {
        info!("Creating device manager");

        let store = crate::DeviceStore::new(db_path)?;

        // Load paired devices from database
        let paired = store.load_all_devices().await.unwrap_or_default();

        info!(count = paired.len(), "Loaded paired devices from database");

        Ok(Self {
            store,
            discovered: Arc::new(RwLock::new(Vec::new())),
            paired: Arc::new(RwLock::new(paired)),
            connected: Arc::new(RwLock::new(Vec::new())),
        })
    }

    // ========== Device Discovery ==========

    /// Add a newly discovered device
    pub async fn add_discovered_device(&self, device: DiscoveredDevice) {
        debug!(device_id = %device.device_id, "Adding discovered device");

        let mut discovered = self.discovered.write().await;
        if !discovered.iter().any(|d| d.device_id == device.device_id) {
            discovered.push(device);
        }
    }

    /// Get all discovered devices
    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        self.discovered.read().await.clone()
    }

    /// Clear discovered devices list
    pub async fn clear_discovered_devices(&self) {
        self.discovered.write().await.clear();
    }

    // ========== Pairing ==========

    /// Pair a device (save to database and memory)
    pub async fn pair_device(&self, device: PairedDevice) -> Result<(), DeviceError> {
        info!(device_id = %device.device_id, "Pairing device");

        self.store.save_device(&device).await?;

        let mut paired = self.paired.write().await;
        if !paired.iter().any(|d| d.device_id == device.device_id) {
            paired.push(device);
        }

        Ok(())
    }

    /// Unpair a device (remove from database and memory)
    pub async fn unpair_device(&self, device_id: &str) -> Result<(), DeviceError> {
        info!(device_id, "Unpairing device");

        self.store.remove_device(device_id).await?;

        let mut paired = self.paired.write().await;
        paired.retain(|d| d.device_id != device_id);

        // Also disconnect if connected
        let mut connected = self.connected.write().await;
        connected.retain(|id| id != device_id);

        Ok(())
    }

    // ========== Queries ==========

    /// Get all paired devices
    pub async fn get_paired_devices(&self) -> Vec<PairedDevice> {
        self.paired.read().await.clone()
    }

    /// Get a specific paired device by ID
    pub async fn get_device(&self, device_id: &str) -> Option<PairedDevice> {
        let paired = self.paired.read().await;
        paired.iter().find(|d| d.device_id == device_id).cloned()
    }

    /// Check if a device is paired
    pub async fn is_paired(&self, device_id: &str) -> bool {
        let paired = self.paired.read().await;
        paired.iter().any(|d| d.device_id == device_id)
    }

    // ========== Connection State ==========

    /// Mark a device as connected
    pub async fn mark_connected(&self, device_id: &str) {
        debug!(device_id, "Marking device as connected");

        let mut connected = self.connected.write().await;
        if !connected.contains(&device_id.to_string()) {
            connected.push(device_id.to_string());
        }
    }

    /// Mark a device as disconnected
    pub async fn mark_disconnected(&self, device_id: &str) {
        debug!(device_id, "Marking device as disconnected");

        let mut connected = self.connected.write().await;
        connected.retain(|id| id != device_id);
    }

    /// Get all currently connected devices
    pub async fn get_connected_devices(&self) -> Vec<PairedDevice> {
        let connected = self.connected.read().await;
        let paired = self.paired.read().await;

        paired
            .iter()
            .filter(|d| connected.contains(&d.device_id))
            .cloned()
            .collect()
    }

    /// Check if a device is connected
    pub async fn is_connected(&self, device_id: &str) -> bool {
        let connected = self.connected.read().await;
        connected.contains(&device_id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_device(id: &str) -> PairedDevice {
        PairedDevice {
            device_id: id.to_string(),
            device_name: format!("Test Device {}", id),
            platform: crate::DevicePlatform::MacOS,
            public_key: vec![1, 2, 3, 4],
            shared_secret: vec![5, 6, 7, 8],
            paired_at: 1703577600000,
            last_connected: None,
            last_seen: None,
        }
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let manager = DeviceManager::new(db_path).await.unwrap();
        assert_eq!(manager.get_paired_devices().await.len(), 0);
    }

    #[tokio::test]
    async fn test_pair_and_unpair_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let manager = DeviceManager::new(db_path).await.unwrap();
        let device = create_test_device("device-1");

        // Pair
        manager.pair_device(device.clone()).await.unwrap();
        assert_eq!(manager.get_paired_devices().await.len(), 1);
        assert!(manager.is_paired("device-1").await);

        // Unpair
        manager.unpair_device("device-1").await.unwrap();
        assert_eq!(manager.get_paired_devices().await.len(), 0);
        assert!(!manager.is_paired("device-1").await);
    }

    #[tokio::test]
    async fn test_connection_state() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let manager = DeviceManager::new(db_path).await.unwrap();
        let device = create_test_device("device-1");

        manager.pair_device(device).await.unwrap();

        // Connect
        manager.mark_connected("device-1").await;
        assert!(manager.is_connected("device-1").await);
        assert_eq!(manager.get_connected_devices().await.len(), 1);

        // Disconnect
        manager.mark_disconnected("device-1").await;
        assert!(!manager.is_connected("device-1").await);
        assert_eq!(manager.get_connected_devices().await.len(), 0);
    }

    #[tokio::test]
    async fn test_discovered_devices() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let manager = DeviceManager::new(db_path).await.unwrap();

        let discovered = DiscoveredDevice {
            device_id: "discovered-1".to_string(),
            device_name: "Discovered Device".to_string(),
            platform: crate::DevicePlatform::Android,
            public_key_hash: "hash123".to_string(),
            channel: crate::DiscoveryChannel::BLE {
                peripheral_id: "peripheral-1".to_string(),
                rssi: -50,
            },
        };

        manager.add_discovered_device(discovered.clone()).await;
        let discovered_list = manager.get_discovered_devices().await;
        assert_eq!(discovered_list.len(), 1);
        assert_eq!(discovered_list[0].device_id, "discovered-1");

        // Adding same device again should not duplicate
        manager.add_discovered_device(discovered.clone()).await;
        assert_eq!(manager.get_discovered_devices().await.len(), 1);

        // Clear
        manager.clear_discovered_devices().await;
        assert_eq!(manager.get_discovered_devices().await.len(), 0);
    }

    #[tokio::test]
    async fn test_get_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let manager = DeviceManager::new(db_path).await.unwrap();
        let device = create_test_device("device-1");

        manager.pair_device(device.clone()).await.unwrap();

        let retrieved = manager.get_device("device-1").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().device_name, "Test Device device-1");

        let not_found = manager.get_device("nonexistent").await;
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_persistence() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        // Create first manager and add device
        let manager1 = DeviceManager::new(db_path.clone()).await.unwrap();
        let device = create_test_device("device-1");
        manager1.pair_device(device).await.unwrap();

        // Create second manager with same db path - should load the device
        let manager2 = DeviceManager::new(db_path).await.unwrap();
        assert_eq!(manager2.get_paired_devices().await.len(), 1);
        assert!(manager2.is_paired("device-1").await);
    }
}
