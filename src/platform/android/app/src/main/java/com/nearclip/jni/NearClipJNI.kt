package com.nearclip.jni

import timber.log.Timber

/**
 * NearClip JNI 桥接类
 *
 * 提供 Android Java/Kotlin 与 Rust 核心库之间的接口
 */
class NearClipJNI {

    private var nativeCoreHandle: Long = 0
    private var isInitialized = false

    companion object {
        init {
            try {
                System.loadLibrary("nearclip")
                Timber.i("NearClip native library loaded successfully")
            } catch (e: UnsatisfiedLinkError) {
                Timber.e(e, "Failed to load NearClip native library")
                throw e
            }
        }
    }

    /**
     * 初始化 JNI 桥接
     */
    external fun initializeNative(): Long

    /**
     * 清理原生资源
     */
    external fun cleanupNative(handle: Long)

    // 加密相关方法

    /**
     * 创建加密服务
     */
    external fun cryptoCreate(): Long

    /**
     * 销毁加密服务
     */
    external fun cryptoDestroy(handle: Long)

    /**
     * 生成会话密钥
     */
    external fun cryptoGenerateSessionKey(handle: Long): ByteArray

    /**
     * 生成随机 Nonce
     */
    external fun cryptoGenerateNonce(handle: Long): ByteArray

    /**
     * 加密数据
     */
    external fun cryptoEncrypt(
        handle: Long,
        plaintext: ByteArray,
        key: ByteArray,
        nonce: ByteArray
    ): ByteArray

    /**
     * 解密数据
     */
    external fun cryptoDecrypt(
        handle: Long,
        ciphertext: ByteArray,
        key: ByteArray,
        nonce: ByteArray
    ): ByteArray

    /**
     * 数字签名
     */
    external fun cryptoSign(handle: Long, message: ByteArray): ByteArray

    /**
     * 验证签名
     */
    external fun cryptoVerify(
        handle: Long,
        message: ByteArray,
        signature: ByteArray,
        publicKey: ByteArray
    ): Boolean

    /**
     * 生成配对码
     */
    external fun cryptoGeneratePairingCode(handle: Long): String

    /**
     * 获取设备公钥
     */
    external fun cryptoGetDevicePublicKey(handle: Long): ByteArray

    // BLE 相关方法

    /**
     * 创建 BLE 管理器
     */
    external fun bleCreate(serviceUuid: String): Long

    /**
     * 销毁 BLE 管理器
     */
    external fun bleDestroy(handle: Long)

    /**
     * 开始设备扫描
     */
    external fun bleStartScan(handle: Long, timeoutSeconds: Int): Array<String>

    /**
     * 停止设备扫描
     */
    external fun bleStopScan(handle: Long)

    /**
     * 连接到设备
     */
    external fun bleConnect(handle: Long, deviceId: String): Boolean

    /**
     * 断开设备连接
     */
    external fun bleDisconnect(handle: Long, deviceId: String): Boolean

    /**
     * 发送消息到设备
     */
    external fun bleSendMessage(handle: Long, deviceId: String, message: ByteArray): Boolean

    /**
     * 开始广播
     */
    external fun bleStartAdvertising(handle: Long, deviceInfo: ByteArray): Boolean

    /**
     * 停止广播
     */
    external fun bleStopAdvertising(handle: Long): Boolean

    /**
     * 获取连接状态
     */
    external fun bleGetConnectionState(handle: Long, deviceId: String): Int

    // 核心服务方法

    /**
     * 创建核心服务实例
     */
    external fun coreCreate(): Long

    /**
     * 销毁核心服务实例
     */
    external fun coreDestroy(handle: Long)

    /**
     * 启动核心服务
     */
    external fun coreStart(handle: Long): Boolean

    /**
     * 停止核心服务
     */
    external fun coreStop(handle: Long): Boolean

    /**
     * 获取错误消息
     */
    external fun getErrorMessage(errorCode: Int): String

    // Java/Kotlin 包装方法

    /**
     * 初始化 JNI 桥接
     */
    fun initialize() {
        if (isInitialized) {
            Timber.w("JNI bridge already initialized")
            return
        }

        try {
            nativeCoreHandle = initializeNative()
            isInitialized = true
            Timber.i("JNI bridge initialized with handle: $nativeCoreHandle")
        } catch (e: Exception) {
            Timber.e(e, "Failed to initialize JNI bridge")
            throw e
        }
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        if (!isInitialized || nativeCoreHandle == 0L) {
            return
        }

        try {
            cleanupNative(nativeCoreHandle)
            nativeCoreHandle = 0L
            isInitialized = false
            Timber.i("JNI bridge cleaned up successfully")
        } catch (e: Exception) {
            Timber.e(e, "Failed to cleanup JNI bridge")
        }
    }

    /**
     * 安全调用原生方法，处理异常
     */
    private inline fun <T> safeNativeCall(operation: String, block: () -> T): T? {
        return try {
            if (!isInitialized) {
                Timber.e("JNI bridge not initialized for operation: $operation")
                return null
            }
            block()
        } catch (e: Exception) {
            Timber.e(e, "Native call failed for operation: $operation")
            null
        }
    }

    // 加密服务包装方法

    fun createCryptoService(): Long? {
        return safeNativeCall("cryptoCreate") {
            cryptoCreate()
        }
    }

    fun destroyCryptoService(handle: Long): Boolean {
        return safeNativeCall("cryptoDestroy") {
            cryptoDestroy(handle)
            true
        } ?: false
    }

    fun generateSessionKey(handle: Long): ByteArray? {
        return safeNativeCall("generateSessionKey") {
            cryptoGenerateSessionKey(handle)
        }
    }

    fun generateNonce(handle: Long): ByteArray? {
        return safeNativeCall("generateNonce") {
            cryptoGenerateNonce(handle)
        }
    }

    fun encryptData(handle: Long, plaintext: ByteArray, key: ByteArray, nonce: ByteArray): ByteArray? {
        return safeNativeCall("encryptData") {
            cryptoEncrypt(handle, plaintext, key, nonce)
        }
    }

    fun decryptData(handle: Long, ciphertext: ByteArray, key: ByteArray, nonce: ByteArray): ByteArray? {
        return safeNativeCall("decryptData") {
            cryptoDecrypt(handle, ciphertext, key, nonce)
        }
    }

    fun generatePairingCode(handle: Long): String? {
        return safeNativeCall("generatePairingCode") {
            cryptoGeneratePairingCode(handle)
        }
    }

    // BLE 服务包装方法

    fun createBLEManager(serviceUuid: String): Long? {
        return safeNativeCall("createBLEManager") {
            bleCreate(serviceUuid)
        }
    }

    fun destroyBLEManager(handle: Long): Boolean {
        return safeNativeCall("destroyBLEManager") {
            bleDestroy(handle)
            true
        } ?: false
    }

    fun startDeviceScan(handle: Long, timeoutSeconds: Int): Array<String>? {
        return safeNativeCall("startDeviceScan") {
            bleStartScan(handle, timeoutSeconds)
        }
    }

    fun stopDeviceScan(handle: Long): Boolean {
        return safeNativeCall("stopDeviceScan") {
            bleStopScan(handle)
            true
        } ?: false
    }

    fun connectToDevice(handle: Long, deviceId: String): Boolean {
        return safeNativeCall("connectToDevice") {
            bleConnect(handle, deviceId)
        } ?: false
    }

    fun disconnectFromDevice(handle: Long, deviceId: String): Boolean {
        return safeNativeCall("disconnectFromDevice") {
            bleDisconnect(handle, deviceId)
        } ?: false
    }

    fun sendMessageToDevice(handle: Long, deviceId: String, message: ByteArray): Boolean {
        return safeNativeCall("sendMessageToDevice") {
            bleSendMessage(handle, deviceId, message)
        } ?: false
    }

    fun startAdvertising(handle: Long, deviceInfo: ByteArray): Boolean {
        return safeNativeCall("startAdvertising") {
            bleStartAdvertising(handle, deviceInfo)
        } ?: false
    }

    fun stopAdvertising(handle: Long): Boolean {
        return safeNativeCall("stopAdvertising") {
            bleStopAdvertising(handle)
            true
        } ?: false
    }

    // 核心服务包装方法

    fun createCoreService(): Long? {
        return safeNativeCall("createCoreService") {
            coreCreate()
        }
    }

    fun destroyCoreService(handle: Long): Boolean {
        return safeNativeCall("destroyCoreService") {
            coreDestroy(handle)
            true
        } ?: false
    }

    fun startCoreService(handle: Long): Boolean {
        return safeNativeCall("startCoreService") {
            coreStart(handle)
        } ?: false
    }

    fun stopCoreService(handle: Long): Boolean {
        return safeNativeCall("stopCoreService") {
            coreStop(handle)
        } ?: false
    }

    fun getErrorMessage(errorCode: Int): String {
        return try {
            getErrorMessage(errorCode)
        } catch (e: Exception) {
            Timber.e(e, "Failed to get error message for code: $errorCode")
            "Unknown error"
        }
    }

    /**
     * 检查是否已初始化
     */
    fun isInitialized(): Boolean = isInitialized
}