package com.nearclip.services.ble

import android.util.Log
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

/**
 * 设备发现监听器
 * 负责监听BLE设备发现事件和管理设备列表
 */
class DeviceDiscoveryListener(
    private val bleScanner: BleScanner
) {

    companion object {
        private const val TAG = "DeviceDiscoveryListener"
        private const val DEVICE_CACHE_TIMEOUT_MS = 30000L // 30秒设备缓存超时
        private const val MAX_CACHED_DEVICES = 50 // 最大缓存设备数量
    }

    private val _discoveredDevices = MutableStateFlow<Map<String, BleDevice>>(emptyMap())
    private val _discoveryEvents = Channel<DiscoveryEvent>(capacity = Channel.UNLIMITED)
    private var discoveryJob: Job? = null
    private var cleanupJob: Job? = null

    val discoveredDevices: StateFlow<Map<String, BleDevice>> = _discoveredDevices.asStateFlow()
    val discoveryEvents: Flow<DiscoveryEvent> = _discoveryEvents.receiveAsFlow()

    /**
     * 开始监听设备发现
     */
    fun startDiscovery() {
        Log.d(TAG, "开始设备发现监听")

        if (discoveryJob?.isActive == true) {
            Log.w(TAG, "设备发现监听已在运行")
            return
        }

        // 启动设备监听
        discoveryJob = CoroutineScope(Dispatchers.IO).launch {
            bleScanner.discoveredDevices.collect { device ->
                handleDeviceDiscovered(device)
            }
        }

        // 启动设备清理任务
        startCleanupJob()

        // 发送发现开始事件
        CoroutineScope(Dispatchers.IO).launch {
            _discoveryEvents.send(DiscoveryEvent.DiscoveryStarted)
        }

        Log.d(TAG, "设备发现监听已启动")
    }

    /**
     * 停止监听设备发现
     */
    fun stopDiscovery() {
        Log.d(TAG, "停止设备发现监听")

        discoveryJob?.cancel()
        cleanupJob?.cancel()
        discoveryJob = null
        cleanupJob = null

        // 发送发现停止事件
        CoroutineScope(Dispatchers.IO).launch {
            _discoveryEvents.send(DiscoveryEvent.DiscoveryStopped)
        }

        Log.d(TAG, "设备发现监听已停止")
    }

    /**
     * 手动添加设备（用于测试）
     */
    fun addDevice(device: BleDevice) {
        handleDeviceDiscovered(device)
    }

    /**
     * 移除设备
     */
    fun removeDevice(deviceId: String) {
        val currentDevices = _discoveredDevices.value.toMutableMap()
        val removedDevice = currentDevices.remove(deviceId)

        if (removedDevice != null) {
            _discoveredDevices.value = currentDevices

            CoroutineScope(Dispatchers.IO).launch {
                _discoveryEvents.send(DiscoveryEvent.DeviceRemoved(removedDevice))
            }

            Log.d(TAG, "移除设备: ${removedDevice.deviceName} ($deviceId)")
        }
    }

    /**
     * 清除所有设备
     */
    fun clearDevices() {
        val deviceCount = _discoveredDevices.value.size
        _discoveredDevices.value = emptyMap()

        CoroutineScope(Dispatchers.IO).launch {
            _discoveryEvents.send(DiscoveryEvent.AllDevicesCleared)
        }

        Log.d(TAG, "清除所有设备 ($deviceCount 个)")
    }

    /**
     * 获取NearClip设备列表
     */
    fun getNearClipDevices(): List<BleDevice> {
        return _discoveredDevices.value.values.filter { device ->
            device.deviceType == BleDeviceType.NEARCLIP ||
            device.deviceName.contains("NearClip", true)
        }
    }

    /**
     * 根据设备ID获取设备
     */
    fun getDevice(deviceId: String): BleDevice? {
        return _discoveredDevices.value[deviceId]
    }

    /**
     * 获取设备发现统计
     */
    fun getDiscoveryStats(): DiscoveryStats {
        val devices = _discoveredDevices.value.values
        return DiscoveryStats(
            totalDevices = devices.size,
            nearClipDevices = devices.count { it.deviceType == BleDeviceType.NEARCLIP },
            leDevices = devices.count { it.deviceType == BleDeviceType.LE },
            dualDevices = devices.count { it.deviceType == BleDeviceType.DUAL },
            unknownDevices = devices.count { it.deviceType == BleDeviceType.UNKNOWN },
            lastDiscoveryTime = devices.maxOfOrNull { it.timestamp } ?: 0L
        )
    }

    /**
     * 处理设备发现
     */
    private fun handleDeviceDiscovered(device: BleDevice) {
        val currentDevices = _discoveredDevices.value
        val existingDevice = currentDevices[device.deviceId]

        val shouldUpdate = when {
            existingDevice == null -> {
                // 新设备
                Log.d(TAG, "发现新设备: ${device.deviceName} (${device.deviceId})")
                true
            }
            device.rssi > existingDevice.rssi + 5 -> {
                // 信号强度显著提升
                Log.d(TAG, "设备信号增强: ${device.deviceName} (${device.rssi} dBm)")
                true
            }
            device.timestamp - existingDevice.timestamp > 5000 -> {
                // 超过5秒更新
                Log.d(TAG, "设备信息更新: ${device.deviceName}")
                true
            }
            else -> {
                false
            }
        }

        if (shouldUpdate) {
            val updatedDevices = currentDevices.toMutableMap()
            updatedDevices[device.deviceId] = device
            _discoveredDevices.value = updatedDevices

            // 发送设备发现事件
            CoroutineScope(Dispatchers.IO).launch {
                val event = if (existingDevice == null) {
                    DiscoveryEvent.NewDeviceDiscovered(device)
                } else {
                    DiscoveryEvent.DeviceUpdated(device)
                }
                _discoveryEvents.send(event)
            }
        }
    }

    /**
     * 启动设备清理任务
     */
    private fun startCleanupJob() {
        cleanupJob = CoroutineScope(Dispatchers.IO).launch {
            while (isActive) {
                delay(DEVICE_CACHE_TIMEOUT_MS)
                cleanupOldDevices()
            }
        }
    }

    /**
     * 清理过期设备
     */
    private fun cleanupOldDevices() {
        val currentTime = System.currentTimeMillis()
        val currentDevices = _discoveredDevices.value.toMutableMap()
        val devicesToRemove = mutableListOf<BleDevice>()

        currentDevices.values.forEach { device ->
            if (currentTime - device.timestamp > DEVICE_CACHE_TIMEOUT_MS) {
                devicesToRemove.add(device)
            }
        }

        if (devicesToRemove.isNotEmpty()) {
            devicesToRemove.forEach { device ->
                currentDevices.remove(device.deviceId)
            }

            _discoveredDevices.value = currentDevices

            CoroutineScope(Dispatchers.IO).launch {
                _discoveryEvents.send(DiscoveryEvent.OldDevicesRemoved(devicesToRemove))
            }

            Log.d(TAG, "清理过期设备: ${devicesToRemove.size} 个")
        }

        // 如果设备数量超过限制，移除最旧的设备
        if (currentDevices.size > MAX_CACHED_DEVICES) {
            val sortedDevices = currentDevices.values.sortedBy { it.timestamp }
            val devicesToEvict = sortedDevices.take(currentDevices.size - MAX_CACHED_DEVICES)

            devicesToEvict.forEach { device ->
                currentDevices.remove(device.deviceId)
            }

            _discoveredDevices.value = currentDevices

            CoroutineScope(Dispatchers.IO).launch {
                _discoveryEvents.send(DiscoveryEvent.OldDevicesRemoved(devicesToEvict))
            }

            Log.d(TAG, "移除超量设备: ${devicesToEvict.size} 个")
        }
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        Log.d(TAG, "清理设备发现监听器")

        stopDiscovery()
        _discoveryEvents.close()
    }
}

/**
 * 设备发现事件
 */
sealed class DiscoveryEvent {
    object DiscoveryStarted : DiscoveryEvent()
    object DiscoveryStopped : DiscoveryEvent()
    data class NewDeviceDiscovered(val device: BleDevice) : DiscoveryEvent()
    data class DeviceUpdated(val device: BleDevice) : DiscoveryEvent()
    data class DeviceRemoved(val device: BleDevice) : DiscoveryEvent()
    data class OldDevicesRemoved(val devices: List<BleDevice>) : DiscoveryEvent()
    object AllDevicesCleared : DiscoveryEvent()
}

/**
 * 设备发现统计信息
 */
data class DiscoveryStats(
    val totalDevices: Int,
    val nearClipDevices: Int,
    val leDevices: Int,
    val dualDevices: Int,
    val unknownDevices: Int,
    val lastDiscoveryTime: Long
) {
    fun getLastDiscoveryTimeString(): String {
        if (lastDiscoveryTime == 0L) return "无"
        val now = System.currentTimeMillis()
        val diff = now - lastDiscoveryTime

        return when {
            diff < 60000 -> "${diff / 1000}秒前"
            diff < 3600000 -> "${diff / 60000}分钟前"
            diff < 86400000 -> "${diff / 3600000}小时前"
            else -> "${diff / 86400000}天前"
        }
    }
}