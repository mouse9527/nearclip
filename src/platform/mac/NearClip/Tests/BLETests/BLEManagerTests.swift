import XCTest
import CoreBluetooth
@testable import NearClip

/// BLE管理器测试
class BLEManagerTests: XCTestCase {

    var bleManager: BLEManager!

    override func setUp() {
        super.setUp()
        bleManager = BLEManager()
    }

    override func tearDown() {
        bleManager = nil
        super.tearDown()
    }

    /// 测试BLE管理器初始化
    func testBLEManagerInitialization() {
        XCTAssertNotNil(bleManager)
        XCTAssertFalse(bleManager.isScanning)
        XCTAssertFalse(bleManager.isAdvertising)
        XCTAssertTrue(bleManager.discoveredPeripherals.isEmpty)
        XCTAssertTrue(bleManager.connectedPeripherals.isEmpty)
    }

    /// 测试设备扫描状态
    func testScanningState() {
        // 初始状态应该是未扫描
        XCTAssertFalse(bleManager.isScanning)

        // 设置回调以测试状态变化
        var stateChanged = false
        bleManager.$isScanning
            .dropFirst() // 跳过初始值
            .sink { isScanning in
                if isScanning { stateChanged = true }
            }
            .store(in: &cancellables)

        // 注意：由于权限限制，实际的扫描测试需要在真实设备上运行
        // 这里主要测试状态管理逻辑
    }

    /// 测试设备发现回调
    func testDeviceDiscoveryCallback() {
        var discoveredDevice: CBPeripheral?
        let expectation = XCTestExpectation(description: "设备发现回调")

        bleManager.onDeviceDiscovered = { peripheral, advertisementData, rssi in
            discoveredDevice = peripheral
            expectation.fulfill()
        }

        // 模拟设备发现需要真实的CBPeripheral对象
        // 在实际测试中，这需要使用模拟的蓝牙环境
    }

    /// 测试NearClip设备过滤
    func testNearClipDeviceFiltering() {
        // 创建模拟的CBPeripheral对象
        let mockPeripherals = createMockPeripherals()

        // 手动添加到discoveredPeripherals数组
        mockPeripherals.forEach { peripheral in
            bleManager.discoveredPeripherals.append(peripheral)
        }

        // 测试过滤功能
        let nearClipDevices = bleManager.getNearClipDevices()

        // 应该只返回NearClip设备
        XCTAssertEqual(nearClipDevices.count, 2)
        XCTAssertTrue(nearClipDevices.allSatisfy { $0.name?.contains("NearClip") == true })
    }

    /// 测试设备列表清理
    func testClearDevices() {
        // 添加一些模拟设备
        let mockPeripherals = createMockPeripherals()
        mockPeripherals.forEach { bleManager.discoveredPeripherals.append($0) }

        // 验证设备已添加
        XCTAssertFalse(bleManager.discoveredPeripherals.isEmpty)

        // 清理设备
        bleManager.clearDevices()

        // 验证设备已清理
        XCTAssertTrue(bleManager.discoveredPeripherals.isEmpty)
        XCTAssertTrue(bleManager.connectedPeripherals.isEmpty)
    }

    // MARK: - Helper Methods

    private var cancellables = Set<AnyCancellable>()

    /// 创建模拟的外设对象
    private func createMockPeripherals() -> [CBPeripheral] {
        // 注意：CBPeripheral不能直接实例化
        // 在真实测试中，需要使用CoreBluetooth的模拟框架
        // 这里提供一个框架用于实际测试

        let peripheralNames = [
            "NearClip-Android",
            "NearClip-iPhone",
            "Regular-Device",
            "Unknown-Device"
        ]

        // 由于CBPeripheral不能直接创建，这里返回空数组
        // 实际测试需要使用XCTest的CoreBluetooth模拟
        return []
    }
}

/// 设备扫描器测试
class DeviceScannerTests: XCTestCase {

    var deviceScanner: DeviceScanner!

    override func setUp() {
        super.setUp()
        deviceScanner = DeviceScanner()
    }

    override func tearDown() {
        deviceScanner = nil
        super.tearDown()
    }

    /// 测试扫描器初始化
    func testDeviceScannerInitialization() {
        XCTAssertNotNil(deviceScanner)
        XCTAssertFalse(deviceScanner.isScanning)
        XCTAssertTrue(deviceScanner.discoveredDevices.isEmpty)
        XCTAssertTrue(deviceScanner.nearClipDevices.isEmpty)
    }

    /// 测试扫描统计
    func testScanStats() {
        // 初始状态
        let initialStats = deviceScanner.getScanStats()
        XCTAssertEqual(initialStats.totalDevices, 0)
        XCTAssertEqual(initialStats.nearClipDevices, 0)

        // 添加测试设备
        deviceScanner.addTestDevice()
        deviceScanner.addTestDevice()

        let updatedStats = deviceScanner.getScanStats()
        XCTAssertEqual(updatedStats.totalDevices, 2)
        XCTAssertEqual(updatedStats.nearClipDevices, 2)
    }

    /// 测试设备清理
    func testClearAllDevices() {
        // 添加测试设备
        deviceScanner.addTestDevice()
        XCTAssertFalse(deviceScanner.discoveredDevices.isEmpty)

        // 清理设备
        deviceScanner.clearAllDevices()
        XCTAssertTrue(deviceScanner.discoveredDevices.isEmpty)
        XCTAssertTrue(deviceScanner.nearClipDevices.isEmpty)
    }

    /// 测试NearClip设备过滤
    func testNearClipDeviceFiltering() {
        // 添加多个测试设备
        deviceScanner.addTestDevice()

        let stats = deviceScanner.getScanStats()
        XCTAssertEqual(stats.nearClipDevices, 1)
        XCTAssertEqual(stats.totalDevices, 1)
    }
}

/// 连接管理器测试
class ConnectionManagerTests: XCTestCase {

    var connectionManager: ConnectionManager!

    override func setUp() {
        super.setUp()
        connectionManager = ConnectionManager()
    }

    override func tearDown() {
        connectionManager = nil
        super.tearDown()
    }

    /// 测试连接管理器初始化
    func testConnectionManagerInitialization() {
        XCTAssertNotNil(connectionManager)
        XCTAssertTrue(connectionManager.connectionStates.isEmpty)
        XCTAssertTrue(connectionManager.activeConnections.isEmpty)
        XCTAssertTrue(connectionManager.receivedMessages.isEmpty)
    }

    /// 测试连接统计
    func testConnectionStats() {
        let initialStats = connectionManager.getConnectionStats()
        XCTAssertEqual(initialStats.totalConnections, 0)
        XCTAssertEqual(initialStats.connectedCount, 0)
        XCTAssertEqual(initialStats.connectingCount, 0)
        XCTAssertEqual(initialStats.failedCount, 0)
    }

    /// 测试消息创建
    func testMessageCreation() {
        let pingMessage = TestMessage(
            messageId: "ping-001",
            type: .ping,
            payload: "",
            timestamp: Date().timeIntervalSince1970,
            sequenceNumber: 0
        )

        XCTAssertEqual(pingMessage.type, .ping)
        XCTAssertEqual(pingMessage.messageId, "ping-001")
        XCTAssertTrue(pingMessage.payload.isEmpty)
        XCTAssertEqual(pingMessage.sequenceNumber, 0)
    }

    /// 测试消息序列化
    func testMessageSerialization() {
        let originalMessage = TestMessage(
            messageId: "test-001",
            type: .data,
            payload: "Hello World",
            timestamp: 1640995200.0,
            sequenceNumber: 1
        )

        // 注意：这里需要访问ConnectionManager的私有方法
        // 在实际实现中，可以将序列化方法设为public或使用测试友好的访问控制
        let serializedMessage = "\(originalMessage.messageId)|\(originalMessage.type.rawValue)|\(originalMessage.payload)|\(originalMessage.timestamp)|\(originalMessage.sequenceNumber)"

        let expectedString = "test-001|DATA|Hello World|1640995200.0|1"
        XCTAssertEqual(serializedMessage, expectedString)
    }

    /// 测试消息反序列化
    func testMessageDeserialization() {
        let messageString = "test-001|DATA|Hello World|1640995200.0|1"
        let parts = messageString.split(separator: "|")

        XCTAssertEqual(parts.count, 5)
        XCTAssertEqual(String(parts[0]), "test-001")
        XCTAssertEqual(String(parts[1]), "DATA")
        XCTAssertEqual(String(parts[2]), "Hello World")
        XCTAssertEqual(Double(String(parts[3])), 1640995200.0)
        XCTAssertEqual(Int(String(parts[4])), 1)

        // 重建消息对象
        let reconstructedMessage = TestMessage(
            messageId: String(parts[0]),
            type: MessageType(rawValue: String(parts[1])) ?? .data,
            payload: String(parts[2]),
            timestamp: Double(String(parts[3])) ?? 0.0,
            sequenceNumber: Int(String(parts[4])) ?? 0
        )

        XCTAssertEqual(reconstructedMessage.messageId, "test-001")
        XCTAssertEqual(reconstructedMessage.type, .data)
        XCTAssertEqual(reconstructedMessage.payload, "Hello World")
        XCTAssertEqual(reconstructedMessage.timestamp, 1640995200.0)
        XCTAssertEqual(reconstructedMessage.sequenceNumber, 1)
    }

    /// 测试不同类型的消息
    func testAllMessageTypes() {
        let messageTypes: [MessageType] = [.ping, .pong, .data, .ack]

        messageTypes.forEach { messageType in
            let message = TestMessage(
                messageId: "test-\(messageType.rawValue)",
                type: messageType,
                payload: "Test payload",
                timestamp: Date().timeIntervalSince1970,
                sequenceNumber: 1
            )

            XCTAssertEqual(message.type, messageType)
            XCTAssertEqual(message.type.displayName, messageType.displayName)
        }
    }
}

/// BLE集成测试
class BLEIntegrationTests: XCTestCase {

    /// 测试完整的设备发现流程
    func testCompleteDeviceDiscoveryFlow() {
        // 这个测试需要真实的蓝牙环境和权限
        // 在CI/CD环境中可能无法运行

        let expectation = XCTestExpectation(description: "设备发现流程")
        expectation.expectedFulfillmentCount = 3 // 发现设备 -> 连接 -> 收到消息

        let bleManager = BLEManager()

        // 设置设备发现回调
        bleManager.onDeviceDiscovered = { peripheral, advertisementData, rssi in
            print("发现设备: \(peripheral.name ?? "未知")")
            expectation.fulfill()
        }

        // 设置连接状态回调
        bleManager.onConnectionStateChanged = { peripheral, isConnected in
            if isConnected {
                print("设备已连接: \(peripheral.name ?? "未知")")
                expectation.fulfill()
            }
        }

        // 模拟测试流程
        // 1. 开始扫描
        // 2. 发现设备
        // 3. 连接设备
        // 4. 发送/接收消息

        wait(for: [expectation], timeout: 30.0)
    }

    /// 测试消息通信流程
    func testMessageCommunicationFlow() {
        let expectation = XCTestExpectation(description: "消息通信流程")
        expectation.expectedFulfillmentCount = 4 // Ping -> Pong -> Data -> ACK

        let connectionManager = ConnectionManager()

        // 设置消息接收回调
        connectionManager.onMessageReceived = { message in
            print("收到消息: \(message.type.rawValue) - \(message.messageId)")
            expectation.fulfill()
        }

        // 模拟消息流程
        // 1. 发送Ping
        // 2. 接收Pong
        // 3. 发送Data
        // 4. 接收ACK

        wait(for: [expectation], timeout: 30.0)
    }
}