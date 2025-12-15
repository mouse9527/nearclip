//! BLE 数据传输集成测试
//!
//! 这些测试验证 BLE 数据分片和重组的完整流程。
//!
//! 主要测试:
//! - ChunkHeader 序列化/反序列化
//! - Chunker 数据分片
//! - Reassembler 数据重组 (包括乱序接收)
//! - PeripheralDataReceiver 数据接收

use nearclip_ble::{
    BleError, ChunkHeader, Chunker, DataReceiverCallback, PeripheralDataConfig,
    PeripheralDataReceiver, Reassembler, ATT_HEADER_SIZE, CHUNK_HEADER_SIZE,
    DATA_ACK_CHARACTERISTIC_UUID, DATA_TRANSFER_CHARACTERISTIC_UUID, DEFAULT_BLE_MTU,
    DEFAULT_CHUNK_PAYLOAD_SIZE, DEFAULT_MTU, DEFAULT_REASSEMBLE_TIMEOUT,
    DEFAULT_REASSEMBLE_TIMEOUT_SECS, MAX_BLE_MTU, MAX_CHUNK_PAYLOAD_SIZE,
    MAX_CONCURRENT_MESSAGES,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================
// GATT 数据传输常量测试
// ============================================================

#[test]
fn test_data_transfer_uuid_format() {
    let uuid_str = DATA_TRANSFER_CHARACTERISTIC_UUID.to_string();
    assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000004");
    assert_eq!(DATA_TRANSFER_CHARACTERISTIC_UUID.as_bytes().len(), 16);
}

#[test]
fn test_data_ack_uuid_format() {
    let uuid_str = DATA_ACK_CHARACTERISTIC_UUID.to_string();
    assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000005");
    assert_eq!(DATA_ACK_CHARACTERISTIC_UUID.as_bytes().len(), 16);
}

#[test]
fn test_mtu_constants_consistency() {
    // 验证 MTU 相关常量的一致性
    assert_eq!(DEFAULT_BLE_MTU, 23);
    assert_eq!(ATT_HEADER_SIZE, 3);
    assert_eq!(CHUNK_HEADER_SIZE, 8);
    assert_eq!(MAX_BLE_MTU, 512);

    // 验证计算值
    assert_eq!(DEFAULT_CHUNK_PAYLOAD_SIZE, DEFAULT_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE);
    assert_eq!(MAX_CHUNK_PAYLOAD_SIZE, MAX_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE);

    // 确保 payload 大小为正数
    assert!(DEFAULT_CHUNK_PAYLOAD_SIZE > 0);
    assert!(MAX_CHUNK_PAYLOAD_SIZE > DEFAULT_CHUNK_PAYLOAD_SIZE);
}

// ============================================================
// ChunkHeader 测试
// ============================================================

#[test]
fn test_chunk_header_creation() {
    let header = ChunkHeader {
        message_id: 1234,
        sequence_number: 5,
        total_chunks: 10,
        payload_length: 100,
    };

    assert_eq!(header.message_id, 1234);
    assert_eq!(header.sequence_number, 5);
    assert_eq!(header.total_chunks, 10);
    assert_eq!(header.payload_length, 100);
}

#[test]
fn test_chunk_header_serialization_roundtrip() {
    let original = ChunkHeader {
        message_id: 0xABCD,
        sequence_number: 0x1234,
        total_chunks: 0x5678,
        payload_length: 0x9ABC,
    };

    let bytes = original.to_bytes();
    assert_eq!(bytes.len(), CHUNK_HEADER_SIZE);

    let restored = ChunkHeader::from_bytes(&bytes).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn test_chunk_header_boundary_values() {
    // 测试边界值
    let max_header = ChunkHeader {
        message_id: u16::MAX,
        sequence_number: u16::MAX,
        total_chunks: u16::MAX,
        payload_length: u16::MAX,
    };

    let bytes = max_header.to_bytes();
    let restored = ChunkHeader::from_bytes(&bytes).unwrap();
    assert_eq!(max_header, restored);

    let min_header = ChunkHeader {
        message_id: 0,
        sequence_number: 0,
        total_chunks: 1,
        payload_length: 0,
    };

    let bytes = min_header.to_bytes();
    let restored = ChunkHeader::from_bytes(&bytes).unwrap();
    assert_eq!(min_header, restored);
}

#[test]
fn test_chunk_header_invalid_bytes_length() {
    // 测试无效的字节长度
    let short_bytes = vec![0u8; 4]; // 太短
    let result = ChunkHeader::from_bytes(&short_bytes);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BleError::ChunkError(_)));

    let empty_bytes: [u8; 0] = [];
    let result = ChunkHeader::from_bytes(&empty_bytes);
    assert!(result.is_err());
}

// ============================================================
// Chunker 测试
// ============================================================

#[test]
fn test_chunker_small_data() {
    // 小数据 (单个分片)
    let data = b"Hello";
    let chunks = Chunker::chunk(data, 1, DEFAULT_BLE_MTU).unwrap();

    assert_eq!(chunks.len(), 1);

    // 验证分片结构
    let header = ChunkHeader::from_bytes(&chunks[0][..CHUNK_HEADER_SIZE]).unwrap();
    assert_eq!(header.message_id, 1);
    assert_eq!(header.sequence_number, 0);
    assert_eq!(header.total_chunks, 1);
    assert_eq!(header.payload_length as usize, data.len());

    // 验证 payload
    let payload = &chunks[0][CHUNK_HEADER_SIZE..];
    assert_eq!(payload, data);
}

#[test]
fn test_chunker_large_data() {
    // 大数据 (多个分片)
    let data: Vec<u8> = (0..100).collect();
    let chunks = Chunker::chunk(&data, 42, DEFAULT_BLE_MTU).unwrap();

    // 使用默认 MTU，每个分片 payload 最大 12 字节
    // 100 字节数据需要 ceil(100/12) = 9 个分片
    let expected_chunks = data.len().div_ceil(DEFAULT_CHUNK_PAYLOAD_SIZE);
    assert_eq!(chunks.len(), expected_chunks);

    // 验证所有分片的 header
    for (i, chunk) in chunks.iter().enumerate() {
        let header = ChunkHeader::from_bytes(&chunk[..CHUNK_HEADER_SIZE]).unwrap();
        assert_eq!(header.message_id, 42);
        assert_eq!(header.sequence_number, i as u16);
        assert_eq!(header.total_chunks, expected_chunks as u16);
    }
}

#[test]
fn test_chunker_exact_mtu_boundary() {
    // 数据刚好是 payload 大小的整数倍
    let data = vec![0u8; DEFAULT_CHUNK_PAYLOAD_SIZE * 3];
    let chunks = Chunker::chunk(&data, 1, DEFAULT_BLE_MTU).unwrap();

    assert_eq!(chunks.len(), 3);

    for chunk in &chunks {
        let header = ChunkHeader::from_bytes(&chunk[..CHUNK_HEADER_SIZE]).unwrap();
        assert_eq!(header.payload_length as usize, DEFAULT_CHUNK_PAYLOAD_SIZE);
    }
}

#[test]
fn test_chunker_larger_mtu() {
    // 测试更大的 MTU
    let data: Vec<u8> = (0..200).collect();
    let chunks = Chunker::chunk(&data, 1, MAX_BLE_MTU).unwrap();

    // 使用最大 MTU，每个分片 payload 最大 501 字节
    // 200 字节数据只需要 1 个分片
    assert_eq!(chunks.len(), 1);

    let header = ChunkHeader::from_bytes(&chunks[0][..CHUNK_HEADER_SIZE]).unwrap();
    assert_eq!(header.payload_length as usize, 200);
}

#[test]
fn test_chunker_empty_data() {
    // 空数据会产生一个只有头部的分片
    let data: [u8; 0] = [];
    let chunks = Chunker::chunk(&data, 1, DEFAULT_BLE_MTU).unwrap();

    assert_eq!(chunks.len(), 1);

    let header = ChunkHeader::from_bytes(&chunks[0][..CHUNK_HEADER_SIZE]).unwrap();
    assert_eq!(header.message_id, 1);
    assert_eq!(header.sequence_number, 0);
    assert_eq!(header.total_chunks, 1);
    assert_eq!(header.payload_length, 0);
    assert_eq!(chunks[0].len(), CHUNK_HEADER_SIZE); // 只有头部
}

#[test]
fn test_chunker_invalid_mtu() {
    let data = b"test";

    // MTU 太小 (小于 header + ATT)
    let small_mtu = CHUNK_HEADER_SIZE + ATT_HEADER_SIZE;
    let result = Chunker::chunk(data, 1, small_mtu);
    assert!(result.is_err());
}

// ============================================================
// Reassembler 测试
// ============================================================

#[test]
fn test_reassembler_single_chunk() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 1, timeout);

    let header = ChunkHeader {
        message_id: 1,
        sequence_number: 0,
        total_chunks: 1,
        payload_length: 5,
    };

    reassembler.add_chunk(header, b"Hello".to_vec()).unwrap();

    assert!(reassembler.is_complete());
    assert!(!reassembler.is_expired());

    let data = reassembler.assemble().unwrap();
    assert_eq!(data, b"Hello");
}

#[test]
fn test_reassembler_multiple_chunks_in_order() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 3, timeout);

    // 按顺序添加分片
    for i in 0..3 {
        let payload = format!("P{:02}", i).into_bytes();
        let header = ChunkHeader {
            message_id: 1,
            sequence_number: i,
            total_chunks: 3,
            payload_length: payload.len() as u16, // 实际长度是 3
        };
        reassembler.add_chunk(header, payload).unwrap();
    }

    assert!(reassembler.is_complete());

    let data = reassembler.assemble().unwrap();
    assert_eq!(data, b"P00P01P02");
}

#[test]
fn test_reassembler_out_of_order() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 3, timeout);

    // 乱序添加分片: 2, 0, 1
    let chunks = [
        (2, b"CCC".to_vec()),
        (0, b"AAA".to_vec()),
        (1, b"BBB".to_vec()),
    ];

    for (seq, payload) in chunks {
        let header = ChunkHeader {
            message_id: 1,
            sequence_number: seq,
            total_chunks: 3,
            payload_length: payload.len() as u16,
        };
        reassembler.add_chunk(header, payload).unwrap();
    }

    assert!(reassembler.is_complete());

    let data = reassembler.assemble().unwrap();
    assert_eq!(data, b"AAABBBCCC");
}

#[test]
fn test_reassembler_duplicate_chunk() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 2, timeout);

    let header = ChunkHeader {
        message_id: 1,
        sequence_number: 0,
        total_chunks: 2,
        payload_length: 3,
    };

    // 第一次添加
    reassembler.add_chunk(header, b"AAA".to_vec()).unwrap();

    // 重复添加相同分片应该被忽略
    reassembler.add_chunk(header, b"BBB".to_vec()).unwrap();

    // 添加第二个分片完成重组
    let header2 = ChunkHeader {
        message_id: 1,
        sequence_number: 1,
        total_chunks: 2,
        payload_length: 3,
    };
    reassembler.add_chunk(header2, b"CCC".to_vec()).unwrap();

    let data = reassembler.assemble().unwrap();
    // 应该保留第一次添加的数据
    assert_eq!(data, b"AAACCC");
}

#[test]
fn test_reassembler_wrong_message_id() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 2, timeout);

    let wrong_header = ChunkHeader {
        message_id: 999, // 错误的 message_id
        sequence_number: 0,
        total_chunks: 2,
        payload_length: 3,
    };

    let result = reassembler.add_chunk(wrong_header, b"AAA".to_vec());
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BleError::ChunkError(_)));
}

#[test]
fn test_reassembler_wrong_total_chunks() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 5, timeout);

    let wrong_header = ChunkHeader {
        message_id: 1,
        sequence_number: 0,
        total_chunks: 3, // 不匹配的 total_chunks
        payload_length: 3,
    };

    let result = reassembler.add_chunk(wrong_header, b"AAA".to_vec());
    assert!(result.is_err());
}

#[test]
fn test_reassembler_incomplete_assemble() {
    let timeout = Duration::from_secs(10);
    let mut reassembler = Reassembler::new(1, 3, timeout);

    // 只添加一个分片
    let header = ChunkHeader {
        message_id: 1,
        sequence_number: 0,
        total_chunks: 3,
        payload_length: 3,
    };
    reassembler.add_chunk(header, b"AAA".to_vec()).unwrap();

    assert!(!reassembler.is_complete());

    // 尝试重组不完整的数据
    let result = reassembler.assemble();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), BleError::ChunkError(_)));
}

#[test]
fn test_reassembler_default_timeout() {
    // 验证默认超时常量
    assert_eq!(DEFAULT_REASSEMBLE_TIMEOUT, Duration::from_secs(30));
}

// ============================================================
// PeripheralDataConfig 测试
// ============================================================

#[test]
fn test_peripheral_data_config_default() {
    let config = PeripheralDataConfig::new();

    assert_eq!(config.mtu, DEFAULT_MTU);
    assert_eq!(
        config.reassemble_timeout,
        Duration::from_secs(DEFAULT_REASSEMBLE_TIMEOUT_SECS)
    );
    assert_eq!(config.max_concurrent_messages, MAX_CONCURRENT_MESSAGES);
}

#[test]
fn test_peripheral_data_config_builder() {
    let config = PeripheralDataConfig::new()
        .with_mtu(256)
        .with_reassemble_timeout(Duration::from_secs(60))
        .with_max_concurrent_messages(20);

    assert_eq!(config.mtu, 256);
    assert_eq!(config.reassemble_timeout, Duration::from_secs(60));
    assert_eq!(config.max_concurrent_messages, 20);
}

#[test]
fn test_peripheral_data_config_validation() {
    // 有效配置
    let valid_config = PeripheralDataConfig::new();
    assert!(valid_config.validate().is_ok());

    // MTU 太小 (必须 < CHUNK_HEADER_SIZE + 1 = 9)
    let small_mtu = PeripheralDataConfig::new().with_mtu(5);
    assert!(small_mtu.validate().is_err());

    // 边界值：刚好有效的 MTU (CHUNK_HEADER_SIZE + 1 = 9)
    let boundary_mtu = PeripheralDataConfig::new().with_mtu(9);
    assert!(boundary_mtu.validate().is_ok());

    // 并发数为零
    let zero_concurrent = PeripheralDataConfig::new().with_max_concurrent_messages(0);
    assert!(zero_concurrent.validate().is_err());
}

#[test]
fn test_peripheral_data_config_clone() {
    let config = PeripheralDataConfig::new()
        .with_mtu(128)
        .with_max_concurrent_messages(5);

    let cloned = config.clone();
    assert_eq!(config.mtu, cloned.mtu);
    assert_eq!(
        config.max_concurrent_messages,
        cloned.max_concurrent_messages
    );
}

// ============================================================
// PeripheralDataReceiver 测试
// ============================================================

/// 测试用的回调实现
struct TestCallback {
    received_data: Arc<Mutex<Vec<(Vec<u8>, String)>>>,
    errors: Arc<Mutex<Vec<BleError>>>,
}

impl TestCallback {
    fn new() -> Self {
        Self {
            received_data: Arc::new(Mutex::new(Vec::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn received_count(&self) -> usize {
        self.received_data.lock().unwrap().len()
    }

    #[allow(dead_code)]
    fn error_count(&self) -> usize {
        self.errors.lock().unwrap().len()
    }

    fn get_received(&self) -> Vec<(Vec<u8>, String)> {
        self.received_data.lock().unwrap().clone()
    }
}

impl DataReceiverCallback for TestCallback {
    fn on_data_received(&self, data: Vec<u8>, from_device: &str) {
        self.received_data
            .lock()
            .unwrap()
            .push((data, from_device.to_string()));
    }

    fn on_receive_error(&self, error: BleError) {
        self.errors.lock().unwrap().push(error);
    }
}

#[tokio::test]
async fn test_peripheral_data_receiver_creation() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback).await;
    assert!(receiver.is_ok());
}

#[tokio::test]
async fn test_peripheral_data_receiver_invalid_config() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new().with_mtu(5); // 无效 MTU

    let receiver = PeripheralDataReceiver::new(config, callback).await;
    assert!(receiver.is_err());
    assert!(matches!(receiver.unwrap_err(), BleError::Configuration(_)));
}

#[tokio::test]
async fn test_peripheral_data_receiver_handle_single_chunk() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 创建单个分片的消息
    let header = ChunkHeader {
        message_id: 1,
        sequence_number: 0,
        total_chunks: 1,
        payload_length: 5,
    };

    let mut chunk_data = header.to_bytes().to_vec();
    chunk_data.extend_from_slice(b"Hello");

    // 处理分片
    let result = receiver.handle_chunk(&chunk_data, "device-001").await;
    assert!(result.is_ok());

    // 验证回调被调用
    assert_eq!(callback.received_count(), 1);
    let received = callback.get_received();
    assert_eq!(received[0].0, b"Hello");
    assert_eq!(received[0].1, "device-001");
}

#[tokio::test]
async fn test_peripheral_data_receiver_handle_multi_chunk() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 创建 3 个分片的消息
    let chunks = [
        (0, b"AAA".to_vec()),
        (1, b"BBB".to_vec()),
        (2, b"CCC".to_vec()),
    ];

    for (seq, payload) in &chunks {
        let header = ChunkHeader {
            message_id: 1,
            sequence_number: *seq,
            total_chunks: 3,
            payload_length: payload.len() as u16,
        };

        let mut chunk_data = header.to_bytes().to_vec();
        chunk_data.extend_from_slice(payload);

        receiver.handle_chunk(&chunk_data, "device-001").await.unwrap();
    }

    // 验证消息被完整重组
    assert_eq!(callback.received_count(), 1);
    let received = callback.get_received();
    assert_eq!(received[0].0, b"AAABBBCCC");
}

#[tokio::test]
async fn test_peripheral_data_receiver_out_of_order() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 乱序发送分片
    let chunks = [
        (2, b"CCC".to_vec()),
        (0, b"AAA".to_vec()),
        (1, b"BBB".to_vec()),
    ];

    for (seq, payload) in &chunks {
        let header = ChunkHeader {
            message_id: 1,
            sequence_number: *seq,
            total_chunks: 3,
            payload_length: payload.len() as u16,
        };

        let mut chunk_data = header.to_bytes().to_vec();
        chunk_data.extend_from_slice(payload);

        receiver.handle_chunk(&chunk_data, "device-001").await.unwrap();
    }

    // 验证消息被正确重组（按顺序）
    assert_eq!(callback.received_count(), 1);
    let received = callback.get_received();
    assert_eq!(received[0].0, b"AAABBBCCC");
}

#[tokio::test]
async fn test_peripheral_data_receiver_multiple_messages() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 发送两条不同的消息
    for msg_id in 1..=2 {
        let payload = format!("Msg{}", msg_id);
        let header = ChunkHeader {
            message_id: msg_id,
            sequence_number: 0,
            total_chunks: 1,
            payload_length: payload.len() as u16, // "Msg1" = 4 bytes
        };

        let mut chunk_data = header.to_bytes().to_vec();
        chunk_data.extend_from_slice(payload.as_bytes());

        receiver.handle_chunk(&chunk_data, "device-001").await.unwrap();
    }

    // 验证两条消息都被接收
    assert_eq!(callback.received_count(), 2);
}

#[tokio::test]
async fn test_peripheral_data_receiver_invalid_chunk() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 发送无效的分片数据（太短）
    let invalid_data = vec![0u8; 4];
    let result = receiver.handle_chunk(&invalid_data, "device-001").await;

    // 应该返回错误
    assert!(result.is_err());
}

#[tokio::test]
async fn test_peripheral_data_receiver_start_stop() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();

    let mut receiver = PeripheralDataReceiver::new(config, callback).await.unwrap();

    // 在当前平台上 start/stop 返回 PlatformNotSupported
    let start_result = receiver.start().await;
    match start_result {
        Ok(()) => {
            // 如果支持，停止应该也成功
            assert!(receiver.stop().await.is_ok());
        }
        Err(BleError::PlatformNotSupported) => {
            // 预期在大多数平台上的行为
        }
        Err(e) => panic!("Unexpected error: {}", e),
    }
}

// ============================================================
// 完整数据传输流程集成测试
// ============================================================

#[tokio::test]
async fn test_full_chunking_and_reassembly_flow() {
    // 模拟完整的数据传输流程：
    // 1. 发送方分片数据
    // 2. 接收方重组数据
    // 3. 验证数据完整性

    let original_data = b"This is a test message for BLE data transfer integration testing. \
                         The message should be split into multiple chunks and reassembled correctly.";

    // 发送方：分片数据
    let chunks = Chunker::chunk(original_data, 12345, DEFAULT_BLE_MTU).unwrap();
    assert!(chunks.len() > 1); // 确保数据被分成多个分片

    // 接收方：设置回调
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();
    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 接收方：处理所有分片
    for chunk in &chunks {
        receiver.handle_chunk(chunk, "sender-device").await.unwrap();
    }

    // 验证数据完整性
    assert_eq!(callback.received_count(), 1);
    let received = callback.get_received();
    assert_eq!(received[0].0, original_data);
    assert_eq!(received[0].1, "sender-device");
}

#[tokio::test]
async fn test_concurrent_messages_from_different_devices() {
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new();
    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    // 模拟两个设备同时发送消息
    let msg1 = b"Message from device 1";
    let msg2 = b"Message from device 2";

    let chunks1 = Chunker::chunk(msg1, 1, DEFAULT_BLE_MTU).unwrap();
    let chunks2 = Chunker::chunk(msg2, 2, DEFAULT_BLE_MTU).unwrap();

    // 交替接收来自不同设备的分片
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < chunks1.len() || i2 < chunks2.len() {
        if i1 < chunks1.len() {
            receiver
                .handle_chunk(&chunks1[i1], "device-001")
                .await
                .unwrap();
            i1 += 1;
        }
        if i2 < chunks2.len() {
            receiver
                .handle_chunk(&chunks2[i2], "device-002")
                .await
                .unwrap();
            i2 += 1;
        }
    }

    // 验证两条消息都被正确接收
    assert_eq!(callback.received_count(), 2);

    let received = callback.get_received();
    let msgs: Vec<&[u8]> = received.iter().map(|(d, _)| d.as_slice()).collect();
    assert!(msgs.contains(&msg1.as_slice()));
    assert!(msgs.contains(&msg2.as_slice()));
}

#[tokio::test]
async fn test_large_data_transfer() {
    // 测试较大数据的传输
    let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    // 分片
    let chunks = Chunker::chunk(&large_data, 9999, MAX_BLE_MTU).unwrap();

    // 接收并重组
    let callback = Arc::new(TestCallback::new());
    let config = PeripheralDataConfig::new().with_mtu(MAX_BLE_MTU);
    let receiver = PeripheralDataReceiver::new(config, callback.clone())
        .await
        .unwrap();

    for chunk in &chunks {
        receiver.handle_chunk(chunk, "device").await.unwrap();
    }

    // 验证
    assert_eq!(callback.received_count(), 1);
    let received = callback.get_received();
    assert_eq!(received[0].0, large_data);
}

// ============================================================
// BleError 数据传输相关变体测试
// ============================================================

#[test]
fn test_ble_error_chunk_error() {
    let err = BleError::ChunkError("test chunk error".to_string());
    assert!(err.to_string().contains("Chunk error"));
    assert!(err.to_string().contains("test chunk error"));
}

#[test]
fn test_ble_error_data_transfer() {
    let err = BleError::DataTransfer("transfer failed".to_string());
    assert!(err.to_string().contains("Data transfer error"));
    assert!(err.to_string().contains("transfer failed"));
}

#[test]
fn test_ble_error_timeout() {
    let err = BleError::Timeout("operation timed out".to_string());
    assert!(err.to_string().contains("Timeout"));
    assert!(err.to_string().contains("operation timed out"));
}

#[test]
fn test_ble_error_clone() {
    let err = BleError::ChunkError("test".to_string());
    let cloned = err.clone();
    assert_eq!(err.to_string(), cloned.to_string());
}
