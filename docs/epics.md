---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - docs/prd.md
  - docs/architecture.md
workflowType: 'epics-stories'
project_name: 'nearclip'
user_name: 'Mouse'
date: '2025-12-13'
status: 'complete'
total_epics: 6
total_stories: 47
---

# NearClip - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for NearClip, decomposing the requirements from the PRD and Architecture into implementable stories.

## Requirements Inventory

### Functional Requirements

**FR-1: 设备发现与配对**
- FR-1.1: 同一局域网设备通过 mDNS 自动发现 [P0]
- FR-1.2: 蓝牙范围内设备通过 BLE 广播发现 [P0]
- FR-1.3: 首次配对通过二维码扫描完成 [P0]
- FR-1.4: 配对信息本地持久化，重启后自动重连 [P0]
- FR-1.5: 支持同时配对多台设备（1:N 同步）[P1]

**FR-2: 剪贴板同步**
- FR-2.1: 监听本地剪贴板变化 [P0]
- FR-2.2: 检测到变化后自动同步到已配对设备 [P0]
- FR-2.3: 接收远程剪贴板内容并写入本地剪贴板 [P0]
- FR-2.4: 同步内容类型：纯文本 [P0]
- FR-2.5: 防止同步循环（标记来源，不回传）[P0]

**FR-3: 通信通道**
- FR-3.1: WiFi 优先：通过 TCP/TLS 传输数据 [P0]
- FR-3.2: BLE 备选：WiFi 不可用时自动切换 [P0]
- FR-3.3: 通道状态监测与自动切换 [P0]
- FR-3.4: 连接失败时重试 3 次 [P0]
- FR-3.5: 重试失败后用户可选择：丢弃/等待/继续重试 [P1]

**FR-4: 安全与隐私**
- FR-4.1: 所有传输使用 TLS 1.3 加密 [P0]
- FR-4.2: 配对时交换公钥，建立信任关系 [P0]
- FR-4.3: 数据仅在设备间直接传输，不经过任何服务器 [P0]
- FR-4.4: 本地存储的配对信息加密保护 [P1]

**FR-5: 平台集成**
- FR-5.1: 菜单栏常驻图标，显示连接状态 [macOS] [P0]
- FR-5.2: 无障碍服务后台运行，自动同步 [Android] [P0]
- FR-5.3: 系统托盘图标，显示连接状态 [Windows] [P0 Phase 2]
- FR-5.4: 快捷指令触发同步 [iOS] [P0 Phase 2]

**FR-6: 用户反馈**
- FR-6.1: 同步成功时显示低调的系统通知 [P1]
- FR-6.2: 连接状态实时显示（已连接/断开/同步中）[P0]
- FR-6.3: 同步失败时提示用户 [P1]

### NonFunctional Requirements

**NFR-1: 性能**
- NFR-1.1: WiFi 同步延迟 < 3 秒
- NFR-1.2: BLE 同步延迟 < 10 秒
- NFR-1.3: 同步成功率 > 95%
- NFR-1.4: 后台内存占用 < 50 MB
- NFR-1.5: CPU 空闲占用 < 1%

**NFR-2: 安装包大小**
- NFR-2.1: 桌面端安装包 < 10 MB
- NFR-2.2: 移动端安装包 < 5 MB

**NFR-3: 可靠性**
- NFR-3.1: 后台稳定运行，不被系统杀死
- NFR-3.2: 网络切换恢复，WiFi 断开/重连后自动恢复
- NFR-3.3: 设备重启恢复，重启后自动启动并重连

**NFR-4: 兼容性**
- NFR-4.1: macOS 12+ (Monterey)
- NFR-4.2: Android 10+ (API 29)
- NFR-4.3: Windows 10 1903+ [Phase 2]
- NFR-4.4: iOS 15+ [Phase 2]

**NFR-5: 可维护性**
- NFR-5.1: 模块化架构，Rust 核心可独立测试
- NFR-5.2: 日志记录，关键操作有日志
- NFR-5.3: 代码可读性，AI Coding 生成的代码需保持清晰结构

### Additional Requirements

**架构决策 (from Architecture.md):**
- 项目采用 Custom Rust Workspace 结构，需初始化 6 个 crate
- 使用 uniffi 0.28 生成 Swift/Kotlin FFI 绑定
- 异步运行时使用 tokio 1.x
- TLS 加密使用 rustls 0.23.x
- 序列化使用 MessagePack (rmp-serde)
- 日志使用 tracing 0.1.x
- 设备发现使用 mdns-sd
- 错误处理使用 thiserror + NearClipError 统一类型

**配对与安全 (from Architecture.md):**
- 二维码 + ECDH 密钥交换
- TOFU (Trust On First Use) 信任模型
- 平台密钥库存储（macOS Keychain / Android Keystore）

**通信模式 (from Architecture.md):**
- Push 模式同步（复制即推送）
- WiFi 优先，BLE 备选
- BLE 需要分片传输支持长文本
- Message 结构体统一消息格式

**实现顺序 (from Architecture.md):**
1. nearclip-crypto (加密基础)
2. nearclip-net (mDNS + TCP)
3. nearclip-ble (蓝牙通信)
4. nearclip-sync (同步逻辑)
5. nearclip-core (协调层)
6. nearclip-ffi (绑定生成)
7. macOS 客户端
8. Android 客户端

### FR Coverage Map

| FR | Epic | 说明 |
|----|------|------|
| FR-1.1 | Epic 2 | mDNS 发现 |
| FR-1.2 | Epic 2 | BLE 发现 |
| FR-1.3 | Epic 2 | 二维码配对 |
| FR-1.4 | Epic 2 | 配对持久化 |
| FR-1.5 | Epic 6 | 多设备配对 |
| FR-2.1 | Epic 3 | 剪贴板监听 |
| FR-2.2 | Epic 3 | 自动同步 |
| FR-2.3 | Epic 3 | 接收写入剪贴板 |
| FR-2.4 | Epic 3 | 纯文本同步 |
| FR-2.5 | Epic 3 | 防止同步循环 |
| FR-3.1 | Epic 3 | WiFi/TCP/TLS |
| FR-3.2 | Epic 3 | BLE 备选 |
| FR-3.3 | Epic 3 | 通道切换 |
| FR-3.4 | Epic 3 | 重试机制 |
| FR-3.5 | Epic 6 | 重试策略选择 |
| FR-4.1 | Epic 2 | TLS 1.3 |
| FR-4.2 | Epic 2 | 公钥交换 |
| FR-4.3 | Epic 2 | 本地直传 |
| FR-4.4 | Epic 6 | 加密存储 |
| FR-5.1 | Epic 4 | macOS 菜单栏 |
| FR-5.2 | Epic 5 | Android 无障碍 |
| FR-6.1 | Epic 6 | 同步通知 |
| FR-6.2 | Epic 4 & 5 | 状态显示 |
| FR-6.3 | Epic 6 | 失败提示 |

## Epic List

### Epic 1: 项目基础设施
**用户价值：** 开发环境就绪，Rust 核心库可编译测试

**目标：** 初始化 Rust workspace，创建 6 个 crate 结构，建立统一的错误处理、日志和消息协议基础。

**覆盖需求：** 架构附加需求（Rust Workspace）、NFR-5.1、NFR-5.2

---

### Epic 2: 设备发现与安全配对
**用户价值：** 用户可以发现附近设备并安全配对

**目标：** 实现 mDNS 和 BLE 设备发现，二维码配对流程，ECDH 密钥交换，建立设备间信任关系。

**覆盖需求：** FR-1.1, FR-1.2, FR-1.3, FR-1.4, FR-4.1, FR-4.2, FR-4.3

---

### Epic 3: 跨设备剪贴板同步
**用户价值：** 用户在一台设备复制，另一台设备可粘贴

**目标：** 实现 WiFi (TCP/TLS) 和 BLE 双通道剪贴板同步，包括剪贴板监听、消息协议、通道切换、重试机制。

**覆盖需求：** FR-2.1, FR-2.2, FR-2.3, FR-2.4, FR-2.5, FR-3.1, FR-3.2, FR-3.3, FR-3.4, NFR-1.1, NFR-1.2

---

### Epic 4: macOS 原生客户端
**用户价值：** macOS 用户获得原生菜单栏体验

**目标：** 实现 Swift/SwiftUI 菜单栏应用，集成 Rust FFI，剪贴板监听，配对界面，设置界面。

**覆盖需求：** FR-5.1, FR-6.2 (macOS), NFR-4.1

---

### Epic 5: Android 原生客户端
**用户价值：** Android 用户获得无缝后台同步体验

**目标：** 实现 Kotlin/Compose 应用，无障碍服务，前台服务，集成 Rust FFI，配对和设置界面。

**覆盖需求：** FR-5.2, FR-6.2 (Android), NFR-4.2

---

### Epic 6: 增强用户体验
**用户价值：** 用户获得完善的通知反馈和多设备支持

**目标：** 实现同步通知、错误提示、多设备配对、失败重试策略选择、配对信息加密存储。

**覆盖需求：** FR-1.5, FR-3.5, FR-4.4, FR-6.1, FR-6.3

---

## Epic 1: 项目基础设施

**目标：** 初始化 Rust workspace，创建 6 个 crate 结构，建立统一的错误处理、日志和消息协议基础。

### Story 1.1: 初始化 Rust Workspace

As a 开发者,
I want 创建完整的 Rust workspace 和 crate 结构,
So that 可以开始在正确的项目结构中进行开发.

**Acceptance Criteria:**

**Given** 空的项目目录
**When** 执行项目初始化脚本
**Then** 创建包含 6 个 crate 的 Rust workspace
**And** Cargo.toml 正确配置 workspace members
**And** 每个 crate 有独立的 Cargo.toml 和 src/lib.rs
**And** `cargo build` 成功编译所有 crate

---

### Story 1.2: 实现统一错误类型

As a 开发者,
I want 使用统一的 NearClipError 错误类型,
So that 所有模块的错误处理保持一致.

**Acceptance Criteria:**

**Given** nearclip-core crate 已创建
**When** 定义 NearClipError 枚举
**Then** 包含 Network、Bluetooth、Crypto、DeviceNotFound 等变体
**And** 使用 thiserror 派生 Error trait
**And** 所有公开函数返回 Result<T, NearClipError>
**And** 单元测试验证错误类型正确构造

---

### Story 1.3: 配置结构化日志系统

As a 开发者,
I want 使用 tracing 进行结构化日志记录,
So that 可以方便地调试和追踪问题.

**Acceptance Criteria:**

**Given** nearclip-core crate 已创建
**When** 集成 tracing 日志框架
**Then** 支持 error/warn/info/debug/trace 五个级别
**And** 日志包含时间戳和模块来源
**And** 提供初始化函数供各平台调用
**And** 测试验证日志输出格式正确

---

### Story 1.4: 定义消息协议结构

As a 开发者,
I want 定义统一的 Message 消息结构,
So that 所有网络通信使用一致的格式.

**Acceptance Criteria:**

**Given** nearclip-sync crate 已创建
**When** 定义 Message 和 MessageType 类型
**Then** Message 包含 msg_type、payload、timestamp、device_id 字段
**And** MessageType 包含 ClipboardSync、PairingRequest、PairingResponse、Heartbeat、Ack
**And** 使用 rmp-serde 进行 MessagePack 序列化
**And** 单元测试验证序列化/反序列化正确

---

## Epic 2: 设备发现与安全配对

**目标：** 实现 mDNS 和 BLE 设备发现，二维码配对流程，ECDH 密钥交换，建立设备间信任关系。

### Story 2.1: 实现 ECDH 密钥对生成

As a 用户,
I want 设备能生成安全的密钥对,
So that 可以与其他设备建立加密通信.

**Acceptance Criteria:**

**Given** nearclip-crypto crate 已创建
**When** 调用密钥生成函数
**Then** 生成 ECDH P-256 密钥对
**And** 私钥可安全存储
**And** 公钥可导出为字节数组
**And** 单元测试验证密钥对有效性

---

### Story 2.2: 实现 TLS 1.3 配置

As a 用户,
I want 所有通信使用 TLS 1.3 加密,
So that 剪贴板内容传输安全.

**Acceptance Criteria:**

**Given** ECDH 密钥对已生成
**When** 配置 rustls TLS 连接
**Then** 使用 TLS 1.3 协议
**And** 使用自签名证书（基于 ECDH 密钥）
**And** 客户端和服务端配置正确
**And** 集成测试验证加密连接建立

---

### Story 2.3: 实现 mDNS 服务广播

As a 用户,
I want 设备在局域网广播自己的存在,
So that 其他设备可以发现我.

**Acceptance Criteria:**

**Given** nearclip-net crate 已创建
**When** 启动 mDNS 服务广播
**Then** 使用 _nearclip._tcp.local 服务类型
**And** 广播包含设备 ID 和公钥哈希
**And** 可以停止广播
**And** 测试验证服务注册成功

---

### Story 2.4: 实现 mDNS 设备发现

As a 用户,
I want 发现局域网内的其他 NearClip 设备,
So that 可以选择配对目标.

**Acceptance Criteria:**

**Given** mDNS 服务广播已实现
**When** 启动设备发现扫描
**Then** 返回发现的设备列表（设备 ID、IP、端口）
**And** 持续监听新设备上线
**And** 检测设备离线
**And** 集成测试验证发现流程

---

### Story 2.5: 实现 BLE 设备广播

As a 用户,
I want 设备通过蓝牙广播自己,
So that 无 WiFi 时也能被发现.

**Acceptance Criteria:**

**Given** nearclip-ble crate 已创建
**When** 启动 BLE 外设模式广播
**Then** 使用 NearClip 专用 Service UUID
**And** 广播包含设备 ID 特征
**And** 可以停止广播
**And** 测试验证广播数据正确

---

### Story 2.6: 实现 BLE 设备扫描

As a 用户,
I want 扫描附近的 NearClip 蓝牙设备,
So that 可以在无 WiFi 环境下配对.

**Acceptance Criteria:**

**Given** BLE 广播已实现
**When** 启动 BLE 中心模式扫描
**Then** 过滤 NearClip Service UUID
**And** 返回发现的设备列表
**And** 可以停止扫描
**And** 测试验证扫描结果正确

---

### Story 2.7: 生成配对二维码

As a 用户,
I want 生成包含配对信息的二维码,
So that 其他设备可以扫描配对.

**Acceptance Criteria:**

**Given** 设备公钥已生成
**When** 生成配对二维码
**Then** 二维码包含设备 ID、公钥、连接信息
**And** 使用 JSON 格式编码
**And** 返回二维码图片数据（PNG）
**And** 测试验证二维码可解析

---

### Story 2.8: 扫描并解析配对二维码

As a 用户,
I want 扫描其他设备的二维码完成配对,
So that 两台设备建立信任关系.

**Acceptance Criteria:**

**Given** 目标设备显示配对二维码
**When** 扫描并解析二维码内容
**Then** 提取设备 ID、公钥、连接信息
**And** 通过 ECDH 完成密钥协商
**And** 双方确认配对成功
**And** 测试验证完整配对流程

---

### Story 2.9: 持久化配对设备信息

As a 用户,
I want 配对信息保存到本地,
So that 重启后自动重连无需重新配对.

**Acceptance Criteria:**

**Given** 设备配对成功
**When** 保存配对信息
**Then** 存储设备 ID、公钥、连接偏好
**And** 使用平台安全存储（预留接口）
**And** 支持读取、更新、删除操作
**And** 测试验证持久化正确

---

## Epic 3: 跨设备剪贴板同步

**目标：** 实现 WiFi (TCP/TLS) 和 BLE 双通道剪贴板同步，包括剪贴板监听、消息协议、通道切换、重试机制。

### Story 3.1: 实现 TCP 服务端

As a 用户,
I want 设备能接收其他设备的连接,
So that 可以接收剪贴板内容.

**Acceptance Criteria:**

**Given** TLS 配置已完成
**When** 启动 TCP 服务端
**Then** 监听指定端口（动态分配）
**And** 使用 TLS 加密连接
**And** 支持多个并发连接
**And** 集成测试验证连接建立

---

### Story 3.2: 实现 TCP 客户端连接

As a 用户,
I want 设备能连接到其他设备,
So that 可以发送剪贴板内容.

**Acceptance Criteria:**

**Given** 已知目标设备 IP 和端口
**When** 建立 TCP 客户端连接
**Then** 完成 TLS 握手
**And** 验证目标设备身份（公钥匹配）
**And** 连接失败时返回错误
**And** 测试验证双向通信

---

### Story 3.3: 实现 BLE 数据传输（外设端）

As a 用户,
I want 通过蓝牙接收数据,
So that 无 WiFi 时也能同步.

**Acceptance Criteria:**

**Given** BLE 广播已启动
**When** 中心设备连接并写入数据
**Then** 接收完整的消息数据
**And** 支持分片重组（MTU 限制）
**And** 触发消息处理回调
**And** 测试验证数据完整性

---

### Story 3.4: 实现 BLE 数据传输（中心端）

As a 用户,
I want 通过蓝牙发送数据,
So that 无 WiFi 时也能同步.

**Acceptance Criteria:**

**Given** 已发现目标 BLE 设备
**When** 连接并发送数据
**Then** 自动分片发送（MTU 限制）
**And** 等待确认响应
**And** 超时重试
**And** 测试验证发送成功

---

### Story 3.5: 实现剪贴板内容发送

As a 用户,
I want 复制内容后自动发送到其他设备,
So that 实现无感同步.

**Acceptance Criteria:**

**Given** 已配对且连接的设备
**When** 调用发送剪贴板函数
**Then** 构建 ClipboardSync 消息
**And** 选择可用通道（WiFi 优先）
**And** 发送并等待 ACK
**And** 测试验证消息到达

---

### Story 3.6: 实现剪贴板内容接收

As a 用户,
I want 收到其他设备的剪贴板内容,
So that 可以直接粘贴.

**Acceptance Criteria:**

**Given** 监听已启动
**When** 收到 ClipboardSync 消息
**Then** 解析消息内容
**And** 通过回调通知上层
**And** 发送 ACK 确认
**And** 测试验证接收流程

---

### Story 3.7: 实现通道状态监测

As a 用户,
I want 系统自动检测通道可用性,
So that 选择最佳通道同步.

**Acceptance Criteria:**

**Given** WiFi 和 BLE 通道均已初始化
**When** 持续监测通道状态
**Then** 检测 WiFi 连接断开
**And** 检测 BLE 连接断开
**And** 触发状态变更回调
**And** 测试验证状态检测准确

---

### Story 3.8: 实现通道自动切换

As a 用户,
I want WiFi 断开时自动切换到蓝牙,
So that 同步不中断.

**Acceptance Criteria:**

**Given** WiFi 通道不可用
**When** 需要发送剪贴板
**Then** 自动选择 BLE 通道
**And** WiFi 恢复后切回
**And** 切换过程对用户透明
**And** 测试验证切换逻辑

---

### Story 3.9: 实现发送重试机制

As a 用户,
I want 发送失败时自动重试,
So that 提高同步成功率.

**Acceptance Criteria:**

**Given** 发送失败（超时或错误）
**When** 触发重试逻辑
**Then** 最多重试 3 次
**And** 重试间隔 2 秒
**And** 3 次失败后返回错误
**And** 测试验证重试行为

---

### Story 3.10: 实现同步循环防护

As a 用户,
I want 系统防止剪贴板内容回传,
So that 不会无限循环同步.

**Acceptance Criteria:**

**Given** 收到远程剪贴板内容
**When** 写入本地剪贴板
**Then** 标记内容来源为远程
**And** 本地剪贴板变化检测忽略该内容
**And** 不会重新发送到源设备
**And** 测试验证防循环有效

---

### Story 3.11: 实现核心协调层

As a 用户,
I want 统一的接口管理所有同步功能,
So that 平台客户端调用简单.

**Acceptance Criteria:**

**Given** 所有底层模块已实现
**When** 初始化 NearClipManager
**Then** 提供 start/stop/sync 等高层接口
**And** 协调 net、ble、crypto、sync 模块
**And** 管理设备列表和连接状态
**And** 测试验证完整工作流

---

## Epic 4: macOS 原生客户端

**目标：** 实现 Swift/SwiftUI 菜单栏应用，集成 Rust FFI，剪贴板监听，配对界面，设置界面。

### Story 4.1: 生成 uniffi Swift 绑定

As a macOS 用户,
I want Swift 可以调用 Rust 核心库,
So that 应用能使用同步功能.

**Acceptance Criteria:**

**Given** nearclip-ffi crate 已实现
**When** 运行 uniffi-bindgen
**Then** 生成 Swift 绑定代码
**And** 生成 XCFramework
**And** 可在 Xcode 项目中导入
**And** 测试验证基本调用成功

---

### Story 4.2: 创建 macOS 菜单栏应用

As a macOS 用户,
I want 应用作为菜单栏常驻,
So that 不占用 Dock 空间且随时可用.

**Acceptance Criteria:**

**Given** Xcode 项目已创建
**When** 配置为菜单栏应用
**Then** 应用图标显示在菜单栏
**And** 点击图标显示下拉菜单
**And** 应用不显示在 Dock
**And** 支持开机自启动配置

---

### Story 4.3: 实现连接状态显示

As a macOS 用户,
I want 看到当前连接状态,
So that 知道同步是否可用.

**Acceptance Criteria:**

**Given** 菜单栏应用已运行
**When** 查看菜单栏图标或下拉菜单
**Then** 显示当前连接状态（已连接/断开/同步中）
**And** 图标颜色反映状态
**And** 显示已连接设备列表
**And** 状态实时更新

---

### Story 4.4: 实现剪贴板监听

As a macOS 用户,
I want 系统自动监听剪贴板变化,
So that 复制后自动同步.

**Acceptance Criteria:**

**Given** 应用已启动
**When** 用户复制内容到剪贴板
**Then** 检测到剪贴板变化
**And** 调用 Rust 核心发送内容
**And** 忽略远程同步写入的内容
**And** 测试验证监听准确

---

### Story 4.5: 实现剪贴板写入

As a macOS 用户,
I want 收到同步内容后自动写入剪贴板,
So that 可以直接粘贴.

**Acceptance Criteria:**

**Given** 收到 Rust 核心的回调
**When** 有新的剪贴板内容到达
**Then** 写入系统剪贴板
**And** 标记为远程来源（防循环）
**And** 可选显示通知
**And** 测试验证写入成功

---

### Story 4.6: 实现配对界面

As a macOS 用户,
I want 通过界面完成设备配对,
So that 可以开始使用同步功能.

**Acceptance Criteria:**

**Given** 点击"添加设备"
**When** 打开配对窗口
**Then** 显示本机二维码供扫描
**And** 或提供扫描其他设备二维码选项
**And** 配对成功后显示确认
**And** 配对失败显示错误信息

---

### Story 4.7: 实现设置界面

As a macOS 用户,
I want 配置应用行为,
So that 按我的偏好工作.

**Acceptance Criteria:**

**Given** 点击"设置"
**When** 打开设置窗口
**Then** 可配置开机自启动
**And** 可查看/删除已配对设备
**And** 可选择通道偏好（WiFi/BLE/自动）
**And** 设置保存并生效

---

### Story 4.8: 实现 Keychain 密钥存储

As a macOS 用户,
I want 密钥安全存储在 Keychain,
So that 配对信息受系统保护.

**Acceptance Criteria:**

**Given** 设备配对成功
**When** 存储配对密钥
**Then** 使用 macOS Keychain API
**And** 密钥与应用绑定
**And** 支持读取和删除
**And** 测试验证 Keychain 操作

---

## Epic 5: Android 原生客户端

**目标：** 实现 Kotlin/Compose 应用，无障碍服务，前台服务，集成 Rust FFI，配对和设置界面。

### Story 5.1: 生成 uniffi Kotlin 绑定

As a Android 用户,
I want Kotlin 可以调用 Rust 核心库,
So that 应用能使用同步功能.

**Acceptance Criteria:**

**Given** nearclip-ffi crate 已实现
**When** 运行 uniffi-bindgen
**Then** 生成 Kotlin 绑定代码
**And** 生成 Android .so 库文件
**And** 可在 Android 项目中导入
**And** 测试验证基本调用成功

---

### Story 5.2: 创建 Android 应用框架

As a Android 用户,
I want 基础应用结构就绪,
So that 可以构建完整功能.

**Acceptance Criteria:**

**Given** Android Studio 项目已创建
**When** 配置项目结构
**Then** 使用 Kotlin + Jetpack Compose
**And** 集成 Rust .so 库
**And** 配置必要权限（蓝牙、网络）
**And** 应用可以编译运行

---

### Story 5.3: 实现前台服务

As a Android 用户,
I want 应用在后台持续运行,
So that 随时可以接收同步.

**Acceptance Criteria:**

**Given** 应用已启动
**When** 启动前台服务
**Then** 显示持久通知
**And** 通知显示连接状态
**And** 服务不被系统杀死
**And** 可从通知快速访问应用

---

### Story 5.4: 实现无障碍服务剪贴板监听

As a Android 用户,
I want 自动监听剪贴板变化,
So that 复制后自动同步.

**Acceptance Criteria:**

**Given** 用户授权无障碍服务
**When** 用户复制内容
**Then** 无障碍服务检测到变化
**And** 调用 Rust 核心发送内容
**And** 忽略远程同步写入的内容
**And** 测试验证监听准确

---

### Story 5.5: 实现剪贴板写入

As a Android 用户,
I want 收到同步内容后自动写入剪贴板,
So that 可以直接粘贴.

**Acceptance Criteria:**

**Given** 收到 Rust 核心的回调
**When** 有新的剪贴板内容到达
**Then** 写入系统剪贴板
**And** 标记为远程来源（防循环）
**And** 可选显示通知
**And** 测试验证写入成功

---

### Story 5.6: 实现连接状态显示

As a Android 用户,
I want 看到当前连接状态,
So that 知道同步是否可用.

**Acceptance Criteria:**

**Given** 应用或前台通知可见
**When** 查看状态
**Then** 显示当前连接状态
**And** 通知图标反映状态
**And** 显示已连接设备
**And** 状态实时更新

---

### Story 5.7: 实现配对界面

As a Android 用户,
I want 通过界面完成设备配对,
So that 可以开始使用同步功能.

**Acceptance Criteria:**

**Given** 打开应用主界面
**When** 点击"添加设备"
**Then** 显示本机二维码供扫描
**And** 或打开相机扫描其他设备二维码
**And** 配对成功后显示确认
**And** 配对失败显示错误信息

---

### Story 5.8: 实现设置界面

As a Android 用户,
I want 配置应用行为,
So that 按我的偏好工作.

**Acceptance Criteria:**

**Given** 打开设置页面
**When** 调整设置
**Then** 可启用/禁用无障碍服务引导
**And** 可查看/删除已配对设备
**And** 可选择通道偏好
**And** 设置保存并生效

---

### Story 5.9: 实现 Android Keystore 密钥存储

As a Android 用户,
I want 密钥安全存储在 Keystore,
So that 配对信息受系统保护.

**Acceptance Criteria:**

**Given** 设备配对成功
**When** 存储配对密钥
**Then** 使用 Android Keystore API
**And** 密钥与应用绑定
**And** 支持读取和删除
**And** 测试验证 Keystore 操作

---

## Epic 6: 增强用户体验

**目标：** 实现同步通知、错误提示、多设备配对、失败重试策略选择、配对信息加密存储。

### Story 6.1: 实现同步成功通知

As a 用户,
I want 同步成功时收到低调提示,
So that 知道内容已到达.

**Acceptance Criteria:**

**Given** 剪贴板内容同步成功
**When** 目标设备收到内容
**Then** 显示系统通知（可配置）
**And** 通知内容简洁（如"已同步: 来自 Mac"）
**And** 通知自动消失（3秒）
**And** 可在设置中关闭通知

---

### Story 6.2: 实现同步失败提示

As a 用户,
I want 同步失败时被告知,
So that 可以采取补救措施.

**Acceptance Criteria:**

**Given** 剪贴板同步失败（重试耗尽）
**When** 系统检测到失败
**Then** 显示失败通知
**And** 通知包含失败原因
**And** 提供重试选项
**And** 可点击查看详情

---

### Story 6.3: 实现多设备配对管理

As a 用户,
I want 同时配对多台设备,
So that 内容可以同步到所有设备.

**Acceptance Criteria:**

**Given** 已配对至少一台设备
**When** 添加新设备配对
**Then** 新设备加入已配对列表
**And** 剪贴板同步到所有已配对设备
**And** 可独立管理每个设备（删除、暂停）
**And** 最多支持 5 台设备

---

### Story 6.4: 实现重试策略选择

As a 用户,
I want 选择同步失败后的处理方式,
So that 按我的偏好处理.

**Acceptance Criteria:**

**Given** 同步重试 3 次失败
**When** 显示策略选择
**Then** 可选择"丢弃"（放弃本次）
**And** 可选择"等待"（设备上线后发送）
**And** 可选择"继续重试"
**And** 可设置默认策略

---

### Story 6.5: 实现配对信息加密存储

As a 用户,
I want 配对信息加密存储,
So that 即使设备丢失也安全.

**Acceptance Criteria:**

**Given** 配对信息需要存储
**When** 写入本地存储
**Then** 使用平台密钥库加密
**And** 非授权应用无法读取
**And** 设备锁定���数据受保护
**And** 测试验证加密有效

---

### Story 6.6: 实现网络恢复自动重连

As a 用户,
I want 网络恢复后自动重连,
So that 无需手动干预.

**Acceptance Criteria:**

**Given** 网络断开导致连接中断
**When** 网络恢复
**Then** 自动尝试重新连接
**And** 重连成功恢复同步
**And** 重连过程对用户透明
**And** 多次重连失败后通知用户

