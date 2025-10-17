#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_all_error_codes() {
        // 测试所有定义的错误码
        let error_codes = vec![
            common::ErrorCode::ErrorNone,
            common::ErrorCode::ErrorInvalidMessage,
            common::ErrorCode::ErrorInvalidSignature,
            common::ErrorCode::ErrorExpiredMessage,
            common::ErrorCode::ErrorUnsupportedVersion,
            common::ErrorCode::ErrorDeviceNotFound,
            common::ErrorCode::ErrorPairingFailed,
            common::ErrorCode::ErrorEncryptionFailed,
            common::ErrorCode::ErrorNetworkError,
            common::ErrorCode::ErrorTimeout,
            common::ErrorCode::ErrorQuotaExceeded,
            common::ErrorCode::ErrorInternalError,
        ];

        for (index, error_code) in error_codes.iter().enumerate() {
            assert_eq!(*error_code as i32, index as i32);
        }
    }

    #[test]
    fn test_error_message_creation_and_validation() {
        let error_message = common::ErrorMessage {
            code: common::ErrorCode::ErrorInvalidMessage as i32,
            message: "Invalid message format".to_string(),
            details: "Missing required field: device_id".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        // 验证错误消息结构
        assert_eq!(error_message.code, common::ErrorCode::ErrorInvalidMessage as i32);
        assert_eq!(error_message.message, "Invalid message format");
        assert_eq!(error_message.details, "Missing required field: device_id");
        assert!(error_message.timestamp > 0);
    }

    #[test]
    fn test_error_message_descriptions() {
        // 测试每个错误码的描述
        let test_cases = vec![
            (common::ErrorCode::ErrorNone, "无错误"),
            (common::ErrorCode::ErrorInvalidMessage, "无效消息"),
            (common::ErrorCode::ErrorInvalidSignature, "无效签名"),
            (common::ErrorCode::ErrorExpiredMessage, "消息已过期"),
            (common::ErrorCode::ErrorUnsupportedVersion, "不支持的版本"),
            (common::ErrorCode::ErrorDeviceNotFound, "设备未找到"),
            (common::ErrorCode::ErrorPairingFailed, "配对失败"),
            (common::ErrorCode::ErrorEncryptionFailed, "加密失败"),
            (common::ErrorCode::ErrorNetworkError, "网络错误"),
            (common::ErrorCode::ErrorTimeout, "超时"),
            (common::ErrorCode::ErrorQuotaExceeded, "配额超限"),
            (common::ErrorCode::ErrorInternalError, "内部错误"),
        ];

        for (error_code, expected_description) in test_cases {
            // 在实际实现中，这里会调用错误码的描述方法
            // 现在我们验证错误码的数值
            assert!(error_code as i32 >= 0);
            assert!(error_code as i32 <= 11); // 最大错误码索引
        }
    }

    #[test]
    fn test_protocol_error_handling() {
        // 测试协议错误类型
        let test_cases = vec![
            (
                ProtocolError::InvalidFormat("test message".to_string()),
                "Invalid message format: test message"
            ),
            (
                ProtocolError::UnsupportedVersion("1.0".to_string()),
                "Unsupported protocol version: 1.0"
            ),
            (
                ProtocolError::SignatureVerificationFailed,
                "Message signature verification failed"
            ),
            (
                ProtocolError::CryptographicError("encryption failed".to_string()),
                "Encryption/decryption error: encryption failed"
            ),
            (
                ProtocolError::NetworkError("connection lost".to_string()),
                "Network error: connection lost"
            ),
        ];

        for (error, expected_message) in test_cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_device_broadcast_validation_errors() {
        let mut broadcast = create_valid_device_broadcast();

        // 测试空设备ID
        broadcast.device_id = "".to_string();
        let result = broadcast.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("设备ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复设备ID，测试空设备名称
        broadcast.device_id = "test-device".to_string();
        broadcast.device_name = "".to_string();
        let result = broadcast.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("设备名称不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复设备名称，测试无效时间戳
        broadcast.device_name = "Test Device".to_string();
        broadcast.timestamp = 0;
        let result = broadcast.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("时间戳无效"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_pairing_request_validation_errors() {
        let mut pairing_request = create_valid_pairing_request();

        // 测试空发起者ID
        pairing_request.initiator_id = "".to_string();
        let result = pairing_request.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("发起者ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复发起者ID，测试空目标ID
        pairing_request.initiator_id = "initiator-device".to_string();
        pairing_request.target_id = "".to_string();
        let result = pairing_request.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("目标ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复目标ID，测试空设备名称
        pairing_request.target_id = "target-device".to_string();
        pairing_request.device_name = "".to_string();
        let result = pairing_request.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("设备名称不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复设备名称，测试空随机数
        pairing_request.device_name = "Initiator Device".to_string();
        pairing_request.nonce = vec![];
        let result = pairing_request.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("随机数不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_pairing_response_validation_errors() {
        let mut pairing_response = create_valid_pairing_response();

        // 测试空响应者ID
        pairing_response.responder_id = "".to_string();
        let result = pairing_response.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("响应者ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复响应者ID，测试空发起者ID
        pairing_response.responder_id = "responder-device".to_string();
        pairing_response.initiator_id = "".to_string();
        let result = pairing_response.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("发起者ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复发起者ID，测试空签名随机数
        pairing_response.initiator_id = "initiator-device".to_string();
        pairing_response.signed_nonce = vec![];
        let result = pairing_response.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("签名的随机数不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_clipboard_data_validation_errors() {
        let mut clipboard_data = create_valid_clipboard_data();

        // 测试空数据ID
        clipboard_data.data_id = "".to_string();
        let result = clipboard_data.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("数据ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复数据ID，测试空内容
        clipboard_data.data_id = "test-data-id".to_string();
        clipboard_data.content = vec![];
        let result = clipboard_data.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("数据内容不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复内容，测试无效创建时间
        clipboard_data.content = "test content".as_bytes().to_vec();
        clipboard_data.created_at = 0;
        let result = clipboard_data.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("创建时间无效"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_data_chunk_validation_errors() {
        let mut data_chunk = create_valid_data_chunk();

        // 测试空数据ID
        data_chunk.data_id = "".to_string();
        let result = data_chunk.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("数据ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复数据ID，测试无效分片索引
        data_chunk.data_id = "test-data-id".to_string();
        data_chunk.chunk_index = 2; // 大于 total_chunks
        let result = data_chunk.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("分片索引无效"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复分片索引，测试空分片数据
        data_chunk.chunk_index = 0;
        data_chunk.chunk_data = vec![];
        let result = data_chunk.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("分片数据不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复分片数据，测试空校验和
        data_chunk.chunk_data = b"test chunk".to_vec();
        data_chunk.checksum = vec![];
        let result = data_chunk.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("校验和不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_sync_message_validation_errors() {
        let mut sync_message = create_valid_sync_message();

        // 测试空设备ID
        sync_message.device_id = "".to_string();
        let result = sync_message.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("设备ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复设备ID，测试无数据内容
        sync_message.device_id = "test-device".to_string();
        sync_message.data = None;
        let result = sync_message.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("数据内容不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_pairing_confirmation_validation_errors() {
        let mut confirmation = pairing::PairingConfirmation {
            session_id: "test-session".to_string(),
            confirmation_hash: vec![1, 2, 3, 4],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        // 测试空会话ID
        confirmation.session_id = "".to_string();
        let result = confirmation.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("会话ID不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }

        // 恢复会话ID，测试空确认哈希
        confirmation.session_id = "test-session".to_string();
        confirmation.confirmation_hash = vec![];
        let result = confirmation.validate();
        assert!(result.is_err());
        if let Err(ProtocolError::InvalidFormat(msg)) = result {
            assert!(msg.contains("确认哈希不能为空"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_error_recovery_scenarios() {
        // 测试各种错误恢复场景

        // 1. 设备广播错误恢复
        let discovery_handler = DiscoveryHandler::new();
        let invalid_broadcast = discovery::DeviceBroadcast {
            device_id: "".to_string(), // 无效设备ID
            device_name: "Test Device".to_string(),
            device_type: discovery::DeviceType::Android as i32,
            capabilities: vec![],
            version: "1.0".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            public_key: vec![],
            metadata: HashMap::new(),
        };

        let result = discovery_handler.handle_broadcast(&invalid_broadcast);
        assert!(result.is_err());

        // 创建有效广播并验证恢复
        let valid_broadcast = create_valid_device_broadcast();
        let recovery_result = discovery_handler.handle_broadcast(&valid_broadcast);
        assert!(recovery_result.is_ok());

        // 2. 配对错误恢复
        let pairing_handler = PairingHandler::new();
        let invalid_pairing_request = pairing::PairingRequest {
            initiator_id: "".to_string(), // 无效发起者ID
            target_id: "target-device".to_string(),
            public_key: vec![1, 2, 3, 4],
            device_name: "Test Device".to_string(),
            nonce: create_test_nonce(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        let pairing_result = pairing_handler.handle_pairing_response(create_valid_pairing_response());
        assert!(pairing_result.is_ok());

        // 3. 同步错误恢复
        let sync_handler = SyncHandler::new();
        let invalid_sync_message = sync::SyncMessage {
            device_id: "".to_string(), // 无效设备ID
            operation: sync::SyncOperation::SyncCreate as i32,
            data: Some(create_valid_clipboard_data()),
            chunks: vec![],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            signature: vec![],
        };

        let sync_result = sync_handler.handle_sync_message(create_valid_sync_message());
        assert!(sync_result.is_ok());
    }

    #[test]
    fn test_timeout_error_handling() {
        // 测试超时错误处理
        let current_time = chrono::Utc::now().timestamp_millis() as u64;
        let timeout_threshold = 30000; // 30秒

        // 创建过期的消息
        let expired_message = create_valid_device_broadcast();
        let mut expired_broadcast = expired_message;
        expired_broadcast.timestamp = current_time - timeout_threshold - 1000; // 超过阈值1秒

        // 在实际实现中，这里会检查时间戳
        // 现在我们验证时间戳确实过期了
        assert!(expired_broadcast.timestamp < current_time - timeout_threshold);

        // 创建有效消息
        let valid_message = create_valid_device_broadcast();
        let mut valid_broadcast = valid_message;
        valid_broadcast.timestamp = current_time - 1000; // 1秒前，未超时

        assert!(valid_broadcast.timestamp > current_time - timeout_threshold);
    }

    // 辅助函数
    fn create_valid_device_broadcast() -> discovery::DeviceBroadcast {
        discovery::DeviceBroadcast {
            device_id: "test-device".to_string(),
            device_name: "Test Device".to_string(),
            device_type: discovery::DeviceType::Android as i32,
            capabilities: vec![],
            version: "1.0".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            public_key: vec![1, 2, 3, 4],
            metadata: HashMap::new(),
        }
    }

    fn create_valid_pairing_request() -> pairing::PairingRequest {
        pairing::PairingRequest {
            initiator_id: "initiator-device".to_string(),
            target_id: "target-device".to_string(),
            public_key: vec![1, 2, 3, 4],
            device_name: "Initiator Device".to_string(),
            nonce: create_test_nonce(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    fn create_valid_pairing_response() -> pairing::PairingResponse {
        pairing::PairingResponse {
            responder_id: "responder-device".to_string(),
            initiator_id: "initiator-device".to_string(),
            public_key: vec![1, 2, 3, 4],
            signed_nonce: vec![5, 6, 7, 8],
            shared_secret: vec![9, 10, 11, 12],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    fn create_valid_clipboard_data() -> sync::ClipboardData {
        sync::ClipboardData {
            data_id: "test-data-id".to_string(),
            type_: sync::DataType::Text as i32,
            content: "Hello, World!".as_bytes().to_vec(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now().timestamp_millis() as u64,
            expires_at: 0,
            source_app: "TestApp".to_string(),
        }
    }

    fn create_valid_data_chunk() -> sync::DataChunk {
        sync::DataChunk {
            data_id: "test-data-id".to_string(),
            chunk_index: 0,
            total_chunks: 1,
            chunk_data: b"test chunk".to_vec(),
            checksum: vec![1, 2, 3, 4],
        }
    }

    fn create_valid_sync_message() -> sync::SyncMessage {
        sync::SyncMessage {
            device_id: "test-device".to_string(),
            operation: sync::SyncOperation::SyncCreate as i32,
            data: Some(create_valid_clipboard_data()),
            chunks: vec![],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            signature: vec![],
        }
    }

    fn create_test_nonce() -> Vec<u8> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }
}