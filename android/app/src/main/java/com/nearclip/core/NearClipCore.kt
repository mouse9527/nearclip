package com.nearclip.core

import android.content.Context
import android.util.Log
import com.nearclip.ble.BLEManager
import com.nearclip.crypto.CryptoService
import com.nearclip.sync.SyncManager
import com.nearclip.jni.NearClipJNI
import kotlinx.coroutines.*
import timber.log.Timber
import java.util.concurrent.Executors

/**
 * NearClip Android 核心类
 *
 * 这是 NearClip 在 Android 平台的主要接口，封装了 Rust 核心库并提供 Android 特定的功能
 */
class NearClipCore private constructor(
    private val context: Context
) {
    companion object {
        @Volatile
        private var INSTANCE: NearClipCore? = null

        /**
         * 获取核心实例（单例模式）
         */
        fun getInstance(context: Context): NearClipCore {
            return INSTANCE ?: synchronized(this) {
                INSTANCE ?: NearClipCore(context.applicationContext).also { INSTANCE = it }
            }
        }
    }

    // 核心服务
    private val jniBridge: NearClipJNI = NearClipJNI()
    private lateinit var cryptoService: CryptoService
    private lateinit var bleManager: BLEManager
    private lateinit var syncManager: SyncManager

    // 协程作用域
    private val coreScope = CoroutineScope(Dispatchers.Default + SupervisorJob())
    private val singleThreadExecutor = Executors.newSingleThreadExecutor()

    // 状态管理
    private var isStarted = false
    private var isInitialized = false

    /**
     * 初始化核心服务
     */
    fun initialize() {
        if (isInitialized) {
            Timber.w("NearClip core already initialized")
            return
        }

        try {
            // 加载 Rust 本地库
            System.loadLibrary("nearclip")
            Timber.i("Native library loaded successfully")

            // 初始化 JNI 桥接
            jniBridge.initialize()

            // 初始化各个服务
            cryptoService = CryptoService(jniBridge)
            bleManager = BLEManager(context, jniBridge)
            syncManager = SyncManager(cryptoService, bleManager)

            isInitialized = true
            Timber.i("NearClip core initialized successfully")

        } catch (e: Exception) {
            Timber.e(e, "Failed to initialize NearClip core")
            throw e
        }
    }

    /**
     * 启动 NearClip 服务
     */
    fun start() {
        if (!isInitialized) {
            initialize()
        }

        if (isStarted) {
            Timber.w("NearClip services already started")
            return
        }

        coreScope.launch {
            try {
                // 启动 BLE 管理
                bleManager.start()

                // 启动同步管理
                syncManager.start()

                isStarted = true
                Timber.i("NearClip services started successfully")

            } catch (e: Exception) {
                Timber.e(e, "Failed to start NearClip services")
                // 可以在这里发送错误事件或通知
            }
        }
    }

    /**
     * 停止 NearClip 服务
     */
    fun stop() {
        if (!isStarted) {
            return
        }

        coreScope.launch {
            try {
                // 停止同步管理
                syncManager.stop()

                // 停止 BLE 管理
                bleManager.stop()

                isStarted = false
                Timber.i("NearClip services stopped successfully")

            } catch (e: Exception) {
                Timber.e(e, "Failed to stop NearClip services")
            }
        }
    }

    /**
     * 暂停后台任务（用于节省电池）
     */
    fun pauseBackgroundTasks() {
        coreScope.launch {
            try {
                bleManager.pauseScanning()
                syncManager.pauseAutoSync()
                Timber.d("Background tasks paused")
            } catch (e: Exception) {
                Timber.e(e, "Failed to pause background tasks")
            }
        }
    }

    /**
     * 恢复后台任务
     */
    fun resumeBackgroundTasks() {
        coreScope.launch {
            try {
                bleManager.resumeScanning()
                syncManager.resumeAutoSync()
                Timber.d("Background tasks resumed")
            } catch (e: Exception) {
                Timber.e(e, "Failed to resume background tasks")
            }
        }
    }

    /**
     * 清理缓存
     */
    fun clearCache() {
        coreScope.launch {
            try {
                syncManager.clearCache()
                bleManager.clearDeviceCache()
                Timber.d("Cache cleared")
            } catch (e: Exception) {
                Timber.e(e, "Failed to clear cache")
            }
        }
    }

    /**
     * 减少缓存大小
     */
    fun reduceCacheSize() {
        coreScope.launch {
            try {
                syncManager.reduceCacheSize()
                bleManager.reduceDeviceCache()
                Timber.d("Cache size reduced")
            } catch (e: Exception) {
                Timber.e(e, "Failed to reduce cache size")
            }
        }
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        coreScope.launch {
            try {
                stop()
                jniBridge.cleanup()
                Timber.d("NearClip core cleaned up")
            } catch (e: Exception) {
                Timber.e(e, "Failed to cleanup NearClip core")
            }
        }
    }

    /**
     * 完全关闭（用于应用退出时）
     */
    fun shutdown() {
        cleanup()
        coreScope.cancel()
        singleThreadExecutor.shutdown()
        INSTANCE = null
        Timber.i("NearClip core shutdown completed")
    }

    // Getters

    /**
     * 获取加密服务
     */
    fun getCryptoService(): CryptoService {
        if (!::cryptoService.isInitialized) {
            throw IllegalStateException("CryptoService not initialized")
        }
        return cryptoService
    }

    /**
     * 获取 BLE 管理器
     */
    fun getBLEManager(): BLEManager {
        if (!::bleManager.isInitialized) {
            throw IllegalStateException("BLEManager not initialized")
        }
        return bleManager
    }

    /**
     * 获取同步管理器
     */
    fun getSyncManager(): SyncManager {
        if (!::syncManager.isInitialized) {
            throw IllegalStateException("SyncManager not initialized")
        }
        return syncManager
    }

    /**
     * 检查是否已启动
     */
    fun isStarted(): Boolean = isStarted

    /**
     * 检查是否已初始化
     */
    fun isInitialized(): Boolean = isInitialized

    /**
     * 获取当前状态信息
     */
    fun getStatus(): Map<String, Any> {
        return mapOf(
            "initialized" to isInitialized,
            "started" to isStarted,
            "ble_enabled" to if (::bleManager.isInitialized) bleManager.isEnabled() else false,
            "connected_devices" to if (::bleManager.isInitialized) bleManager.getConnectedDeviceCount() else 0,
            "sync_active" to if (::syncManager.isInitialized) syncManager.isSyncActive() else false
        )
    }
}