#!/bin/bash

# NearClip macOS 构建脚本
# 专门用于构建 macOS 平台

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
MAC_DIR="$PROJECT_ROOT/mac"

# 日志函数
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# 检查 macOS 构建环境
check_macos_environment() {
    log_info "检查 macOS 构建环境..."

    # 检查操作系统
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_error "此脚本只能在 macOS 上运行"
        return 1
    fi

    # 检查 Xcode 命令行工具
    if ! command -v xcodebuild &> /dev/null; then
        log_error "Xcode 命令行工具未安装"
        log_info "请运行: xcode-select --install"
        return 1
    fi

    # 检查 Xcode
    if ! xcodebuild -version &> /dev/null; then
        log_error "Xcode 未正确安装"
        return 1
    fi

    # 显示 Xcode 版本
    local xcode_version=$(xcodebuild -version | head -n 1)
    log_info "使用 $xcode_version"

    # 检查 Swift
    if ! command -v swift &> /dev/null; then
        log_error "Swift 未找到"
        return 1
    fi

    local swift_version=$(swift --version | head -n 1)
    log_info "使用 $swift_version"

    log_success "macOS 构建环境检查完成"
}

# 为 macOS 构建 Rust 库
build_rust_for_macos() {
    log_info "为 macOS 构建 Rust 核心库..."

    cd "$SHARED_RUST_DIR"

    # macOS 目标架构
    local macos_targets=(
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )

    local mac_frameworks_dir="$MAC_DIR/NearClip/Frameworks"
    rm -rf "$mac_frameworks_dir"
    mkdir -p "$mac_frameworks_dir"

    local built_libs=()

    for target in "${macos_targets[@]}"; do
        log_info "构建 $target..."

        # 安装目标平台
        if ! rustup target list --installed | grep -q "$target"; then
            log_info "安装目标平台: $target"
            rustup target add "$target"
        fi

        # 构建
        cargo build --release --target="$target"

        local lib_path="target/$target/release/libnearclip.dylib"
        if [ -f "$lib_path" ]; then
            built_libs+=("$lib_path")
            log_success "✅ $target 构建成功"
        else
            log_error "❌ $target 构建失败: $lib_path 不存在"
            return 1
        fi
    done

    # 创建通用二进制文件（如果支持）
    if [ ${#built_libs[@]} -eq 2 ]; then
        log_info "创建通用二进制文件..."
        lipo -create "${built_libs[0]}" "${built_libs[1]}" \
              -output "$mac_frameworks_dir/libnearclip.dylib"

        if [ $? -eq 0 ]; then
            log_success "✅ 通用二进制文件创建成功"

            # 显示架构信息
            local archs=$(lipo -info "$mac_frameworks_dir/libnearclip.dylib")
            log_info "架构: $archs"
        else
            log_warning "⚠️  通用二进制文件创建失败，使用单独的库文件"
            cp "${built_libs[0]}" "$mac_frameworks_dir/"
        fi
    elif [ ${#built_libs[@]} -eq 1 ]; then
        cp "${built_libs[0]}" "$mac_frameworks_dir/"
        log_info "使用单一架构库文件"
    fi

    # 设置库文件权限
    chmod 755 "$mac_frameworks_dir/libnearclip.dylib"

    log_success "macOS Rust 库构建完成"
}

# 生成 macOS Protocol Buffers
generate_macos_protobuf() {
    log_info "生成 macOS Protocol Buffers 代码..."

    cd "$PROJECT_ROOT/shared/protobuf"

    # 检查 protoc
    if ! command -v protoc &> /dev/null; then
        log_error "protoc 未安装"
        log_info "请运行: brew install protobuf"
        return 1
    fi

    # 检查 SwiftProtobuf 插件
    if ! command -v protoc-gen-swift &> /dev/null; then
        log_warning "SwiftProtobuf 插件未找到，使用基础 protoc"
        local use_swift_grpc=false
    else
        log_info "使用 SwiftProtobuf 插件"
        local use_swift_grpc=true
    fi

    # 创建输出目录
    local output_dir="$MAC_DIR/NearClip/Generated"
    mkdir -p "$output_dir"

    # 生成 Swift 代码
    if [ "$use_swift_grpc" = true ]; then
        protoc --swift_out="$output_dir" \
               --grpc-swift_out="$output_dir" \
               -I proto \
               proto/*.proto
    else
        protoc --swift_out="$output_dir" \
               -I proto \
               proto/*.proto
    fi

    log_success "macOS Protocol Buffers 代码生成完成"
}

# 构建 macOS 应用
build_macos_app() {
    log_info "构建 macOS 应用..."

    cd "$MAC_DIR"

    # 检查项目文件
    if [ ! -f "NearClip.xcodeproj/project.pbxproj" ]; then
        log_error "Xcode 项目文件不存在"
        return 1
    fi

    # 创建构建目录
    local build_dir="build"
    mkdir -p "$build_dir"

    # 获取可用的 scheme
    local schemes=$(xcodebuild -project NearClip.xcodeproj -list | sed -n '/Schemes:/,/^\s*$/p' | grep -v 'Schemes:' | grep -v '^\s*$' | tr -d ' ')
    local scheme="NearClip"

    if ! echo "$schemes" | grep -q "$scheme"; then
        log_error "未找到 scheme: $scheme"
        log_info "可用的 schemes: $schemes"
        return 1
    fi

    log_info "使用 scheme: $scheme"

    # 构建 Debug 版本
    log_info "构建 Debug 版本..."
    xcodebuild -project NearClip.xcodeproj \
               -scheme "$scheme" \
               -configuration Debug \
               -derivedDataPath "$build_dir" \
               ONLY_ACTIVE_ARCH=NO

    if [ $? -eq 0 ]; then
        log_success "✅ macOS Debug 版本构建成功"

        # 显示应用位置
        local app_path="$build_dir/Build/Products/Debug/NearClip.app"
        if [ -d "$app_path" ]; then
            log_info "应用位置: $app_path"
            log_info "应用大小: $(du -sh "$app_path" | cut -f1)"
        fi
    else
        log_error "❌ macOS Debug 版本构建失败"
        return 1
    fi

    # 构建 Release 版本
    log_info "构建 Release 版本..."
    xcodebuild -project NearClip.xcodeproj \
               -scheme "$scheme" \
               -configuration Release \
               -derivedDataPath "$build_dir" \
               ONLY_ACTIVE_ARCH=NO

    if [ $? -eq 0 ]; then
        log_success "✅ macOS Release 版本构建成功"

        # 显示应用位置
        local release_app_path="$build_dir/Build/Products/Release/NearClip.app"
        if [ -d "$release_app_path" ]; then
            log_info "Release 应用位置: $release_app_path"
            log_info "Release 应用大小: $(du -sh "$release_app_path" | cut -f1)"
        fi
    else
        log_warning "⚠️  macOS Release 版本构建失败"
    fi
}

# 运行 macOS 测试
run_macos_tests() {
    log_info "运行 macOS 测试..."

    cd "$MAC_DIR"

    # 单元测试
    log_info "运行单元测试..."
    xcodebuild test \
        -project NearClip.xcodeproj \
        -scheme NearClip \
        -destination 'platform=macOS' \
        -derivedDataPath build

    if [ $? -eq 0 ]; then
        log_success "✅ macOS 单元测试通过"
    else
        log_warning "⚠️  macOS 单元测试失败"
    fi
}

# 创建应用包
create_app_package() {
    log_info "创建应用包..."

    local build_dir="$MAC_DIR/build"
    local app_name="NearClip"
    local package_dir="$MAC_DIR/package"

    mkdir -p "$package_dir"

    # 复制 Release 应用
    local release_app="$build_dir/Build/Products/Release/$app_name.app"
    if [ -d "$release_app" ]; then
        cp -R "$release_app" "$package_dir/"
        log_success "✅ 应用复制到包目录"
    else
        local debug_app="$build_dir/Build/Products/Debug/$app_name.app"
        if [ -d "$debug_app" ]; then
            cp -R "$debug_app" "$package_dir/"
            log_info "使用 Debug 版本创建包"
        else
            log_error "未找到构建的应用"
            return 1
        fi
    fi

    # 创建 DMG（可选）
    if command -v hdiutil &> /dev/null; then
        log_info "创建 DMG 镜像..."

        local dmg_path="$package_dir/$app_name.dmg"
        local temp_dmg="$package_dir/temp.dmg"

        # 创建临时 DMG
        hdiutil create -size 100m -volname "$app_name" -fs HFS+ "$temp_dmg"

        # 挂载临时 DMG
        local mount_dir=$(hdiutil attach "$temp_dmg" | grep -o '/Volumes/.*')

        # 复制应用
        cp -R "$package_dir/$app_name.app" "$mount_dir/"

        # 卸载并创建最终 DMG
        hdiutil detach "$mount_dir"
        hdiutil convert "$temp_dmg" -format UDZO -o "$dmg_path"
        rm "$temp_dmg"

        log_success "✅ DMG 创建完成: $dmg_path"
    fi

    log_success "应用包创建完成: $package_dir"
}

# 运行应用
run_app() {
    local app_path="$MAC_DIR/build/Build/Products/Debug/NearClip.app"

    if [ ! -d "$app_path" ]; then
        local release_app_path="$MAC_DIR/build/Build/Products/Release/NearClip.app"
        if [ -d "$release_app_path" ]; then
            app_path="$release_app_path"
        else
            log_error "未找到构建的应用"
            return 1
        fi
    fi

    log_info "启动应用: $app_path"
    open "$app_path"
}

# 显示帮助
show_help() {
    echo "NearClip macOS 构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -h, --help      显示此帮助信息"
    echo "  -r, --rust      仅构建 Rust 库"
    echo "  -p, --protobuf  仅生成 Protocol Buffers"
    echo "  -b, --build     仅构建 macOS 应用"
    echo "  -t, --test      仅运行测试"
    echo "  -k, --package   创建应用包"
    echo "  -n, --run       运行应用"
    echo "  --clean         清理构建文件"
    echo ""
    echo "示例:"
    echo "  $0              # 完整构建"
    echo "  $0 --rust       # 仅构建 Rust 库"
    echo "  $0 --run        # 构建并运行应用"
    echo "  $0 --package    # 创建应用包"
}

# 主函数
main() {
    local rust_only=false
    local protobuf_only=false
    local build_only=false
    local test_only=false
    local create_package=false
    local run_app_only=false
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
            -k|--package)
                create_package=true
                shift
                ;;
            -n|--run)
                run_app_only=true
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
    echo "🍎 NearClip macOS 构建脚本"
    echo "=================================================="
    echo ""

    # 检查环境
    check_macos_environment

    # 清理模式
    if [ "$clean_only" = true ]; then
        rm -rf "$MAC_DIR/build"
        rm -rf "$MAC_DIR/package"
        rm -rf "$MAC_DIR/NearClip/Frameworks"
        log_success "清理完成"
        exit 0
    fi

    # 测试模式
    if [ "$test_only" = true ]; then
        run_macos_tests
        exit 0
    fi

    # 运行模式
    if [ "$run_app_only" = true ]; then
        run_app
        exit 0
    fi

    # 构建流程
    if [ "$rust_only" = true ] || [ "$protobuf_only" = true ] || [ "$build_only" = true ]; then
        # 单独构建
        if [ "$rust_only" = true ]; then
            build_rust_for_macos
        elif [ "$protobuf_only" = true ]; then
            generate_macos_protobuf
        elif [ "$build_only" = true ]; then
            build_macos_app
        fi
    else
        # 完整构建流程
        build_rust_for_macos
        generate_macos_protobuf
        build_macos_app
        run_macos_tests
    fi

    # 创建应用包
    if [ "$create_package" = true ]; then
        create_app_package
    fi

    echo ""
    echo "=================================================="
    log_success "🎉 macOS 构建完成！"
    echo "=================================================="

    # 显示输出信息
    echo ""
    echo "构建输出："
    echo "  Rust 库: $MAC_DIR/NearClip/Frameworks/"
    echo "  Swift 代码: $MAC_DIR/NearClip/Generated/"

    if [ -d "$MAC_DIR/build/Build/Products/Debug/NearClip.app" ]; then
        echo "  Debug 应用: $MAC_DIR/build/Build/Products/Debug/NearClip.app"
    fi

    if [ -d "$MAC_DIR/build/Build/Products/Release/NearClip.app" ]; then
        echo "  Release 应用: $MAC_DIR/build/Build/Products/Release/NearClip.app"
    fi

    if [ -d "$MAC_DIR/package" ]; then
        echo "  应用包: $MAC_DIR/package/"
    fi

    # 提供运行选项
    if [ ! "$rust_only" = true ] && [ ! "$protobuf_only" = true ]; then
        echo ""
        log_info "运行应用: $0 --run"
    fi
}

# 错误处理
trap 'log_error "构建过程中发生错误，退出码: $?"' ERR

# 运行主函数
main "$@"