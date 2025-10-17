//! Utility functions and helpers

use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a unique timestamp in milliseconds
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Generate a unique ID for devices or sync operations
pub fn generate_unique_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let timestamp = current_timestamp_ms();
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("{}-{}", timestamp, counter)
}

/// Validate device ID format
pub fn is_valid_device_id(device_id: &str) -> bool {
    !device_id.is_empty()
        && device_id.len() <= 64
        && device_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp_ms() {
        let ts1 = current_timestamp_ms();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let ts2 = current_timestamp_ms();
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_generate_unique_id() {
        let id1 = generate_unique_id();
        let id2 = generate_unique_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_is_valid_device_id() {
        assert!(is_valid_device_id("device-123"));
        assert!(is_valid_device_id("android_device_001"));
        assert!(!is_valid_device_id(""));
        assert!(!is_valid_device_id("device@invalid"));
        assert!(!is_valid_device_id("a".repeat(65).as_str()));
    }
}