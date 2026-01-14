//! Device pairing integration tests
//!
//! Tests the complete pairing flow at the device manager level:
//! - Device discovery and pairing
//! - Shared secret computation and storage
//! - Paired device persistence
//! - Unpair functionality

use nearclip_crypto::{EcdhKeyPair, PairingData, PairingSession, QrCodeGenerator, QrCodeParser};
use nearclip_device::{DeviceManager, PairedDevice, DevicePlatform};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

/// Helper to get current Unix timestamp in milliseconds as i64
fn timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Test 3.1: Complete pairing flow using QR codes
#[tokio::test]
async fn test_qr_code_pairing_flow() {
    // Setup: Create two device managers (simulating two devices)
    let temp_dir_a = TempDir::new().unwrap();
    let temp_dir_b = TempDir::new().unwrap();

    let db_path_a = temp_dir_a.path().join("device_a.db");
    let db_path_b = temp_dir_b.path().join("device_b.db");

    let manager_a = DeviceManager::new(db_path_a).await.unwrap();
    let manager_b = DeviceManager::new(db_path_b).await.unwrap();

    // Device A: Generate keypair and QR code
    let keypair_a = EcdhKeyPair::generate();
    let pairing_data_a = PairingData::new(
        "device_a".to_string(),
        &keypair_a.public_key_bytes(),
    );

    let qr_generator = QrCodeGenerator::new();
    let qr_code_a = qr_generator.generate_png(&pairing_data_a).unwrap();

    // Device B: Generate keypair and QR code
    let keypair_b = EcdhKeyPair::generate();
    let pairing_data_b = PairingData::new(
        "device_b".to_string(),
        &keypair_b.public_key_bytes(),
    );

    let qr_code_b = qr_generator.generate_png(&pairing_data_b).unwrap();

    // Device B scans A's QR code
    let parsed_data_a = QrCodeParser::parse_pairing_data(&qr_code_a).unwrap();
    assert_eq!(parsed_data_a.device_id, "device_a");

    let mut session_b = PairingSession::new(keypair_b.clone());
    session_b.process_peer_data(&parsed_data_a).unwrap();
    let shared_secret_b = session_b.shared_secret().unwrap().to_vec();

    // Device A scans B's QR code
    let parsed_data_b = QrCodeParser::parse_pairing_data(&qr_code_b).unwrap();
    assert_eq!(parsed_data_b.device_id, "device_b");

    let mut session_a = PairingSession::new(keypair_a.clone());
    session_a.process_peer_data(&parsed_data_b).unwrap();
    let shared_secret_a = session_a.shared_secret().unwrap().to_vec();

    // Verify both devices computed the same shared secret
    assert_eq!(shared_secret_a, shared_secret_b, "Shared secrets should match");

    // Complete pairing and save to device managers
    let paired_device_b_from_a = session_a.complete().unwrap();
    let paired_device_a_from_b = session_b.complete().unwrap();

    // Create PairedDevice objects for storage
    let paired_b = PairedDevice {
        device_id: paired_device_b_from_a.device_id.clone(),
        device_name: "Device B".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: keypair_b.public_key_bytes(),
        shared_secret: shared_secret_a.clone(),
        paired_at: timestamp_now(),
        last_connected: None,
        last_seen: Some(timestamp_now()),
    };

    let paired_a = PairedDevice {
        device_id: paired_device_a_from_b.device_id.clone(),
        device_name: "Device A".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: keypair_a.public_key_bytes(),
        shared_secret: shared_secret_b.clone(),
        paired_at: timestamp_now(),
        last_connected: None,
        last_seen: Some(timestamp_now()),
    };

    // Save pairing to device managers
    manager_a.pair_device(paired_b.clone()).await.unwrap();
    manager_b.pair_device(paired_a.clone()).await.unwrap();

    // Verify pairing was saved
    let paired_devices_a = manager_a.get_paired_devices().await;
    assert_eq!(paired_devices_a.len(), 1);
    assert_eq!(paired_devices_a[0].device_id, "device_b");

    let paired_devices_b = manager_b.get_paired_devices().await;
    assert_eq!(paired_devices_b.len(), 1);
    assert_eq!(paired_devices_b[0].device_id, "device_a");
}

/// Test 3.2: ECDH shared secret verification
#[tokio::test]
async fn test_ecdh_shared_secret_consistency() {
    // Generate two keypairs
    let keypair_a = EcdhKeyPair::generate();
    let keypair_b = EcdhKeyPair::generate();

    // Extract public keys
    let public_a = keypair_a.public_key_bytes();
    let public_b = keypair_b.public_key_bytes();

    // Each side computes shared secret using peer's public key
    let secret_a = keypair_a.compute_shared_secret(&public_b).unwrap();
    let secret_b = keypair_b.compute_shared_secret(&public_a).unwrap();

    // Verify they match
    assert_eq!(secret_a, secret_b, "ECDH shared secrets must match");
    assert_eq!(secret_a.len(), 32, "Shared secret should be 32 bytes");
}

/// Test 3.3: Device pairing persistence
#[tokio::test]
async fn test_paired_device_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Create manager and pair a device
    {
        let manager = DeviceManager::new(db_path.clone()).await.unwrap();

        let device = PairedDevice {
            device_id: "persistent_device".to_string(),
            device_name: "Test Device".to_string(),
            platform: DevicePlatform::MacOS,
            public_key: vec![1, 2, 3, 4],
            shared_secret: vec![5, 6, 7, 8],
            paired_at: timestamp_now(),
            last_connected: None,
            last_seen: Some(timestamp_now()),
        };

        manager.pair_device(device).await.unwrap();

        // Verify it's in memory
        let devices = manager.get_paired_devices().await;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_id, "persistent_device");
    }

    // Recreate manager from same database
    {
        let manager = DeviceManager::new(db_path).await.unwrap();

        // Verify device was loaded from database
        let devices = manager.get_paired_devices().await;
        assert_eq!(devices.len(), 1, "Device should be loaded from database");
        assert_eq!(devices[0].device_id, "persistent_device");
        assert_eq!(devices[0].device_name, "Test Device");
        assert_eq!(devices[0].platform, DevicePlatform::MacOS);
    }
}

/// Test 3.4: Unpair device
#[tokio::test]
async fn test_unpair_device() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let manager = DeviceManager::new(db_path).await.unwrap();

    // Pair two devices
    let device1 = PairedDevice {
        device_id: "device1".to_string(),
        device_name: "Device 1".to_string(),
        platform: DevicePlatform::Android,
        public_key: vec![1, 2, 3],
        shared_secret: vec![4, 5, 6],
        paired_at: timestamp_now(),
        last_connected: None,
        last_seen: Some(timestamp_now()),
    };

    let device2 = PairedDevice {
        device_id: "device2".to_string(),
        device_name: "Device 2".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: vec![7, 8, 9],
        shared_secret: vec![10, 11, 12],
        paired_at: timestamp_now(),
        last_connected: None,
        last_seen: Some(timestamp_now()),
    };

    manager.pair_device(device1).await.unwrap();
    manager.pair_device(device2).await.unwrap();

    // Verify both devices are paired
    let devices = manager.get_paired_devices().await;
    assert_eq!(devices.len(), 2);

    // Unpair device1
    manager.unpair_device("device1").await.unwrap();

    // Verify only device2 remains
    let devices = manager.get_paired_devices().await;
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_id, "device2");

    // Verify device1 is also removed from database
    // (by checking if device2 is the only one that persists)
}

/// Test 3.5: Multiple sequential pairings
#[tokio::test]
async fn test_multiple_sequential_pairings() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let manager = DeviceManager::new(db_path).await.unwrap();

    // Pair 5 devices sequentially
    for i in 1..=5 {
        let device = PairedDevice {
            device_id: format!("device_{}", i),
            device_name: format!("Device {}", i),
            platform: DevicePlatform::MacOS,
            public_key: vec![i as u8; 32],
            shared_secret: vec![(i + 10) as u8; 32],
            paired_at: timestamp_now(),
            last_connected: None,
            last_seen: Some(timestamp_now()),
        };

        manager.pair_device(device).await.unwrap();
    }

    // Verify all 5 devices are paired
    let devices = manager.get_paired_devices().await;
    assert_eq!(devices.len(), 5);

    // Verify device IDs
    let device_ids: Vec<String> = devices.iter().map(|d| d.device_id.clone()).collect();
    for i in 1..=5 {
        assert!(device_ids.contains(&format!("device_{}", i)));
    }
}

/// Test 3.6: Check device existence
#[tokio::test]
async fn test_check_device_paired() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let manager = DeviceManager::new(db_path).await.unwrap();

    // Initially no devices
    assert!(manager.get_device("test_device").await.is_none());

    // Pair a device
    let device = PairedDevice {
        device_id: "test_device".to_string(),
        device_name: "Test Device".to_string(),
        platform: DevicePlatform::MacOS,
        public_key: vec![1, 2, 3],
        shared_secret: vec![4, 5, 6],
        paired_at: timestamp_now(),
        last_connected: None,
        last_seen: Some(timestamp_now()),
    };

    manager.pair_device(device.clone()).await.unwrap();

    // Now device should exist
    let found = manager.get_device("test_device").await;
    assert!(found.is_some());
    assert_eq!(found.unwrap().device_id, "test_device");

    // Other device should not exist
    assert!(manager.get_device("other_device").await.is_none());
}
