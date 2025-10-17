#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_device_broadcast_validation() {
        let mut broadcast = discovery::DeviceBroadcast {
            device_id: "test-device".to_string(),
            device_name: "Test Device".to_string(),
            device_type: discovery::DeviceType::Android as i32,
            capabilities: vec![],
            version: "1.0".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            public_key: vec![1, 2, 3, 4],
            metadata: HashMap::new(),
        };

        // 测试有效广播消息
        assert!(broadcast.validate().is_ok());

        // 测试无效设备ID
        broadcast.device_id = "".to_string();
        assert!(broadcast.validate().is_err());

        // 恢复有效设备ID，测试无效设备名称
        broadcast.device_id = "test-device".to_string();
        broadcast.device_name = "".to_string();
        assert!(broadcast.validate().is_err());

        // 测试无效时间戳
        broadcast.device_name = "Test Device".to_string();
        broadcast.timestamp = 0;
        assert!(broadcast.validate().is_err());
    }

    #[test]
    fn test_discovery_handler_scan_request() {
        let handler = DiscoveryHandler::new();
        let result = handler.create_scan_request(30);

        assert!(result.is_ok());
        let scan_request = result.unwrap();
        assert_eq!(scan_request.timeout_seconds, 30);
        assert!(scan_request.filter_types.is_empty());
        assert!(scan_request.required_capabilities.is_empty());
    }

    #[test]
    fn test_pairing_request_validation() {
        let mut pairing_request = pairing::PairingRequest {
            initiator_id: "initiator-device".to_string(),
            target_id: "target-device".to_string(),
            public_key: vec![1, 2, 3, 4],
            device_name: "Initiator Device".to_string(),
            nonce: vec![5, 6, 7, 8],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        // 测试有效配对请求
        assert!(pairing_request.validate().is_ok());

        // 测试无效发起者ID
        pairing_request.initiator_id = "".to_string();
        assert!(pairing_request.validate().is_err());

        // 恢复有效发起者ID，测试无效目标ID
        pairing_request.initiator_id = "initiator-device".to_string();
        pairing_request.target_id = "".to_string();
        assert!(pairing_request.validate().is_err());

        // 测试空设备名称
        pairing_request.target_id = "target-device".to_string();
        pairing_request.device_name = "".to_string();
        assert!(pairing_request.validate().is_err());

        // 测试空随机数
        pairing_request.device_name = "Initiator Device".to_string();
        pairing_request.nonce = vec![];
        assert!(pairing_request.validate().is_err());
    }

    #[test]
    fn test_pairing_handler_initiate_pairing() {
        let handler = PairingHandler::new();
        let result = handler.initiate_pairing("target-device");

        assert!(result.is_ok());
        let pairing_request = result.unwrap();
        assert_eq!(pairing_request.target_id, "target-device");
        assert!(!pairing_request.initiator_id.is_empty());
        assert!(!pairing_request.device_name.is_empty());
        assert!(!pairing_request.nonce.is_empty());
        assert!(pairing_request.timestamp > 0);
    }

    #[test]
    fn test_clipboard_data_validation() {
        let mut clipboard_data = sync::ClipboardData {
            data_id: "test-data-id".to_string(),
            type_: sync::DataType::Text as i32,
            content: "Hello, World!".as_bytes().to_vec(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now().timestamp_millis() as u64,
            expires_at: 0, // 永不过期
            source_app: "TestApp".to_string(),
        };

        // 测试有效剪贴板数据
        assert!(clipboard_data.validate().is_ok());

        // 测试无效数据ID
        clipboard_data.data_id = "".to_string();
        assert!(clipboard_data.validate().is_err());

        // 恢复有效数据ID，测试空内容
        clipboard_data.data_id = "test-data-id".to_string();
        clipboard_data.content = vec![];
        assert!(clipboard_data.validate().is_err());

        // 测试无效创建时间
        clipboard_data.content = "Hello, World!".as_bytes().to_vec();
        clipboard_data.created_at = 0;
        assert!(clipboard_data.validate().is_err());
    }

    #[test]
    fn test_data_chunk_validation() {
        let mut data_chunk = sync::DataChunk {
            data_id: "test-data-id".to_string(),
            chunk_index: 0,
            total_chunks: 1,
            chunk_data: b"Hello, World!".to_vec(),
            checksum: vec![1, 2, 3, 4],
        };

        // 测试有效数据分片
        assert!(data_chunk.validate().is_ok());

        // 测试无效数据ID
        data_chunk.data_id = "".to_string();
        assert!(data_chunk.validate().is_err());

        // 恢复有效数据ID，测试无效分片索引
        data_chunk.data_id = "test-data-id".to_string();
        data_chunk.chunk_index = 2;
        assert!(data_chunk.validate().is_err());

        // 测试空分片数据
        data_chunk.chunk_index = 0;
        data_chunk.chunk_data = vec![];
        assert!(data_chunk.validate().is_err());

        // 测试空校验和
        data_chunk.chunk_data = b"Hello, World!".to_vec();
        data_chunk.checksum = vec![];
        assert!(data_chunk.validate().is_err());
    }

    #[test]
    fn test_sync_message_creation() {
        let handler = SyncHandler::new();
        let clipboard_data = sync::ClipboardData {
            data_id: "test-sync-data".to_string(),
            type_: sync::DataType::Text as i32,
            content: "Sync test content".as_bytes().to_vec(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now().timestamp_millis() as u64,
            expires_at: 0,
            source_app: "TestApp".to_string(),
        };

        let result = handler.create_sync_message(clipboard_data.clone());
        assert!(result.is_ok());

        let sync_message = result.unwrap();
        assert_eq!(sync_message.device_id, "local-device-id");
        assert_eq!(sync_message.operation, sync::SyncOperation::SyncCreate as i32);
        assert_eq!(sync_message.data.as_ref().unwrap().data_id, clipboard_data.data_id);
        assert!(sync_message.timestamp > 0);
    }

    #[test]
    fn test_sync_message_validation() {
        let sync_message = sync::SyncMessage {
            device_id: "test-device".to_string(),
            operation: sync::SyncOperation::SyncCreate as i32,
            data: Some(sync::ClipboardData {
                data_id: "test-data".to_string(),
                type_: sync::DataType::Text as i32,
                content: "Test content".as_bytes().to_vec(),
                metadata: HashMap::new(),
                created_at: chrono::Utc::now().timestamp_millis() as u64,
                expires_at: 0,
                source_app: "TestApp".to_string(),
            }),
            chunks: vec![],
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            signature: vec![],
        };

        // 测试有效同步消息
        assert!(sync_message.validate().is_ok());

        // 测试无效设备ID
        let mut invalid_message = sync_message.clone();
        invalid_message.device_id = "".to_string();
        assert!(invalid_message.validate().is_err());

        // 测试无数据内容
        invalid_message = sync_message.clone();
        invalid_message.data = None;
        assert!(invalid_message.validate().is_err());
    }

    #[test]
    fn test_error_message_creation() {
        let error_message = common::ErrorMessage::new(
            common::ErrorCode::ErrorInvalidMessage,
            "Invalid message format".to_string()
        ).with_details("Missing required field".to_string());

        assert_eq!(error_message.code, common::ErrorCode::ErrorInvalidMessage as i32);
        assert_eq!(error_message.message, "Invalid message format");
        assert_eq!(error_message.details, "Missing required field");
        assert!(error_message.timestamp > 0);
    }

    #[test]
    fn test_heartbeat_messages() {
        let heartbeat = common::Heartbeat::new("test-device".to_string(), 42);
        assert_eq!(heartbeat.device_id, "test-device");
        assert_eq!(heartbeat.sequence_number, 42);
        assert!(heartbeat.timestamp > 0);

        let heartbeat_ack = common::HeartbeatAck::new("test-device".to_string(), 42);
        assert_eq!(heartbeat_ack.device_id, "test-device");
        assert_eq!(heartbeat_ack.sequence_number, 42);
        assert!(heartbeat_ack.received_timestamp > 0);
    }

    #[test]
    fn test_protocol_version_compatibility() {
        let version_1_0 = common::ProtocolVersion::new(1, 0, 0);
        let version_1_1 = common::ProtocolVersion::new(1, 1, 0);
        let version_2_0 = common::ProtocolVersion::new(2, 0, 0);

        // 测试版本兼容性
        assert!(version_1_0.is_compatible_with(&version_1_1));
        assert!(version_1_1.is_compatible_with(&version_1_0));
        assert!(!version_1_0.is_compatible_with(&version_2_0));
        assert!(!version_2_0.is_compatible_with(&version_1_0));
    }

    #[test]
    fn test_capability_negotiation() {
        let min_version = common::ProtocolVersion::new(1, 0, 0);
        let max_version = common::ProtocolVersion::new(1, 2, 0);

        let mut negotiation = common::CapabilityNegotiation::new(min_version.clone(), max_version.clone());
        negotiation = negotiation.with_supported_feature("clipboard_sync".to_string());
        negotiation = negotiation.with_required_feature("encryption".to_string());

        assert_eq!(negotiation.min_version.as_ref().unwrap().major, 1);
        assert_eq!(negotiation.min_version.as_ref().unwrap().minor, 0);
        assert_eq!(negotiation.max_version.as_ref().unwrap().major, 1);
        assert_eq!(negotiation.max_version.as_ref().unwrap().minor, 2);
        assert!(negotiation.supported_features.contains(&"clipboard_sync".to_string()));
        assert!(negotiation.required_features.contains(&"encryption".to_string()));
    }
}