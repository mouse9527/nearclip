# Story 2: 安全配对与密钥交换

## INVEST 分析

- **Independent**: 在设备发现基础上独立实现
- **Negotiable**: 配对方式和加密算法可调整
- **Valuable**: 确保数据传输安全
- **Estimable**: 密码学实现复杂度可估算
- **Small**: 核心配对流程适合单个sprint
- **Testable**: 可通过安全性测试验证

## 用户场景

作为用户，我希望能够安全地配对设备并建立加密通道，这样我的剪贴板数据就不会被窃取。

## 验收标准

- [ ] 支持二维码配对方式
- [ ] 支持PIN码配对方式
- [ ] 配对过程使用ECDH密钥交换
- [ ] 数据传输使用AES-256-GCM加密
- [ ] 配对状态持久化存储
- [ ] 支持取消配对和删除密钥
- [ ] 配对失败时有明确的错误提示

## 技术实现

### 核心组件
- **ECDH密钥交换**: 使用 `x25519-dalek` crate
- **AES-256-GCM加密**: 使用 `aes-gcm` crate
- **随机数生成**: 使用 `rand` crate
- **密钥存储**: 安全存储配对密钥
- **二维码生成**: 二维码配对支持

### 配对流程
1. 设备A生成ECDH密钥对
2. 设备A通过二维码/PIN码分享公钥
3. 设备B扫描/输入并生成共享密钥
4. 双方验证密钥交换成功
5. 建立加密通信通道

### 数据结构
```rust
struct PairedDevice {
    device_id: String,
    public_key: [u8; 32],
    shared_secret: [u8; 32],
    pairing_time: SystemTime,
    last_used: SystemTime,
}
```

## 相关任务

- [task-0201.md](./task-0201.md) - 实现ECDH密钥交换
- [task-0202.md](./task-0202.md) - 实现AES-256-GCM加密
- [task-0203.md](./task-0203.md) - 实现二维码配对
- [task-0204.md](./task-0204.md) - 实现PIN码配对
- [task-0205.md](./task-0205.md) - 实现密钥持久化存储