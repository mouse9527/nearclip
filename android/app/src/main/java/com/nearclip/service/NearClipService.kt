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
import com.nearclip.ffi.*

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
    private var isRunning = false
    private var pendingContent: ByteArray? = null
    private var multicastLock: WifiManager.MulticastLock? = null

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
                onNetworkRestored = {
                    android.util.Log.i("NearClipService", "Network restored, restarting service")
                    restartSync()
                }

                onReconnectFailed = {
                    android.util.Log.w("NearClipService", "Reconnection failed after multiple attempts")
                    notificationHelper?.showSyncFailureNotification(
                        reason = "Unable to reconnect after network recovery"
                    )
                }

                isConnectedToDevices = {
                    manager?.getConnectedDevices()?.isNotEmpty() == true
                }
            }
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
            isRunning = manager?.isRunning() ?: false
            android.util.Log.i("NearClipService", "isRunning=$isRunning")
            listeners.forEach { it.onRunningStateChanged(isRunning) }
        } catch (e: NearClipException) {
            android.util.Log.e("NearClipService", "startSync failed: ${e.message}", e)
            releaseMulticastLock()
            listeners.forEach { it.onSyncError("Start failed: ${e.message}") }
        } catch (e: Exception) {
            android.util.Log.e("NearClipService", "startSync unexpected error: ${e.message}", e)
            releaseMulticastLock()
        }
    }

    private fun stopSync() {
        clipboardMonitor?.stopMonitoring()
        networkMonitor?.stopMonitoring()
        manager?.stop()
        // Release multicast lock after stopping mDNS services
        releaseMulticastLock()
        isRunning = false
        listeners.forEach { it.onRunningStateChanged(isRunning) }
    }

    // Public methods for UI interaction
    fun isRunning(): Boolean = isRunning

    fun getManager(): FfiNearClipManager? = manager

    fun getConnectedDevices(): List<FfiDeviceInfo> = manager?.getConnectedDevices() ?: emptyList()

    fun getPairedDevices(): List<FfiDeviceInfo> = manager?.getPairedDevices() ?: emptyList()

    fun connectDevice(deviceId: String) {
        android.util.Log.i("NearClipService", "connectDevice called for $deviceId, manager=${manager != null}")
        // Run on background thread to avoid ANR
        Thread {
            try {
                manager?.connectDevice(deviceId)
                android.util.Log.i("NearClipService", "connectDevice completed for $deviceId")
            } catch (e: NearClipException) {
                android.util.Log.e("NearClipService", "connectDevice failed: ${e.message}", e)
                android.os.Handler(android.os.Looper.getMainLooper()).post {
                    listeners.forEach { it.onSyncError("Connect failed: ${e.message}") }
                }
            } catch (e: Exception) {
                android.util.Log.e("NearClipService", "connectDevice unexpected error: ${e.message}", e)
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

    fun syncClipboard(content: ByteArray) {
        android.util.Log.i("NearClipService", "syncClipboard called with ${content.size} bytes, manager=${manager != null}")
        // Run on background thread to avoid ANR (FFI uses block_on which blocks)
        Thread {
            try {
                val connected = manager?.getConnectedDevices()?.size ?: 0
                android.util.Log.i("NearClipService", "syncClipboard: $connected connected devices before sync")
                manager?.syncClipboard(content)
                android.util.Log.i("NearClipService", "syncClipboard completed successfully")
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

    // FfiNearClipCallback implementation
    override fun onDeviceConnected(device: FfiDeviceInfo) {
        updateNotification()
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

        listeners.forEach { it.onClipboardReceived(content, fromDevice) }
    }

    override fun onSyncError(errorMessage: String) {
        // Show failure notification
        notificationHelper?.showSyncFailureNotification(reason = errorMessage)
        listeners.forEach { it.onSyncError(errorMessage) }
    }
}
