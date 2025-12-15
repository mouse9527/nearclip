#!/bin/bash
# Build Android bindings for NearClip
# This script generates Kotlin bindings and builds native libraries for Android

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$PROJECT_ROOT/target/kotlin"
FFI_CRATE="$PROJECT_ROOT/crates/nearclip-ffi"
UDL_FILE="$FFI_CRATE/src/nearclip.udl"
ANDROID_DIR="$PROJECT_ROOT/android"

# Android targets and their ABI names
# Format: "rust_target:android_abi"
TARGETS=(
    "aarch64-linux-android:arm64-v8a"
    "armv7-linux-androideabi:armeabi-v7a"
    "x86_64-linux-android:x86_64"
)

# Helper function to get target from pair
get_target() { echo "$1" | cut -d: -f1; }
get_abi() { echo "$1" | cut -d: -f2; }

echo "=== NearClip Android Build ==="
echo "Project root: $PROJECT_ROOT"
echo "Output directory: $OUTPUT_DIR"

# Check for Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    # Try common locations
    if [ -d "$HOME/Library/Android/sdk/ndk" ]; then
        # Find the latest NDK version
        ANDROID_NDK_HOME=$(ls -d "$HOME/Library/Android/sdk/ndk"/* 2>/dev/null | sort -V | tail -1)
    elif [ -d "$HOME/Android/Sdk/ndk" ]; then
        ANDROID_NDK_HOME=$(ls -d "$HOME/Android/Sdk/ndk"/* 2>/dev/null | sort -V | tail -1)
    fi
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Step 1: Build uniffi-bindgen binary
echo ""
echo "=== Building uniffi-bindgen ==="
cargo build -p nearclip-ffi --bin uniffi-bindgen --release

UNIFFI_BINDGEN="$PROJECT_ROOT/target/release/uniffi-bindgen"

# Step 2: Generate Kotlin bindings
echo ""
echo "=== Generating Kotlin bindings ==="
"$UNIFFI_BINDGEN" generate "$UDL_FILE" \
    --language kotlin \
    --out-dir "$OUTPUT_DIR" \
    --config "$FFI_CRATE/uniffi.toml"

echo "Generated Kotlin bindings:"
find "$OUTPUT_DIR" -name "*.kt" -type f

# Step 3: Check NDK and compile for Android targets
if [ -n "$ANDROID_NDK_HOME" ] && [ -d "$ANDROID_NDK_HOME" ]; then
    echo ""
    echo "=== Android NDK found: $ANDROID_NDK_HOME ==="

    # Add NDK toolchain to PATH
    NDK_TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt"
    if [ -d "$NDK_TOOLCHAIN/darwin-x86_64" ]; then
        NDK_BIN="$NDK_TOOLCHAIN/darwin-x86_64/bin"
    elif [ -d "$NDK_TOOLCHAIN/darwin-arm64" ]; then
        NDK_BIN="$NDK_TOOLCHAIN/darwin-arm64/bin"
    elif [ -d "$NDK_TOOLCHAIN/linux-x86_64" ]; then
        NDK_BIN="$NDK_TOOLCHAIN/linux-x86_64/bin"
    else
        echo "Warning: Could not find NDK toolchain binaries"
        NDK_BIN=""
    fi

    if [ -n "$NDK_BIN" ] && [ -d "$NDK_BIN" ]; then
        export PATH="$NDK_BIN:$PATH"

        # Install Rust targets if needed
        echo ""
        echo "=== Ensuring Rust Android targets are installed ==="
        for pair in "${TARGETS[@]}"; do
            target=$(get_target "$pair")
            if ! rustup target list --installed | grep -q "$target"; then
                echo "Installing target: $target"
                rustup target add "$target"
            fi
        done

        # Build for each target
        for pair in "${TARGETS[@]}"; do
            target=$(get_target "$pair")
            abi=$(get_abi "$pair")
            echo ""
            echo "=== Building for $target ($abi) ==="

            cargo build --release -p nearclip-ffi --target "$target" || {
                echo "Warning: Failed to build for $target, skipping"
                continue
            }

            # Create output directory for this ABI
            JNI_DIR="$ANDROID_DIR/app/src/main/jniLibs/$abi"
            mkdir -p "$JNI_DIR"

            # Copy the library
            LIB_PATH="$PROJECT_ROOT/target/$target/release/libnearclip_ffi.so"
            if [ -f "$LIB_PATH" ]; then
                cp "$LIB_PATH" "$JNI_DIR/"
                echo "Copied: $LIB_PATH -> $JNI_DIR/"
            else
                echo "Warning: Library not found: $LIB_PATH"
            fi
        done
    else
        echo ""
        echo "=== Skipping native compilation (NDK toolchain not found) ==="
        echo "To enable cross-compilation:"
        echo "  1. Install Android NDK"
        echo "  2. Set ANDROID_NDK_HOME environment variable"
        echo "  3. Or install via Android Studio SDK Manager"
    fi
else
    echo ""
    echo "=== Skipping native compilation (Android NDK not found) ==="
    echo "To enable cross-compilation:"
    echo "  export ANDROID_NDK_HOME=/path/to/ndk"
    echo "  Or install via: Android Studio > SDK Manager > SDK Tools > NDK"
fi

# Step 4: Copy Kotlin source to Android project
echo ""
echo "=== Copying Kotlin bindings to Android project ==="
KOTLIN_SRC_DIR="$ANDROID_DIR/app/src/main/java"
mkdir -p "$KOTLIN_SRC_DIR"

# Copy the generated Kotlin package
if [ -d "$OUTPUT_DIR/com" ]; then
    cp -r "$OUTPUT_DIR/com" "$KOTLIN_SRC_DIR/"
    echo "Copied Kotlin bindings to: $KOTLIN_SRC_DIR/com/nearclip/ffi/"
fi

echo ""
echo "=== Build Complete ==="
echo ""
echo "Output files:"
echo "  Kotlin source: $OUTPUT_DIR/com/nearclip/ffi/nearclip.kt"
echo "  Android project: $ANDROID_DIR"
echo ""
if [ -n "$ANDROID_NDK_HOME" ]; then
    echo "Native libraries copied to:"
    for pair in "${TARGETS[@]}"; do
        abi=$(get_abi "$pair")
        echo "  - $ANDROID_DIR/app/src/main/jniLibs/$abi/"
    done
else
    echo "Native libraries NOT built (NDK not found)"
    echo "Run this script again after installing Android NDK"
fi
echo ""
