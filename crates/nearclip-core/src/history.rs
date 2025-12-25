//! Sync History Manager
//!
//! Manages synchronization history using SQLite for persistent storage.
//! Replaces platform-specific history storage implementations.

use crate::error::{NearClipError, Result};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Database schema version for migrations
const SCHEMA_VERSION: i32 = 1;

/// Sync history entry
#[derive(Debug, Clone)]
pub struct SyncHistoryEntry {
    /// Unique ID
    pub id: i64,

    /// Device ID that sent/received the sync
    pub device_id: String,

    /// Device name
    pub device_name: String,

    /// Content preview (first 100 chars)
    pub content_preview: String,

    /// Content size in bytes
    pub content_size: usize,

    /// Direction: "sent" or "received"
    pub direction: String,

    /// Timestamp (milliseconds since UNIX epoch)
    pub timestamp_ms: i64,

    /// Success status
    pub success: bool,

    /// Error message (if failed)
    pub error_message: Option<String>,
}

/// History manager for sync operations
pub struct HistoryManager {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl HistoryManager {
    /// Create a new history manager
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to SQLite database file
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)
            .map_err(|e| NearClipError::Io(format!("Failed to open database: {}", e)))?;

        let manager = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        };
        manager.init_database()?;
        Ok(manager)
    }

    /// Initialize database schema
    fn init_database(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        // Check schema version
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
            [],
        )
        .map_err(|e| NearClipError::Io(format!("Failed to create schema_version table: {}", e)))?;

        let current_version: i32 = conn
            .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        if current_version < SCHEMA_VERSION {
            // Create sync_history table
            conn.execute(
                "CREATE TABLE IF NOT EXISTS sync_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    device_id TEXT NOT NULL,
                    device_name TEXT NOT NULL,
                    content_preview TEXT NOT NULL,
                    content_size INTEGER NOT NULL,
                    direction TEXT NOT NULL,
                    timestamp_ms INTEGER NOT NULL,
                    success INTEGER NOT NULL,
                    error_message TEXT
                )",
                [],
            )
            .map_err(|e| NearClipError::Io(format!("Failed to create sync_history table: {}", e)))?;

            // Create indexes for efficient queries
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_timestamp ON sync_history(timestamp_ms DESC)",
                [],
            )
            .map_err(|e| NearClipError::Io(format!("Failed to create timestamp index: {}", e)))?;

            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_device ON sync_history(device_id)",
                [],
            )
            .map_err(|e| NearClipError::Io(format!("Failed to create device index: {}", e)))?;

            // Update schema version
            conn.execute("DELETE FROM schema_version", [])
                .map_err(|e| NearClipError::Io(format!("Failed to clear schema_version: {}", e)))?;
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?)",
                params![SCHEMA_VERSION],
            )
            .map_err(|e| NearClipError::Io(format!("Failed to update schema_version: {}", e)))?;
        }

        tracing::info!(path = ?self.db_path, version = SCHEMA_VERSION, "History database initialized");
        Ok(())
    }

    /// Add a sync history entry
    pub fn add_entry(&self, entry: SyncHistoryEntry) -> Result<i64> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        conn.execute(
            "INSERT INTO sync_history (device_id, device_name, content_preview, content_size, direction, timestamp_ms, success, error_message)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                entry.device_id,
                entry.device_name,
                entry.content_preview,
                entry.content_size as i64,
                entry.direction,
                entry.timestamp_ms,
                entry.success as i32,
                entry.error_message,
            ],
        )
        .map_err(|e| NearClipError::Io(format!("Failed to insert history entry: {}", e)))?;

        let id = conn.last_insert_rowid();

        tracing::debug!(
            id = id,
            device_id = %entry.device_id,
            direction = %entry.direction,
            size = entry.content_size,
            "Added sync history entry"
        );

        Ok(id)
    }

    /// Get recent history entries
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of entries to return
    pub fn get_recent(&self, limit: usize) -> Result<Vec<SyncHistoryEntry>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        let mut stmt = conn
            .prepare("SELECT id, device_id, device_name, content_preview, content_size, direction, timestamp_ms, success, error_message FROM sync_history ORDER BY timestamp_ms DESC LIMIT ?")
            .map_err(|e| NearClipError::Io(format!("Failed to prepare query: {}", e)))?;

        let entries = stmt
            .query_map(params![limit as i64], |row| {
                Ok(SyncHistoryEntry {
                    id: row.get(0)?,
                    device_id: row.get(1)?,
                    device_name: row.get(2)?,
                    content_preview: row.get(3)?,
                    content_size: row.get::<_, i64>(4)? as usize,
                    direction: row.get(5)?,
                    timestamp_ms: row.get(6)?,
                    success: row.get::<_, i32>(7)? != 0,
                    error_message: row.get(8)?,
                })
            })
            .map_err(|e| NearClipError::Io(format!("Failed to query history: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| NearClipError::Io(format!("Failed to collect history entries: {}", e)))?;

        Ok(entries)
    }

    /// Get history entries for a specific device
    pub fn get_by_device(&self, device_id: &str, limit: usize) -> Result<Vec<SyncHistoryEntry>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        let mut stmt = conn
            .prepare("SELECT id, device_id, device_name, content_preview, content_size, direction, timestamp_ms, success, error_message FROM sync_history WHERE device_id = ? ORDER BY timestamp_ms DESC LIMIT ?")
            .map_err(|e| NearClipError::Io(format!("Failed to prepare query: {}", e)))?;

        let entries = stmt
            .query_map(params![device_id, limit as i64], |row| {
                Ok(SyncHistoryEntry {
                    id: row.get(0)?,
                    device_id: row.get(1)?,
                    device_name: row.get(2)?,
                    content_preview: row.get(3)?,
                    content_size: row.get::<_, i64>(4)? as usize,
                    direction: row.get(5)?,
                    timestamp_ms: row.get(6)?,
                    success: row.get::<_, i32>(7)? != 0,
                    error_message: row.get(8)?,
                })
            })
            .map_err(|e| NearClipError::Io(format!("Failed to query history by device: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| NearClipError::Io(format!("Failed to collect history entries: {}", e)))?;

        Ok(entries)
    }

    /// Clear all history
    pub fn clear_all(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        conn.execute("DELETE FROM sync_history", [])
            .map_err(|e| NearClipError::Io(format!("Failed to clear history: {}", e)))?;

        tracing::info!("Cleared all sync history");
        Ok(())
    }

    /// Clear history older than specified days
    pub fn clear_older_than(&self, days: u32) -> Result<usize> {
        let cutoff_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| NearClipError::Io(e.to_string()))?
            .as_millis() as i64
            - (days as i64 * 24 * 60 * 60 * 1000);

        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        let deleted = conn
            .execute(
                "DELETE FROM sync_history WHERE timestamp_ms < ?",
                params![cutoff_ms],
            )
            .map_err(|e| NearClipError::Io(format!("Failed to delete old history: {}", e)))?;

        tracing::info!(days = days, deleted = deleted, "Cleared old sync history");
        Ok(deleted)
    }

    /// Get total entry count
    pub fn get_count(&self) -> Result<usize> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| NearClipError::Io(format!("Failed to lock database: {}", e)))?;

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM sync_history", [], |row| row.get(0))
            .map_err(|e| NearClipError::Io(format!("Failed to count history: {}", e)))?;

        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn create_test_entry(device_id: &str, direction: &str) -> SyncHistoryEntry {
        SyncHistoryEntry {
            id: 0,
            device_id: device_id.to_string(),
            device_name: "Test Device".to_string(),
            content_preview: "Hello World".to_string(),
            content_size: 11,
            direction: direction.to_string(),
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            success: true,
            error_message: None,
        }
    }

    #[test]
    fn test_history_manager_creation() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone());
        assert!(manager.is_ok());

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_add_and_get_entry() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_add_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        let entry = create_test_entry("device-1", "sent");
        let id = manager.add_entry(entry).unwrap();
        assert!(id > 0);

        let entries = manager.get_recent(10).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].device_id, "device-1");
        assert_eq!(entries[0].direction, "sent");

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_get_by_device() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_device_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        // Add entries for different devices
        manager.add_entry(create_test_entry("device-1", "sent")).unwrap();
        manager.add_entry(create_test_entry("device-2", "received")).unwrap();
        manager.add_entry(create_test_entry("device-1", "received")).unwrap();

        let device1_entries = manager.get_by_device("device-1", 10).unwrap();
        assert_eq!(device1_entries.len(), 2);

        let device2_entries = manager.get_by_device("device-2", 10).unwrap();
        assert_eq!(device2_entries.len(), 1);

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_clear_all() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_clear_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        manager.add_entry(create_test_entry("device-1", "sent")).unwrap();
        manager.add_entry(create_test_entry("device-2", "received")).unwrap();

        assert_eq!(manager.get_count().unwrap(), 2);

        manager.clear_all().unwrap();
        assert_eq!(manager.get_count().unwrap(), 0);

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_clear_older_than() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_old_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        // Add a recent entry
        manager.add_entry(create_test_entry("device-1", "sent")).unwrap();

        // Add an old entry (manually insert with old timestamp)
        {
            let conn = manager.conn.lock().unwrap();
            let old_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64
                - (10 * 24 * 60 * 60 * 1000); // 10 days ago

            conn.execute(
                "INSERT INTO sync_history (device_id, device_name, content_preview, content_size, direction, timestamp_ms, success, error_message)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                params!["device-old", "Old Device", "Old content", 11i64, "sent", old_timestamp, 1, Option::<String>::None],
            ).unwrap();
        }

        assert_eq!(manager.get_count().unwrap(), 2);

        // Clear entries older than 5 days
        let deleted = manager.clear_older_than(5).unwrap();
        assert_eq!(deleted, 1);
        assert_eq!(manager.get_count().unwrap(), 1);

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_get_count() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_count_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        assert_eq!(manager.get_count().unwrap(), 0);

        manager.add_entry(create_test_entry("device-1", "sent")).unwrap();
        assert_eq!(manager.get_count().unwrap(), 1);

        manager.add_entry(create_test_entry("device-2", "received")).unwrap();
        assert_eq!(manager.get_count().unwrap(), 2);

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_entry_with_error() {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join(format!("test_history_error_{}.db", uuid::Uuid::new_v4()));

        let manager = HistoryManager::new(db_path.clone()).unwrap();

        let entry = SyncHistoryEntry {
            id: 0,
            device_id: "device-1".to_string(),
            device_name: "Test Device".to_string(),
            content_preview: "Failed content".to_string(),
            content_size: 14,
            direction: "sent".to_string(),
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            success: false,
            error_message: Some("Connection timeout".to_string()),
        };

        manager.add_entry(entry).unwrap();

        let entries = manager.get_recent(10).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].success);
        assert_eq!(entries[0].error_message, Some("Connection timeout".to_string()));

        // Cleanup
        let _ = std::fs::remove_file(db_path);
    }
}
