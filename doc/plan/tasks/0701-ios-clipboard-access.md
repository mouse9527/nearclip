# Task 0701: 实现iOS剪贴板访问权限 (TDD版本)

## 任务描述

按照TDD原则实现iOS平台的剪贴板访问权限检查和处理。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```swift
// ios/NearClipTests/ClipboardPermissionTests.swift
import XCTest
@testable import NearClip

class ClipboardPermissionTests: XCTestCase {
    
    func testClipboardAccessGranted() {
        // RED: 测试剪贴板访问权限已授予
        let checker = ClipboardPermissionChecker()
        let result = checker.checkClipboardAccess()
        
        switch result {
        case .granted:
            XCTAssertTrue(true)
        default:
            XCTFail("Expected granted permission")
        }
    }
    
    func testClipboardAccessDenied() {
        // RED: 测试剪贴板访问权限被拒绝
        let checker = MockClipboardPermissionChecker(denied: true)
        let result = checker.checkClipboardAccess()
        
        switch result {
        case .denied:
            XCTAssertTrue(true)
        default:
            XCTFail("Expected denied permission")
        }
    }
    
    func testPasteboardItemDetection() {
        // RED: 测试剪贴板项目检测
        let checker = ClipboardPermissionChecker()
        let hasItems = checker.hasPasteboardItems()
        
        XCTAssertTrue(hasItems) // 假设有测试数据
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```swift
// ios/NearClip/Clipboard/ClipboardPermissionChecker.swift
import UIKit

class ClipboardPermissionChecker {
    
    enum PermissionStatus {
        case granted
        case denied
        case restricted
        case unknown
    }
    
    func checkClipboardAccess() -> PermissionStatus {
        // iOS 14+ 需要权限，但这里是简化实现
        return .granted
    }
    
    func hasPasteboardItems() -> Bool {
        return UIPasteboard.general.hasStrings
    }
}

// Mock for testing
class MockClipboardPermissionChecker: ClipboardPermissionChecker {
    private let denied: Bool
    
    init(denied: Bool = false) {
        self.denied = denied
    }
    
    override func checkClipboardAccess() -> PermissionStatus {
        return denied ? .denied : .granted
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```swift
// 重构以消除重复，提高代码质量
import UIKit
import Combine

class ClipboardPermissionChecker: ObservableObject {
    
    enum PermissionStatus {
        case granted
        case denied
        case restricted
        case unknown
    }
    
    @Published var currentStatus: PermissionStatus = .unknown
    private var cancellables = Set<AnyCancellable>()
    
    init() {
        checkCurrentPermission()
    }
    
    func checkClipboardAccess() -> PermissionStatus {
        guard #available(iOS 14.0, *) else {
            return .granted // iOS 14之前不需要权限
        }
        
        // 检查是否有权限访问剪贴板
        let pasteboard = UIPasteboard.general
        if pasteboard.hasStrings {
            // 尝试读取一个字符来测试权限
            if let _ = pasteboard.string?.prefix(1) {
                return .granted
            } else {
                return .denied
            }
        }
        
        return .unknown
    }
    
    func hasPasteboardItems() -> Bool {
        let pasteboard = UIPasteboard.general
        return pasteboard.hasStrings || 
               pasteboard.hasURLs || 
               pasteboard.hasImages || 
               pasteboard.hasColors
    }
    
    func getAvailablePasteboardTypes() -> [String] {
        let pasteboard = UIPasteboard.general
        var types: [String] = []
        
        if pasteboard.hasStrings { types.append("text") }
        if pasteboard.hasURLs { types.append("url") }
        if pasteboard.hasImages { types.append("image") }
        if pasteboard.hasColors { types.append("color") }
        
        return types
    }
    
    func requestClipboardAccess() -> Bool {
        // iOS 14+ 会自动提示用户授权
        return checkClipboardAccess() == .granted
    }
    
    private func checkCurrentPermission() {
        DispatchQueue.main.async {
            self.currentStatus = self.checkClipboardAccess()
        }
    }
    
    func monitorPermissionChanges() {
        // 监听应用进入前台时重新检查权限
        NotificationCenter.default.publisher(for: UIApplication.willEnterForegroundNotification)
            .sink { [weak self] _ in
                self?.checkCurrentPermission()
            }
            .store(in: &cancellables)
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为iOS平台的infrastructure层实现：

```swift
// ios/NearClip/Infrastructure/ClipboardPermissionChecker.swift
class ClipboardPermissionChecker {
    // iOS剪贴板权限实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查
- [ ] Xcode编译无错误
- [ ] iOS 14+权限处理正确

## 依赖任务

- [Task 0102: 定义设备状态枚举](0102-device-status-enum.md)

## 后续任务

- [Task 0702: 实现iOS剪贴板变化监听](0702-ios-clipboard-monitor.md)
- [Task 0703: 实现iOS剪贴板内容过滤](0703-ios-content-filter.md)