# NearClip 开发指导文档

## 简介

本文档为 NearClip 项目提供全面的开发指导，专注于代码实现、具体代码模板、测试策略和最佳实践。基于已完成的 PRD 和架构文档，本指南为 AI 代理和开发团队提供具体的实施路径。

### 文档范围

- Monorepo 结构搭建和基础配置
- 核心通信协议实现指导
- 具体代码模板和实施示例
- 测试策略和验证方法
- 最佳实践和常见陷阱

### 变更日志

| 日期 | 版本 | 描述 | 作者 |
|------|------|------|------|
| 2025-01-15 | 1.0 | 初始开发指导文档 | Winston (BMAD Architect) |

## 快速参考 - 关键文件和模板

### 核心实现文件优先级

1. **Protocol Buffers 定义**: `shared/protobuf/` - 跨平台消息协议
2. **Rust 核心逻辑**: `shared/rust/src/` - 加密、传输控制
3. **生成代码**: `shared/generated/` - 自动生成的 Kotlin/Swift 代码
4. **Android 集成**: `android/app/src/main/java/com/nearclip/`
5. **Mac 集成**: `mac/NearClip/Sources/`

### MVP 实施优先级

基于 PRD 史诗 1 的故事优先级：
1. 项目基础设施 (1.1-1.3)
2. 通信协议设计 (1.4)
3. BLE 通信能力验证 (1.5)

## Monorepo 项目结构初始化

### 实际项目结构

```
nearclip/
├── .github/                           # CI/CD 工作流
│   └── workflows/
│       ├── ci-android.yaml
│       └── ci-mac.yaml
├── shared/                            # 共享协议和工具
│   ├── protobuf/                     # Protocol Buffers 定义
│   │   ├── nearclip.proto             # 消息协议定义
│   │   ├── device.proto              # 设备相关协议
│   │   └── sync.proto                # 同步相关协议
│   ├── rust/                          # Rust 核心模块
│   │   ├── Cargo.toml                # Rust 项目配置
│   │   ├── src/
│   │   │   ├── lib.rs                 # 库入口
│   │   │   ├── protocol/              # 协议处理
│   │   │   │   ├── mod.rs
│   │   │   │   ├── serialization.rs  # Protobuf 序列化
│   │   │   │   └── validation.rs      # 消息验证
│   │   │   ├── crypto/                # 加密模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── rsa.rs            # RSA 加密
│   │   │   │   ├── aes.rs            # AES 加密
│   │   │   │   ├── key_management.rs # 密钥管理
│   │   │   │   └── signature.rs      # 数字签名
│   │   │   ├── ble/                   # BLE 通信控制
│   │   │   │   ├── mod.rs
│   │   │   │   ├── discovery.rs      # 设备发现
│   │   │   │   ├── connection.rs     # 连接管理
│   │   │   │   ├── advertising.rs    # 广播功能
│   │   │   │   └── packet_handler.rs # 数据包处理
│   │   │   ├── transport/              # 传输层抽象
│   │   │   │   ├── mod.rs
│   │   │   │   ├── ble_transport.rs  # BLE 传输实现
│   │   │   │   └── transport_trait.rs # 传输接口定义
│   │   │   ├── utils/                 # 工具函数
│   │   │   │   ├── mod.rs
│   │   │   │   ├── error.rs           # 错误处理
│   │   │   │   ├── logger.rs          # 日志系统
│   │   │   │   └── constants.rs       # 常量定义
│   │   │   └── c_api/                 # C API 接口
│   │   │       ├── lib.rs             # C 库入口
│   │   │       └── ffi.rs             # FFI 绑一接口
│   │   ├── include/                    # C 头文件
│   │   │   └── nearclip.h             # C API 定义
│   │   └── target/                    # 编译目标
│   │       ├── aarch64-linux-android/ # Android ARM64
│   │       ├── x86_64-apple-darwin/   # macOS Intel
│   │       └── aarch64-apple-darwin/   # macOS M1/M2
│   ├── generated/                     # 自动生成的代码
│   │   ├── kotlin/                  # Kotlin 代码生成
│   │   │   └── com/nearclip/protocol/
│   │   └── swift/                   # Swift 代码生成
│   │       └── NearClip/
│   │           └── Generated/
│   └── scripts/                       # 构建脚本
│       ├── build-rust.sh           # Rust 库构建
│       └── generate-proto.sh        # Protobuf 代码生成
├── android/                           # Android 应用
│   ├── app/
│   │   ├── src/
│   │   │   ├── main/
│   │   │   │   ├── java/com/nearclip/
│   │   │   │   │   ├── ui/            # UI 组件
│   │   │   │   │   ├── services/      # 后端服务
│   │   │   │   │   ├── data/          # 数据层
│   │   │   │   │   ├── core/          # 核心功能
│   │   │   │   │   └── MainActivity.kt
│   │   │   │   └── res/               # 资源文件
│   │   │   └── test/                  # 测试代码
│   │   ├── build.gradle.kts
│   │   └── proguard-rules.pro
│   ├── build.gradle.kts
│   └── gradle.properties
├── mac/                               # macOS 应用
│   ├── NearClip/
│   │   ├── Sources/
│   │   │   ├── App/
│   │   │   │   ├── ContentView.swift
│   │   │   │   └── NearClipApp.swift
│   │   │   ├── Core/                  # 核心功能
│   │   │   ├── Services/              # 后端服务
│   │   │   ├── Views/                 # UI 组件
│   │   │   ├── Models/                # 数据模型
│   │   │   └── Utils/                 # 工具类
│   │   ├── Tests/
│   │   └── Package.swift
│   └── NearClip.xcodeproj/
├── scripts/                           # 构建和部署脚本
│   ├── build-android.sh
│   ├── build-mac.sh
│   └── test.sh
├── docs/                              # 文档
│   ├── prd.md
│   ├── front-end-spec.md
│   ├── architecture/
│   └── development-guide.md
├── .env.example                       # 环境变量模板
├── .gitignore
├── README.md
└── LICENSE
```

### 关键模块初始化模板

#### shared/protocol/MessageTypes.kt

```kotlin
/**
 * NearClip 消息类型定义
 * 跨平台通信协议的标准消息格式
 */

sealed class NearClipMessage {
    abstract val messageId: String
    abstract val deviceId: String
    abstract val timestamp: Long
    abstract val type: MessageType

    data class DiscoverMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val deviceName: String,
        val deviceType: DeviceType,
        val capabilities: List<Capability>,
        val publicKey: String
    ) : NearClipMessage() {
        override val type = MessageType.DISCOVER
    }

    data class PairRequestMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val targetDeviceId: String,
        val pairingCode: String,
        val publicKey: String
    ) : NearClipMessage() {
        override val type = MessageType.PAIR_REQUEST
    }

    data class PairResponseMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val requestId: String,
        val accepted: Boolean,
        val publicKey: String
    ) : NearClipMessage() {
        override val type = MessageType.PAIR_RESPONSE
    }

    data class SyncMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val content: String,
        val contentType: ContentType,
        val syncId: String
    ) : NearClipMessage() {
        override val type = MessageType.SYNC
    }

    data class AckMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val originalMessageId: String,
        val status: AckStatus,
        val errorCode: String? = null
    ) : NearClipMessage() {
        override val type = MessageType.ACK
    }

    data class ErrorMessage(
        override val messageId: String,
        override val deviceId: String,
        override val timestamp: Long,
        val originalMessageId: String,
        val errorCode: ErrorCode,
        val errorMessage: String
    ) : NearClipMessage() {
        override val type = MessageType.ERROR
    }
}

enum class MessageType {
    DISCOVER, PAIR_REQUEST, PAIR_RESPONSE, SYNC, ACK, ERROR
}

enum class DeviceType {
    ANDROID, MAC
}

enum class Capability {
    BLE, WIFI_DIRECT
}

enum class ContentType {
    TEXT, URL
}

enum class AckStatus {
    SUCCESS, ERROR
}

enum class ErrorCode {
    INVALID_MESSAGE, ENCRYPTION_FAILED, DEVICE_NOT_FOUND, SYNC_FAILED
}
```

#### shared/protocol/Serialization.kt

```kotlin
/**
 * NearClip 消息序列化工具
 * 支持跨平台的 JSON 序列化和反序列化
 */

object MessageSerializer {
    private val gson = Gson()

    fun serialize(message: NearClipMessage): String {
        return when (message) {
            is NearClipMessage.DiscoverMessage ->
                gson.toJson(mapOf(
                    "messageId" to message.messageId,
                    "deviceId" to message.deviceId,
                    "timestamp" to message.timestamp,
                    "type" to message.type.name,
                    "deviceName" to message.deviceName,
                    "deviceType" to message.deviceType.name,
                    "capabilities" to message.capabilities.map { it.name },
                    "publicKey" to message.publicKey
                ))

            is NearClipMessage.PairRequestMessage ->
                gson.toJson(mapOf(
                    "messageId" to message.messageId,
                    "deviceId" to message.deviceId,
                    "timestamp" to message.timestamp,
                    "type" to message.type.name,
                    "targetDeviceId" to message.targetDeviceId,
                    "pairingCode" to message.pairingCode,
                    "publicKey" to message.publicKey
                ))

            // ... 其他消息类型的序列化
            else -> gson.toJson(message)
        }
    }

    fun deserialize(json: String): NearClipMessage {
        val map = gson.fromJson(json, Map::class.java) as Map<String, Any>
        val type = MessageType.valueOf(map["type"] as String)

        return when (type) {
            MessageType.DISCOVER -> createDiscoverMessage(map)
            MessageType.PAIR_REQUEST -> createPairRequestMessage(map)
            MessageType.PAIR_RESPONSE -> createPairResponseMessage(map)
            MessageType.SYNC -> createSyncMessage(map)
            MessageType.ACK -> createAckMessage(map)
            MessageType.ERROR -> createErrorMessage(map)
        }
    }

    private fun createDiscoverMessage(map: Map<String, Any>): NearClipMessage.DiscoverMessage {
        return NearClipMessage.DiscoverMessage(
            messageId = map["messageId"] as String,
            deviceId = map["deviceId"] as String,
            timestamp = (map["timestamp"] as Double).toLong(),
            deviceName = map["deviceName"] as String,
            deviceType = DeviceType.valueOf(map["deviceType"] as String),
            capabilities = (map["capabilities"] as List<String>).map { Capability.valueOf(it) },
            publicKey = map["publicKey"] as String
        )
    }

    // ... 其他消息类型的创建方法
}
```

## Protocol Buffers 协议定义

### 消息协议标准 (shared/protobuf/nearclip.proto)

```protobuf
syntax = "proto3";

package nearclip;

// 通用消息包装
message NearClipMessage {
  string message_id = 1;
  string device_id = 2;
  int64 timestamp = 3;
  MessageType type = 4;
  bytes signature = 5;
  oneof payload {
    DiscoverMessage discover = 6;
    PairRequestMessage pair_request = 7;
    PairResponseMessage pair_response = 8;
    SyncMessage sync = 9;
    AckMessage ack = 10;
    ErrorMessage error = 11;
  }
}

// 消息类型枚举
enum MessageType {
  DISCOVER = 0;
  PAIR_REQUEST = 1;
  PAIR_RESPONSE = 2;
  SYNC = 3;
  ACK = 4;
  ERROR = 5;
}

// 设备发现消息
message DiscoverMessage {
  string device_name = 1;
  DeviceType device_type = 2;
  repeated Capability capabilities = 3;
  string public_key = 4;
}

enum DeviceType {
  ANDROID = 0;
  MAC = 1;
}

enum Capability {
  BLE = 0;
  WIFI_DIRECT = 1;
}

// 配对请求消息
message PairRequestMessage {
  string target_device_id = 1;
  string pairing_code = 2;
  string public_key = 3;
}

// 配对响应消息
message PairResponseMessage {
  string request_id = 1;
  bool accepted = 2;
  string public_key = 3;
}

// 同步消息
message SyncMessage {
  string content = 1;
  ContentType content_type = 2;
  string sync_id = 3;
}

enum ContentType {
  TEXT = 0;
  URL = 1;
}

// 确认消息
message AckMessage {
  string original_message_id = 1;
  AckStatus status = 2;
  string error_code = 3;
}

enum AckStatus {
  SUCCESS = 0;
  ERROR = 1;
}

// 错误消息
message ErrorMessage {
  string original_message_id = 1;
  ErrorCode error_code = 2;
  string error_message = 3;
}

enum ErrorCode {
  INVALID_MESSAGE = 0;
  ENCRYPTION_FAILED = 1;
  DEVICE_NOT_FOUND = 2;
  SYNC_FAILED = 3;
  TIMEOUT = 4;
}
```

### 设备相关协议 (shared/protobuf/device.proto)

```protobuf
syntax = "proto3";

package nearclip;

import "nearclip.proto";

// 设备信息
message DeviceInfo {
  string device_id = 1;
  string device_name = 2;
  DeviceType device_type = 3;
  repeated Capability capabilities = 4;
  string public_key = 5;
  string alias = 6;
  int64 last_seen = 7;
  ConnectionStatus connection_status = 8;
}

enum ConnectionStatus {
  DISCONNECTED = 0;
  CONNECTING = 1;
  CONNECTED = 2;
  PAIRING = 3;
  ERROR = 4;
}

// 设备发现广播
message DeviceBroadcast {
  DeviceInfo device_info = 1;
  int32 rssi = 2;
  repeated string service_uuids = 3;
  map<string, bytes> service_data = 4;
}

// 设备列表响应
message DeviceListResponse {
  repeated DeviceInfo devices = 1;
  string cursor = 2;
  bool has_more = 3;
}
```

### 同步相关协议 (shared/protobuf/sync.proto)

```protobuf
syntax = "proto3";

package nearclip;

import "nearclip.proto";

// 同步内容
message SyncContent {
  string content = 1;
  ContentType content_type = 2;
  SyncMetadata metadata = 3;
}

// 同步元数据
message SyncMetadata {
  string sync_id = 1;
  int64 created_at = 2;
  string source_device_name = 3;
  repeated string target_device_ids = 4;
  bool requires_ack = 5;
}

// 同步历史记录
message SyncHistory {
  repeated SyncRecord records = 1;
  string cursor = 2;
  bool has_more = 3;
}

message SyncRecord {
  string sync_id = 1;
  string source_device_id = 2;
  string content = 1;
  ContentType content_type = 2;
  int64 timestamp = 3;
  SyncStatus status = 4;
  repeated string target_device_ids = 5;
}

enum SyncStatus {
  PENDING = 0;
  SUCCESS = 1;
  FAILED = 2;
  CANCELLED = 3;
}
```

## Rust 核心逻辑实现

### Rust 项目配置 (shared/rust/Cargo.toml)

```toml
[package]
name = "nearclip-core"
version = "0.1.0"
edition = "2021"
authors = ["NearClip Team"]
description = "NearClip core library with Protocol Buffers and cryptography"

[lib]
name = "nearclip_core"
crate-type = ["cdylib", "rlib"]

[dependencies]
# Protocol Buffers
prost = { version = "0.11", features = ["serde"] }
prost-types = "0.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 加密库
ring = "0.16"
base64 = "0.21"
sha2 = "0.10"

# 异步运行时
tokio = { version = "1.0", features = ["full", "rt-multi-thread"] }
tokio-util = "0.7"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 日志
log = "0.4"
env_logger = "0.10"

# UUID 生成
uuid = { version = "1.0", features = ["v4"] }

# FFI 相关
libc = "0.2"
lazy_static = "1.4"

[build-dependencies]
cbindgen = "0.24"
prost-build = "0.11"
```

### Protocol Buffers 处理 (shared/rust/src/protocol/serialization.rs)

```rust
use prost::Message;
use crate::protocol::nearclip::{NearClipMessage, MessageType};
use crate::protocol::device::DeviceInfo;
use crate::protocol::sync::SyncContent;
use crate::utils::error::NearClipError;

/// Protocol Buffers 序列化工具
pub struct MessageSerializer;

impl MessageSerializer {
    /// 序列化消息到字节
    pub fn serialize_message(message: &NearClipMessage) -> Result<Vec<u8>, NearClipError> {
        match message.encode() {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(NearClipError::SerializationError(
                format!("Failed to serialize message: {}", e)
            )),
        }
    }

    /// 反序列化字节到消息
    pub fn deserialize_message(data: &[u8]) -> Result<NearClipMessage, NearClipError> {
        match NearClipMessage::decode(data) {
            Ok(message) => {
                // 验证消息完整性
                Self::validate_message(&message)?;
                Ok(message)
            }
            Err(e) => Err(NearClipError::SerializationError(
                format!("Failed to deserialize message: {}", e)
            )),
        }
    }

    /// 验证消息完整性
    pub fn validate_message(message: &NearClipMessage) -> Result<(), NearClipError> {
        // 检查必需字段
        if message.message_id.is_empty() {
            return Err(NearClipError::ValidationError("Missing message_id"));
        }

        if message.device_id.is_empty() {
            return Err(NearClipError::ValidationError("Missing device_id"));
        }

        if message.timestamp == 0 {
            return Err(NearClipError::ValidationError("Invalid timestamp"));
        }

        // 检查消息签名
        if !Self::verify_signature(message) {
            return Err(NearClipError::ValidationError("Invalid signature"));
        }

        Ok(())
    }

    /// 验证消息签名
    fn verify_signature(message: &NearClipMessage) -> bool {
        // TODO: 实现数字签名验证
        // 这里需要验证消息的数字签名
        // 使用公钥验证消息的完整性和来源
        true // 暂时返回 true，实际实现需要加密支持
    }

    /// 获取消息大小（字节）
    pub fn get_message_size(message: &NearClipMessage) -> usize {
        message.encoded_len()
    }

    /// 检查消息是否适合 BLE 传输
    pub fn is_ble_safe(message: &NearClipMessage) -> bool {
        const MAX_BLE_MESSAGE_SIZE: usize = 512; // 512 字节限制
        Self::get_message_size(message) <= MAX_BLE_MESSAGE_SIZE
    }

    /// 分割大消息为 BLE 友传输
    pub fn split_message(
        message: &NearClipMessage,
        chunk_size: usize,
    ) -> Result<Vec<Vec<u8>>, NearClipError> {
        let data = Self::serialize_message(message)?;
        let chunks = data.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();
        Ok(chunks)
    }
}
```

#### 设备发现协议

```kotlin
// Android 端设备发现实现
class DeviceDiscoveryService(
    private val bluetoothAdapter: BluetoothAdapter,
    private val messageHandler: MessageHandler
) {
    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            val device = result.device
            val scanRecord = result.scanRecord

            // 验证是否为 NearClip 设备
            if (isNearClipDevice(scanRecord)) {
                handleNearClipDevice(device, scanRecord)
            }
        }

        override fun onScanFailed(errorCode: Int) {
            handleScanError(errorCode)
        }
    }

    fun startDiscovery(): Flow<DiscoveredDevice> = callbackFlow {
        val scanner = bluetoothAdapter.bluetoothLeScanner
        val settings = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
            .build()

        scanner.startScan(emptyList(), settings, scanCallback)

        awaitClose { scanner.stopScan(scanCallback) }
    }

    private fun isNearClipDevice(scanRecord: ScanRecord?): Boolean {
        return scanRecord?.serviceUuids?.contains(SERVICE_UUID) == true
    }

    private fun handleNearClipDevice(device: BluetoothDevice, scanRecord: ScanRecord) {
        val deviceInfo = DiscoveredDevice(
            deviceId = device.address,
            deviceName = device.name ?: "Unknown",
            deviceType = detectDeviceType(scanRecord),
            rssi = scanRecord.rssi,
            serviceData = extractServiceData(scanRecord)
        )

        trySend(deviceInfo)
    }
}

// Mac 端设备发现实现
class DeviceDiscoveryManager: NSObject, CBCentralManagerDelegate {
    private var centralManager: CBCentralManager!
    private var discoveredDevices: [String: DiscoveredDevice] = [:]

    override init() {
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        if central.state == .poweredOn {
            startScanning()
        }
    }

    func startScanning() {
        centralManager.scanForPeripherals(
            withServices: [CBUUID(string: SERVICE_UUID)],
            options: [CBCentralManagerScanOptionAllowDuplicatesKey: false]
        )
    }

    func centralManager(
        _ central: CBCentralManager,
        didDiscover peripheral: CBPeripheral,
        advertisementData: [String: Any],
        rssi RSSI: NSNumber
    ) {
        let deviceInfo = DiscoveredDevice(
            deviceId: peripheral.identifier.uuidString,
            deviceName: peripheral.name ?? "Unknown",
            deviceType: detectDeviceType(advertisementData),
            rssi: RSSI.intValue,
            serviceData: extractServiceData(advertisementData)
        )

        discoveredDevices[deviceInfo.deviceId] = deviceInfo
        notifyDeviceDiscovered(deviceInfo)
    }
}
```

### 配对认证流程

#### 公钥加密配对实现

```kotlin
class DevicePairingManager(
    private val cryptoService: CryptoService,
    private val messageHandler: MessageHandler
) {
    suspend fun initiatePairing(
        targetDevice: DiscoveredDevice,
        onPairingCodeGenerated: (String) -> Unit
    ): Result<PairingResult> {
        return try {
            // 1. 生成临时配对信息
            val pairingCode = generateSecurePairingCode()
            val ephemeralKeyPair = cryptoService.generateEphemeralKeypair()

            onPairingCodeGenerated(pairingCode)

            // 2. 创建配对请求消息
            val pairRequest = NearClipMessage.PairRequestMessage(
                messageId = UUID.randomUUID().toString(),
                deviceId = getCurrentDeviceId(),
                timestamp = System.currentTimeMillis(),
                targetDeviceId = targetDevice.deviceId,
                pairingCode = pairingCode,
                publicKey = ephemeralKeyPair.publicKey
            )

            // 3. 发送配对请求
            val sendResult = messageHandler.sendMessage(targetDevice, pairRequest)

            if (sendResult.isSuccess) {
                // 4. 等待配对响应
                val response = waitForPairingResponse(pairRequest.messageId)
                PairingResult.SUCCESS
            } else {
                Result.failure(sendResult.exceptionOrNull() ?: Exception("Failed to send pairing request"))
            }
        } catch (error: Exception) {
            Result.failure(error)
        }
    }

    suspend fun handlePairingRequest(
        message: NearClipMessage.PairRequestMessage
    ): Result<NearClipMessage.PairResponseMessage> {
        return try {
            // 1. 验证配对请求
            validatePairingRequest(message)

            // 2. 显示配对确认给用户
            val userConfirmed = showPairingConfirmation(message)

            if (userConfirmed) {
                // 3. 生成响应
                val responseKeyPair = cryptoService.generateKeypair()
                val response = NearClipMessage.PairResponseMessage(
                    messageId = UUID.randomUUID().toString(),
                    deviceId = getCurrentDeviceId(),
                    timestamp = System.currentTimeMillis(),
                    requestId = message.messageId,
                    accepted = true,
                    publicKey = responseKeyPair.publicKey
                )

                // 4. 保存配对信息
                savePairedDevice(message.deviceId, message.publicKey)

                Result.success(response)
            } else {
                Result.rejectPairing(message.messageId)
            }
        } catch (error: Exception) {
            Result.failure(error)
        }
    }

    private fun generateSecurePairingCode(): String {
        // 生成 6 位数字配对码
        return (100000..999999).random().toString()
    }

    private suspend fun waitForPairingResponse(
        requestId: String,
        timeoutMs: Long = 30000L
    ): NearClipMessage.PairResponseMessage {
        // 实现等待配对响应的逻辑
        // 使用超时机制避免无限等待
    }
}

// Swift 版本的配对管理器
class DevicePairingManager: NSObject {
    private let cryptoService: CryptoService
    private let messageHandler: MessageHandler

    init(cryptoService: CryptoService, messageHandler: MessageHandler) {
        self.cryptoService = cryptoService
        self.messageHandler = messageHandler
    }

    func initiatePairing(
        with targetDevice: DiscoveredDevice,
        completion: @escaping (Result<PairingResult, Error>) -> Void
    ) {
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                // 1. 生成临时配对信息
                let pairingCode = self.generateSecurePairingCode()
                let ephemeralKeyPair = try self.cryptoService.generateEphemeralKeypair()

                DispatchQueue.main.async {
                    // 2. 显示配对码给用户
                    self.showPairingCode(pairingCode)
                }

                // 3. 创建配对请求消息
                let pairRequest = PairRequestMessage(
                    messageId: UUID().uuidString,
                    deviceId: self.getCurrentDeviceId(),
                    timestamp: Date().timeIntervalSince1970,
                    targetDeviceId: targetDevice.deviceId,
                    pairingCode: pairingCode,
                    publicKey: ephemeralKeyPair.publicKey
                )

                // 4. 发送配对请求
                let sendResult = try await self.messageHandler.sendMessage(
                    to: targetDevice,
                    message: pairRequest
                )

                if sendResult {
                    // 5. 等待配对响应
                    let response = try await self.waitForPairingResponse(
                        requestId: pairRequest.messageId,
                        timeout: 30.0
                    )

                    DispatchQueue.main.async {
                        completion(.success(.initiated))
                    }
                } else {
                    throw PairingError.failedToSend
                }
            } catch {
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
}
```

## 数据模型和存储

### 跨平台数据模型定义

#### shared/models/Device.kt

```kotlin
/**
 * 设备数据模型
 * 跨平台统一的设备信息表示
 */

data class Device(
    val deviceId: String,
    val deviceName: String,
    val deviceType: DeviceType,
    val publicKey: String,
    val lastSeen: Long = System.currentTimeMillis(),
    val connectionStatus: ConnectionStatus = ConnectionStatus.DISCONNECTED,
    val isTrusted: Boolean = false,
    val alias: String? = null,  // 用户自定义设备别名
    val createdAt: Long = System.currentTimeMillis(),
    val updatedAt: Long = System.currentTimeMillis()
) {
    /**
     * 设备是否在线
     */
    val isOnline: Boolean
        get() = connectionStatus == ConnectionStatus.CONNECTED

    /**
     * 设备显示名称（优先使用别名）
     */
    val displayName: String
        get() = alias ?: deviceName

    /**
     * 更新最后活跃时间
     */
    fun updateLastSeen(): Device {
        return copy(lastSeen = System.currentTimeMillis(), updatedAt = System.currentTimeMillis())
    }

    /**
     * 更新连接状态
     */
    fun updateConnectionStatus(status: ConnectionStatus): Device {
        return copy(
            connectionStatus = status,
            lastSeen = if (status == ConnectionStatus.CONNECTED) System.currentTimeMillis() else lastSeen,
            updatedAt = System.currentTimeMillis()
        )
    }

    /**
     * 设置设备别名
     */
    fun setAlias(newAlias: String?): Device {
        return copy(alias = newAlias, updatedAt = System.currentTimeMillis())
    }
}

enum class DeviceType(val displayName: String) {
    ANDROID("Android"),
    MAC("macOS");

    companion object {
        fun fromString(value: String): DeviceType {
            return values().find { it.name.equals(value, ignoreCase = true) }
                ?: throw IllegalArgumentException("Unknown device type: $value")
        }
    }
}

enum class ConnectionStatus(val displayName: String) {
    DISCONNECTED("已断开"),
    CONNECTING("连接中"),
    CONNECTED("已连接"),
    PAIRING("配对中"),
    ERROR("错误");

    companion object {
        fun fromString(value: String): ConnectionStatus {
            return values().find { it.name.equals(value, ignoreCase = true) }
                ?: throw IllegalArgumentException("Unknown connection status: $value")
        }
    }
}
```

#### 数据访问层接口

```kotlin
/**
 * 设备数据访问接口
 * 统一的数据访问抽象，支持不同实现
 */

interface DeviceRepository {
    /**
     * 获取所有设备
     */
    fun getAllDevices(): Flow<List<Device>>

    /**
     * 获取已连接的设备
     */
    fun getConnectedDevices(): Flow<List<Device>>

    /**
     * 根据设备ID获取设备
     */
    suspend fun getDeviceById(deviceId: String): Device?

    /**
     * 保存或更新设备
     */
    suspend fun saveDevice(device: Device): Result<Unit>

    /**
     * 删除设备
     */
    suspend fun deleteDevice(deviceId: String): Result<Unit>

    /**
     * 更新连接状态
     */
    suspend fun updateConnectionStatus(deviceId: String, status: ConnectionStatus): Result<Unit>

    /**
     * 获取信任的设备
     */
    fun getTrustedDevices(): Flow<List<Device>>

    /**
     * 设置设备信任状态
     */
    suspend fun setDeviceTrusted(deviceId: String, trusted: Boolean): Result<Unit>
}

// Android Room 实现
@Dao
interface DeviceDao {
    @Query("SELECT * FROM devices ORDER BY lastSeen DESC")
    fun getAllDevices(): Flow<List<DeviceEntity>>

    @Query("SELECT * FROM devices WHERE connectionStatus = 'CONNECTED'")
    fun getConnectedDevices(): Flow<List<DeviceEntity>>

    @Query("SELECT * FROM devices WHERE deviceId = :deviceId")
    suspend fun getDeviceById(deviceId: String): DeviceEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertOrUpdateDevice(device: DeviceEntity): Long

    @Query("DELETE FROM devices WHERE deviceId = :deviceId")
    suspend fun deleteDevice(deviceId: String)

    @Query("UPDATE devices SET connectionStatus = :status, lastSeen = :timestamp WHERE deviceId = :deviceId")
    suspend fun updateConnectionStatus(deviceId: String, status: String, timestamp: Long)

    @Query("SELECT * FROM devices WHERE isTrusted = 1")
    fun getTrustedDevices(): Flow<List<DeviceEntity>>

    @Query("UPDATE devices SET isTrusted = :trusted WHERE deviceId = :deviceId")
    suspend fun setDeviceTrusted(deviceId: String, trusted: Boolean)
}

// Room 数据库实体
@Entity(tableName = "devices")
data class DeviceEntity(
    @PrimaryKey val deviceId: String,
    val deviceName: String,
    val deviceType: String,
    val publicKey: String,
    val lastSeen: Long,
    val connectionStatus: String,
    val isTrusted: Boolean,
    val alias: String?,
    val createdAt: Long,
    val updatedAt: Long
)

// 数据库到领域模型的映射
fun DeviceEntity.toDevice(): Device {
    return Device(
        deviceId = deviceId,
        deviceName = deviceName,
        deviceType = DeviceType.fromString(deviceType),
        publicKey = publicKey,
        lastSeen = lastSeen,
        connectionStatus = ConnectionStatus.fromString(connectionStatus),
        isTrusted = isTrusted,
        alias = alias,
        createdAt = createdAt,
        updatedAt = updatedAt
    )
}

fun Device.toEntity(): DeviceEntity {
    return DeviceEntity(
        deviceId = deviceId,
        deviceName = deviceName,
        deviceType = deviceType.name,
        publicKey = publicKey,
        lastSeen = lastSeen,
        connectionStatus = connectionStatus.name,
        isTrusted = isTrusted,
        alias = alias,
        createdAt = createdAt,
        updatedAt = updatedAt
    )
}
```

## 核心工作流实现

### 设备配对工作流

#### Android 端实现

```kotlin
/**
 * Android 设备配对工作流
 * 完整的配对流程处理，包括错误恢复和用户交互
 */

class DevicePairingWorkflow(
    private val discoveryService: DeviceDiscoveryService,
    private val pairingManager: DevicePairingManager,
    private val deviceRepository: DeviceRepository
) {
    private val _pairingState = MutableStateFlow<PairingState>(PairingState.Idle)
    val pairingState: StateFlow<PairingState> = _pairingState.asStateFlow()

    /**
     * 启动设备发现
     */
    fun startDeviceDiscovery(): Flow<DiscoveredDevice> {
        _pairingState.value = PairingState.Discovering
        return discoveryService.startDiscovery()
            .onEach { _pairingState.value = PairingState.DevicesFound(listOf(it)) }
            .catch { error ->
                _pairingState.value = PairingState.Error(error.message ?: "Discovery failed")
            }
    }

    /**
     * 发起配对
     */
    suspend fun initiatePairing(device: DiscoveredDevice): Result<PairingResult> {
        _pairingState.value = PairingState.Pairing(device.displayName)

        return pairingManager.initiatePairing(device) { pairingCode ->
            _pairingState.value = PairingState.ShowPairingCode(device.displayName, pairingCode)
        }.fold(
            onSuccess = { result ->
                _pairingState.value = PairingState.Paired(device.displayName)
                Result.success(result)
            },
            onFailure = { error ->
                _pairingState.value = PairingState.Error(error.message ?: "Pairing failed")
                Result.failure(error)
            }
        )
    }

    /**
     * 处理传入的配对请求
     */
    suspend fun handlePairingRequest(
        request: NearClipMessage.PairRequestMessage
    ): Result<NearClipMessage.PairResponseMessage> {
        _pairingState.value = PairingState.IncomingRequest(
            deviceName = extractDeviceNameFromRequest(request),
            pairingCode = request.pairingCode
        )

        return pairingManager.handlePairingRequest(request).fold(
            onSuccess = { response ->
                _pairingState.value = PairingState.Paired(extractDeviceNameFromRequest(request))
                Result.success(response)
            },
            onFailure = { error ->
                _pairingState.value = PairingState.Error(error.message ?: "Failed to handle pairing request")
                Result.failure(error)
            }
        )
    }

    /**
     * 取消配对
     */
    fun cancelPairing() {
        _pairingState.value = PairingState.Idle
    }
}

sealed class PairingState {
    object Idle : PairingState()
    object Discovering : PairingState()
    data class DevicesFound(val devices: List<String>) : PairingState()
    data class Pairing(val deviceName: String) : PairingState()
    data class ShowPairingCode(val deviceName: String, val code: String) : PairingState()
    data class IncomingRequest(val deviceName: String, val pairingCode: String) : PairingState()
    data class Paired(val deviceName: String) : PairingState()
    data class Error(val message: String) : PairingState()
}
```

#### Mac 端实现

```swift
/**
 * macOS 设备配对工作流
 * 使用 Combine 框架的响应式实现
 */

class DevicePairingWorkflow: ObservableObject {
    @Published var pairingState: PairingState = .idle

    private let discoveryService: DeviceDiscoveryService
    private let pairingManager: DevicePairingManager
    private var cancellables = Set<AnyCancellable>()

    init(discoveryService: DeviceDiscoveryService, pairingManager: DevicePairingManager) {
        self.discoveryService = discoveryService
        self.pairingManager = pairingManager
    }

    /**
     * 启动设备发现
     */
    func startDeviceDiscovery() -> AnyPublisher<DiscoveredDevice, Never> {
        pairingState = .discovering

        return discoveryService.startDiscovery()
            .handleEvents(receiveSubscription: { _ in
                pairingState = .discovering
            }, receiveOutput: { device in
                pairingState = .devicesFound([device.displayName])
            }, receiveCompletion: { completion in
                if case .failure(let error) = completion {
                    pairingState = .error(error.localizedDescription)
                }
            }, receiveCancel: {
                pairingState = .idle
            })
            .eraseToAnyPublisher()
    }

    /**
     * 发起配对
     */
    func initiatePairing(with device: DiscoveredDevice) {
        pairingState = .pairing(device.displayName)

        pairingManager.initiatePairing(with: device) { [weak self] result in
            DispatchQueue.main.async {
                switch result {
                case .success:
                    self?.pairingState = .paired(device.displayName)
                case .failure(let error):
                    self?.pairingState = .error(error.localizedDescription)
                }
            }
        }

        pairingManager.onPairingCodeGenerated = { [weak self] code in
            DispatchQueue.main.async {
                self?.pairingState = .showPairingCode(deviceName: device.displayName, code: code)
            }
        }
    }

    /**
     * 处理传入的配对请求
     */
    func handlePairingRequest(_ request: PairRequestMessage) {
        let deviceName = extractDeviceName(from: request)
        pairingState = .incomingRequest(deviceName: deviceName, pairingCode: request.pairingCode)

        pairingManager.handlePairingRequest(request) { [weak self] result in
            DispatchQueue.main.async {
                switch result {
                case .success:
                    self?.pairingState = .paired(deviceName: deviceName)
                case .failure(let error):
                    self?.pairingState = .error(error.localizedDescription)
                }
            }
        }
    }

    /**
     * 取消配对
     */
    func cancelPairing() {
        pairingState = .idle
        cancellables.removeAll()
    }
}

enum PairingState: Equatable {
    case idle
    case discovering
    case devicesFound([String])
    case pairing(String)
    case showPairingCode(deviceName: String, code: String)
    case incomingRequest(deviceName: String, pairingCode: String)
    case paired(String)
    case error(String)
}
```

## 测试策略和实现

### 测试组织结构

```
shared/testing/
├── TestDevices.kt              # 测试设备模拟
├── TestUtils.kt                # 测试工具类
├── TestData.kt                 # 测试数据生成
├── MockBluetoothService.kt      # BLE 服务模拟
├── TestScenarios.kt             # 测试场景定义
└── IntegrationTestBase.kt       # 集成测试基类

android/app/src/test/
├── integration/
│   ├── DevicePairingTest.kt     # 设备配对集成测试
│   ├── SyncWorkflowTest.kt      # 同步工作流测试
│   └── CommunicationTest.kt      # 通信协议测试
├── unit/
│   ├── services/
│   ├── data/
│   └── ui/
└── androidTest/
    ├── EndToEndTest.kt          # 端到端测试
    └── UITest.kt                 # UI 测试

mac/NearClip/Tests/
├── Integration/
│   ├── DevicePairingTests.swift # 设备配对集成测试
│   ├── SyncWorkflowTests.swift  # 同步工作流测试
│   └── CommunicationTests.swift # 通信协议测试
├── Unit/
│   ├── Services/
│   ├── Models/
│   └── Views/
└── UITests/
    ├── NearClipUITests.swift    # UI 测试
    └── EndToEndTests.swift       # 端到端测试
```

### 设备配对集成测试

```kotlin
/**
 * 设备配对集成测试
 * 模拟完整的设备发现、配对、连接流程
 */

@RunWith(AndroidJUnit4::class)
class DevicePairingIntegrationTest {

    @get:Rule
    val coroutineTestRule = CoroutineTestRule()

    private lateinit var mockBluetoothService: MockBluetoothService
    private lateinit var pairingWorkflow: DevicePairingWorkflow
    private lateinit var testDevice1: MockBluetoothDevice
    private lateinit var testDevice2: MockBluetoothDevice

    @Before
    fun setup() {
        mockBluetoothService = MockBluetoothService()
        val discoveryService = DeviceDiscoveryService(mockBluetoothService, mockMessageHandler)
        val pairingManager = DevicePairingManager(mockCryptoService, mockMessageHandler)
        val deviceRepository = MockDeviceRepository()

        pairingWorkflow = DevicePairingWorkflow(discoveryService, pairingManager, deviceRepository)

        // 设置测试设备
        testDevice1 = MockBluetoothDevice(
            address = "AA:BB:CC:DD:EE:FF",
            name = "Test Android",
            type = DeviceType.ANDROID
        )

        testDevice2 = MockBluetoothDevice(
            address = "11:22:33:44:55:66",
            name = "Test Mac",
            type = DeviceType.MAC
        )
    }

    @Test
    fun `complete pairing workflow should succeed`() = runBlocking {
        // 1. 设备1启动发现
        val discoveryResults = mutableListOf<DiscoveredDevice>()
        val discoveryJob = launch {
            pairingWorkflow.startDeviceDiscovery().collect { device ->
                discoveryResults.add(device)
            }
        }

        // 2. 设备2开始广播
        mockBluetoothService.startAdvertising(testDevice2)

        // 3. 等待设备1发现设备2
        delay(1000)
        assertTrue(discoveryResults.isNotEmpty())

        // 4. 设备1发起配对
        val targetDevice = discoveryResults.first { it.deviceId == testDevice2.address }
        val pairingResult = pairingWorkflow.initiatePairing(targetDevice)

        assertTrue(pairingResult.isSuccess)

        // 5. 验证配对状态变化
        pairingWorkflow.pairingState.test {
            assertEquals(PairingState.Discovering, awaitItem())
            assertEquals(PairingState.DevicesFound(listOf(testDevice2.name)), awaitItem())
            assertEquals(PairingState.Pairing(testDevice2.name), awaitItem())
            assertEquals(PairingState.ShowPairingCode(testDevice2.name, any()), awaitItem())
            assertEquals(PairingState.Paired(testDevice2.name), awaitItem())
        }

        discoveryJob.cancel()
    }

    @Test
    fun `pairing timeout should handle gracefully`() = runBlocking {
        // 1. 启动发现但无设备响应
        val discoveryResults = mutableListOf<DiscoveredDevice>()
        val discoveryJob = launch {
            pairingWorkflow.startDeviceDiscovery().collect { device ->
                discoveryResults.add(device)
            }
        }

        // 2. 发现设备但不响应配对
        mockBluetoothService.startAdvertising(testDevice2, shouldRespond = false)
        delay(1000)

        // 3. 尝试配对
        if (discoveryResults.isNotEmpty()) {
            val targetDevice = discoveryResults.first()
            val pairingResult = pairingWorkflow.initiatePairing(targetDevice)

            // 4. 验证超时处理
            assertTrue(pairingResult.isFailure)
            assertTrue(pairingResult.exceptionOrNull() is TimeoutException)

            // 5. 验证状态恢复到空闲
            assertEquals(PairingState.Idle, pairingWorkflow.pairingState.value)
        }

        discoveryJob.cancel()
    }

    @Test
    fun `multiple concurrent pairing requests should be handled correctly`() = runBlocking {
        // 1. 设置多个测试设备
        val testDevices = listOf(testDevice1, testDevice2)

        // 2. 同时启动发现
        val discoveryResults = mutableListOf<DiscoveredDevice>()
        val discoveryJob = launch {
            pairingWorkflow.startDeviceDiscovery().collect { device ->
                discoveryResults.add(device)
            }
        }

        // 3. 所有设备开始广播
        testDevices.forEach { mockBluetoothService.startAdvertising(it) }
        delay(1000)

        // 4. 尝试同时配对多个设备
        val pairingResults = discoveryResults.map { device ->
            async {
                pairingWorkflow.initiatePairing(device)
            }
        }

        // 5. 验证所有配对都成功
        val results = pairingResults.awaitAll()
        assertTrue(results.all { it.isSuccess })

        discoveryJob.cancel()
    }
}
```

### Swift 集成测试

```swift
/**
 * 设备配对集成测试 (Swift)
 * 使用 XCTest 框架进行完整的配对流程测试
 */

class DevicePairingIntegrationTests: XCTestCase {

    var pairingWorkflow: DevicePairingWorkflow!
    var mockBluetoothService: MockBluetoothService!
    var mockDiscoveryService: MockDeviceDiscoveryService!
    var expectation: XCTestExpectation!

    override func setUp() {
        super.setUp()

        mockBluetoothService = MockBluetoothService()
        mockDiscoveryService = MockDeviceDiscoveryService()
        let pairingManager = DevicePairingManager(
            cryptoService: MockCryptoService(),
            messageHandler: MockMessageHandler()
        )

        pairingWorkflow = DevicePairingWorkflow(
            discoveryService: mockDiscoveryService,
            pairingManager: pairingManager
        )
    }

    override func tearDown() {
        pairingWorkflow = nil
        mockBluetoothService = nil
        mockDiscoveryService = nil
        super.tearDown()
    }

    func testCompletePairingWorkflow() async throws {
        // 1. 设置期望
        expectation = XCTestExpectation(description: "Complete pairing workflow")

        // 2. 创建测试设备
        let testDevice = DiscoveredDevice(
            deviceId: "AA:BB:CC:DD:EE:FF",
            deviceName: "Test Mac",
            deviceType: .mac,
            rssi: -50
        )

        // 3. 模拟设备发现
        mockDiscoveryService.addDiscoveredDevice(testDevice)

        // 4. 启动发现流程
        let discoveryPublisher = pairingWorkflow.startDeviceDiscovery()

        var discoveredDevices: [DiscoveredDevice] = []
        let cancellable = discoveryPublisher
            .sink { device in
                discoveredDevices.append(device)
            }

        // 5. 等待设备发现
        try await Task.sleep(nanoseconds: 100_000_000) // 0.1 秒
        XCTAssertFalse(discoveredDevices.isEmpty)

        // 6. 启动配对
        let targetDevice = discoveredDevices.first!
        pairingWorkflow.initiatePairing(with: targetDevice)

        // 7. 验证配对状态变化
        var stateHistory: [PairingState] = []
        let stateCancellable = pairingWorkflow.$pairingState
            .sink { state in
                stateHistory.append(state)

                if case .paired = state {
                    expectation.fulfill()
                }
            }

        // 8. 模拟配对成功
        try await Task.sleep(nanoseconds: 500_000_000) // 0.5 秒
        mockDiscoveryService.simulatePairingSuccess(for: targetDevice.deviceId)

        // 9. 验证完整流程
        await fulfillment(of: [expectation], timeout: 2.0)

        XCTAssertEqual(stateHistory.count, 5) // idle -> discovering -> devicesFound -> pairing -> paired
        XCTAssertEqual(stateHistory[0], .idle)
        XCTAssertEqual(stateHistory[1], .discovering)

        if case .devicesFound(let devices) = stateHistory[2] {
            XCTAssertEqual(devices.count, 1)
            XCTAssertEqual(devices.first, testDevice.displayName)
        } else {
            XCTFail("Expected devicesFound state")
        }

        if case .pairing(let deviceName) = stateHistory[3] {
            XCTAssertEqual(deviceName, testDevice.displayName)
        } else {
            XCTFail("Expected pairing state")
        }

        if case .paired(let deviceName) = stateHistory[4] {
            XCTAssertEqual(deviceName, testDevice.displayName)
        } else {
            XCTFail("Expected paired state")
        }

        cancellable.cancel()
        stateCancellable.cancel()
    }

    func testPairingErrorHandling() async throws {
        // 1. 设置期望
        expectation = XCTestExpectation(description: "Pairing error handling")

        // 2. 创建测试设备（将配对失败）
        let testDevice = DiscoveredDevice(
            deviceId: "AA:BB:CC:DD:EE:FF",
            deviceName: "Test Mac",
            deviceType: .mac,
            rssi: -50
        )

        // 3. 模拟设备发现
        mockDiscoveryService.addDiscoveredDevice(testDevice)

        // 4. 启动发现和配对
        let discoveryPublisher = pairingWorkflow.startDeviceDiscovery()
        let discoveryCancellable = discoveryPublisher.sink { _ in }

        let stateCancellable = pairingWorkflow.$pairingState
            .sink { state in
                if case .error(let message) = state {
                    XCTAssertFalse(message.isEmpty)
                    expectation.fulfill()
                }
            }

        // 5. 等待设备发现
        try await Task.sleep(nanoseconds: 100_000_000)

        // 6. 启动配对（将失败）
        let discoveredDevices = mockDiscoveryService.getDiscoveredDevices()
        pairingWorkflow.initiatePairing(with: discoveredDevices.first!)

        // 7. 模拟配对失败
        try await Task.sleep(nanoseconds: 200_000_000)
        mockDiscoveryService.simulatePairingFailure(for: testDevice.deviceId, error: .timeout)

        // 8. 验证错误处理
        await fulfillment(of: [expectation], timeout: 2.0)

        discoveryCancellable.cancel()
        stateCancellable.cancel()
    }
}
```

## 最佳实践和常见陷阱

### 开发最佳实践

#### 1. BLE 通信优化

```kotlin
/**
 * BLE 通信最佳实践
 * 性能优化和电池使用平衡
 */

class OptimizedBLEService {
    companion object {
        // BLE 优化常量
        private const val SCAN_INTERVAL_MS = 5000L        // 扫描间隔
        private const val ADVERTISEMENT_INTERVAL_MS = 1000L  // 广播间隔
        private const val CONNECTION_TIMEOUT_MS = 10000L     // 连接超时
        private const val MAX_RETRY_COUNT = 3                // 最大重试次数

        // 数据包大小优化
        private const val MAX_MTU_SIZE = 517                // BLE 最大 MTU
        private const val SAFE_PACKET_SIZE = 400            // 安全的数据包大小
    }

    /**
     * 智能扫描策略
     * 根据设备状态调整扫描频率
     */
    fun startSmartScanning(): Flow<DiscoveredDevice> = callbackFlow {
        var scanJob: Job? = null
        var isScanning = false

        suspend fun startScanIfNeeded() {
            if (!isScanning) {
                isScanning = true
                scanJob = launch {
                    bluetoothScanner.startScan(scanCallback)
                    delay(SCAN_INTERVAL_MS)
                    bluetoothScanner.stopScan(scanCallback)
                    isScanning = false
                }
            }
        }

        // 当需要设备时才开始扫描
        startScanIfNeeded()

        awaitClose {
            scanJob?.cancel()
            bluetoothScanner.stopScan(scanCallback)
        }
    }

    /**
     * 数据分片传输
     * 大数据分割为小包传输
     */
    suspend fun sendLargeData(
        device: BluetoothDevice,
        data: ByteArray
    ): Result<Unit> {
        return try {
            val chunks = data.chunked(SAFE_PACKET_SIZE)

            chunks.forEachIndexed { index, chunk ->
                val chunkedMessage = ChunkedMessage(
                    messageId = UUID.randomUUID().toString(),
                    chunkIndex = index,
                    totalChunks = chunks.size,
                    data = chunk,
                    isLastChunk = index == chunks.lastIndex
                )

                val result = sendMessage(device, chunkedMessage)
                if (result.isFailure) {
                    return result
                }

                // 小延迟避免 BLE 缓冲区溢出
                delay(50)
            }

            Result.success(Unit)
        } catch (error: Exception) {
            Result.failure(error)
        }
    }
}
```

#### 2. 错误处理和恢复机制

```kotlin
/**
 * 错误处理最佳实践
 * 统一的错误处理和自动恢复
 */

sealed class NearClipError(
    message: String,
    cause: Throwable? = null,
    val recoverable: Boolean = true
) : Exception(message, cause) {

    class BluetoothError(message: String, cause: Throwable? = null)
        : NearClipError(message, cause, recoverable = true)

    class EncryptionError(message: String, cause: Throwable? = null)
        : NearClipError(message, cause, recoverable = false)

    class PairingError(message: String, cause: Throwable? = null)
        : NearClipError(message, cause, recoverable = true)

    class SyncError(message: String, cause: Throwable? = null)
        : NearClipError(message, cause, recoverable = true)

    class ValidationError(message: String)
        : NearClipError(message, recoverable = false)
}

class ErrorRecoveryManager {
    private val retryPolicy = ExponentialBackoffRetry(
        maxRetries = 3,
        initialDelayMs = 1000,
        maxDelayMs = 5000
    )

    suspend fun <T> executeWithRecovery(
        operation: suspend () -> Result<T>,
        onRetry: (Int, NearClipError) -> Unit = { _, _ -> }
    ): Result<T> {
        return retryPolicy.execute {
            val result = operation()

            result.fold(
                onSuccess = { it },
                onFailure = { error ->
                    if (error is NearClipError) {
                        if (!error.recoverable) {
                            throw error
                        }
                        // 记录错误用于调试
                        logger.error("Recoverable error occurred", error)
                    } else {
                        throw NearClipError.BluetoothError("Unexpected error", error)
                    }
                }
            )
        }
    }

    /**
     * 自动重连机制
     */
    suspend fun autoReconnect(
        device: Device,
        bluetoothService: BluetoothService
    ): Result<Boolean> {
        return executeWithRecovery(
            operation = {
                bluetoothService.connectToDevice(device.deviceId)
            },
            onRetry = { attempt, error ->
                logger.warn("Auto-reconnect attempt $attempt failed for ${device.deviceId}", error)
                // 延迟增加，避免频繁重连
                delay(1000L * attempt)
            }
        )
    }
}
```

#### 3. 状态管理最佳实践

```kotlin
/**
 * 状态管理最佳实践
 * 使用 StateFlow 实现响应式状态管理
 */

class NearClipStateManager {
    private val _uiState = MutableStateFlow(NearClipUiState())
    val uiState: StateFlow<NearClipUiState> = _uiState.asStateFlow()

    private val _singleEvents = MutableSharedFlow<NearClipEvent>()
    val singleEvents: SharedFlow<NearClipEvent> = _singleEvents.asSharedFlow()

    /**
     * 状态更新方法
     */
    private fun updateState(update: NearClipUiState.() -> NearClipUiState) {
        _uiState.update { update(it) }
    }

    /**
     * 发送一次性事件
     */
    private fun sendEvent(event: NearClipEvent) {
        _singleEvents.tryEmit(event)
    }

    /**
     * 设备连接状态更新
     */
    fun onDeviceConnected(device: Device) {
        updateState {
            copy(
                connectedDevices = connectedDevices + device,
                isLoading = false,
                errorMessage = null
            )
        }

        sendEvent(NearClipEvent.DeviceConnected(device))
    }

    /**
     * 同步操作状态更新
     */
    fun onSyncStarted() {
        updateState {
            copy(
                isSyncing = true,
                lastSyncStatus = null
            )
        }
    }

    fun onSyncCompleted(content: String) {
        updateState {
            copy(
                isSyncing = false,
                lastSyncStatus = SyncStatus.Success(content),
                lastSyncTime = System.currentTimeMillis()
            )
        }

        sendEvent(NearClipEvent.SyncCompleted(content))
    }

    fun onSyncFailed(error: String) {
        updateState {
            copy(
                isSyncing = false,
                lastSyncStatus = SyncStatus.Failed(error)
            )
        }

        sendEvent(NearClipEvent.SyncFailed(error))
    }
}

data class NearClipUiState(
    val connectedDevices: List<Device> = emptyList(),
    val discoveredDevices: List<DiscoveredDevice> = emptyList(),
    val isScanning: Boolean = false,
    val isAdvertising: Boolean = false,
    val isSyncing: Boolean = false,
    val lastSyncStatus: SyncStatus? = null,
    val lastSyncTime: Long? = null,
    val isLoading: Boolean = false,
    val errorMessage: String? = null
) {
    val hasConnectedDevices: Boolean
        get() = connectedDevices.isNotEmpty()

    val canSync: Boolean
        get() = hasConnectedDevices && !isSyncing && !isLoading

    val isIdle: Boolean
        get() = !isScanning && !isAdvertising && !isSyncing && !isLoading
}

sealed class NearClipEvent {
    data class DeviceConnected(val device: Device) : NearClipEvent()
    data class DeviceDisconnected(val device: Device) : NearClipEvent()
    data class PairingRequest(val deviceName: String, val code: String) : NearClipEvent()
    data class SyncCompleted(val content: String) : NearClipEvent()
    data class SyncFailed(val error: String) : NearClipEvent()
    data class ShowToast(val message: String) : NearClipEvent()
}

sealed class SyncStatus {
    data class Success(val content: String) : SyncStatus()
    data class Failed(val error: String) : SyncStatus()
}
```

### 常见陷阱和解决方案

#### 1. BLE 连接不稳定

```kotlin
/**
 * 常见陷阱：BLE 连接不稳定
 * 解决方案：连接健康监控和自动重连
 */

class BLEConnectionHealthMonitor {
    private val connectionHealth = MutableStateFlow<Map<String, ConnectionHealth>>(emptyMap())

    data class ConnectionHealth(
        val lastPingTime: Long,
        val consecutiveFailures: Int,
        val averageLatency: Long,
        val isHealthy: Boolean
    )

    /**
     * 定期健康检查
     */
    fun startHealthCheck(devices: List<Device>) = CoroutineScope(Dispatchers.IO).launch {
        while (true) {
            devices.forEach { device ->
                val health = checkDeviceHealth(device)
                updateConnectionHealth(device.deviceId, health)

                // 如果设备不健康，触发重连
                if (!health.isHealthy) {
                    handleUnhealthyDevice(device, health)
                }
            }

            delay(5000) // 每5秒检查一次
        }
    }

    private fun checkDeviceHealth(device: Device): ConnectionHealth {
        return try {
            val startTime = System.currentTimeMillis()
            val pingResult = pingDevice(device)
            val latency = System.currentTimeMillis() - startTime

            val currentHealth = connectionHealth.value[device.deviceId]
            val consecutiveFailures = if (pingResult) 0 else {
                (currentHealth?.consecutiveFailures ?: 0) + 1
            }

            ConnectionHealth(
                lastPingTime = System.currentTimeMillis(),
                consecutiveFailures = consecutiveFailures,
                averageLatency = latency,
                isHealthy = pingResult && consecutiveFailures < 3
            )
        } catch (error: Exception) {
            ConnectionHealth(
                lastPingTime = 0,
                consecutiveFailures = Int.MAX_VALUE,
                averageLatency = Long.MAX_VALUE,
                isHealthy = false
            )
        }
    }

    private suspend fun pingDevice(device: Device): Boolean {
        // 实现 ping 逻辑
        return true
    }

    private fun handleUnhealthyDevice(device: Device, health: ConnectionHealth) {
        if (health.consecutiveFailures >= 3) {
            // 触发重连
            reconnectionManager.reconnectDevice(device)
        }
    }
}
```

#### 2. 内存泄漏防护

```kotlin
/**
 * 常见陷阱：内存泄漏
 * 解决方案：生命周期管理和资源清理
 */

class ResourceManager {
    private val activeConnections = mutableSetOf<BluetoothGatt>()
    private val activeSubscriptions = mutableSetOf<Job>()

    /**
     * 安全的资源管理
     */
    inline fun <T> withManagedResource(
        resource: T,
        block: (T) -> Unit
    ) where T : AutoCloseable {
        try {
            block(resource)
        } finally {
            resource.close()
        }
    }

    /**
     * BLE 连接管理
     */
    fun manageConnection(gatt: BluetoothGatt, onClose: () -> Unit = {}) {
        activeConnections.add(gatt)

        gatt.connect()

        // 监听连接状态
        gatt.onStateChange = { status ->
            if (status == BluetoothGatt.STATE_DISCONNECTED) {
                cleanupConnection(gatt)
                onClose()
            }
        }
    }

    private fun cleanupConnection(gatt: BluetoothGatt) {
        activeConnections.remove(gatt)
        gatt.disconnect()
        gatt.close()
    }

    /**
     * 协程管理
     */
    fun launchManagedCoroutine(
        context: CoroutineContext = EmptyCoroutineContext,
        block: suspend CoroutineScope.() -> Unit
    ): Job {
        val job = CoroutineScope(context).launch {
            try {
                block()
            } catch (error: Exception) {
                logger.error("Managed coroutine failed", error)
            } finally {
                activeSubscriptions.remove(this)
            }
        }

        activeSubscriptions.add(job)
        job.invokeOnCompletion {
            activeSubscriptions.remove(job)
        }

        return job
    }

    /**
     * 清理所有资源
     */
    fun cleanup() {
        activeConnections.forEach { gatt ->
            gatt.disconnect()
            gatt.close()
        }
        activeConnections.clear()

        activeSubscriptions.forEach { it.cancel() }
        activeSubscriptions.clear()
    }
}
```

## 性能监控和调试

### 性能指标收集

```kotlin
/**
 * 性能监控和调试工具
 * 收集关键性能指标用于优化
 */

class PerformanceMonitor {
    private val metrics = mutableMapOf<String, PerformanceMetric>()

    data class PerformanceMetric(
        val name: String,
        val count: Long,
        val totalTime: Long,
        val averageTime: Long,
        val minTime: Long,
        val maxTime: Long,
        val recentSamples: Queue<Long> = ArrayDeque(100) // 最近100个样本
    ) {
        fun addSample(timeMs: Long): PerformanceMetric {
            return copy(
                count = count + 1,
                totalTime = totalTime + timeMs,
                averageTime = (totalTime + timeMs) / (count + 1),
                minTime = minOf(minTime, timeMs),
                maxTime = maxOf(maxTime, timeMs)
            )
        }
    }

    /**
     * 记录操作耗时
     */
    inline fun <T> measureTime(operationName: String, operation: () -> T): T {
        val startTime = System.nanoTime()
        val result = operation()
        val endTime = System.nanoTime()
        val durationMs = (endTime - startTime) / 1_000_000

        recordMetric(operationName, durationMs)
        return result
    }

    private fun recordMetric(name: String, timeMs: Long) {
        val current = metrics[name] ?: PerformanceMetric(
            name = name,
            count = 0,
            totalTime = 0,
            averageTime = 0,
            minTime = Long.MAX_VALUE,
            maxTime = Long.MIN_VALUE
        )

        metrics[name] = current.addSample(timeMs)

        // 记录性能异常
        if (timeMs > current.averageTime * 2) {
            logger.warn("Performance anomaly detected: $name took ${timeMs}ms (avg: ${current.averageTime}ms)")
        }
    }

    /**
     * 获取性能报告
     */
    fun getPerformanceReport(): String {
        return buildString {
            appendLine("=== Performance Report ===")
            appendLine()

            metrics.values.sortedByDescending { it.averageTime }.forEach { metric ->
                appendLine("${metric.name}:")
                appendLine("  Count: ${metric.count}")
                appendLine("  Average: ${metric.averageTime}ms")
                appendLine("  Min: ${metric.minTime}ms")
                appendLine("  Max: ${metric.maxTime}ms")
                appendLine("  Total: ${metric.totalTime}ms")
                appendLine()
            }
        }
    }
}

// 性能监控使用示例
class SyncService {
    private val performanceMonitor = PerformanceMonitor()

    suspend fun syncContent(content: String, targets: List<Device>): Result<Unit> {
        return performanceMonitor.measureTime("sync_operation") {
            targets.forEach { device ->
                performanceMonitor.measureTime("sync_to_${device.deviceId}") {
                    sendToDevice(device, content)
                }
            }
            Result.success(Unit)
        }
    }
}
```

### 调试工具集成

```kotlin
/**
 * 调试工具集成
 * 提供开发阶段的调试功能
 */

class NearClipDebugger {
    private val debugLogs = mutableListOf<DebugLog>()
    private val isDebugEnabled = BuildConfig.DEBUG

    data class DebugLog(
        val timestamp: Long,
        val level: LogLevel,
        val tag: String,
        val message: String,
        val data: Map<String, Any> = emptyMap()
    )

    enum class LogLevel {
        VERBOSE, DEBUG, INFO, WARN, ERROR
    }

    /**
     * 记录调试日志
     */
    fun log(level: LogLevel, tag: String, message: String, data: Map<String, Any> = emptyMap()) {
        if (!isDebugEnabled) return

        val log = DebugLog(
            timestamp = System.currentTimeMillis(),
            level = level,
            tag = tag,
            message = message,
            data = data
        )

        debugLogs.add(log)

        // 输出到控制台
        when (level) {
            LogLevel.ERROR -> Log.e(tag, message, data)
            LogLevel.WARN -> Log.w(tag, message, data)
            LogLevel.INFO -> Log.i(tag, message, data)
            LogLevel.DEBUG -> Log.d(tag, message, data)
            LogLevel.VERBOSE -> Log.v(tag, message, data)
        }
    }

    /**
     * 导出调试日志
     */
    fun exportDebugLogs(): String {
        return debugLogs.joinToString("\n") { log ->
            val timestamp = SimpleDateFormat("yyyy-MM-dd HH:mm:ss.SSS", Locale.getDefault())
                .format(Date(log.timestamp))
            "[$timestamp] [${log.level}] [${log.tag}] ${log.message} ${log.data}"
        }
    }

    /**
     * BLE 消息调试
     */
    fun logBLEMessage(direction: String, message: NearClipMessage, device: String) {
        log(LogLevel.DEBUG, "BLE", "Message $direction", mapOf(
            "device" to device,
            "type" to message.type.name,
            "messageId" to message.messageId,
            "size" to serialize(message).length
        ))
    }

    /**
     * 设备状态调试
     */
    fun logDeviceStateChange(device: Device, oldStatus: ConnectionStatus, newStatus: ConnectionStatus) {
        log(LogLevel.INFO, "DEVICE_STATE", "Device ${device.displayName} status changed", mapOf(
            "device" to device.displayName,
            "oldStatus" to oldStatus.name,
            "newStatus" to newStatus.name,
            "reason" to "State update"
        ))
    }
}

// 扩展函数简化调试调用
fun NearClipDebugger.info(tag: String, message: String, data: Map<String, Any> = emptyMap()) =
    log(LogLevel.INFO, tag, message, data)

fun NearClipDebugger.debug(tag: String, message: String, data: Map<String, Any> = emptyMap()) =
    log(LogLevel.DEBUG, tag, message, data)

fun NearClipDebugger.error(tag: String, message: String, error: Throwable? = null) =
    log(LogLevel.ERROR, tag, message, if (error != null) mapOf("error" to error.message) else emptyMap())
```

## 下一步实施建议

### 开发优先级路线图

#### 阶段 1: 基础架构搭建 (第1-2周)

1. **Monorepo 初始化**
   - 创建项目结构
   - 配置 Gradle (Android) 和 Xcode (Mac)
   - 设置共享协议模块

2. **核心通信协议**
   - 实现消息类型定义
   - 实现序列化/反序列化
   - 创建测试数据生成工具

#### 阶段 2: BLE 通信基础 (第3-4周)

1. **设备发现功能**
   - Android BLE 扫描实现
   - Mac BLE 广播实现
   - 设备信息解析

2. **配对功能**
   - QR 码生成和扫描
   - 公钥加密配对
   - 配对状态管理

#### 阶段 3: 核心同步功能 (第5-6周)

1. **粘贴板监听**
   - Android 粘贴板监听实现
   - Mac 粘贴板监听实现
   - 内容变化检测

2. **数据同步**
   - 消息广播机制
   - 同步状态管理
   - 冲突解决

#### 阶段 4: 完善和优化 (第7-8周)

1. **UI/UX 完善**
   - 设备列表界面
   - 连接状态显示
   - 错误处理界面

2. **测试和优化**
   - 完整测试覆盖
   - 性能优化
   - 电池使用优化

### AI 代理开发指导

对于 AI 代理开发，建议按照以下优先级：

1. **首先实施共享协议层** - 为跨平台通信建立基础
2. **然后实现 BLE 通信核心** - 关注消息收发和错误处理
3. **接着实现设备管理** - 设备发现、配对、状态管理
4. **最后实现 UI 层** - 基于 Jetpack Compose 和 SwiftUI

每个阶段都应该有完整的单元测试，确保代码质量和功能正确性。

## Rust 核心模块完整实现

### 加密模块实现 (crypto.rs)

```rust
// shared/rust/src/crypto.rs
use ring::{aead, rand, signature};
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{AES_256_GCM, LessSafeKey, Nonce, UnboundKey};
use signature::{Ed25519KeyPair, KeyPair as SignatureKeyPair, Signature};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
}

pub type Result<T> = std::result::Result<T, CryptoError>;

/// 加密服务 - 提供端到端加密功能
pub struct CryptoService {
    rng: SystemRandom,
    device_key_pair: Arc<Ed25519KeyPair>,
}

impl CryptoService {
    /// 创建新的加密服务实例
    pub fn new() -> Result<Self> {
        let rng = SystemRandom::new();
        let device_key_pair = Self::generate_device_keypair(&rng)?;

        Ok(CryptoService {
            rng,
            device_key_pair: Arc::new(device_key_pair),
        })
    }

    /// 从现有密钥对创建加密服务
    pub fn from_keypair(key_pair_bytes: &[u8]) -> Result<Self> {
        let device_key_pair = Ed25519KeyPair::from_pkcs8(key_pair_bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(e.to_string()))?;

        Ok(CryptoService {
            rng: SystemRandom::new(),
            device_key_pair: Arc::new(device_key_pair),
        })
    }

    /// 生成设备密钥对
    fn generate_device_keypair(rng: &SystemRandom) -> Result<Ed25519KeyPair> {
        let key_pair_bytes = signature::Ed25519KeyPair::generate_pkcs8(rng)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))?;

        Ed25519KeyPair::from_pkcs8(&key_pair_bytes)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))
    }

    /// 获取设备公钥
    pub fn get_device_public_key(&self) -> &[u8] {
        self.device_key_pair.public_key().as_ref()
    }

    /// 获取设备私钥的 PKCS8 格式
    pub fn get_device_private_key(&self) -> Vec<u8> {
        self.device_key_pair.as_ref().as_ref().to_vec()
    }

    /// 生成临时 AES 密钥用于消息加密
    pub fn generate_session_key(&self) -> Result<Vec<u8>> {
        let mut key = vec![0u8; 32]; // 256 bits for AES-256
        self.rng.fill(&mut key)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))?;
        Ok(key)
    }

    /// 生成随机 Nonce
    pub fn generate_nonce(&self) -> Result<Vec<u8>> {
        let mut nonce = vec![0u8; 12]; // 96 bits for AES-GCM
        self.rng.fill(&mut nonce)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))?;
        Ok(nonce)
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat("Key must be 32 bytes".to_string()));
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidKeyFormat("Nonce must be 12 bytes".to_string()));
        }

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());

        let mut ciphertext = plaintext.to_vec();
        less_safe_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut ciphertext)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        Ok(ciphertext)
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat("Key must be 32 bytes".to_string()));
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidKeyFormat("Nonce must be 12 bytes".to_string()));
        }

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());

        let mut plaintext = ciphertext.to_vec();
        let decrypted_len = less_safe_key.open_in_place(nonce, aead::Aad::empty(), &mut plaintext)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

        plaintext.truncate(decrypted_len);
        Ok(plaintext)
    }

    /// 数字签名
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.device_key_pair.sign(message).as_ref().to_vec()
    }

    /// 验证签名
    pub fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<()> {
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);

        public_key.verify(message, signature)
            .map_err(|_| CryptoError::SignatureVerificationFailed)
    }

    /// 生成安全的配对码
    pub fn generate_pairing_code(&self) -> Result<String> {
        let mut code = [0u8; 4];
        self.rng.fill(&mut code)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))?;

        // 转换为6位数字配对码
        let code_num = u32::from_be_bytes(code) % 1_000_000;
        Ok(format!("{:06}", code_num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let crypto = CryptoService::new().unwrap();
        let key = crypto.generate_session_key().unwrap();
        let nonce = crypto.generate_nonce().unwrap();

        let plaintext = b"Hello, NearClip!";
        let ciphertext = crypto.encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted = crypto.decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_signature_verification() {
        let crypto1 = CryptoService::new().unwrap();
        let crypto2 = CryptoService::new().unwrap();

        let message = b"Test message";
        let signature = crypto1.sign(message);

        // 验证签名应该成功
        assert!(crypto1.verify(message, &signature, crypto1.get_device_public_key()).is_ok());

        // 使用错误的公钥验证应该失败
        assert!(crypto1.verify(message, &signature, crypto2.get_device_public_key()).is_err());
    }

    #[test]
    fn test_pairing_code_generation() {
        let crypto = CryptoService::new().unwrap();
        let code1 = crypto.generate_pairing_code().unwrap();
        let code2 = crypto.generate_pairing_code().unwrap();

        assert_ne!(code1, code2);
        assert_eq!(code1.len(), 6);
        assert_eq!(code2.len(), 6);
    }
}
```

### BLE 通信控制模块 (ble.rs)

```rust
// shared/rust/src/ble.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BLEError {
    #[error("BLE not available")]
    NotAvailable,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Operation timeout")]
    Timeout,
    #[error("Invalid data format: {0}")]
    InvalidDataFormat(String),
}

pub type Result<T> = std::result::Result<T, BLEError>;

/// BLE 设备信息
#[derive(Debug, Clone)]
pub struct BLEDevice {
    pub id: String,
    pub name: String,
    pub address: String,
    pub rssi: i32,
    pub service_data: HashMap<String, Vec<u8>>,
    pub last_seen: Instant,
}

/// BLE 连接状态
#[derive(Debug, Clone, PartialEq)]
pub enum BLEConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Bonded,
}

/// BLE 消息类型
#[derive(Debug, Clone, Copy)]
pub enum BLEMessageType {
    Discovery,
    PairingRequest,
    PairingResponse,
    SyncData,
    Acknowledgment,
}

/// BLE 管理器 - 控制设备发现和连接
pub struct BLEManager {
    devices: Arc<Mutex<HashMap<String, BLEDevice>>>,
    connection_state: Arc<Mutex<HashMap<String, BLEConnectionState>>>,
    is_scanning: Arc<Mutex<bool>>,
    service_uuid: String,
    max_packet_size: usize,
}

impl BLEManager {
    /// 创建新的 BLE 管理器
    pub fn new(service_uuid: String) -> Self {
        BLEManager {
            devices: Arc::new(Mutex::new(HashMap::new())),
            connection_state: Arc::new(Mutex::new(HashMap::new())),
            is_scanning: Arc::new(Mutex::new(false)),
            service_uuid,
            max_packet_size: 20, // BLE 默认 MTU - 3 bytes header
        }
    }

    /// 开始设备扫描
    pub async fn start_scan(&self, timeout_seconds: u64) -> Result<Vec<BLEDevice>> {
        let mut is_scanning = self.is_scanning.lock().unwrap();
        if *is_scanning {
            return Err(BLEError::ConnectionFailed("Already scanning".to_string()));
        }
        *is_scanning = true;
        drop(is_scanning);

        // 清除旧的设备列表
        let mut devices = self.devices.lock().unwrap();
        devices.clear();
        drop(devices);

        // 模拟设备扫描过程
        let scan_duration = Duration::from_secs(timeout_seconds);
        let start_time = Instant::now();

        while start_time.elapsed() < scan_duration {
            // 在实际实现中，这里会调用系统BLE API
            if let Some(device) = self.simulate_device_discovery().await {
                let mut devices = self.devices.lock().unwrap();
                devices.insert(device.id.clone(), device);
            }

            sleep(Duration::from_millis(100)).await;
        }

        let is_scanning = self.is_scanning.lock().unwrap();
        *is_scanning = false;

        let devices = self.devices.lock().unwrap();
        Ok(devices.values().cloned().collect())
    }

    /// 停止设备扫描
    pub fn stop_scan(&self) {
        let mut is_scanning = self.is_scanning.lock().unwrap();
        *is_scanning = false;
    }

    /// 获取发现的设备列表
    pub fn get_discovered_devices(&self) -> Vec<BLEDevice> {
        let devices = self.devices.lock().unwrap();
        devices.values().cloned().collect()
    }

    /// 连接到设备
    pub async fn connect_to_device(&self, device_id: &str) -> Result<()> {
        let devices = self.devices.lock().unwrap();
        if !devices.contains_key(device_id) {
            return Err(BLEError::DeviceNotFound(device_id.to_string()));
        }
        drop(devices);

        let mut connection_state = self.connection_state.lock().unwrap();
        connection_state.insert(device_id.to_string(), BLEConnectionState::Connecting);
        drop(connection_state);

        // 模拟连接过程
        sleep(Duration::from_millis(500)).await;

        let mut connection_state = self.connection_state.lock().unwrap();
        connection_state.insert(device_id.to_string(), BLEConnectionState::Connected);

        Ok(())
    }

    /// 断开设备连接
    pub async fn disconnect_from_device(&self, device_id: &str) -> Result<()> {
        let mut connection_state = self.connection_state.lock().unwrap();
        if connection_state.get(device_id) != Some(&BLEConnectionState::Connected) {
            return Err(BLEError::ConnectionFailed("Device not connected".to_string()));
        }

        connection_state.insert(device_id.to_string(), BLEConnectionState::Disconnected);
        drop(connection_state);

        // 模拟断开连接过程
        sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    /// 获取设备连接状态
    pub fn get_connection_state(&self, device_id: &str) -> BLEConnectionState {
        let connection_state = self.connection_state.lock().unwrap();
        connection_state.get(device_id).cloned().unwrap_or(BLEConnectionState::Disconnected)
    }

    /// 发送消息到设备
    pub async fn send_message(&self, device_id: &str, message: &[u8]) -> Result<()> {
        if self.get_connection_state(device_id) != BLEConnectionState::Connected {
            return Err(BLEError::ConnectionFailed("Device not connected".to_string()));
        }

        // 分片发送大消息
        let chunks = self.chunk_message(message)?;

        for chunk in chunks {
            self.send_chunk(device_id, &chunk).await?;
            sleep(Duration::from_millis(10)).await; // 避免BLE缓冲区溢出
        }

        Ok(())
    }

    /// 分片消息以适应BLE包大小限制
    fn chunk_message(&self, message: &[u8]) -> Result<Vec<Vec<u8>>> {
        if message.len() <= self.max_packet_size {
            return Ok(vec![message.to_vec()]);
        }

        let mut chunks = Vec::new();
        let mut offset = 0;

        while offset < message.len() {
            let end = (offset + self.max_packet_size).min(message.len());
            chunks.push(message[offset..end].to_vec());
            offset = end;
        }

        Ok(chunks)
    }

    /// 发送单个数据块
    async fn send_chunk(&self, device_id: &str, chunk: &[u8]) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE写入API
        println!("Sending {} bytes to device: {}", chunk.len(), device_id);

        // 模拟发送延迟
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    /// 开始广播设备信息
    pub async fn start_advertising(&self, device_info: &[u8]) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE广播API
        println!("Starting advertising with device info: {} bytes", device_info.len());

        // 模拟广播启动
        sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    /// 停止广播
    pub async fn stop_advertising(&self) -> Result<()> {
        // 在实际实现中，这里会调用系统BLE停止广播API
        println!("Stopping advertising");

        Ok(())
    }

    /// 模拟设备发现（仅用于演示）
    async fn simulate_device_discovery(&self) -> Option<BLEDevice> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        // 10% 的概率发现一个设备
        if rng.gen_range(0..10) == 0 {
            Some(BLEDevice {
                id: format!("device-{}", rng.gen_range(1000..9999)),
                name: format!("NearClip-Device-{}", rng.gen_range(100..999)),
                address: format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256),
                    rng.gen_range(0..256)
                ),
                rssi: rng.gen_range(-90..-30),
                service_data: HashMap::new(),
                last_seen: Instant::now(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_scan() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 启动扫描
        let devices = ble_manager.start_scan(2).await.unwrap();

        // 验证扫描结果
        assert!(!devices.is_empty());

        for device in devices {
            assert!(!device.id.is_empty());
            assert!(!device.name.is_empty());
            assert!(!device.address.is_empty());
        }
    }

    #[tokio::test]
    async fn test_device_connection() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 先扫描设备
        let devices = ble_manager.start_scan(1).await.unwrap();
        if let Some(device) = devices.first() {
            // 连接设备
            assert!(ble_manager.connect_to_device(&device.id).await.is_ok());

            // 检查连接状态
            assert_eq!(ble_manager.get_connection_state(&device.id), BLEConnectionState::Connected);

            // 断开连接
            assert!(ble_manager.disconnect_from_device(&device.id).await.is_ok());

            // 检查断开状态
            assert_eq!(ble_manager.get_connection_state(&device.id), BLEConnectionState::Disconnected);
        }
    }

    #[test]
    fn test_message_chunking() {
        let ble_manager = BLEManager::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string());

        // 小消息不应分片
        let small_message = vec![1, 2, 3];
        let chunks = ble_manager.chunk_message(&small_message).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], small_message);

        // 大消息应该分片
        let large_message = vec![1; 50]; // 超过默认的20字节限制
        let chunks = ble_manager.chunk_message(&large_message).unwrap();
        assert!(chunks.len() > 1);

        // 验证重组后的消息
        let mut reassembled = Vec::new();
        for chunk in chunks {
            reassembled.extend_from_slice(&chunk);
        }
        assert_eq!(reassembled, large_message);
    }
}
```

### C FFI 接口 (ffi.rs)

```rust
// shared/rust/src/ffi.rs
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint8_t};
use std::ptr;
use std::slice;
use crate::crypto::{CryptoService, CryptoError};
use crate::ble::{BLEManager, BLEError};

// 导出 C API

/// 创建加密服务实例
#[no_mangle]
pub extern "C" fn nearclip_crypto_create() -> *mut CryptoService {
    let crypto = match CryptoService::new() {
        Ok(crypto) => crypto,
        Err(_) => return ptr::null_mut(),
    };
    Box::into_raw(Box::new(crypto))
}

/// 销毁加密服务实例
#[no_mangle]
pub extern "C" fn nearclip_crypto_destroy(crypto: *mut CryptoService) {
    if !crypto.is_null() {
        unsafe {
            let _ = Box::from_raw(crypto);
        }
    }
}

/// 生成会话密钥
#[no_mangle]
pub extern "C" fn nearclip_crypto_generate_session_key(
    crypto: *mut CryptoService,
    key_buffer: *mut c_uint8_t,
    key_size: c_int
) -> c_int {
    if crypto.is_null() || key_buffer.is_null() || key_size != 32 {
        return -1;
    }

    let crypto = unsafe { &mut *crypto };
    match crypto.generate_session_key() {
        Ok(key) => {
            unsafe {
                ptr::copy_nonoverlapping(key.as_ptr(), key_buffer, 32);
            }
            0
        }
        Err(_) => -1,
    }
}

/// 加密数据
#[no_mangle]
pub extern "C" fn nearclip_crypto_encrypt(
    crypto: *mut CryptoService,
    plaintext: *const c_uint8_t,
    plaintext_len: c_int,
    key: *const c_uint8_t,
    nonce: *const c_uint8_t,
    ciphertext: *mut c_uint8_t,
    ciphertext_len: *mut c_int
) -> c_int {
    if crypto.is_null() || plaintext.is_null() || key.is_null() || nonce.is_null() || ciphertext.is_null() {
        return -1;
    }

    let crypto = unsafe { &mut *crypto };
    let plaintext_slice = unsafe { slice::from_raw_parts(plaintext, plaintext_len as usize) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let nonce_slice = unsafe { slice::from_raw_parts(nonce, 12) };

    match crypto.encrypt(plaintext_slice, key_slice, nonce_slice) {
        Ok(result) => {
            if result.len() <= ciphertext_len as usize {
                unsafe {
                    ptr::copy_nonoverlapping(result.as_ptr(), ciphertext, result.len());
                    *ciphertext_len = result.len() as c_int;
                }
                0
            } else {
                -2 // 缓冲区太小
            }
        }
        Err(_) => -1,
    }
}

/// 创建 BLE 管理器实例
#[no_mangle]
pub extern "C" fn nearclip_ble_create(service_uuid: *const c_char) -> *mut BLEManager {
    if service_uuid.is_null() {
        return ptr::null_mut();
    }

    let uuid_str = unsafe { CStr::from_ptr(service_uuid) }.to_str().unwrap_or("");
    let ble_manager = BLEManager::new(uuid_str.to_string());
    Box::into_raw(Box::new(ble_manager))
}

/// 销毁 BLE 管理器实例
#[no_mangle]
pub extern "C" fn nearclip_ble_destroy(ble: *mut BLEManager) {
    if !ble.is_null() {
        unsafe {
            let _ = Box::from_raw(ble);
        }
    }
}

/// 开始设备扫描
#[no_mangle]
pub extern "C" fn nearclip_ble_start_scan(
    ble: *mut BLEManager,
    timeout_seconds: c_int,
    callback: extern "C" fn(*const c_char, *const c_char, c_int)
) -> c_int {
    if ble.is_null() {
        return -1;
    }

    let ble = unsafe { &mut *ble };

    // 在实际实现中，这里需要使用异步运行时
    // 为了演示，这里只是同步调用回调
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        match ble.start_scan(timeout_seconds as u64).await {
            Ok(devices) => {
                for device in devices {
                    let id_cstring = CString::new(device.id).unwrap();
                    let name_cstring = CString::new(device.name).unwrap();
                    callback(id_cstring.as_ptr(), name_cstring.as_ptr(), device.rssi);
                }
                0
            }
            Err(_) => -1,
        }
    })
}

/// 连接到设备
#[no_mangle]
pub extern "C" fn nearclip_ble_connect(
    ble: *mut BLEManager,
    device_id: *const c_char
) -> c_int {
    if ble.is_null() || device_id.is_null() {
        return -1;
    }

    let ble = unsafe { &mut *ble };
    let device_id_str = unsafe { CStr::from_ptr(device_id) }.to_str().unwrap_or("");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        match ble.connect_to_device(device_id_str).await {
            Ok(_) => 0,
            Err(_) => -1,
        }
    })
}

/// 发送消息
#[no_mangle]
pub extern "C" fn nearclip_ble_send_message(
    ble: *mut BLEManager,
    device_id: *const c_char,
    message: *const c_uint8_t,
    message_len: c_int
) -> c_int {
    if ble.is_null() || device_id.is_null() || message.is_null() {
        return -1;
    }

    let ble = unsafe { &mut *ble };
    let device_id_str = unsafe { CStr::from_ptr(device_id) }.to_str().unwrap_or("");
    let message_slice = unsafe { slice::from_raw_parts(message, message_len as usize) };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        match ble.send_message(device_id_str, message_slice).await {
            Ok(_) => 0,
            Err(_) => -1,
        }
    })
}

/// 错误码定义
pub const NEARCLIP_ERROR_SUCCESS: c_int = 0;
pub const NEARCLIP_ERROR_INVALID_PARAM: c_int = -1;
pub const NEARCLIP_ERROR_BUFFER_TOO_SMALL: c_int = -2;
pub const NEARCLIP_ERROR_CRYPTO_FAILED: c_int = -3;
pub const NEARCLIP_ERROR_BLE_FAILED: c_int = -4;
pub const NEARCLIP_ERROR_TIMEOUT: c_int = -5;

/// 获取错误描述
#[no_mangle]
pub extern "C" fn nearclip_get_error_message(error_code: c_int) -> *const c_char {
    let message = match error_code {
        NEARCLIP_ERROR_SUCCESS => "Success",
        NEARCLIP_ERROR_INVALID_PARAM => "Invalid parameter",
        NEARCLIP_ERROR_BUFFER_TOO_SMALL => "Buffer too small",
        NEARCLIP_ERROR_CRYPTO_FAILED => "Cryptographic operation failed",
        NEARCLIP_ERROR_BLE_FAILED => "BLE operation failed",
        NEARCLIP_ERROR_TIMEOUT => "Operation timeout",
        _ => "Unknown error",
    };

    CString::new(message).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_ffi() {
        let crypto = nearclip_crypto_create();
        assert!(!crypto.is_null());

        let mut key = [0u8; 32];
        let result = nearclip_crypto_generate_session_key(crypto, key.as_mut_ptr(), 32);
        assert_eq!(result, 0);

        // 验证密钥不为空
        assert!(key.iter().any(|&b| b != 0));

        nearclip_crypto_destroy(crypto);
    }

    #[test]
    fn test_ble_ffi() {
        let service_uuid = CString::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e").unwrap();
        let ble = nearclip_ble_create(service_uuid.as_ptr());
        assert!(!ble.is_null());

        nearclip_ble_destroy(ble);
    }
}
```

## 构建脚本和配置

### Android 构建脚本 (build-android.sh)

```bash
#!/bin/bash
# scripts/build-android.sh

set -e

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHARED_RUST_DIR="$PROJECT_ROOT/shared/rust"
ANDROID_LIBS_DIR="$PROJECT_ROOT/android/app/src/main/jniLibs"

echo "🔨 Building NearClip Android libraries..."

# 清理旧的构建文件
rm -rf "$ANDROID_LIBS_DIR"
mkdir -p "$ANDROID_LIBS_DIR"

# 进入 Rust 目录
cd "$SHARED_RUST_DIR"

# Android 目标架构
TARGETS=(
    "aarch64-linux-android"     # ARM64
    "armv7-linux-androideabi"   # ARM32
    "x86_64-linux-android"      # x64
    "i686-linux-android"        # x86
)

# 构建 Android 库
for target in "${TARGETS[@]}"; do
    echo "📱 Building for $target..."

    # 安装目标（如果尚未安装）
    if ! rustup target list --installed | grep -q "$target"; then
        rustup target add "$target"
    fi

    # 构建
    cargo build --release --target="$target"

    # 复制库文件到 Android 项目
    case "$target" in
        "aarch64-linux-android")
            arch_dir="arm64-v8a"
            ;;
        "armv7-linux-androideabi")
            arch_dir="armeabi-v7a"
            ;;
        "x86_64-linux-android")
            arch_dir="x86_64"
            ;;
        "i686-linux-android")
            arch_dir="x86"
            ;;
    esac

    mkdir -p "$ANDROID_LIBS_DIR/$arch_dir"
    cp "target/$target/release/libnearclip.so" "$ANDROID_LIBS_DIR/$arch_dir/"

    echo "✅ Built for $arch_dir"
done

# 构建 Protobuf Java 文件
echo "📝 Building Protobuf Java files..."
cd "$PROJECT_ROOT/shared/protobuf"

# 确保 protoc 和 protoc-gen-grpc-java 已安装
if ! command -v protoc &> /dev/null; then
    echo "❌ protoc not found. Please install Protocol Buffers compiler."
    exit 1
fi

# 生成 Java 文件
protoc --java_out="$PROJECT_ROOT/android/app/src/main/java" \
       --grpc-java_out="$PROJECT_ROOT/android/app/src/main/java" \
       nearclip.proto device.proto sync.proto

echo "🎉 Android build completed successfully!"
```

### macOS 构建脚本 (build-macos.sh)

```bash
#!/bin/bash
# scripts/build-macos.sh

set -e

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHARED_RUST_DIR="$PROJECT_ROOT/shared/rust"
MAC_FRAMEWORKS_DIR="$PROJECT_ROOT/mac/NearClip/Frameworks"

echo "🔨 Building NearClip macOS libraries..."

# 清理旧的构建文件
rm -rf "$MAC_FRAMEWORKS_DIR"
mkdir -p "$MAC_FRAMEWORKS_DIR"

# 进入 Rust 目录
cd "$SHARED_RUST_DIR"

# macOS 目标架构
TARGETS=(
    "x86_64-apple-darwin"     # Intel Mac
    "aarch64-apple-darwin"    # Apple Silicon Mac
)

# 构建 macOS 库
for target in "${TARGETS[@]}"; do
    echo "💻 Building for $target..."

    # 安装目标（如果尚未安装）
    if ! rustup target list --installed | grep -q "$target"; then
        rustup target add "$target"
    fi

    # 构建
    cargo build --release --target="$target"

    # 复制库文件
    cp "target/$target/release/libnearclip.dylib" "$MAC_FRAMEWORKS_DIR/"

    echo "✅ Built for $target"
done

# 如果是 Apple Silicon Mac，创建通用二进制文件
if [[ "$(uname -m)" == "arm64" ]] && [[ -f "$MAC_FRAMEWORKS_DIR/libnearclip.dylib" ]]; then
    echo "🔗 Creating universal binary..."

    # 检查是否两个架构的库都存在
    if [[ -f "$MAC_FRAMEWORKS_DIR/libnearclip.dylib" ]] && \
       [[ -f "$MAC_FRAMEWORKS_DIR/libnearclip.dylib" ]]; then
        # 创建通用二进制文件
        lipo -create \
             "$MAC_FRAMEWORKS_DIR/libnearclip.dylib" \
             "$MAC_FRAMEWORKS_DIR/libnearclip.dylib" \
             -output "$MAC_FRAMEWORKS_DIR/libnearclip_universal.dylib"

        mv "$MAC_FRAMEWORKS_DIR/libnearclip_universal.dylib" "$MAC_FRAMEWORKS_DIR/libnearclip.dylib"
        echo "✅ Universal binary created"
    fi
fi

# 构建 Swift Protobuf 文件
echo "📝 Building Swift Protobuf files..."
cd "$PROJECT_ROOT/shared/protobuf"

# 生成 Swift 文件
protoc --swift_out="$PROJECT_ROOT/mac/NearClip/Generated" \
       nearclip.proto device.proto sync.proto

echo "🎉 macOS build completed successfully!"
```

### Cargo.toml 完整配置

```toml
# shared/rust/Cargo.toml
[package]
name = "nearclip-core"
version = "0.1.0"
edition = "2021"
authors = ["NearClip Team"]
description = "NearClip core functionality for cross-platform clipboard synchronization"
license = "MIT"

[lib]
name = "nearclip"
crate-type = ["cdylib", "rlib"] # 同时支持动态库和静态库

[dependencies]
# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 序列化
prost = "0.12"
prost-types = "0.12"

# 加密
ring = "0.17"
base64 = "0.21"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 日志
log = "0.4"
env_logger = "0.10"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 随机数
rand = "0.8"

# 并发
crossbeam = "0.8"

# FFI
libc = "0.2"

[build-dependencies]
prost-build = "0.12"

[features]
default = ["std"]
std = []

# 目标特定配置
[target.'cfg(target_os = "android")'.dependencies]
android_logger = "0.13"

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"
core-bluetooth = "0.2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 0
debug = true
```

这些完整的实现为 NearClip 项目提供了：

1. **完整的 Rust 核心功能**：包括加密服务、BLE 管理、FFI 接口
2. **跨平台构建脚本**：支持 Android 和 macOS 的自动化构建
3. **详细的错误处理**：使用 Rust 的 Result 类型进行错误管理
4. **异步支持**：使用 Tokio 进行异步操作
5. **内存安全**：利用 Rust 的内存安全保证
6. **性能优化**：编译时优化和 LTO 链接优化

这个架构确保了高性能、安全性和跨平台兼容性，为 AI 代理提供了坚实的基础来实现完整的应用功能。

---

*Generated on 2025-01-15 by Winston (BMAD Architect)*
*Development Guide Version: 1.0*