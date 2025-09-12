# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NearClip is a high-performance, low-resource cross-device clipboard synchronization tool. It uses Rust for core logic (device discovery, encryption, hybrid transport management) with native UI implementations for each platform.

## Key Architecture Principles

### Hybrid Transport Architecture
- **Unified Abstraction**: Users see only devices, not transport methods. Transport selection is completely transparent.
- **Smart Selection**: Automatically chooses optimal transport (WiFi/BLE) based on network environment, signal quality, power consumption, and other factors.
- **Seamless Switching**: Network changes trigger automatic transport switching without user awareness.
- **Protocol Bridging**: Enables communication between WiFi-only and BLE-only devices.

### Core Technical Stack
- **Rust Core Library**: Device discovery, encryption, hybrid transport management
- **Android UI**: Kotlin/Jetpack Compose + Rust FFI
- **iOS UI**: SwiftUI + Rust FFI
- **Desktop UI**: Tauri + Rust (or platform-specific native)

## Essential Development Commands

This project is currently in planning phase. No build system is configured yet. When implemented, expect:

```bash
# Rust core library development
cd rust-core
cargo build
cargo test
cargo clippy

# Android UI development  
cd android
./gradlew build
./gradlew test

# iOS UI development
cd ios
xcodebuild -workspace NearClip.xcworkspace -scheme NearClip build

# Desktop UI development (Tauri)
cd desktop
npm run tauri build
```

## Critical Architecture Patterns

### Transport Layer Abstraction
All transport implementations must follow the `Transport` trait defined in `rust-core/transport/transport.rs`:

```rust
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<(), TransportError>;
    async fn send_data(&mut self, data: &[u8]) -> Result<(), TransportError>;
    fn get_quality_score(&self) -> f32;
    // ... other required methods
}
```

### Device Management Pattern
The `UnifiedDevice` structure provides device abstraction that aggregates multiple discovery methods and transport capabilities. Users interact with devices, not transport methods.

### Context-Aware Decision Making
Transport selection uses real-time context information:
- Network quality and availability
- BLE environment and signal strength  
- Device power state and battery level
- User preferences and historical patterns

## Security Requirements

All implementations must follow these security patterns:
- End-to-end encryption using ECDH key exchange + AES-256-GCM
- Secure pairing via QR codes and PIN codes
- Protocol-level bridging with encryption between WiFi and BLE
- No sensitive information in logs or commits

## Performance Targets

Critical resource constraints that all implementations must respect:
- Memory usage: < 50MB runtime memory for Rust core
- CPU usage: < 5% CPU during background operation
- Battery impact: < 2% battery consumption/hour in BLE mode
- Network traffic: < 1KB/minute for heartbeat, < 10KB/discovery

## Multi-Platform Considerations

### Android Specific
- Background service persistence and Doze mode adaptation
- Power saving mode connection strategies
- Foreground service notifications for connection status

### iOS Specific  
- Background App Refresh mode device management
- App suspension state connection recovery
- System push notification reconnection triggers

### Desktop Specific
- System startup auto-connection
- Sleep/wake reconnection mechanisms
- Tray icon for connection status

## Development Workflow

### Current Phase
Project planning and architecture design

### Implementation Methodology
**STRICT TDD ADHERENCE**: All development MUST follow Test-Driven Development
1. **RED**: Write a failing test that defines a new function or improvement
2. **GREEN**: Write minimal production code to make the test pass
3. **REFACTOR**: Clean up both test and production code while keeping tests green
4. **NO EXTRA CODE**: Write ONLY the code needed to pass tests - no gold plating

### Clean Architecture Requirements
- **Entities**: Core business rules (device management, encryption algorithms)
- **Use Cases**: Application-specific business rules (device discovery, pairing, sync)
- **Interface Adapters**: Convert data from/to external form (transport protocols, UI)
- **Frameworks & Drivers**: UI, Database, External APIs

### XP (Extreme Programming) Best Practices
- **Pair Programming**: All production code developed in pairs
- **Continuous Integration**: Automated builds and testing
- **Small Releases**: Frequent, incremental releases
- **Simple Design**: KISS principle, YAGNI (You Ain't Gonna Need It)
- **Testing**: Unit tests, integration tests, acceptance tests
- **Coding Standards**: Consistent formatting and naming conventions
- **Sustainable Pace**: 40-hour work weeks, no overtime

## File Structure Conventions

- `doc/`: Planning documentation, stories, and task specifications
- `rust-core/`: Core Rust library implementation (when implemented)
- `android/`: Android UI implementation (when implemented)
- `ios/`: iOS UI implementation (when implemented)
- `desktop/`: Desktop UI implementation (when implemented)

## Documentation Standards

### Markdown Reference Format
When referencing other files in task documentation, always use standard markdown link format:

```markdown
- [Task 0101: 实现设备抽象层](../tasks/0101-device-abstraction-layer.md)
- [加密密钥结构](0201-encryption-key-structure.md)
```

**DO NOT use plain text references:**
```markdown
- Task 0101: 实现设备抽象层
- 加密密钥结构 (0201-encryption-key-structure.md)
```

This ensures all file references are clickable and consistent across the documentation.

## Important Notes

- This is a defensive security tool only - focus on secure clipboard synchronization
- The hybrid transport architecture is the key innovation - maintain transport transparency for users
- Performance optimization is critical due to resource constraints
- Cross-platform consistency is essential for user experience