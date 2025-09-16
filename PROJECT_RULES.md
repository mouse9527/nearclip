# Nearclip Project Rules

## Overview
Nearclip is a cross-platform clipboard synchronization tool supporting BLE and LAN synchronization. This project strictly follows:

- **TDD** (Test-Driven Development)
- **DDD** (Domain-Driven Design)
- **Clean Architecture**
- **XP** (Extreme Programming)

## Project Structure
- **platforms/** - Platform-specific native implementations
- **shared/** - Shared protocol definitions and utilities
- **docs/** - Project documentation
- **scripts/** - Build and deployment scripts
- **assets/** - Shared resources

## Development Practices

### 1. Test-Driven Development (TDD)

**Red-Green-Refactor Cycle:**
1. Write a failing test first
2. Implement the minimum code to make the test pass
3. Refactor the code while keeping tests green

**Test Requirements:**
- Unit test coverage: Minimum 90%
- Integration tests for all external dependencies
- End-to-end tests for core user flows
- Tests must run in isolation
- No production code without tests

### 2. Domain-Driven Design (DDD)

**Domain Structure:**
- Focus on business logic and domain models
- Separate core domain from infrastructure concerns
- Use ubiquitous language across codebase
- Define clear bounded contexts

**Core Domains:**
- Clipboard Management
- Synchronization (BLE/LAN)
- Device Discovery
- Data Transfer

### 3. Clean Architecture

**Layer Structure:**
```
src/
├── domain/          # Core business logic, entities
├── application/     # Use cases, application services
├── infrastructure/  # External dependencies, implementations
└── presentation/    # UI, controllers, views
```

**Dependency Rules:**
- Dependencies point inward
- Domain layer has no external dependencies
- Infrastructure depends on application/domain interfaces
- Presentation depends on application layer

### 4. Extreme Programming (XP)

**Core Practices:**
- Pair programming for complex features
- Continuous integration
- Small, frequent releases
- Simple design
- Collective code ownership
- Coding standards

**Code Quality:**
- Maximum function length: 20 lines
- Maximum file length: 500 lines
- Cyclomatic complexity: < 10
- No code duplication
- Meaningful names

## Code Standards

### Naming Conventions
- Use clear, descriptive names
- No abbreviations unless widely understood
- Boolean variables should start with `is`, `has`, `can`
- Constants in UPPER_SNAKE_CASE
- Protocol Buffer messages use PascalCase
- Cross-platform entities use consistent naming across platforms

### File Organization
- One class per file (platform conventions apply)
- File name matches class/entity name
- Group related functionality in modules
- Clear directory structure per platform
- Shared protocols in `shared/proto/`

### Documentation
- Code should be self-documenting
- Only comment complex algorithms or business rules
- Protocol Buffer documentation for cross-platform APIs
- Platform-specific API documentation
- Architecture and setup guides

## Technology Stack Guidelines

### Platform-Specific Languages
- **Android**: Kotlin (primary), Java (legacy)
- **iOS/macOS**: Swift (primary), Objective-C (legacy)
- **Windows**: C++ (primary), C# (alternative)
- **Linux**: C++ (primary), Rust (alternative)
- Each platform follows its own best practices
- Leverage native type systems and features

### Dependencies
- Minimal external dependencies
- Prefer native solutions over third-party libraries
- Security-first approach for all dependencies

## Cross-Platform Communication

### Protocol Buffers
- All cross-platform communication uses Protocol Buffers
- Shared protocol definitions in `shared/proto/`
- Version controlled protocol evolution
- Backward compatibility maintained

### Common Data Models
- Shared entities defined in Protocol Buffers
- Platform-specific implementations
- Consistent serialization across platforms

## Build and Deployment

### Platform-Specific Builds
- Each platform has its own build system
- Shared build scripts in `scripts/`
- Automated dependency management
- Platform-specific packaging

### Continuous Integration
- All platform tests must pass before merge
- Code review required for all changes
- Automated builds on all platforms
- Deployment automation per platform

### Version Control
- Feature branches for all work
- Descriptive commit messages
- Regular pulls from main branch
- Tagged releases with platform-specific artifacts

## Security

### Data Protection
- Encrypt sensitive data in transit
- Secure authentication mechanisms
- Regular security audits
- No hardcoded credentials

### Privacy
- Minimal data collection
- User consent required
- Clear privacy policy

## Performance

### Requirements
- Low memory footprint
- Fast synchronization
- Responsive UI
- Efficient network usage

### Monitoring
- Performance metrics collection
- Error tracking
- User analytics (opt-in)

## Platform-Specific Guidelines

### Android (Kotlin/Java)
- Follow Android Jetpack guidelines
- Use modern Android architecture components
- Gradle for dependency management
- Android testing framework

### iOS/macOS (Swift)
- Follow Swift design patterns
- Use SwiftUI/UIKit as appropriate
- Swift Package Manager for dependencies
- XCTest for testing

### Windows (C++/C#)
- Follow Windows API guidelines
- Use modern C++ features
- vcpkg/NuGet for dependencies
- Microsoft testing frameworks

### Linux (C++/Rust)
- Follow Linux desktop conventions
- Use modern C++ or Rust patterns
- CMake/Cargo for dependencies
- Google Test/Rust testing

## Team Guidelines

### Communication
- Daily stand-ups
- Clear task assignments
- Regular retrospectives
- Open knowledge sharing
- Platform-specific expertise sharing

### Quality Assurance
- Code reviews mandatory
- Platform-specific testing
- Cross-platform integration testing
- Manual testing cycles
- User acceptance testing across platforms

---

*This document ensures consistency and quality across the nearclip project. All team members must follow these guidelines.*