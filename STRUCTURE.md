# Nearclip Multi-Platform Native Project Structure

## Root Directory
```
nearclip/
├── platforms/              # Platform-specific native implementations
├── shared/                 # Shared protocol definitions and utilities
├── docs/                   # Documentation
├── scripts/                # Build and deployment scripts
├── assets/                 # Shared assets
├── .github/                # GitHub workflows
├── project.json            # Project configuration
├── .gitignore              # Git ignore rules
└── README.md               # Project overview
```

## Platform-Specific Structure

### Android (Kotlin/Java)
```
platforms/android/
├── app/
│   ├── src/main/
│   │   ├── java/com/nearclip/
│   │   │   ├── domain/        # Domain layer
│   │   │   ├── application/    # Application layer
│   │   │   ├── infrastructure/ # Infrastructure layer
│   │   │   ├── presentation/   # Presentation layer
│   │   │   ├── data/          # Data models
│   │   │   └── utils/         # Platform utilities
│   │   ├── res/              # Resources
│   │   └── AndroidManifest.xml
│   └── build.gradle
├── core/                    # Core Android library
├── ble/                     # BLE communication module
├── lan/                     # LAN communication module
└── tests/                   # Android tests
```

### iOS (Swift)
```
platforms/ios/
├── Nearclip/
│   ├── Sources/
│   │   ├── Domain/          # Domain layer
│   │   ├── Application/     # Application layer
│   │   ├── Infrastructure/  # Infrastructure layer
│   │   ├── Presentation/    # Presentation layer
│   │   ├── Data/           # Data models
│   │   └── Utils/          # Platform utilities
│   ├── Resources/           # Resources
│   └── Info.plist
├── Nearclip.xcodeproj/
├── Packages/
│   ├── Core/               # Core Swift package
│   ├── BLE/                # BLE communication package
│   └── LAN/                # LAN communication package
└── Tests/                  # iOS tests
```

### Windows (C++/C#)
```
platforms/windows/
├── src/
│   ├── Domain/             # Domain layer
│   ├── Application/        # Application layer
│   ├── Infrastructure/     # Infrastructure layer
│   ├── Presentation/       # Presentation layer
│   ├── Data/              # Data models
│   └── Utils/             # Platform utilities
├── include/                # Header files
├── lib/                    # Library files
├── modules/
│   ├── Core/              # Core module
│   ├── BLE/               # BLE communication module
│   └── LAN/               # LAN communication module
├── resources/             # Resources
├── CMakeLists.txt
└── vcpkg.json             # Package management
```

### macOS (Swift)
```
platforms/macos/
├── Nearclip/
│   ├── Sources/
│   │   ├── Domain/          # Domain layer
│   │   ├── Application/     # Application layer
│   │   ├── Infrastructure/  # Infrastructure layer
│   │   ├── Presentation/    # Presentation layer
│   │   ├── Data/           # Data models
│   │   └── Utils/          # Platform utilities
│   ├── Resources/           # Resources
│   └── Info.plist
├── Nearclip.xcodeproj/
├── Packages/
│   ├── Core/               # Core Swift package
│   ├── BLE/                # BLE communication package
│   └── LAN/                # LAN communication package
└── Tests/                  # macOS tests
```

### Linux (C++/Rust)
```
platforms/linux/
├── src/
│   ├── Domain/             # Domain layer
│   ├── Application/        # Application layer
│   ├── Infrastructure/     # Infrastructure layer
│   ├── Presentation/       # Presentation layer
│   ├── Data/              # Data models
│   └── Utils/             # Platform utilities
├── include/                # Header files
├── lib/                    # Library files
├── modules/
│   ├── Core/              # Core module
│   ├── BLE/               # BLE communication module
│   └── LAN/               # LAN communication module
├── resources/             # Resources
├── CMakeLists.txt
└── Cargo.toml             # Rust package management (if using Rust)
```

## Shared Code Structure

### shared/
```
shared/
├── core/                   # Core business logic
│   ├── domain/             # Domain models (protocol definitions)
│   ├── entities/           # Entity definitions
│   ├── usecases/           # Use case definitions
│   └── interfaces/         # Interface definitions
├── proto/                  # Protocol definitions
│   ├── clipboard/          # Clipboard protocols
│   ├── sync/               # Synchronization protocols
│   ├── device/             # Device discovery protocols
│   └── transfer/           # Data transfer protocols
├── utils/                  # Shared utilities
│   ├── crypto/             # Cryptographic utilities
│   ├── encoding/           # Encoding utilities
│   └── compression/        # Compression utilities
└── types/                  # Shared type definitions
    ├── primitives/         # Primitive types
    ├── data/              # Data types
    └── network/           # Network types
```

## Protocol Definitions

### shared/proto/
```
shared/proto/
├── clipboard.proto         # Clipboard message format
├── sync.proto              # Synchronization protocol
├── device.proto            # Device discovery protocol
├── transfer.proto          # Data transfer protocol
├── security.proto          # Security protocol
└── common.proto            # Common types
```

## Documentation Structure

### docs/
```
docs/
├── api/                    # API documentation
│   ├── protobuf/           # Protocol Buffer documentation
│   ├── rest/               # REST API documentation
│   └── websocket/          # WebSocket API documentation
├── architecture/           # Architecture documentation
│   ├── domain/             # Domain model documentation
│   ├── patterns/           # Architecture patterns
│   └── platform/           # Platform-specific architecture
├── protocols/              # Protocol documentation
│   ├── ble/                # BLE protocol
│   ├── lan/                # LAN protocol
│   └── security/           # Security protocol
└── development/            # Development guides
    ├── setup/              # Setup guides
    ├── testing/            # Testing guides
    └── deployment/         # Deployment guides
```

## Build Scripts

### scripts/
```
scripts/
├── build/
│   ├── android.sh          # Android build script
│   ├── ios.sh              # iOS build script
│   ├── windows.bat         # Windows build script
│   ├── macos.sh            # macOS build script
│   └── linux.sh            # Linux build script
├── test/
│   ├── android.sh          # Android test script
│   ├── ios.sh              # iOS test script
│   ├── windows.bat         # Windows test script
│   ├── macos.sh            # macOS test script
│   └── linux.sh            # Linux test script
└── deploy/
    ├── android.sh          # Android deployment script
    ├── ios.sh              # iOS deployment script
    ├── windows.bat         # Windows deployment script
    ├── macos.sh            # macOS deployment script
    └── linux.sh            # Linux deployment script
```

## Configuration Files

### Root Level
```
nearclip/
├── project.json            # Project configuration
├── .gitignore              # Git ignore rules
├── .github/                # GitHub workflows
└── README.md               # Project README
```

## Development Tools

### scripts/tools/
```
scripts/tools/
├── proto/                 # Protocol Buffer compilation tools
├── test/                  # Testing tools
├── lint/                  # Code linting tools
└── format/                # Code formatting tools
```

## Testing Structure

### Each Platform
```
platforms/{platform}/tests/
├── unit/                  # Unit tests
├── integration/           # Integration tests
├── e2e/                   # End-to-end tests
├── fixtures/              # Test fixtures
└── mocks/                 # Test mocks
```

## Common Patterns

### Domain Layer (All Platforms)
- Implement shared domain models
- Follow DDD principles
- Use shared protocol definitions

### Application Layer (All Platforms)
- Implement use cases
- Handle business logic
- Coordinate between layers

### Infrastructure Layer (All Platforms)
- Platform-specific implementations
- External service integration
- Data persistence

### Presentation Layer (All Platforms)
- Platform-specific UI
- User interaction handling
- Platform-specific navigation

## Communication Between Platforms

### Protocol Buffers
- Shared message definitions
- Cross-platform serialization
- Type-safe communication

### Common APIs
- RESTful API for web clients
- WebSocket for real-time sync
- Platform-specific APIs

## Build System Integration

### Dependency Management
- Android: Gradle
- iOS: Swift Package Manager / CocoaPods
- Windows: vcpkg / NuGet
- macOS: Swift Package Manager
- Linux: CMake / Cargo

### Continuous Integration
- GitHub Actions workflows
- Platform-specific build pipelines
- Automated testing
- Deployment automation