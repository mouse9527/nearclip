//! Device storage using SQLite

use crate::{DeviceError, PairedDevice};
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// SQLite-based device storage
pub struct DeviceStore {
    db: Arc<RwLock<Connection>>,
}

impl DeviceStore {
    /// Create a new device store with the given database path
    ///
    /// # Arguments
    /// * `db_path` - Path to the SQLite database file
    ///
    /// # Returns
    /// A new `DeviceStore` instance
    pub fn new(db_path: PathBuf) -> Result<Self, DeviceError> {
        info!(path = %db_path.display(), "Creating device store");

        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent access
        // Note: Ignore errors on PRAGMA as some environments may not support all options
        let _ = conn.execute("PRAGMA journal_mode=WAL", []);
        let _ = conn.execute("PRAGMA synchronous=NORMAL", []);
        let _ = conn.execute("PRAGMA foreign_keys=ON", []);

        // Create devices table
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

        // Create indexes for common queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_devices_platform ON devices(platform)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_devices_last_seen ON devices(last_seen)",
            [],
        )?;

        info!("Device store initialized successfully");

        Ok(Self {
            db: Arc::new(RwLock::new(conn)),
        })
    }

    /// Save a device to the database (insert or update)
    pub async fn save_device(&self, device: &PairedDevice) -> Result<(), DeviceError> {
        debug!(device_id = %device.device_id, "Saving device");

        let db = self.db.read().await;
        let platform_str = serialize_platform(&device.platform);

        db.execute(
            "INSERT OR REPLACE INTO devices (id, name, platform, public_key, shared_secret, paired_at, last_connected, last_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                device.device_id,
                device.device_name,
                platform_str,
                device.public_key,
                device.shared_secret,
                device.paired_at,
                device.last_connected,
                device.last_seen,
            ],
        )?;

        info!(device_id = %device.device_id, "Device saved");
        Ok(())
    }

    /// Remove a device from the database
    pub async fn remove_device(&self, device_id: &str) -> Result<(), DeviceError> {
        debug!(device_id, "Removing device");

        let db = self.db.read().await;
        let rows_affected = db.execute("DELETE FROM devices WHERE id = ?1", params![device_id])?;

        if rows_affected == 0 {
            return Err(DeviceError::NotFound(device_id.to_string()));
        }

        info!(device_id, "Device removed");
        Ok(())
    }

    /// Load all paired devices from the database
    pub async fn load_all_devices(&self) -> Result<Vec<PairedDevice>, DeviceError> {
        debug!("Loading all devices");

        let db = self.db.read().await;
        let mut stmt = db.prepare(
            "SELECT id, name, platform, public_key, shared_secret, paired_at, last_connected, last_seen
             FROM devices"
        )?;

        let devices = stmt.query_map([], |row| {
            Ok(PairedDevice {
                device_id: row.get(0)?,
                device_name: row.get(1)?,
                platform: deserialize_platform(row.get(2)?),
                public_key: row.get(3)?,
                shared_secret: row.get(4)?,
                paired_at: row.get(5)?,
                last_connected: row.get(6)?,
                last_seen: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;

        debug!(count = devices.len(), "Devices loaded");
        Ok(devices)
    }

    /// Get a specific device by ID
    pub async fn get_device(&self, device_id: &str) -> Result<Option<PairedDevice>, DeviceError> {
        debug!(device_id, "Getting device");

        let db = self.db.read().await;
        let mut stmt = db.prepare(
            "SELECT id, name, platform, public_key, shared_secret, paired_at, last_connected, last_seen
             FROM devices WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![device_id])?;

        if let Some(row) = rows.next()? {
            let device = PairedDevice {
                device_id: row.get(0)?,
                device_name: row.get(1)?,
                platform: deserialize_platform(row.get(2)?),
                public_key: row.get(3)?,
                shared_secret: row.get(4)?,
                paired_at: row.get(5)?,
                last_connected: row.get(6)?,
                last_seen: row.get(7)?,
            };
            Ok(Some(device))
        } else {
            Ok(None)
        }
    }
}

fn serialize_platform(platform: &crate::DevicePlatform) -> &str {
    match platform {
        crate::DevicePlatform::MacOS => "macos",
        crate::DevicePlatform::Windows => "windows",
        crate::DevicePlatform::Linux => "linux",
        crate::DevicePlatform::Android => "android",
        crate::DevicePlatform::Ios => "ios",
    }
}

fn deserialize_platform(s: String) -> crate::DevicePlatform {
    match s.as_str() {
        "macos" => crate::DevicePlatform::MacOS,
        "windows" => crate::DevicePlatform::Windows,
        "linux" => crate::DevicePlatform::Linux,
        "android" => crate::DevicePlatform::Android,
        "ios" => crate::DevicePlatform::Ios,
        _ => crate::DevicePlatform::Linux, // Default fallback
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

    #[test]
    fn test_store_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let store = DeviceStore::new(db_path).unwrap();
        // Verify connection is valid by querying database version
        let db = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { store.db.read().await });
        let version: String = db.query_row("SELECT sqlite_version()", [], |row| row.get(0)).unwrap();
        assert!(!version.is_empty());
    }

    #[tokio::test]
    async fn test_save_and_load_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let store = DeviceStore::new(db_path).unwrap();
        let device = create_test_device("device-1");

        store.save_device(&device).await.unwrap();

        let loaded = store.load_all_devices().await.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].device_id, "device-1");
        assert_eq!(loaded[0].device_name, "Test Device device-1");
    }

    #[tokio::test]
    async fn test_remove_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let store = DeviceStore::new(db_path).unwrap();
        let device = create_test_device("device-1");

        store.save_device(&device).await.unwrap();
        assert_eq!(store.load_all_devices().await.unwrap().len(), 1);

        store.remove_device("device-1").await.unwrap();
        assert_eq!(store.load_all_devices().await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let store = DeviceStore::new(db_path).unwrap();
        let result = store.remove_device("nonexistent").await;

        assert!(matches!(result, Err(DeviceError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_get_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let store = DeviceStore::new(db_path).unwrap();
        let device = create_test_device("device-1");

        store.save_device(&device).await.unwrap();

        let loaded = store.get_device("device-1").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().device_id, "device-1");

        let not_found = store.get_device("nonexistent").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_update_device() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let store = DeviceStore::new(db_path).unwrap();
        let mut device = create_test_device("device-1");

        store.save_device(&device).await.unwrap();

        // Update device
        device.last_connected = Some(1703577700000);
        store.save_device(&device).await.unwrap();

        let loaded = store.load_all_devices().await.unwrap();
        assert_eq!(loaded.len(), 1); // Still only 1 device
        assert_eq!(loaded[0].last_connected, Some(1703577700000));
    }
}
