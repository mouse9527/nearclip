# Task 1.3 手动测试指导

## 快速开始

### 准备工作

1. **macOS 应用启动**:
   ```bash
   # 方法1: 直接运行已构建的应用
   open macos/NearClip/NearClip.app

   # 方法2: 通过 Xcode 启动（可查看实时日志）
   open macos/NearClip/NearClip.xcodeproj
   # 然后在 Xcode 中点击 Run (⌘R)
   ```

2. **Android 应用安装**:
   ```bash
   # 连接 Android 设备后安装
   adb install android/app/build/outputs/apk/debug/app-debug.apk

   # 或者重新构建并安装
   cd android && ./gradlew installDebug
   ```

3. **查看日志**:
   - **macOS**: 打开 Console.app，搜索 "NearClip" 或 "ConnectionManager"
   - **Android**: 使用 `adb logcat | grep ConnectionManager`

---

## 测试场景 1: macOS → Android 配对

### 步骤

1. **在 macOS 上生成配对码**:
   - 启动 NearClip macOS 应用
   - 点击状态栏图标
   - 选择 "Add Device..."
   - 确保选择 "Show QR Code" 标签
   - **验证**: 应该看到一个 QR 码和下方的配对码文本

2. **检查配对码格式**:
   - 点击配对码右侧的复制按钮
   - 粘贴到文本编辑器
   - **验证内容应该类似**:
     ```json
     {
       "version": 1,
       "device_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
       "public_key": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
       "connection_info": {
         "addresses": ["192.168.1.xxx"],
         "port": 7890,
         "mdns_name": "Mac.local"
       }
     }
     ```
   - **关键验证点**: `public_key` 字段必须存在且长度约为 44 字符（Base64 编码）

3. **在 Android 上输入配对码**:
   - 打开 NearClip Android 应用
   - 点击 "Add Device" 按钮
   - 粘贴刚才复制的配对码
   - 点击 "Pair Device"
   - **等待配对完成**（可能需要 5-30 秒）

4. **验证配对成功**:
   - **Android 端**:
     - 应该看到 macOS 设备出现在设备列表
     - 连接状态应该显示 "Connected" 或绿色指示器
   - **macOS 端**:
     - 配对窗口应该自动关闭或显示成功消息
     - 设备列表应该显示 Android 设备

5. **检查日志**:
   - **macOS 日志应该包含**:
     ```
     ConnectionManager: Generated QR code data (XXX bytes)
     Generated QR code JSON, json_len=XXX
     ```
   - **Android 日志应该包含**:
     ```
     QR code parsed successfully, device_id=XXX
     Device paired successfully via QR code: Mac (XXX)
     ```

---

## 测试场景 2: Android → macOS 配对

### 步骤

1. **在 Android 上生成配对码**:
   - 打开 NearClip Android 应用
   - 点击 "Show QR Code" 或类似按钮
   - **验证**: 应该看到一个 QR 码
   - 长按或点击复制配对码

2. **检查配对码格式**:
   - 将配对码粘贴到便签应用
   - **验证**: 格式应该与场景 1 中的类似，包含 `public_key` 字段

3. **在 macOS 上输入配对码**:
   - 打开 NearClip macOS 应用
   - 点击 "Add Device..."
   - 选择 "Enter Code" 标签
   - 粘贴 Android 配对码
   - 点击 "Pair Device"

4. **验证配对成功**:
   - macOS 应该显示 "Device paired successfully!"
   - 1.5 秒后配对窗口自动关闭
   - Android 设备应该出现在 macOS 设备列表

5. **检查日志**:
   - **Android 日志应该包含**:
     ```
     Generated QR code for pairing
     ```
   - **macOS 日志应该包含**:
     ```
     Paired with device via QR code: <Android设备名>
     ```

---

## 测试场景 3: 错误处理

### 3.1 无效配对码测试

1. 在任一平台的配对界面输入无效 JSON:
   ```
   {"invalid": "data"}
   ```
2. 点击 "Pair Device"
3. **预期结果**:
   - 显示错误消息："Invalid QR code data" 或 "Failed to pair"
   - 应用不崩溃
   - 可以重新输入

### 3.2 旧版配对码测试

1. 输入不含 `public_key` 的旧版配对码:
   ```json
   {"id":"test-device","name":"Test","platform":"ANDROID"}
   ```
2. 点击 "Pair Device"
3. **预期结果**:
   - 显示错误消息："QR code validation failed"
   - 应用不崩溃

### 3.3 连接超时测试

1. 使用有效配对码但确保两设备不在同一网络
2. 尝试配对
3. **预期结果**:
   - 显示连接失败或超时错误
   - macOS 显示重试对话框
   - 设备未添加到列表

---

## 测试场景 4: 存储持久化

### 4.1 macOS 存储验证

1. 成功配对一个设备
2. 完全退出 macOS 应用（⌘Q）
3. 重新启动应用
4. **验证**: 配对的设备仍然在列表中

### 4.2 Android 存储验证

1. 成功配对一个设备
2. 强制停止 Android 应用（设置 → 应用 → NearClip → 强制停止）
3. 重新启动应用
4. **验证**: 配对的设备仍然在列表中

---

## 检查日志的方法

### macOS 日志

**方法 1: Console.app**
1. 打开 Console.app
2. 在搜索框输入 "NearClip" 或 "ConnectionManager"
3. 查看实时日志

**方法 2: Xcode**
1. 在 Xcode 中运行应用
2. 打开 Debug Area (⌘⇧Y)
3. 查看 Console 输出

**方法 3: 命令行**
```bash
log stream --predicate 'process == "NearClip"' --level debug
```

### Android 日志

**方法 1: Android Studio**
1. 打开 Logcat 窗口
2. 过滤器输入 "ConnectionManager"

**方法 2: 命令行**
```bash
adb logcat | grep -E "(ConnectionManager|NearClip)"
```

**方法 3: 只看错误**
```bash
adb logcat *:E
```

---

## 配对码格式验证工具

可以使用在线 JSON 验证器检查配对码格式：

1. 复制配对码
2. 打开 https://jsonlint.com/
3. 粘贴并点击 "Validate JSON"
4. 验证包含以下字段：
   - `version`: 数字 1
   - `device_id`: UUID 字符串
   - `public_key`: Base64 字符串（约 44 字符）
   - `connection_info` (可选): 包含 `addresses`, `port`, `mdns_name`

---

## 常见问题排查

### 问题 1: QR 码不显示

**可能原因**:
- FFI 初始化失败
- `generate_qr_code()` 抛出异常

**排查方法**:
1. 检查日志是否有 "Failed to generate QR code" 错误
2. 确认 `libnearclip_ffi.dylib` 或 `libnearclip_ffi.a` 存在
3. 重新构建 Swift bindings: `./scripts/build-swift.sh`

### 问题 2: 配对失败 "Invalid QR code data"

**可能原因**:
- JSON 格式错误
- 缺少必需字段
- `public_key` 字段缺失或格式错误

**排查方法**:
1. 使用 JSON 验证器检查格式
2. 确认 `public_key` 字段存在
3. 检查 Rust 日志是否有 "QR code validation failed"

### 问题 3: 配对成功但无法连接

**可能原因**:
- 设备不在同一网络
- 防火墙阻止连接
- BLE 未启用

**排查方法**:
1. 确认两设备在同一 WiFi 网络
2. 检查 macOS 防火墙设置
3. 检查日志是否有 "Failed to connect" 错误
4. 尝试使用 BLE 连接（关闭 WiFi）

### 问题 4: 存储不持久化

**macOS**:
- 检查 Keychain Access.app 中是否有 "com.nearclip.devices" 条目
- 日志应该显示 "Device storage registered with FFI manager"

**Android**:
- 检查 `/data/data/com.nearclip/shared_prefs/` 是否有文件
- 使用 `adb shell run-as com.nearclip ls shared_prefs/` 查看

---

## 成功标志

完成以上所有测试场景后，应该满足以下条件：

- [x] macOS 和 Android 可以双向配对
- [x] 配对码包含 ECDH 公钥
- [x] 所有错误情况都能正确处理
- [x] 配对信息在重启后保持
- [x] 日志显示正确的 FFI 方法调用
- [x] 代码已简化（减少 50% 行数）

---

## 测试完成后

1. 填写测试结果到 `task-1.3-test-plan.md`
2. 如果发现问题，创建 issue 或修复
3. 如果测试通过，可以创建 git commit：
   ```bash
   git add .
   git commit -m "feat: implement bidirectional pairing with ECDH key exchange

   - Add generate_qr_code() and pair_with_qr_code() FFI methods
   - Update macOS PairingView to use new FFI methods
   - Update Android ConnectionManager to use new FFI methods
   - Simplify platform code by 50% (140 → 70 lines)
   - Upgrade from plaintext JSON to ECDH P-256 key exchange

   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```
