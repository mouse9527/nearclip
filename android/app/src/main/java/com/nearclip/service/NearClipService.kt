package com.nearclip.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.net.wifi.WifiManager
import android.os.Binder
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import com.nearclip.MainActivity
import com.nearclip.R
import com.nearclip.data.SecureStorage
import com.nearclip.data.SyncDirection
import com.nearclip.data.SyncHistoryRepository
import com.nearclip.ffi.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONException
import org.json.JSONObject

class NearClipService : Service(), FfiNearClipCallback {

    companion object {
        const val CHANNEL_ID = "nearclip_sync_channel"
        const val NOTIFICATION_ID = 1
        const val ACTION_STOP = "com.nearclip.action.STOP"
        const val ACTION_SYNC_NOW = "com.nearclip.action.SYNC_NOW"
        // Retry strategy actions
        const val ACTION_RETRY_SYNC = NotificationHelper.ACTION_RETRY_SYNC
        const val ACTION_DISCARD_SYNC = NotificationHelper.ACTION_DISCARD_SYNC
        const val ACTION_WAIT_SYNC = NotificationHelper.ACTION_WAIT_SYNC
        // Device ID persistence
        private const val PREFS_NAME = "nearclip_prefs"
        private const val KEY_DEVICE_ID = "nearclip_device_id"
        const val MAX_PAIRED_DEVICES = 5

        fun startService(context: Context) {
            val intent = Intent(context, NearClipService::class.java)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                context.startForegroundService(intent)
            } else {
                context.startService(intent)
            }
        }

        fun stopService(context: Context) {
            val intent = Intent(context, NearClipService::class.java)
            context.stopService(intent)
        }

        fun syncNow(context: Context) {
            val intent = Intent(context, NearClipService::class.java).apply {
                action = ACTION_SYNC_NOW
            }
            context.startService(intent)
        }
    }

    private var manager: FfiNearClipManager? = null
    private var clipboardMonitor: ClipboardMonitor? = null
    private var clipboardWriter: ClipboardWriter? = null
    private var notificationHelper: NotificationHelper? = null
    private var networkMonitor: NetworkMonitor? = null
    private var secureStorage: SecureStorage? = null
    private var syncHistoryRepository: SyncHistoryRepository? = null
    private var bleManager: BleManager? = null
    private var isRunning = false
    private var pendingContent: ByteArray? = null
    private var multicastLock: WifiManager.MulticastLock? = null
    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.Main)

    // Binder for local binding
    private val binder = LocalBinder()

    inner class LocalBinder : Binder() {
        fun getService(): NearClipService = this@NearClipService
    }

    // Listeners for UI updates
    private val listeners = mutableListOf<ServiceListener>()

    interface ServiceListener {
        fun onDeviceConnected(device: FfiDeviceInfo)
        fun onDeviceDisconnected(deviceId: String)
        fun onClipboardReceived(content: ByteArray, fromDevice: String)
        fun onSyncError(errorMessage: String)
        fun onRunningStateChanged(isRunning: Boolean)
    }

    fun addListener(listener: ServiceListener) {
        listeners.add(listener)
    }

    fun removeListener(listener: ServiceListener) {
        listeners.remove(listener)
    }

    /**
     * Manually trigger clipboard sync.
     * Called when user taps the sync notification.
     */
    fun syncClipboardNow() {
        android.util.Log.i("NearClipService", "syncClipboardNow called")
        clipboardMonitor?.syncCurrentClipboard()
    }

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        initializeManager()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_STOP -> {
                stopSync()
                stopSelf()
                return START_NOT_STICKY
            }
            ACTION_SYNC_NOW, ACTION_RETRY_SYNC -> {
                // Retry sync - resync current clipboard
                android.util.Log.i("NearClipService", "ACTION_SYNC_NOW received, clipboardMonitor=${clipboardMonitor != null}")
                clipboardMonitor?.syncCurrentClipboard()
                return START_STICKY
            }
            ACTION_DISCARD_SYNC -> {
                // Discard - clear any pending content
                pendingContent = null
                return START_STICKY
            }
            ACTION_WAIT_SYNC -> {
                // Wait for device - content is already saved, just acknowledge
                // Pending content will be sent when device reconnects
                return START_STICKY
            }
        }

        startForeground(NOTIFICATION_ID, createNotification())
        startSync()

        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder {
        return binder
    }

    override fun onDestroy() {
        stopSync()
        manager?.destroy()
        super.onDestroy()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "NearClip Sync",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Shows when NearClip is syncing clipboard"
                setShowBadge(false)
            }

            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }

    private fun createNotification(): Notification {
        // Intent to open app
        val openIntent = Intent(this, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_SINGLE_TOP
        }
        val openPendingIntent = PendingIntent.getActivity(
            this, 0, openIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        // Intent to stop service
        val stopIntent = Intent(this, NearClipService::class.java).apply {
            action = ACTION_STOP
        }
        val stopPendingIntent = PendingIntent.getService(
            this, 0, stopIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val connectedCount = manager?.getConnectedDevices()?.size ?: 0
        val contentText = if (connectedCount > 0) {
            "$connectedCount device(s) connected"
        } else {
            "Waiting for connections..."
        }

        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("NearClip Sync")
            .setContentText(contentText)
            .setSmallIcon(android.R.drawable.ic_menu_share)
            .setOngoing(true)
            .setContentIntent(openPendingIntent)
            .addAction(
                android.R.drawable.ic_menu_close_clear_cancel,
                "Stop",
                stopPendingIntent
            )
            .build()
    }

    private fun updateNotification() {
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.notify(NOTIFICATION_ID, createNotification())
    }

    private fun initializeManager() {
        try {
            android.util.Log.i("NearClipService", "initializeManager() starting")
            // Load persisted device ID (empty string means auto-generate)
            val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            val persistedDeviceId = prefs.getString(KEY_DEVICE_ID, "") ?: ""
            android.util.Log.i("NearClipService", "persistedDeviceId=$persistedDeviceId")

            val config = FfiNearClipConfig(
                deviceName = "${Build.MANUFACTURER} ${Build.MODEL}",
                deviceId = persistedDeviceId,
                wifiEnabled = true,
                bleEnabled = true,
                autoConnect = true,
                connectionTimeoutSecs = 30u,
                heartbeatIntervalSecs = 5u,
                maxRetries = 3u
            )
            android.util.Log.i("NearClipService", "Creating FfiNearClipManager...")
            manager = FfiNearClipManager(config, this)
            android.util.Log.i("NearClipService", "FfiNearClipManager created successfully")

            // Save generated device ID if it was newly created
            if (persistedDeviceId.isEmpty()) {
                val generatedId = manager?.getDeviceId()
                if (!generatedId.isNullOrEmpty()) {
                    prefs.edit().putString(KEY_DEVICE_ID, generatedId).apply()
                    android.util.Log.i("NearClipService", "Saved new device ID: $generatedId")
                }
            }

            // Initialize secure storage and load paired devices
            secureStorage = SecureStorage(this)
            loadPairedDevicesFromStorage()

            // Initialize sync history repository
            syncHistoryRepository = SyncHistoryRepository(this)

            // Initialize clipboard monitor
            clipboardMonitor = ClipboardMonitor(this) { content ->
                syncClipboard(content)
            }

            // Initialize clipboard writer
            clipboardWriter = ClipboardWriter(this, clipboardMonitor)

            // Initialize notification helper
            notificationHelper = NotificationHelper(this)

            // Initialize network monitor
            networkMonitor = NetworkMonitor(this).apply {
                onNetworkLost = {
                    android.util.Log.i("NearClipService", "Network lost, attempting BLE fallback")
                    handleNetworkLost()
                }

                onNetworkRestored = {
                    android.util.Log.i("NearClipService", "Network restored, restarting WiFi sync (preserving BLE)")
                    restartWifiSync()
                }

                onReconnectFailed = {
                    android.util.Log.w("NearClipService", "Reconnection failed after multiple attempts")
                    notificationHelper?.showSyncFailureNotification(
                        reason = "Unable to reconnect after network recovery"
                    )
                }

                isConnectedToDevices = {
                    // Check both WiFi and BLE connections
                    val wifiConnected = manager?.getConnectedDevices()?.isNotEmpty() == true
                    val bleConnected = bleManager?.hasConnectedDevices() == true
                    wifiConnected || bleConnected
                }
            }

            // Initialize BLE manager
            initializeBleManager()
        } catch (e: Exception) {
            android.util.Log.e("NearClipService", "initializeManager failed: ${e.message}", e)
            e.printStackTrace()
        }
    }

    private fun loadPairedDevicesFromStorage() {
        val storage = secureStorage ?: return
        val result = storage.loadPairedDevicesResult()
        when (result) {
            is SecureStorage.StorageResult.Success -> {
                for (device in result.data) {
                    try {
                        manager?.addPairedDevice(device)
                        android.util.Log.i("NearClipService", "Loaded paired device: ${device.name} (${device.id})")
                    } catch (e: NearClipException) {
                        // Device already exists in manager - this is expected
                        android.util.Log.d("NearClipService", "Device ${device.id} already exists in manager")
                    } catch (e: Exception) {
                        android.util.Log.e("NearClipService", "Failed to add device ${device.id} to manager", e)
                    }
                }
            }
            is SecureStorage.StorageResult.Error -> {
                android.util.Log.e("NearClipService", "Failed to load paired devices from storage: ${result.message}")
            }
        }
    }

    private fun restartSync() {
        stopSync()
        android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
            startSync()
        }, 500)
    }

    /**
     * Restart only WiFi-related sync services without affecting BLE connections.
     * This is used when network is restored to avoid disrupting existing BLE connections.
     */
    private fun restartWifiSync() {
        android.util.Log.i("NearClipService", "Restarting WiFi sync (preserving BLE connections)")

        // Stop WiFi-related services only (don't touch BLE)
        manager?.stop()
        releaseMulticastLock()

        // Restart after a short delay
        android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
            // Re-initialize WiFi services
            acquireMulticastLock()

            // Restart the FFI manager
            try {
                val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
                val persistedDeviceId = prefs.getString(KEY_DEVICE_ID, "") ?: ""

                val config = FfiNearClipConfig(
                    deviceName = "${Build.MANUFACTURER} ${Build.MODEL}",
                    deviceId = persistedDeviceId,
                    wifiEnabled = true,
                    bleEnabled = true,
                    autoConnect = true,
                    connectionTimeoutSecs = 30u,
                    heartbeatIntervalSecs = 5u,
                    maxRetries = 3u
                )
                manager = FfiNearClipManager(config, this)
                manager?.start()

                // Re-add paired devices
                loadPairedDevicesFromStorage()

                // Try to connect to paired devices after a delay
                android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                    val connectedCount = manager?.tryConnectPairedDevices() ?: 0u
                    android.util.Log.i("NearClipService", "Auto-connecting to $connectedCount paired devices after WiFi restore")
                }, 2000)

                android.util.Log.i("NearClipService", "WiFi sync restarted successfully")
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "Failed to restart WiFi sync: ${e.message}")
            }
        }, 500)
    }

    /**
     * Handle network loss - attempt to connect to paired devices via BLE.
     */
    private fun handleNetworkLost() {
        if (!isRunning) return

        android.util.Log.i("NearClipService", "Network lost, attempting BLE fallback for paired devices")

        // Ensure BLE is active
        if (!hasBlePermissions()) {
            android.util.Log.w("NearClipService", "BLE permissions not granted, cannot fallback to BLE")
            return
        }

        // Start BLE scanning if not already scanning
        bleManager?.startScanning()
        bleManager?.startAdvertising()

        // Try to connect to paired devices via BLE
        val pairedDevices = manager?.getPairedDevices() ?: return
        for (device in pairedDevices) {
            if (bleManager?.isDeviceConnected(device.id) != true) {
                // Check if device was discovered via BLE
                if (bleManager?.isDeviceDiscovered(device.id) == true) {
                    android.util.Log.i("NearClipService", "Attempting BLE connection to: ${device.name} (${device.id})")
                    bleManager?.connect(device.id)
                } else {
                    android.util.Log.i("NearClipService", "Device ${device.name} not discovered via BLE yet, scanning...")
                }
            } else {
                android.util.Log.i("NearClipService", "Device ${device.name} already connected via BLE")
            }
        }
    }

    private fun acquireMulticastLock() {
        if (multicastLock == null) {
            val wifiManager = applicationContext.getSystemService(Context.WIFI_SERVICE) as? WifiManager
            multicastLock = wifiManager?.createMulticastLock("NearClip-mDNS")?.apply {
                setReferenceCounted(true)
            }
        }
        multicastLock?.let { lock ->
            if (!lock.isHeld) {
                lock.acquire()
                android.util.Log.i("NearClipService", "Acquired multicast lock for mDNS")
            }
        }
    }

    private fun releaseMulticastLock() {
        multicastLock?.let { lock ->
            if (lock.isHeld) {
                lock.release()
                android.util.Log.i("NearClipService", "Released multicast lock")
            }
        }
    }

    private fun startSync() {
        try {
            android.util.Log.i("NearClipService", "startSync() called, manager=${manager != null}")
            // Acquire multicast lock before starting mDNS services
            acquireMulticastLock()
            manager?.start()
            android.util.Log.i("NearClipService", "manager.start() completed")
            clipboardMonitor?.startMonitoring()
            networkMonitor?.startMonitoring()
            // Start BLE scanning and advertising
            startBle()
            isRunning = manager?.isRunning() ?: false
            android.util.Log.i("NearClipService", "isRunning=$isRunning")
            listeners.forEach { it.onRunningStateChanged(isRunning) }

            // Auto-connect to all paired devices after a delay for mDNS discovery
            android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                autoConnectAllPairedDevices()
            }, 3000)
        } catch (e: NearClipException) {
            android.util.Log.e("NearClipService", "startSync failed: ${e.message}", e)
            releaseMulticastLock()
            listeners.forEach { it.onSyncError("Start failed: ${e.message}") }
        } catch (e: Exception) {
            android.util.Log.e("NearClipService", "startSync unexpected error: ${e.message}", e)
            releaseMulticastLock()
        }
    }

    /**
     * Auto-connect to all paired devices that are not already connected.
     */
    private fun autoConnectAllPairedDevices() {
        val mgr = manager ?: return
        val pairedDevices = mgr.getPairedDevices()
        val connectedIds = mgr.getConnectedDevices().map { it.id }.toSet()

        android.util.Log.i("NearClipService", "Auto-connecting to ${pairedDevices.size} paired devices (${connectedIds.size} already connected)")

        Thread {
            for (device in pairedDevices) {
                if (device.id !in connectedIds) {
                    try {
                        android.util.Log.i("NearClipService", "Auto-connecting to: ${device.name} (${device.id})")
                        mgr.connectDevice(device.id)
                    } catch (e: Exception) {
                        android.util.Log.w("NearClipService", "Auto-connect failed for ${device.name}: ${e.message}")
                    }
                }
            }
        }.start()
    }

    private fun stopSync() {
        clipboardMonitor?.stopMonitoring()
        networkMonitor?.stopMonitoring()
        bleManager?.destroy()
        manager?.stop()
        // Release multicast lock after stopping mDNS services
        releaseMulticastLock()
        isRunning = false
        listeners.forEach { it.onRunningStateChanged(isRunning) }
    }

    private fun initializeBleManager() {
        try {
            bleManager = BleManager(this).apply {
                callback = bleCallback
            }

            // Configure with device info
            val deviceId = manager?.getDeviceId() ?: return
            val publicKeyHash = deviceId.toByteArray(Charsets.UTF_8)
                .let { java.security.MessageDigest.getInstance("SHA-256").digest(it) }
                .let { android.util.Base64.encodeToString(it, android.util.Base64.NO_WRAP) }

            bleManager?.configure(deviceId, publicKeyHash)
            bleManager?.startConnectionHealthMonitoring()
            android.util.Log.i("NearClipService", "BLE manager initialized with health monitoring")
        } catch (e: Exception) {
            android.util.Log.e("NearClipService", "Failed to initialize BLE manager: ${e.message}", e)
        }
    }

    private val bleCallback = object : BleManager.Callback {
        override fun onDeviceDiscovered(deviceId: String, publicKeyHash: String?, rssi: Int) {
            android.util.Log.i("NearClipService", "BLE device discovered: $deviceId, RSSI: $rssi")

            // Auto-connect to paired devices
            val pairedDevices = manager?.getPairedDevices() ?: emptyList()
            if (pairedDevices.any { it.id == deviceId }) {
                // Check if not already connected via BLE
                if (!isDeviceConnectedViaBle(deviceId)) {
                    android.util.Log.i("NearClipService", "Auto-connecting to paired device via BLE: $deviceId")
                    bleManager?.connect(deviceId)
                }
            }
        }

        override fun onDeviceLost(deviceId: String) {
            android.util.Log.i("NearClipService", "BLE device lost: $deviceId")
        }

        override fun onDeviceConnected(deviceId: String) {
            android.util.Log.i("NearClipService", "BLE device connected: $deviceId")
        }

        override fun onDeviceDisconnected(deviceId: String) {
            android.util.Log.i("NearClipService", "BLE device disconnected: $deviceId")
        }

        override fun onDataReceived(deviceId: String, data: ByteArray) {
            android.util.Log.i("NearClipService", "BLE data received from $deviceId: ${data.size} bytes")
            // Handle received clipboard data same as WiFi
            onClipboardReceived(data, deviceId)
        }

        override fun onError(deviceId: String?, error: String) {
            android.util.Log.e("NearClipService", "BLE error for $deviceId: $error")
        }
    }

    fun startBle() {
        android.util.Log.i("NearClipService", "startBle() called")
        // Check BLE permissions before starting
        if (!hasBlePermissions()) {
            android.util.Log.w("NearClipService", "BLE permissions not granted, skipping BLE start")
            return
        }
        android.util.Log.i("NearClipService", "BLE permissions granted, starting scanning and advertising")
        bleManager?.startScanning()
        bleManager?.startAdvertising()
    }

    private fun hasBlePermissions(): Boolean {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            // Android 12+ requires BLUETOOTH_SCAN and BLUETOOTH_ADVERTISE
            checkSelfPermission(android.Manifest.permission.BLUETOOTH_SCAN) == android.content.pm.PackageManager.PERMISSION_GRANTED &&
            checkSelfPermission(android.Manifest.permission.BLUETOOTH_ADVERTISE) == android.content.pm.PackageManager.PERMISSION_GRANTED &&
            checkSelfPermission(android.Manifest.permission.BLUETOOTH_CONNECT) == android.content.pm.PackageManager.PERMISSION_GRANTED
        } else {
            // Older Android versions use location permission for BLE
            checkSelfPermission(android.Manifest.permission.ACCESS_FINE_LOCATION) == android.content.pm.PackageManager.PERMISSION_GRANTED
        }
    }

    fun stopBle() {
        bleManager?.stopScanning()
        bleManager?.stopAdvertising()
    }

    fun syncClipboardViaBle(content: ByteArray, deviceId: String) {
        bleManager?.sendData(deviceId, content)
    }

    fun isDeviceConnectedViaBle(deviceId: String): Boolean {
        return bleManager?.isDeviceConnected(deviceId) == true
    }

    // Public methods for UI interaction
    fun isRunning(): Boolean = isRunning

    fun getManager(): FfiNearClipManager? = manager

    fun getConnectedDevices(): List<FfiDeviceInfo> = manager?.getConnectedDevices() ?: emptyList()

    fun getPairedDevices(): List<FfiDeviceInfo> = manager?.getPairedDevices() ?: emptyList()

    fun getSyncHistoryRepository(): SyncHistoryRepository? = syncHistoryRepository

    fun connectDevice(deviceId: String) {
        android.util.Log.i("NearClipService", "connectDevice called for $deviceId, manager=${manager != null}")
        // Run on background thread to avoid ANR
        Thread {
            var wifiConnected = false

            // First try WiFi connection
            try {
                manager?.connectDevice(deviceId)
                wifiConnected = true
                android.util.Log.i("NearClipService", "WiFi connectDevice completed for $deviceId")
            } catch (e: NearClipException) {
                android.util.Log.w("NearClipService", "WiFi connectDevice failed: ${e.message}, trying BLE fallback")
            } catch (e: Exception) {
                android.util.Log.w("NearClipService", "WiFi connectDevice unexpected error: ${e.message}, trying BLE fallback")
            }

            // If WiFi failed, try BLE connection
            if (!wifiConnected) {
                android.util.Log.i("NearClipService", "Attempting BLE connection for $deviceId")

                // Ensure BLE is scanning
                bleManager?.startScanning()

                // Check if device is already discovered via BLE
                if (bleManager?.isDeviceDiscovered(deviceId) == true) {
                    android.util.Log.i("NearClipService", "Device $deviceId found via BLE, connecting...")
                    bleManager?.connect(deviceId)
                } else if (bleManager?.isDeviceConnected(deviceId) == true) {
                    android.util.Log.i("NearClipService", "Device $deviceId already connected via BLE")
                } else {
                    android.util.Log.i("NearClipService", "Device $deviceId not discovered via BLE yet, scanning...")
                    // Wait a bit for BLE discovery
                    Thread.sleep(3000)
                    if (bleManager?.isDeviceDiscovered(deviceId) == true) {
                        android.util.Log.i("NearClipService", "Device $deviceId found via BLE after waiting, connecting...")
                        bleManager?.connect(deviceId)
                    } else {
                        android.util.Log.w("NearClipService", "Device $deviceId not found via BLE")
                        android.os.Handler(android.os.Looper.getMainLooper()).post {
                            listeners.forEach { it.onSyncError("Connect failed: Device not found (WiFi and BLE unavailable)") }
                        }
                    }
                }
            }
        }.start()
    }

    fun disconnectDevice(deviceId: String) {
        android.util.Log.i("NearClipService", "disconnectDevice called for $deviceId")
        // Run on background thread to avoid ANR
        Thread {
            try {
                manager?.disconnectDevice(deviceId)
                android.util.Log.i("NearClipService", "disconnectDevice completed for $deviceId")
            } catch (e: NearClipException) {
                android.util.Log.e("NearClipService", "disconnectDevice failed: ${e.message}", e)
                android.os.Handler(android.os.Looper.getMainLooper()).post {
                    listeners.forEach { it.onSyncError("Disconnect failed: ${e.message}") }
                }
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "disconnectDevice unexpected error: ${e.message}", e)
            }
        }.start()
    }

    /**
     * Unpair and remove a device.
     * This notifies the remote device and removes from local storage.
     */
    fun unpairDevice(deviceId: String) {
        android.util.Log.i("NearClipService", "unpairDevice called for $deviceId")
        // Run on background thread to avoid ANR
        Thread {
            try {
                manager?.unpairDevice(deviceId)
                android.util.Log.i("NearClipService", "unpairDevice from manager completed for $deviceId")

                // Also remove from secure storage
                secureStorage?.removePairedDevice(deviceId)
                android.util.Log.i("NearClipService", "Removed device from secure storage: $deviceId")

                updateNotification()
            } catch (e: NearClipException) {
                android.util.Log.e("NearClipService", "unpairDevice failed: ${e.message}", e)
                android.os.Handler(android.os.Looper.getMainLooper()).post {
                    listeners.forEach { it.onSyncError("Unpair failed: ${e.message}") }
                }
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "unpairDevice unexpected error: ${e.message}", e)
            }
        }.start()
    }

    fun syncClipboard(content: ByteArray) {
        android.util.Log.i("NearClipService", "syncClipboard called with ${content.size} bytes, manager=${manager != null}")
        // Run on background thread to avoid ANR (FFI uses block_on which blocks)
        Thread {
            try {
                val wifiConnectedDevices = manager?.getConnectedDevices() ?: emptyList()
                val pairedDevices = manager?.getPairedDevices() ?: emptyList()

                // Find BLE-only devices (paired but not connected via WiFi, but connected via BLE)
                val wifiConnectedIds = wifiConnectedDevices.map { it.id }.toSet()
                val bleOnlyDevices = pairedDevices.filter { device ->
                    !wifiConnectedIds.contains(device.id) && isDeviceConnectedViaBle(device.id)
                }

                android.util.Log.i("NearClipService", "syncClipboard: ${wifiConnectedDevices.size} WiFi devices, ${bleOnlyDevices.size} BLE-only devices")

                val syncedDevices = mutableListOf<FfiDeviceInfo>()

                // Try WiFi first (preferred)
                if (wifiConnectedDevices.isNotEmpty()) {
                    try {
                        manager?.syncClipboard(content)
                        syncedDevices.addAll(wifiConnectedDevices)
                        android.util.Log.i("NearClipService", "syncClipboard via WiFi completed successfully")
                    } catch (e: Exception) {
                        android.util.Log.w("NearClipService", "WiFi sync failed: ${e.message}, trying BLE")
                        // WiFi failed, try BLE for each device
                        for (device in wifiConnectedDevices) {
                            if (isDeviceConnectedViaBle(device.id)) {
                                syncClipboardViaBle(content, device.id)
                                syncedDevices.add(device)
                                android.util.Log.i("NearClipService", "Synced to ${device.name} via BLE (fallback)")
                            }
                        }
                    }
                }

                // Sync to BLE-only devices
                for (device in bleOnlyDevices) {
                    syncClipboardViaBle(content, device.id)
                    syncedDevices.add(device)
                    android.util.Log.i("NearClipService", "Synced to ${device.name} via BLE (BLE-only)")
                }

                // Record sync history for synced devices
                if (syncedDevices.isNotEmpty()) {
                    android.os.Handler(android.os.Looper.getMainLooper()).post {
                        serviceScope.launch {
                            for (device in syncedDevices) {
                                syncHistoryRepository?.recordSent(
                                    deviceId = device.id,
                                    deviceName = device.name,
                                    content = content
                                )
                            }
                        }
                    }
                } else {
                    android.util.Log.w("NearClipService", "No devices available for sync")
                }
            } catch (e: NearClipException) {
                android.util.Log.e("NearClipService", "syncClipboard failed: ${e.message}", e)
                android.os.Handler(android.os.Looper.getMainLooper()).post {
                    listeners.forEach { it.onSyncError("Sync failed: ${e.message}") }
                }
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "syncClipboard unexpected error: ${e.message}", e)
            }
        }.start()
    }

    /**
     * Add a device from a pairing code (JSON string).
     * @return the name of the added device
     * @throws IllegalArgumentException if the code is invalid or missing required fields
     * @throws IllegalStateException if the manager is not initialized or device limit reached
     */
    suspend fun addDeviceFromCode(code: String): String {
        val mgr = manager
            ?: throw IllegalStateException("Service manager not initialized")

        val currentPaired = mgr.getPairedDevices()

        // Check device limit first
        if (currentPaired.size >= MAX_PAIRED_DEVICES) {
            throw IllegalStateException("Maximum $MAX_PAIRED_DEVICES devices reached. Remove a device to add a new one.")
        }

        // Parse and validate JSON
        val json = try {
            JSONObject(code)
        } catch (e: JSONException) {
            throw IllegalArgumentException("Invalid pairing code format: not valid JSON")
        }

        // Validate required fields
        val id = json.optString("id", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'id' field")
        val name = json.optString("name", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'name' field")
        val platformStr = json.optString("platform", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'platform' field")

        // Check if device already exists (allow re-adding)
        val isExisting = currentPaired.any { it.id == id }
        if (!isExisting && currentPaired.size >= MAX_PAIRED_DEVICES) {
            throw IllegalStateException("Maximum $MAX_PAIRED_DEVICES devices reached")
        }

        // Validate platform enum (handle different naming conventions)
        val platform = when (platformStr.lowercase()) {
            "macos", "mac_os" -> DevicePlatform.MAC_OS
            "android" -> DevicePlatform.ANDROID
            else -> {
                val validPlatforms = listOf("macOS", "Android")
                throw IllegalArgumentException("Invalid pairing code: unknown platform '$platformStr'. Valid: $validPlatforms")
            }
        }

        val device = FfiDeviceInfo(
            id = id,
            name = name,
            platform = platform,
            status = DeviceStatus.DISCONNECTED
        )

        // Run FFI calls on IO dispatcher
        return withContext(Dispatchers.IO) {
            // Add to manager first - this is the source of truth
            try {
                mgr.addPairedDevice(device)
                android.util.Log.i("NearClipService", "Added paired device to manager: ${device.name} (${device.id})")
            } catch (e: NearClipException) {
                throw IllegalStateException("Failed to add device to manager: ${e.message}")
            }

            // Persist to secure storage (best effort - manager is already updated)
            try {
                secureStorage?.addPairedDevice(device)
                android.util.Log.i("NearClipService", "Persisted device to secure storage: ${device.id}")
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "Failed to persist device to storage (device added to manager)", e)
            }

            // Try to connect to the newly added device after a delay for mDNS discovery
            kotlinx.coroutines.delay(2000)
            try {
                android.util.Log.i("NearClipService", "Auto-connecting to newly paired device: ${device.id}")
                mgr.connectDevice(device.id)
                android.util.Log.i("NearClipService", "Auto-connect successful for: ${device.id}")
            } catch (e: Exception) {
                android.util.Log.w("NearClipService", "Auto-connect failed (device may not be discovered yet): ${e.message}")
            }

            name
        }
    }

    // FfiNearClipCallback implementation
    override fun onDeviceConnected(device: FfiDeviceInfo) {
        updateNotification()

        // Auto-add to paired devices if not already present (bidirectional pairing)
        val pairedDevices = manager?.getPairedDevices() ?: emptyList()
        if (!pairedDevices.any { it.id == device.id }) {
            android.util.Log.i("NearClipService", "Auto-adding connected device to paired list: ${device.name}")
            try {
                manager?.addPairedDevice(device)
                secureStorage?.addPairedDevice(device)
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "Failed to auto-add device: ${e.message}")
            }
        }

        listeners.forEach { it.onDeviceConnected(device) }

        // Send pending content if using "wait for device" strategy
        sendPendingContentIfNeeded()
    }

    private fun sendPendingContentIfNeeded() {
        val content = pendingContent ?: return
        val connectedDevices = manager?.getConnectedDevices() ?: return
        if (connectedDevices.isEmpty()) return

        try {
            manager?.syncClipboard(content)
            pendingContent = null
        } catch (e: Exception) {
            // Keep pending for next attempt
        }
    }

    override fun onDeviceDisconnected(deviceId: String) {
        updateNotification()
        listeners.forEach { it.onDeviceDisconnected(deviceId) }
    }

    override fun onDeviceUnpaired(deviceId: String) {
        // Remove from local storage
        SecureStorage(this).removePairedDevice(deviceId)
        // Update notification and notify listeners
        updateNotification()
        listeners.forEach { it.onDeviceDisconnected(deviceId) }
    }

    override fun onPairingRejected(deviceId: String, reason: String) {
        android.util.Log.w("NearClipService", "Pairing rejected by device: $deviceId, reason: $reason")

        // Get device name before removing
        val deviceName = manager?.getPairedDevices()?.find { it.id == deviceId }?.name ?: deviceId

        // Remove from FFI manager
        manager?.removePairedDevice(deviceId)

        // Remove from local storage
        secureStorage?.removePairedDevice(deviceId)

        // Show notification to user
        notificationHelper?.showPairingRejectedNotification(deviceName, reason)

        // Update notification and notify listeners
        updateNotification()
        listeners.forEach { it.onDeviceDisconnected(deviceId) }
    }

    override fun onClipboardReceived(content: ByteArray, fromDevice: String) {
        // Write to local clipboard (also marks as remote to prevent sync loop)
        clipboardWriter?.writeText(content, "NearClip from $fromDevice")

        // Show sync success notification
        val contentPreview = try {
            String(content, Charsets.UTF_8)
        } catch (e: Exception) {
            null
        }

        // Find device name from connected devices
        val deviceName = manager?.getConnectedDevices()?.find { it.id == fromDevice }?.name ?: fromDevice
        notificationHelper?.showSyncSuccessNotification(deviceName, contentPreview)

        // Record sync history
        serviceScope.launch {
            syncHistoryRepository?.recordReceived(
                deviceId = fromDevice,
                deviceName = deviceName,
                content = content
            )
        }

        listeners.forEach { it.onClipboardReceived(content, fromDevice) }
    }

    override fun onSyncError(errorMessage: String) {
        // Show failure notification
        notificationHelper?.showSyncFailureNotification(reason = errorMessage)
        listeners.forEach { it.onSyncError(errorMessage) }
    }
}
