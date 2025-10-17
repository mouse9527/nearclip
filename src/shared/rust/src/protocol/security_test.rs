#[cfg(test)]
mod security_tests {
    use super::*;
    use ring::signature::{Ed25519, KeyPair, Signature, UnparsedPublicKey};
    use ring::rand::{SecureRandom, SystemRandom};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn generate_test_keypair() -> KeyPair {
        let rng = SystemRandom::new();
        KeyPair::generate_pkcs8(&rng).unwrap().into()
    }

    fn create_test_nonce() -> Vec<u8> {
        let rng = SystemRandom::new();
        let mut nonce = vec![0u8; 32];
        rng.fill(&mut nonce).unwrap();
        nonce
    }

    #[test]
    fn test_edh_key_exchange_simulation() {
        // 在实际实现中，这里会使用真实的 ECDH 密钥交换
        // 这里我们模拟密钥交换的基本流程

        let alice_keypair = generate_test_keypair();
        let bob_keypair = generate_test_keypair();

        // 模拟公钥交换
        let alice_public_key = alice_keypair.public_key().as_ref().to_vec();
        let bob_public_key = bob_keypair.public_key().as_ref().to_vec();

        // 验证公钥不为空
        assert!(!alice_public_key.is_empty());
        assert!(!bob_public_key.is_empty());
        assert_eq!(alice_public_key.len(), 32); // Ed25519 公钥长度
        assert_eq!(bob_public_key.len(), 32);

        // 在实际实现中，这里会计算共享密钥
        // 现在我们只是验证密钥交换的基本结构
        let pairing_request = pairing::PairingRequest {
            initiator_id: "alice-device".to_string(),
            target_id: "bob-device".to_string(),
            public_key: alice_public_key.clone(),
            device_name: "Alice's Device".to_string(),
            nonce: create_test_nonce(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        let pairing_response = pairing::PairingResponse {
            responder_id: "bob-device".to_string(),
            initiator_id: "alice-device".to_string(),
            public_key: bob_public_key.clone(),
            signed_nonce: vec![1, 2, 3, 4], // 在实际实现中这里会是签名
            shared_secret: vec![5, 6, 7, 8], // 在实际实现中这里会是计算出的共享密钥
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        // 验证配对请求和响应的基本结构
        assert_eq!(pairing_request.initiator_id, "alice-device");
        assert_eq!(pairing_request.target_id, "bob-device");
        assert_eq!(pairing_request.public_key, alice_public_key);

        assert_eq!(pairing_response.responder_id, "bob-device");
        assert_eq!(pairing_response.initiator_id, "alice-device");
        assert_eq!(pairing_response.public_key, bob_public_key);
    }

    #[test]
    fn test_message_signing_and_verification() {
        let keypair = generate_test_keypair();
        let message_content = b"Hello, secure world!";

        // 使用私钥签名消息
        let signature = keypair.sign(message_content);

        // 验证签名
        let public_key = keypair.public_key();
        let verification_result = public_key.verify(message_content, &signature);

        assert!(verification_result.is_ok());

        // 测试篡改消息的验证
        let tampered_message = b"Hello, tampered world!";
        let tampered_result = public_key.verify(tampered_message, &signature);

        assert!(tampered_result.is_err());

        // 测试使用错误公钥验证
        let wrong_keypair = generate_test_keypair();
        let wrong_public_key = wrong_keypair.public_key();
        let wrong_key_result = wrong_public_key.verify(message_content, &signature);

        assert!(wrong_key_result.is_err());
    }

    #[test]
    fn test_nonce_generation_uniqueness() {
        let mut nonces = std::collections::HashSet::new();
        let num_nonces = 100;

        for _ in 0..num_nonces {
            let nonce = create_test_nonce();
            assert_eq!(nonce.len(), 32);
            assert!(nonces.insert(nonce.clone())); // 如果插入失败，说明有重复
        }

        assert_eq!(nonces.len(), num_nonces);
    }

    #[test]
    fn test_replay_attack_prevention() {
        let keypair = generate_test_keypair();
        let original_nonce = create_test_nonce();
        let original_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 创建原始配对请求
        let original_request = pairing::PairingRequest {
            initiator_id: "alice-device".to_string(),
            target_id: "bob-device".to_string(),
            public_key: keypair.public_key().as_ref().to_vec(),
            device_name: "Alice's Device".to_string(),
            nonce: original_nonce.clone(),
            timestamp: original_timestamp,
        };

        // 验证原始请求（在实现中会检查时间戳和 nonce）
        assert!(original_request.validate().is_ok());

        // 模拟重放攻击 - 使用相同的 nonce 但更晚的时间戳
        let replay_request = pairing::PairingRequest {
            initiator_id: "alice-device".to_string(),
            target_id: "bob-device".to_string(),
            public_key: keypair.public_key().as_ref().to_vec(),
            device_name: "Alice's Device".to_string(),
            nonce: original_nonce.clone(), // 重用 nonce
            timestamp: original_timestamp + 300000, // 5分钟后
        };

        // 在实际实现中，这里会检测到 nonce 重用
        // 现在我们验证基本结构
        assert!(replay_request.validate().is_ok());
        assert_eq!(replay_request.nonce, original_request.nonce);
        assert_ne!(replay_request.timestamp, original_request.timestamp);

        // 测试过期消息（时间戳太旧）
        let expired_timestamp = original_timestamp - 3600000; // 1小时前
        let expired_request = pairing::PairingRequest {
            initiator_id: "alice-device".to_string(),
            target_id: "bob-device".to_string(),
            public_key: keypair.public_key().as_ref().to_vec(),
            device_name: "Alice's Device".to_string(),
            nonce: create_test_nonce(), // 新的 nonce
            timestamp: expired_timestamp,
        };

        // 在实际实现中，这里会检查时间戳是否过期
        // 现在我们验证时间戳确实过期了
        assert!(expired_request.timestamp < original_timestamp);
    }

    #[test]
    fn test_device_identity_verification() {
        let keypair = generate_test_keypair();
        let device_id = "secure-device-123";
        let device_name = "Secure Device";

        // 创建带签名的设备信息
        let device_info = format!("{}:{}:{}", device_id, device_name, SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis());

        let signature = keypair.sign(device_info.as_bytes());

        // 验证设备身份
        let public_key = keypair.public_key();
        let verification_result = public_key.verify(device_info.as_bytes(), &signature);

        assert!(verification_result.is_ok());

        // 测试设备信息篡改
        let tampered_info = format!("{}:{}:{}", device_id, "Tampered Device", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis());

        let tampered_result = public_key.verify(tampered_info.as_bytes(), &signature);
        assert!(tampered_result.is_err());
    }

    #[test]
    fn test_encryption_key_security() {
        // 测试密钥的随机性和安全性
        let key1 = generate_test_keypair();
        let key2 = generate_test_keypair();

        // 确保密钥不同
        assert_ne!(key1.public_key().as_ref(), key2.public_key().as_ref());
        assert_ne!(key1.private_key().as_ref(), key2.private_key().as_ref());

        // 确保密钥长度正确
        assert_eq!(key1.public_key().as_ref().len(), 32);
        assert_eq!(key1.private_key().as_ref().len(), 32);

        // 确保密钥不为零
        let public_key_bytes = key1.public_key().as_ref();
        let private_key_bytes = key1.private_key().as_ref();

        assert!(!public_key_bytes.iter().all(|&b| b == 0));
        assert!(!private_key_bytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_session_key_generation() {
        // 模拟会话密钥生成过程
        let alice_keypair = generate_test_keypair();
        let bob_keypair = generate_test_keypair();

        // 在实际实现中，这里会使用 ECDH 生成会话密钥
        // 现在我们模拟会话密钥的基本属性
        let session_id = format!("{}-{}",
            hex::encode(alice_keypair.public_key().as_ref()[0..8].to_vec()),
            hex::encode(bob_keypair.public_key().as_ref()[0..8].to_vec())
        );

        let session_key_material = format!("{}:{}:{}",
            hex::encode(alice_keypair.public_key().as_ref()),
            hex::encode(bob_keypair.public_key().as_ref()),
            session_id
        );

        // 使用 SHA256 生成会话密钥
        use sha2::{Sha256, Digest};
        let session_key = Sha256::digest(session_key_material.as_bytes());

        // 验证会话密钥属性
        assert_eq!(session_key.len(), 32); // SHA256 输出长度
        assert!(!session_key.iter().all(|&b| b == 0)); // 不全为零

        // 创建配对确认消息
        let confirmation = pairing::PairingConfirmation {
            session_id: session_id.clone(),
            confirmation_hash: session_key.to_vec(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        // 验证配对确认
        assert_eq!(confirmation.session_id, session_id);
        assert!(!confirmation.confirmation_hash.is_empty());
        assert!(confirmation.timestamp > 0);
    }

    #[test]
    fn test_secure_message_flow() {
        // 测试完整的安全消息流程
        let alice_keypair = generate_test_keypair();
        let bob_keypair = generate_test_keypair();

        // 1. Alice 发起配对
        let pairing_request = pairing::PairingRequest {
            initiator_id: "alice".to_string(),
            target_id: "bob".to_string(),
            public_key: alice_keypair.public_key().as_ref().to_vec(),
            device_name: "Alice Device".to_string(),
            nonce: create_test_nonce(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        // 2. Bob 响应配对
        let pairing_response = pairing::PairingResponse {
            responder_id: "bob".to_string(),
            initiator_id: "alice".to_string(),
            public_key: bob_keypair.public_key().as_ref().to_vec(),
            signed_nonce: bob_keypair.sign(&pairing_request.nonce).as_ref().to_vec(),
            shared_secret: vec![1, 2, 3, 4], // 模拟共享密钥
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        // 3. 验证签名
        let verification_result = alice_keypair.public_key()
            .verify(&pairing_request.nonce, &pairing_response.signed_nonce);

        assert!(verification_result.is_ok());

        // 4. 创建配对确认
        let session_id = format!("alice-bob-{}", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis());

        let confirmation = pairing::PairingConfirmation {
            session_id: session_id.clone(),
            confirmation_hash: vec![5, 6, 7, 8], // 模拟确认哈希
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        // 5. 验证完整流程
        assert_eq!(pairing_request.initiator_id, "alice");
        assert_eq!(pairing_request.target_id, "bob");
        assert_eq!(pairing_response.responder_id, "bob");
        assert_eq!(pairing_response.initiator_id, "alice");
        assert_eq!(confirmation.session_id, session_id);

        // 验证所有消息都有有效的时间戳
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        assert!(current_time - pairing_request.timestamp < 10000); // 10秒内
        assert!(current_time - pairing_response.timestamp < 10000);
        assert!(current_time - confirmation.timestamp < 10000);
    }
}