package com.nearclip.ffi

/**
 * JNI回调接口定义
 * 用于Rust端与Android端的通信
 */
interface JniCallbackHandler {

    /**
     * 设备发现回调
     */
    fun onDeviceDiscovered(deviceJson: String)

    /**
     * 设备发现状态变化回调
     */
    fun onDiscoveryStateChanged(state: String)

    /**
     * 连接状态变化回调
     */
    fun onConnectionChanged(deviceId: String, connected: Boolean)

    /**
     * 剪贴板数据接收回调
     */
    fun onClipboardDataReceived(data: String)

    /**
     * 错误回调
     */
    fun onError(error: String)
}

/**
 * JNI回调管理器
 */
object JniCallbackManager {

    private var callbackHandler: JniCallbackHandler? = null

    /**
     * 设置回调处理器
     */
    fun setCallbackHandler(handler: JniCallbackHandler) {
        callbackHandler = handler
    }

    /**
     * 移除回调处理器
     */
    fun removeCallbackHandler() {
        callbackHandler = null
    }

    // JNI调用入口 - 由Rust端调用
    @JvmStatic
    external fun onDeviceDiscovered(deviceJson: String)

    @JvmStatic
    external fun onDiscoveryStateChanged(state: String)

    @JvmStatic
    external fun onConnectionChanged(deviceId: String, connected: Boolean)

    @JvmStatic
    external fun onClipboardDataReceived(data: String)

    @JvmStatic
    external fun onError(error: String)
}