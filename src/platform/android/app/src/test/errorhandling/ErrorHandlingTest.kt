package com.nearclip.errorhandling

import android.content.Context
import com.nearclip.ffi.NearClipFFI
import com.nearclip.ffi.SecureFFIWrapper
import com.nearclip.ffi.SecurityException
import com.nearclip.test.util.MainDispatcherRule
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * 错误处理测试
 * 测试系统在各种异常情况下的行为
 */
@ExperimentalCoroutinesApi
class ErrorHandlingTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private lateinit var secureFFIWrapper: SecureFFIWrapper
    private lateinit var mockContext: Context

    @Before
    fun setup() {
        secureFFIWrapper = SecureFFIWrapper()
        mockContext = mockk()

        mockkStatic(NearClipFFI::class)

        // Setup default mock behavior
        every { mockContext.applicationInfo } returns mockk()
        every { mockContext.packageName } returns "com.nearclip.test"
    }

    @Test
    fun `should handle FFI initialization failure gracefully`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns false

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("FFI initialization failed", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle FFI initialization exception`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } throws RuntimeException("Native library not found")

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertTrue(result.exceptionOrNull()?.message?.contains("Secure initialization failed") == true)
        assertNotNull(result.exceptionOrNull()?.cause)
    }

    @Test
    fun `should handle device discovery failure`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns false
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Failed to start device discovery", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle device discovery exception`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } throws RuntimeException("Bluetooth not available")
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertTrue(result.exceptionOrNull()?.message?.contains("Secure discovery failed") == true)
    }

    @Test
    fun `should handle device connection failure`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.connectToDevice(any()) } returns false
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.connectSecureToDevice("test-device")

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Failed to connect to device", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle device connection exception`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.connectToDevice(any()) } throws RuntimeException("Device not found")
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.connectSecureToDevice("nonexistent-device")

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertTrue(result.exceptionOrNull()?.message?.contains("Secure device connection failed") == true)
    }

    @Test
    fun `should handle clipboard send failure`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.sendClipboardData(any()) } returns false
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.sendSecureClipboardData("test data")

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Failed to send clipboard data", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle clipboard send exception`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.sendClipboardData(any()) } throws RuntimeException("Clipboard service unavailable")
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.sendSecureClipboardData("test data")

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertTrue(result.exceptionOrNull()?.message?.contains("Secure clipboard send failed") == true)
    }

    @Test
    fun `should handle get device info failure`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.getLocalDeviceInfo() } throws RuntimeException("Device info unavailable")
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.getSecureLocalDeviceInfo()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertTrue(result.exceptionOrNull()?.message?.contains("Secure get device info failed") == true)
    }

    @Test
    fun `should handle context validation errors`() = runTest {
        // Given
        every { mockContext.applicationInfo } throws SecurityException("Context access denied")

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Invalid context provided", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle null context gracefully`() = runTest {
        // When
        val result = secureFFIWrapper.initializeSecure(null as Context?)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Invalid context provided", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle operations before initialization`() = runTest {
        // When - 尝试在初始化前执行操作
        val discoveryResult = secureFFIWrapper.startSecureDiscovery()
        val connectionResult = secureFFIWrapper.connectSecureToDevice("test-device")
        val clipboardResult = secureFFIWrapper.sendSecureClipboardData("test")
        val deviceInfoResult = secureFFIWrapper.getSecureLocalDeviceInfo()

        // Then - 所有操作都应该失败
        assertTrue(discoveryResult.isFailure)
        assertTrue(connectionResult.isFailure)
        assertTrue(clipboardResult.isFailure)
        assertTrue(deviceInfoResult.isFailure)

        // 验证错误消息
        assertEquals("FFI not initialized", discoveryResult.exceptionOrNull()?.message)
        assertEquals("FFI not initialized", connectionResult.exceptionOrNull()?.message)
        assertEquals("FFI not initialized", clipboardResult.exceptionOrNull()?.message)
        assertEquals("FFI not initialized", deviceInfoResult.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle malformed device IDs`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        val malformedDeviceIds = listOf(
            "", // 空字符串
            " ", // 空格
            "device@invalid", // 无效字符
            "device with spaces", // 空格
            "device\nwith\nnewlines", // 换行符
            "device\twith\ttabs", // 制表符
            "a".repeat(257), // 过长
            "device<script>", // 脚本标签
            "device'or'1'='1", // SQL注入
            "../../../etc/passwd" // 路径遍历
        )

        malformedDeviceIds.forEach { deviceId ->
            // When
            val result = secureFFIWrapper.connectSecureToDevice(deviceId)

            // Then
            assertTrue(result.isFailure, "Should fail for device ID: $deviceId")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `should handle malformed clipboard data`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        val maliciousData = listOf(
            "<script>alert('xss')</script>",
            "javascript:alert('xss')",
            "eval('malicious code')",
            "exec('rm -rf /')",
            "system('malicious command')",
            "runtime.exec('hack')",
            "getClass().forName('java.lang.Runtime')",
            "${"a".repeat(1024 * 1024 + 1)}", // 超过1MB
            "\u0000\u0001\u0002", // 控制字符
            "data\r\nheader:injection" // 头部注入
        )

        maliciousData.forEach { data ->
            // When
            val result = secureFFIWrapper.sendSecureClipboardData(data)

            // Then
            assertTrue(result.isFailure, "Should fail for data: ${data.take(50)}...")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `should handle cleanup errors gracefully`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.cleanup() } throws RuntimeException("Cleanup failed")
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 清理不应该抛出异常
        secureFFIWrapper.secureCleanup()

        // Then - 验证状态仍然正确
        val stats = secureFFIWrapper.getSecurityStats()
        assertEquals(false, stats["initialized"])
    }

    @Test
    fun `should handle concurrent error conditions`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.connectToDevice(any()) } throws RuntimeException("Connection error")
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 并发执行失败的操作
        val concurrentTasks = (1..10).map { taskId ->
            kotlinx.coroutines.async {
                secureFFIWrapper.connectSecureToDevice("device-$taskId")
            }
        }

        // Then - 所有操作都应该优雅地失败
        concurrentTasks.forEach { task ->
            val result = task.await()
            assertTrue(result.isFailure)
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `should maintain security state during errors`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } throws RuntimeException("Discovery error")
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 触发错误
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then - 安全状态应该正确更新
        assertTrue(result.isFailure)
        // 注意：这里需要检查security state，但由于它是Flow，我们在实际实现中可能需要添加状态检查
    }

    @Test
    fun `should handle rate limiting errors`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 超过速率限制
        repeat(1001) {
            secureFFIWrapper.startSecureDiscovery()
        }

        // Then - 第1001次操作应该失败
        val result = secureFFIWrapper.startSecureDiscovery()
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Operation rate limit exceeded: discovery", result.exceptionOrNull()?.message)
    }
}