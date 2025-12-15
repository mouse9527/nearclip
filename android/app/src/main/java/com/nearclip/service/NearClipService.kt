package com.nearclip.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Binder
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import com.nearclip.MainActivity
import com.nearclip.R
import com.nearclip.ffi.*

class NearClipService : Service(), FfiNearClipCallback {

    companion object {
        const val CHANNEL_ID = "nearclip_sync_channel"
        const val NOTIFICATION_ID = 1
        const val ACTION_STOP = "com.nearclip.action.STOP"
        const val ACTION_SYNC_NOW = "com.nearclip.action.SYNC_NOW"

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
    private var isRunning = false

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
            ACTION_SYNC_NOW -> {
                clipboardMonitor?.syncCurrentClipboard()
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
            val config = FfiNearClipConfig(
                deviceName = "${Build.MANUFACTURER} ${Build.MODEL}",
                wifiEnabled = true,
                bleEnabled = true,
                autoConnect = true,
                connectionTimeoutSecs = 30u,
                heartbeatIntervalSecs = 5u,
                maxRetries = 3u
            )
            manager = FfiNearClipManager(config, this)

            // Initialize clipboard monitor
            clipboardMonitor = ClipboardMonitor(this) { content ->
                syncClipboard(content)
            }

            // Initialize clipboard writer
            clipboardWriter = ClipboardWriter(this, clipboardMonitor)

            // Initialize notification helper
            notificationHelper = NotificationHelper(this)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    private fun startSync() {
        try {
            manager?.start()
            clipboardMonitor?.startMonitoring()
            isRunning = manager?.isRunning() ?: false
            listeners.forEach { it.onRunningStateChanged(isRunning) }
        } catch (e: NearClipException) {
            listeners.forEach { it.onSyncError("Start failed: ${e.message}") }
        }
    }

    private fun stopSync() {
        clipboardMonitor?.stopMonitoring()
        manager?.stop()
        isRunning = false
        listeners.forEach { it.onRunningStateChanged(isRunning) }
    }

    // Public methods for UI interaction
    fun isRunning(): Boolean = isRunning

    fun getManager(): FfiNearClipManager? = manager

    fun syncClipboard(content: ByteArray) {
        try {
            manager?.syncClipboard(content)
        } catch (e: NearClipException) {
            listeners.forEach { it.onSyncError("Sync failed: ${e.message}") }
        }
    }

    // FfiNearClipCallback implementation
    override fun onDeviceConnected(device: FfiDeviceInfo) {
        updateNotification()
        listeners.forEach { it.onDeviceConnected(device) }
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
        listeners.forEach { it.onSyncError(errorMessage) }
    }
}
