# Story 4.1: 生成 uniffi Swift 绑定

Status: done

## Story

As a macOS 用户,
I want Swift 可以调用 Rust 核心库,
So that 应用能使用同步功能.

## Acceptance Criteria

1. **Given** nearclip-ffi crate 已实现 **When** 运行 uniffi-bindgen **Then** 生成 Swift 绑定代码
2. **And** 生成 XCFramework
3. **And** 可在 Xcode 项目中导入
4. **And** 测试验证基本调用成功

## Tasks / Subtasks

- [x] Task 1: 创建 UDL 接口定义 (AC: 1)
  - [x] 1.1 创建 `crates/nearclip-ffi/src/nearclip.udl`
  - [x] 1.2 定义 namespace nearclip
  - [x] 1.3 定义 NearClipError 错误枚举
  - [x] 1.4 定义 DevicePlatform、DeviceStatus 枚举
  - [x] 1.5 定义 DeviceInfo 结构体
  - [x] 1.6 定义 NearClipConfig 结构体
  - [x] 1.7 定义 NearClipCallback 回调接口
  - [x] 1.8 定义 NearClipManager 接口

- [x] Task 2: 配置 uniffi 构建 (AC: 1)
  - [x] 2.1 创建 `crates/nearclip-ffi/uniffi.toml`
  - [x] 2.2 创建 `crates/nearclip-ffi/build.rs`
  - [x] 2.3 更新 Cargo.toml 添加 build-dependencies

- [x] Task 3: 实现 FFI 包装层 (AC: 1, 4)
  - [x] 3.1 更新 `lib.rs` 启用 uniffi scaffolding
  - [x] 3.2 实现 FFI 错误类型映射
  - [x] 3.3 实现 FFI 枚举类型导出
  - [x] 3.4 实现 FFI 结构体导出
  - [x] 3.5 实现 NearClipCallback trait 导出
  - [x] 3.6 实现 NearClipManager 导出

- [x] Task 4: 生成 Swift 绑定 (AC: 1, 2)
  - [x] 4.1 创建 uniffi-bindgen 二进制
  - [x] 4.2 创建构建脚本 `scripts/build-swift.sh`
  - [x] 4.3 生成 Swift 源码文件 (NearClip.swift)
  - [x] 4.4 生成 modulemap 文件 (NearClipFFI.modulemap)

- [x] Task 5: 构建 macOS 静态库 (AC: 2)
  - [x] 5.1 构建 aarch64-apple-darwin (Apple Silicon)
  - [x] 5.2 构建 x86_64-apple-darwin (Intel) - 可选，需安装 target
  - [x] 5.3 使用 lipo 合并为通用二进制 (或单架构)
  - [x] 5.4 创建 XCFramework 结构

- [x] Task 6: 编写测试 (AC: 4)
  - [x] 6.1 创建 Rust 端单元测试 (9 个测试)
  - [x] 6.2 验证 UDL 编译无错误
  - [x] 6.3 验证静态库导出符号正确

- [x] Task 7: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 7.1 运行 `cargo build -p nearclip-ffi`
  - [x] 7.2 运行 `cargo test -p nearclip-ffi`
  - [x] 7.3 运行 `cargo clippy -p nearclip-ffi`
  - [x] 7.4 运行构建脚本生成完整输出

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **uniffi 0.28**: 使用 Mozilla 官方 uniffi 工具
2. **单点导出**: nearclip-ffi 是唯一的 FFI 层
3. **命名约定**: Rust snake_case，uniffi 自动转换到 Swift camelCase
4. **错误处理**: 使用 `[Throws=NearClipError]` 标记可失败函数

### 与其他 Story 的关系

- Story 3-11: NearClipManager 已实现，本 Story 导出其 FFI 接口
- Story 4-2: macOS 菜单栏应用将使用本 Story 生成的绑定
- Story 5-1: Android Kotlin 绑定将复用相同的 UDL 定义

### UDL 接口设计

```udl
namespace nearclip {
    // 日志初始化
    void init_logging(LogLevel level);
    void flush_logs();
};

[Error]
enum NearClipError {
    "Network",
    "Bluetooth",
    "Crypto",
    "DeviceNotFound",
    "Config",
    "Protocol",
    "Sync",
    "Timeout",
    "NotRunning",
    "AlreadyRunning",
};

enum LogLevel {
    "Trace",
    "Debug",
    "Info",
    "Warn",
    "Error",
};

enum DevicePlatform {
    "MacOS",
    "Android",
    "Unknown",
};

enum DeviceStatus {
    "Connected",
    "Disconnected",
    "Connecting",
    "Failed",
};

dictionary DeviceInfo {
    string id;
    string name;
    DevicePlatform platform;
    DeviceStatus status;
};

dictionary NearClipConfig {
    string device_name;
    boolean wifi_enabled;
    boolean ble_enabled;
    boolean auto_connect;
    u64 connection_timeout_secs;
    u64 heartbeat_interval_secs;
    u32 max_retries;
};

callback interface NearClipCallback {
    void on_device_connected(DeviceInfo device);
    void on_device_disconnected(string device_id);
    void on_clipboard_received(bytes content, string from_device);
    void on_sync_error(NearClipError error);
};

interface NearClipManager {
    [Throws=NearClipError]
    constructor(NearClipConfig config, NearClipCallback callback);

    [Throws=NearClipError]
    void start();

    void stop();

    boolean is_running();

    [Throws=NearClipError]
    void sync_clipboard(bytes content);

    sequence<DeviceInfo> get_paired_devices();
    sequence<DeviceInfo> get_connected_devices();

    [Throws=NearClipError]
    void connect_device(string device_id);

    [Throws=NearClipError]
    void disconnect_device(string device_id);

    void add_paired_device(DeviceInfo device);
    void remove_paired_device(string device_id);
};
```

### 输出目录结构

```
target/
└── swift/
    ├── nearclip.swift           # Swift 绑定代码
    ├── nearclipFFI.h            # C header
    ├── nearclipFFI.modulemap    # Module map
    └── NearClip.xcframework/    # Universal framework
        └── macos-arm64_x86_64/
            └── libnearclip_ffi.a
```

### 构建命令参考

```bash
# 安装 uniffi-bindgen
cargo install uniffi-bindgen-cli --version 0.28

# 生成 Swift 绑定
uniffi-bindgen generate \
    crates/nearclip-ffi/src/nearclip.udl \
    --language swift \
    --out-dir target/swift

# 构建静态库
cargo build --release -p nearclip-ffi --target aarch64-apple-darwin
cargo build --release -p nearclip-ffi --target x86_64-apple-darwin

# 合并通用二进制
lipo -create \
    target/aarch64-apple-darwin/release/libnearclip_ffi.a \
    target/x86_64-apple-darwin/release/libnearclip_ffi.a \
    -output target/swift/libnearclip_ffi.a
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing (9 tests)
- [x] Clippy warnings resolved (only 1 from generated code)
- [x] Swift bindings generated (NearClip.swift - 50KB)
- [x] XCFramework created (arm64 for Apple Silicon)
- [x] Story file updated to 'done'
