# NearClip macOS Build Instructions

## Prerequisites
- Rust toolchain (`cargo`)
- Swift toolchain (`swift`)
- Xcode Command Line Tools (or full Xcode)

## Building the macOS Application

1. **Build the Rust FFI Library**
   First, build the Rust library that powers the core logic.
   ```bash
   cargo build --package nearclip-ffi --target aarch64-apple-darwin
   ```
   *Note: If you are on an Intel Mac, use `--target x86_64-apple-darwin`.*

2. **Generate Swift Bindings**
   Generate the Swift bindings for the Rust library.
   ```bash
   cargo run --bin uniffi-bindgen generate crates/nearclip-ffi/src/nearclip.udl --language swift --out-dir macos/NearClip/Sources/NearClipFFI
   ```

3. **Prepare the Static Library**
   The Swift Package expects the static library to be in `target/swift`.
   ```bash
   mkdir -p target/swift
   cp target/aarch64-apple-darwin/debug/libnearclip_ffi.a target/swift/libnearclip_ffi.a
   ```
   *Note: Adjust the source path if you built for a different target or profile (e.g., release).*

4. **Build the Swift Application**
   Navigate to the macOS project directory and build using Swift Package Manager.
   ```bash
   cd macos/NearClip
   swift build
   ```

5. **Run the Application**
   After a successful build, the executable will be located in the `.build/debug` directory.
   ```bash
   ./.build/debug/NearClip
   ```

## Troubleshooting
- **Linker Warnings**: You may see warnings about object files being built for a newer macOS version. These are generally safe to ignore for local development.
- **Missing Library**: If `swift build` fails due to a missing library, ensure you have copied `libnearclip_ffi.a` to the correct location as described in step 3.
