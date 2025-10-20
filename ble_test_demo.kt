import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.le.BluetoothLeScanner
import android.content.Context
import android.content.pm.PackageManager
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

// 简单的BLE功能演示
// 这个脚本演示了我们的BLE管理器的核心功能

class BleDemoApp {

    private val scope = CoroutineScope(Dispatchers.Main + SupervisorJob())

    fun runDemo() {
        println("=== NearClip BLE 功能演示 ===")

        scope.launch {
            try {
                // 1. 测试BLE服务工厂
                testBleServiceFactory()

                // 2. 测试设备创建
                testDeviceCreation()

                // 3. 测试消息创建
                testMessageCreation()

                println("\n✅ 所有演示测试完成!")

            } catch (e: Exception) {
                println("❌ 演示过程中发生错误: ${e.message}")
                e.printStackTrace()
            }
        }
    }

    private suspend fun testBleServiceFactory() {
        println("\n📱 测试BLE服务工厂...")

        // 创建测试设备
        val testDevice1 = BleServiceFactory.createTestDevice(
            deviceId = "demo-001",
            deviceName = "NearClip-Demo-1",
            rssi = -45,
            deviceType = BleDeviceType.NEARCLIP
        )

        val testDevice2 = BleServiceFactory.createTestDevice(
            deviceId = "demo-002",
            deviceName = "NearClip-Demo-2",
            rssi = -65,
            deviceType = BleDeviceType.LE
        )

        println("  ✓ 创建测试设备1: ${testDevice1.deviceName} (${testDevice1.deviceId})")
        println("  ✓ 创建测试设备2: ${testDevice2.deviceName} (${testDevice2.deviceId})")

        // 获取调试信息
        val debugInfo = BleServiceFactory.getDebugInfo()
        println("  ✓ 调试信息: $debugInfo")
    }

    private suspend fun testDeviceCreation() {
        println("\n🔍 测试设备发现...")

        // 模拟设备发现流程
        val mockDevices = listOf(
            BleServiceFactory.createTestDevice(
                deviceId = "mock-001",
                deviceName = "NearClip-Android",
                rssi = -50,
                deviceType = BleDeviceType.NEARCLIP
            ),
            BleServiceFactory.createTestDevice(
                deviceId = "mock-002",
                deviceName = "Regular-Device",
                rssi = -70,
                deviceType = BleDeviceType.LE
            ),
            BleServiceFactory.createTestDevice(
                deviceId = "mock-003",
                deviceName = "Dual-Mode-Device",
                rssi = -60,
                deviceType = BleDeviceType.DUAL
            )
        )

        println("  ✓ 模拟发现 ${mockDevices.size} 个设备:")
        mockDevices.forEach { device ->
            println("    - ${device.deviceName} (${device.deviceType.name}, RSSI: ${device.rssi}dBm)")
        }

        // 模拟NearClip设备过滤
        val nearClipDevices = mockDevices.filter {
            it.deviceType == BleDeviceType.NEARCLIP ||
            it.deviceName.contains("NearClip", true)
        }

        println("  ✓ 过滤出 ${nearClipDevices.size} 个NearClip设备:")
        nearClipDevices.forEach { device ->
            println("    - ${device.deviceName} (${device.deviceId})")
        }
    }

    private suspend fun testMessageCreation() {
        println("\n📨 测试消息创建...")

        // 创建不同类型的测试消息
        val pingMessage = BleServiceFactory.createTestMessage(
            messageId = "ping-001",
            type = MessageType.PING,
            payload = "",
            sequenceNumber = 0
        )

        val pongMessage = BleServiceFactory.createTestMessage(
            messageId = "pong-001",
            type = MessageType.PONG,
            payload = "ping-001",
            sequenceNumber = 0
        )

        val dataMessage = BleServiceFactory.createTestMessage(
            messageId = "data-001",
            type = MessageType.DATA,
            payload = "Hello from Android!",
            sequenceNumber = 1
        )

        val ackMessage = BleServiceFactory.createTestMessage(
            messageId = "ack-001",
            type = MessageType.ACK,
            payload = "data-001",
            sequenceNumber = 2
        )

        val messages = listOf(pingMessage, pongMessage, dataMessage, ackMessage)

        println("  ✓ 创建了 ${messages.size} 个测试消息:")
        messages.forEach { message ->
            val payloadText = if (message.payload.isEmpty()) "(空)" else message.payload
            println("    - [${message.type.name}] ${message.messageId}: \"$payloadText\"")
        }

        // 模拟消息序列化/反序列化
        println("  ✓ 测试消息序列化:")
        messages.forEach { message ->
            val serialized = serializeMessage(message)
            val deserialized = deserializeMessage(serialized)
            val isValid = message == deserialized
            println("    - ${message.messageId}: ${if (isValid) "✓" else "❌"}")
        }
    }

    // 简单的消息序列化 (实际实现会在BLEConnectionManager中)
    private fun serializeMessage(message: TestMessage): String {
        return "${message.messageId}|${message.type}|${message.payload}|${message.timestamp}|${message.sequenceNumber}"
    }

    // 简单的消息反序列化
    private fun deserializeMessage(data: String): TestMessage {
        val parts = data.split("|")
        return TestMessage(
            messageId = parts.getOrNull(0) ?: "",
            type = MessageType.valueOf(parts.getOrNull(1) ?: "DATA"),
            payload = parts.getOrNull(2) ?: "",
            timestamp = parts.getOrNull(3)?.toLongOrNull() ?: System.currentTimeMillis(),
            sequenceNumber = parts.getOrNull(4)?.toIntOrNull() ?: 0
        )
    }

    fun cleanup() {
        scope.cancel()
    }
}

// 主函数
fun main() {
    val demo = BleDemoApp()
    demo.runDemo()

    // 等待演示完成
    Thread.sleep(2000)

    demo.cleanup()
}

// 为了演示，我们需要包含必要的类型定义
// 这些类型定义来自我们的BLE实现

data class BleDevice(
    val deviceId: String,
    val deviceName: String,
    val deviceType: BleDeviceType,
    val rssi: Int,
    val timestamp: Long,
    val bluetoothDevice: Any = Object() // 简化演示，使用Object代替真实的BluetoothDevice
)

enum class BleDeviceType {
    UNKNOWN,
    LE,           // 低功耗设备
    DUAL,         // 双模设备
    NEARCLIP      // NearClip设备
}

data class TestMessage(
    val messageId: String,
    val type: MessageType,
    val payload: String,
    val timestamp: Long,
    val sequenceNumber: Int
)

enum class MessageType {
    PING,
    PONG,
    DATA,
    ACK
}

// 简化的BLE服务工厂 (仅用于演示)
object BleServiceFactory {

    suspend fun getDebugInfo(): Map<String, Any> {
        return mapOf(
            "managerInitialized" to false,
            "managerState" to "READY",
            "isScanning" to false,
            "isAdvertising" to false,
            "discoveredDevicesCount" to 0,
            "connectedDevicesCount" to 0
        )
    }

    fun createTestDevice(
        deviceId: String = "test-device-${System.currentTimeMillis()}",
        deviceName: String = "Test-NearClip",
        rssi: Int = -50,
        deviceType: BleDeviceType = BleDeviceType.NEARCLIP
    ): BleDevice {
        return BleDevice(
            deviceId = deviceId,
            deviceName = deviceName,
            deviceType = deviceType,
            rssi = rssi,
            timestamp = System.currentTimeMillis()
        )
    }

    fun createTestMessage(
        messageId: String = "test-${System.currentTimeMillis()}",
        type: MessageType = MessageType.PING,
        payload: String = "Test message",
        sequenceNumber: Int = 0
    ): TestMessage {
        return TestMessage(
            messageId = messageId,
            type = type,
            payload = payload,
            timestamp = System.currentTimeMillis(),
            sequenceNumber = sequenceNumber
        )
    }
}