#!/bin/bash

# NearClip Rust Shared Library Build Script
# Usage: ./scripts/build-rust.sh

set -e

echo "🦀 Building NearClip Rust Shared Library..."

# Navigate to Rust project directory
cd "$(dirname "$0")/../src/shared/rust"

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "📦 Building shared library for all targets..."

# Build for current platform (development)
echo "🔧 Building for development platform..."
cargo build

# Build for Android ARM64
echo "🔧 Building for Android ARM64..."
cargo build --target aarch64-linux-android

# Build for macOS x64 and ARM64
echo "🔧 Building for macOS x64..."
cargo build --target x86_64-apple-darwin

echo "🔧 Building for macOS ARM64..."
cargo build --target aarch64-apple-darwin

echo "✅ Rust build completed successfully!"
echo "📚 Library locations:"
echo "   - Development: target/debug/libnearclip_core.*"
echo "   - Android ARM64: target/aarch64-linux-android/debug/libnearclip_core.*"
echo "   - macOS x64: target/x86_64-apple-darwin/debug/libnearclip_core.*"
echo "   - macOS ARM64: target/aarch64-apple-darwin/debug/libnearclip_core.*"