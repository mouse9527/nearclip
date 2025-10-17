package com.nearclip.ffi

import android.content.Context
import com.nearclip.test.util.MainDispatcherRule
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.verify
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlin.test.fail

@ExperimentalCoroutinesApi
class SecureFFIWrapperTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private lateinit var secureFFIWrapper: SecureFFIWrapper
    private lateinit var mockContext: Context

    @Before
    fun setup() {
        secureFFIWrapper = SecureFFIWrapper()
        mockContext = mockk()

        // Mock NearClipFFI
        mockkStatic(NearClipFFI::class)

        // Setup default mock behavior
        every { mockContext.applicationInfo } returns mockk()
        every { mockContext.packageName } returns "com.nearclip.test"
    }

    @Test
    fun `initializeSecure should succeed with valid context`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isSuccess)
        assertTrue(result.getOrNull() == true)
        assertEquals(SecurityState.Safe, secureFFIWrapper.securityState.first())
    }

    @Test
    fun `initializeSecure should fail with invalid context`() = runTest {
        // Given
        val invalidContext: Context? = null

        // When
        val result = secureFFIWrapper.initializeSecure(invalidContext!!)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Invalid context provided", result.exceptionOrNull()?.message)
    }

    @Test
    fun `initializeSecure should handle FFI initialization failure`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns false

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals(SecurityState.Error("Failed to initialize FFI"), secureFFIWrapper.securityState.first())
    }

    @Test
    fun `startSecureDiscovery should succeed when initialized`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then
        assertTrue(result.isSuccess)
        assertTrue(result.getOrNull() == true)
    }

    @Test
    fun `startSecureDiscovery should fail when not initialized`() = runTest {
        // When
        val result = secureFFIWrapper.startSecureDiscovery()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("FFI not initialized", result.exceptionOrNull()?.message)
    }

    @Test
    fun `connectSecureToDevice should succeed with valid device ID`() = runTest {
        // Given
        val validDeviceId = "test-device-123"
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.connectToDevice(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.connectSecureToDevice(validDeviceId)

        // Then
        assertTrue(result.isSuccess)
        assertTrue(result.getOrNull() == true)
        verify { NearClipFFI.connectToDevice(validDeviceId) }
    }

    @Test
    fun `connectSecureToDevice should fail with invalid device ID`() = runTest {
        // Given
        val invalidDeviceIds = listOf(
            "", // 空字符串
            "a".repeat(257), // 超过最大长度
            "device@invalid", // 包含无效字符
            "device with spaces", // 包含空格
            "device<script>" // 包含脚本字符
        )

        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        invalidDeviceIds.forEach { deviceId ->
            // When
            val result = secureFFIWrapper.connectSecureToDevice(deviceId)

            // Then
            assertTrue(result.isFailure, "Should fail for device ID: $deviceId")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `sendSecureClipboardData should succeed with valid data`() = runTest {
        // Given
        val validData = "Hello, World!"
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.sendClipboardData(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.sendSecureClipboardData(validData)

        // Then
        assertTrue(result.isSuccess)
        assertTrue(result.getOrNull() == true)
        verify { NearClipFFI.sendClipboardData(validData) }
    }

    @Test
    fun `sendSecureClipboardData should fail with oversized data`() = runTest {
        // Given
        val oversizedData = "a".repeat(1024 * 1024 + 1) // 超过1MB
        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.sendSecureClipboardData(oversizedData)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Invalid clipboard data", result.exceptionOrNull()?.message)
    }

    @Test
    fun `sendSecureClipboardData should fail with suspicious content`() = runTest {
        // Given
        val suspiciousData = listOf(
            "<script>alert('xss')</script>",
            "javascript:alert('xss')",
            "eval('malicious code')",
            "exec('rm -rf /')",
            "system('malicious command')",
            "runtime.exec('hack')",
            "getClass().forName('java.lang.Runtime')"
        )

        every { NearClipFFI.initialize(any()) } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        suspiciousData.forEach { data ->
            // When
            val result = secureFFIWrapper.sendSecureClipboardData(data)

            // Then
            assertTrue(result.isFailure, "Should fail for suspicious data: $data")
            assertTrue(result.exceptionOrNull() is SecurityException)
        }
    }

    @Test
    fun `getSecureLocalDeviceInfo should succeed when initialized`() = runTest {
        // Given
        val mockDeviceInfo = DeviceInfo(
            deviceId = "local-device",
            deviceName = "Local Device",
            deviceType = "ANDROID",
            publicKey = "public-key"
        )
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.getLocalDeviceInfo() } returns mockDeviceInfo
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        val result = secureFFIWrapper.getSecureLocalDeviceInfo()

        // Then
        assertTrue(result.isSuccess)
        assertNotNull(result.getOrNull())
        assertEquals("local-device", result.getOrNull()?.deviceId)
    }

    @Test
    fun `secureCleanup should clean up resources`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.cleanup() } returns Unit
        secureFFIWrapper.initializeSecure(mockContext)

        // When
        secureFFIWrapper.secureCleanup()

        // Then
        verify { NearClipFFI.cleanup() }
        assertEquals(SecurityState.Safe, secureFFIWrapper.securityState.first())
    }

    @Test
    fun `getSecurityStats should return correct statistics`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns true
        secureFFIWrapper.initializeSecure(mockContext)
        secureFFIWrapper.startSecureDiscovery()

        // When
        val stats = secureFFIWrapper.getSecurityStats()

        // Then
        assertTrue(stats["initialized"] as Boolean)
        assertTrue((stats["operation_counts"] as Map<*, *>).containsKey("discovery"))
        assertEquals(SecurityState.Safe, stats["security_state"])
    }

    @Test
    fun `should handle operation rate limiting`() = runTest {
        // Given
        every { NearClipFFI.initialize(any()) } returns true
        every { NearClipFFI.startDeviceDiscovery() } returns true
        secureFFIWrapper.initializeSecure(mockContext)

        // When - 模拟大量操作
        repeat(1001) {
            secureFFIWrapper.startSecureDiscovery()
        }

        // Then - 第1001次操作应该失败
        val result = secureFFIWrapper.startSecureDiscovery()
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Operation rate limit exceeded: discovery", result.exceptionOrNull()?.message)
    }

    @Test
    fun `should handle context validation exceptions`() = runTest {
        // Given
        every { mockContext.applicationInfo } throws RuntimeException("Context access denied")

        // When
        val result = secureFFIWrapper.initializeSecure(mockContext)

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
        assertEquals("Invalid context provided", result.exceptionOrNull()?.message)
    }
}