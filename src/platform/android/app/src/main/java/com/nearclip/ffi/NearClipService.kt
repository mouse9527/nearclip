package com.nearclip.ffi

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.util.Log
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.launch
import javax.inject.Inject

/**
 * NearClip后台服务
 * 管理Rust FFI的生命周期和后台任务
 */
@AndroidEntryPoint
class NearClipService : Service() {

    @Inject
    lateinit var nativeBridge: NearClipNativeBridge

    private val serviceScope = CoroutineScope(Dispatchers.Main + SupervisorJob())
    private var isServiceInitialized = false

    companion object {
        const val TAG = "NearClipService"
        const val ACTION_START_SERVICE = "com.nearclip.START_SERVICE"
        const val ACTION_STOP_SERVICE = "com.nearclip.STOP_SERVICE"
        const val ACTION_START_DISCOVERY = "com.nearclip.START_DISCOVERY"
        const val ACTION_STOP_DISCOVERY = "com.nearclip.STOP_DISCOVERY"
    }

    override fun onCreate() {
        super.onCreate()
        Log.d(TAG, "NearClipService created")
        initializeService()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.d(TAG, "onStartCommand called with action: ${intent?.action}")

        return when (intent?.action) {
            ACTION_START_SERVICE -> {
                handleStartService()
                START_STICKY
            }
            ACTION_STOP_SERVICE -> {
                handleStopService()
                START_NOT_STICKY
            }
            ACTION_START_DISCOVERY -> {
                handleStartDiscovery()
                START_STICKY
            }
            ACTION_STOP_DISCOVERY -> {
                handleStopDiscovery()
                START_STICKY
            }
            else -> START_STICKY
        }
    }

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    override fun onDestroy() {
        super.onDestroy()
        Log.d(TAG, "NearClipService destroyed")
        cleanupService()
        serviceScope.cancel()
    }

    /**
     * 初始化服务
     */
    private fun initializeService() {
        if (isServiceInitialized) return

        serviceScope.launch {
            try {
                // 加载JNI库
                if (JniLoader.loadLibrary(this@NearClipService)) {
                    // 初始化原生桥接
                    if (nativeBridge.initialize(this@NearClipService)) {
                        // 监听状态变化
                        observeStatusChanges()
                        isServiceInitialized = true
                        Log.i(TAG, "NearClipService initialized successfully")
                    } else {
                        Log.e(TAG, "Failed to initialize native bridge")
                    }
                } else {
                    Log.e(TAG, "Failed to load JNI library")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error initializing service: ${e.message}")
            }
        }
    }

    /**
     * 监听状态变化
     */
    private fun observeStatusChanges() {
        serviceScope.launch {
            nativeBridge.getDiscoveryStateFlow().collect { state ->
                Log.d(TAG, "Discovery state changed: $state")
                // 可以发送广播或通知给UI层
            }
        }

        serviceScope.launch {
            nativeBridge.getConnectionStateFlow().collect { state ->
                Log.d(TAG, "Connection state changed: $state")
                // 可以发送广播或通知给UI层
            }
        }
    }

    /**
     * 处理启动服务
     */
    private fun handleStartService() {
        if (!isServiceInitialized) {
            initializeService()
        }
    }

    /**
     * 处理停止服务
     */
    private fun handleStopService() {
        cleanupService()
        stopSelf()
    }

    /**
     * 处理开始设备发现
     */
    private fun handleStartDiscovery() {
        if (isServiceInitialized) {
            val success = nativeBridge.startDiscovery()
            Log.d(TAG, "Start discovery result: $success")
        } else {
            Log.w(TAG, "Service not initialized, cannot start discovery")
        }
    }

    /**
     * 处理停止设备发现
     */
    private fun handleStopDiscovery() {
        if (isServiceInitialized) {
            val success = nativeBridge.stopDiscovery()
            Log.d(TAG, "Stop discovery result: $success")
        } else {
            Log.w(TAG, "Service not initialized, cannot stop discovery")
        }
    }

    /**
     * 清理服务资源
     */
    private fun cleanupService() {
        if (isServiceInitialized) {
            try {
                nativeBridge.cleanup()
                isServiceInitialized = false
                Log.i(TAG, "NearClipService cleaned up successfully")
            } catch (e: Exception) {
                Log.e(TAG, "Error cleaning up service: ${e.message}")
            }
        }
    }
}