//! 设备存储集成测试
//!
//! 测试完整的配对 → 保存 → 加载 → 验证流程

use nearclip_crypto::{
    ConnectionInfo, DeviceStore, EcdhKeyPair, FileDeviceStore, FileDeviceStoreConfig,
    PairingData, PairingSession,
};
use std::fs;

/// 创建临时测试存储
fn temp_store() -> FileDeviceStore {
    let temp_dir = std::env::temp_dir().join(format!(
        "nearclip_integration_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let config = FileDeviceStoreConfig::new()
        .with_directory(&temp_dir)
        .with_filename("test_devices.json");
    FileDeviceStore::with_config(config)
}

/// 清理测试目录
fn cleanup(store: &FileDeviceStore) {
    let _ = fs::remove_file(store.file_path());
    if let Some(parent) = store.file_path().parent() {
        let _ = fs::remove_dir(parent);
    }
}

#[test]
fn test_pairing_then_persist_and_reload() {
    // 设备 A 和设备 B 生成密钥对
    let device_a_keypair = EcdhKeyPair::generate();
    let device_b_keypair = EcdhKeyPair::generate();

    // 设备 A 创建配对数据
    let device_a_pairing_data = PairingData::new(
        "device-a".to_string(),
        &device_a_keypair.public_key_bytes(),
    )
    .with_connection_info(
        ConnectionInfo::new()
            .with_ip("192.168.1.100")
            .with_port(8765),
    );

    // 设备 B 创建配对会话并处理设备 A 的数据
    let mut session_b = PairingSession::new(device_b_keypair);
    session_b.process_peer_data(&device_a_pairing_data).unwrap();

    // 完成配对，获取 PairedDevice
    let paired_device_a = session_b.complete().unwrap();

    // 创建存储并保存
    let store = temp_store();
    store.save(&paired_device_a).unwrap();

    // 重新加载并验证
    let loaded = store.load("device-a").unwrap().unwrap();

    assert_eq!(loaded.device_id, "device-a");
    assert_eq!(loaded.public_key_bytes, device_a_keypair.public_key_bytes());
    assert_eq!(loaded.shared_secret_hash, paired_device_a.shared_secret_hash);

    let conn_info = loaded.connection_info.as_ref().unwrap();
    assert_eq!(conn_info.ip, Some("192.168.1.100".to_string()));
    assert_eq!(conn_info.port, Some(8765));

    cleanup(&store);
}

#[test]
fn test_multiple_devices_storage() {
    let store = temp_store();

    // 配对多个设备
    let device_ids = vec!["macbook-pro", "iphone-15", "pixel-8"];

    for device_id in &device_ids {
        let local_keypair = EcdhKeyPair::generate();
        let peer_keypair = EcdhKeyPair::generate();

        let peer_data = PairingData::new(device_id.to_string(), &peer_keypair.public_key_bytes());

        let mut session = PairingSession::new(local_keypair);
        session.process_peer_data(&peer_data).unwrap();
        let paired = session.complete().unwrap();

        store.save(&paired).unwrap();
    }

    // 验证所有设备都已保存
    let all_devices = store.load_all().unwrap();
    assert_eq!(all_devices.len(), 3);

    // 验证每个设备都可以加载
    for device_id in &device_ids {
        assert!(store.exists(device_id).unwrap());
        let loaded = store.load(device_id).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().device_id, *device_id);
    }

    cleanup(&store);
}

#[test]
fn test_delete_then_load_returns_none() {
    let store = temp_store();

    // 创建并保存设备
    let keypair = EcdhKeyPair::generate();
    let peer_keypair = EcdhKeyPair::generate();
    let peer_data = PairingData::new("test-device".to_string(), &peer_keypair.public_key_bytes());

    let mut session = PairingSession::new(keypair);
    session.process_peer_data(&peer_data).unwrap();
    let paired = session.complete().unwrap();

    store.save(&paired).unwrap();

    // 验证设备存在
    assert!(store.exists("test-device").unwrap());

    // 删除设备
    let deleted = store.delete("test-device").unwrap();
    assert!(deleted);

    // 验证设备不再存在
    assert!(!store.exists("test-device").unwrap());
    let loaded = store.load("test-device").unwrap();
    assert!(loaded.is_none());

    cleanup(&store);
}

#[test]
fn test_data_consistency_across_reloads() {
    let temp_dir = std::env::temp_dir().join(format!(
        "nearclip_consistency_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let config = FileDeviceStoreConfig::new()
        .with_directory(&temp_dir)
        .with_filename("consistency_test.json");

    // 第一个存储实例：保存设备
    {
        let store1 = FileDeviceStore::with_config(config.clone());

        let keypair = EcdhKeyPair::generate();
        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new(
            "consistency-device".to_string(),
            &peer_keypair.public_key_bytes(),
        )
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("10.0.0.1")
                .with_port(9999)
                .with_mdns_name("test._nearclip._tcp.local"),
        );

        let mut session = PairingSession::new(keypair);
        session.process_peer_data(&peer_data).unwrap();
        let paired = session.complete().unwrap();

        store1.save(&paired).unwrap();
    }

    // 第二个存储实例：加载并验证
    {
        let store2 = FileDeviceStore::with_config(config.clone());

        let loaded = store2.load("consistency-device").unwrap().unwrap();

        assert_eq!(loaded.device_id, "consistency-device");

        let conn = loaded.connection_info.as_ref().unwrap();
        assert_eq!(conn.ip, Some("10.0.0.1".to_string()));
        assert_eq!(conn.port, Some(9999));
        assert_eq!(conn.mdns_name, Some("test._nearclip._tcp.local".to_string()));
    }

    // 清理
    let store_cleanup = FileDeviceStore::with_config(config);
    cleanup(&store_cleanup);
}

#[test]
fn test_update_device_preserves_other_devices() {
    let store = temp_store();

    // 保存三个设备
    for id in ["device-1", "device-2", "device-3"] {
        let keypair = EcdhKeyPair::generate();
        let peer_keypair = EcdhKeyPair::generate();
        let peer_data = PairingData::new(id.to_string(), &peer_keypair.public_key_bytes());

        let mut session = PairingSession::new(keypair);
        session.process_peer_data(&peer_data).unwrap();
        let paired = session.complete().unwrap();

        store.save(&paired).unwrap();
    }

    // 更新 device-2
    let new_keypair = EcdhKeyPair::generate();
    let new_peer_keypair = EcdhKeyPair::generate();
    let updated_peer_data = PairingData::new(
        "device-2".to_string(),
        &new_peer_keypair.public_key_bytes(),
    )
    .with_connection_info(ConnectionInfo::new().with_ip("192.168.1.200"));

    let mut new_session = PairingSession::new(new_keypair);
    new_session.process_peer_data(&updated_peer_data).unwrap();
    let updated_paired = new_session.complete().unwrap();

    store.save(&updated_paired).unwrap();

    // 验证仍然有三个设备
    let all = store.load_all().unwrap();
    assert_eq!(all.len(), 3);

    // 验证 device-2 已更新
    let loaded = store.load("device-2").unwrap().unwrap();
    let conn = loaded.connection_info.as_ref().unwrap();
    assert_eq!(conn.ip, Some("192.168.1.200".to_string()));

    // 验证其他设备未受影响
    assert!(store.exists("device-1").unwrap());
    assert!(store.exists("device-3").unwrap());

    cleanup(&store);
}

#[test]
fn test_verify_shared_secret_after_reload() {
    let store = temp_store();

    // 设备配对
    let local_keypair = EcdhKeyPair::generate();
    let peer_keypair = EcdhKeyPair::generate();

    let peer_data = PairingData::new("verify-device".to_string(), &peer_keypair.public_key_bytes());

    let mut session = PairingSession::new(local_keypair.clone());
    session.process_peer_data(&peer_data).unwrap();

    // 获取共享密钥
    let shared_secret = session.shared_secret().unwrap().to_vec();

    let paired = session.complete().unwrap();
    store.save(&paired).unwrap();

    // 重新加载
    let loaded = store.load("verify-device").unwrap().unwrap();

    // 验证共享密钥
    assert!(loaded.verify_shared_secret(&shared_secret));

    // 验证错误的共享密钥失败
    let wrong_secret = vec![0xFF; 32];
    assert!(!loaded.verify_shared_secret(&wrong_secret));

    cleanup(&store);
}

#[test]
fn test_empty_store_operations() {
    let store = temp_store();

    // 空存储的各种操作
    assert_eq!(store.count().unwrap(), 0);
    assert!(!store.exists("any-device").unwrap());
    assert!(store.load("any-device").unwrap().is_none());
    assert!(!store.delete("any-device").unwrap());

    let all = store.load_all().unwrap();
    assert!(all.is_empty());

    cleanup(&store);
}

#[test]
fn test_device_serialization_preserves_all_fields() {
    let store = temp_store();

    // 创建具有所有字段的设备
    let keypair = EcdhKeyPair::generate();
    let peer_keypair = EcdhKeyPair::generate();
    let peer_data = PairingData::new("full-device".to_string(), &peer_keypair.public_key_bytes())
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("172.16.0.1")
                .with_port(54321)
                .with_mdns_name("full._nearclip._tcp.local"),
        );

    let mut session = PairingSession::new(keypair);
    session.process_peer_data(&peer_data).unwrap();
    let paired = session.complete().unwrap();

    let original_paired_at = paired.paired_at;
    let original_hash = paired.shared_secret_hash.clone();
    let original_public_key = paired.public_key_bytes.clone();

    store.save(&paired).unwrap();

    // 重新加载
    let loaded = store.load("full-device").unwrap().unwrap();

    // 验证所有字段
    assert_eq!(loaded.device_id, "full-device");
    assert_eq!(loaded.public_key_bytes, original_public_key);
    assert_eq!(loaded.shared_secret_hash, original_hash);
    assert_eq!(loaded.paired_at, original_paired_at);

    let conn = loaded.connection_info.as_ref().unwrap();
    assert_eq!(conn.ip, Some("172.16.0.1".to_string()));
    assert_eq!(conn.port, Some(54321));
    assert_eq!(conn.mdns_name, Some("full._nearclip._tcp.local".to_string()));

    cleanup(&store);
}
