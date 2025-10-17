package com.nearclip

import org.junit.Test

import org.junit.Assert.*

/**
 * NearClip基础单元测试示例
 *
 * 这是一个示例测试文件，展示了如何编写基础的单元测试。
 * 开发团队可以基于这个模式创建更多的测试。
 */
class ExampleUnitTest {

    @Test
    fun addition_isCorrect() {
        assertEquals(4, 2 + 2)
    }

    @Test
    fun testDeviceIdGeneration() {
        // 测试设备ID生成逻辑
        val deviceId = "test-device-123"
        assertNotNull(deviceId)
        assertTrue(deviceId.isNotEmpty())
        assertTrue(deviceId.contains("test"))
    }

    @Test
    fun testConnectionStatusValues() {
        // 验证连接状态枚举值
        val statuses = listOf("CONNECTED", "CONNECTING", "DISCONNECTED", "ERROR")
        statuses.forEach { status ->
            assertNotNull(status)
            assertTrue(status.isNotEmpty())
        }
    }
}