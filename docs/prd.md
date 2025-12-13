---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
status: completed
inputDocuments:
  - docs/analysis/product-brief-nearclip-2025-12-13.md
  - docs/analysis/research/technical-clipboard-sync-research-2025-12-12.md
documentCounts:
  briefs: 1
  research: 1
  brainstorming: 0
  projectDocs: 0
workflowType: 'prd'
lastStep: 10
project_name: 'nearclip'
user_name: 'Mouse'
date: '2025-12-13'
---

# Product Requirements Document - NearClip

**Author:** Mouse
**Date:** 2025-12-13

---

## Executive Summary

NearClip 是一款轻量级、跨平台的本地剪贴板同步工具，通过 WiFi 和蓝牙实现设备间的无缝内容传输。无需云服务、无需公网服务器——只要设备在通信范围内，即可实现剪贴板内容的即时同步。

**核心价值主张：** 一台设备上复制，其他设备直接粘贴，无需思考「怎么传」。

**目标用户：** 跨平台多设备用户（Mac + Windows + Android 组合），对效率敏感，不愿为简单的复制粘贴花费额外时间。

**技术架构：** Rust 核心库 + 原生 UI，追求极致的轻量与高效。

### What Makes This Special

| 差异化 | 说明 |
|--------|------|
| **零云依赖** | 完全本地化，无需注册、无需服务器，数据永不离开本地网络 |
| **极致轻量** | Rust 核心 + 原生开发，桌面端 < 10MB，移动端 < 5MB |
| **蓝牙兜底** | WiFi 不可用时自动切换 BLE，覆盖更多使用场景 |
| **真正跨平台** | macOS、Windows、Android、iOS 全覆盖，体验一致 |
| **隐私至上** | 端到端 TLS 1.3 加密，数据只在设备间直接传输 |

## Project Classification

**Technical Type:** 跨平台桌面+移动应用
**Domain:** 通用工具/效率类
**Complexity:** 中等
**Project Context:** Greenfield - 全新项目

**技术特征：**
- 核心语言：Rust（跨平台逻辑）
- UI 层：Swift (macOS/iOS)、Kotlin (Android)、C#/Rust (Windows)
- 网络协议：mDNS 设备发现、TCP/TLS 数据传输、BLE 备选通道

---

## Success Criteria

### User Success

**核心成功场景：**

| 场景 | 成功标准 |
|------|---------|
| Mac → Android | 在 Mac 复制链接，解锁 Android 后直接可粘贴 |
| Android → Mac | 在 Android 复制验证码，Mac 上直接可粘贴 |
| 任意设备 → 任意设备 | 复制后，目标设备解锁即可粘贴，无需额外操作 |

**用户体验目标：**
- 「无感同步」- 用户不需要思考同步过程
- 「解锁即用」- 目标设备解锁后剪贴板已就绪
- 「零学习成本」- 首次配对后无需任何操作

### Business Success

个人项目，无商业化目标。

**成功定义：** 自己日常使用顺手，解决跨设备复制粘贴的痛点。

### Technical Success

| 指标 | 目标 |
|------|------|
| WiFi 同步延迟 | < 3 秒 |
| BLE 同步延迟 | < 10 秒（可接受） |
| 同步成功率 | > 95% |
| 桌面端安装包 | < 10 MB |
| 移动端安装包 | < 5 MB |
| 后台内存占用 | < 50 MB |

### Measurable Outcomes

- ✅ Mac ↔ Android 通过 WiFi 成功同步文本
- ✅ Mac ↔ Android 通过蓝牙成功同步文本
- ✅ 日常使用稳定可靠，无需手动干预

---

## Product Scope

### MVP - Minimum Viable Product

**Phase 1: Mac + Android**
- WiFi 局域网同步（mDNS 发现）
- BLE 蓝牙同步（备选通道）
- 文本内容同步
- 端到端 TLS 加密
- 设备配对流程

**平台实现：**
- macOS: 原生 Swift 应用，剪贴板监听
- Android: Kotlin 应用，无障碍服务自动同步

### Growth Features (Post-MVP)

**Phase 2: 平台扩展**
- Windows 客户端
- iOS 客户端（快捷指令触发）

**Phase 3: 功能增强**
- 剪贴板历史（PC 端）
- 图片同步支持

### Vision (Future)

- 文件传输
- Linux 支持
- 富文本支持
- 更多自动化集成

---

## User Journeys

### Journey 1: 小明的验证码困境

小明正在 Mac 上登录某个网站，需要输入手机收到的验证码。以前，他得解锁手机，打开短信，记住 6 位数字，然后在电脑上手动输入——经常输错，又要重新获取。

现在，小明的手机收到验证码短信，他习惯性地长按复制。几乎同时，他的 Mac 右上角闪过一个低调的提示「已同步」。他直接在网页上 Cmd+V，验证码就出现了。整个过程不到 3 秒，他甚至没有意识到自己「传输」了什么——这就是他想要的体验。

**关键时刻：** 粘贴成功的那一刻，小明意识到他再也不用在两个设备间来回切换了。

---

### Journey 2: 小明的链接分享

小明在 Mac 上浏览技术文章，发现一篇很好的教程想在地铁上用手机继续阅读。以前他需要打开微信，发给「文件传输助手」，再在手机上打开微信复制链接。

现在，他直接 Cmd+C 复制链接。下班路上，他掏出手机解锁，打开浏览器，长按粘贴——链接已经在那里了。他甚至不记得自己什么时候「发送」过这个链接，因为根本不需要任何发送操作。

**关键时刻：** 第一次在手机上「凭空」粘贴出电脑上的内容时，小明感到一种魔法般的便捷。

---

### Journey 3: 小明的首次配对

小明刚下载安装了 NearClip，Mac 版和 Android 版各装了一个。打开 Mac 应用后，界面简洁——只有一个二维码和「等待配对」的提示。他打开 Android 版，扫描二维码，两边同时显示「配对成功」。

从此以后，只要两台设备在同一网络或蓝牙范围内，剪贴板就是通的。没有账号注册，没有云端同步，没有隐私担忧。小明喜欢这种「一次配对，永久生效」的简单。

**关键时刻：** 配对成功后第一次同步成功，小明确认「就是这么简单」。

---

### Journey Requirements Summary

| 旅程 | 揭示的功能需求 |
|------|---------------|
| 验证码同步 | 剪贴板实时监听、快速同步、低调通知 |
| 链接分享 | 后台常驻、解锁即用、历史保持 |
| 首次配对 | 二维码配对、设备发现、配对状态持久化 |

**核心能力清单：**
- 剪贴板实时监听（Mac 原生、Android 无障碍服务）
- 设备发现与配对（mDNS + 二维码）
- 数据同步（WiFi 优先、BLE 备选）
- 状态通知（低调、非侵入式）
- 配对信息持久化（一次配对，长期有效）

---

## Cross-Platform App Specific Requirements

### Project-Type Overview

NearClip 采用 **Rust 核心库 + 原生 UI** 架构，在各平台实现一致的核心功能，同时保持原生体验。

### Technical Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Rust Core Library                  │
│  ┌─────────────────────────────────────────────┐    │
│  │  网络发现 │ 数据同步 │ 加密 │ 设备管理      │    │
│  └─────────────────────────────────────────────┘    │
└───────────────────────┬─────────────────────────────┘
                        │ FFI
        ┌───────────────┼───────────────┐
        │               │               │
   ┌────▼────┐    ┌────▼────┐    ┌────▼────┐
   │  macOS  │    │ Android │    │ Windows │
   │  Swift  │    │ Kotlin  │    │  C#/WinUI│
   │ 菜单栏   │    │ 无障碍   │    │ 系统托盘 │
   └─────────┘    └─────────┘    └─────────┘
```

### Platform-Specific Implementation

| 平台 | UI 框架 | 系统集成 | 剪贴板访问 |
|------|---------|---------|-----------|
| **macOS** | SwiftUI | 菜单栏常驻 | NSPasteboard 监听 |
| **Android** | Kotlin/Compose | 无障碍服务 | AccessibilityService |
| **Windows** | WinUI 3 / C# | 系统托盘 | Clipboard Listener |
| **iOS** (Phase 2) | SwiftUI | 快捷指令 | UIPasteboard (手动触发) |

### Connection Failure Handling

当无法连接到目标设备时，提供用户可配置的策略：

| 策略 | 行为 | 适用场景 |
|------|------|---------|
| **重试** (默认) | 自动重试 3 次，间隔 2 秒 | 临时网络波动 |
| **等待** | 保持队列，设备上线后自动同步 | 用户期望稳定送达 |
| **丢弃** | 放弃本次同步，不阻塞后续 | 用户偏好实时性 |

**默认行为：** 重试 3 次后询问用户选择（等待/丢弃）

### Rust Core Modules

| 模块 | 职责 |
|------|------|
| `nearclip-core` | 核心协调、配置管理 |
| `nearclip-net` | mDNS 发现、TCP/TLS 传输 |
| `nearclip-ble` | BLE 设备发现与数据传输 |
| `nearclip-crypto` | TLS 1.3、密钥管理 |
| `nearclip-sync` | 同步逻辑、冲突处理 |

### FFI Bindings

| 平台 | 绑定方式 |
|------|---------|
| macOS/iOS | Swift Package + C FFI |
| Android | JNI via `uniffi` 或 `jni-rs` |
| Windows | C# P/Invoke 或 `windows-rs` |

### Implementation Considerations

**暂不考虑：**
- 应用内自动更新
- Linux 支持
- 图片/文件传输

**Phase 1 聚焦：**
- Mac + Android 核心功能
- WiFi + BLE 双通道
- 文本同步

---

## Project Scoping & Phased Development

### Technical Risk Assessment

| 风险领域 | 风险等级 | 说明 | 缓解措施 |
|---------|---------|------|---------|
| **Rust 核心开发** | 高 | 开发者不熟悉 Rust | AI Coding 辅助 |
| **Swift/macOS** | 高 | 开发者不熟悉 Swift | AI Coding 辅助 |
| **Kotlin/Android** | 高 | 开发者不熟悉 Kotlin | AI Coding 辅助 |
| **mDNS 网络发现** | 高 | 跨平台网络协议 | 参考成熟库实现 |
| **BLE 蓝牙通信** | 高 | 各平台 API 差异大 | 分平台逐步验证 |
| **FFI 绑定** | 中 | Rust 与原生语言交互 | 使用 uniffi 简化 |

**整体技术风险：高**

开发者对所有技术栈均不熟悉，完全依赖 AI Coding 辅助。建议采用渐进式开发策略：
1. 先验证单一平台核心功能
2. 再扩展到跨平台通信
3. 保持每个功能模块可独立测试

### Development Phases

#### Phase 1: Mac + Android MVP（不可拆分）

| 组件 | 内容 | 优先级 |
|------|------|--------|
| **Rust Core** | 核心逻辑、加密、同步协议 | P0 |
| **mDNS 发现** | 局域网设备发现 | P0 |
| **BLE 通道** | 蓝牙设备发现与传输 | P0 |
| **macOS 客户端** | 菜单栏应用、剪贴板监听 | P0 |
| **Android 客户端** | 无障碍服务、自动同步 | P0 |
| **设备配对** | 二维码配对流程 | P0 |
| **TLS 加密** | 端到端安全传输 | P0 |

**关键约束：** WiFi 和 BLE 必须一起完成，不可先做 WiFi 后做 BLE。

#### Phase 2: 平台扩展

| 组件 | 内容 |
|------|------|
| **Windows 客户端** | 系统托盘、剪贴板监听 |
| **iOS 客户端** | 快捷指令触发同步 |

#### Phase 3: 功能增强

| 组件 | 内容 |
|------|------|
| **剪贴板历史** | PC 端（Mac/Windows） |
| **图片同步** | 扩展内容类型支持 |

### AI Coding Strategy

由于开发者对所有技术栈不熟悉，建议采用以下 AI Coding 策略：

1. **模块化开发**：每个 Rust 模块独立开发和测试
2. **平台优先级**：先完成 macOS，再对齐 Android
3. **参考实现**：利用 AI 分析类似开源项目（如 KDE Connect）
4. **渐进验证**：每完成一个功能点立即测试，不堆积

---

## Functional Requirements

### FR-1: 设备发现与配对

| ID | 需求 | 优先级 |
|----|------|--------|
| FR-1.1 | 同一局域网设备通过 mDNS 自动发现 | P0 |
| FR-1.2 | 蓝牙范围内设备通过 BLE 广播发现 | P0 |
| FR-1.3 | 首次配对通过二维码扫描完成 | P0 |
| FR-1.4 | 配对信息本地持久化，重启后自动重连 | P0 |
| FR-1.5 | 支持同时配对多台设备（1:N 同步） | P1 |

### FR-2: 剪贴板同步

| ID | 需求 | 优先级 |
|----|------|--------|
| FR-2.1 | 监听本地剪贴板变化 | P0 |
| FR-2.2 | 检测到变化后自动同步到已配对设备 | P0 |
| FR-2.3 | 接收远程剪贴板内容并写入本地剪贴板 | P0 |
| FR-2.4 | 同步内容类型：纯文本 | P0 |
| FR-2.5 | 防止同步循环（标记来源，不回传） | P0 |

### FR-3: 通信通道

| ID | 需求 | 优先级 |
|----|------|--------|
| FR-3.1 | WiFi 优先：通过 TCP/TLS 传输数据 | P0 |
| FR-3.2 | BLE 备选：WiFi 不可用时自动切换 | P0 |
| FR-3.3 | 通道状态监测与自动切换 | P0 |
| FR-3.4 | 连接失败时重试 3 次 | P0 |
| FR-3.5 | 重试失败后用户可选择：丢弃/等待/继续重试 | P1 |

### FR-4: 安全与隐私

| ID | 需求 | 优先级 |
|----|------|--------|
| FR-4.1 | 所有传输使用 TLS 1.3 加密 | P0 |
| FR-4.2 | 配对时交换公钥，建立信任关系 | P0 |
| FR-4.3 | 数据仅在设备间直接传输，不经过任何服务器 | P0 |
| FR-4.4 | 本地存储的配对信息加密保护 | P1 |

### FR-5: 平台集成

| ID | 需求 | 平台 | 优先级 |
|----|------|------|--------|
| FR-5.1 | 菜单栏常驻图标，显示连接状态 | macOS | P0 |
| FR-5.2 | 无障碍服务后台运行，自动同步 | Android | P0 |
| FR-5.3 | 系统托盘图标，显示连接状态 | Windows | P0 (Phase 2) |
| FR-5.4 | 快捷指令触发同步 | iOS | P0 (Phase 2) |

### FR-6: 用户反馈

| ID | 需求 | 优先级 |
|----|------|--------|
| FR-6.1 | 同步成功时显示低调的系统通知 | P1 |
| FR-6.2 | 连接状态实时显示（已连接/断开/同步中） | P0 |
| FR-6.3 | 同步失败时提示用户 | P1 |

---

## Non-Functional Requirements

### NFR-1: 性能

| ID | 需求 | 目标值 |
|----|------|--------|
| NFR-1.1 | WiFi 同步延迟 | < 3 秒 |
| NFR-1.2 | BLE 同步延迟 | < 10 秒 |
| NFR-1.3 | 同步成功率 | > 95% |
| NFR-1.4 | 后台内存占用 | < 50 MB |
| NFR-1.5 | CPU 空闲占用 | < 1% |

### NFR-2: 安装包大小

| ID | 需求 | 目标值 |
|----|------|--------|
| NFR-2.1 | 桌面端安装包 | < 10 MB |
| NFR-2.2 | 移动端安装包 | < 5 MB |

### NFR-3: 可靠性

| ID | 需求 | 说明 |
|----|------|------|
| NFR-3.1 | 后台稳定运行 | 不被系统杀死，持续监听 |
| NFR-3.2 | 网络切换恢复 | WiFi 断开/重连后自动恢复 |
| NFR-3.3 | 设备重启恢复 | 重启后自动启动并重连 |

### NFR-4: 兼容性

| ID | 需求 | 说明 |
|----|------|------|
| NFR-4.1 | macOS 版本 | macOS 12+ (Monterey) |
| NFR-4.2 | Android 版本 | Android 10+ (API 29) |
| NFR-4.3 | Windows 版本 | Windows 10 1903+ |
| NFR-4.4 | iOS 版本 | iOS 15+ |

### NFR-5: 可维护性

| ID | 需求 | 说明 |
|----|------|------|
| NFR-5.1 | 模块化架构 | Rust 核心可独立测试 |
| NFR-5.2 | 日志记录 | 关键操作有日志，便于调试 |
| NFR-5.3 | 代码可读性 | AI Coding 生成的代码需保持清晰结构 |

---

## Appendix

### 术语表

| 术语 | 说明 |
|------|------|
| mDNS | Multicast DNS，局域网设备发现协议 |
| BLE | Bluetooth Low Energy，低功耗蓝牙 |
| TLS | Transport Layer Security，传输层安全协议 |
| FFI | Foreign Function Interface，外部函数接口 |
| uniffi | Mozilla 的 Rust FFI 绑定生成工具 |

### 参考资料

- [KDE Connect](https://kdeconnect.kde.org/) - 开源跨平台设备连接工具
- [LocalSend](https://localsend.org/) - 本地网络文件传输工具
- [Clipt](https://clipt.app/) - 云端剪贴板同步工具（作为反面参考）

### 文档历史

| 版本 | 日期 | 作者 | 变更 |
|------|------|------|------|
| 1.0 | 2025-12-13 | Mouse | 初始版本 |

