---
stepsCompleted: [1, 2, 3, 4, 5, 6]
status: completed
inputDocuments:
  - docs/analysis/research/technical-clipboard-sync-research-2025-12-12.md
workflowType: 'product-brief'
lastStep: 5
project_name: 'nearclip'
user_name: 'Mouse'
date: '2025-12-13'
---

# Product Brief: NearClip

**Date:** 2025-12-13
**Author:** Mouse

---

## Executive Summary

NearClip 是一款轻量级、跨平台的本地剪贴板同步工具，通过 WiFi 和蓝牙实现设备间的无缝内容传输。无需云服务、无需公网服务器、无需设备处于同一网络——只要设备在通信范围内，即可实现剪贴板内容的即时同步。

NearClip 采用 Rust 核心库 + 原生 UI 的架构，追求极致的轻量与高效，覆盖 Android、iOS、macOS、Windows 和 Linux 全平台。

---

## Core Vision

### Problem Statement

用户在多设备间传递文本、链接、图片等剪贴板内容时，往往需要借助微信、Telegram 等通讯工具「自己发给自己」，操作繁琐且打断工作流。

### Problem Impact

- **工作效率损失**：每次跨设备复制都需要切换应用、登录账号、发送消息
- **隐私风险**：敏感内容经过第三方服务器传输
- **场景受限**：无网络环境下完全无法跨设备传输

### Why Existing Solutions Fall Short

| 现有方案 | 痛点 |
|---------|------|
| 通讯工具中转 | 操作繁琐，需登录，隐私风险 |
| 云剪贴板 (如 iCloud) | 需要互联网，仅限单一生态，延迟高 |
| 局域网同步工具 | 平台覆盖不全，必须处于同一 WiFi |
| KDE Connect 等 | 主要针对 Linux，iOS 支持差 |

### Proposed Solution

NearClip 通过**本地网络优先、蓝牙兜底**的策略实现剪贴板同步：

- **WiFi 直连**：同网络设备通过 mDNS 自动发现，高速传输
- **蓝牙备选**：无 WiFi 时自动切换 BLE，覆盖更多使用场景
- **端到端加密**：TLS 1.3 保护传输安全，内容不经过任何服务器
- **全平台覆盖**：Rust 核心库确保一致性，原生 UI 保证体验

### Key Differentiators

1. **零云依赖**：完全本地化，无需注册、无需服务器
2. **极致轻量**：Rust 核心 + 原生开发，安装包小、运行快
3. **蓝牙兜底**：WiFi 不可用时仍可同步，覆盖更多场景
4. **真正跨平台**：Android/iOS/macOS/Windows/Linux 全覆盖
5. **隐私至上**：数据只在设备间直接传输，端到端加密

---

## Target Users

### Primary Users

**跨平台多设备用户 - 「小明」**

**背景：**
- 使用 Mac 作为主力工作电脑，Windows 作为家用/游戏电脑，Android 手机随身携带
- 在不同设备间频繁切换，没有单一生态的锁定
- 对效率敏感，不愿意为简单的复制粘贴花费额外时间

**痛点场景：**
- 手机上收到一个长链接，想在电脑浏览器打开
- 电脑上复制了一段地址/文字，需要发到手机上的打车/外卖 App
- 在一台电脑上复制的内容，需要粘贴到另一台电脑
- 收到短信验证码，需要在电脑上输入

**现有解决方案：**
- 微信/Telegram「文件传输助手」中转 → 繁琐，需要登录多端
- 云笔记当中转站 → 需要联网，操作步骤多
- 手动重新输入 → 效率低，容易出错

**理想体验：**
- 一台设备上复制，其他设备直接能粘贴
- 无需思考「怎么传」，专注于「做什么」

### Secondary Users

暂不考虑次要用户群体，MVP 阶段专注服务核心的跨平台多设备用户。

### User Journey

| 阶段 | 用户行为 |
|------|---------|
| **发现** | 搜索「跨平台剪贴板同步」或通过技术社区/朋友推荐 |
| **安装** | 在各设备上安装对应客户端 |
| **配对** | 首次启动时，设备通过局域网/蓝牙自动发现并配对 |
| **日常使用** | 复制即同步，无需额外操作（Android 自动，iOS 通过快捷指令触发） |
| **Aha 时刻** | 第一次在手机上复制、电脑上直接粘贴成功时 |

---

## Success Metrics

### 核心成功标准

**功能完整性：**
- ✅ 支持 macOS、Windows、Linux、Android、iOS 五个平台
- ✅ WiFi 同网络设备能自动发现并同步
- ✅ 蓝牙作为备选通道可用
- ✅ 端到端加密保护隐私

**用户体验：**
- ✅ 复制后 3 秒内其他设备可粘贴（同 WiFi）
- ✅ 同步成功率 > 95%
- ✅ 首次配对流程简单直观

**轻量目标：**
- ✅ 桌面端安装包 < 10 MB
- ✅ 移动端安装包 < 5 MB
- ✅ 后台运行内存占用 < 50 MB

### Business Objectives

个人项目，无商业化目标。成功 = 自己用着顺手。

### Key Performance Indicators

N/A - 不需要复杂的业务指标，功能可用即成功。

---

## MVP Scope

### Core Features

**MVP 必须包含：**

| 功能 | 说明 |
|------|------|
| **平台支持** | macOS、Windows、Android、iOS |
| **WiFi 同步** | 同局域网设备通过 mDNS 发现，自动同步 |
| **蓝牙同步** | BLE 作为备选通道，无 WiFi 时可用 |
| **文本同步** | 支持文本、链接等纯文本内容 |
| **端到端加密** | TLS 1.3 保护传输安全 |
| **自动同步** | 桌面端全自动，Android 通过无障碍服务，iOS 通过快捷指令 |

**开发顺序：**
1. **Phase 1**：Mac + Android（验证核心功能）
2. **Phase 2**：扩展到 Windows + iOS

### Out of Scope for MVP

| 功能 | 状态 | 说明 |
|------|------|------|
| 剪贴板历史 | 🔜 后续版本 | PC 端优先，移动端可选 |
| 图片/文件传输 | 🔜 后续版本 | 先验证文本同步可行性 |
| Linux 支持 | 🔜 后续版本 | 优先覆盖主流平台 |
| 跨网段穿透 | ❌ 暂不考虑 | 保持本地网络定位 |

### MVP Success Criteria

- ✅ Mac ↔ Android 能通过 WiFi 成功同步文本
- ✅ Mac ↔ Android 能通过蓝牙成功同步文本
- ✅ 同步延迟 < 3 秒
- ✅ 日常使用稳定可靠

### Future Vision

**v1.1 - 平台扩展：**
- Windows + iOS 客户端
- 剪贴板历史（PC 端）

**v1.2 - 内容扩展：**
- 图片同步支持
- 富文本支持

**v2.0 - 功能增强：**
- 文件传输
- Linux 支持
- 更多自动化集成
