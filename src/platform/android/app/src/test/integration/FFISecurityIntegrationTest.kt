package com.nearclip.integration

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
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * FFI安全集成测试
 * 测试完整的FFI安全工作流
 */
@ExperimentalCoroutinesApi
class FFISecurityIntegrationTest {

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
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns true
        every { NearClipFFI.stopDeviceDiscovery() } returns true
        every { NearClipFFI.connectToDevice(any()) } returns true
        every { NearClipFFI.disconnectFromDevice(any()) } returns true
        every { NearClipFFI.sendClipboardData(any()) } returns true
        every { NearClipFFI.cleanup() } returns Unit
    }

    @Test
    fun `complete secure workflow should work end to end`() = runTest {
        // 1. 初始化
        val initResult = secureFFIWrapper.initializeSecure(mockContext)
        assertTrue(initResult.isSuccess)

        // 2. 开始设备发现
        val discoveryResult = secureFFIWrapper.startSecureDiscovery()
        assertTrue(discoveryResult.isSuccess)

        // 3. 连接到设备
        val connectionResult = secureFFIWrapper.connectSecureToDevice("test-device-123")
        assertTrue(connectionResult.isSuccess)

        // 4. 发送剪贴板数据
        val clipboardResult = secureFFIWrapper.sendSecureClipboardData("Hello, World!")
        assertTrue(clipboardResult.isSuccess)

        // 5. 获取设备信息
        val deviceInfoResult = secureFFIWrapper.getSecureLocalDeviceInfo()
        assertTrue(deviceInfoResult.isSuccess)

        // 6. 清理
        secureFFIWrapper.secureCleanup()

        // 验证统计信息
        val stats = secureFFIWrapper.getSecurityStats()
        assertFalse(stats["initialized"] as Boolean)
    }

    @Test
    fun `security validation should prevent malicious operations`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When & Then - 尝试恶意操作
        val maliciousDeviceIds = listOf(
            "device<script>alert('xss')</script>",
            "device'or'1'='1",
            "../../../etc/passwd",
            "device\x00null",
            "a".repeat(300) // 过长
        )

        maliciousDeviceIds.forEach { deviceId ->
            val result = secureFFIWrapper.connectSecureToDevice(deviceId)
            assertTrue(result.isFailure, "Should reject malicious device ID: $deviceId")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }

        val maliciousData = listOf(
            "<script>malicious()</script>",
            "eval('attack')",
            "exec('rm -rf /')",
            "${"a".repeat(2 * 1024 * 1024)}" // 超过1MB
        )

        maliciousData.forEach { data ->
            val result = secureFFIWrapper.sendSecureClipboardData(data)
            assertTrue(result.isFailure, "Should reject malicious data: ${data.take(50)}...")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `should handle FFI errors gracefully and maintain security`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.connectToDevice(any()) } throws RuntimeException("Native error")
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.connectSecureToDevice("test-device")

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)

        // 验证系统仍然可以处理其他操作
        val discoveryResult = secureFFIWrapper.startSecureDiscovery()
        assertTrue(discoveryResult.isSuccess)
    }

    @Test
    fun `should enforce rate limiting across operations`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 快速执行大量操作
        repeat(500) {
            secureFFIWrapper.startSecureDiscovery()
            secureFFIWrapper.connectSecureToDevice("device-$it")
        }

        // Then - 最终操作应该被限制
        val discoveryLimitResult = secureFFIWrapper.startSecureDiscovery()
        val connectionLimitResult = secureFFIWrapper.connectSecureToDevice("limited-device")

        assertTrue(discoveryLimitResult.isFailure)
        assertTrue(connectionLimitResult.isFailure)
        assertTrue(discoveryLimitResult.exceptionOrNull() is SecurityException)
        assertTrue(connectionLimitResult.exceptionOrNull() is SecurityException)
    }

    @Test
    fun `should maintain security state across operations`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } throws RuntimeException("Discovery failed")
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 触发错误
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then - 系统应该仍然可以处理其他操作
        assertTrue(result.isFailure)

        val connectionResult = secureFFIWrapper.connectSecureToDevice("backup-device")
        assertTrue(connectionResult.isSuccess)

        // 验证统计信息
        val stats = secureFFIWrapper.getSecurityStats()
        assertTrue(stats["initialized"] as Boolean)
        assertNotNull(stats["operation_counts"])
    }

    @Test
    fun `should handle context security validation`() = runTest {
        // When - 尝试使用无效上下文
        val nullContextResult = secureFFIWrapper.initializeSecure(null)
        assertTrue(nullContextResult.isFailure)
        assertTrue(nullContextResult.exceptionOrNull() is SecurityException)

        // Given - 模拟上下文访问被拒绝
        every { mockContext.applicationInfo } throws SecurityException("Access denied")
        val deniedContextResult = secureFFIWrapper.initializeSecure(mockContext)
        assertTrue(deniedContextResult.isFailure)
        assertTrue(deniedContextResult.exceptionOrNull() is SecurityException)
    }

    @Test
    fun `should properly cleanup and reset security state`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // 执行一些操作
        secureFFIWrapper.startSecureDiscovery()
        secureFFIWrapper.connectSecureToDevice("test-device")
        secureFFIWrapper.sendSecureClipboardData("test data")

        // 验证操作已被记录
        val statsBeforeCleanup = secureFFIWrapper.getSecurityStats()
        assertTrue(statsBeforeCleanup["initialized"] as Boolean)
        assertTrue((statsBeforeCleanup["operation_counts"] as Map<*, *>).isNotEmpty())

        // When - 清理
        secureFFIWrapper.secureCleanup()

        // Then - 状态应该被重置
        val statsAfterCleanup = secureFFIWrapper.getSecurityStats()
        assertFalse(statsAfterCleanup["initialized"] as Boolean)
        assertTrue((statsAfterCleanup["operation_counts"] as Map<*, *>).isEmpty())

        // 验证清理后无法执行操作
        val postCleanupResult = secureFFIWrapper.startSecureDiscovery()
        assertTrue(postCleanupResult.isFailure)
        assertEquals("FFI not initialized", postCleanupResult.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle concurrent secure operations safely`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 并发执行多个操作
        val concurrentTasks = (1..20).map { taskId ->
            kotlinx.coroutines.async {
                val results = mutableListOf<Boolean>()

                // 执行设备发现
                val discoveryResult = secureFFIWrapper.startSecureDiscovery()
                results.add(discoveryResult.isSuccess)

                // 执行设备连接
                val connectionResult = secureFFIWrapper.connectSecureToDevice("device-$taskId")
                results.add(connectionResult.isSuccess)

                // 发送剪贴板数据
                val clipboardResult = secureFFIWrapper.sendSecureClipboardData("data-$taskId")
                results.add(clipboardResult.isSuccess)

                results
            }
        }

        // Then - 所有操作都应该成功
        val allResults = concurrentTasks.map { it.await() }.flatten()
        assertTrue(allResults.all { it }, "All concurrent operations should succeed")
    }

    @Test
    fun `should validate input sanitization across data types`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // Device ID验证测试
        val invalidDeviceIds = listOf(
            "", // 空
            " ", // 空格
            "a".repeat(257), // 过长
            "device@invalid", // 无效字符
            "device\nwith\nnewlines", // 换行符
            "device\twith\ttabs", // 制表符
            "device<script>", // 脚本
            "device'or'1'='1" // SQL注入
        )

        invalidDeviceIds.forEach { deviceId ->
            val result = secureFFIWrapper.connectSecureToDevice(deviceId)
            assertTrue(result.isFailure, "Should reject device ID: $deviceId")
        }

        // 剪贴板数据验证测试
        val invalidClipboardData = listOf(
            "${"a".repeat(1024 * 1024 + 1)}", // 超过1MB
            "<script>alert('xss')</script>", // XSS
            "javascript:alert('xss')", // JavaScript
            "eval('malicious')", // 代码执行
            "exec('rm -rf /')", // 系统命令
            "system('hack')" // 系统调用
        )

        invalidClipboardData.forEach { data ->
            val result = secureFFIWrapper.sendSecureClipboardData(data)
            assertTrue(result.isFailure, "Should reject clipboard data: ${data.take(50)}...")
        }
    }
}