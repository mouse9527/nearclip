package com.nearclip.data.protocol

import com.nearclip.data.network.proto.*
import org.junit.Test
import org.junit.Assert.*
import org.junit.Before

class ProtocolHandlerTest {

    private lateinit var discoveryHandler: DiscoveryHandler
    private lateinit var pairingHandler: PairingHandler
    private lateinit var syncHandler: SyncHandler

    @Before
    fun setUp() {
        discoveryHandler = DiscoveryHandler()
        pairingHandler = PairingHandler()
        syncHandler = SyncHandler()
    }

    @Test
    fun `discovery handler validates broadcast message correctly`() {
        // 创建有效的广播消息
        val validBroadcast = DeviceBroadcast.newBuilder()
            .setDeviceId("test-device")
            .setDeviceName("Test Device")
            .setDeviceType(DeviceType.DEVICE_TYPE_ANDROID)
            .setTimestamp(System.currentTimeMillis())
            .build()

        // 验证成功
        val result = discoveryHandler.handleBroadcast(validBroadcast)
        assertTrue(result.isSuccess)

        // 测试无效消息 - 空设备ID
        val invalidBroadcast = validBroadcast.toBuilder()
            .setDeviceId("")
            .build()

        val invalidResult = discoveryHandler.handleBroadcast(invalidBroadcast)
        assertTrue(invalidResult.isFailure)
        assertTrue(invalidResult.exceptionOrNull() is ProtocolError)
    }

    @Test
    fun `discovery handler creates scan request correctly`() {
        val scanRequest = discoveryHandler.createScanRequest(30)

        assertEquals(30, scanRequest.timeoutSeconds)
        assertTrue(scanRequest.filterTypesList.isEmpty())
        assertTrue(scanRequest.requiredCapabilitiesList.isEmpty())
    }

    @Test
    fun `discovery handler handles message serialization correctly`() {
        val broadcast = DeviceBroadcast.newBuilder()
            .setDeviceId("test-device")
            .setDeviceName("Test Device")
            .setDeviceType(DeviceType.DEVICE_TYPE_ANDROID)
            .setTimestamp(System.currentTimeMillis())
            .build()

        val serialized = broadcast.toByteArray()
        val result = discoveryHandler.handleMessage(serialized)

        assertTrue(result.isSuccess)
    }

    @Test
    fun `discovery handler validates message correctly`() {
        val validBroadcast = DeviceBroadcast.newBuilder()
            .setDeviceId("test-device")
            .setDeviceName("Test Device")
            .setDeviceType(DeviceType.DEVICE_TYPE_ANDROID)
            .setTimestamp(System.currentTimeMillis())
            .build()

        val serialized = validBroadcast.toByteArray()
        val result = discoveryHandler.validateMessage(serialized)

        assertTrue(result.isSuccess)

        // 测试无效消息
        val invalidBroadcast = validBroadcast.toBuilder()
            .setDeviceId("")
            .build()

        val invalidSerialized = invalidBroadcast.toByteArray()
        val invalidResult = discoveryHandler.validateMessage(invalidSerialized)

        assertTrue(invalidResult.isFailure)
    }

    @Test
    fun `pairing handler initiates pairing correctly`() {
        val pairingRequest = pairingHandler.initiatePairing("target-device", "Test Initiator")

        assertEquals("target-device", pairingRequest.targetId)
        assertEquals("Test Initiator", pairingRequest.deviceName)
        assertTrue(pairingRequest.initiatorId.isNotEmpty())
        assertTrue(pairingRequest.nonce.size > 0)
        assertTrue(pairingRequest.timestamp > 0)
    }

    @Test
    fun `pairing handler validates pairing response correctly`() {
        val validResponse = PairingResponse.newBuilder()
            .setResponderId("responder-device")
            .setInitiatorId("initiator-device")
            .setPublicKey(ByteString.copyFromUtf8("test-public-key"))
            .setSignedNonce(ByteString.copyFromUtf8("signed-nonce"))
            .setSharedSecret(ByteString.copyFromUtf8("shared-secret"))
            .setTimestamp(System.currentTimeMillis())
            .build()

        val result = pairingHandler.handlePairingResponse(validResponse)
        assertTrue(result.isSuccess)

        // 测试无效响应 - 空响应者ID
        val invalidResponse = validResponse.toBuilder()
            .setResponderId("")
            .build()

        val invalidResult = pairingHandler.handlePairingResponse(invalidResponse)
        assertTrue(invalidResult.isFailure)
    }

    @Test
    fun `pairing handler handles message serialization correctly`() {
        val pairingResponse = PairingResponse.newBuilder()
            .setResponderId("responder-device")
            .setInitiatorId("initiator-device")
            .setPublicKey(ByteString.copyFromUtf8("test-public-key"))
            .setSignedNonce(ByteString.copyFromUtf8("signed-nonce"))
            .setSharedSecret(ByteString.copyFromUtf8("shared-secret"))
            .setTimestamp(System.currentTimeMillis())
            .build()

        val serialized = pairingResponse.toByteArray()
        val result = pairingHandler.handleMessage(serialized)

        assertTrue(result.isSuccess)
    }

    @Test
    fun `sync handler creates sync message correctly`() {
        val clipboardData = ClipboardData.newBuilder()
            .setDataId("test-data-id")
            .setType(DataType.DATA_TYPE_TEXT)
            .setContent(ByteString.copyFromUtf8("Hello, World!"))
            .putMetadata("source", "test-app")
            .setCreatedAt(System.currentTimeMillis())
            .setExpiresAt(0) // 永不过期
            .setSourceApp("TestApp")
            .build()

        val syncMessage = syncHandler.createSyncMessage(clipboardData, SyncOperation.SYNC_CREATE)

        assertEquals("local-device-id", syncMessage.deviceId)
        assertEquals(SyncOperation.SYNC_CREATE, syncMessage.operation)
        assertEquals("test-data-id", syncMessage.data.dataId)
        assertTrue(syncMessage.timestamp > 0)
    }

    @Test
    fun `sync handler handles sync message correctly`() {
        val clipboardData = ClipboardData.newBuilder()
            .setDataId("test-data-id")
            .setType(DataType.DATA_TYPE_TEXT)
            .setContent(ByteString.copyFromUtf8("Hello, World!"))
            .setCreatedAt(System.currentTimeMillis())
            .build()

        val syncMessage = SyncMessage.newBuilder()
            .setDeviceId("test-device")
            .setOperation(SyncOperation.SYNC_CREATE)
            .setData(clipboardData)
            .setTimestamp(System.currentTimeMillis())
            .build()

        val result = syncHandler.handleSyncMessage(syncMessage)
        assertTrue(result.isSuccess)
        assertEquals("test-data-id", result.dataId)
        assertTrue(result.success)
    }

    @Test
    fun `sync handler validates sync message correctly`() {
        val validClipboardData = ClipboardData.newBuilder()
            .setDataId("test-data-id")
            .setType(DataType.DATA_TYPE_TEXT)
            .setContent(ByteString.copyFromUtf8("Hello, World!"))
            .setCreatedAt(System.currentTimeMillis())
            .build()

        val validSyncMessage = SyncMessage.newBuilder()
            .setDeviceId("test-device")
            .setOperation(SyncOperation.SYNC_CREATE)
            .setData(validClipboardData)
            .setTimestamp(System.currentTimeMillis())
            .build()

        val serialized = validSyncMessage.toByteArray()
        val result = syncHandler.validateMessage(serialized)

        assertTrue(result.isSuccess)

        // 测试无效消息 - 空设备ID
        val invalidSyncMessage = validSyncMessage.toBuilder()
            .setDeviceId("")
            .build()

        val invalidSerialized = invalidSyncMessage.toByteArray()
        val invalidResult = syncHandler.validateMessage(invalidSerialized)

        assertTrue(invalidResult.isFailure)
    }

    @Test
    fun `sync handler handles large data with chunks correctly`() {
        val largeContent = "A".repeat(10000) // 10KB 数据

        val clipboardData = ClipboardData.newBuilder()
            .setDataId("large-data-id")
            .setType(DataType.DATA_TYPE_TEXT)
            .setContent(ByteString.copyFromUtf8(largeContent))
            .setCreatedAt(System.currentTimeMillis())
            .build()

        // 创建数据分片
        val chunkSize = 1000
        val chunks = mutableListOf<DataChunk>()

        for (i in 0 until largeContent.length step chunkSize) {
            val endIndex = minOf(i + chunkSize, largeContent.length)
            val chunkContent = largeContent.substring(i, endIndex)

            val chunk = DataChunk.newBuilder()
                .setDataId("large-data-id")
                .setChunkIndex((i / chunkSize).toUInt())
                .setTotalChunks((largeContent.length / chunkSize + 1).toUInt())
                .setChunkData(ByteString.copyFromUtf8(chunkContent))
                .setChecksum(ByteString.copyFromUtf8("checksum-$i"))
                .build()

            chunks.add(chunk)
        }

        val syncMessage = SyncMessage.newBuilder()
            .setDeviceId("test-device")
            .setOperation(SyncOperation.SYNC_CREATE)
            .setData(clipboardData)
            .addAllChunks(chunks)
            .setTimestamp(System.currentTimeMillis())
            .build()

        val result = syncHandler.handleSyncMessage(syncMessage)
        assertTrue(result.isSuccess)
        assertEquals("large-data-id", result.dataId)
    }

    @Test
    fun `protocol handlers handle malformed messages gracefully`() {
        val malformedData = byteArrayOf(0x01, 0x02, 0x03, 0x04)

        // 测试发现处理器
        val discoveryResult = discoveryHandler.handleMessage(malformedData)
        assertTrue(discoveryResult.isFailure)
        assertTrue(discoveryResult.exceptionOrNull() is ProtocolError)

        // 测试配对处理器
        val pairingResult = pairingHandler.handleMessage(malformedData)
        assertTrue(pairingResult.isFailure)
        assertTrue(pairingResult.exceptionOrNull() is ProtocolError)

        // 测试同步处理器
        val syncResult = syncHandler.handleMessage(malformedData)
        assertTrue(syncResult.isFailure)
        assertTrue(syncResult.exceptionOrNull() is ProtocolError)
    }
}