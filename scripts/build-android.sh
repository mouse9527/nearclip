#!/bin/bash

# NearClip Android 构建脚本
# 专门用于构建 Android 平台

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SHARED_RUST_DIR="$PROJECT_ROOT/shared/rust"
ANDROID_DIR="$PROJECT_ROOT/android"

# 日志函数
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# 检查 Android 构建环境
check_android_environment() {
    log_info "检查 Android 构建环境..."

    # 检查 Java
    if ! command -v java &> /dev/null; then
        log_error "Java 未安装"
        return 1
    fi

    # 检查 Android SDK
    if [ -z "$ANDROID_HOME" ]; then
        log_warning "ANDROID_HOME 环境变量未设置"
        if [ -d "$HOME/Library/Android/sdk" ]; then
            export ANDROID_HOME="$HOME/Library/Android/sdk"
            log_info "使用默认 Android SDK: $ANDROID_HOME"
        else
            log_error "无法找到 Android SDK"
            return 1
        fi
    fi

    # 检查 Gradle
    if [ -f "$ANDROID_DIR/gradlew" ]; then
        log_info "使用项目 Gradle Wrapper"
        GRADLE_CMD="$ANDROID_DIR/gradlew"
    elif command -v gradle &> /dev/null; then
        log_info "使用系统 Gradle"
        GRADLE_CMD="gradle"
    else
        log_error "Gradle 未找到"
        return 1
    fi

    log_success "Android 构建环境检查完成"
}

# 为 Android 构建 Rust 库
build_rust_for_android() {
    log_info "为 Android 构建 Rust 核心库..."

    cd "$SHARED_RUST_DIR"

    # Android 目标架构
    declare -A android_targets=(
        ["arm64-v8a"]="aarch64-linux-android"
        ["armeabi-v7a"]="armv7-linux-androideabi"
        ["x86_64"]="x86_64-linux-android"
        ["x86"]="i686-linux-android"
    )

    local android_libs_dir="$ANDROID_DIR/app/src/main/jniLibs"
    rm -rf "$android_libs_dir"
    mkdir -p "$android_libs_dir"

    for arch_dir in "${!android_targets[@]}"; do
        local target="${android_targets[$arch_dir]}"
        local lib_path="target/$target/release/libnearclip.so"

        log_info "构建 $arch_dir ($target)..."

        # 安装目标平台
        if ! rustup target list --installed | grep -q "$target"; then
            log_info "安装目标平台: $target"
            rustup target add "$target"
        fi

        # 构建
        cargo build --release --target="$target"

        # 复制库文件
        if [ -f "$lib_path" ]; then
            mkdir -p "$android_libs_dir/$arch_dir"
            cp "$lib_path" "$android_libs_dir/$arch_dir/"
            log_success "✅ $arch_dir 构建成功"
        else
            log_error "❌ $arch_dir 构建失败: $lib_path 不存在"
            return 1
        fi
    done

    log_success "Android Rust 库构建完成"
}

# 生成 Android Protocol Buffers
generate_android_protobuf() {
    log_info "生成 Android Protocol Buffers 代码..."

    cd "$PROJECT_ROOT/shared/protobuf"

    # 检查 protoc 和 gRPC 插件
    if ! command -v protoc &> /dev/null; then
        log_error "protoc 未安装"
        return 1
    fi

    # 创建输出目录
    local output_dir="$ANDROID_DIR/app/src/main/java"
    mkdir -p "$output_dir"

    # 生成 Java 代码
    protoc --java_out="$output_dir" \
           --grpc-java_out="$output_dir" \
           -I proto \
           proto/*.proto

    log_success "Android Protocol Buffers 代码生成完成"
}

# 构建 Android 应用
build_android_app() {
    log_info "构建 Android 应用..."

    cd "$ANDROID_DIR"

    # 检查构建配置
    if [ ! -f "app/build.gradle" ]; then
        log_error "Android 项目配置文件不存在"
        return 1
    fi

    # 清理之前的构建
    log_info "清理之前的构建..."
    $GRADLE_CMD clean

    # 构建 Debug 版本
    log_info "构建 Debug 版本..."
    $GRADLE_CMD assembleDebug

    if [ $? -eq 0 ]; then
        log_success "✅ Android Debug 版本构建成功"

        # 显示 APK 位置
        local apk_path="$ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
        if [ -f "$apk_path" ]; then
            log_info "APK 位置: $apk_path"
            log_info "APK 大小: $(du -h "$apk_path" | cut -f1)"
        fi
    else
        log_error "❌ Android Debug 版本构建失败"
        return 1
    fi

    # 构建 Release 版本（如果签名配置可用）
    if grep -q "signingConfigs.release" "app/build.gradle"; then
        log_info "构建 Release 版本..."
        $GRADLE_CMD assembleRelease

        if [ $? -eq 0 ]; then
            log_success "✅ Android Release 版本构建成功"

            local release_apk_path="$ANDROID_DIR/app/build/outputs/apk/release/app-release.apk"
            if [ -f "$release_apk_path" ]; then
                log_info "Release APK 位置: $release_apk_path"
                log_info "Release APK 大小: $(du -h "$release_apk_path" | cut -f1)"
            fi
        else
            log_warning "⚠️  Android Release 版本构建失败（可能需要签名配置）"
        fi
    else
        log_info "跳过 Release 版本构建（未找到签名配置）"
    fi
}

# 运行 Android 测试
run_android_tests() {
    log_info "运行 Android 测试..."

    cd "$ANDROID_DIR"

    # 单元测试
    log_info "运行单元测试..."
    $GRADLE_CMD testDebugUnitTest

    if [ $? -eq 0 ]; then
        log_success "✅ Android 单元测试通过"
    else
        log_warning "⚠️  Android 单元测试失败"
    fi

    # 如果有连接的设备，运行仪器测试
    if command -v adb &> /dev/null && adb devices | grep -q "device$"; then
        log_info "运行仪器测试..."
        $GRADLE_TASK connectedDebugAndroidTest

        if [ $? -eq 0 ]; then
            log_success "✅ Android 仪器测试通过"
        else
            log_warning "⚠️  Android 仪器测试失败"
        fi
    else
        log_info "跳过仪器测试（未检测到连接的设备）"
    fi
}

# 安装 APK 到设备
install_apk() {
    local apk_type=${1:-debug}

    if ! command -v adb &> /dev/null; then
        log_error "adb 未找到，请安装 Android SDK Platform Tools"
        return 1
    fi

    local apk_path
    if [ "$apk_type" = "release" ]; then
        apk_path="$ANDROID_DIR/app/build/outputs/apk/release/app-release.apk"
    else
        apk_path="$ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
    fi

    if [ ! -f "$apk_path" ]; then
        log_error "APK 文件不存在: $apk_path"
        return 1
    fi

    log_info "安装 $apk_type APK 到设备..."
    adb install "$apk_path"

    if [ $? -eq 0 ]; then
        log_success "✅ APK 安装成功"
    else
        log_error "❌ APK 安装失败"
        return 1
    fi
}

# 显示帮助
show_help() {
    echo "NearClip Android 构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -h, --help      显示此帮助信息"
    echo "  -r, --rust      仅构建 Rust 库"
    echo "  -p, --protobuf  仅生成 Protocol Buffers"
    echo "  -b, --build     仅构建 Android 应用"
    echo "  -t, --test      仅运行测试"
    echo "  -i, --install   安装 APK 到设备"
    echo "  --release       使用 Release 版本"
    echo "  --clean         清理构建文件"
    echo ""
    echo "示例:"
    echo "  $0                    # 完整构建"
    echo "  $0 --rust            # 仅构建 Rust 库"
    echo "  $0 --install         # 构建并安装 Debug APK"
    echo "  $0 --install --release # 构建并安装 Release APK"
}

# 主函数
main() {
    local rust_only=false
    local protobuf_only=false
    local build_only=false
    local test_only=false
    local install_apk=false
    local use_release=false
    local clean_only=false

    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -r|--rust)
                rust_only=true
                shift
                ;;
            -p|--protobuf)
                protobuf_only=true
                shift
                ;;
            -b|--build)
                build_only=true
                shift
                ;;
            -t|--test)
                test_only=true
                shift
                ;;
            -i|--install)
                install_apk=true
                shift
                ;;
            --release)
                use_release=true
                shift
                ;;
            --clean)
                clean_only=true
                shift
                ;;
            *)
                log_error "未知选项: $1"
                show_help
                exit 1
                ;;
        esac
    done

    echo "=================================================="
    echo "🤖 NearClip Android 构建脚本"
    echo "=================================================="
    echo ""

    # 检查环境
    check_android_environment

    # 清理模式
    if [ "$clean_only" = true ]; then
        cd "$ANDROID_DIR"
        $GRADLE_CMD clean
        rm -rf "$ANDROID_DIR/app/src/main/jniLibs"
        log_success "清理完成"
        exit 0
    fi

    # 测试模式
    if [ "$test_only" = true ]; then
        run_android_tests
        exit 0
    fi

    # 安装模式
    if [ "$install_apk" = true ]; then
        # 先构建（如果 APK 不存在）
        local apk_type="debug"
        if [ "$use_release" = true ]; then
            apk_type="release"
        fi

        local apk_path="$ANDROID_DIR/app/build/outputs/apk/$apk_type/app-$apk_type.apk"
        if [ ! -f "$apk_path" ]; then
            log_info "APK 不存在，开始构建..."
            build_rust_for_android
            generate_android_protobuf
            build_android_app
        fi

        install_apk "$apk_type"
        exit 0
    fi

    # 构建流程
    if [ "$rust_only" = true ] || [ "$protobuf_only" = true ] || [ "$build_only" = true ]; then
        # 单独构建
        if [ "$rust_only" = true ]; then
            build_rust_for_android
        elif [ "$protobuf_only" = true ]; then
            generate_android_protobuf
        elif [ "$build_only" = true ]; then
            build_android_app
        fi
    else
        # 完整构建流程
        build_rust_for_android
        generate_android_protobuf
        build_android_app
        run_android_tests
    fi

    echo ""
    echo "=================================================="
    log_success "🎉 Android 构建完成！"
    echo "=================================================="

    # 显示输出信息
    echo ""
    echo "构建输出："
    echo "  Rust 库: $ANDROID_DIR/app/src/main/jniLibs/"
    echo "  Java 代码: $ANDROID_DIR/app/src/main/java/com/nearclip/"

    if [ -f "$ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk" ]; then
        echo "  Debug APK: $ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
    fi

    if [ -f "$ANDROID_DIR/app/build/outputs/apk/release/app-release.apk" ]; then
        echo "  Release APK: $ANDROID_DIR/app/build/outputs/apk/release/app-release.apk"
    fi
}

# 错误处理
trap 'log_error "构建过程中发生错误，退出码: $?"' ERR

# 运行主函数
main "$@"