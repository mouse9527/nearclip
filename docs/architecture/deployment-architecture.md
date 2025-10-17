# 部署架构

## 部署策略

**前端部署：**
- **平台：** Google Play Store (Android), Mac App Store (macOS)
- **统一构建命令：** `./gradlew buildAll`
- **平台特定构建：** `./gradlew buildAndroid`, `./gradlew buildMac`
- **输出目录：**
  - Android: `src/platform/android/app/build/outputs/apk/release/`
  - macOS: `src/platform/mac/.build/release/`
- **CDN/边缘：** 不适用（本地应用）

**后端部署：**
- **平台：** 本地设备部署，无需服务器
- **构建命令：** 集成在统一构建中
- **部署方式：** P2P 设备直连

**构建流程：**
1. **Protocol Buffers 生成** → `./gradlew generateProtobuf`
2. **Rust 共享库构建** → `./gradlew buildRust`
3. **平台应用构建** → 并行构建 Android 和 macOS
4. **最终产物打包** → 生成 APK 和 macOS 应用包

## CI/CD 流水线

### GitHub Actions 工作流

```yaml
name: NearClip CI/CD - 统一构建

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  # Rust 共享库测试
  test-rust:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Rust tests
      run: |
        cd src/shared/rust
        cargo test

    - name: Rust build
      run: |
        cd src/shared/rust
        cargo build --release

  # Android 构建和测试
  build-android:
    runs-on: ubuntu-latest
    needs: test-rust
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

    - name: Generate Protocol Buffers
      run: ./gradlew generateProtobuf

    - name: Android tests
      run: |
        cd src/platform/android
        ./gradlew test

    - name: Android build
      run: |
        cd src/platform/android
        ./gradlew assembleRelease

  # macOS 构建和测试
  build-mac:
    runs-on: macos-latest
    needs: test-rust
    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Generate Protocol Buffers
      run: ./gradlew generateProtobuf

    - name: macOS tests
      run: |
        cd src/platform/mac
        swift test

    - name: macOS build
      run: |
        cd src/platform/mac
        swift build -c release

  # 统一构建验证
  build-all:
    runs-on: macos-latest
    needs: [test-rust]
    if: github.ref == 'refs/heads/main'
    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Set up JDK 17
      uses: actions/setup-java@v3
      with:
        java-version: '17'
        distribution: 'temurin'

    - name: Build all platforms
      run: ./gradlew buildAll

    - name: Run all tests
      run: ./gradlew testAll

    - name: Upload build artifacts
      uses: actions/upload-artifact@v3
      with:
        name: build-artifacts
        path: |
          src/platform/android/app/build/outputs/apk/release/
          src/platform/mac/.build/release/
```

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
