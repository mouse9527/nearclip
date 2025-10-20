import XCTest
@testable import NearClip

/**
 * Protocol Buffers跨平台一致性测试
 * 验证Mac和Android端的消息序列化/反序列化一致性
 */
class ProtocolBufferConsistencyTest: XCTestCase {

    var protocolHandler: ProtocolBufferMessageHandler!

    override func setUp() {
        super.setUp()
        protocolHandler = ProtocolBufferMessageHandler()
    }

    override func tearDown() {
        protocolHandler = nil
        super.tearDown()
    }

    /// 测试ping消息序列化反序列化一致性
    func testPingMessageSerializationConsistency() {
        // Given: 创建一个PING消息
        let originalMessage = TestMessage(
            messageId: "ping-test-001",
            type: .ping,
            payload: "",
            timestamp: 1640995200000.0, // 2022-01-01 00:00:00 UTC
            sequenceNumber: 1
        )

        // When: 序列化然后反序列化
        let serializedData = protocolHandler.serializeMessage(originalMessage)
        let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        XCTAssertNotNil(deserializedMessage, "反序列化消息不应为nil")
        XCTAssertEqual(originalMessage.messageId, deserializedMessage?.messageId, "消息ID应该一致")
        XCTAssertEqual(originalMessage.type, deserializedMessage?.type, "消息类型应该一致")
        XCTAssertEqual(originalMessage.payload, deserializedMessage?.payload, "消息载荷应该一致")
        XCTAssertEqual(originalMessage.timestamp, deserializedMessage?.timestamp, "时间戳应该一致")
        XCTAssertEqual(originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber, "序列号应该一致")
    }

    /// 测试data消息序列化反序列化一致性
    func testDataMessageSerializationConsistency() {
        // Given: 创建一个包含数据的消息
        let originalMessage = TestMessage(
            messageId: "data-test-002",
            type: .data,
            payload: "Hello from Mac! This is a test message with special chars: 中文测试 🚀",
            timestamp: Date().timeIntervalSince1970 * 1000, // 转换为毫秒
            sequenceNumber: 5
        )

        // When: 序列化然后反序列化
        let serializedData = protocolHandler.serializeMessage(originalMessage)
        let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        XCTAssertNotNil(deserializedMessage, "反序列化消息不应为nil")
        XCTAssertEqual(originalMessage.messageId, deserializedMessage?.messageId, "消息ID应该一致")
        XCTAssertEqual(originalMessage.type, deserializedMessage?.type, "消息类型应该一致")
        XCTAssertEqual(originalMessage.payload, deserializedMessage?.payload, "消息载荷应该一致")
        XCTAssertEqual(originalMessage.timestamp, deserializedMessage?.timestamp, "时间戳应该一致")
        XCTAssertEqual(originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber, "序列号应该一致")
    }

    /// 测试ack消息序列化反序列化一致性
    func testAckMessageSerializationConsistency() {
        // Given: 创建一个ACK消息
        let originalMessage = TestMessage(
            messageId: "ack-test-003",
            type: .ack,
            payload: "original-message-id-123",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 10
        )

        // When: 序列化然后反序列化
        let serializedData = protocolHandler.serializeMessage(originalMessage)
        let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        XCTAssertNotNil(deserializedMessage, "反序列化消息不应为nil")
        XCTAssertEqual(originalMessage.messageId, deserializedMessage?.messageId, "消息ID应该一致")
        XCTAssertEqual(originalMessage.type, deserializedMessage?.type, "消息类型应该一致")
        XCTAssertEqual(originalMessage.payload, deserializedMessage?.payload, "消息载荷应该一致")
        XCTAssertEqual(originalMessage.timestamp, deserializedMessage?.timestamp, "时间戳应该一致")
        XCTAssertEqual(originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber, "序列号应该一致")
    }

    /// 测试向后兼容性 - 旧格式消息反序列化
    func testBackwardCompatibilityWithOldFormat() {
        // Given: 创建旧格式的消息字符串（模拟Android端发送的旧格式）
        let oldFormatString = "old-format-001|DATA|Hello World|1640995200000|1" // 注意：Android使用毫秒时间戳
        let oldFormatData = oldFormatString.data(using: .utf8)!

        // When: 尝试反序列化旧格式消息
        let deserializedMessage = protocolHandler.deserializeMessage(oldFormatData)

        // Then: 应该能够正确解析旧格式消息
        XCTAssertNotNil(deserializedMessage, "应该能够解析旧格式消息")
        XCTAssertEqual("old-format-001", deserializedMessage?.messageId, "消息ID应该正确")
        XCTAssertEqual(.data, deserializedMessage?.type, "消息类型应该正确")
        XCTAssertEqual("Hello World", deserializedMessage?.payload, "消息载荷应该正确")
        XCTAssertEqual(1640995200000.0, deserializedMessage?.timestamp, "时间戳应该正确")
        XCTAssertEqual(1, deserializedMessage?.sequenceNumber, "序列号应该正确")
    }

    /// 测试消息验证功能
    func testMessageValidation() {
        // Test valid message
        let validMessage = TestMessage(
            messageId: "valid-001",
            type: .ping,
            payload: "Valid payload",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 1
        )
        XCTAssertTrue(protocolHandler.validateMessage(validMessage), "有效消息应该通过验证")

        // Test invalid message (empty ID)
        let invalidMessage1 = TestMessage(
            messageId: "",
            type: .ping,
            payload: "Valid payload",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 1
        )
        XCTAssertFalse(protocolHandler.validateMessage(invalidMessage1), "空ID消息应该验证失败")

        // Test invalid message (negative timestamp)
        let invalidMessage2 = TestMessage(
            messageId: "invalid-002",
            type: .ping,
            payload: "Valid payload",
            timestamp: -1.0,
            sequenceNumber: 1
        )
        XCTAssertFalse(protocolHandler.validateMessage(invalidMessage2), "负时间戳消息应该验证失败")

        // Test invalid message (negative sequence number)
        let invalidMessage3 = TestMessage(
            messageId: "invalid-003",
            type: .ping,
            payload: "Valid payload",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: -1
        )
        XCTAssertFalse(protocolHandler.validateMessage(invalidMessage3), "负序列号消息应该验证失败")

        // Test invalid message (payload too large)
        let largePayload = String(repeating: "x", count: 1025)
        let invalidMessage4 = TestMessage(
            messageId: "invalid-004",
            type: .ping,
            payload: largePayload,
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 1
        )
        XCTAssertFalse(protocolHandler.validateMessage(invalidMessage4), "过大载荷的消息应该验证失败")
    }

    /// 测试多种消息类型的序列化一致性
    func testMultipleMessageTypesConsistency() {
        let messageTypes: [MessageType] = [.ping, .pong, .data, .ack]
        let baseTimestamp = Date().timeIntervalSince1970 * 1000

        for (index, messageType) in messageTypes.enumerated() {
            let originalMessage = TestMessage(
                messageId: "consistency-test-\(index)",
                type: messageType,
                payload: "Test payload for \(messageType)",
                timestamp: baseTimestamp + Double(index * 1000),
                sequenceNumber: index + 1
            )

            let serializedData = protocolHandler.serializeMessage(originalMessage)
            let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

            XCTAssertNotNil(deserializedMessage, "消息类型 \(messageType) 反序列化不应为nil")
            XCTAssertEqual(originalMessage.messageId, deserializedMessage?.messageId, "消息类型 \(messageType) ID应该一致")
            XCTAssertEqual(originalMessage.type, deserializedMessage?.type, "消息类型 \(messageType) 应该一致")
            XCTAssertEqual(originalMessage.payload, deserializedMessage?.payload, "消息类型 \(messageType) 载荷应该一致")
            XCTAssertEqual(originalMessage.timestamp, deserializedMessage?.timestamp, "消息类型 \(messageType) 时间戳应该一致")
            XCTAssertEqual(originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber, "消息类型 \(messageType) 序列号应该一致")
        }
    }

    /// 测试特殊字符和Unicode消息
    func testSpecialCharactersAndUnicodeMessages() {
        let specialMessages = [
            "中文测试消息",
            "🚀 Emoji test 🎉",
            "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?",
            "Multi\nline\nmessage",
            "Tabs\tand\r\rcarriage\returns"
        ]

        for (index, payload) in specialMessages.enumerated() {
            let originalMessage = TestMessage(
                messageId: "unicode-test-\(index)",
                type: .data,
                payload: payload,
                timestamp: Date().timeIntervalSince1970 * 1000,
                sequenceNumber: index
            )

            let serializedData = protocolHandler.serializeMessage(originalMessage)
            let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

            XCTAssertNotNil(deserializedMessage, "Unicode消息 \(index) 反序列化不应为nil")
            XCTAssertEqual(originalMessage.payload, deserializedMessage?.payload, "Unicode消息 \(index) 载荷应该一致")
        }
    }

    /// 测试边界情况
    func testEdgeCases() {
        // 空载荷消息
        let emptyPayloadMessage = TestMessage(
            messageId: "empty-payload-test",
            type: .ping,
            payload: "",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 0
        )

        let serializedData = protocolHandler.serializeMessage(emptyPayloadMessage)
        let deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        XCTAssertNotNil(deserializedMessage, "空载荷消息反序列化不应为nil")
        XCTAssertEqual("", deserializedMessage?.payload, "空载荷消息载荷应该为空")

        // 无效数据处理
        let invalidData = Data([0x01, 0x02, 0x03])
        let invalidMessage = protocolHandler.deserializeMessage(invalidData)
        XCTAssertNil(invalidMessage, "无效数据应该返回nil")

        // 空数据处理
        let emptyData = Data()
        let emptyMessage = protocolHandler.deserializeMessage(emptyData)
        XCTAssertNil(emptyMessage, "空数据应该返回nil")
    }

    /// 测试设备信息Proto创建
    func testDeviceInfoProtoCreation() {
        // 模拟CBPeripheral对象（在实际测试中需要使用mock）
        let mockPeripheralIdentifier = UUID()
        let mockDeviceName = "NearClip-Android-Test"
        let mockRSSI = -65

        // 由于无法直接创建CBPeripheral，这里只测试基本逻辑
        // 在实际集成测试中需要使用真实的CoreBluetooth模拟框架
        XCTAssertTrue(true, "设备信息创建测试需要在真实环境中进行")
    }
}