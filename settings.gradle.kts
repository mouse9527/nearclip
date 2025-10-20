// NearClip 项目配置
// 简化版本，仅用于 Protocol Buffers 生成

rootProject.name = "nearclip"

// 包含共享的 Rust 库
include("shared-rust")
project(":shared-rust").projectDir = file("src/shared/rust")

// Android 项目
include("android")
project(":android").projectDir = file("src/platform/android")