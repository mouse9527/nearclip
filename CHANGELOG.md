# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-15

### Added
- **Testing**: Comprehensive test suite with 1321+ tests across 7 crates
  - Unit tests for all core modules
  - Integration tests for FFI layer (57 tests)
  - Cross-crate integration tests
- **CI/CD**: GitHub Actions workflow with quality gates
  - Automated testing on push/PR
  - Clippy linting with warnings-as-errors
  - Documentation generation
- **Documentation**: Manual testing guide for platform-specific features
- **Architecture**: New crates for better separation of concerns
  - `nearclip-device`: Device management and pairing
  - `nearclip-transport`: Unified transport abstraction
  - `nearclip-protocol`: Message protocol definitions

### Changed
- **Architecture**: Migrated to 7-crate structure for better modularity
- **BLE**: Bidirectional pairing with ECDH key exchange
- **macOS**: Device storage migrated from UserDefaults to Keychain
- **Code Quality**: Fixed all actionable Clippy warnings

### Fixed
- Deprecated API usage in cipher module (Nonce::from_slice)
- Redundant field names in test code
- Unused imports across multiple modules
- Type complexity warnings in test helpers

## [0.1.7] - 2025-12-26

### Changed
- **Architecture**: Migrate device storage to FFI layer with dependency inversion pattern
  - Rust layer now controls when devices are saved/loaded/removed
  - Platform layers (macOS/Android) implement `FfiDeviceStorage` interface for actual storage
  - Unified pairing flow: pair → connect → save on success
- **FFI**: Add `FfiBleHardware` interface consolidating BLE operations
- **FFI**: Add `FfiDeviceStorage` interface for platform storage implementations

### Fixed
- Device pairing now only persists after successful connection (prevents orphaned entries)
- Sync history storage migrated to FFI layer for consistency

## [0.1.6] - 2025-12-16

### Fixed
- Android: Regenerate and format UniFFI Kotlin bindings

## [0.1.5] - 2025-12-16

### Fixed
- macOS: Fix dylib install name before Swift compilation (use @rpath)

## [0.1.4] - 2025-12-16

### Fixed
- Android: Add missing launcher icons (ic_launcher, ic_launcher_round)

## [0.1.3] - 2025-12-16

### Fixed
- macOS: Fix dylib loading path in app bundle

## [0.1.2] - 2025-12-16

### Fixed
- Android: Add gradle wrapper files for CI builds
- Docs: Add installation instructions for unsigned apps

## [0.1.1] - 2025-12-16

### Fixed
- CI/CD: Create proper macOS .app bundle with Info.plist
- CI/CD: Generate DMG installer instead of ZIP archive
- CI/CD: Use cargo-ndk for Android cross-compilation
- CI/CD: Properly name release artifacts with version

## [0.1.0] - 2025-12-15

### Added

#### Core Infrastructure (Epic 1)
- Rust workspace with 6 modular crates
- Unified error types (`NearClipError`) across all modules
- Structured logging with `tracing`
- MessagePack-based message protocol

#### Device Discovery & Pairing (Epic 2)
- ECDH P-256 key pair generation for secure pairing
- TLS 1.3 configuration with self-signed certificates
- mDNS service broadcast and discovery
- BLE device advertising and scanning
- QR code generation and parsing for pairing
- Persistent storage for paired devices

#### Clipboard Sync (Epic 3)
- TCP server/client with TLS encryption
- BLE GATT data transfer (peripheral & central)
- Clipboard content send/receive
- Channel status monitoring
- Automatic Wi-Fi/BLE channel switching
- Exponential backoff retry mechanism
- Sync loop prevention (LoopGuard)
- Core coordination layer (`NearClipManager`)

#### macOS Client (Epic 4)
- SwiftUI menubar application
- UniFFI Swift bindings
- Connection status display with animated icons
- System clipboard monitoring
- Clipboard write with remote content marking
- Device pairing UI with QR code display
- Settings UI (sync options, retry strategy)
- Keychain storage for paired devices

#### Android Client (Epic 5)
- Kotlin/Jetpack Compose application
- UniFFI Kotlin bindings
- Foreground service for background sync
- Accessibility-based clipboard monitoring
- Clipboard write functionality
- Connection status display
- Pairing UI with QR code scanner
- Settings UI with Material 3 design
- EncryptedSharedPreferences storage

#### Enhanced User Experience (Epic 6)
- Sync success notifications (both platforms)
- Sync failure alerts with retry options
- Multi-device pairing support (up to 5 devices)
- Configurable retry strategies (Discard/Wait/Retry)
- Encrypted pairing data storage
- Network recovery auto-reconnect

### Security
- End-to-end encryption using ECDH + TLS 1.3
- Secure local storage (Keychain/Android Keystore)
- No cloud dependencies - fully P2P

### Developer Experience
- Comprehensive README with architecture overview
- Detailed build guide for all platforms
- GitHub Actions CI/CD workflows
- Rust documentation generation

## [Unreleased]

### Planned
- iOS client
- Windows client
- Image/file clipboard support
- Clipboard history
- Selective device sync
