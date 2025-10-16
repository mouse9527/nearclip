# 编码标准

## 关键的全栈规则

- **类型安全：** 在所有平台（Rust、Kotlin、Swift）都使用强类型系统
- **消息验证：** 所有接收到的 Protocol Buffers 消息必须验证格式、签名和时间戳
- **错误处理：** 所有网络操作必须包含适当的错误处理和重试逻辑
- **资源管理：** BLE 连接和 Rust 内存必须正确管理生命周期
- **日志记录：** 使用统一的日志格式，不记录敏感信息
- **状态管理：** UI 状态更新必须通过状态管理器进行
- **加密安全：** 所有设备间通信必须端到端加密
- **FFI 安全：** 所有跨语言调用必须验证参数和返回值
- **内存安全：** Rust 代码必须避免内存泄漏和数据竞争

## 多语言命名约定

| 元素 | Rust | Kotlin | Swift | 示例 |
|------|------|--------|-------|------|
| 结构体/类 | PascalCase | PascalCase | PascalCase | `DeviceInfo` |
| 函数/方法 | snake_case | camelCase | camelCase | `start_device_discovery()` / `startDeviceDiscovery()` |
| 变量 | snake_case | camelCase | camelCase | `device_count` / `deviceCount` |
| 常量 | UPPER_SNAKE_CASE | UPPER_SNAKE_CASE | UPPER_SNAKE_CASE | `MAX_RETRY_COUNT` |
| 枚举 | PascalCase | PascalCase | PascalCase | `ConnectionStatus` |
| 模块/包 | snake_case | snake_case | snake_case | `device_manager` |
| FFI 函数 | snake_case | camelCase | camelCase | `nearclip_start_discovery()` |

## Rust 编码规范

### 基础规范

```rust
// 使用 rustfmt 进行代码格式化
// 使用 clippy 进行代码检查

// 错误处理：使用 Result 类型
fn connect_to_device(device_id: &str) -> Result<Connection, NearclipError> {
    // 实现逻辑
}

// 使用 Option 处理可选值
fn get_device(device_id: &str) -> Option<&Device> {
    // 实现逻辑
}

// 使用 ? 操作符进行错误传播
fn send_message(device: &Device, message: &[u8]) -> Result<(), NearclipError> {
    let connection = establish_connection(&device.id)?;
    connection.send(message)?;
    Ok(())
}
```

### FFI 接口规范

```rust
// FFI 函数必须使用 extern "C"
#[no_mangle]
pub extern "C" fn nearclip_start_discovery(
    callback: extern "C" fn(device: *const DeviceInfo)
) -> NearclipResult {
    // 安全检查
    if callback.is_null() {
        return NearclipResult::InvalidArgument;
    }

    // 实现逻辑
    match start_discovery_internal(callback) {
        Ok(()) => NearclipResult::Success,
        Err(e) => {
            log::error!("Discovery failed: {}", e);
            NearclipResult::InternalError
        }
    }
}

// 内存安全的字符串处理
#[no_mangle]
pub extern "C" fn nearclip_get_error_message() -> *const c_char {
    let error_msg = get_last_error();
    // 使用 CString 确保内存安全
    CString::new(error_msg).unwrap().into_raw()
}

// 记住要释放分配的内存
#[no_mangle]
pub extern "C" fn nearclip_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
```

### 并发安全规范

```rust
// 使用 Arc + Mutex 进行共享状态
use std::sync::{Arc, Mutex};
use std::thread;

pub struct DeviceManager {
    devices: Arc<Mutex<Vec<Device>>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_device(&self, device: Device) -> Result<(), DeviceError> {
        let mut devices = self.devices.lock().unwrap();
        devices.push(device);
        Ok(())
    }
}

// 使用 tokio 进行异步操作
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DeviceManager::new();

    // 异步设备发现
    tokio::spawn(async move {
        discover_devices(&manager).await;
    });

    Ok(())
}
```

## Kotlin 编码规范

### 基础规范

```kotlin
// 使用可空类型
fun findDevice(deviceId: String): Device? {
    return deviceRepository.findById(deviceId)
}

// 使用 sealed class 定义错误类型
sealed class NearclipError : Exception() {
    object DeviceNotFound : NearclipError()
    object ConnectionFailed : NearclipError()
    data class ProtocolError(val message: String) : NearclipError()
}

// 使用协程处理异步操作
suspend fun sendSyncMessage(device: Device, content: String): Result<Unit> {
    return try {
        rustBridge.sendSyncMessage(device.id, content.toByteArray())
        Result.success(Unit)
    } catch (e: Exception) {
        Result.failure(e)
    }
}
```

### JNI 集成规范

```kotlin
class RustNativeBridge {
    companion object {
        init {
            // 加载 Rust 库
            System.loadLibrary("nearclip_core")
        }

        // 安全的 JNI 调用
        @JvmStatic
        external fun startDeviceDiscovery(callback: DeviceDiscoveryCallback): Int

        // 包装 JNI 调用，提供 Kotlin 友好的 API
        fun startDeviceDiscoverySafe(callback: (Device) -> Unit): Result<Unit> {
            return try {
                val result = startDeviceDiscovery(object : DeviceDiscoveryCallback {
                    override fun onDeviceFound(device: Device) {
                        callback(device)
                    }
                })

                if (result == 0) {
                    Result.success(Unit)
                } else {
                    Result.failure(NearclipException("Failed to start discovery: $result"))
                }
            } catch (e: Exception) {
                Result.failure(e)
            }
        }
    }
}
```

## Swift 编码规范

### 基础规范

```swift
// 使用 struct 进行值类型建模
struct Device: Codable, Identifiable, Equatable {
    let id: String
    let name: String
    let type: DeviceType
    var connectionStatus: ConnectionStatus

    enum DeviceType: String, Codable, CaseIterable {
        case android = "ANDROID"
        case mac = "MAC"
    }
}

// 使用 Result 类型处理错误
enum NearclipError: Error, LocalizedError {
    case deviceNotFound(String)
    case connectionFailed(String)
    case protocolError(String)

    var errorDescription: String? {
        switch self {
        case .deviceNotFound(let id):
            return "Device not found: \(id)"
        case .connectionFailed(let reason):
            return "Connection failed: \(reason)"
        case .protocolError(let message):
            return "Protocol error: \(message)"
        }
    }
}

// 使用 async/await 处理异步操作
func sendSyncMessage(to device: Device, content: String) async throws {
    try await withCheckedThrowingContinuation { continuation in
        rustBridge.sendSyncMessage(deviceId: device.id, content: content) { result in
            if result == 0 {
                continuation.resume()
            } else {
                continuation.resume(throwing: NearclipError.connectionFailed("Error code: \(result)"))
            }
        }
    }
}
```

### C 互操作规范

```swift
// 安全的 C 接口封装
struct NearclipDevice {
    let deviceId: UnsafePointer<Int8>
    let deviceName: UnsafePointer<Int8>
    let deviceType: NearclipDeviceType
    let isConnected: Bool
}

// 使用 @_silgen_name 指定 C 函数名
@_silgen_name("nearclip_start_discovery")
func nearclip_start_discovery(_ callback: @convention(c) (UnsafePointer<NearclipDevice>) -> Void) -> NearclipResult

// Swift 友好的包装类
class NearClipManager {
    private var deviceCallback: ((Device) -> Void)?

    func startDeviceDiscovery(callback: @escaping (Device) -> Void) throws {
        self.deviceCallback = callback

        let result = nearclip_start_discovery { [weak self] cDevice in
            guard let self = self else { return }
            let device = self.convertCDevice(cDevice)
            DispatchQueue.main.async {
                callback(device)
            }
        }

        guard result == .success else {
            throw NearclipError.connectionFailed("Failed to start discovery")
        }
    }

    private func convertCDevice(_ cDevice: UnsafePointer<NearclipDevice>) -> Device {
        let deviceId = String(cString: cDevice.pointee.deviceId)
        let deviceName = String(cString: cDevice.pointee.deviceName)
        let deviceType = DeviceType(rawValue: String(cString: cDevice.pointee.deviceType.rawValue)) ?? .unknown

        return Device(
            id: deviceId,
            name: deviceName,
            type: deviceType,
            connectionStatus: cDevice.pointee.isConnected ? .connected : .disconnected
        )
    }
}
```

## 跨语言一致性规则

### 错误处理统一

所有平台都必须遵循相同的错误处理模式：

1. **错误类型统一**: 使用相同的错误码和错误消息
2. **日志格式统一**: 使用相同的日志级别和格式
3. **重试策略统一**: 使用相同的重试次数和退避策略

### 内存管理安全

1. **Rust**: 遵循所有权和借用规则
2. **Kotlin**: 注意 JNI 对象生命周期管理
3. **Swift**: 正确处理 C 指针和内存释放

### Protocol Buffers 一致性

1. **字段命名**: 使用 snake_case，各语言自动转换
2. **版本兼容**: 遵循 Protocol Buffers 向后兼容规则
3. **验证规则**: 所有平台使用相同的验证逻辑
