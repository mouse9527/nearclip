#!/bin/bash

# NearClip 原型构建脚本
# 用于构建测试原型验证核心功能

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROTOTYPE_DIR="$PROJECT_ROOT/prototype"
SHARED_RUST_DIR="$PROJECT_ROOT/src/shared/rust"

# 日志函数
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }

echo "=================================================="
echo "🧪 NearClip 原型构建脚本"
echo "=================================================="

# 检查基础环境
check_environment() {
    log_info "检查构建环境..."

    if ! command -v rustc &> /dev/null; then
        log_error "Rust 未安装，请先安装 Rust 工具链"
        exit 1
    fi

    if ! command -v protoc &> /dev/null; then
        log_error "protoc 未安装，请先安装 Protocol Buffers"
        exit 1
    fi

    log_success "环境检查通过"
}

# 构建 Rust 核心库（测试版本）
build_rust_core() {
    log_info "构建 Rust 核心库（测试版本）..."

    cd "$SHARED_RUST_DIR"

    # 开发模式构建，包含调试信息
    cargo build

    if [ $? -eq 0 ]; then
        log_success "Rust 核心库构建成功"

        # 检查生成的库文件
        local lib_path="target/debug/libnearclip.dylib"
        if [ -f "$lib_path" ]; then
            log_success "Rust 库文件生成: $lib_path"
        else
            log_warning "Rust 库文件未找到，可能需要调整构建配置"
        fi
    else
        log_error "Rust 核心库构建失败"
        exit 1
    fi
}

# 生成 Protocol Buffers 测试代码
generate_protobuf() {
    log_info "生成 Protocol Buffers 代码..."

    local proto_dir="$PROJECT_ROOT/src/shared/protobuf/proto"
    local output_dir="$PROTOTYPE_DIR/generated"

    mkdir -p "$output_dir"

    # 生成 Rust 代码
    log_info "生成 Rust Protocol Buffers 代码..."
    protoc --rust_out="$output_dir" \
           -I "$proto_dir" \
           "$proto_dir"/*.proto

    if [ $? -eq 0 ]; then
        log_success "Rust Protocol Buffers 代码生成成功"
    else
        log_error "Protocol Buffers 代码生成失败"
        exit 1
    fi
}

# 创建测试配置
create_test_config() {
    log_info "创建测试配置..."

    # 创建测试配置文件
    cat > "$PROTOTYPE_DIR/test-config.json" << EOF
{
  "test_settings": {
    "ble_timeout_seconds": 30,
    "sync_timeout_seconds": 10,
    "test_text_content": "Hello NearClip Prototype! 🚀",
    "test_device_name": "NearClip-Test-Device",
    "max_retry_attempts": 3
  },
  "devices": {
    "android": {
      "package_name": "com.nearclip.prototype",
      "main_activity": "com.nearclip.MainActivity"
    },
    "macos": {
      "bundle_id": "com.nearclip.prototype",
      "app_name": "NearClip Prototype"
    }
  },
  "logging": {
    "level": "debug",
    "file_path": "$PROTOTYPE_DIR/logs/prototype.log"
  }
}
EOF

    log_success "测试配置创建完成"
}

# 创建测试日志目录
create_log_directory() {
    mkdir -p "$PROTOTYPE_DIR/logs"
    log_success "日志目录创建完成: $PROTOTYPE_DIR/logs"
}

# 创建测试脚本
create_test_scripts() {
    log_info "创建测试脚本..."

    # 创建 Android 测试脚本
    cat > "$PROTOTYPE_DIR/test-android.sh" << 'EOF'
#!/bin/bash

echo "🤖 启动 Android 原型测试..."

# 检查 Android 环境
if [ -z "$ANDROID_HOME" ]; then
    echo "❌ ANDROID_HOME 环境变量未设置"
    exit 1
fi

# 启动 Android 模拟器（如果未运行）
if ! adb devices | grep -q "emulator"; then
    echo "📱 启动 Android 模拟器..."
    emulator @nearclip-test-avd &
    sleep 10
fi

# 安装测试应用
echo "📦 安装测试应用..."
adb install -r "$PROJECT_ROOT/src/platform/android/app/build/outputs/apk/debug/app-debug.apk"

# 启动应用
echo "🚀 启动 NearClip 原型应用..."
adb shell am start -n com.nearclip/.MainActivity

echo "✅ Android 测试环境准备完成"
EOF

    # 创建 macOS 测试脚本
    cat > "$PROTOTYPE_DIR/test-macos.sh" << 'EOF'
#!/bin/bash

echo "🍎 启动 macOS 原型测试..."

# 检查 macOS 应用
if [ ! -d "$PROJECT_ROOT/src/platform/mac/build/Build/Products/Debug/NearClip.app" ]; then
    echo "❌ macOS 应用未找到，请先构建应用"
    echo "运行: ./scripts/build-macos.sh"
    exit 1
fi

# 启动应用
echo "🚀 启动 NearClip 原型应用..."
open "$PROJECT_ROOT/src/platform/mac/build/Build/Products/Debug/NearClip.app"

echo "✅ macOS 测试环境准备完成"
EOF

    chmod +x "$PROTOTYPE_DIR/test-android.sh"
    chmod +x "$PROTOTYPE_DIR/test-macos.sh"

    log_success "测试脚本创建完成"
}

# 创建原型验证报告模板
create_test_report_template() {
    log_info "创建测试报告模板..."

    cat > "$PROTOTYPE_DIR/test-report-template.md" << 'EOF'
# NearClip 原型验证报告

## 测试环境
- 测试时间: [填写测试日期]
- 测试设备: [填写设备信息]
- 操作系统: [填写系统版本]
- 构建版本: [填写构建版本]

## 功能验证结果

### 1. 设备发现
- [ ] Android 设备发现功能
- [ ] macOS 设备发现功能
- [ ] 设备识别准确性
- [ ] 发现时间性能

### 2. 设备配对
- [ ] QR码生成和扫描
- [ ] 公钥加密配对
- [ ] 配对码验证
- [ ] 配对状态管理

### 3. 数据同步
- [ ] 文本内容同步
- [ ] 同步延迟测试
- [ ] 数据完整性验证
- [ ] 冲突处理测试

### 4. 错误处理
- [ ] 网络中断处理
- [ ] 设备断连处理
- [ ] 应用崩溃恢复
- [ ] 用户提示信息

## 性能测试结果

| 指标 | 目标值 | 实际值 | 状态 |
|------|--------|--------|------|
| 设备发现时间 | < 5秒 |  |  |
| 配对完成时间 | < 10秒 |  |  |
| 文本同步延迟 | < 2秒 |  |  |
| 连接成功率 | > 95% |  |  |

## 问题和建议

### 发现的问题
1. [问题描述]
   - 严重程度: [高/中/低]
   - 重现步骤: [步骤描述]
   - 影响范围: [影响描述]

### 改进建议
1. [建议描述]
   - 优先级: [高/中/低]
   - 预期效果: [效果描述]

## 结论
- [ ] 原型验证通过，可以进入下一阶段
- [ ] 需要调整架构设计
- [ ] 需要解决关键问题
- [ ] 建议重新评估需求

## 下一步行动
1. [行动项 1]
2. [行动项 2]
3. [行动项 3]
EOF

    log_success "测试报告模板创建完成"
}

# 主函数
main() {
    log_info "开始构建 NearClip 原型..."

    # 创建原型目录
    mkdir -p "$PROTOTYPE_DIR"

    # 执行构建步骤
    check_environment
    build_rust_core
    generate_protobuf
    create_test_config
    create_log_directory
    create_test_scripts
    create_test_report_template

    echo ""
    echo "=================================================="
    log_success "🎉 NearClip 原型构建完成！"
    echo "=================================================="
    echo ""
    echo "📁 原型目录: $PROTOTYPE_DIR"
    echo "🧪 测试配置: $PROTOTYPE_DIR/test-config.json"
    echo "📋 测试计划: $PROTOTYPE_DIR/test-plan.md"
    echo "📊 测试报告模板: $PROTOTYPE_DIR/test-report-template.md"
    echo ""
    echo "🚀 下一步操作:"
    echo "  1. 运行测试: $PROTOTYPE_DIR/test-android.sh"
    echo "  2. 运行测试: $PROTOTYPE_DIR/test-macos.sh"
    echo "  3. 记录结果: $PROTOTYPE_DIR/test-report-template.md"
    echo ""
    echo "🔍 验证重点:"
    echo "  • BLE 设备发现和连接"
    echo "  • Protocol Buffers 消息序列化"
    echo "  • Rust FFI 接口集成"
    echo "  • 基础数据同步功能"
}

# 运行主函数
main "$@"