# 部署架构

## 部署策略

**前端部署：**
- **平台：** Google Play Store (Android), Mac App Store (macOS)
- **构建命令：** `./scripts/build-android.sh`, `./scripts/build-mac.sh`
- **输出目录：** `android/app/build/outputs/apk/`, `mac/build/`
- **CDN/边缘：** 不适用（本地应用）

**后端部署：**
- **平台：** 本地设备部署，无需服务器
- **构建命令：** 集成在应用构建中
- **部署方式：** P2P 设备直连

## CI/CD 流水线

```yaml
name: Android CI/CD
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Set up JDK 17
      uses: actions/setup-java@v3
      with:
        java-version: '17'
        distribution: 'temurin'

    - name: Cache Gradle packages
      uses: actions/cache@v3
      with:
        path: |
          ~/.gradle/caches
          ~/.gradle/wrapper
        key: ${{ runner.os }}-gradle-${{ hashFiles('**/*.gradle*', '**/gradle-wrapper.properties') }}

    - name: Run tests
      run: ./gradlew test

    - name: Run lint
      run: ./gradlew lint

    - name: Build debug APK
      run: ./gradlew assembleDebug

    - name: Upload build artifacts
      uses: actions/upload-artifact@v3
      with:
        name: android-apk
        path: android/app/build/outputs/apk/debug/
```

```yaml
name: macOS CI/CD
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: macos-latest
    steps:
    - uses: actions/checkout@v3

    - name: Select Xcode
      run: sudo xcode-select -switch /Applications/Xcode.app/Contents/Developer

    - name: Build and test
      run: |
        cd mac
        swift build
        swift test

    - name: Build release
      run: |
        cd mac
        xcodebuild -scheme NearClip -configuration Release build

    - name: Upload build artifacts
      uses: actions/upload-artifact@v3
      with:
        name: mac-build
        path: mac/build/
```

## 环境

| 环境 | 前端 URL | 后端 URL | 用途 |
|------|----------|----------|------|
| 开发 | 本地设备 | 本地设备 | 本地开发和测试 |
| 测试 | TestFlight | 本地设备 | 内部测试版本 |
| 生产 | App Store | 本地设备 | 公开发布版本 |
