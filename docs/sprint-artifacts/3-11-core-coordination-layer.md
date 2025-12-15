# Story 3.11: 实现核心协调层

Status: done

## Story

As a 用户,
I want 统一的接口管理所有同步功能,
So that 平台客户端调用简单.

## Acceptance Criteria

1. **Given** 所有底层模块已实现 **When** 初始化 NearClipManager **Then** 提供 start/stop/sync 等高层接口
2. **And** 协调 net、ble、crypto、sync 模块
3. **And** 管理设备列表和连接状态
4. **And** 测试验证完整工作流

## Tasks / Subtasks

- [x] Task 1: 定义设备信息结构 (AC: 3)
  - [x] 1.1 创建 `crates/nearclip-core/src/device.rs`
  - [x] 1.2 定义 `DeviceInfo` 结构体 (id, name, platform, connection_status)
  - [x] 1.3 定义 `DeviceStatus` 枚举 (Connected, Disconnected, Connecting)
  - [x] 1.4 定义 `DevicePlatform` 枚举 (MacOS, Android, Unknown)

- [x] Task 2: 定义配置结构 (AC: 1, 2)
  - [x] 2.1 创建 `crates/nearclip-core/src/config.rs`
  - [x] 2.2 定义 `NearClipConfig` 结构体
  - [x] 2.3 实现 builder pattern
  - [x] 2.4 实现 `validate()` 方法

- [x] Task 3: 定义回调接口 (AC: 1, 3)
  - [x] 3.1 创建 `NearClipCallback` trait
  - [x] 3.2 定义 `on_device_connected(&self, device: &DeviceInfo)`
  - [x] 3.3 定义 `on_device_disconnected(&self, device_id: &str)`
  - [x] 3.4 定义 `on_clipboard_received(&self, content: &[u8], from_device: &str)`
  - [x] 3.5 定义 `on_sync_error(&self, error: &NearClipError)`

- [x] Task 4: 实现 NearClipManager 核心 (AC: 1, 2, 3)
  - [x] 4.1 创建 `crates/nearclip-core/src/manager.rs`
  - [x] 4.2 定义 `NearClipManager` 结构体
  - [x] 4.3 实现 `new(config, callback)` 构造函数
  - [x] 4.4 实现 `start()` 启动服务
  - [x] 4.5 实现 `stop()` 停止服务
  - [x] 4.6 实现 `sync_clipboard(content)` 同步剪贴板

- [x] Task 5: 实现设备管理 (AC: 3)
  - [x] 5.1 实现 `get_paired_devices() -> Vec<DeviceInfo>` 获取已配对设备
  - [x] 5.2 实现 `get_connected_devices() -> Vec<DeviceInfo>` 获取已连接设备
  - [x] 5.3 实现 `connect_device(device_id)` 连接设备
  - [x] 5.4 实现 `disconnect_device(device_id)` 断开设备

- [x] Task 6: 实现状态查询 (AC: 1, 3)
  - [x] 6.1 实现 `is_running() -> bool` 查询运行状态
  - [x] 6.2 实现 `get_device_status(device_id) -> Option<DeviceStatus>`
  - [x] 6.3 实现 `get_current_channel() -> Option<Channel>`

- [x] Task 7: 导出模块 (AC: 1)
  - [x] 7.1 在 `lib.rs` 添加模块导出
  - [x] 7.2 添加 re-exports

- [x] Task 8: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 测试 `DeviceInfo` 创建
  - [x] 8.2 测试 `NearClipConfig` 配置
  - [x] 8.3 测试 `NearClipManager` 创建
  - [x] 8.4 测试 start/stop 状态切换
  - [x] 8.5 测试设备列表管理

- [x] Task 9: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 9.1 创建 `tests/manager_integration.rs`
  - [x] 9.2 测试：完整生命周期 (create -> start -> stop)
  - [x] 9.3 测试：设备连接流程
  - [x] 9.4 测试：剪贴板同步流程

- [x] Task 10: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 10.1 运行 `cargo build -p nearclip-core` 确保无错误
  - [x] 10.2 运行 `cargo test -p nearclip-core` 确保测试通过
  - [x] 10.3 运行 `cargo clippy -p nearclip-core` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **核心协调层**: `nearclip-core` 作为统一入口
2. **模块协调**: 协调 net, ble, crypto, sync 模块
3. **回调模式**: 使用 trait 定义回调接口
4. **线程安全**: 使用 Arc<RwLock> 保护共享状态

### 与其他 Story 的关系

- Story 3-1 ~ 3-10: 底层同步模块已实现
- Story 4-1: macOS FFI 绑定将使用此协调层
- Story 5-1: Android FFI 绑定将使用此协调层

### 设计决策

1. **单例模式**: 每个应用一个 NearClipManager 实例
2. **异步操作**: 使用 tokio 处理异步任务
3. **回调通知**: 通过 callback 通知状态变化
4. **错误统一**: 所有错误转换为 NearClipError

### 数据结构

```rust
// 设备平台
enum DevicePlatform {
    MacOS,
    Android,
    Unknown,
}

// 设备状态
enum DeviceStatus {
    Connected,
    Disconnected,
    Connecting,
}

// 设备信息
struct DeviceInfo {
    id: String,
    name: String,
    platform: DevicePlatform,
    status: DeviceStatus,
}

// 配置
struct NearClipConfig {
    device_name: String,
    enable_wifi: bool,
    enable_ble: bool,
    auto_connect: bool,
}

// 回调接口
trait NearClipCallback: Send + Sync {
    fn on_device_connected(&self, device: &DeviceInfo);
    fn on_device_disconnected(&self, device_id: &str);
    fn on_clipboard_received(&self, content: &[u8], from_device: &str);
    fn on_sync_error(&self, error: &NearClipError);
}

// 核心管理器
struct NearClipManager {
    config: NearClipConfig,
    callback: Arc<dyn NearClipCallback>,
    devices: RwLock<HashMap<String, DeviceInfo>>,
    running: AtomicBool,
    // ... 内部模块引用
}
```

### 模块协调关系

```
                  NearClipManager
                        │
    ┌───────────────────┼───────────────────┐
    │                   │                   │
    ▼                   ▼                   ▼
nearclip-net      nearclip-ble      nearclip-sync
(TCP/mDNS)          (BLE)        (协议/通道切换/重试/防循环)
    │                   │                   │
    └───────────────────┼───────────────────┘
                        │
                        ▼
                  nearclip-crypto
                  (TLS/密钥/配对)
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing
- [x] Integration tests passing
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
