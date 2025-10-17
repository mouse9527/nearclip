import XCTest
@testable import NearClip

class ProtocolHandlerTests: XCTestCase {

    var discoveryHandler: DiscoveryHandler!
    var pairingHandler: PairingHandler!
    var syncHandler: SyncHandler!

    override func setUp() {
        super.setUp()
        discoveryHandler = DiscoveryHandler()
        pairingHandler = PairingHandler()
        syncHandler = SyncHandler()
    }

    override func tearDown() {
        discoveryHandler = nil
        pairingHandler = nil
        syncHandler = nil
        super.tearDown()
    }

    // MARK: - Discovery Handler Tests

    func testDeviceBroadcastValidation() throws {
        // 创建有效广播消息
        let validBroadcast = DeviceBroadcast.with {
            $0.deviceID = "test-device"
            $0.deviceName = "Test Device"
            $0.deviceType = .deviceTypeAndroid
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
            $0.publicKey = Data([1, 2, 3, 4])
        }

        // 验证成功
        XCTAssertNoThrow(try discoveryHandler.handleBroadcast(validBroadcast))

        // 测试无效消息 - 空设备ID
        let invalidBroadcast = validBroadcast.with {
            $0.deviceID = ""
        }

        XCTAssertThrowsError(try discoveryHandler.handleBroadcast(invalidBroadcast)) { error in
            XCTAssertTrue(error is ProtocolError)
            if case let .invalidFormat(let message) = error {
                XCTAssertTrue(message.contains("设备ID不能为空"))
            } else {
                XCTFail("Expected ProtocolError.invalidFormat")
            }
        }
    }

    func testDiscoveryHandlerScanRequest() {
        let scanRequest = discoveryHandler.createScanRequest(timeoutSeconds: 30)

        XCTAssertEqual(scanRequest.timeoutSeconds, 30)
        XCTAssertTrue(scanRequest.filterTypes.isEmpty)
        XCTAssertTrue(scanRequest.requiredCapabilities.isEmpty)
    }

    func testDiscoveryHandlerMessageSerialization() throws {
        let broadcast = DeviceBroadcast.with {
            $0.deviceID = "test-device"
            $0.deviceName = "Test Device"
            $0.deviceType = .deviceTypeAndroid
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let serialized = try broadcast.serializedData()
        let result = try discoveryHandler.handleMessage(serialized)

        XCTAssertTrue(result.isEmpty) // 成功处理返回空数据
    }

    func testDiscoveryHandlerMessageValidation() throws {
        let validBroadcast = DeviceBroadcast.with {
            $0.deviceID = "test-device"
            $0.deviceName = "Test Device"
            $0.deviceType = .deviceTypeAndroid
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let serialized = try validBroadcast.serializedData()

        // 验证成功
        XCTAssertNoThrow(try discoveryHandler.validateMessage(serialized))

        // 测试无效消息
        let invalidBroadcast = validBroadcast.with {
            $0.deviceID = ""
        }

        let invalidSerialized = try invalidBroadcast.serializedData()

        XCTAssertThrowsError(try discoveryHandler.validateMessage(invalidSerialized))
    }

    // MARK: - Pairing Handler Tests

    func testPairingHandlerInitiatePairing() throws {
        let pairingRequest = try pairingHandler.initiatePairing(
            targetId: "target-device",
            deviceName: "Test Initiator"
        )

        XCTAssertEqual(pairingRequest.targetID, "target-device")
        XCTAssertEqual(pairingRequest.deviceName, "Test Initiator")
        XCTAssertFalse(pairingRequest.initiatorID.isEmpty)
        XCTAssertFalse(pairingRequest.nonce.isEmpty)
        XCTAssertGreaterThan(pairingRequest.timestamp, 0)
    }

    func testPairingHandlerValidateResponse() throws {
        let validResponse = PairingResponse.with {
            $0.responderID = "responder-device"
            $0.initiatorID = "initiator-device"
            $0.publicKey = Data("test-public-key".utf8)
            $0.signedNonce = Data("signed-nonce".utf8)
            $0.sharedSecret = Data("shared-secret".utf8)
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        // 验证成功
        XCTAssertNoThrow(try pairingHandler.handlePairingResponse(validResponse))

        // 测试无效响应 - 空响应者ID
        let invalidResponse = validResponse.with {
            $0.responderID = ""
        }

        XCTAssertThrowsError(try pairingHandler.handlePairingResponse(invalidResponse)) { error in
            XCTAssertTrue(error is ProtocolError)
            if case let .invalidFormat(let message) = error {
                XCTAssertTrue(message.contains("响应者ID不能为空"))
            } else {
                XCTFail("Expected ProtocolError.invalidFormat")
            }
        }
    }

    func testPairingHandlerMessageSerialization() throws {
        let pairingResponse = PairingResponse.with {
            $0.responderID = "responder-device"
            $0.initiatorID = "initiator-device"
            $0.publicKey = Data("test-public-key".utf8)
            $0.signedNonce = Data("signed-nonce".utf8)
            $0.sharedSecret = Data("shared-secret".utf8)
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let serialized = try pairingResponse.serializedData()
        let result = try pairingHandler.handleMessage(serialized)

        XCTAssertTrue(result.isEmpty) // 成功处理返回空数据
    }

    // MARK: - Sync Handler Tests

    func testSyncHandlerCreateSyncMessage() throws {
        let clipboardData = ClipboardData.with {
            $0.dataID = "test-data-id"
            $0.type = .dataTypeText
            $0.content = Data("Hello, World!".utf8)
            $0.metadata["source"] = "test-app"
            $0.createdAt = UInt64(Date().timeIntervalSince1970 * 1000)
            $0.sourceApp = "TestApp"
        }

        let syncMessage = try syncHandler.createSyncMessage(
            data: clipboardData,
            operation: .syncCreate
        )

        XCTAssertEqual(syncMessage.deviceID, "local-device-id")
        XCTAssertEqual(syncMessage.operation, .syncCreate)
        XCTAssertEqual(syncMessage.data.dataID, "test-data-id")
        XCTAssertGreaterThan(syncMessage.timestamp, 0)
    }

    func testSyncHandlerHandleSyncMessage() throws {
        let clipboardData = ClipboardData.with {
            $0.dataID = "test-data-id"
            $0.type = .dataTypeText
            $0.content = Data("Hello, World!".utf8)
            $0.createdAt = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let syncMessage = SyncMessage.with {
            $0.deviceID = "test-device"
            $0.operation = .syncCreate
            $0.data = clipboardData
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let syncAck = try syncHandler.handleSyncMessage(syncMessage)

        XCTAssertEqual(syncAck.dataID, "test-data-id")
        XCTAssertTrue(syncAck.success)
        XCTAssertTrue(syncAck.errorMessage.isEmpty)
        XCTAssertGreaterThan(syncAck.timestamp, 0)
    }

    func testSyncHandlerMessageValidation() throws {
        let validClipboardData = ClipboardData.with {
            $0.dataID = "test-data-id"
            $0.type = .dataTypeText
            $0.content = Data("Hello, World!".utf8)
            $0.createdAt = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let validSyncMessage = SyncMessage.with {
            $0.deviceID = "test-device"
            $0.operation = .syncCreate
            $0.data = validClipboardData
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let serialized = try validSyncMessage.serializedData()

        // 验证成功
        XCTAssertNoThrow(try syncHandler.validateMessage(serialized))

        // 测试无效消息 - 空设备ID
        let invalidSyncMessage = validSyncMessage.with {
            $0.deviceID = ""
        }

        let invalidSerialized = try invalidSyncMessage.serializedData()

        XCTAssertThrowsError(try syncHandler.validateMessage(invalidSerialized))
    }

    func testSyncHandlerLargeDataWithChunks() throws {
        let largeContent = String(repeating: "A", count: 10000) // 10KB 数据
        let largeData = Data(largeContent.utf8)

        let clipboardData = ClipboardData.with {
            $0.dataID = "large-data-id"
            $0.type = .dataTypeText
            $0.content = largeData
            $0.createdAt = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        // 创建数据分片
        let chunkSize = 1000
        var chunks: [DataChunk] = []

        for (index, start) in stride(from: 0, to: largeData.count, by: chunkSize).enumerated() {
            let end = min(start + chunkSize, largeData.count)
            let chunkData = largeData[start..<end]

            let chunk = DataChunk.with {
                $0.dataID = "large-data-id"
                $0.chunkIndex = UInt32(index)
                $0.totalChunks = UInt32((largeData.count + chunkSize - 1) / chunkSize)
                $0.chunkData = chunkData
                $0.checksum = Data("checksum-\(index)".utf8)
            }

            chunks.append(chunk)
        }

        let syncMessage = SyncMessage.with {
            $0.deviceID = "test-device"
            $0.operation = .syncCreate
            $0.data = clipboardData
            $0.chunks = chunks
            $0.timestamp = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let syncAck = try syncHandler.handleSyncMessage(syncMessage)

        XCTAssertEqual(syncAck.dataID, "large-data-id")
        XCTAssertTrue(syncAck.success)
    }

    func testProtocolHandlersMalformedMessages() {
        let malformedData = Data([0x01, 0x02, 0x03, 0x04])

        // 测试发现处理器
        XCTAssertThrowsError(try discoveryHandler.handleMessage(malformedData)) { error in
            XCTAssertTrue(error is ProtocolError)
        }

        // 测试配对处理器
        XCTAssertThrowsError(try pairingHandler.handleMessage(malformedData)) { error in
            XCTAssertTrue(error is ProtocolError)
        }

        // 测试同步处理器
        XCTAssertThrowsError(try syncHandler.handleMessage(malformedData)) { error in
            XCTAssertTrue(error is ProtocolError)
        }
    }

    // MARK: - Device Broadcast Extensions Tests

    func testDeviceBroadcastCapabilities() {
        let broadcast = DeviceBroadcast.with {
            $0.deviceID = "test-device"
            $0.deviceName = "Test Device"
            $0.deviceType = .deviceTypeAndroid
            $0.capabilities = [
                .capabilityClipboardRead,
                .capabilityClipboardWrite,
                .capabilityEncryption
            ]
        }

        XCTAssertTrue(broadcast.hasCapability(.capabilityClipboardRead))
        XCTAssertTrue(broadcast.hasCapability(.capabilityClipboardWrite))
        XCTAssertTrue(broadcast.hasCapability(.capabilityEncryption))
        XCTAssertFalse(broadcast.hasCapability(.capabilityFileTransfer))
    }

    // MARK: - Pairing Status Extensions Tests

    func testPairingStatusProperties() {
        XCTAssertEqual(PairingStatus.pairingCompleted.isCompleted, true)
        XCTAssertEqual(PairingStatus.pairingFailed.isFailed, true)
        XCTAssertEqual(PairingStatus.pairingPending.isPending, true)
        XCTAssertEqual(PairingStatus.pairingInitiated.isPending, true)
        XCTAssertEqual(PairingStatus.pairingConfirmed.isCompleted, false)
    }

    // MARK: - Sync Message Extensions Tests

    func testSyncMessageProperties() {
        let clipboardData = ClipboardData.with {
            $0.dataID = "test-data"
            $0.type = .dataTypeText
            $0.content = Data("test content".utf8)
            $0.createdAt = UInt64(Date().timeIntervalSince1970 * 1000)
        }

        let syncMessage = SyncMessage.with {
            $0.deviceID = "test-device"
            $0.operation = .syncCreate
            $0.data = clipboardData
        }

        XCTAssertEqual(syncMessage.totalSize, 0) // 无分片时大小为0
        XCTAssertFalse(syncMessage.requiresChunking)

        // 添加分片
        let chunk = DataChunk.with {
            $0.dataID = "test-data"
            $0.chunkIndex = 0
            $0.totalChunks = 1
            $0.chunkData = Data("chunk data".utf8)
        }

        let syncMessageWithChunks = syncMessage.with {
            $0.chunks = [chunk]
        }

        XCTAssertEqual(syncMessageWithChunks.totalSize, "chunk data".count)
        XCTAssertTrue(syncMessageWithChunks.requiresChunking)
    }

    // MARK: - Data Chunk Validation Tests

    func testDataChunkChecksum() {
        let chunk = DataChunk.with {
            $0.dataID = "test-data"
            $0.chunkIndex = 0
            $0.totalChunks = 1
            $0.chunkData = Data("test data".utf8)
            $0.checksum = SHA256.hash(data: Data("test data".utf8))
        }

        XCTAssertTrue(chunk.verifyChecksum())

        // 修改数据使校验和不匹配
        let invalidChunk = chunk.with {
            $0.chunkData = Data("invalid data".utf8)
        }

        XCTAssertFalse(invalidChunk.verifyChecksum())
    }
}