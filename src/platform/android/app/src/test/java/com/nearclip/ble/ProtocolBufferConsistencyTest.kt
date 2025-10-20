package com.nearclip.ble

import com.nearclip.services.ble.ProtocolBufferMessageHandler
import com.nearclip.services.ble.TestMessage
import com.nearclip.services.ble.MessageType
import org.junit.Test
import org.junit.Assert.*
import java.util.*

/**
 * Protocol Buffers跨平台一致性测试
 * 验证Android和Mac端的消息序列化/反序列化一致性
 */
class ProtocolBufferConsistencyTest {

    private val protocolHandler = ProtocolBufferMessageHandler()

    @Test
    fun `测试ping消息序列化反序列化一致性`() {
        // Given: 创建一个PING消息
        val originalMessage = TestMessage(
            messageId = "ping-test-001",
            type = MessageType.PING,
            payload = "",
            timestamp = 1640995200000L, // 2022-01-01 00:00:00 UTC
            sequenceNumber = 1
        )

        // When: 序列化然后反序列化
        val serializedData = protocolHandler.serializeMessage(originalMessage)
        val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        assertNotNull("反序列化消息不应为null", deserializedMessage)
        assertEquals("消息ID应该一致", originalMessage.messageId, deserializedMessage?.messageId)
        assertEquals("消息类型应该一致", originalMessage.type, deserializedMessage?.type)
        assertEquals("消息载荷应该一致", originalMessage.payload, deserializedMessage?.payload)
        assertEquals("时间戳应该一致", originalMessage.timestamp, deserializedMessage?.timestamp)
        assertEquals("序列号应该一致", originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber)
    }

    @Test
    fun `测试data消息序列化反序列化一致性`() {
        // Given: 创建一个包含数据的消息
        val originalMessage = TestMessage(
            messageId = "data-test-002",
            type = MessageType.DATA,
            payload = "Hello from Android! This is a test message with special chars: 中文测试 🚀",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 5
        )

        // When: 序列化然后反序列化
        val serializedData = protocolHandler.serializeMessage(originalMessage)
        val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        assertNotNull("反序列化消息不应为null", deserializedMessage)
        assertEquals("消息ID应该一致", originalMessage.messageId, deserializedMessage?.messageId)
        assertEquals("消息类型应该一致", originalMessage.type, deserializedMessage?.type)
        assertEquals("消息载荷应该一致", originalMessage.payload, deserializedMessage?.payload)
        assertEquals("时间戳应该一致", originalMessage.timestamp, deserializedMessage?.timestamp)
        assertEquals("序列号应该一致", originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber)
    }

    @Test
    fun `测试ack消息序列化反序列化一致性`() {
        // Given: 创建一个ACK消息
        val originalMessage = TestMessage(
            messageId = "ack-test-003",
            type = MessageType.ACK,
            payload = "original-message-id-123",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 10
        )

        // When: 序列化然后反序列化
        val serializedData = protocolHandler.serializeMessage(originalMessage)
        val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        // Then: 消息应该完全一致
        assertNotNull("反序列化消息不应为null", deserializedMessage)
        assertEquals("消息ID应该一致", originalMessage.messageId, deserializedMessage?.messageId)
        assertEquals("消息类型应该一致", originalMessage.type, deserializedMessage?.type)
        assertEquals("消息载荷应该一致", originalMessage.payload, deserializedMessage?.payload)
        assertEquals("时间戳应该一致", originalMessage.timestamp, deserializedMessage?.timestamp)
        assertEquals("序列号应该一致", originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber)
    }

    @Test
    fun `测试向后兼容性 - 旧格式消息反序列化`() {
        // Given: 创建旧格式的消息字符串（模拟Mac端发送的旧格式）
        val oldFormatString = "old-format-001|DATA|Hello World|1640995200.0|1"
        val oldFormatData = oldFormatString.toByteArray(Charsets.UTF_8)

        // When: 尝试反序列化旧格式消息
        val deserializedMessage = protocolHandler.deserializeMessage(oldFormatData)

        // Then: 应该能够正确解析旧格式消息
        assertNotNull("应该能够解析旧格式消息", deserializedMessage)
        assertEquals("消息ID应该正确", "old-format-001", deserializedMessage?.messageId)
        assertEquals("消息类型应该正确", MessageType.DATA, deserializedMessage?.type)
        assertEquals("消息载荷应该正确", "Hello World", deserializedMessage?.payload)
        assertEquals("时间戳应该正确", 1640995200000L, deserializedMessage?.timestamp) // 秒转毫秒
        assertEquals("序列号应该正确", 1, deserializedMessage?.sequenceNumber)
    }

    @Test
    fun `测试消息验证功能`() {
        // Test valid message
        val validMessage = TestMessage(
            messageId = "valid-001",
            type = MessageType.PING,
            payload = "Valid payload",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 1
        )
        assertTrue("有效消息应该通过验证", protocolHandler.validateMessage(validMessage))

        // Test invalid message (empty ID)
        val invalidMessage1 = validMessage.copy(messageId = "")
        assertFalse("空ID消息应该验证失败", protocolHandler.validateMessage(invalidMessage1))

        // Test invalid message (negative timestamp)
        val invalidMessage2 = validMessage.copy(timestamp = -1L)
        assertFalse("负时间戳消息应该验证失败", protocolHandler.validateMessage(invalidMessage2))

        // Test invalid message (negative sequence number)
        val invalidMessage3 = validMessage.copy(sequenceNumber = -1)
        assertFalse("负序列号消息应该验证失败", protocolHandler.validateMessage(invalidMessage3))

        // Test invalid message (payload too large)
        val largePayload = "x".repeat(1025)
        val invalidMessage4 = validMessage.copy(payload = largePayload)
        assertFalse("过大载荷的消息应该验证失败", protocolHandler.validateMessage(invalidMessage4))
    }

    @Test
    fun `测试多种消息类型的序列化一致性`() {
        val messageTypes = listOf(MessageType.PING, MessageType.PONG, MessageType.DATA, MessageType.ACK)
        val baseTimestamp = System.currentTimeMillis()

        messageTypes.forEachIndexed { index, messageType ->
            val originalMessage = TestMessage(
                messageId = "consistency-test-$index",
                type = messageType,
                payload = "Test payload for $messageType",
                timestamp = baseTimestamp + index * 1000,
                sequenceNumber = index + 1
            )

            val serializedData = protocolHandler.serializeMessage(originalMessage)
            val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

            assertNotNull("消息类型 $messageType 反序列化不应为null", deserializedMessage)
            assertEquals("消息类型 $messageType ID应该一致", originalMessage.messageId, deserializedMessage?.messageId)
            assertEquals("消息类型 $messageType 应该一致", originalMessage.type, deserializedMessage?.type)
            assertEquals("消息类型 $messageType 载荷应该一致", originalMessage.payload, deserializedMessage?.payload)
            assertEquals("消息类型 $messageType 时间戳应该一致", originalMessage.timestamp, deserializedMessage?.timestamp)
            assertEquals("消息类型 $messageType 序列号应该一致", originalMessage.sequenceNumber, deserializedMessage?.sequenceNumber)
        }
    }

    @Test
    fun `测试特殊字符和Unicode消息`() {
        val specialMessages = listOf(
            "中文测试消息",
            "🚀 Emoji test 🎉",
            "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?",
            "Multi\nline\nmessage",
            "Tabs\tand\r\rcarriage\returns"
        )

        specialMessages.forEachIndexed { index, payload ->
            val originalMessage = TestMessage(
                messageId = "unicode-test-$index",
                type = MessageType.DATA,
                payload = payload,
                timestamp = System.currentTimeMillis(),
                sequenceNumber = index
            )

            val serializedData = protocolHandler.serializeMessage(originalMessage)
            val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

            assertNotNull("Unicode消息 $index 反序列化不应为null", deserializedMessage)
            assertEquals("Unicode消息 $index 载荷应该一致", originalMessage.payload, deserializedMessage?.payload)
        }
    }

    @Test
    fun `测试边界情况`() {
        // 空载荷消息
        val emptyPayloadMessage = TestMessage(
            messageId = "empty-payload-test",
            type = MessageType.PING,
            payload = "",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 0
        )

        val serializedData = protocolHandler.serializeMessage(emptyPayloadMessage)
        val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

        assertNotNull("空载荷消息反序列化不应为null", deserializedMessage)
        assertEquals("空载荷消息载荷应该为空", "", deserializedMessage?.payload)

        // 无效数据处理
        val invalidData = byteArrayOf(0x01, 0x02, 0x03)
        val invalidMessage = protocolHandler.deserializeMessage(invalidData)
        assertNull("无效数据应该返回null", invalidMessage)

        // 空数据处理
        val emptyData = ByteArray(0)
        val emptyMessage = protocolHandler.deserializeMessage(emptyData)
        assertNull("空数据应该返回null", emptyMessage)
    }
}