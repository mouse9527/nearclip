# Task 4.3: FFI 层测试实施计划

**任务**: 为 nearclip-ffi 添加单元测试和 smoke 测试
**优先级**: 🔴 高
**估计时间**: 4-6 小时
**依赖**: Task 4.2 (代码覆盖率分析)
**目标**: 将 FFI 层测试覆盖率从 0% 提升到 60%+

---

## 1. 背景

根据 Task 4.2 的覆盖率分析,`nearclip-ffi` 是唯一没有任何测试的 crate,覆盖率为 0%。这是一个高优先级的改进项,因为 FFI 层是平台集成的关键接口。

### 1.1 FFI 层特点

- **UniFFI 生成代码**: 大部分绑定代码由 uniffi 自动生成
- **平台依赖**: 部分功能依赖平台实现(Swift/Kotlin)
- **回调接口**: 使用回调模式与平台交互
- **类型转换**: 需要在 FFI 类型和内部类型之间转换

### 1.2 测试挑战

- FFI 层难以直接测试(需要 Mock 平台回调)
- UniFFI 生成的代码不需要测试
- 跨语言集成测试复杂

### 1.3 测试策略

- ✅ **Smoke 测试**: 验证基本功能可用
- ✅ **类型转换测试**: 验证 FFI 类型转换正确性
- ✅ **错误处理测试**: 验证错误正确传播
- ✅ **Mock 回调测试**: 使用 Mock 实现测试回调机制
- ⏳ **跨语言测试**: 留待 Task 4.4 端到端测试

---

## 2. 测试计划

### 2.1 Smoke 测试 (2-3 小时)

#### Test 1.1: FfiNearClipManager 创建
```rust
#[tokio::test]
async fn test_ffi_manager_creation() {
    let config = create_test_config();
    let callback = Arc::new(MockCallback::new());

    let manager = FfiNearClipManager::new(config, callback);
    assert!(manager.is_ok());
}
```

#### Test 1.2: 基础生命周期
```rust
#[tokio::test]
async fn test_ffi_manager_lifecycle() {
    let manager = create_test_manager();

    // 启动
    manager.start().await.unwrap();
    assert!(manager.is_running());

    // 停止
    manager.stop().await;
    assert!(!manager.is_running());
}
```

#### Test 1.3: 获取设备 ID
```rust
#[test]
fn test_ffi_get_device_id() {
    let manager = create_test_manager();
    let device_id = manager.get_device_id();

    assert!(!device_id.is_empty());
    assert_eq!(device_id.len(), 36); // UUID 长度
}
```

#### Test 1.4: QR 码生成
```rust
#[tokio::test]
async fn test_ffi_generate_qr_code() {
    let manager = create_test_manager();
    let qr_data = manager.generate_qr_code().await;

    assert!(qr_data.is_ok());
    let qr_string = qr_data.unwrap();
    assert!(!qr_string.is_empty());

    // 验证 JSON 格式
    let parsed: serde_json::Value = serde_json::from_str(&qr_string).unwrap();
    assert!(parsed["device_id"].is_string());
    assert!(parsed["public_key"].is_string());
}
```

#### Test 1.5: 配对设备管理
```rust
#[tokio::test]
async fn test_ffi_device_management() {
    let manager = create_test_manager();

    // 初始状态无设备
    assert_eq!(manager.get_paired_devices().await.len(), 0);

    // 添加设备
    let device = create_test_device_info();
    manager.add_paired_device(device.clone()).await;

    // 验证设备已添加
    let devices = manager.get_paired_devices().await;
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, device.id);

    // 移除设备
    manager.remove_paired_device(&device.id).await;
    assert_eq!(manager.get_paired_devices().await.len(), 0);
}
```

---

### 2.2 类型转换测试 (1 小时)

#### Test 2.1: FfiDeviceInfo 转换
```rust
#[test]
fn test_ffi_device_info_conversion() {
    use nearclip_core::DeviceInfo;

    let ffi_device = FfiDeviceInfo {
        id: "test-id".to_string(),
        name: "Test Device".to_string(),
        platform: DevicePlatform::MacOS,
        status: DeviceStatus::Connected,
    };

    let device: DeviceInfo = ffi_device.clone().into();
    assert_eq!(device.id, ffi_device.id);
    assert_eq!(device.name, ffi_device.name);

    let ffi_device2: FfiDeviceInfo = device.into();
    assert_eq!(ffi_device2.id, ffi_device.id);
}
```

#### Test 2.2: FfiNearClipConfig 转换
```rust
#[test]
fn test_ffi_config_conversion() {
    let ffi_config = FfiNearClipConfig {
        device_name: "My Device".to_string(),
        device_id: "test-id".to_string(),
        wifi_enabled: true,
        ble_enabled: true,
        auto_connect: true,
        connection_timeout_secs: 30,
        heartbeat_interval_secs: 10,
        max_retries: 3,
    };

    let config: NearClipConfig = ffi_config.clone().into();
    assert_eq!(config.device_name, ffi_config.device_name);
    assert_eq!(config.wifi_enabled, ffi_config.wifi_enabled);
}
```

#### Test 2.3: FfiSyncHistoryEntry 转换
```rust
#[test]
fn test_ffi_history_entry_conversion() {
    let ffi_entry = FfiSyncHistoryEntry {
        id: 1,
        device_id: "test".to_string(),
        device_name: "Test".to_string(),
        content_preview: "Hello".to_string(),
        content_size: 5,
        direction: "sent".to_string(),
        timestamp_ms: 1000000,
        success: true,
        error_message: None,
    };

    let entry: SyncHistoryEntry = ffi_entry.clone().into();
    assert_eq!(entry.id, ffi_entry.id);
    assert_eq!(entry.success, ffi_entry.success);
}
```

---

### 2.3 错误处理测试 (1 小时)

#### Test 3.1: 未初始化错误
```rust
#[tokio::test]
async fn test_ffi_not_initialized_error() {
    let manager = create_test_manager();

    // 未调用 start() 就尝试操作
    let result = manager.sync_clipboard(vec![1, 2, 3]).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        NearClipError::NotInitialized => {},
        _ => panic!("Expected NotInitialized error"),
    }
}
```

#### Test 3.2: 设备不存在错误
```rust
#[tokio::test]
async fn test_ffi_device_not_found_error() {
    let manager = create_test_manager();
    manager.start().await.unwrap();

    let result = manager.connect_device("nonexistent-id").await;
    assert!(result.is_err());

    match result.unwrap_err() {
        NearClipError::DeviceNotFound => {},
        _ => panic!("Expected DeviceNotFound error"),
    }
}
```

#### Test 3.3: QR 码解析错误
```rust
#[tokio::test]
async fn test_ffi_invalid_qr_code() {
    let manager = create_test_manager();

    // 无效的 QR 数据
    let result = manager.pair_with_qr_code("invalid json").await;
    assert!(result.is_err());
}
```

---

### 2.4 Mock 回调测试 (1-2 小时)

#### Test 4.1: 设备连接回调
```rust
#[tokio::test]
async fn test_ffi_callback_device_connected() {
    let callback = Arc::new(MockCallback::new());
    let manager = create_manager_with_callback(callback.clone());

    // 模拟设备连接
    let device = create_test_device_info();
    manager.add_paired_device(device.clone()).await;
    manager.connect_device(&device.id).await.unwrap();

    // 验证回调被调用
    assert!(callback.was_called("on_device_connected"));
    assert_eq!(callback.get_connected_device_id(), Some(device.id));
}
```

#### Test 4.2: 剪贴板接收回调
```rust
#[tokio::test]
async fn test_ffi_callback_clipboard_received() {
    let callback = Arc::new(MockCallback::new());
    let manager = create_manager_with_callback(callback.clone());

    // 模拟剪贴板数据接收
    let content = b"Test clipboard content";
    // ... 触发接收逻辑 ...

    // 验证回调被调用
    assert!(callback.was_called("on_clipboard_received"));
    assert_eq!(callback.get_received_content(), Some(content.to_vec()));
}
```

#### Test 4.3: 错误回调
```rust
#[tokio::test]
async fn test_ffi_callback_sync_error() {
    let callback = Arc::new(MockCallback::new());
    let manager = create_manager_with_callback(callback.clone());

    // 触发同步错误
    // ... 模拟错误场景 ...

    // 验证回调被调用
    assert!(callback.was_called("on_sync_error"));
    let error_msg = callback.get_error_message().unwrap();
    assert!(!error_msg.is_empty());
}
```

---

### 2.5 辅助工具函数 (1 小时)

创建测试辅助文件: `tests/common/mod.rs`

```rust
use std::sync::{Arc, Mutex};
use nearclip_ffi::*;

/// Mock 回调实现
pub struct MockCallback {
    calls: Arc<Mutex<Vec<String>>>,
    connected_devices: Arc<Mutex<Vec<String>>>,
    received_content: Arc<Mutex<Option<Vec<u8>>>>,
    error_messages: Arc<Mutex<Vec<String>>>,
}

impl MockCallback {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            connected_devices: Arc::new(Mutex::new(Vec::new())),
            received_content: Arc::new(Mutex::new(None)),
            error_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn was_called(&self, method: &str) -> bool {
        self.calls.lock().unwrap().contains(&method.to_string())
    }

    pub fn get_connected_device_id(&self) -> Option<String> {
        self.connected_devices.lock().unwrap().last().cloned()
    }

    pub fn get_received_content(&self) -> Option<Vec<u8>> {
        self.received_content.lock().unwrap().clone()
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error_messages.lock().unwrap().last().cloned()
    }
}

impl FfiNearClipCallback for MockCallback {
    fn on_device_connected(&self, device: FfiDeviceInfo) {
        self.calls.lock().unwrap().push("on_device_connected".to_string());
        self.connected_devices.lock().unwrap().push(device.id);
    }

    fn on_device_disconnected(&self, device_id: String) {
        self.calls.lock().unwrap().push("on_device_disconnected".to_string());
    }

    fn on_clipboard_received(&self, content: Vec<u8>, from_device: String) {
        self.calls.lock().unwrap().push("on_clipboard_received".to_string());
        *self.received_content.lock().unwrap() = Some(content);
    }

    fn on_sync_error(&self, error_message: String) {
        self.calls.lock().unwrap().push("on_sync_error".to_string());
        self.error_messages.lock().unwrap().push(error_message);
    }

    // ... 其他回调方法 ...
}

/// 创建测试配置
pub fn create_test_config() -> FfiNearClipConfig {
    FfiNearClipConfig {
        device_name: "Test Device".to_string(),
        device_id: "test-device-id".to_string(),
        wifi_enabled: true,
        ble_enabled: true,
        auto_connect: false,
        connection_timeout_secs: 30,
        heartbeat_interval_secs: 10,
        max_retries: 3,
    }
}

/// 创建测试设备信息
pub fn create_test_device_info() -> FfiDeviceInfo {
    FfiDeviceInfo {
        id: "test-device-1".to_string(),
        name: "Test Device 1".to_string(),
        platform: DevicePlatform::MacOS,
        status: DeviceStatus::Disconnected,
    }
}

/// 创建测试管理器
pub fn create_test_manager() -> FfiNearClipManager {
    let config = create_test_config();
    let callback = Arc::new(MockCallback::new());
    FfiNearClipManager::new(config, callback).unwrap()
}
```

---

## 3. 测试文件结构

```
crates/nearclip-ffi/
├── tests/
│   ├── common/
│   │   ├── mod.rs                 # Mock 回调和辅助函数
│   │   └── mock_callback.rs       # Mock 回调实现
│   ├── smoke_tests.rs             # Smoke 测试 (创建、生命周期)
│   ├── type_conversion_tests.rs   # 类型转换测试
│   ├── error_handling_tests.rs    # 错误处理测试
│   └── callback_tests.rs          # 回调机制测试
```

---

## 4. 验收标准

- [ ] 至少 15 个 FFI 层测试
- [ ] Smoke 测试覆盖所有主要 API
- [ ] 类型转换测试覆盖所有 FFI 类型
- [ ] 错误处理测试覆盖主要错误场景
- [ ] Mock 回调测试验证回调机制
- [ ] 所有测试编译通过
- [ ] 所有测试执行通过
- [ ] FFI 层估算覆盖率 > 60%

---

## 5. 不包含在本任务中

- ❌ 跨语言集成测试(Swift/Kotlin) - 留待 Task 4.4
- ❌ BLE 硬件接口测试 - 需要真实硬件或高级 Mock
- ❌ 设备存储接口测试 - 平台相关,留待端到端测试
- ❌ 实际网络传输测试 - 留待 Task 4.4

---

## 6. 风险和缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| UniFFI 生成代码难测试 | 中 | 只测试手写代码,信任 UniFFI |
| 回调测试复杂 | 中 | 使用简单的 Mock 实现 |
| 异步测试不稳定 | 低 | 使用 tokio::test,添加超时 |

---

## 7. 时间估算

| 任务 | 估计时间 |
|------|----------|
| Smoke 测试 | 2-3 小时 |
| 类型转换测试 | 1 小时 |
| 错误处理测试 | 1 小时 |
| Mock 回调测试 | 1-2 小时 |
| 测试辅助工具 | 1 小时 |
| **总计** | **6-8 小时** |

---

## 8. 实施步骤

### Step 1: 创建测试基础设施 (1 小时)
1. 创建 `tests/common/mod.rs`
2. 实现 `MockCallback`
3. 实现辅助函数

### Step 2: 实现 Smoke 测试 (2 小时)
1. `test_ffi_manager_creation`
2. `test_ffi_manager_lifecycle`
3. `test_ffi_get_device_id`
4. `test_ffi_generate_qr_code`
5. `test_ffi_device_management`

### Step 3: 实现类型转换测试 (1 小时)
1. `test_ffi_device_info_conversion`
2. `test_ffi_config_conversion`
3. `test_ffi_history_entry_conversion`

### Step 4: 实现错误处理测试 (1 小时)
1. `test_ffi_not_initialized_error`
2. `test_ffi_device_not_found_error`
3. `test_ffi_invalid_qr_code`

### Step 5: 实现回调测试 (1-2 小时)
1. `test_ffi_callback_device_connected`
2. `test_ffi_callback_clipboard_received`
3. `test_ffi_callback_sync_error`

### Step 6: 运行和验证 (1 小时)
1. 运行所有测试
2. 修复编译错误
3. 修复失败的测试
4. 生成测试报告

---

## 9. 后续任务

完成 Task 4.3 后,建议继续:
- **Task 4.4**: 端到端平台测试 (macOS ↔ Android)
- **Task 4.5**: CI/CD 集成
- **Task 4.6**: 测试文档和指南

---

**创建时间**: 2026-01-14
**预计完成**: 2026-01-15
**依赖任务**: Task 4.2 ✅
