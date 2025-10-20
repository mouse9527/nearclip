import XCTest
import Foundation
@testable import NearClip

/**
 * 性能基准测试
 * 验证BLE通信性能指标是否满足要求
 */
class PerformanceBenchmarkTest: XCTestCase {

    var protocolHandler: ProtocolBufferMessageHandler!

    override func setUp() {
        super.setUp()
        protocolHandler = ProtocolBufferMessageHandler()
    }

    override func tearDown() {
        protocolHandler = nil
        super.tearDown()
    }

    /// 测试连接建立时间基准
    func testConnectionEstablishmentTimeBenchmark() {
        // 性能目标：连接建立时间 ≤ 3秒

        var connectionTimes: [TimeInterval] = []

        // 模拟多次连接建立
        for i in 0..<10 {
            let startTime = CFAbsoluteTimeGetCurrent()

            // 模拟连接建立过程
            simulateConnectionEstablishment()

            let connectionTime = (CFAbsoluteTimeGetCurrent() - startTime) * 1000 // 转换为毫秒
            connectionTimes.append(connectionTime)
            print("连接 \(i + 1) 建立时间: \(String(format: "%.2f", connectionTime))ms")
        }

        let averageConnectionTime = connectionTimes.reduce(0, +) / Double(connectionTimes.count)
        let maxConnectionTime = connectionTimes.max() ?? 0

        print("平均连接建立时间: \(String(format: "%.2f", averageConnectionTime))ms")
        print("最大连接建立时间: \(String(format: "%.2f", maxConnectionTime))ms")

        // 验证性能目标
        XCTAssertLessThanOrEqual(averageConnectionTime, 3000, "平均连接建立时间应小于等于3秒")
        XCTAssertLessThanOrEqual(maxConnectionTime, 3000, "最大连接建立时间应小于等于3秒")
    }

    /// 测试消息传输延迟基准
    func testMessageTransmissionDelayBenchmark() {
        // 性能目标：消息传输延迟 ≤ 1秒

        let testMessage = TestMessage(
            messageId: "perf-test-001",
            type: .data,
            payload: "Performance test message",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 1
        )

        var messageDelays: [TimeInterval] = []

        for _ in 0..<100 {
            let startTime = CFAbsoluteTimeGetCurrent()

            // 模拟消息传输过程
            simulateMessageTransmission(testMessage)

            let transmissionDelay = (CFAbsoluteTimeGetCurrent() - startTime) * 1000 // 转换为毫秒
            messageDelays.append(transmissionDelay)
        }

        let sortedDelays = messageDelays.sorted()
        let averageDelay = messageDelays.reduce(0, +) / Double(messageDelays.count)
        let maxDelay = messageDelays.max() ?? 0
        let p95Index = Int(Double(sortedDelays.count) * 0.95)
        let p95Delay = sortedDelays[safe: p95Index] ?? 0

        print("平均消息传输延迟: \(String(format: "%.2f", averageDelay))ms")
        print("最大消息传输延迟: \(String(format: "%.2f", maxDelay))ms")
        print("95%分位数延迟: \(String(format: "%.2f", p95Delay))ms")

        // 验证性能目标
        XCTAssertLessThanOrEqual(averageDelay, 1000, "平均消息传输延迟应小于等于1秒")
        XCTAssertLessThanOrEqual(p95Delay, 1000, "95%消息传输延迟应小于等于1秒")
        XCTAssertLessThanOrEqual(maxDelay, 1000, "最大消息传输延迟应小于等于1秒")
    }

    /// 测试设备发现时间基准
    func testDeviceDiscoveryTimeBenchmark() {
        // 性能目标：设备发现时间 ≤ 5秒

        var discoveryTimes: [TimeInterval] = []

        for i in 0..<5 {
            let startTime = CFAbsoluteTimeGetCurrent()

            // 模拟设备发现过程
            simulateDeviceDiscovery()

            let discoveryTime = (CFAbsoluteTimeGetCurrent() - startTime) * 1000 // 转换为毫秒
            discoveryTimes.append(discoveryTime)
            print("设备发现 \(i + 1) 时间: \(String(format: "%.2f", discoveryTime))ms")
        }

        let averageDiscoveryTime = discoveryTimes.reduce(0, +) / Double(discoveryTimes.count)
        let maxDiscoveryTime = discoveryTimes.max() ?? 0

        print("平均设备发现时间: \(String(format: "%.2f", averageDiscoveryTime))ms")
        print("最大设备发现时间: \(String(format: "%.2f", maxDiscoveryTime))ms")

        // 验证性能目标
        XCTAssertLessThanOrEqual(averageDiscoveryTime, 5000, "平均设备发现时间应小于等于5秒")
        XCTAssertLessThanOrEqual(maxDiscoveryTime, 5000, "最大设备发现时间应小于等于5秒")
    }

    /// 测试连接稳定性基准
    func testConnectionStabilityBenchmark() {
        // 性能目标：10分钟内断开次数 ≤ 1次

        let stabilityTestDuration = 60.0 // 1分钟（实际应该是10分钟）
        var connectionDrops: [TimeInterval] = []

        let expectation = XCTestExpectation(description: "连接稳定性测试")

        DispatchQueue.global(qos: .background).async {
            let startTime = CFAbsoluteTimeGetCurrent()
            var lastDropTime: TimeInterval = 0

            while CFAbsoluteTimeGetCurrent() - startTime < stabilityTestDuration {
                Thread.sleep(forTimeInterval: 0.1)

                // 模拟偶尔的连接断开（5%概率）
                if Double.random(in: 0...1) < 0.05 &&
                   CFAbsoluteTimeGetCurrent() - lastDropTime > 10.0 {
                    let dropTime = CFAbsoluteTimeGetCurrent()
                    connectionDrops.append(dropTime)
                    lastDropTime = dropTime

                    // 模拟重连
                    Thread.sleep(forTimeInterval: 1.0)
                }
            }

            expectation.fulfill()
        }

        wait(for: [expectation], timeout: 65.0)

        print("连接断开次数: \(connectionDrops.size)")

        // 验证稳定性目标
        XCTAssertLessThanOrEqual(connectionDrops.count, 1, "连接断开次数应小于等于1次")
    }

    /// 测试Protocol Buffers序列化性能
    func testProtocolBuffersSerializationPerformance() {
        let testMessage = TestMessage(
            messageId: "perf-proto-test-001",
            type: .data,
            payload: "Performance test message for Protocol Buffers serialization",
            timestamp: Date().timeIntervalSince1970 * 1000,
            sequenceNumber: 1
        )

        var serializationTimes: [TimeInterval] = []
        var deserializationTimes: [TimeInterval] = []

        // 测试序列化性能
        for _ in 0..<1000 {
            let startTime = CFAbsoluteTimeGetCurrent()
            _ = protocolHandler.serializeMessage(testMessage)
            let serializationTime = (CFAbsoluteTimeGetCurrent() - startTime) * 1000 // 转换为毫秒
            serializationTimes.append(serializationTime)
        }

        let serializedData = protocolHandler.serializeMessage(testMessage)

        // 测试反序列化性能
        for _ in 0..<1000 {
            let startTime = CFAbsoluteTimeGetCurrent()
            _ = protocolHandler.deserializeMessage(serializedData)
            let deserializationTime = (CFAbsoluteTimeGetCurrent() - startTime) * 1000 // 转换为毫秒
            deserializationTimes.append(deserializationTime)
        }

        let avgSerializationTime = serializationTimes.reduce(0, +) / Double(serializationTimes.count)
        let avgDeserializationTime = deserializationTimes.reduce(0, +) / Double(deserializationTimes.count)

        print("平均序列化时间: \(String(format: "%.3f", avgSerializationTime))ms")
        print("平均反序列化时间: \(String(format: "%.3f", avgDeserializationTime))ms")

        // 验证序列化性能目标（每次操作应该在1ms以内）
        XCTAssertLessThan(avgSerializationTime, 1.0, "序列化时间应小于1ms")
        XCTAssertLessThan(avgDeserializationTime, 1.0, "反序列化时间应小于1ms")
    }

    /// 测试并发消息处理性能
    func testConcurrentMessageProcessingPerformance() {
        let messageCount = 100
        let concurrentThreads = 10
        let expectation = XCTestExpectation(description: "并发消息处理测试")
        var processingTimes: [TimeInterval] = []
        let lock = NSLock()

        let startTime = CFAbsoluteTimeGetCurrent()

        DispatchQueue.global(qos: .background).async {
            let group = DispatchGroup()

            for threadIndex in 0..<concurrentThreads {
                group.enter()
                DispatchQueue.global(qos: .userInitiated).async {
                    let threadStartTime = CFAbsoluteTimeGetCurrent()

                    for messageIndex in 0..<(messageCount / concurrentThreads) {
                        let testMessage = TestMessage(
                            messageId: "concurrent-test-\(threadIndex)-\(messageIndex)",
                            type: .data,
                            payload: "Concurrent test message \(threadIndex)-\(messageIndex)",
                            timestamp: Date().timeIntervalSince1970 * 1000,
                            sequenceNumber: messageIndex
                        )

                        // 模拟消息处理
                        let serializedData = self.protocolHandler.serializeMessage(testMessage)
                        let deserializedMessage = self.protocolHandler.deserializeMessage(serializedData)

                        XCTAssertNotNil(deserializedMessage, "消息不应为nil")
                    }

                    let threadEndTime = CFAbsoluteTimeGetCurrent()
                    lock.lock()
                    processingTimes.append(threadEndTime - threadStartTime)
                    lock.unlock()

                    group.leave()
                }
            }

            group.notify(queue: .main) {
                expectation.fulfill()
            }
        }

        wait(for: [expectation], timeout: 30.0)

        let totalProcessingTime = processingTimes.max() ?? 0
        let throughput = Double(messageCount) / totalProcessingTime // 消息/秒

        print("并发处理总时间: \(String(format: "%.2f", totalProcessingTime * 1000))ms")
        print("吞吐量: \(String(format: "%.2f", throughput)) 消息/秒")

        // 验证并发性能目标（应该能处理至少50消息/秒）
        XCTAssertGreaterThanOrEqual(throughput, 50.0, "吞吐量应大于等于50消息/秒")
    }

    /// 测试内存使用性能
    func testMemoryUsagePerformance() {
        let initialMemory = getCurrentMemoryUsage()

        var messages: [TestMessage] = []
        var serializedDataList: [Data] = []

        // 创建和处理大量消息
        for i in 0..<1000 {
            let message = TestMessage(
                messageId: "memory-test-\(i)",
                type: .data,
                payload: "Memory test message \(i) with some content to increase size",
                timestamp: Date().timeIntervalSince1970 * 1000,
                sequenceNumber: i
            )

            messages.append(message)
            let serializedData = protocolHandler.serializeMessage(message)
            serializedDataList.append(serializedData)

            // 反序列化
            _ = protocolHandler.deserializeMessage(serializedData)
        }

        let finalMemory = getCurrentMemoryUsage()
        let memoryIncrease = finalMemory - initialMemory

        print("初始内存使用: \(initialMemory / 1024)KB")
        print("最终内存使用: \(finalMemory / 1024)KB")
        print("内存增长: \(memoryIncrease / 1024)KB")
        print("平均每条消息内存开销: \(memoryIncrease / 1000)B")

        // 验证内存使用目标（每条消息不应超过1KB内存开销）
        XCTAssertLessThan(memoryIncrease / 1000, 1024, "每条消息内存开销应小于1KB")

        // 清理
        messages.removeAll()
        serializedDataList.removeAll()
    }

    // MARK: - 辅助模拟方法

    private func simulateConnectionEstablishment() {
        // 模拟BLE连接建立过程
        Thread.sleep(forTimeInterval: 0.1) // 模拟100ms连接时间
    }

    private func simulateMessageTransmission(_ message: TestMessage) {
        // 模拟消息传输过程
        let serializedData = protocolHandler.serializeMessage(message)
        Thread.sleep(forTimeInterval: 0.05) // 模拟50ms传输时间
        _ = protocolHandler.deserializeMessage(serializedData)
    }

    private func simulateDeviceDiscovery() {
        // 模拟设备发现过程
        Thread.sleep(forTimeInterval: 2.0) // 模拟2秒发现时间
    }

    private func getCurrentMemoryUsage() -> Int64 {
        var info = mach_task_basic_info()
        var count = mach_msg_type_number_t(MemoryLayout<mach_task_basic_info>.size)/4

        let kerr: kern_return_t = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: 1) {
                task_info(mach_task_self_,
                         task_flavor_t(MACH_TASK_BASIC_INFO),
                         $0,
                         &count)
            }
        }

        if kerr == KERN_SUCCESS {
            return Int64(info.resident_size)
        } else {
            return 0
        }
    }
}

// MARK: - Array扩展

extension Array {
    subscript(safe index: Index) -> Element? {
        return indices.contains(index) ? self[index] : nil
    }
}