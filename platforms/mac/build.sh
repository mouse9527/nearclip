#!/bin/bash

# NearClip macOS Build Script
# 用于构建和运行 macOS 应用

set -e

echo "🚀 Building NearClip for macOS..."

# 检查 Xcode 是否安装
if ! command -v xcodebuild &> /dev/null; then
    echo "❌ Error: Xcode is not installed or xcodebuild is not in PATH"
    exit 1
fi

# 进入项目目录
cd "$(dirname "$0")"

# 清理之前的构建
echo "🧹 Cleaning previous build..."
xcodebuild clean -project NearClip.xcodeproj -scheme NearClip

# 构建项目
echo "🔨 Building project..."
xcodebuild build -project NearClip.xcodeproj -scheme NearClip -configuration Debug

echo "✅ Build completed successfully!"

# 可选：运行应用
if [ "$1" = "--run" ]; then
    echo "🏃 Running NearClip..."
    APP_PATH=$(find ~/Library/Developer/Xcode/DerivedData -name "NearClip.app" -type d 2>/dev/null | head -1)
    if [ -n "$APP_PATH" ]; then
        open "$APP_PATH"
        echo "✅ NearClip launched successfully!"
    else
        echo "❌ Could not find NearClip.app"
        exit 1
    fi
fi

echo "📱 NearClip for macOS is ready!"