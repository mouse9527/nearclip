#!/bin/bash

# NearClip 全平台构建脚本
# 构建所有平台的目标

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHARED_RUST_DIR="$PROJECT_ROOT/shared/rust"
ANDROID_DIR="$PROJECT_ROOT/android"
MAC_DIR="$PROJECT_ROOT/mac"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    log_info "检查构建依赖..."

    # 检查 Rust
    if ! command -v rustc &> /dev/null; then
        log_error "Rust 未安装。请访问 https://rustup.rs/ 安装 Rust。"
        exit 1
    fi

    # 检查 Cargo
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装。请安装 Rust 工具链。"
        exit 1
    fi

    # 检查 Protobuf 编译器
    if ! command -v protoc &> /dev/null; then
        log_error "Protocol Buffers 编译器未安装。"
        log_info "请安装 protoc："
        log_info "  macOS: brew install protobuf"
        log_info "  Ubuntu: sudo apt-get install protobuf-compiler"
        exit 1
    fi

    # 检查 Android 工具（可选）
    if command -v gradle &> /dev/null; then
        log_info "Android 构建工具可用"
    else
        log_warning "Gradle 未找到，将跳过 Android 构建"
    fi

    # 检查 Xcode（可选）
    if command -v xcodebuild &> /dev/null; then
        log_info "Xcode 构建工具可用"
    else
        log_warning "Xcode 未找到，将跳过 macOS 构建"
    fi

    log_success "依赖检查完成"
}

# 构建 Rust 核心库
build_rust_core() {
    log_info "构建 Rust 核心库..."

    cd "$SHARED_RUST_DIR"

    # 为所有目标平台构建
    TARGETS=(
        # Android
        "aarch64-linux-android"
        "armv7-linux-androideabi"
        "x86_64-linux-android"
        "i686-linux-android"
        # macOS
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )

    for target in "${TARGETS[@]}"; do
        log_info "构建目标: $target"

        # 安装目标（如果尚未安装）
        if ! rustup target list --installed | grep -q "$target"; then
            log_info "安装目标平台: $target"
            rustup target add "$target"
        fi

        # 构建
        cargo build --release --target="$target"

        if [ $? -eq 0 ]; then
            log_success "✅ $target 构建成功"
        else
            log_error "❌ $target 构建失败"
            return 1
        fi
    done

    log_success "Rust 核心库构建完成"
}

# 构建 Protocol Buffers
build_protobuf() {
    log_info "构建 Protocol Buffers..."

    cd "$PROJECT_ROOT/shared/protobuf"

    # 检查 .proto 文件是否存在
    if [ ! -f "proto/nearclip.proto" ]; then
        log_error "Protocol Buffers 定义文件不存在"
        return 1
    fi

    # 生成 Java 文件（Android）
    log_info "生成 Java Protocol Buffers 代码..."
    mkdir -p "$ANDROID_DIR/app/src/main/java/com/nearclip/protobuf"
    protoc --java_out="$ANDROID_DIR/app/src/main/java" \
           --grpc-java_out="$ANDROID_DIR/app/src/main/java" \
           -I proto \
           proto/*.proto

    # 生成 Swift 文件（macOS）
    log_info "生成 Swift Protocol Buffers 代码..."
    mkdir -p "$MAC_DIR/NearClip/Generated"
    protoc --swift_out="$MAC_DIR/NearClip/Generated" \
           -I proto \
           proto/*.proto

    log_success "Protocol Buffers 构建完成"
}

# 复制库文件到 Android 项目
copy_android_libs() {
    log_info "复制 Rust 库到 Android 项目..."

    local android_libs_dir="$ANDROID_DIR/app/src/main/jniLibs"
    rm -rf "$android_libs_dir"
    mkdir -p "$android_libs_dir"

    # 目标架构映射
    declare -A android_archs=(
        ["aarch64-linux-android"]="arm64-v8a"
        ["armv7-linux-androideabi"]="armeabi-v7a"
        ["x86_64-linux-android"]="x86_64"
        ["i686-linux-android"]="x86"
    )

    for target in "${!android_archs[@]}"; do
        local arch_dir="${android_archs[$target]}"
        local lib_path="$SHARED_RUST_DIR/target/$target/release/libnearclip.so"

        if [ -f "$lib_path" ]; then
            mkdir -p "$android_libs_dir/$arch_dir"
            cp "$lib_path" "$android_libs_dir/$arch_dir/"
            log_info "✅ 复制 $target -> $arch_dir"
        else
            log_warning "⚠️  库文件不存在: $lib_path"
        fi
    done

    log_success "Android 库文件复制完成"
}

# 复制库文件到 macOS 项目
copy_macos_libs() {
    log_info "复制 Rust 库到 macOS 项目..."

    local mac_frameworks_dir="$MAC_DIR/NearClip/Frameworks"
    rm -rf "$mac_frameworks_dir"
    mkdir -p "$mac_frameworks_dir"

    # 复制不同架构的库
    local intel_lib="$SHARED_RUST_DIR/target/x86_64-apple-darwin/release/libnearclip.dylib"
    local apple_lib="$SHARED_RUST_DIR/target/aarch64-apple-darwin/release/libnearclip.dylib"

    if [ -f "$intel_lib" ] && [ -f "$apple_lib" ]; then
        # 创建通用二进制文件
        log_info "创建通用二进制文件..."
        lipo -create "$intel_lib" "$apple_lib" -output "$mac_frameworks_dir/libnearclip.dylib"
        log_success "✅ 通用二进制文件创建成功"
    elif [ -f "$intel_lib" ]; then
        cp "$intel_lib" "$mac_frameworks_dir/"
        log_info "✅ 仅 Intel x86_64 库已复制"
    elif [ -f "$apple_lib" ]; then
        cp "$apple_lib" "$mac_frameworks_dir/"
        log_info "✅ 仅 Apple Silicon 库已复制"
    else
        log_warning "⚠️  未找到 macOS 库文件"
    fi

    log_success "macOS 库文件复制完成"
}

# 构建 Android 应用
build_android() {
    if ! command -v gradle &> /dev/null; then
        log_warning "Gradle 未安装，跳过 Android 构建"
        return 0
    fi

    log_info "构建 Android 应用..."

    cd "$ANDROID_DIR"

    # 检查是否可以构建
    if [ ! -f "app/build.gradle" ]; then
        log_warning "Android 项目配置不完整，跳过构建"
        return 0
    fi

    # 构建
    ./gradlew assembleDebug

    if [ $? -eq 0 ]; then
        log_success "✅ Android 应用构建成功"
        log_info "APK 位置: $ANDROID_DIR/app/build/outputs/apk/debug/"
    else
        log_error "❌ Android 应用构建失败"
        return 1
    fi
}

# 构建 macOS 应用
build_macos() {
    if ! command -v xcodebuild &> /dev/null; then
        log_warning "Xcode 未安装，跳过 macOS 构建"
        return 0
    fi

    log_info "构建 macOS 应用..."

    cd "$MAC_DIR"

    # 检查项目文件
    if [ ! -f "NearClip.xcodeproj/project.pbxproj" ]; then
        log_warning "macOS 项目配置不完整，跳过构建"
        return 0
    fi

    # 构建
    xcodebuild -project NearClip.xcodeproj \
               -scheme NearClip \
               -configuration Debug \
               -derivedDataPath build

    if [ $? -eq 0 ]; then
        log_success "✅ macOS 应用构建成功"
        log_info "应用位置: $MAC_DIR/build/Build/Products/Debug/NearClip.app"
    else
        log_error "❌ macOS 应用构建失败"
        return 1
    fi
}

# 运行测试
run_tests() {
    log_info "运行测试..."

    # Rust 测试
    cd "$SHARED_RUST_DIR"
    log_info "运行 Rust 测试..."
    cargo test

    if [ $? -eq 0 ]; then
        log_success "✅ Rust 测试通过"
    else
        log_error "❌ Rust 测试失败"
        return 1
    fi

    # Android 测试（如果可用）
    if command -v gradle &> /dev/null && [ -f "$ANDROID_DIR/app/build.gradle" ]; then
        log_info "运行 Android 测试..."
        cd "$ANDROID_DIR"
        ./gradlew testDebugUnitTest

        if [ $? -eq 0 ]; then
            log_success "✅ Android 测试通过"
        else
            log_warning "⚠️  Android 测试失败"
        fi
    fi

    log_success "测试运行完成"
}

# 清理构建文件
clean() {
    log_info "清理构建文件..."

    # Rust 清理
    cd "$SHARED_RUST_DIR"
    cargo clean

    # Android 清理
    if [ -f "$ANDROID_DIR/app/build.gradle" ]; then
        cd "$ANDROID_DIR"
        ./gradlew clean
    fi

    # macOS 清理
    if [ -f "$MAC_DIR/NearClip.xcodeproj/project.pbxproj" ]; then
        rm -rf "$MAC_DIR/build"
    fi

    # 复制的库文件
    rm -rf "$ANDROID_DIR/app/src/main/jniLibs"
    rm -rf "$MAC_DIR/NearClip/Frameworks"

    log_success "清理完成"
}

# 显示帮助信息
show_help() {
    echo "NearClip 构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -h, --help     显示此帮助信息"
    echo "  -c, --clean    清理构建文件"
    echo "  -t, --test     仅运行测试"
    echo "  -r, --rust     仅构建 Rust 核心库"
    echo "  -a, --android  仅构建 Android 应用"
    echo "  -m, --macos    仅构建 macOS 应用"
    echo "  --no-tests     跳过测试"
    echo ""
    echo "示例:"
    echo "  $0              # 完整构建所有平台"
    echo "  $0 --rust       # 仅构建 Rust 核心库"
    echo "  $0 --clean      # 清理构建文件"
}

# 主函数
main() {
    local clean_only=false
    local test_only=false
    local rust_only=false
    local android_only=false
    local macos_only=false
    local skip_tests=false

    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -c|--clean)
                clean_only=true
                shift
                ;;
            -t|--test)
                test_only=true
                shift
                ;;
            -r|--rust)
                rust_only=true
                shift
                ;;
            -a|--android)
                android_only=true
                shift
                ;;
            -m|--macos)
                macos_only=true
                shift
                ;;
            --no-tests)
                skip_tests=true
                shift
                ;;
            *)
                log_error "未知选项: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # 显示标题
    echo "=================================================="
    echo "🚀 NearClip 全平台构建脚本"
    echo "=================================================="
    echo ""

    # 检查依赖
    check_dependencies

    # 清理模式
    if [ "$clean_only" = true ]; then
        clean
        exit 0
    fi

    # 测试模式
    if [ "$test_only" = true ]; then
        run_tests
        exit 0
    fi

    # 构建流程
    log_info "开始构建流程..."

    # 构建 Rust 核心库
    if [ "$rust_only" = false ] && [ "$android_only" = false ] && [ "$macos_only" = false ]; then
        build_rust_core
        build_protobuf
        copy_android_libs
        copy_macos_libs
    fi

    # 单独构建选项
    if [ "$rust_only" = true ]; then
        build_rust_core
        build_protobuf
        copy_android_libs
        copy_macos_libs
    fi

    if [ "$android_only" = true ] || [ "$rust_only" = false ] && [ "$android_only" = false ] && [ "$macos_only" = false ]; then
        build_android
    fi

    if [ "$macos_only" = true ] || [ "$rust_only" = false ] && [ "$android_only" = false ] && [ "$macos_only" = false ]; then
        build_macos
    fi

    # 运行测试
    if [ "$skip_tests" = false ]; then
        run_tests
    fi

    # 完成
    echo ""
    echo "=================================================="
    log_success "🎉 NearClip 构建完成！"
    echo "=================================================="

    # 显示输出位置
    echo ""
    echo "构建输出："
    if [ -d "$ANDROID_DIR/app/src/main/jniLibs" ]; then
        echo "  Android 库: $ANDROID_DIR/app/src/main/jniLibs/"
    fi
    if [ -d "$MAC_DIR/NearClip/Frameworks" ]; then
        echo "  macOS 库: $MAC_DIR/NearClip/Frameworks/"
    fi
    if [ -f "$ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk" ]; then
        echo "  Android APK: $ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
    fi
    if [ -d "$MAC_DIR/build/Build/Products/Debug/NearClip.app" ]; then
        echo "  macOS 应用: $MAC_DIR/build/Build/Products/Debug/NearClip.app"
    fi
}

# 错误处理
trap 'log_error "构建过程中发生错误，退出码: $?"' ERR

# 运行主函数
main "$@"