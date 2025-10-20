package com.nearclip.services.ble

import android.content.Context
import android.util.Log
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

/**
 * BLE统一管理器
 * 提供BLE功能的统一接口和状态管理
 */
class BleManager(private val context: Context) {

    companion object {
        private const val TAG = "BleManager"
    }

    // 组件
    private val permissionManager = BlePermissionManager(context)
    private val scanner = BleScanner(context)
    private val advertiser = BleAdvertiser(context)
    private val connectionManager = BleConnectionManager(context)
    private val discoveryListener = DeviceDiscoveryListener(scanner)

    // 状态
    private val _managerState = MutableStateFlow(BleManagerState.INITIALIZING)
    private val _isScanning = MutableStateFlow(false)
    private val _isAdvertising = MutableStateFlow(false)

    // 公开的状态流
    val managerState: StateFlow<BleManagerState> = _managerState.asStateFlow()
    val isScanning: StateFlow<Boolean> = _isScanning.asStateFlow()
    val isAdvertising: StateFlow<Boolean> = _isAdvertising.asStateFlow()
    val discoveredDevices: StateFlow<Map<String, BleDevice>> = discoveryListener.discoveredDevices
    val connectionStates: StateFlow<Map<String, ConnectionState>> = connectionManager.connectionStates
    val receivedMessages: Flow<TestMessage> = connectionManager.receivedMessages
    val discoveryEvents: Flow<DiscoveryEvent> = discoveryListener.discoveryEvents

    // 作用域
    private val managerScope = CoroutineScope(Dispatchers.Main + SupervisorJob())

    init {
        initialize()
    }

    /**
     * 初始化BLE管理器
     */
    private fun initialize() {
        Log.d(TAG, "初始化BLE管理器")

        managerScope.launch {
            try {
                // 检查蓝牙可用性
                if (!scanner.isBluetoothAvailable()) {
                    _managerState.value = BleManagerState.BLUETOOTH_UNAVAILABLE
                    Log.w(TAG, "蓝牙不可用")
                    return@launch
                }

                // 检查权限
                if (!hasRequiredPermissions()) {
                    _managerState.value = BleManagerState.PERMISSIONS_REQUIRED
                    Log.w(TAG, "缺少必要权限")
                    return@launch
                }

                _managerState.value = BleManagerState.READY
                Log.d(TAG, "BLE管理器初始化完成")
            } catch (e: Exception) {
                _managerState.value = BleManagerState.ERROR
                Log.e(TAG, "BLE管理器初始化失败", e)
            }
        }
    }

    /**
     * 检查是否有必需的权限
     */
    fun hasRequiredPermissions(): Boolean {
        return permissionManager.hasAllBluetoothPermissions() &&
               permissionManager.hasLocationPermissions()
    }

    /**
     * 获取权限说明文本
     */
    fun getPermissionsRationale(): String {
        return permissionManager.getAllPermissionsRationaleText()
    }

    /**
     * 开始设备扫描
     */
    fun startScanning(): Result<Unit> {
        Log.d(TAG, "开始设备扫描")

        if (_managerState.value != BleManagerState.READY) {
            return Result.failure(IllegalStateException("BLE管理器未就绪: ${_managerState.value}"))
        }

        if (_isScanning.value) {
            Log.w(TAG, "已经在扫描中")
            return Result.success(Unit)
        }

        val result = scanner.startScanning()
        if (result.isSuccess) {
            discoveryListener.startDiscovery()
            _isScanning.value = true
            Log.d(TAG, "设备扫描已启动")
        }

        return result
    }

    /**
     * 停止设备扫描
     */
    fun stopScanning() {
        Log.d(TAG, "停止设备扫描")

        if (!_isScanning.value) {
            return
        }

        discoveryListener.stopDiscovery()
        scanner.stopScanning()
        _isScanning.value = false
        Log.d(TAG, "设备扫描已停止")
    }

    /**
     * 开始广播
     */
    fun startAdvertising(deviceName: String = "NearClip-${getDeviceIdentifier()}"): Result<Unit> {
        Log.d(TAG, "开始广播: $deviceName")

        if (_managerState.value != BleManagerState.READY) {
            return Result.failure(IllegalStateException("BLE管理器未就绪: ${_managerState.value}"))
        }

        if (_isAdvertising.value) {
            Log.w(TAG, "已经在广播中")
            return Result.success(Unit)
        }

        val result = advertiser.startAdvertising(deviceName)
        if (result.isSuccess) {
            _isAdvertising.value = true
            Log.d(TAG, "广播已启动")
        }

        return result
    }

    /**
     * 停止广播
     */
    fun stopAdvertising() {
        Log.d(TAG, "停止广播")

        if (!_isAdvertising.value) {
            return
        }

        advertiser.stopAdvertising()
        _isAdvertising.value = false
        Log.d(TAG, "广播已停止")
    }

    /**
     * 连接到设备
     */
    fun connectToDevice(device: BleDevice): Result<Unit> {
        Log.d(TAG, "连接到设备: ${device.deviceName}")

        if (_managerState.value != BleManagerState.READY) {
            return Result.failure(IllegalStateException("BLE管理器未就绪: ${_managerState.value}"))
        }

        return connectionManager.connectToDevice(device)
    }

    /**
     * 断开设备连接
     */
    fun disconnectFromDevice(deviceId: String) {
        Log.d(TAG, "断开设备连接: $deviceId")
        connectionManager.disconnectFromDevice(deviceId)
    }

    /**
     * 发送消息
     */
    fun sendMessage(deviceId: String, message: TestMessage): Result<Unit> {
        Log.d(TAG, "发送消息到设备 $deviceId: ${message.messageId}")
        return connectionManager.sendMessage(deviceId, message)
    }

    /**
     * 发送Ping消息
     */
    fun sendPing(deviceId: String): Result<Unit> {
        val pingMessage = TestMessage(
            messageId = "ping-${System.currentTimeMillis()}",
            type = MessageType.PING,
            payload = "",
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 0
        )
        return sendMessage(deviceId, pingMessage)
    }

    /**
     * 发送Pong消息
     */
    fun sendPong(deviceId: String, originalMessageId: String): Result<Unit> {
        val pongMessage = TestMessage(
            messageId = "pong-${System.currentTimeMillis()}",
            type = MessageType.PONG,
            payload = originalMessageId,
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 0
        )
        return sendMessage(deviceId, pongMessage)
    }

    /**
     * 发送数据消息
     */
    fun sendData(deviceId: String, payload: String): Result<Unit> {
        val dataMessage = TestMessage(
            messageId = "data-${System.currentTimeMillis()}",
            type = MessageType.DATA,
            payload = payload,
            timestamp = System.currentTimeMillis(),
            sequenceNumber = 1
        )
        return sendMessage(deviceId, dataMessage)
    }

    /**
     * 获取NearClip设备列表
     */
    fun getNearClipDevices(): List<BleDevice> {
        return discoveryListener.getNearClipDevices()
    }

    /**
     * 获取已连接的设备列表
     */
    fun getConnectedDevices(): List<String> {
        return connectionManager.getConnectedDevices()
    }

    /**
     * 获取发现统计
     */
    fun getDiscoveryStats(): DiscoveryStats {
        return discoveryListener.getDiscoveryStats()
    }

    /**
     * 清除发现的设备
     */
    fun clearDiscoveredDevices() {
        discoveryListener.clearDevices()
    }

    /**
     * 根据设备ID获取设备
     */
    fun getDevice(deviceId: String): BleDevice? {
        return discoveryListener.getDevice(deviceId)
    }

    /**
     * 获取设备标识符
     */
    private fun getDeviceIdentifier(): String {
        return try {
            android.provider.Settings.Secure.getString(
                context.contentResolver,
                android.provider.Settings.Secure.ANDROID_ID
            ).take(8).uppercase()
        } catch (e: Exception) {
            "UNKNOWN"
        }
    }

    /**
     * 获取管理器状态说明
     */
    fun getStateDescription(): String {
        return when (_managerState.value) {
            BleManagerState.INITIALIZING -> "正在初始化..."
            BleManagerState.READY -> "就绪"
            BleManagerState.PERMISSIONS_REQUIRED -> "需要权限"
            BleManagerState.BLUETOOTH_UNAVAILABLE -> "蓝牙不可用"
            BleManagerState.ERROR -> "发生错误"
        }
    }

    /**
     * 获取状态信息
     */
    fun getStatusInfo(): Map<String, Any> {
        return mapOf(
            "managerState" to _managerState.value.name,
            "stateDescription" to getStateDescription(),
            "isScanning" to _isScanning.value,
            "isAdvertising" to _isAdvertising.value,
            "discoveredDevicesCount" to discoveredDevices.value.size,
            "connectedDevicesCount" to getConnectedDevices().size,
            "nearClipDevicesCount" to getNearClipDevices().size,
            "hasPermissions" to hasRequiredPermissions(),
            "bluetoothAvailable" to scanner.isBluetoothAvailable()
        )
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        Log.d(TAG, "清理BLE管理器")

        managerScope.cancel()

        stopScanning()
        stopAdvertising()

        discoveryListener.cleanup()
        scanner.cleanup()
        advertiser.cleanup()
        connectionManager.cleanup()

        Log.d(TAG, "BLE管理器清理完成")
    }

    /**
     * 重新初始化
     */
    fun reinitialize() {
        Log.d(TAG, "重新初始化BLE管理器")
        cleanup()
        initialize()
    }
}

/**
 * BLE管理器状态
 */
enum class BleManagerState {
    INITIALIZING,
    READY,
    PERMISSIONS_REQUIRED,
    BLUETOOTH_UNAVAILABLE,
    ERROR
}