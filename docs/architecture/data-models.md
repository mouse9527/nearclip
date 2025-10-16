# 数据模型

## Device 设备模型

**目的：** 表示 NearClip 网络中的设备信息，包括设备标识、能力和连接状态。

**关键属性：**
- deviceId: String - 设备唯一标识符
- deviceName: String - 用户友好的设备名称
- deviceType: DeviceType - 设备类型（Android/Mac）
- publicKey: String - 设备公钥，用于加密验证
- lastSeen: Timestamp - 最后在线时间
- connectionStatus: ConnectionStatus - 连接状态
- capabilities: List[Capability] - 设备能力列表

### Protocol Buffers 定义

```protobuf
syntax = "proto3";

package nearclip.models;

message Device {
  string device_id = 1;
  string device_name = 2;
  DeviceType device_type = 3;
  string public_key = 4;
  uint64 last_seen = 5;
  ConnectionStatus connection_status = 6;
  repeated Capability capabilities = 7;
  string alias = 8;              // 用户自定义设备别名
  uint32 battery_level = 9;      // 电池电量百分比 (0-100)
}

enum DeviceType {
  DEVICE_TYPE_UNKNOWN = 0;
  ANDROID = 1;
  MAC = 2;
}

enum ConnectionStatus {
  CONNECTION_STATUS_UNKNOWN = 0;
  DISCONNECTED = 1;
  CONNECTED = 2;
  PAIRING = 3;
  ERROR = 4;
}

enum Capability {
  CAPABILITY_UNKNOWN = 0;
  BLE = 1;
  WIFI_DIRECT = 2;
  CLIPBOARD_READ = 3;
  CLIPBOARD_WRITE = 4;
}
```

### Rust 结构体定义

```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub public_key: String,
    pub last_seen: SystemTime,
    pub connection_status: ConnectionStatus,
    pub capabilities: Vec<Capability>,
    pub alias: Option<String>,
    pub battery_level: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Unknown,
    Android,
    Mac,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Unknown,
    Disconnected,
    Connected,
    Pairing,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Capability {
    Unknown,
    Ble,
    WifiDirect,
    ClipboardRead,
    ClipboardWrite,
}
```

### Kotlin 数据类

```kotlin
@Serializable
data class Device(
    val deviceId: String,
    val deviceName: String,
    val deviceType: DeviceType,
    val publicKey: String,
    val lastSeen: Long,
    val connectionStatus: ConnectionStatus,
    val capabilities: List<Capability>,
    val alias: String? = null,
    val batteryLevel: Int? = null
)

enum class DeviceType {
    UNKNOWN,
    ANDROID,
    MAC
}

enum class ConnectionStatus {
    UNKNOWN,
    DISCONNECTED,
    CONNECTED,
    PAIRING,
    ERROR
}
```

### Swift 结构体

```swift
import Foundation

struct Device: Codable, Identifiable, Equatable {
    let deviceId: String
    let deviceName: String
    let deviceType: DeviceType
    let publicKey: String
    let lastSeen: Date
    let connectionStatus: ConnectionStatus
    let capabilities: [Capability]
    let alias: String?
    let batteryLevel: Int?

    var id: String { deviceId }
}

enum DeviceType: String, Codable, CaseIterable {
    case unknown = "UNKNOWN"
    case android = "ANDROID"
    case mac = "MAC"
}

enum ConnectionStatus: String, Codable, CaseIterable {
    case unknown = "UNKNOWN"
    case disconnected = "DISCONNECTED"
    case connected = "CONNECTED"
    case pairing = "PAIRING"
    case error = "ERROR"
}
```

### 关系关系

- Device 1:n SyncRecord - 设备可以有多个同步记录
- Device 1:n PairingRequest - 设备可以发起或接收多个配对请求

## SyncRecord 同步记录模型

**目的：** 记录粘贴板同步操作的详细信息，用于冲突解决和历史追踪。

**Key Attributes:**
- syncId: String - 同步操作唯一标识符
- sourceDeviceId: String - 源设备 ID
- targetDeviceIds: List[String] - 目标设备 ID 列表
- content: String - 同步的文本内容
- contentType: ContentType - 内容类型（文本/链接）
- timestamp: Timestamp - 同步时间戳
- status: SyncStatus - 同步状态
- priority: SyncPriority - 同步优先级
- retryCount: Int32 - 重试次数

### Protocol Buffers 定义

```protobuf
message SyncRecord {
  string sync_id = 1;
  string source_device_id = 2;
  repeated string target_device_ids = 3;
  string content = 4;
  ContentType content_type = 5;
  uint64 timestamp = 6;
  SyncStatus status = 7;
  SyncPriority priority = 8;
  int32 retry_count = 9;
  uint64 expires_at = 10;         // 过期时间
  string checksum = 11;           // 内容校验和
}

enum ContentType {
  CONTENT_TYPE_UNKNOWN = 0;
  TEXT = 1;
  URL = 2;
}

enum SyncStatus {
  SYNC_STATUS_UNKNOWN = 0;
  PENDING = 1;
  IN_PROGRESS = 2;
  COMPLETED = 3;
  FAILED = 4;
  EXPIRED = 5;
}

enum SyncPriority {
  PRIORITY_UNKNOWN = 0;
  LOW = 1;
  NORMAL = 2;
  HIGH = 3;
  URGENT = 4;
}
```

### Rust 结构体定义

```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub sync_id: String,
    pub source_device_id: String,
    pub target_device_ids: Vec<String>,
    pub content: String,
    pub content_type: ContentType,
    pub timestamp: SystemTime,
    pub status: SyncStatus,
    pub priority: SyncPriority,
    pub retry_count: i32,
    pub expires_at: SystemTime,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Unknown,
    Text,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Unknown,
    Pending,
    InProgress,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncPriority {
    Unknown,
    Low,
    Normal,
    High,
    Urgent,
}
```

### 关系关系

- SyncRecord n:1 Device - 同步记录关联到源设备和目标设备

## PairingRequest 配对请求模型

**目的：** 管理设备配对过程中的请求和验证信息。

**Key Attributes:**
- requestId: String - 配对请求唯一标识符
- initiatorDeviceId: String - 发起配对的设备 ID
- targetDeviceId: String - 目标设备 ID
- pairingCode: String - 配对码或 QR 码内容
- timestamp: Timestamp - 请求时间戳
- status: PairingStatus - 配对状态

### TypeScript 接口

```typescript
interface PairingRequest {
  requestId: string;
  initiatorDeviceId: string;
  targetDeviceId: string;
  pairingCode: string;
  timestamp: number;
  status: 'pending' | 'accepted' | 'rejected' | 'expired';
}

enum PairingStatus {
  PENDING = 'pending',
  ACCEPTED = 'accepted',
  REJECTED = 'rejected',
  EXPIRED = 'expired'
}
```

### 关系关系

- PairingRequest 1:1 Device - 配对请求关联到发起设备和目标设备
