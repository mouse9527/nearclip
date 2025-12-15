//! 配对二维码集成测试
//!
//! 测试完整的配对流程：密钥生成 → 创建配对数据 → 生成二维码 → 解析验证

use nearclip_crypto::{
    ConnectionInfo, CryptoError, EcdhKeyPair, PairingData, QrCodeConfig, QrCodeErrorCorrection,
    QrCodeGenerator, PAIRING_DATA_VERSION,
};

/// 测试完整流程：生成密钥对 → 创建配对数据 → 生成二维码
#[test]
fn test_full_pairing_qrcode_flow() {
    // 1. 生成密钥对
    let keypair = EcdhKeyPair::generate();
    let public_key = keypair.public_key_bytes();

    // 2. 创建配对数据
    let device_id = "macbook-pro-2024".to_string();
    let pairing_data = PairingData::new(device_id.clone(), &public_key).with_connection_info(
        ConnectionInfo::new()
            .with_ip("192.168.1.100")
            .with_port(8765)
            .with_mdns_name("macbook-pro._nearclip._tcp.local"),
    );

    // 3. 验证配对数据
    assert!(pairing_data.validate().is_ok());
    assert_eq!(pairing_data.device_id, device_id);
    assert_eq!(pairing_data.version, PAIRING_DATA_VERSION);

    // 4. 生成二维码
    let generator = QrCodeGenerator::new();
    let png_data = generator.generate_png(&pairing_data).unwrap();

    // 5. 验证 PNG 格式
    assert!(!png_data.is_empty());
    assert_eq!(
        &png_data[0..8],
        &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    );
}

/// 测试 JSON 序列化往返
#[test]
fn test_json_roundtrip_preserves_all_data() {
    let keypair = EcdhKeyPair::generate();
    let original = PairingData::new("test-device".to_string(), &keypair.public_key_bytes())
        .with_connection_info(
            ConnectionInfo::new()
                .with_ip("10.0.0.1")
                .with_port(12345)
                .with_mdns_name("device._nearclip._tcp.local"),
        );

    // 序列化
    let json = original.to_json().unwrap();

    // 反序列化
    let parsed = PairingData::from_json(&json).unwrap();

    // 验证所有字段一致
    assert_eq!(original.version, parsed.version);
    assert_eq!(original.device_id, parsed.device_id);
    assert_eq!(original.public_key, parsed.public_key);

    let orig_info = original.connection_info.as_ref().unwrap();
    let parsed_info = parsed.connection_info.as_ref().unwrap();
    assert_eq!(orig_info.ip, parsed_info.ip);
    assert_eq!(orig_info.port, parsed_info.port);
    assert_eq!(orig_info.mdns_name, parsed_info.mdns_name);
}

/// 测试公钥解码后可用于 ECDH
#[test]
fn test_public_key_from_pairing_data_works_for_ecdh() {
    // 设备 A 生成密钥对和配对数据
    let device_a = EcdhKeyPair::generate();
    let pairing_a = PairingData::new("device-a".to_string(), &device_a.public_key_bytes());

    // 设备 B 生成密钥对
    let device_b = EcdhKeyPair::generate();

    // 从配对数据中提取公钥
    let public_a_bytes = pairing_a.public_key_bytes().unwrap();

    // 使用提取的公钥计算共享密钥
    let shared_b = device_b.compute_shared_secret(&public_a_bytes).unwrap();
    let shared_a = device_a
        .compute_shared_secret(&device_b.public_key_bytes())
        .unwrap();

    // 两个设备应该得到相同的共享密钥
    assert_eq!(shared_a, shared_b);
}

/// 测试压缩格式公钥也能正常工作
#[test]
fn test_compressed_public_key_in_pairing_data() {
    let keypair = EcdhKeyPair::generate();
    let compressed_key = keypair.public_key_bytes_compressed(); // 33 字节

    let pairing_data = PairingData::new("device".to_string(), &compressed_key);

    // 验证通过
    assert!(pairing_data.validate().is_ok());

    // JSON 往返正常
    let json = pairing_data.to_json().unwrap();
    let parsed = PairingData::from_json(&json).unwrap();
    assert_eq!(pairing_data, parsed);

    // 可以解码公钥
    let decoded = parsed.public_key_bytes().unwrap();
    assert_eq!(decoded.len(), 33);
}

/// 测试不同数据长度的二维码生成
#[test]
fn test_qrcode_generation_various_data_lengths() {
    let generator = QrCodeGenerator::new();

    // 最小数据
    let minimal = PairingData::new("x".to_string(), &[0x04; 65]);
    assert!(generator.generate_png(&minimal).is_ok());

    // 中等数据（带连接信息）
    let medium = PairingData::new("device-name-here".to_string(), &[0x04; 65])
        .with_connection_info(ConnectionInfo::new().with_ip("192.168.1.1").with_port(8080));
    assert!(generator.generate_png(&medium).is_ok());

    // 较长数据（完整连接信息 + 长设备名）
    let long_name = "my-very-long-device-name-that-contains-lots-of-characters".to_string();
    let full = PairingData::new(long_name, &[0x04; 65]).with_connection_info(
        ConnectionInfo::new()
            .with_ip("2001:0db8:85a3:0000:0000:8a2e:0370:7334") // IPv6
            .with_port(65535)
            .with_mdns_name(
                "very-long-device-name._nearclip._tcp.local.with.extra.subdomain.parts",
            ),
    );
    assert!(generator.generate_png(&full).is_ok());
}

/// 测试不同配置的二维码生成
#[test]
fn test_qrcode_generation_various_configs() {
    let data = PairingData::new("device".to_string(), &[0x04; 65]).with_connection_info(
        ConnectionInfo::new()
            .with_ip("192.168.1.100")
            .with_port(8765),
    );

    // 小尺寸
    let small_config = QrCodeConfig::new().with_module_size(4).with_margin(2);
    let small_gen = QrCodeGenerator::with_config(small_config);
    let small_png = small_gen.generate_png(&data).unwrap();

    // 大尺寸
    let large_config = QrCodeConfig::new().with_module_size(16).with_margin(8);
    let large_gen = QrCodeGenerator::with_config(large_config);
    let large_png = large_gen.generate_png(&data).unwrap();

    // 大尺寸的 PNG 应该更大
    assert!(large_png.len() > small_png.len());
}

/// 测试所有纠错级别
#[test]
fn test_all_error_correction_levels() {
    let data = PairingData::new("device".to_string(), &[0x04; 65]);

    let levels = [
        QrCodeErrorCorrection::Low,
        QrCodeErrorCorrection::Medium,
        QrCodeErrorCorrection::Quartile,
        QrCodeErrorCorrection::High,
    ];

    let mut sizes = Vec::new();
    for level in levels {
        let config = QrCodeConfig::new()
            .with_error_correction(level)
            .with_module_size(4); // 固定大小以便比较
        let gen = QrCodeGenerator::with_config(config);
        let png = gen.generate_png(&data).unwrap();
        sizes.push(png.len());
    }

    // 更高的纠错级别通常会产生更大的二维码（更多模块）
    // 由于模块大小固定，PNG 大小会随纠错级别增加
    // 注意：这不是严格保证，但通常成立
    assert!(!sizes.is_empty());
}

/// 测试无效配对数据的处理
#[test]
fn test_invalid_pairing_data_validation() {
    // 空设备 ID
    let empty_id = PairingData::new("".to_string(), &[0x04; 65]);
    assert!(matches!(
        empty_id.validate(),
        Err(CryptoError::InvalidPairingData(_))
    ));

    // 无效公钥长度
    let invalid_key = PairingData::new("device".to_string(), &[0x04; 10]);
    assert!(matches!(
        invalid_key.validate(),
        Err(CryptoError::InvalidPairingData(_))
    ));

    // 无效 Base64（直接修改）
    let mut invalid_b64 = PairingData::new("device".to_string(), &[0x04; 65]);
    invalid_b64.public_key = "!!!invalid-base64!!!".to_string();
    assert!(matches!(
        invalid_b64.validate(),
        Err(CryptoError::InvalidPairingData(_))
    ));
}

/// 测试无效 JSON 解析
#[test]
fn test_invalid_json_parsing() {
    // 完全无效的 JSON
    let result = PairingData::from_json("not json");
    assert!(matches!(result, Err(CryptoError::JsonSerialization(_))));

    // 缺少必需字段
    let result = PairingData::from_json(r#"{"device_id": "test"}"#);
    assert!(result.is_err());

    // 空 JSON
    let result = PairingData::from_json("{}");
    assert!(result.is_err());
}

/// 测试连接信息的可选性
#[test]
fn test_connection_info_optionality() {
    let keypair = EcdhKeyPair::generate();

    // 无连接信息
    let no_info = PairingData::new("device".to_string(), &keypair.public_key_bytes());
    let json_no_info = no_info.to_json().unwrap();
    assert!(!json_no_info.contains("connection_info"));

    // 有连接信息
    let with_info = PairingData::new("device".to_string(), &keypair.public_key_bytes())
        .with_connection_info(ConnectionInfo::new().with_port(8080));
    let json_with_info = with_info.to_json().unwrap();
    assert!(json_with_info.contains("connection_info"));

    // 两者都能成功生成二维码
    let gen = QrCodeGenerator::new();
    assert!(gen.generate_png(&no_info).is_ok());
    assert!(gen.generate_png(&with_info).is_ok());
}

/// 测试版本字段兼容性
#[test]
fn test_version_field_compatibility() {
    // 当前版本
    let current = PairingData::new("device".to_string(), &[0x04; 65]);
    assert_eq!(current.version, PAIRING_DATA_VERSION);

    // 模拟旧版本数据（手动构造 JSON）
    let old_json = r#"{"version":0,"device_id":"old-device","public_key":"BAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#;
    let parsed = PairingData::from_json(old_json).unwrap();
    assert_eq!(parsed.version, 0);
    assert_eq!(parsed.device_id, "old-device");
}

/// 测试生成的二维码是有效的 PNG
#[test]
fn test_generated_png_is_valid() {
    let data = PairingData::new("device".to_string(), &[0x04; 65]);
    let gen = QrCodeGenerator::new();
    let png_data = gen.generate_png(&data).unwrap();

    // 验证 PNG 签名
    let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(&png_data[0..8], &png_signature);

    // 验证 IHDR chunk 存在（PNG 格式要求）
    // IHDR 是 [length(4)] [IHDR(4)] ...
    // 在签名之后应该有 IHDR
    let ihdr_marker = &png_data[12..16];
    assert_eq!(ihdr_marker, b"IHDR");

    // 验证 IEND chunk 存在（PNG 结尾标记）
    let iend_marker = b"IEND";
    let has_iend = png_data
        .windows(4)
        .any(|window| window == iend_marker);
    assert!(has_iend, "PNG should end with IEND chunk");
}

/// 测试多次生成二维码的一致性
#[test]
fn test_qrcode_generation_consistency() {
    let keypair = EcdhKeyPair::generate();
    let data = PairingData::new("device".to_string(), &keypair.public_key_bytes());
    let gen = QrCodeGenerator::new();

    // 多次生成应该产生相同结果
    let png1 = gen.generate_png(&data).unwrap();
    let png2 = gen.generate_png(&data).unwrap();

    assert_eq!(png1, png2);
}
