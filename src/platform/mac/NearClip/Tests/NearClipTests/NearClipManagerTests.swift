import XCTest
@testable import NearClip
@testable import NearClipModels
@testable import NearClipServices

final class NearClipManagerTests: XCTestCase {
    var nearClipManager: NearClipManager!

    override func setUp() {
        super.setUp()
        nearClipManager = NearClipManager()
    }

    override func tearDown() {
        nearClipManager = nil
        super.tearDown()
    }

    func testInitialize() throws {
        // Given
        let manager = NearClipManager()

        // When
        let result = manager.isInitialized

        // Then
        XCTAssertTrue(result, "NearClipManager应该成功初始化")
    }

    func testStartDeviceDiscovery() throws {
        // Given
        let expectation = XCTestExpectation(description: "设备发现开始")
        var discoveryStarted = false

        // When
        do {
            try nearClipManager.startDeviceDiscovery { device in
                discoveryStarted = true
                expectation.fulfill()
            }

            // Then
            XCTAssertTrue(nearClipManager.isDiscovering, "设备发现应该已开始")
        } catch {
            XCTFail("设备发现启动失败: \(error.localizedDescription)")
        }
    }

    func testStopDeviceDiscovery() throws {
        // Given
        try nearClipManager.startDeviceDiscovery { _ in }
        XCTAssertTrue(nearClipManager.isDiscovering)

        // When
        nearClipManager.stopDeviceDiscovery()

        // Then
        XCTAssertFalse(nearClipManager.isDiscovering, "设备发现应该已停止")
    }

    func testConnectToDevice() async throws {
        // Given
        let mockDevice = Device(
            id: "test-device-1",
            name: "Test Mac",
            type: .mac,
            connectionStatus: .disconnected,
            lastSeen: Date(),
            capabilities: [.ble, .clipboardRead]
        )

        // When
        try await nearClipManager.connectToDevice(mockDevice)

        // Then
        XCTAssertEqual(nearClipManager.connectedDevices.count, 1)
        XCTAssertEqual(nearClipManager.connectedDevices.first?.id, mockDevice.id)
        XCTAssertEqual(nearClipManager.connectedDevices.first?.connectionStatus, .connected)
    }

    func testDisconnectFromDevice() throws {
        // Given
        let mockDevice = Device(
            id: "test-device-1",
            name: "Test Mac",
            type: .mac,
            connectionStatus: .connected,
            lastSeen: Date(),
            capabilities: [.ble, .clipboardRead]
        )
        nearClipManager.connectedDevices.append(mockDevice)

        // When
        nearClipManager.disconnectFromDevice(mockDevice)

        // Then
        XCTAssertEqual(nearClipManager.connectedDevices.count, 1)
        XCTAssertEqual(nearClipManager.connectedDevices.first?.connectionStatus, .disconnected)
    }
}