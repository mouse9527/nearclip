# Story 1.4: 定义消息协议结构

Status: Done

## Story

As a 开发者,
I want 定义统一的 Message 消息结构,
So that 所有网络通信使用一致的格式.

## Acceptance Criteria

1. **Given** nearclip-sync crate 已创建 **When** 定义 Message 和 MessageType 类型 **Then** Message 包含 msg_type、payload、timestamp、device_id 字段
2. **And** MessageType 包含 ClipboardSync、PairingRequest、PairingResponse、Heartbeat、Ack
3. **And** 使用 rmp-serde 进行 MessagePack 序列化
4. **And** 单元测试验证序列化/反序列化正确

## Tasks / Subtasks

- [x] Task 1: 创建 protocol.rs 模块 (AC: 1, 2) ✅
  - [x] 1.1 在 `crates/nearclip-sync/src/` 创建 `protocol.rs`
  - [x] 1.2 定义 `MessageType` 枚举（ClipboardSync, PairingRequest, PairingResponse, Heartbeat, Ack）
  - [x] 1.3 定义 `Message` 结构体（msg_type, payload, timestamp, device_id）
  - [x] 1.4 添加 Serialize/Deserialize derive

- [x] Task 2: 实现序列化/反序列化函数 (AC: 3) ✅
  - [x] 2.1 实现 `Message::serialize()` 使用 rmp-serde
  - [x] 2.2 实现 `Message::deserialize()` 从字节反序列化
  - [x] 2.3 实现 `Message::new()` 构造函数

- [x] Task 3: 添加辅助方法 (AC: 1) ✅
  - [x] 3.1 实现 `Message::timestamp_now()` 生成当前时间戳
  - [x] 3.2 实现 `Message::clipboard_sync()` 便捷构造器
  - [x] 3.3 实现 `Message::heartbeat()` 便捷构造器
  - [x] 3.4 实现 `Message::ack()` 便捷构造器
  - [x] 3.5 (额外) 实现 `Message::pairing_request()` 便捷构造器
  - [x] 3.6 (额外) 实现 `Message::pairing_response()` 便捷构造器
  - [x] 3.7 (额外) 实现 `Message::ack_with_payload()` 带载荷确认
  - [x] 3.8 (额外) 实现 `Message::is_expired()` 过期检查
  - [x] 3.9 (额外) 实现 `Message::age_ms()` 消息年龄
  - [x] 3.10 (额外) 实现 `MessageType::as_str()` 字符串表示
  - [x] 3.11 (额外) 实现 `MessageType::requires_ack()` ACK 需求检查

- [x] Task 4: 导出模块 (AC: 1, 2, 3) ✅
  - [x] 4.1 在 `lib.rs` 中添加 `pub mod protocol;`
  - [x] 4.2 重新导出 `Message` 和 `MessageType`

- [x] Task 5: 编写单元测试 (AC: 4) ✅
  - [x] 5.1 测试 Message 序列化/反序列化往返
  - [x] 5.2 测试各 MessageType 变体
  - [x] 5.3 测试时间戳生成正确性
  - [x] 5.4 测试空 payload 处理
  - [x] 5.5 测试反序列化错误处理
  - [x] 5.6 (额外) 测试大 payload
  - [x] 5.7 (额外) 测试二进制 payload
  - [x] 5.8 (额外) 测试 Unicode device_id
  - [x] 5.9 (额外) 测试消息过期检查

- [x] Task 6: 验证构建 (AC: 1, 2, 3, 4) ✅
  - [x] 6.1 运行 `cargo build` 确保无错误
  - [x] 6.2 运行 `cargo test -p nearclip-sync` 确保测试通过

## Dev Notes

### 架构约束 (来自 architecture.md)

**消息协议结构（强制遵循）：**
```rust
// 所有网络消息必须遵循此结构
#[derive(Serialize, Deserialize)]
struct Message {
    msg_type: MessageType,    // 消息类型枚举
    payload: Vec<u8>,         // MessagePack 序列化的 payload
    timestamp: u64,           // Unix 毫秒时间戳
    device_id: String,        // 发送方设备 ID
}

enum MessageType {
    ClipboardSync,    // 剪贴板同步
    PairingRequest,   // 配对请求
    PairingResponse,  // 配对响应
    Heartbeat,        // 心跳
    Ack,              // 确认
}
```

**协议规范（来自 project_context.md）：**
- ✅ 所有网络消息必须遵循 `Message` 结构
- ✅ payload 使用 MessagePack 序列化
- ✅ timestamp 使用 Unix 毫秒时间戳
- ❌ 禁止自定义消息格式
- ❌ 禁止使用 JSON 序列化（性能考虑）

### 文件位置

**目标文件：**
- `crates/nearclip-sync/src/protocol.rs` - 消息协议定义
- `crates/nearclip-sync/src/lib.rs` - 模块导出

### 技术栈版本

| 依赖 | 版本 | 用途 |
|------|------|------|
| serde | 1.x (workspace) | 序列化框架 |
| rmp-serde | 1.x (workspace) | MessagePack 序列化 |

**注意：** serde 和 rmp-serde 已在 Story 1-1 中配置为 workspace 依赖，无需再添加版本号。

### 从前一个 Story 继承的上下文

**Story 1-1 完成的基础设施：**
- ✅ nearclip-sync crate 已创建
- ✅ workspace 配置了 serde = { version = "1", features = ["derive"] }
- ✅ workspace 配置了 rmp-serde = "1"
- ✅ Cargo.toml 使用 workspace 依赖继承

**Story 1-2 完成的错误处理：**
- ✅ NearClipError 统一错误类型已定义
- ✅ 可以添加 Serialization 错误变体

**Story 1-3 完成的日志：**
- ✅ tracing 日志框架已配置
- ✅ 可在协议模块中使用结构化日志

**crate 间依赖关系：**
```
nearclip-ffi → nearclip-core → nearclip-sync → nearclip-net/ble → nearclip-crypto
```
消息协议在 nearclip-sync 中定义，被 net 和 ble 通道使用。

### 代码模板

**protocol.rs 完整模板：**
```rust
//! NearClip 消息协议
//!
//! 定义所有网络通信使用的统一消息格式。

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 消息类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 剪贴板同步内容
    ClipboardSync,
    /// 配对请求
    PairingRequest,
    /// 配对响应
    PairingResponse,
    /// 心跳保活
    Heartbeat,
    /// 确认收到
    Ack,
}

/// 统一消息结构
///
/// 所有网络通信必须使用此结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// 消息类型
    pub msg_type: MessageType,
    /// MessagePack 序列化的载荷数据
    pub payload: Vec<u8>,
    /// Unix 毫秒时间戳
    pub timestamp: u64,
    /// 发送方设备 ID
    pub device_id: String,
}

impl Message {
    /// 创建新消息
    pub fn new(msg_type: MessageType, payload: Vec<u8>, device_id: String) -> Self {
        Self {
            msg_type,
            payload,
            timestamp: Self::timestamp_now(),
            device_id,
        }
    }

    /// 获取当前 Unix 毫秒时间戳
    pub fn timestamp_now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    /// 创建剪贴板同步消息
    pub fn clipboard_sync(content: &[u8], device_id: String) -> Self {
        Self::new(MessageType::ClipboardSync, content.to_vec(), device_id)
    }

    /// 创建心跳消息
    pub fn heartbeat(device_id: String) -> Self {
        Self::new(MessageType::Heartbeat, Vec::new(), device_id)
    }

    /// 创建确认消息
    pub fn ack(device_id: String) -> Self {
        Self::new(MessageType::Ack, Vec::new(), device_id)
    }

    /// 序列化为 MessagePack 字节
    pub fn serialize(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(self)
    }

    /// 从 MessagePack 字节反序列化
    pub fn deserialize(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_serialize() {
        let msg_type = MessageType::ClipboardSync;
        let serialized = rmp_serde::to_vec(&msg_type).unwrap();
        let deserialized: MessageType = rmp_serde::from_slice(&serialized).unwrap();
        assert_eq!(msg_type, deserialized);
    }

    #[test]
    fn test_message_roundtrip() {
        let original = Message::new(
            MessageType::ClipboardSync,
            b"hello world".to_vec(),
            "device-123".to_string(),
        );
        let serialized = original.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert_eq!(original.msg_type, deserialized.msg_type);
        assert_eq!(original.payload, deserialized.payload);
        assert_eq!(original.device_id, deserialized.device_id);
    }

    #[test]
    fn test_timestamp_now() {
        let ts1 = Message::timestamp_now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = Message::timestamp_now();
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_clipboard_sync_convenience() {
        let msg = Message::clipboard_sync(b"test content", "device-456".to_string());
        assert_eq!(msg.msg_type, MessageType::ClipboardSync);
        assert_eq!(msg.payload, b"test content".to_vec());
        assert_eq!(msg.device_id, "device-456");
    }

    #[test]
    fn test_heartbeat_convenience() {
        let msg = Message::heartbeat("device-789".to_string());
        assert_eq!(msg.msg_type, MessageType::Heartbeat);
        assert!(msg.payload.is_empty());
    }

    #[test]
    fn test_ack_convenience() {
        let msg = Message::ack("device-abc".to_string());
        assert_eq!(msg.msg_type, MessageType::Ack);
        assert!(msg.payload.is_empty());
    }

    #[test]
    fn test_empty_payload() {
        let msg = Message::new(
            MessageType::Heartbeat,
            Vec::new(),
            "device-def".to_string(),
        );
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert!(deserialized.payload.is_empty());
    }

    #[test]
    fn test_all_message_types() {
        let types = [
            MessageType::ClipboardSync,
            MessageType::PairingRequest,
            MessageType::PairingResponse,
            MessageType::Heartbeat,
            MessageType::Ack,
        ];
        for msg_type in types {
            let serialized = rmp_serde::to_vec(&msg_type).unwrap();
            let deserialized: MessageType = rmp_serde::from_slice(&serialized).unwrap();
            assert_eq!(msg_type, deserialized);
        }
    }

    #[test]
    fn test_large_payload() {
        let large_payload = vec![0u8; 10000];
        let msg = Message::new(
            MessageType::ClipboardSync,
            large_payload.clone(),
            "device-large".to_string(),
        );
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.payload.len(), 10000);
    }
}
```

**lib.rs 更新模板：**
```rust
//! NearClip Sync Module
//!
//! Synchronization protocol implementation including message format,
//! channel selection strategy, and retry logic.

pub mod protocol;

// Re-export protocol types for convenience
pub use protocol::{Message, MessageType};

// Future modules:
// mod sender;     // Clipboard content sending logic
// mod receiver;   // Clipboard content receiving logic
// mod channel;    // Channel selection and switching
```

### References

- [Source: docs/architecture.md#Message Protocol Patterns]
- [Source: docs/project_context.md#消息协议规则]
- [Source: docs/epics.md#Story 1.4]

## Dev Agent Record

### Context Reference

Story context created by create-story workflow with exhaustive artifact analysis.

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- cargo build -p nearclip-sync: SUCCESS
- cargo test -p nearclip-sync: 29 unit tests + 8 doc tests passed (post code review)

### Completion Notes List

- Created `crates/nearclip-sync/src/protocol.rs` with MessageType enum and Message struct
- MessageType enum includes 5 variants: ClipboardSync, PairingRequest, PairingResponse, Heartbeat, Ack
- Message struct includes: msg_type, payload (Vec<u8>), timestamp (u64), device_id (String)
- Implemented serialize/deserialize methods using rmp-serde (MessagePack)
- Added convenience constructors: clipboard_sync, pairing_request, pairing_response, heartbeat, ack, ack_with_payload
- Added utility methods: timestamp_now, is_expired, age_ms
- Added MessageType methods: as_str, requires_ack
- Updated lib.rs to export protocol module and re-export Message/MessageType
- Added comprehensive unit tests covering:
  - Serialization roundtrip
  - All message types
  - Large payloads (10KB)
  - Binary payloads
  - Unicode device IDs
  - Message expiration
  - Error handling for invalid data
- All 37 tests pass (29 unit + 8 doc tests)

### Code Review Fixes (2025-12-13)

以下问题在代码审查后已修复：

| Issue | Severity | Fix |
|-------|----------|-----|
| `timestamp_now()` 使用 `.expect()` | MEDIUM | 改用 `unwrap_or(0)` 安全处理 |
| `serialize()`/`deserialize()` 返回外部库错误类型 | MEDIUM | 新增 `ProtocolError` 类型，统一错误处理 |
| payload 文档描述不准确 | LOW | 修正为"原始载荷数据" |
| MessageType 缺少 Default 实现 | LOW | 添加 `#[derive(Default)]` 和 `#[default]` 属性 |

新增类型：
- `ProtocolError::Serialization(String)` - 序列化错误
- `ProtocolError::Deserialization(String)` - 反序列化错误

新增测试：
- `test_message_type_default` - 验证 Default 实现
- `test_protocol_error_display` - 验证错误显示
- `test_protocol_error_clone_eq` - 验证 Clone 和 PartialEq
- `test_deserialize_error_type` - 验证错误类型匹配

### File List

**New Files:**
- crates/nearclip-sync/src/protocol.rs

**Modified Files:**
- crates/nearclip-sync/src/lib.rs

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] protocol.rs 文件已创建
- [x] MessageType 枚举包含所有五个变体
- [x] Message 结构体包含所有必需字段
- [x] serialize() 和 deserialize() 方法正确工作
- [x] 便捷构造器 (clipboard_sync, heartbeat, ack) 可用
- [x] lib.rs 正确导出 protocol 模块
- [x] Message 和 MessageType 可以从 crate 根导入
- [x] 所有单元测试通过
- [x] `cargo build` 成功
- [x] `cargo test -p nearclip-sync` 成功

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-13 | Story created by create-story workflow |
| 2025-12-13 | Implementation completed - all tasks done, 33 tests passing |
| 2025-12-13 | Code review completed - 4 issues fixed, 37 tests passing |

---

Story created: 2025-12-13
Implementation completed: 2025-12-13
Code review completed: 2025-12-13
