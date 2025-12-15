# NearClip 构建指南

本文档详细说明如何构建 NearClip 的各个组件。

## 环境要求

### 通用要求
- Git
- Rust 1.75+ (`rustup` 推荐)

### macOS 开发
- macOS 13.0+ (Ventura)
- Xcode 15+ (含 Command Line Tools)
- Swift 5.9+

### Android 开发
- Android Studio Hedgehog+
- Android SDK (API 26+)
- Android NDK r25+
- JDK 17+

## 构建 Rust Core

### 1. 克隆仓库

```bash
git clone https://github.com/user/nearclip.git
cd nearclip
```

### 2. 构建所有 Crates

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 运行测试
cargo test

# 运行特定 crate 测试
cargo test -p nearclip-core
cargo test -p nearclip-sync
```

### 3. 生成文档

```bash
cargo doc --open
```

## 构建 macOS 客户端

### 方法 1: Swift Package Manager

```bash
cd macos/NearClip

# 构建
swift build

# Release 构建
swift build -c release

# 运行
swift run
```

### 方法 2: 使用构建脚本

```bash
# 生成 Swift 绑定并构建
./scripts/build-swift.sh
```

### 方法 3: Xcode

1. 打开 `macos/NearClip/Package.swift`
2. 选择目标设备
3. `Cmd + B` 构建
4. `Cmd + R` 运行

### 生成 Swift 绑定

如需手动生成 UniFFI Swift 绑定：

```bash
# 构建 FFI crate
cargo build -p nearclip-ffi --release

# 生成 Swift 代码
cargo run -p nearclip-ffi --bin uniffi-bindgen generate \
    --library target/release/libnearclip_ffi.dylib \
    --language swift \
    --out-dir macos/NearClip/Sources/NearClipFFI
```

## 构建 Android 客户端

### 1. 安装 Rust Android 目标

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

### 2. 配置 NDK

在 `~/.cargo/config.toml` 添加：

```toml
[target.aarch64-linux-android]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"

[target.x86_64-linux-android]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang"

[target.i686-linux-android]
linker = "/path/to/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang"
```

### 3. 构建 Rust 库 (Android)

```bash
# ARM64 (主流设备)
cargo build -p nearclip-ffi --release --target aarch64-linux-android

# ARM32 (旧设备)
cargo build -p nearclip-ffi --release --target armv7-linux-androideabi

# x86_64 (模拟器)
cargo build -p nearclip-ffi --release --target x86_64-linux-android
```

### 4. 生成 Kotlin 绑定

```bash
cargo run -p nearclip-ffi --bin uniffi-bindgen generate \
    --library target/aarch64-linux-android/release/libnearclip_ffi.so \
    --language kotlin \
    --out-dir android/app/src/main/java
```

### 5. 复制原生库

```bash
# 创建 jniLibs 目录
mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64}

# 复制库文件
cp target/aarch64-linux-android/release/libnearclip_ffi.so \
   android/app/src/main/jniLibs/arm64-v8a/

cp target/armv7-linux-androideabi/release/libnearclip_ffi.so \
   android/app/src/main/jniLibs/armeabi-v7a/

cp target/x86_64-linux-android/release/libnearclip_ffi.so \
   android/app/src/main/jniLibs/x86_64/
```

### 6. 构建 APK

```bash
cd android

# Debug APK
./gradlew assembleDebug

# Release APK
./gradlew assembleRelease

# 安装到设备
./gradlew installDebug
```

## 开发工作流

### 日常开发

```bash
# 1. 修改 Rust 代码
vim crates/nearclip-core/src/lib.rs

# 2. 运行测试
cargo test

# 3. 构建
cargo build

# 4. 如修改了 FFI 接口，重新生成绑定
./scripts/build-swift.sh
# 或
./scripts/build-kotlin.sh
```

### 代码检查

```bash
# 格式化
cargo fmt

# Lint
cargo clippy

# 安全审计
cargo audit
```

### 性能分析

```bash
# 带符号的 Release 构建
cargo build --release

# 使用 Instruments (macOS)
instruments -t "Time Profiler" target/release/nearclip

# 使用 perf (Linux)
perf record target/release/nearclip
perf report
```

## 常见问题

### Q: Swift 绑定找不到 Rust 库

确保 `libnearclip_ffi.dylib` 在正确路径：
```bash
ls -la target/release/libnearclip_ffi.dylib
```

### Q: Android 构建失败 "NDK not found"

设置 `ANDROID_NDK_HOME` 环境变量：
```bash
export ANDROID_NDK_HOME=/path/to/android-ndk
```

### Q: cargo build 报错 "ring" 编译失败

确保安装了必要的系统依赖：
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential pkg-config libssl-dev
```

### Q: BLE 功能在模拟器不工作

BLE 需要真实硬件，建议使用真机测试。

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Build

on: [push, pull_request]

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - run: cargo test

  macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build -p nearclip-ffi --release
      - run: cd macos/NearClip && swift build

  android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: rustup target add aarch64-linux-android
      - run: cargo build -p nearclip-ffi --target aarch64-linux-android
```

## 发布构建

### macOS App Bundle

```bash
# 构建 Release
cd macos/NearClip
swift build -c release

# 创建 .app
# (需要额外的打包脚本)
```

### Android Release APK

```bash
cd android

# 配置签名 (在 keystore.properties)
# 构建签名 APK
./gradlew assembleRelease
```
