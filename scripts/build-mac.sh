#!/bin/bash

# NearClip Mac Build Script
# Usage: ./scripts/build-mac.sh

set -e

echo "🔨 Building NearClip Mac App..."

# Navigate to Mac project directory
cd "$(dirname "$0")/../src/platform/mac"

# Check if Swift is available
if ! command -v swift &> /dev/null; then
    echo "❌ Swift is not installed. Please install Xcode or Swift toolchain."
    exit 1
fi

echo "📦 Resolving dependencies..."
swift package resolve

echo "🔧 Building NearClip..."
swift build

echo "✅ Mac build completed successfully!"
echo "🖥️ Executable location: .build/release/NearClip"