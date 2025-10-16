#!/bin/bash

# NearClip 开发环境验证脚本
# 检查所有必需的依赖和配置

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# 统计变量
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# 日志函数
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; ((PASSED_CHECKS++)); }
log_warning() { echo -e "${YELLOW}[⚠]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; ((FAILED_CHECKS++)); }

# 检查函数
check_command() {
    local cmd=$1
    local name=$2
    local min_version=${3:-""}

    ((TOTAL_CHECKS++))

    if command -v "$cmd" &> /dev/null; then
        if [ -n "$min_version" ]; then
            local version=$($cmd --version 2>/dev/null | head -n1 | grep -oE '[0-9]+\.[0-9]+' | head -n1)
            if [ "$version" \< "$min_version" ]; then
                log_error "$name (版本 $version < 最低要求 $min_version)"
                return 1
            fi
        fi
        log_success "$name 已安装"
        return 0
    else
        log_error "$name 未安装"
        return 1
    fi
}

check_file() {
    local file=$1
    local name=$2

    ((TOTAL_CHECKS++))

    if [ -f "$file" ]; then
        log_success "$name 存在"
        return 0
    else
        log_error "$name 不存在"
        return 1
    fi
}

check_directory() {
    local dir=$1
    local name=$2

    ((TOTAL_CHECKS++))

    if [ -d "$dir" ]; then
        log_success "$name 存在"
        return 0
    else
        log_error "$name 不存在"
        return 1
    fi
}

check_env_var() {
    local var=$1
    local name=$2

    ((TOTAL_CHECKS++))

    if [ -n "${!var}" ]; then
        log_success "$name 环境变量已设置: ${!var}"
        return 0
    else
        log_warning "$name 环境变量未设置"
        return 1
    fi
}

# 显示标题
echo "=================================================="
echo "🔍 NearClip 开发环境验证"
echo "=================================================="
echo ""

# 基础工具检查
log_info "检查基础开发工具..."

check_command "git" "Git" "2.0"
check_command "curl" "Curl"
check_command "wget" "Wget"

# Rust 工具链
log_info "检查 Rust 工具链..."

if check_command "rustc" "Rust 编译器" "1.70"; then
    # 检查 Rust 目标
    log_info "检查 Rust 目标平台..."

    rust_targets=(
        "aarch64-linux-android"
        "armv7-linux-androideabi"
        "x86_64-linux-android"
        "i686-linux-android"
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )

    for target in "${rust_targets[@]}"; do
        if rustup target list --installed | grep -q "$target"; then
            log_success "Rust 目标 $target 已安装"
            ((PASSED_CHECKS++))
        else
            log_warning "Rust 目标 $target 未安装"
            ((FAILED_CHECKS++))
        fi
        ((TOTAL_CHECKS++))
    done
fi

check_command "cargo" "Cargo 包管理器"

# Protocol Buffers
log_info "检查 Protocol Buffers..."

if check_command "protoc" "Protocol Buffers 编译器" "3.0"; then
    # 检查 protoc 插件
    check_command "protoc-gen-swift" "Swift Protocol Buffers 插件"
    check_command "protoc-gen-grpc-java" "Java gRPC 插件"
fi

# 平台特定工具
log_info "检查平台特定工具..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    check_command "xcodebuild" "Xcode 构建工具"
    check_command "swift" "Swift 编译器"
    check_command "clang" "Clang 编译器"
    check_command "lipo" "Lipo 工具"

    # 检查 Xcode 许可协议
    if xcodebuild -checkFirstLaunchStatus &> /dev/null; then
        log_success "Xcode 许可协议已同意"
        ((PASSED_CHECKS++))
    else
        log_warning "需要运行: sudo xcodebuild -license"
        ((FAILED_CHECKS++))
    fi
    ((TOTAL_CHECKS++))

elif [[ "$OSTYPE" == "linux"* ]]; then
    # Linux
    check_command "gcc" "GCC 编译器"
    check_command "make" "Make"
    check_command "pkg-config" "pkg-config"
fi

# Android 开发工具
log_info "检查 Android 开发工具..."

if check_env_var "ANDROID_HOME" "Android SDK" || check_env_var "ANDROID_SDK_ROOT" "Android SDK"; then
    android_home="${ANDROID_HOME:-$ANDROID_SDK_ROOT}"

    check_directory "$android_home" "Android SDK 目录"
    check_directory "$android_home/platforms" "Android SDK 平台"
    check_directory "$android_home/build-tools" "Android SDK 构建工具"
    check_command "$android_home/platform-tools/adb" "Android Debug Bridge"

    # 检查 Gradle
    if check_command "gradle" "Gradle"; then
        gradle_version=$(gradle --version | grep -oE 'Gradle [0-9]+\.[0-9]+' | cut -d' ' -f2)
        log_success "Gradle 版本: $gradle_version"
    fi
else
    log_warning "Android SDK 未配置，跳过 Android 开发检查"
fi

# 项目结构检查
log_info "检查项目结构..."

project_dirs=(
    "$PROJECT_ROOT/shared/rust"
    "$PROJECT_ROOT/shared/protobuf"
    "$PROJECT_ROOT/android"
    "$PROJECT_ROOT/mac"
    "$PROJECT_ROOT/scripts"
    "$PROJECT_ROOT/docs"
)

for dir in "${project_dirs[@]}"; do
    check_directory "$dir" "项目目录: $(basename "$dir")"
done

# 核心文件检查
log_info "检查核心配置文件..."

core_files=(
    "$PROJECT_ROOT/shared/rust/Cargo.toml"
    "$PROJECT_ROOT/shared/rust/src/lib.rs"
    "$PROJECT_ROOT/android/app/build.gradle"
    "$PROJECT_ROOT/mac/NearClip.xcodeproj/project.pbxproj"
    "$PROJECT_ROOT/scripts/build-all.sh"
)

for file in "${core_files[@]}"; do
    check_file "$file" "核心文件: $(basename "$file")"
done

# Rust 依赖检查
log_info "检查 Rust 依赖..."

if [ -f "$PROJECT_ROOT/shared/rust/Cargo.toml" ]; then
    cd "$PROJECT_ROOT/shared/rust"

    if cargo check --quiet 2>/dev/null; then
        log_success "Rust 依赖完整"
        ((PASSED_CHECKS++))
    else
        log_error "Rust 依赖不完整"
        ((FAILED_CHECKS++))
    fi
    ((TOTAL_CHECKS++))
fi

# 权限检查
log_info "检查文件权限..."

scripts=(
    "$PROJECT_ROOT/scripts/build-all.sh"
    "$PROJECT_ROOT/scripts/build-android.sh"
    "$PROJECT_ROOT/scripts/build-macos.sh"
)

for script in "${scripts[@]}"; do
    if [ -f "$script" ]; then
        if [ -x "$script" ]; then
            log_success "脚本可执行: $(basename "$script")"
            ((PASSED_CHECKS++))
        else
            log_error "脚本不可执行: $(basename "$script")"
            ((FAILED_CHECKS++))
        fi
        ((TOTAL_CHECKS++))
    fi
done

# 网络连接检查
log_info "检查网络连接..."

if curl -s --connect-timeout 5 https://crates.io > /dev/null; then
    log_success "可以访问 Rust crates.io"
    ((PASSED_CHECKS++))
else
    log_warning "无法访问 Rust crates.io"
    ((FAILED_CHECKS++))
fi
((TOTAL_CHECKS++))

if curl -s --connect-timeout 5 https://github.com > /dev/null; then
    log_success "可以访问 GitHub"
    ((PASSED_CHECKS++))
else
    log_warning "无法访问 GitHub"
    ((FAILED_CHECKS++))
fi
((TOTAL_CHECKS++))

# 磁盘空间检查
log_info "检查磁盘空间..."

available_space=$(df -BG "$PROJECT_ROOT" | awk 'NR==2 {print $4}' | sed 's/G//')
if [ "$available_space" -gt 5 ]; then
    log_success "可用磁盘空间: ${available_space}GB"
    ((PASSED_CHECKS++))
else
    log_warning "可用磁盘空间不足: ${available_space}GB (建议至少 5GB)"
    ((FAILED_CHECKS++))
fi
((TOTAL_CHECKS++))

# 显示结果
echo ""
echo "=================================================="
echo "📊 验证结果"
echo "=================================================="

if [ $FAILED_CHECKS -eq 0 ]; then
    log_success "🎉 所有检查通过！环境配置完整。"
    echo ""
    echo "✅ 可以开始 NearClip 开发："
    echo "   ./scripts/build-all.sh          # 构建所有平台"
    echo "   ./scripts/build-android.sh      # 仅构建 Android"
    echo "   ./scripts/build-macos.sh        # 仅构建 macOS"
else
    log_warning "⚠️  发现 $FAILED_CHECKS 个问题需要解决。"
    echo ""
    echo "请根据上述错误信息安装缺失的工具或配置环境。"
fi

echo ""
echo "检查统计："
echo "  总检查数: $TOTAL_CHECKS"
echo "  通过: $PASSED_CHECKS"
echo "  失败: $FAILED_CHECKS"

exit $FAILED_CHECKS