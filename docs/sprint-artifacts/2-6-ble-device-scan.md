# Story 2.6: 实现 BLE 设备扫描

Status: done

## Story

As a 用户,
I want 扫描附近的 NearClip 蓝牙设备,
So that 可以在无 WiFi 环境下配对.

## Acceptance Criteria

1. **Given** BLE 广播已实现 **When** 启动 BLE 中心模式扫描 **Then** 过滤 NearClip Service UUID
2. **And** 返回发现的设备列表
3. **And** 可以停止扫描
4. **And** 测试验证扫描结果正确

## Tasks / Subtasks

- [x] Task 1: 创建 BLE 扫描器核心结构 (AC: 1, 2)
  - [x] 1.1 创建 `crates/nearclip-ble/src/central.rs`
  - [x] 1.2 定义 `BleScannerConfig` 配置结构体（扫描超时、过滤模式等）
  - [x] 1.3 定义 `BleScanner` 主结构体
  - [x] 1.4 实现 `BleScanner::new(config)` 构造函数

- [x] Task 2: 定义发现设备数据结构 (AC: 2)
  - [x] 2.1 创建 `DiscoveredDevice` 结构体（device_id, rssi, public_key_hash, advertisement_data）
  - [x] 2.2 实现 `DiscoveredDevice::new()` 和 `validate()` 方法
  - [x] 2.3 确保与 `BleAdvertiserConfig` 数据格式一致

- [x] Task 3: 实现扫描启动和停止 (AC: 1, 3)
  - [x] 3.1 实现 `start()` 方法启动 BLE 中心模式扫描（当前返回 PlatformNotSupported，等待平台实现）
  - [x] 3.2 实现 `stop()` 方法停止扫描
  - [x] 3.3 实现 `is_scanning()` 状态查询
  - [x] 3.4 实现 Drop trait 确保资源清理

- [x] Task 4: 实现设备发现事件处理 (AC: 1, 2)
  - [x] 4.1 使用 `tokio::sync::broadcast` 实现设备发现事件通道
  - [x] 4.2 实现 `subscribe()` 方法返回设备发现 Receiver
  - [x] 4.3 过滤非 NearClip Service UUID 的设备（通过 config.filter_nearclip_only）
  - [x] 4.4 去重处理（同一设备多次广播 - HashMap 实现）

- [x] Task 5: 实现设备列表管理 (AC: 2)
  - [x] 5.1 实现 `discovered_devices()` 返回当前发现的设备列表
  - [x] 5.2 实现设备超时移除（`cleanup_expired_devices()` + `is_expired()`）
  - [x] 5.3 实现设备信息更新（`update()` 方法更新 RSSI）

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 中添加 `pub mod central;`
  - [x] 6.2 添加 re-exports: `BleScanner`, `BleScannerConfig`, `DiscoveredDevice`
  - [x] 6.3 更新模块文档

- [x] Task 7: 编写单元测试 (AC: 4)
  - [x] 7.1 测试 `BleScannerConfig` 验证 (6 tests)
  - [x] 7.2 测试 `DiscoveredDevice` 构造和验证 (7 tests)
  - [x] 7.3 测试 `BleScanner` 状态管理 (6 tests)
  - [x] 7.4 测试设备去重逻辑

- [x] Task 8: 编写集成测试 (AC: 4)
  - [x] 8.1 创建 `tests/ble_scan_integration.rs` (28 tests)
  - [x] 8.2 测试：启动扫描 → 验证状态 (#[ignore] - BLE dependent)
  - [x] 8.3 测试：停止扫描 → 验证状态 (#[ignore] - BLE dependent)
  - [x] 8.4 测试：重复启动/停止安全处理

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-ble` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-ble` 确保测试通过 (109 tests)
  - [x] 9.3 运行 `cargo clippy -p nearclip-ble` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

**错误处理规则（强制遵循）：**
- ✅ 所有公开函数必须返回 `Result<T, BleError>` 或 `Result<T, NearClipError>`
- ❌ 禁止在库代码中使用 `unwrap()` 或 `expect()`
- ❌ 禁止使用 `panic!()` 宏
- ✅ 使用 `thiserror` 定义错误类型

**日志规则：**
- ✅ 使用 `tracing` 记录日志
- ✅ 扫描启动/停止使用 `info!` 级别
- ✅ 设备发现使用 `debug!` 级别
- ✅ BLE 错误使用 `warn!` 或 `error!` 级别
- ❌ 禁止使用 `println!()`

**BLE 模块在架构中的位置：**
```
nearclip-ffi → nearclip-core → nearclip-sync → nearclip-net → nearclip-crypto
                                                    ↓
                                               nearclip-ble → nearclip-crypto
```

### Previous Story (2-5) Intelligence

**关键学习：**
- 使用平台抽象层而非第三方 BLE crate（ble-peripheral 的 crates.io 版本缺少 lib target）
- GATT UUID 已在 `gatt.rs` 中定义，可直接复用
- 使用 `tokio::sync::broadcast` 实现事件广播
- 使用 `Arc<RwLock<>>` 保护共享状态
- 实现 Drop trait 确保资源正确清理
- BLE 硬件相关测试标记为 `#[ignore]`
- 添加 `public_key_hash` 的 Base64 和长度验证

**已建立的模式：**
- 配置结构体 + 主结构体模式 (如 `BleAdvertiserConfig` + `BleAdvertiser`)
- `start()`/`stop()`/`is_xxx()` 方法签名
- 使用 `#[instrument]` 属性进行函数追踪
- 平台不支持时返回 `BleError::PlatformNotSupported`

**已有代码可复用：**
- `NEARCLIP_SERVICE_UUID` - 用于过滤设备
- `DEVICE_ID_CHARACTERISTIC_UUID` - 读取设备 ID
- `PUBKEY_HASH_CHARACTERISTIC_UUID` - 读取公钥哈希
- `BleError` - 错误类型
- `PUBKEY_HASH_LENGTH` - 验证公钥哈希长度

### BLE Central vs Peripheral 模式

**Story 2-5 (Peripheral 模式):** 设备作为外设广播自己
**Story 2-6 (Central 模式):** 设备作为中心扫描其他设备

```
┌─────────────────┐         ┌─────────────────┐
│   Device A      │         │   Device B      │
│  (Peripheral)   │ ◀─────▶ │   (Central)     │
│ - BleAdvertiser │   BLE   │ - BleScanner    │
│ - 广播 Service  │         │ - 扫描 Service  │
└─────────────────┘         └─────────────────┘
```

### 设计决策

**BleScannerConfig 结构：**
```rust
/// BLE 扫描配置
#[derive(Debug, Clone)]
pub struct BleScannerConfig {
    /// 扫描超时时间（毫秒，0 表示无限）
    pub scan_timeout_ms: u64,
    /// 设备超时时间（毫秒，超过此时间未收到广播则移除）
    pub device_timeout_ms: u64,
    /// 是否只扫描 NearClip 设备
    pub filter_nearclip_only: bool,
}

impl BleScannerConfig {
    pub fn new() -> Self;
    pub fn with_timeout(self, timeout_ms: u64) -> Self;
    pub fn with_device_timeout(self, timeout_ms: u64) -> Self;
    pub fn validate(&self) -> Result<(), BleError>;
}

impl Default for BleScannerConfig {
    fn default() -> Self {
        Self {
            scan_timeout_ms: 0,        // 无限扫描
            device_timeout_ms: 30000,  // 30秒设备超时
            filter_nearclip_only: true,
        }
    }
}
```

**DiscoveredDevice 结构：**
```rust
/// 发现的 BLE 设备
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// 设备 ID（从 GATT 特征读取）
    pub device_id: String,
    /// 公钥哈希（从 GATT 特征读取）
    pub public_key_hash: String,
    /// 信号强度 (dBm)
    pub rssi: i16,
    /// 广播名称
    pub advertise_name: Option<String>,
    /// 最后发现时间
    pub last_seen: std::time::Instant,
    /// 平台特定标识符（用于后续连接）
    pub platform_identifier: String,
}

impl DiscoveredDevice {
    pub fn from_advertisement(/* platform specific */) -> Result<Self, BleError>;
    pub fn is_expired(&self, timeout_ms: u64) -> bool;
}
```

**BleScanner 结构：**
```rust
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

/// BLE 设备扫描器
///
/// # 设计说明
///
/// 此结构体故意不实现 `Clone`，因为：
/// - BLE 扫描是独占硬件资源的操作
/// - 每个设备应只有一个活跃的扫描器实例
/// - 如需跨任务共享，应使用 `Arc<Mutex<BleScanner>>`
#[derive(Debug)]
pub struct BleScanner {
    config: BleScannerConfig,
    is_scanning: Arc<RwLock<bool>>,
    discovered_devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
    event_sender: broadcast::Sender<DiscoveredDevice>,
}

impl BleScanner {
    /// 创建新的扫描器
    pub async fn new(config: BleScannerConfig) -> Result<Self, BleError>;

    /// 开始扫描
    pub async fn start(&mut self) -> Result<(), BleError>;

    /// 停止扫描
    pub async fn stop(&mut self) -> Result<(), BleError>;

    /// 是否正在扫描
    pub async fn is_scanning(&self) -> bool;

    /// 订阅设备发现事件
    pub fn subscribe(&self) -> broadcast::Receiver<DiscoveredDevice>;

    /// 获取当前发现的设备列表
    pub async fn discovered_devices(&self) -> Vec<DiscoveredDevice>;

    /// 获取配置
    pub fn config(&self) -> &BleScannerConfig;
}
```

**与 MdnsDiscovery 对比：**
| 方面 | MdnsDiscovery | BleScanner |
|------|---------------|------------|
| 发现方式 | mDNS 查询 | BLE 广播扫描 |
| 事件通道 | `broadcast::channel` | `broadcast::channel` |
| 设备超时 | 有 | 有 |
| 过滤机制 | 服务类型 | Service UUID |

### 文件位置

**目标文件：**
- `crates/nearclip-ble/src/central.rs` - BLE 中心模式扫描实现（新建）
- `crates/nearclip-ble/src/lib.rs` - 模块导出（修改）
- `crates/nearclip-ble/tests/ble_scan_integration.rs` - 集成测试（新建）

**参考文件：**
- `crates/nearclip-ble/src/peripheral.rs` - BleAdvertiser 实现模式
- `crates/nearclip-ble/src/gatt.rs` - GATT UUID 常量
- `crates/nearclip-ble/src/error.rs` - BleError 定义
- `crates/nearclip-net/src/mdns/discovery.rs` - MdnsDiscovery 实现模式

### 依赖说明

**已有依赖（无需添加）：**
```toml
[dependencies]
thiserror.workspace = true
tracing.workspace = true
tokio.workspace = true
uuid = "1.0"
base64 = "0.22"

[dev-dependencies]
tokio = { workspace = true, features = ["rt-multi-thread", "macros"] }
```

**需要添加的标准库导入：**
```rust
use std::collections::HashMap;
use std::time::Instant;
```

### 代码模板

**central.rs:**
```rust
//! BLE 中心模式扫描
//!
//! 实现 BLE central 模式，用于扫描发现附近的 NearClip 设备。
//!
//! # Architecture
//!
//! ```text
//! BleScanner
//! ├── start()     - 启动扫描
//! ├── stop()      - 停止扫描
//! ├── subscribe() - 订阅设备发现事件
//! └── discovered_devices() - 获取设备列表
//! ```

use crate::error::BleError;
use crate::gatt::NEARCLIP_SERVICE_UUID;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, instrument, warn};

// ... 结构体和实现
```

### 平台实现说明

当前实现为平台抽象层，`start()` 方法返回 `BleError::PlatformNotSupported`。

**平台特定实现预留：**
```rust
#[cfg(target_os = "macos")]
async fn platform_start_scan(&self) -> Result<(), BleError> {
    // TODO: 实现 macOS CoreBluetooth CBCentralManager
    Err(BleError::PlatformNotSupported)
}

#[cfg(target_os = "linux")]
async fn platform_start_scan(&self) -> Result<(), BleError> {
    // TODO: 实现 Linux BlueZ D-Bus API
    Err(BleError::PlatformNotSupported)
}

#[cfg(target_os = "android")]
async fn platform_start_scan(&self) -> Result<(), BleError> {
    // TODO: 通过 JNI 调用 BluetoothLeScanner
    Err(BleError::PlatformNotSupported)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
async fn platform_start_scan(&self) -> Result<(), BleError> {
    Err(BleError::PlatformNotSupported)
}
```

---

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by context workflow -->

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List

- `crates/nearclip-ble/src/central.rs` - BleScanner 和 BleScannerConfig（新建）
- `crates/nearclip-ble/src/lib.rs` - 模块导出和文档（修改）
- `crates/nearclip-ble/tests/ble_scan_integration.rs` - 集成测试（新建）

---

## Implementation Checklist

在完成实现后，验证以下项目：

- [x] central.rs 文件已创建
- [x] BleScannerConfig 结构体完整
- [x] DiscoveredDevice 结构体完整
- [x] BleScanner 实现 start/stop 方法
- [x] 事件订阅机制��作正常
- [x] 设备去重逻辑正确
- [x] 所有单元测试通过
- [x] 集成测试验证扫描流程
- [x] `cargo build -p nearclip-ble` 成功
- [x] `cargo test -p nearclip-ble` 成功 (109 tests)
- [x] `cargo clippy -p nearclip-ble` 无警告

---

## Change Log

| Date | Change |
|------|--------|
| 2025-12-15 | Story created by create-story workflow |
| 2025-12-15 | Implementation completed - BleScanner, BleScannerConfig, DiscoveredDevice |

---

Story created: 2025-12-15
