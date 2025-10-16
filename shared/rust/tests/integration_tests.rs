//! 集成测试
//!
//! 测试各个模块之间的集成

use nearclip_core::*;

#[tokio::test]
async fn test_complete_workflow() {
    // 创建核心实例
    let core = NearClipCore::new().await.unwrap();

    // 启动服务
    assert!(core.start().await.is_ok());

    // 获取服务实例
    let crypto_service = core.crypto_service();
    let ble_manager = core.ble_manager();

    // 测试加密服务
    let session_key = crypto_service.generate_session_key().unwrap();
    let nonce = crypto_service.generate_nonce().unwrap();

    let plaintext = b"Hello, NearClip Integration Test!";
    let ciphertext = crypto_service.encrypt(plaintext, &session_key, &nonce).unwrap();
    let decrypted = crypto_service.decrypt(&ciphertext, &session_key, &nonce).unwrap();
    assert_eq!(plaintext.to_vec(), decrypted);

    // 测试 BLE 管理器
    let devices = ble_manager.start_scan(1).await.unwrap();
    println!("Found {} devices during integration test", devices.len());

    // 停止服务
    assert!(core.stop().await.is_ok());
}

#[tokio::test]
async fn test_crypto_and_ble_integration() {
    let crypto_service = CryptoService::new().unwrap();
    let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

    // 生成测试数据
    let test_data = b"Integration test data between crypto and BLE modules";
    let session_key = crypto_service.generate_session_key().unwrap();
    let nonce = crypto_service.generate_nonce().unwrap();

    // 加密数据
    let encrypted_data = crypto_service.encrypt(test_data, &session_key, &nonce).unwrap();

    // 模拟通过 BLE 发送加密数据
    let devices = ble_manager.start_scan(1).await.unwrap();
    if !devices.is_empty() {
        let device = &devices[0];
        assert!(ble_manager.connect_to_device(&device.id).await.is_ok());

        // 在实际环境中，这里会通过 BLE 发送数据
        // let result = ble_manager.send_message(&device.id, &encrypted_data).await;
        // assert!(result.is_ok());

        assert!(ble_manager.disconnect_from_device(&device.id).await.is_ok());
    }
}

#[tokio::test]
async fn test_sync_manager_integration() {
    let crypto_service = std::sync::Arc::new(CryptoService::new().unwrap());
    let ble_manager = std::sync::Arc::new(BLEManager::new("test-uuid".to_string()));
    let sync_manager = SyncManager::new(crypto_service.clone(), ble_manager.clone());

    // 启动同步管理器
    assert!(sync_manager.start().await.is_ok());

    // 创建测试同步数据
    let sync_data = SyncData::new(
        "test-sync-123".to_string(),
        SyncDataType::Text,
        b"Integration test sync content".to_vec(),
        "test-device".to_string(),
    );

    // 序列化和反序列化测试
    let serialized = sync_manager.serialize_sync_data(&sync_data).unwrap();
    let deserialized = sync_manager.deserialize_sync_data(&serialized).unwrap();

    assert_eq!(sync_data.data_type, deserialized.data_type);
    assert_eq!(sync_data.source_device, deserialized.source_device);
    assert_eq!(sync_data.content, deserialized.content);

    // 停止同步管理器
    assert!(sync_manager.stop().await.is_ok());
}

#[test]
fn test_error_handling_integration() {
    use nearclip_core::error::*;

    // 测试错误码转换
    assert_eq!(error_message_from_code(0), "操作成功");
    assert_eq!(error_message_from_code(-2), "加密操作失败");
    assert_eq!(error_message_from_code(-3), "BLE 操作失败");

    // 测试错误类型转换
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test file");
    let nearclip_err: NearClipError = io_err.into();
    assert!(matches!(nearclip_err, NearClipError::IoError(_)));
}

#[test]
fn test_device_management_integration() {
    use nearclip_core::device::*;

    let mut manager = DeviceManager::new("current-device".to_string());

    // 创建测试设备
    let device1 = Device::new(
        "device1".to_string(),
        "Android Device".to_string(),
        DeviceType::Android,
        vec![1, 2, 3, 4],
    );

    let device2 = Device::new(
        "device2".to_string(),
        "Mac Device".to_string(),
        DeviceType::MacOS,
        vec![5, 6, 7, 8],
    );

    // 添加设备
    manager.add_device(device1.clone());
    manager.add_device(device2.clone());

    // 测试设备配对
    let mut device1_clone = device1.clone();
    device1_clone.set_paired(true);
    manager.add_device(device1_clone);

    // 验证已配对设备
    let paired_devices = manager.get_paired_devices();
    assert_eq!(paired_devices.len(), 1);
    assert!(paired_devices[0].is_paired);
}