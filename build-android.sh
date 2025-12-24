#!/bin/bash
# Build NearClip FFI for Android

NDK_HOME="$HOME/Library/Android/sdk/ndk/26.1.10909125"
NDK_BIN="$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin"

export PATH="$NDK_BIN:$PATH"

cargo build -p nearclip-ffi --target aarch64-linux-android --release
