# 开发工作流

## 本地开发设置

### 先决条件

```bash
# Android 开发环境
# 安装 Android Studio
# 配置 Android SDK (API 24+)
# 启用 BLE 开发选项

# macOS 开发环境
# 安装 Xcode 15.0+
# 配置开发者账号
# 启用 BLE 开发权限
```

### 初始设置

```bash
# 克隆仓库
git clone https://github.com/your-org/nearclip.git
cd nearclip

# 设置 Android 开发环境
cd android
./gradlew build

# 设置 macOS 开发环境
cd ../mac
swift package resolve
open NearClip.xcodeproj

# 运行初始测试
./scripts/test.sh
```

### 开发命令

```bash
# 启动所有服务
./scripts/dev-start.sh

# 仅启动 Android 开发
cd android && ./gradlew installDebug

# 仅启动 Mac 开发
cd mac && xcodebuild -scheme NearClip run

# 运行测试
./scripts/test.sh
```

## 环境配置

### 必需的环境变量

```bash
# Android (.env.local)
ANDROID_KEYSTORE_PASSWORD=your_keystore_password
ANDROID_KEY_ALIAS=your_key_alias

# Mac (.env)
DEVELOPER_TEAM_ID=your_developer_team_id
CODE_SIGN_IDENTITY=Apple Development

# 共享
LOG_LEVEL=DEBUG
PAIRING_TIMEOUT_MS=300000
SYNC_RETRY_LIMIT=3
```
