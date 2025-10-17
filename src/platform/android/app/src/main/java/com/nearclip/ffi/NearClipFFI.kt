package com.nearclip.ffi

import android.content.Context
import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * NearClip Rust FFI 桥接类
 * 提供与Rust核心库的交互接口
 */
object NearClipFFI {

    // 加载本地库
    init {
        System.loadLibrary("nearclip_jni")
    }

    // 设备发现状态流
    private val _discoveryState = MutableStateFlow<DiscoveryState>(DiscoveryState.Stopped)
    val discoveryState: Flow<DiscoveryState> = _discoveryState.asStateFlow()

    // 连接状态流
    private val _connectionState = MutableStateFlow<ConnectionState>(ConnectionState.Disconnected)
    val connectionState: Flow<ConnectionState> = _connectionState.asStateFlow()

    // 发现的设备流
    private val _discoveredDevices = MutableStateFlow<List<Device>>(emptyList())
    val discoveredDevices: Flow<List<Device>> = _discoveredDevices.asStateFlow()

    /**
     * 初始化NearClip核心
     */
    external fun initialize(context: Context): Boolean

    /**
     * 开始设备发现
     */
    external fun startDeviceDiscovery(): Boolean

    /**
     * 停止设备发现
     */
    external fun stopDeviceDiscovery(): Boolean

    /**
     * 连接到设备
     */
    external fun connectToDevice(deviceId: String): Boolean

    /**
     * 断开设备连接
     */
    external fun disconnectFromDevice(deviceId: String): Boolean

    /**
     * 发送剪贴板数据
     */
    external fun sendClipboardData(data: String): Boolean

    /**
     * 获取本地设备信息
     */
    external fun getLocalDeviceInfo(): DeviceInfo?

    /**
     * 清理资源
     */
    external fun cleanup()

    // JNI回调方法 - 由Rust端调用
    @JvmStatic
    private fun onDeviceDiscovered(deviceJson: String) {
        // 解析Rust传来的设备信息
        try {
            val device = Device.fromJson(deviceJson)
            val currentList = _discoveredDevices.value.toMutableList()
            if (!currentList.any { it.deviceId == device.deviceId }) {
                currentList.add(device)
                _discoveredDevices.value = currentList
            }
        } catch (e: Exception) {
            // 处理解析错误
        }
    }

    @JvmStatic
    private fun onDiscoveryStateChanged(state: String) {
        _discoveryState.value = when (state) {
            "starting" -> DiscoveryState.Starting
            "running" -> DiscoveryState.Running
            "stopping" -> DiscoveryState.Stopping
            "stopped" -> DiscoveryState.Stopped
            else -> DiscoveryState.Stopped
        }
    }

    @JvmStatic
    private fun onConnectionChanged(deviceId: String, connected: Boolean) {
        _connectionState.value = if (connected) {
            ConnectionState.Connected(deviceId)
        } else {
            ConnectionState.Disconnected
        }
    }

    @JvmStatic
    private fun onClipboardDataReceived(data: String) {
        // 处理接收到的剪贴板数据
        // 这里可以通过事件总线或其他方式通知UI层
    }

    @JvmStatic
    private fun onError(error: String) {
        // 处理Rust端传来的错误
        // 可以通过全局错误处理器处理
    }
}

/**
 * 设备发现状态
 */
sealed class DiscoveryState {
    object Stopped : DiscoveryState()
    object Starting : DiscoveryState()
    object Running : DiscoveryState()
    object Stopping : DiscoveryState()
}

/**
 * 连接状态
 */
sealed class ConnectionState {
    object Disconnected : ConnectionState()
    object Connecting : ConnectionState()
    data class Connected(val deviceId: String) : ConnectionState()
    data class Error(val message: String) : ConnectionState()
}

/**
 * 设备信息数据类
 */
data class DeviceInfo(
    val deviceId: String,
    val deviceName: String,
    val deviceType: String,
    val publicKey: String
)