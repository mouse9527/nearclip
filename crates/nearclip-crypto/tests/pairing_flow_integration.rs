//! 配对流程集成测试
//!
//! 测试完整的配对流程：
//! 1. 设备 A 生成配对二维码
//! 2. 设备 B 扫描并解析二维码
//! 3. 设备 B 使用配对数据完成 ECDH 密钥协商
//! 4. 双向配对验证

use nearclip_crypto::{
    ConnectionInfo, CryptoError, EcdhKeyPair, PairedDevice, PairingData, PairingSession,
    QrCodeGenerator, QrCodeParser,
};

/// 测试完整配对流程：生成二维码 → 解析 → 密钥协商 → 配对成功
#[test]
fn test_full_pairing_flow() {
    // === 设备 A: 生成配对二维码 ===
    let keypair_a = EcdhKeyPair::generate();
    let pairing_data_a = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes())
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("192.168.1.100")
                .with_port(8765)
                .with_mdns_name("device-a._nearclip._tcp.local"),
        );

    let generator = QrCodeGenerator::new();
    let qrcode_png = generator.generate_png(&pairing_data_a).unwrap();

    // === 设备 B: 扫描二维码并配对 ===
    // 1. 解析二维码
    let parsed_data = QrCodeParser::parse_pairing_data(&qrcode_png).unwrap();

    // 验证解析的数据与原始数据一致
    assert_eq!(parsed_data.device_id, "device-a");
    assert_eq!(parsed_data.public_key, pairing_data_a.public_key);

    // 2. 创建配对会话
    let keypair_b = EcdhKeyPair::generate();
    let mut session_b = PairingSession::new(keypair_b);

    // 3. 处理配对数据
    session_b.process_peer_data(&parsed_data).unwrap();

    // 4. 完成配对
    let paired_device = session_b.complete().unwrap();

    // === 验证配对结果 ===
    assert_eq!(paired_device.device_id, "device-a");
    assert!(!paired_device.shared_secret_hash.is_empty());
    assert!(paired_device.paired_at > 0);

    // 验证连接信息
    let conn = paired_device.connection_info.as_ref().unwrap();
    assert_eq!(conn.ip, Some("192.168.1.100".to_string()));
    assert_eq!(conn.port, Some(8765));
    assert_eq!(conn.mdns_name, Some("device-a._nearclip._tcp.local".to_string()));
}

/// 测试双向配对（设备 A 扫描 B，B 扫描 A）
#[test]
fn test_bidirectional_pairing() {
    // === 设备 A 准备 ===
    let keypair_a = EcdhKeyPair::generate();
    let pairing_data_a = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes());

    // === 设备 B 准备 ===
    let keypair_b = EcdhKeyPair::generate();
    let pairing_data_b = PairingData::new("device-b".to_string(), &keypair_b.public_key_bytes());

    // === 生成二维码 ===
    let generator = QrCodeGenerator::new();
    let qrcode_a = generator.generate_png(&pairing_data_a).unwrap();
    let qrcode_b = generator.generate_png(&pairing_data_b).unwrap();

    // === 设备 A 扫描 B 的二维码 ===
    let parsed_b = QrCodeParser::parse_pairing_data(&qrcode_b).unwrap();
    let mut session_a = PairingSession::new(keypair_a);
    session_a.process_peer_data(&parsed_b).unwrap();

    // === 设备 B 扫描 A 的二维码 ===
    let parsed_a = QrCodeParser::parse_pairing_data(&qrcode_a).unwrap();
    let mut session_b = PairingSession::new(keypair_b);
    session_b.process_peer_data(&parsed_a).unwrap();

    // === 验证双方得到相同的共享密钥 ===
    assert_eq!(session_a.shared_secret(), session_b.shared_secret());

    // === 完成配对 ===
    let shared_secret = session_a.shared_secret().unwrap().to_vec();

    let paired_a = session_a.complete().unwrap();
    let paired_b = session_b.complete().unwrap();

    // 验证双方的配对设备信息正确
    assert_eq!(paired_a.device_id, "device-b");
    assert_eq!(paired_b.device_id, "device-a");

    // 验证双方都能用相同的共享密钥验证
    assert!(paired_a.verify_shared_secret(&shared_secret));
    assert!(paired_b.verify_shared_secret(&shared_secret));
}

/// 测试配对数据完整性验证
#[test]
fn test_pairing_data_integrity() {
    let keypair = EcdhKeyPair::generate();
    let original = PairingData::new("test-device".to_string(), &keypair.public_key_bytes())
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("10.0.0.1")
                .with_port(12345)
                .with_mdns_name("test._nearclip._tcp.local"),
        );

    // 生成二维码
    let generator = QrCodeGenerator::new();
    let png = generator.generate_png(&original).unwrap();

    // 解析二维码
    let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();

    // 验证所有字段完整
    assert_eq!(original.version, parsed.version);
    assert_eq!(original.device_id, parsed.device_id);
    assert_eq!(original.public_key, parsed.public_key);

    let orig_conn = original.connection_info.as_ref().unwrap();
    let parsed_conn = parsed.connection_info.as_ref().unwrap();
    assert_eq!(orig_conn.ip, parsed_conn.ip);
    assert_eq!(orig_conn.port, parsed_conn.port);
    assert_eq!(orig_conn.mdns_name, parsed_conn.mdns_name);
}

/// 测试压缩公钥的配对流程
#[test]
fn test_pairing_with_compressed_key() {
    let keypair_a = EcdhKeyPair::generate();
    let compressed_key = keypair_a.public_key_bytes_compressed(); // 33 bytes

    let pairing_data_a = PairingData::new("device-a".to_string(), &compressed_key);

    // 生成二维码
    let generator = QrCodeGenerator::new();
    let png = generator.generate_png(&pairing_data_a).unwrap();

    // 解析二维码
    let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
    assert_eq!(parsed.public_key_bytes().unwrap().len(), 33);

    // 配对
    let keypair_b = EcdhKeyPair::generate();
    let mut session_b = PairingSession::new(keypair_b);
    session_b.process_peer_data(&parsed).unwrap();

    // 验证共享密钥计算正确
    assert!(session_b.shared_secret().is_some());
    assert_eq!(session_b.shared_secret().unwrap().len(), 32);
}

/// 测试 PairedDevice 序列化和反序列化
#[test]
fn test_paired_device_persistence() {
    // 完成配对
    let keypair_local = EcdhKeyPair::generate();
    let mut session = PairingSession::new(keypair_local);

    let keypair_peer = EcdhKeyPair::generate();
    let peer_data = PairingData::new("peer-device".to_string(), &keypair_peer.public_key_bytes())
        .with_connection_info(ConnectionInfo::new().with_ip("192.168.1.1").with_port(8080));

    session.process_peer_data(&peer_data).unwrap();
    let paired_device = session.complete().unwrap();

    // 序列化
    let json = paired_device.to_json().unwrap();

    // 反序列化
    let restored = PairedDevice::from_json(&json).unwrap();

    // 验证数据一致
    assert_eq!(paired_device, restored);
}

/// 测试无效二维码的错误处理
#[test]
fn test_invalid_qrcode_handling() {
    // 测试非图片数据
    let result = QrCodeParser::parse_from_bytes(b"not an image");
    assert!(matches!(result, Err(CryptoError::QrCodeParsing(_))));

    // 测试空数据
    let result = QrCodeParser::parse_from_bytes(&[]);
    assert!(matches!(result, Err(CryptoError::QrCodeParsing(_))));
}

/// 测试无效配对数据的错误处理
#[test]
fn test_invalid_pairing_data_handling() {
    let keypair = EcdhKeyPair::generate();
    let mut session = PairingSession::new(keypair);

    // 空设备 ID
    let invalid_data = PairingData::new("".to_string(), &[0x04; 65]);
    let result = session.process_peer_data(&invalid_data);
    assert!(result.is_err());
}

/// 测试配对会话未完成时的错误处理
#[test]
fn test_incomplete_session_handling() {
    let keypair = EcdhKeyPair::generate();
    let session = PairingSession::new(keypair);

    // 未调用 process_peer_data 就尝试 complete
    let result = session.complete();
    assert!(matches!(result, Err(CryptoError::PairingFailed(_))));
}

/// 测试多次生成相同数据的二维码一致性
#[test]
fn test_qrcode_generation_consistency() {
    let keypair = EcdhKeyPair::generate();
    let data = PairingData::new("device".to_string(), &keypair.public_key_bytes());

    let generator = QrCodeGenerator::new();
    let png1 = generator.generate_png(&data).unwrap();
    let png2 = generator.generate_png(&data).unwrap();

    // 相同数据应该生成相同的二维码
    assert_eq!(png1, png2);

    // 两次解析都应该得到相同结果
    let parsed1 = QrCodeParser::parse_pairing_data(&png1).unwrap();
    let parsed2 = QrCodeParser::parse_pairing_data(&png2).unwrap();
    assert_eq!(parsed1, parsed2);
}

/// 测试公钥从二维码解析后可用于 ECDH
#[test]
fn test_parsed_public_key_works_for_ecdh() {
    // 设备 A 生成密钥对和二维码
    let keypair_a = EcdhKeyPair::generate();
    let data_a = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes());
    let generator = QrCodeGenerator::new();
    let png = generator.generate_png(&data_a).unwrap();

    // 设备 B 解析二维码
    let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
    let parsed_public_key = parsed.public_key_bytes().unwrap();

    // 设备 B 生成密钥对并计算共享密钥
    let keypair_b = EcdhKeyPair::generate();
    let shared_b = keypair_b.compute_shared_secret(&parsed_public_key).unwrap();

    // 设备 A 也计算共享密钥
    let shared_a = keypair_a
        .compute_shared_secret(&keypair_b.public_key_bytes())
        .unwrap();

    // 双方应该得到相同的共享密钥
    assert_eq!(shared_a, shared_b);
}

/// 测试配对后重新计算共享密钥验证
#[test]
fn test_shared_secret_verification_after_pairing() {
    // 设备 A
    let keypair_a = EcdhKeyPair::generate();
    let data_a = PairingData::new("device-a".to_string(), &keypair_a.public_key_bytes());

    // 设备 B 配对
    let keypair_b = EcdhKeyPair::generate();
    let mut session = PairingSession::new(keypair_b.clone());
    session.process_peer_data(&data_a).unwrap();
    let paired_device = session.complete().unwrap();

    // 模拟重新连接：使用存储的公钥重新计算共享密钥
    let recomputed_secret = keypair_b
        .compute_shared_secret(&paired_device.public_key_bytes)
        .unwrap();

    // 验证重新计算的密钥与存储的哈希匹配
    assert!(paired_device.verify_shared_secret(&recomputed_secret));
}

/// 测试大数据量配对信息
#[test]
fn test_large_pairing_data() {
    let keypair = EcdhKeyPair::generate();
    let long_device_id = "very-long-device-name-that-contains-many-characters-to-test-capacity";
    let data = PairingData::new(long_device_id.to_string(), &keypair.public_key_bytes())
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("2001:0db8:85a3:0000:0000:8a2e:0370:7334") // IPv6
                .with_port(65535)
                .with_mdns_name("very-long-device-name._nearclip._tcp.local.with.extra.subdomain"),
        );

    let generator = QrCodeGenerator::new();
    let png = generator.generate_png(&data).unwrap();

    let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
    assert_eq!(parsed.device_id, long_device_id);
}
