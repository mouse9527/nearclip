//! GATT 服务定义
//!
//! NearClip BLE GATT 服务和特征 UUID 定义。
//!
//! # GATT 结构
//!
//! ```text
//! NearClip Service (NEARCLIP_SERVICE_UUID)
//! ├── Device ID Characteristic (DEVICE_ID_CHARACTERISTIC_UUID) - Read
//! ├── Public Key Hash Characteristic (PUBKEY_HASH_CHARACTERISTIC_UUID) - Read
//! ├── Data Transfer Characteristic (DATA_TRANSFER_CHARACTERISTIC_UUID) - Write
//! └── Data Ack Characteristic (DATA_ACK_CHARACTERISTIC_UUID) - Read + Notify
//! ```

use uuid::Uuid;

/// NearClip 服务 UUID (128-bit 自定义)
///
/// 用于标识 NearClip BLE 服务。使用自定义 UUID 避免与标准蓝牙服务冲突。
///
/// UUID: `4e454152-434c-4950-0000-000000000001`
/// 格式解读: "NEAR" "CLIP" 后跟服务编号
pub const NEARCLIP_SERVICE_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c, // CL
    0x49, 0x50, // IP
    0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // service number
]);

/// 设备 ID 特征 UUID
///
/// 只读特征，包含设备标识符（UTF-8 字符串）。
///
/// UUID: `4e454152-434c-4950-0000-000000000002`
pub const DEVICE_ID_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c, // CL
    0x49, 0x50, // IP
    0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // characteristic number
]);

/// 公钥哈希特征 UUID
///
/// 只读特征，包含 Base64 编码的公钥哈希。
///
/// UUID: `4e454152-434c-4950-0000-000000000003`
pub const PUBKEY_HASH_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c, // CL
    0x49, 0x50, // IP
    0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // characteristic number
]);

/// 数据传输特征 UUID
///
/// 可写特征，用于接收剪贴板数据分片。
/// 属性: Write Without Response
///
/// UUID: `4e454152-434c-4950-0000-000000000004`
pub const DATA_TRANSFER_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c, // CL
    0x49, 0x50, // IP
    0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x04, // characteristic number
]);

/// 数据确认特征 UUID
///
/// 可读可通知特征，用于返回 ACK 和传输状态。
/// 属性: Read + Notify
///
/// UUID: `4e454152-434c-4950-0000-000000000005`
pub const DATA_ACK_CHARACTERISTIC_UUID: Uuid = Uuid::from_bytes([
    0x4e, 0x45, 0x41, 0x52, // NEAR
    0x43, 0x4c, // CL
    0x49, 0x50, // IP
    0x00, 0x00, // reserved
    0x00, 0x00, 0x00, 0x00, 0x00, 0x05, // characteristic number
]);

/// 默认广播名称
pub const DEFAULT_ADVERTISE_NAME: &str = "NearClip";

/// 广播名称最大长度（BLE 广播包限制）
pub const MAX_ADVERTISE_NAME_LENGTH: usize = 29;

/// 设备 ID 最大长度
pub const MAX_DEVICE_ID_LENGTH: usize = 64;

/// 公钥哈希长度（SHA-256 的 Base64 编码）
pub const PUBKEY_HASH_LENGTH: usize = 44;

// ============================================
// BLE MTU 相关常量
// ============================================

/// 默认 BLE ATT MTU (最小值)
///
/// BLE 规范定义的最小 ATT MTU 为 23 字节
pub const DEFAULT_BLE_MTU: usize = 23;

/// ATT 协议头部大小
///
/// ATT 协议需要 3 字节头部 (opcode + handle)
pub const ATT_HEADER_SIZE: usize = 3;

/// 分片头部大小
///
/// ChunkHeader 占用 8 字节
pub const CHUNK_HEADER_SIZE: usize = 8;

/// 默认有效 payload 大小
///
/// = DEFAULT_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE = 23 - 3 - 8 = 12 bytes
pub const DEFAULT_CHUNK_PAYLOAD_SIZE: usize =
    DEFAULT_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE;

/// 最大协商 MTU
///
/// BLE 4.2+ 支持最大 512 字节 MTU
pub const MAX_BLE_MTU: usize = 512;

/// 最大有效 payload 大小
///
/// = MAX_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE = 512 - 3 - 8 = 501 bytes
pub const MAX_CHUNK_PAYLOAD_SIZE: usize = MAX_BLE_MTU - ATT_HEADER_SIZE - CHUNK_HEADER_SIZE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_format() {
        // 验证 UUID 是有效的 128-bit UUID
        assert_eq!(NEARCLIP_SERVICE_UUID.as_bytes().len(), 16);
        assert_eq!(DEVICE_ID_CHARACTERISTIC_UUID.as_bytes().len(), 16);
        assert_eq!(PUBKEY_HASH_CHARACTERISTIC_UUID.as_bytes().len(), 16);
        assert_eq!(DATA_TRANSFER_CHARACTERISTIC_UUID.as_bytes().len(), 16);
        assert_eq!(DATA_ACK_CHARACTERISTIC_UUID.as_bytes().len(), 16);
    }

    #[test]
    fn test_uuids_are_different() {
        let uuids = [
            NEARCLIP_SERVICE_UUID,
            DEVICE_ID_CHARACTERISTIC_UUID,
            PUBKEY_HASH_CHARACTERISTIC_UUID,
            DATA_TRANSFER_CHARACTERISTIC_UUID,
            DATA_ACK_CHARACTERISTIC_UUID,
        ];
        for i in 0..uuids.len() {
            for j in (i + 1)..uuids.len() {
                assert_ne!(uuids[i], uuids[j], "UUIDs at {} and {} should be different", i, j);
            }
        }
    }

    #[test]
    fn test_service_uuid_string_format() {
        // 验证 UUID 字符串格式
        let uuid_str = NEARCLIP_SERVICE_UUID.to_string();
        assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000001");
    }

    #[test]
    fn test_device_id_uuid_string_format() {
        let uuid_str = DEVICE_ID_CHARACTERISTIC_UUID.to_string();
        assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000002");
    }

    #[test]
    fn test_pubkey_hash_uuid_string_format() {
        let uuid_str = PUBKEY_HASH_CHARACTERISTIC_UUID.to_string();
        assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000003");
    }

    #[test]
    fn test_default_advertise_name() {
        assert_eq!(DEFAULT_ADVERTISE_NAME, "NearClip");
        assert!(DEFAULT_ADVERTISE_NAME.len() <= MAX_ADVERTISE_NAME_LENGTH);
    }

    #[test]
    fn test_constants_values() {
        assert_eq!(MAX_ADVERTISE_NAME_LENGTH, 29);
        assert_eq!(MAX_DEVICE_ID_LENGTH, 64);
        assert_eq!(PUBKEY_HASH_LENGTH, 44); // SHA-256 Base64 encoded
    }

    #[test]
    fn test_data_transfer_uuid_string_format() {
        let uuid_str = DATA_TRANSFER_CHARACTERISTIC_UUID.to_string();
        assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000004");
    }

    #[test]
    fn test_data_ack_uuid_string_format() {
        let uuid_str = DATA_ACK_CHARACTERISTIC_UUID.to_string();
        assert_eq!(uuid_str, "4e454152-434c-4950-0000-000000000005");
    }

    #[test]
    fn test_mtu_constants() {
        // 验证 MTU 相关常量
        assert_eq!(DEFAULT_BLE_MTU, 23);
        assert_eq!(ATT_HEADER_SIZE, 3);
        assert_eq!(CHUNK_HEADER_SIZE, 8);
        assert_eq!(MAX_BLE_MTU, 512);

        // 验证计算出的 payload 大小
        assert_eq!(DEFAULT_CHUNK_PAYLOAD_SIZE, 12); // 23 - 3 - 8 = 12
        assert_eq!(MAX_CHUNK_PAYLOAD_SIZE, 501);    // 512 - 3 - 8 = 501
    }

    #[test]
    fn test_chunk_payload_positive() {
        // 确保 payload 大小为正数
        assert!(DEFAULT_CHUNK_PAYLOAD_SIZE > 0);
        assert!(MAX_CHUNK_PAYLOAD_SIZE > DEFAULT_CHUNK_PAYLOAD_SIZE);
    }
}
