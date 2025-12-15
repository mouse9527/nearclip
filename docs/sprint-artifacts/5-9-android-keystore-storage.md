# Story 5.9: 实现 Android Keystore 存储

Status: done

## Story

As a Android 用户,
I want 安全存储配对信息和密钥,
So that 我的数据不会被恶意应用访问.

## Acceptance Criteria

1. **Given** 配对完成 **When** 保存设备信息 **Then** 加密存储
2. **And** 使用 Android Keystore 管理加密密钥
3. **And** 应用重启后可以解密读取

## Tasks / Subtasks

- [x] Task 1: 创建 SecureStorage (AC: 1, 2)
  - [x] 1.1 使用 Android Keystore 生成/获取密钥
  - [x] 1.2 AES-GCM 加密/解密数据
  - [x] 1.3 EncryptedSharedPreferences 存储

- [x] Task 2: 集成到 ConnectionManager (AC: 1, 3)
  - [x] 2.1 保存配对设备信息
  - [x] 2.2 读取配对设备信息
  - [x] 2.3 删除设备时清除数据

## Dev Notes

### 依赖

```kotlin
implementation("androidx.security:security-crypto:1.1.0-alpha06")
```

### Android Keystore

- 密钥存储在硬件安全模块 (TEE/SE)
- 密钥不可导出
- 支持 Android 6.0+ (API 23+)

### 与其他 Story 的关系

- Story 5-7: 配对信息需要安全存储
- Story 2-9: Rust 核心的配对数据持久化

## Checklist

- [x] All tasks completed
- [x] Data encrypted at rest
- [x] Survives app restart
- [x] Story file updated to 'done'

## Implementation Summary

### SecureStorage

位置: `data/SecureStorage.kt`

功能:
- `MasterKey` 使用 AES256_GCM 方案
- `EncryptedSharedPreferences` 加密存储
- 密钥自动存储在 Android Keystore

API:
- `savePairedDevices()` / `loadPairedDevices()`: 配对设备列表
- `addPairedDevice()` / `removePairedDevice()`: 单个设备操作
- `saveDeviceKeys()` / `loadDeviceKeys()`: 设备加密密钥
- `clearAll()`: 清除所有数据

### ConnectionManager 集成

变更:
- 改为 `AndroidViewModel` 获取 Application Context
- 初始化时从 SecureStorage 加载配对设备
- `addDeviceFromCode()` 持久化到 SecureStorage
- `removeDevice()` 从 SecureStorage 删除

### 安全特性

- AES256_GCM 加密数据
- AES256_SIV 加密键名
- 密钥存储在 Android Keystore (硬件支持)
- 密钥不可导出
