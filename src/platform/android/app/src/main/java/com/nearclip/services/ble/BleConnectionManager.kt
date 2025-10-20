package com.nearclip.services.ble

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.*
import android.bluetooth.le.ScanResult
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import androidx.core.content.ContextCompat
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.*
import java.util.*
import java.util.concurrent.ConcurrentHashMap

/**
 * BLE连接管理器
 * 负责管理BLE设备的连接、断开和通信
 */
class BleConnectionManager(private val context: Context) {

    companion object {
        private const val TAG = "BleConnectionManager"
        private val SERVICE_UUID = UUID.fromString("0000FE2C-0000-1000-8000-00805F9B34FB")
        private val CHARACTERISTIC_UUID = UUID.fromString("0000FE2D-0000-1000-8000-00805F9B34FB")
        private val DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805F9B34FB")
        private const val CONNECTION_TIMEOUT_MS = 10000L // 10秒连接超时
    }

    private val bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
    private val _connectionStates = MutableStateFlow<Map<String, ConnectionState>>(emptyMap())
    private val _receivedMessages = Channel<TestMessage>(capacity = Channel.UNLIMITED)
    private val activeConnections = ConcurrentHashMap<String, BluetoothGatt>()
    private var connectionJobs = ConcurrentHashMap<String, Job>()

    val connectionStates: StateFlow<Map<String, ConnectionState>> = _connectionStates.asStateFlow()
    val receivedMessages: Flow<TestMessage> = _receivedMessages.receiveAsFlow()

    /**
     * 检查BLE权限
     */
    fun hasRequiredPermissions(): Boolean {
        return ContextCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_CONNECT) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查BLE是否可用
     */
    fun isBluetoothAvailable(): Boolean {
        return bluetoothAdapter != null && bluetoothAdapter.isEnabled
    }

    /**
     * 连接到设备
     */
    @SuppressLint("MissingPermission")
    fun connectToDevice(device: BleDevice): Result<Unit> {
        Log.d(TAG, "连接到设备: ${device.deviceName} (${device.deviceId})")

        if (!hasRequiredPermissions()) {
            Log.e(TAG, "缺少连接权限")
            return Result.failure(SecurityException("缺少BLE连接权限"))
        }

        if (!isBluetoothAvailable()) {
            Log.e(TAG, "蓝牙不可用")
            return Result.failure(IllegalStateException("蓝牙不可用"))
        }

        if (activeConnections.containsKey(device.deviceId)) {
            Log.w(TAG, "设备已连接: ${device.deviceId}")
            return Result.success(Unit)
        }

        try {
            // 创建连接超时任务
            val timeoutJob = CoroutineScope(Dispatchers.IO).launch {
                delay(CONNECTION_TIMEOUT_MS)
                updateConnectionState(device.deviceId, ConnectionState.FAILED)
                connectionJobs.remove(device.deviceId)
            }

            connectionJobs[device.deviceId] = timeoutJob

            // 更新连接状态
            updateConnectionState(device.deviceId, ConnectionState.CONNECTING)

            // 建立GATT连接
            val gatt = device.bluetoothDevice.connectGatt(
                context,
                false, // 不自动重连
                gattCallback,
                BluetoothDevice.TRANSPORT_LE
            )

            gatt?.let {
                activeConnections[device.deviceId] = it
                Log.d(TAG, "开始连接到设备: ${device.deviceId}")
                return Result.success(Unit)
            } ?: run {
                Log.e(TAG, "无法创建GATT连接")
                updateConnectionState(device.deviceId, ConnectionState.FAILED)
                connectionJobs.remove(device.deviceId)
                return Result.failure(IllegalStateException("无法创建GATT连接"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "连接设备失败", e)
            updateConnectionState(device.deviceId, ConnectionState.FAILED)
            connectionJobs.remove(device.deviceId)
            return Result.failure(e)
        }
    }

    /**
     * 断开设备连接
     */
    @SuppressLint("MissingPermission")
    fun disconnectFromDevice(deviceId: String) {
        Log.d(TAG, "断开设备连接: $deviceId")

        connectionJobs[deviceId]?.cancel()
        connectionJobs.remove(deviceId)

        activeConnections[deviceId]?.let { gatt ->
            gatt.disconnect()
            gatt.close()
        }
        activeConnections.remove(deviceId)

        updateConnectionState(deviceId, ConnectionState.DISCONNECTED)
    }

    /**
     * 发送消息
     */
    @SuppressLint("MissingPermission")
    fun sendMessage(deviceId: String, message: TestMessage): Result<Unit> {
        Log.d(TAG, "发送消息到设备 $deviceId: ${message.messageId}")

        val gatt = activeConnections[deviceId]
        if (gatt == null) {
            Log.e(TAG, "设备未连接: $deviceId")
            return Result.failure(IllegalStateException("设备未连接"))
        }

        val connectionState = _connectionStates.value[deviceId]
        if (connectionState != ConnectionState.CONNECTED) {
            Log.e(TAG, "设备连接状态不正确: $deviceId, 状态: $connectionState")
            return Result.failure(IllegalStateException("设备连接状态不正确"))
        }

        try {
            val characteristic = gatt.getService(SERVICE_UUID)?.getCharacteristic(CHARACTERISTIC_UUID)
            if (characteristic == null) {
                Log.e(TAG, "找不到特征值")
                return Result.failure(IllegalStateException("找不到特征值"))
            }

            // 序列化消息
            val messageData = serializeMessage(message)
            characteristic.value = messageData
            characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT

            val success = gatt.writeCharacteristic(characteristic)
            if (success) {
                Log.d(TAG, "消息发送成功: ${message.messageId}")
                return Result.success(Unit)
            } else {
                Log.e(TAG, "消息发送失败")
                return Result.failure(IllegalStateException("消息发送失败"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "发送消息异常", e)
            return Result.failure(e)
        }
    }

    /**
     * 获取已连接的设备列表
     */
    fun getConnectedDevices(): List<String> {
        return _connectionStates.value
            .filter { it.value == ConnectionState.CONNECTED }
            .keys
            .toList()
    }

    /**
     * GATT回调
     */
    private val gattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            val deviceId = gatt.device.address
            Log.d(TAG, "连接状态变化: $deviceId, status: $status, newState: $newState")

            // 取消超时任务
            connectionJobs[deviceId]?.cancel()
            connectionJobs.remove(deviceId)

            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.d(TAG, "设备已连接: $deviceId")
                    updateConnectionState(deviceId, ConnectionState.CONNECTED)
                    // 发现服务
                    gatt.discoverServices()
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.d(TAG, "设备已断开: $deviceId")
                    updateConnectionState(deviceId, ConnectionState.DISCONNECTED)
                    activeConnections.remove(deviceId)
                    gatt.close()
                }
                else -> {
                    Log.w(TAG, "未知的连接状态: $newState")
                }
            }
        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            val deviceId = gatt.device.address
            Log.d(TAG, "服务发现完成: $deviceId, status: $status")

            if (status == BluetoothGatt.GATT_SUCCESS) {
                val service = gatt.getService(SERVICE_UUID)
                if (service != null) {
                    Log.d(TAG, "找到NearClip服务")
                    // 订阅特征值通知
                    subscribeToNotifications(gatt, service)
                } else {
                    Log.w(TAG, "找不到NearClip服务")
                }
            } else {
                Log.e(TAG, "服务发现失败: $status")
                updateConnectionState(deviceId, ConnectionState.FAILED)
            }
        }

        override fun onCharacteristicRead(gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int) {
            Log.d(TAG, "特征值读取: ${characteristic.uuid}, status: $status")
        }

        override fun onCharacteristicWrite(gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic, status: Int) {
            Log.d(TAG, "特征值写入: ${characteristic.uuid}, status: $status")
        }

        override fun onCharacteristicChanged(gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic) {
            Log.d(TAG, "收到特征值通知: ${characteristic.uuid}")

            val data = characteristic.value
            if (data != null) {
                try {
                    val message = deserializeMessage(data)
                    Log.d(TAG, "收到消息: ${message.messageId}")

                    CoroutineScope(Dispatchers.IO).launch {
                        _receivedMessages.send(message)
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "解析消息失败", e)
                }
            }
        }

        override fun onDescriptorWrite(gatt: BluetoothGatt, descriptor: BluetoothGattDescriptor, status: Int) {
            Log.d(TAG, "描述符写入: ${descriptor.uuid}, status: $status")
        }
    }

    /**
     * 订阅特征值通知
     */
    @SuppressLint("MissingPermission")
    private fun subscribeToNotifications(gatt: BluetoothGatt, service: BluetoothGattService) {
        val characteristic = service.getCharacteristic(CHARACTERISTIC_UUID)
        if (characteristic != null) {
            // 启用通知
            gatt.setCharacteristicNotification(characteristic, true)

            // 写入描述符以启用通知
            val descriptor = characteristic.getDescriptor(DESCRIPTOR_UUID)
            descriptor?.let {
                it.value = BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
                gatt.writeDescriptor(it)
            }
        }
    }

    /**
     * 更新连接状态
     */
    private fun updateConnectionState(deviceId: String, state: ConnectionState) {
        val currentStates = _connectionStates.value.toMutableMap()
        currentStates[deviceId] = state
        _connectionStates.value = currentStates
        Log.d(TAG, "更新连接状态: $deviceId -> $state")
    }

    /**
     * 序列化消息
     */
    private fun serializeMessage(message: TestMessage): ByteArray {
        val data = "${message.messageId}|${message.type}|${message.payload}|${message.timestamp}|${message.sequenceNumber}"
        return data.toByteArray(Charsets.UTF_8)
    }

    /**
     * 反序列化消息
     */
    private fun deserializeMessage(data: ByteArray): TestMessage {
        val messageString = String(data, Charsets.UTF_8)
        val parts = messageString.split("|")

        return TestMessage(
            messageId = parts.getOrNull(0) ?: "",
            type = MessageType.valueOf(parts.getOrNull(1) ?: "DATA"),
            payload = parts.getOrNull(2) ?: "",
            timestamp = parts.getOrNull(3)?.toLongOrNull() ?: System.currentTimeMillis(),
            sequenceNumber = parts.getOrNull(4)?.toIntOrNull() ?: 0
        )
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        Log.d(TAG, "清理连接管理器")

        // 取消所有连接任务
        connectionJobs.values.forEach { it.cancel() }
        connectionJobs.clear()

        // 断开所有连接
        activeConnections.values.forEach { gatt ->
            gatt.disconnect()
            gatt.close()
        }
        activeConnections.clear()

        // 关闭消息通道
        _receivedMessages.close()
    }
}

/**
 * 连接状态枚举
 */
enum class ConnectionState {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    FAILED
}

/**
 * 测试消息数据类
 */
data class TestMessage(
    val messageId: String,
    val type: MessageType,
    val payload: String,
    val timestamp: Long,
    val sequenceNumber: Int
)

/**
 * 消息类型枚举
 */
enum class MessageType {
    PING,
    PONG,
    DATA,
    ACK
}