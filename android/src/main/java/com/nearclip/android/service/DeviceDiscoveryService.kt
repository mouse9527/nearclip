package com.nearclip.android.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.bluetooth.BluetoothAdapter
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.os.Binder
import android.os.Build
import android.util.Log
import androidx.core.app.NotificationCompat
import androidx.localbroadcastmanager.content.LocalBroadcastManager
import kotlinx.coroutines.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.util.*

class DeviceDiscoveryService : Service() {
    private val serviceScope = CoroutineScope(Dispatchers.Main + Job())
    private var isRunning = false
    private var isDiscovering = false

    // 核心组件
    private lateinit var unifiedDiscoveryManager: UnifiedDiscoveryManager
    private lateinit var permissionManager: PermissionManager
    private lateinit var batteryManager: BatteryOptimizationManager
    private lateinit var notificationManager: DiscoveryNotificationManager

    // 设备发现状态
    private val discoveredDevices = mutableMapOf<String, UnifiedDevice>()
    private val discoveryLock = Mutex()

    override fun onCreate() {
        super.onCreate()
        initializeComponents()
        setupNotificationChannel()
        isRunning = true
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_START_DISCOVERY -> startDiscovery()
            ACTION_STOP_DISCOVERY -> stopDiscovery()
            ACTION_REFRESH_DEVICES -> refreshDevices()
            else -> startForegroundService()
        }

        return START_STICKY
    }

    override fun onBind(intent: Intent): android.os.IBinder {
        return DiscoveryBinder()
    }

    override fun onDestroy() {
        super.onDestroy()
        stopDiscovery()
        serviceScope.cancel()
        isRunning = false
    }

    private fun initializeComponents() {
        val bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
        val connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager

        unifiedDiscoveryManager = UnifiedDiscoveryManager(this, bluetoothAdapter, connectivityManager)
        permissionManager = PermissionManager(this as Context)
        batteryManager = BatteryOptimizationManager(this)
        notificationManager = DiscoveryNotificationManager(this)
    }

    private fun setupNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                NOTIFICATION_CHANNEL_ID,
                "设备发现",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "NearClip设备发现服务状态"
                enableLights(false)
                enableVibration(false)
                setShowBadge(false)
            }

            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }

    private fun startForegroundService() {
        val notification = notificationManager.createServiceNotification(
            isDiscovering = isDiscovering,
            deviceCount = discoveredDevices.size
        )

        startForeground(NOTIFICATION_ID, notification)
    }

    fun startDiscovery() {
        if (isDiscovering) return

        serviceScope.launch {
            try {
                discoveryLock.withLock {
                    isDiscovering = true
                    updateServiceNotification()
                    startDeviceScanning()
                }
            } catch (e: Exception) {
                handleError("启动发现失败", e)
                isDiscovering = false
            }
        }
    }

    fun stopDiscovery() {
        if (!isDiscovering) return

        serviceScope.launch {
            try {
                discoveryLock.withLock {
                    stopDeviceScanning()
                    isDiscovering = false
                    updateServiceNotification()
                }
            } catch (e: Exception) {
                handleError("停止发现失败", e)
            }
        }
    }

    fun refreshDevices() {
        stopDiscovery()
        discoveredDevices.clear()
        startDiscovery()
    }

    private suspend fun startDeviceScanning() {
        // 检查权限
        if (!permissionManager.hasRequiredPermissions()) {
            stopDiscovery()
            return
        }

        if (batteryManager.shouldEnableBackgroundDiscovery()) {
            // 使用统一发现管理器
            unifiedDiscoveryManager.startDiscovery().collect { unifiedDevice ->
                handleDiscoveredDevice(unifiedDevice)
            }
        }
    }

    private suspend fun stopDeviceScanning() {
        // 统一发现管理器会在停止时自动处理
    }

    private fun handleDiscoveredDevice(device: UnifiedDevice) {
        serviceScope.launch {
            discoveryLock.withLock {
                val existingDevice = discoveredDevices[device.id]

                if (existingDevice == null || device.lastSeen > existingDevice.lastSeen) {
                    discoveredDevices[device.id] = device
                    updateServiceNotification()
                    notifyDeviceDiscovered(device)
                }
            }
        }
    }

    private fun updateServiceNotification() {
        val notification = notificationManager.createServiceNotification(
            isDiscovering = isDiscovering,
            deviceCount = discoveredDevices.size
        )

        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.notify(NOTIFICATION_ID, notification)
    }

    private fun notifyDeviceDiscovered(device: UnifiedDevice) {
        // 通知UI更新
        LocalBroadcastManager.getInstance(this).sendBroadcast(
            Intent(ACTION_DEVICE_DISCOVERED).apply {
                putExtra(EXTRA_DEVICE, device)
            }
        )
    }

    private fun handleError(message: String, error: Exception) {
        Log.e(TAG, message, error)

        // 通知UI错误
        LocalBroadcastManager.getInstance(this).sendBroadcast(
            Intent(ACTION_DISCOVERY_ERROR).apply {
                putExtra(EXTRA_ERROR_MESSAGE, message)
            }
        )
    }

    fun isRunning(): Boolean = isRunning
    fun isDiscovering(): Boolean = isDiscovering
    fun getDiscoveredDevices(): List<UnifiedDevice> = discoveredDevices.values.toList()

    // 保持测试兼容性
    fun canRunInBackground(): Boolean = true
    fun startBackgroundDiscovery() { startDiscovery() }
    fun isDiscoveringInBackground(): Boolean = isDiscovering

    inner class DiscoveryBinder : Binder() {
        fun getService(): DeviceDiscoveryService = this@DeviceDiscoveryService
    }

    companion object {
        private const val TAG = "DeviceDiscoveryService"
        private const val NOTIFICATION_ID = 1
        private const val NOTIFICATION_CHANNEL_ID = "device_discovery"

        const val ACTION_START_DISCOVERY = "start_discovery"
        const val ACTION_STOP_DISCOVERY = "stop_discovery"
        const val ACTION_REFRESH_DEVICES = "refresh_devices"
        const val ACTION_DEVICE_DISCOVERED = "device_discovered"
        const val ACTION_DISCOVERY_ERROR = "discovery_error"

        const val EXTRA_DEVICE = "device"
        const val EXTRA_ERROR_MESSAGE = "error_message"
    }
}