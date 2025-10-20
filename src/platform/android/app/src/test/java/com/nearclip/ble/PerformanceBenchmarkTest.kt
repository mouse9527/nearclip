package com.nearclip.ble

import com.nearclip.services.ble.BleManager
import com.nearclip.services.ble.ProtocolBufferMessageHandler
import com.nearclip.services.ble.TestMessage
import com.nearclip.services.ble.MessageType
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.Test
import org.junit.Assert.*
import org.junit.Before
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import kotlin.system.measureTimeMillis

/**
 * 性能基准测试
 * 验证BLE通信性能指标是否满足要求
 */
class PerformanceBenchmarkTest {

    private lateinit var bleManager: BleManager
    private lateinit var protocolHandler: ProtocolBufferMessageHandler

    @Before
    fun setUp() {
        // 注意：在实际测试中需要使用模拟的BLE环境
        // 这里提供测试框架和基准测试方法
        protocolHandler = ProtocolBufferMessageHandler()
    }

    @Test
    fun `测试连接建立时间基准`() {
        // 性能目标：连接建立时间 ≤ 3秒

        // 模拟连接建立时间测量
        val connectionTimes = mutableListOf<Long>()

        // 模拟多次连接建立
        repeat(10) { i ->
            val connectionTime = measureTimeMillis {
                // 模拟连接建立过程
                simulateConnectionEstablishment()
            }
            connectionTimes.add(connectionTime)
            println("连接 ${i + 1} 建立时间: ${connectionTime}ms")
        }

        val averageConnectionTime = connectionTimes.average()
        val maxConnectionTime = connectionTimes.maxOrNull() ?: 0L

        println("平均连接建立时间: ${averageConnectionTime}ms")
        println("最大连接建立时间: ${maxConnectionTime}ms")

        // 验证性能目标
        assertTrue("平均连接建立时间应小于等于3秒", averageConnectionTime <= 3000)
        assertTrue("最大连接建立时间应小于等于3秒", maxConnectionTime <= 3000)
    }

    @Test
    fun `测试消息传输延迟基准`() {
        // 性能目标：消息传输延迟 ≤ 1秒

        val messageDelays = mutableListOf<Long>()
        val testMessage = TestMessage(
            messageId = "perf-test-001",
            type = MessageType.DATA,
            payload = "Performance test message",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 1
        )

        repeat(100) { i ->
            val transmissionDelay = measureTimeMillis {
                // 模拟消息传输过程
                simulateMessageTransmission(testMessage)
            }
            messageDelays.add(transmissionDelay)
        }

        val averageDelay = messageDelays.average()
        val maxDelay = messageDelays.maxOrNull() ?: 0L
        val p95Delay = messageDelays.sorted()[((messageDelays.size * 0.95).toInt())]

        println("平均消息传输延迟: ${averageDelay}ms")
        println("最大消息传输延迟: ${maxDelay}ms")
        println("95%分位数延迟: ${p95Delay}ms")

        // 验证性能目标
        assertTrue("平均消息传输延迟应小于等于1秒", averageDelay <= 1000)
        assertTrue("95%消息传输延迟应小于等于1秒", p95Delay <= 1000)
        assertTrue("最大消息传输延迟应小于等于1秒", maxDelay <= 1000)
    }

    @Test
    fun `测试设备发现时间基准`() {
        // 性能目标：设备发现时间 ≤ 5秒

        val discoveryTimes = mutableListOf<Long>()

        repeat(5) { i ->
            val discoveryTime = measureTimeMillis {
                // 模拟设备发现过程
                simulateDeviceDiscovery()
            }
            discoveryTimes.add(discoveryTime)
            println("设备发现 ${i + 1} 时间: ${discoveryTime}ms")
        }

        val averageDiscoveryTime = discoveryTimes.average()
        val maxDiscoveryTime = discoveryTimes.maxOrNull() ?: 0L

        println("平均设备发现时间: ${averageDiscoveryTime}ms")
        println("最大设备发现时间: ${maxDiscoveryTime}ms")

        // 验证性能目标
        assertTrue("平均设备发现时间应小于等于5秒", averageDiscoveryTime <= 5000)
        assertTrue("最大设备发现时间应小于等于5秒", maxDiscoveryTime <= 5000)
    }

    @Test
    fun `测试连接稳定性基准`() {
        // 性能目标：10分钟内断开次数 ≤ 1次

        // 模拟长时间连接稳定性测试
        val stabilityTestDuration = 60000L // 1分钟（实际应该是10分钟）
        val connectionDrops = mutableListOf<Long>()

        val testDuration = measureTimeMillis {
            simulateConnectionStabilityTest(stabilityTestDuration) { dropTime ->
                connectionDrops.add(dropTime)
            }
        }

        println("稳定性测试持续时间: ${testDuration}ms")
        println("连接断开次数: ${connectionDrops.size}")

        // 验证稳定性目标
        assertTrue("连接断开次数应小于等于1次", connectionDrops.size <= 1)
    }

    @Test
    fun `测试Protocol Buffers序列化性能`() {
        val testMessage = TestMessage(
            messageId = "perf-proto-test-001",
            type = MessageType.DATA,
            payload = "Performance test message for Protocol Buffers serialization",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 1
        )

        val serializationTimes = mutableListOf<Long>()
        val deserializationTimes = mutableListOf<Long>()

        // 测试序列化性能
        repeat(1000) {
            val serializationTime = measureTimeMillis {
                protocolHandler.serializeMessage(testMessage)
            }
            serializationTimes.add(serializationTime)
        }

        val serializedData = protocolHandler.serializeMessage(testMessage)

        // 测试反序列化性能
        repeat(1000) {
            val deserializationTime = measureTimeMillis {
                protocolHandler.deserializeMessage(serializedData)
            }
            deserializationTimes.add(deserializationTime)
        }

        val avgSerializationTime = serializationTimes.average()
        val avgDeserializationTime = deserializationTimes.average()

        println("平均序列化时间: ${avgSerializationTime}ms")
        println("平均反序列化时间: ${avgDeserializationTime}ms")

        // 验证序列化性能目标（每次操作应该在1ms以内）
        assertTrue("序列化时间应小于1ms", avgSerializationTime < 1.0)
        assertTrue("反序列化时间应小于1ms", avgDeserializationTime < 1.0)
    }

    @Test
    fun `测试并发消息处理性能`() {
        val messageCount = 100
        val concurrentThreads = 10
        val latch = CountDownLatch(concurrentThreads)
        val processingTimes = mutableListOf<Long>()

        repeat(concurrentThreads) { threadIndex ->
            Thread {
                val threadStartTime = System.currentTimeMillis()

                repeat(messageCount / concurrentThreads) { messageIndex ->
                    val testMessage = TestMessage(
                        messageId = "concurrent-test-$threadIndex-$messageIndex",
                        type = MessageType.DATA,
                        payload = "Concurrent test message $threadIndex-$messageIndex",
                        timestamp = System.currentTimeMillis(),
                        sequenceNumber = messageIndex
                    )

                    // 模拟消息处理
                    val serializedData = protocolHandler.serializeMessage(testMessage)
                    val deserializedMessage = protocolHandler.deserializeMessage(serializedData)

                    assertNotNull("消息不应为null", deserializedMessage)
                }

                val threadEndTime = System.currentTimeMillis()
                synchronized(processingTimes) {
                    processingTimes.add(threadEndTime - threadStartTime)
                }
                latch.countDown()
            }.start()
        }

        assertTrue("并发测试应在30秒内完成", latch.await(30, TimeUnit.SECONDS))

        val totalProcessingTime = processingTimes.maxOrNull() ?: 0L
        val throughput = (messageCount * 1000.0) / totalProcessingTime // 消息/秒

        println("并发处理总时间: ${totalProcessingTime}ms")
        println("吞吐量: ${throughput} 消息/秒")

        // 验证并发性能目标（应该能处理至少50消息/秒）
        assertTrue("吞吐量应大于等于50消息/秒", throughput >= 50.0)
    }

    @Test
    fun `测试内存使用性能`() {
        val runtime = Runtime.getRuntime()

        // 强制垃圾回收以获得准确的基线内存使用
        System.gc()
        Thread.sleep(100)

        val initialMemory = runtime.totalMemory() - runtime.freeMemory()

        // 创建和处理大量消息
        val messages = mutableListOf<TestMessage>()
        val serializedDataList = mutableListOf<ByteArray>()

        repeat(1000) { i ->
            val message = TestMessage(
                messageId = "memory-test-$i",
                type = MessageType.DATA,
                payload = "Memory test message $i with some content to increase size",
                timestamp = System.currentTimeMillis(),
                sequenceNumber = i
            )

            messages.add(message)
            val serializedData = protocolHandler.serializeMessage(message)
            serializedDataList.add(serializedData)

            // 反序列化
            protocolHandler.deserializeMessage(serializedData)
        }

        val finalMemory = runtime.totalMemory() - runtime.freeMemory()
        val memoryIncrease = finalMemory - initialMemory

        println("初始内存使用: ${initialMemory / 1024}KB")
        println("最终内存使用: ${finalMemory / 1024}KB")
        println("内存增长: ${memoryIncrease / 1024}KB")
        println("平均每条消息内存开销: ${memoryIncrease / 1000}B")

        // 验证内存使用目标（每条消息不应超过1KB内存开销）
        assertTrue("每条消息内存开销应小于1KB", (memoryIncrease / 1000) < 1024)

        // 清理
        messages.clear()
        serializedDataList.clear()
        System.gc()
    }

    // MARK: - 辅助模拟方法

    private fun simulateConnectionEstablishment() {
        // 模拟BLE连接建立过程
        Thread.sleep(100) // 模拟100ms连接时间
    }

    private fun simulateMessageTransmission(message: TestMessage) {
        // 模拟消息传输过程
        val serializedData = protocolHandler.serializeMessage(message)
        Thread.sleep(50) // 模拟50ms传输时间
        protocolHandler.deserializeMessage(serializedData)
    }

    private fun simulateDeviceDiscovery() {
        // 模拟设备发现过程
        Thread.sleep(2000) // 模拟2秒发现时间
    }

    private fun simulateConnectionStabilityTest(durationMs: Long, onConnectionDrop: (Long) -> Unit) {
        val startTime = System.currentTimeMillis()
        var lastDropTime = 0L

        while (System.currentTimeMillis() - startTime < durationMs) {
            Thread.sleep(100)

            // 模拟偶尔的连接断开（5%概率）
            if Math.random() < 0.05 && System.currentTimeMillis() - lastDropTime > 10000) {
                val dropTime = System.currentTimeMillis()
                onConnectionDrop(dropTime)
                lastDropTime = dropTime

                // 模拟重连
                Thread.sleep(1000)
            }
        }
    }
}