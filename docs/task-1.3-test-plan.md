# Task 1.3 测试计划：双向配对 FFI 集成

## 测试目标

验证使用 ECDH 密钥交换的双向配对协议在 macOS 和 Android 平台上的功能正确性和安全性。

## 测试环境

- **macOS 设备**: macOS 14+ (已构建 NearClip.app)
- **Android 设备**: Android 8.0+ (已构建 debug APK)
- **网络环境**: 同一 WiFi 网络或蓝牙范围内

## 测试场景

### 1. macOS → Android QR 码配对

**前提条件**:
- macOS 应用已启动
- Android 应用已安装并启动

**测试步骤**:
1. 在 macOS 上打开配对界面（"Add Device"）
2. 选择 "Show QR Code" 标签
3. 验证 QR 码显示正常
4. 验证配对码（JSON）包含 `public_key` 字段（Base64 编码）
5. 在 Android 上打开配对界面
6. 扫描或手动输入 macOS 的配对码
7. 等待配对完成

**预期结果**:
- ✅ QR 码成功生成并显示
- ✅ 配对码包含 ECDH 公钥（非空 Base64 字符串）
- ✅ Android 成功解析配对码
- ✅ 设备连接建立（WiFi 或 BLE）
- ✅ macOS 设备出现在 Android 配对设备列表
- ✅ 连接状态显示为 "Connected"

**验证点**:
- [ ] macOS `generate_qr_code()` 被调用
- [ ] Android `pairWithQrCode()` 被调用
- [ ] 日志中显示 "Generated QR code JSON"
- [ ] 日志中显示 "QR code parsed successfully"
- [ ] 日志中显示 "Device paired successfully via QR code"

---

### 2. Android → macOS QR 码配对

**前提条件**:
- Android 应用已启动
- macOS 应用已启动

**测试步骤**:
1. 在 Android 上生成配对 QR 码
2. 验证配对码包含 `public_key` 字段
3. 在 macOS 上打开配对界面
4. 选择 "Enter Code" 标签
5. 输入或扫描 Android 的配对码
6. 点击 "Pair Device" 按钮
7. 等待配对完成

**预期结果**:
- ✅ Android QR 码成功生成
- ✅ 配对码包含 ECDH 公钥
- ✅ macOS 成功解析配对码
- ✅ 设备连接建立（WiFi 或 BLE）
- ✅ Android 设备出现在 macOS 配对设备列表
- ✅ 连接状态显示为 "Connected"
- ✅ 配对成功后界面自动关闭（1.5秒后）

**验证点**:
- [ ] Android `generatePairingCode()` 被调用
- [ ] macOS `pairWithQRCode()` 被调用
- [ ] 日志中显示 "Generated QR code data"
- [ ] 日志中显示 "Paired with device via QR code"

---

### 3. 手动代码输入测试

**测试步骤**:
1. 从一个设备复制配对码（JSON 字符串）
2. 在另一个设备上手动粘贴配对码
3. 验证配对成功

**预期结果**:
- ✅ 支持复制粘贴配对码
- ✅ 配对过程与扫描 QR 码相同
- ✅ 配对成功

---

### 4. 错误处理验证

#### 4.1 无效配对码测试

**测试步骤**:
1. 在配对界面输入无效的 JSON 字符串
2. 尝试配对

**预期结果**:
- ✅ 显示错误消息："Invalid QR code data" 或类似提示
- ✅ 不会崩溃
- ✅ 用户可以重新输入

#### 4.2 缺少公钥字段测试

**测试步骤**:
1. 输入旧版配对码（不含 `public_key` 字段）
2. 尝试配对

**预期结果**:
- ✅ 显示错误消息："QR code validation failed"
- ✅ 不会崩溃

#### 4.3 连接失败测试

**测试步骤**:
1. 使用有效配对码但设备不在网络/蓝牙范围内
2. 尝试配对

**预期结果**:
- ✅ 显示超时或连接失败错误
- ✅ macOS 显示重试对话框（连接失败提示）
- ✅ 设备未添加到配对列表

---

### 5. 存储验证

#### 5.1 macOS Keychain 存储

**测试步骤**:
1. 成功配对一个设备
2. 重启 macOS 应用
3. 检查配对设备列表

**预期结果**:
- ✅ 配对设备信息保持不变
- ✅ 设备 ID、名称、平台信息正确
- ✅ 日志显示 "Device storage registered with FFI manager"

#### 5.2 Android SharedPreferences 存储

**测试步骤**:
1. 成功配对一个设备
2. 重启 Android 应用
3. 检查配对设备列表

**预期结果**:
- ✅ 配对设备信息保持不变
- ✅ 设备 ID、名称、平台信息正确

---

### 6. 代码简化验证

**验证点**:
- [x] macOS `PairingView.swift` `pairWithCode()` 方法从 ~60 行简化到 ~28 行
- [x] Android `ConnectionManager.kt` `addDeviceFromCode()` 方法从 ~80 行简化到 ~42 行
- [x] 所有 JSON 解析逻辑移至 Rust 层
- [x] 所有 ECDH 密钥交换逻辑在 Rust 层实现

---

### 7. 安全性验证

**验证点**:
- [ ] 配对码包含 Base64 编码的 ECDH 公钥（~44 字符）
- [ ] 公钥格式符合 P-256 规范（32 字节压缩格式）
- [ ] 配对过程使用 ECDH 密钥交换（检查日志）
- [ ] 旧版配对码（无 `public_key`）被拒绝

---

## 测试检查清单

### 功能测试
- [ ] macOS → Android 配对成功
- [ ] Android → macOS 配对成功
- [ ] QR 码生成正常
- [ ] 手动代码输入成功
- [ ] 配对设备列表正确显示
- [ ] 连接状态正确更新

### 错误处理
- [ ] 无效 JSON 处理正确
- [ ] 缺少公钥字段处理正确
- [ ] 连接失败处理正确
- [ ] 超时处理正确

### 存储持久化
- [ ] macOS Keychain 存储/加载正确
- [ ] Android SharedPreferences 存储/加载正确
- [ ] 重启后设备列表保持一致

### 代码质量
- [x] 代码行数减少 50%
- [x] 逻辑集中到 Rust 层
- [x] 平台代码仅处理 UI 和异步调度

### 安全性
- [ ] ECDH 公钥存在于配对码
- [ ] 密钥格式正确
- [ ] 密钥长度正确（32 字节）

---

## 日志检查点

### macOS 日志（Console.app 或 Xcode）
```
ConnectionManager: Generated QR code data (XXX bytes)
ConnectionManager: Paired with device via QR code: <device_name>
```

### Android 日志（Logcat）
```
ConnectionManager: Generated QR code for pairing
ConnectionManager: Device paired successfully via QR code: <device_name> (<device_id>)
```

### Rust FFI 日志（tracing）
```
Generating QR code for pairing
Generated QR code JSON, json_len=XXX
Pairing with device from QR code, qr_data_len=XXX
QR code parsed successfully, device_id=XXX
QR code pairing successful, device_id=XXX
```

---

## 测试结果记录

### 测试执行日期
- 日期: ___________
- 测试人员: ___________
- 测试设备: macOS _________, Android _________

### 通过/失败统计
- 总测试场景: 7
- 通过: ___
- 失败: ___
- 阻塞: ___

### 发现的问题
1.
2.
3.

### 备注
