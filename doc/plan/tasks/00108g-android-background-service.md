# Task 00108g: Android 后台服务

## 任务描述

实现Android平台的后台服务，确保设备发现在应用后台和系统限制下仍能正常工作。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class DeviceDiscoveryServiceTest {
    @Test
    fun testServiceLifecycle() {
        // RED: 测试服务生命周期
        val service = DeviceDiscoveryService()
        
        assertFalse(service.isRunning)
        
        service.onCreate()
        service.onStartCommand(null, 0, 0)
        
        assertTrue(service.isRunning)
        
        service.onDestroy()
        assertFalse(service.isRunning)
    }

    @Test
    fun testBackgroundDiscovery() {
        // RED: 测试后台发现
        val service = DeviceDiscoveryService()
        
        service.onCreate()
        assertTrue(service.canRunInBackground())
        
        service.startBackgroundDiscovery()
        assertTrue(service.isDiscoveringInBackground())
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class DeviceDiscoveryService : Service() {
    private val serviceScope = CoroutineScope(Dispatchers.Main + Job())
    private var isRunning = false
    private var isDiscovering = false
    
    // 核心组件
    private lateinit var bleScanner: BLEScannerManager
    private lateinit var wifiDiscovery: WiFiDiscoveryManager
    private lateinit var permissionManager: PermissionManager
    private lateinit var batteryManager: BatteryOptimizationManager
    private lateinit var notificationManager: DiscoveryNotificationManager
    
    // 设备发现状态
    private val discoveredDevices = mutableMapOf<String, Device>()
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
    
    override fun onBind(intent: Intent): IBinder {
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
        
        bleScanner = BLEScannerManager(this, bluetoothAdapter)
        wifiDiscovery = WiFiDiscoveryManager(this, connectivityManager)
        permissionManager = PermissionManager(this as Activity)
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
        
        // 并行启动BLE和WiFi发现
        val bleJob = launchBLEDiscovery()
        val wifiJob = launchWiFiDiscovery()
        
        // 等待任一发现方式完成
        select {
            bleJob.join()
            wifiJob.join()
        }
    }
    
    private suspend fun stopDeviceScanning() {
        bleScanner.stopScan()
        wifiDiscovery.stopDiscovery()
    }
    
    private suspend fun launchBLEDiscovery() = serviceScope.launch {
        if (batteryManager.shouldEnableBackgroundDiscovery()) {
            bleScanner.startScan().collect { device ->
                handleDiscoveredDevice(device)
            }
        }
    }
    
    private suspend fun launchWiFiDiscovery() = serviceScope.launch {
        if (batteryManager.shouldEnableBackgroundDiscovery()) {
            wifiDiscovery.startDiscovery().collect { device ->
                handleDiscoveredDevice(device)
            }
        }
    }
    
    private fun handleDiscoveredDevice(device: Device) {
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
    
    private fun notifyDeviceDiscovered(device: Device) {
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
    fun getDiscoveredDevices(): List<Device> = discoveredDevices.values.toList()
    
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

class DiscoveryNotificationManager(
    private val context: Context
) {
    fun createServiceNotification(
        isDiscovering: Boolean,
        deviceCount: Int
    ): Notification {
        val intent = Intent(context, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK
        }
        
        val pendingIntent = PendingIntent.getActivity(
            context, 0, intent,
            PendingIntent.FLAG_IMMUTABLE
        )
        
        val stopIntent = Intent(context, DeviceDiscoveryService::class.java).apply {
            action = DeviceDiscoveryService.ACTION_STOP_DISCOVERY
        }
        
        val stopPendingIntent = PendingIntent.getService(
            context, 0, stopIntent,
            PendingIntent.FLAG_IMMUTABLE
        )
        
        return NotificationCompat.Builder(context, DeviceDiscoveryService.NOTIFICATION_CHANNEL_ID)
            .setContentTitle("NearClip 设备发现")
            .setContentText(
                if (isDiscovering) "发现中... (${deviceCount}个设备)" 
                else "已停止 (${deviceCount}个设备)"
            )
            .setSmallIcon(R.drawable.ic_discovery)
            .setContentIntent(pendingIntent)
            .addAction(
                R.drawable.ic_stop,
                "停止",
                stopPendingIntent
            )
            .setOngoing(isDiscovering)
            .setOnlyAlertOnce(true)
            .build()
    }
}

class BootReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action == Intent.ACTION_BOOT_COMPLETED) {
            val serviceIntent = Intent(context, DeviceDiscoveryService::class.java).apply {
                action = DeviceDiscoveryService.ACTION_START_DISCOVERY
            }
            
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                context.startForegroundService(serviceIntent)
            } else {
                context.startService(serviceIntent)
            }
        }
    }
}

class NetworkChangeReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        when (intent.action) {
            WifiManager.WIFI_STATE_CHANGED_ACTION,
            ConnectivityManager.CONNECTIVITY_ACTION -> {
                // 通知服务重新评估网络状态
                val serviceIntent = Intent(context, DeviceDiscoveryService::class.java).apply {
                    action = DeviceDiscoveryService.ACTION_REFRESH_DEVICES
                }
                
                context.startService(serviceIntent)
            }
        }
    }
}
```

### REFACTOR阶段
```kotlin
// 添加服务生命周期管理
// 添加后台任务调度
// 添加系统状态适配
// 添加性能监控
```

## 验收标准
- [ ] 后台服务正常启动停止
- [ ] 前台通知正确显示
- [ ] 设备发现在后台工作
- [ ] 系统启动自动恢复
- [ ] 网络变化自动适配

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108e: Android 发现UI组件](00108e-android-discovery-ui.md)

## 后续任务
- [Task 00109: macOS 设备发现实现](00109-macos-device-discovery.md)