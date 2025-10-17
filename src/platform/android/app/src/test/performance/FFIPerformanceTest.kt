package com.nearclip.performance

import android.content.Context
import com.nearclip.ffi.NearClipFFI
import com.nearclip.ffi.SecureFFIWrapper
import com.nearclip.test.util.MainDispatcherRule
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.system.measureTimeMillis
import kotlin.test.assertTrue

/**
 * FFI性能基准测试
 * 测试JNI调用的性能指标
 */
@ExperimentalCoroutinesApi
class FFIPerformanceTest {

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
        every { NearClipFFI.connectToDevice(any()) } returns true
        every { NearClipFFI.sendClipboardData(any()) } returns true
        every { NearClipFFI.getLocalDeviceInfo() } returns mockk()

        // 初始化
        secureFFIWrapper.initializeSecure(mockContext)
    }

    @Test
    fun `device discovery performance test`() = runTest {
        val iterations = 100
        val totalTime = measureTimeMillis {
            repeat(iterations) {
                secureFFIWrapper.startSecureDiscovery()
            }
        }

        val averageTime = totalTime / iterations.toDouble()
        println("Device discovery average time: ${averageTime}ms")

        // 性能要求：平均响应时间应该小于100ms
        assertTrue(averageTime < 100.0, "Device discovery should complete in < 100ms, actual: ${averageTime}ms")
    }

    @Test
    fun `device connection performance test`() = runTest {
        val deviceId = "test-device-123"
        val iterations = 100
        val totalTime = measureTimeMillis {
            repeat(iterations) {
                secureFFIWrapper.connectSecureToDevice(deviceId)
            }
        }

        val averageTime = totalTime / iterations.toDouble()
        println("Device connection average time: ${averageTime}ms")

        // 性能要求：平均响应时间应该小于50ms
        assertTrue(averageTime < 50.0, "Device connection should complete in < 50ms, actual: ${averageTime}ms")
    }

    @Test
    fun `clipboard data send performance test`() = runTest {
        val testData = "Hello, NearClip!"
        val iterations = 100
        val totalTime = measureTimeMillis {
            repeat(iterations) {
                secureFFIWrapper.sendSecureClipboardData(testData)
            }
        }

        val averageTime = totalTime / iterations.toDouble()
        println("Clipboard send average time: ${averageTime}ms")

        // 性能要求：平均响应时间应该小于20ms
        assertTrue(averageTime < 20.0, "Clipboard send should complete in < 20ms, actual: ${averageTime}ms")
    }

    @Test
    fun `large clipboard data performance test`() = runTest {
        val largeData = "a".repeat(1024 * 10) // 10KB数据
        val iterations = 50
        val totalTime = measureTimeMillis {
            repeat(iterations) {
                secureFFIWrapper.sendSecureClipboardData(largeData)
            }
        }

        val averageTime = totalTime / iterations.toDouble()
        println("Large clipboard send average time: ${averageTime}ms")

        // 性能要求：大数据传输应该小于100ms
        assertTrue(averageTime < 100.0, "Large clipboard send should complete in < 100ms, actual: ${averageTime}ms")
    }

    @Test
    fun `get device info performance test`() = runTest {
        val iterations = 100
        val totalTime = measureTimeMillis {
            repeat(iterations) {
                secureFFIWrapper.getSecureLocalDeviceInfo()
            }
        }

        val averageTime = totalTime / iterations.toDouble()
        println("Get device info average time: ${averageTime}ms")

        // 性能要求：平均响应时间应该小于10ms
        assertTrue(averageTime < 10.0, "Get device info should complete in < 10ms, actual: ${averageTime}ms")
    }

    @Test
    fun `concurrent operations performance test`() = runTest {
        val concurrentTasks = (1..10).map { taskId ->
            kotlinx.coroutines.async {
                val totalTime = measureTimeMillis {
                    repeat(10) {
                        secureFFIWrapper.connectSecureToDevice("device-$taskId-$it")
                        secureFFIWrapper.sendSecureClipboardData("data-$taskId-$it")
                    }
                }
                totalTime
            }
        }

        val times = concurrentTasks.map { it.await() }
        val averageTime = times.average()
        val maxTime = times.maxOrNull() ?: 0

        println("Concurrent operations average time: ${averageTime}ms")
        println("Concurrent operations max time: ${maxTime}ms")

        // 性能要求：并发操作平均时间应该小于200ms
        assertTrue(averageTime < 200.0, "Concurrent operations should complete in < 200ms, actual: ${averageTime}ms")
        assertTrue(maxTime < 500.0, "Max concurrent operation time should be < 500ms, actual: ${maxTime}ms")
    }

    @Test
    fun `memory usage performance test`() = runTest {
        val runtime = Runtime.getRuntime()

        // 强制垃圾回收
        System.gc()
        Thread.sleep(100)

        val initialMemory = runtime.totalMemory() - runtime.freeMemory()

        // 执行大量操作
        repeat(1000) {
            secureFFIWrapper.connectSecureToDevice("device-$it")
            secureFFIWrapper.sendSecureClipboardData("test data $it")
        }

        System.gc()
        Thread.sleep(100)

        val finalMemory = runtime.totalMemory() - runtime.freeMemory()
        val memoryIncrease = finalMemory - initialMemory

        println("Initial memory: ${initialMemory / 1024}KB")
        println("Final memory: ${finalMemory / 1024}KB")
        println("Memory increase: ${memoryIncrease / 1024}KB")

        // 内存要求：内存增长应该小于10MB
        assertTrue(memoryIncrease < 10 * 1024 * 1024, "Memory increase should be < 10MB, actual: ${memoryIncrease / 1024}KB")
    }

    @Test
    fun `security validation performance test`() = runTest {
        val validDeviceIds = (1..1000).map { "valid-device-$it" }
        val invalidDeviceIds = (1..1000).map { "invalid@device#$it" }

        val validTime = measureTimeMillis {
            validDeviceIds.forEach { deviceId ->
                secureFFIWrapper.connectSecureToDevice(deviceId)
            }
        }

        val invalidTime = measureTimeMillis {
            invalidDeviceIds.forEach { deviceId ->
                runCatching {
                    secureFFIWrapper.connectSecureToDevice(deviceId)
                }
            }
        }

        val averageValidTime = validTime / 1000.0
        val averageInvalidTime = invalidTime / 1000.0

        println("Valid ID validation average time: ${averageValidTime}ms")
        println("Invalid ID validation average time: ${averageInvalidTime}ms")

        // 安全验证性能要求：验证时间应该小于1ms
        assertTrue(averageValidTime < 1.0, "Valid ID validation should be < 1ms, actual: ${averageValidTime}ms")
        assertTrue(averageInvalidTime < 1.0, "Invalid ID validation should be < 1ms, actual: ${averageInvalidTime}ms")
    }

    @Test
    fun `throughput performance test`() = runTest {
        val duration = 5000L // 5秒测试
        val startTime = System.currentTimeMillis()
        var operationCount = 0

        while (System.currentTimeMillis() - startTime < duration) {
            secureFFIWrapper.connectSecureToDevice("device-$operationCount")
            secureFFIWrapper.sendSecureClipboardData("data-$operationCount")
            operationCount += 2
        }

        val actualDuration = System.currentTimeMillis() - startTime
        val throughput = (operationCount * 1000.0) / actualDuration // operations per second

        println("Operations per second: ${throughput}")
        println("Total operations: $operationCount")
        println("Actual duration: ${actualDuration}ms")

        // 吞吐量要求：每秒应该能处理至少100个操作
        assertTrue(throughput >= 100.0, "Throughput should be >= 100 ops/sec, actual: ${throughput}")
    }
}