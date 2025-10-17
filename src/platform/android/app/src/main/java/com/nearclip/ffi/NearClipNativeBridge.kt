package com.nearclip.ffi

import android.content.Context
import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow
import javax.inject.Inject
import javax.inject.Singleton

/**
 * NearClip原生桥接服务
 * 封装Rust FFI调用并提供更高级的接口
 */
@Singleton
class NearClipNativeBridge @Inject constructor() {

    private var isInitialized = false

    /**
     * 初始化桥接服务
     */
    fun initialize(context: Context): Boolean {
        if (!isInitialized) {
            isInitialized = NearClipFFI.initialize(context)
        }
        return isInitialized
    }

    /**
     * 开始设备发现
     */
    fun startDiscovery(): Boolean {
        return if (isInitialized) {
            NearClipFFI.startDeviceDiscovery()
        } else {
            false
        }
    }

    /**
     * 停止设备发现
     */
    fun stopDiscovery(): Boolean {
        return if (isInitialized) {
            NearClipFFI.stopDeviceDiscovery()
        } else {
            false
        }
    }

    /**
     * 连接到设备
     */
    fun connectToDevice(deviceId: String): Boolean {
        return if (isInitialized) {
            NearClipFFI.connectToDevice(deviceId)
        } else {
            false
        }
    }

    /**
     * 断开设备连接
     */
    fun disconnectFromDevice(deviceId: String): Boolean {
        return if (isInitialized) {
            NearClipFFI.disconnectFromDevice(deviceId)
        } else {
            false
        }
    }

    /**
     * 发送剪贴板数据
     */
    fun sendClipboardData(data: String): Boolean {
        return if (isInitialized) {
            NearClipFFI.sendClipboardData(data)
        } else {
            false
        }
    }

    /**
     * 获取本地设备信息
     */
    fun getLocalDeviceInfo(): DeviceInfo? {
        return if (isInitialized) {
            NearClipFFI.getLocalDeviceInfo()
        } else {
            null
        }
    }

    /**
     * 获取设备发现状态流
     */
    fun getDiscoveryStateFlow(): Flow<NearClipFFI.DiscoveryState> {
        return NearClipFFI.discoveryState
    }

    /**
     * 获取连接状态流
     */
    fun getConnectionStateFlow(): Flow<NearClipFFI.ConnectionState> {
        return NearClipFFI.connectionState
    }

    /**
     * 获取发现的设备流
     */
    fun getDiscoveredDevicesFlow(): Flow<List<Device>> {
        return NearClipFFI.discoveredDevices
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        if (isInitialized) {
            NearClipFFI.cleanup()
            isInitialized = false
        }
    }
}