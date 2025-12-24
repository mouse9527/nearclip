# nearclip-transport

统一传输层抽象，为 NearClip 提供 WiFi 和 BLE 双通道传输能力。

## 概述

`nearclip-transport` 提供了统一的 `Transport` trait 抽象，使上层代码无需关心具体的传输方式（WiFi 或 BLE）。它还包含 `TransportManager` 用于管理多设备、多通道连接，以及自动通道选择和故障转移。

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│ 应用层 (NearClipManager)                                     │
│   - sync_clipboard(content)                                  │
│   - connect_device(device_id)                                │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│ TransportManager                                             │
│   - 管理所有设备连接                                          │
│   - 自动选择最佳通道 (WiFi 优先)                              │
│   - 处理通道切换和故障转移                                     │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│ Transport Trait (统一传输接口)                               │
│   - send(&self, msg) -> Result<()>                          │
│   - recv(&self) -> Result<Message>                          │
│   - is_connected(&self) -> bool                             │
│   - channel(&self) -> Channel                               │
└─────────────┬─────────────────────┬─────────────────────────┘
              │                     │
┌─────────────▼─────────┐   ┌───────▼──────────┐   ┌──────────────────┐
│ WifiTransport         │   │ BleTransport     │   │ MockTransport    │
│ - TCP/TLS 连接         │   │ - FFI 回调桥接    │   │ - 内存队列       │
│ - 帧协议编解码          │   │ - 数据分片/重组   │   │ - 可配置延迟/错误 │
└───────────────────────┘   └──────────────────┘   └──────────────────┘
```

## 模块

### `traits.rs` - 核心 Trait 定义

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, msg: &Message) -> Result<(), TransportError>;
    async fn recv(&self) -> Result<Message, TransportError>;
    fn is_connected(&self) -> bool;
    fn channel(&self) -> Channel;
    fn peer_device_id(&self) -> &str;
    async fn close(&self) -> Result<(), TransportError>;
}
```

### `wifi.rs` - WiFi/TCP 传输

基于 TCP 的传输实现，支持：
- 长度前缀帧协议（4 字节大端长度 + 数据）
- 最大消息大小限制（16MB）
- 自动连接状态管理

```rust
let transport = WifiTransport::new(device_id, read_half, write_half);
transport.send(&message).await?;
let received = transport.recv().await?;
```

### `ble.rs` - BLE 传输

通过 FFI 回调桥接平台原生 BLE 实现：
- 自动数据分片（适应 BLE MTU 限制）
- 消息重组（支持多块数据）
- 平台回调接口

```rust
// 平台需要实现 BleSender trait
pub trait BleSender: Send + Sync {
    fn send_ble_data(&self, device_id: &str, data: &[u8]) -> Result<(), String>;
    fn is_ble_connected(&self, device_id: &str) -> bool;
    fn get_mtu(&self, device_id: &str) -> usize;
}

let transport = BleTransport::new(device_id, sender);

// 平台收到 BLE 数据时调用
transport.on_data_received(&chunk).await;
```

### `mock.rs` - Mock 传输（测试用）

用于单元测试和集成测试的 Mock 实现：

```rust
// 创建互联的 Mock 传输对
let (transport_a, transport_b) = create_mock_pair("device_a", "device_b");

// 可配置延迟、丢包、错误
let config = MockConfig::new()
    .with_latency(Duration::from_millis(100))
    .with_drop_rate(0.1)
    .with_error(TransportError::ConnectionClosed);

let transport = MockTransport::new("device_1", config);

// 注入测试消息
transport.inject_message(test_message);

// 检查发送的消息
let sent = transport.get_sent_messages().await;
```

### `manager.rs` - 传输管理器

管理多设备、多通道连接：

```rust
let manager = TransportManager::new();

// 添加传输
manager.add_transport("device_1", wifi_transport).await;
manager.add_transport("device_1", ble_transport).await;

// 自动选择最佳通道发送
manager.send_to_device("device_1", &message).await?;

// 广播到所有设备
let results = manager.broadcast(&message).await;

// 查询连接状态
let devices = manager.connected_devices().await;
let channels = manager.device_channels("device_1").await;
```

### `error.rs` - 错误类型

```rust
pub enum TransportError {
    ConnectionClosed,
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    Timeout,
    NotConnected(String),
    NoAvailableChannel(String),
    Serialization(String),
    Ble(String),
    Io(String),
}
```

## 通道选择策略

默认使用 `PriorityChannelSelector`：
1. **WiFi 优先** - 带宽高、延迟低
2. **BLE 备选** - WiFi 不可用时自动切换

```rust
// 自定义选择器
let selector = Box::new(MyCustomSelector);
let manager = TransportManager::with_selector(selector);
```

## 故障转移

当主通道发送失败时，自动尝试备用通道：

```rust
let config = TransportManagerConfig {
    auto_select_channel: true,
    failover_on_error: true,  // 启用故障转移
};
let manager = TransportManager::with_config(config);
```

## 测试

```bash
cargo test -p nearclip-transport
```

## 依赖

- `tokio` - 异步运行时
- `async-trait` - 异步 trait 支持
- `nearclip-sync` - 消息协议定义
- `nearclip-ble` - BLE 分片/重组
- `tracing` - 结构化日志

## 平台集成

### macOS (Swift)

```swift
class BleManagerBridge: FfiBleSender {
    func sendBleData(deviceId: String, data: Data) throws {
        // 调用 CoreBluetooth 发送
    }

    func isBleConnected(deviceId: String) -> Bool {
        // 检查连接状态
    }

    func getMtu(deviceId: String) -> UInt64 {
        // 返回协商的 MTU
    }
}
```

### Android (Kotlin)

```kotlin
class BleManagerBridge : FfiBleSender {
    override fun sendBleData(deviceId: String, data: ByteArray) {
        // 调用 Android BLE API 发送
    }

    override fun isBleConnected(deviceId: String): Boolean {
        // 检查连接状态
    }

    override fun getMtu(deviceId: String): ULong {
        // 返回协商的 MTU
    }
}
```
