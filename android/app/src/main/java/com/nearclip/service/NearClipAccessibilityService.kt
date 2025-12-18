package com.nearclip.service

import android.accessibilityservice.AccessibilityService
import android.accessibilityservice.AccessibilityServiceInfo
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.os.Build
import android.provider.Settings
import android.view.accessibility.AccessibilityEvent
import androidx.core.app.NotificationCompat
import java.security.MessageDigest

/**
 * Accessibility Service for monitoring clipboard changes on Android 10+.
 *
 * This service allows reading clipboard content even when the app is in background,
 * bypassing Android 10+ clipboard access restrictions.
 */
class NearClipAccessibilityService : AccessibilityService() {

    companion object {
        private const val TAG = "NearClipAccessibility"
        private const val CHANNEL_ID = "nearclip_clipboard_sync"
        private const val NOTIFICATION_ID = 2001

        @Volatile
        private var instance: NearClipAccessibilityService? = null

        @Volatile
        private var pendingClipboardSync = false

        /**
         * Check if the accessibility service is enabled.
         */
        fun isEnabled(context: Context): Boolean {
            val enabledServices = Settings.Secure.getString(
                context.contentResolver,
                Settings.Secure.ENABLED_ACCESSIBILITY_SERVICES
            ) ?: return false

            val serviceName = "${context.packageName}/${NearClipAccessibilityService::class.java.canonicalName}"
            return enabledServices.contains(serviceName)
        }

        /**
         * Open accessibility settings for user to enable the service.
         */
        fun openAccessibilitySettings(context: Context) {
            val intent = Intent(Settings.ACTION_ACCESSIBILITY_SETTINGS).apply {
                flags = Intent.FLAG_ACTIVITY_NEW_TASK
            }
            context.startActivity(intent)
        }

        /**
         * Get the current instance if service is running.
         */
        fun getInstance(): NearClipAccessibilityService? = instance

        /**
         * Try to read clipboard content using the accessibility service.
         * Returns null if service is not available or clipboard is empty.
         */
        fun readClipboard(context: Context): ByteArray? {
            return instance?.readClipboardContent()
        }

        /**
         * Check if there's a pending clipboard sync request.
         */
        fun hasPendingSync(): Boolean = pendingClipboardSync

        /**
         * Clear the pending sync flag after sync is complete.
         */
        fun clearPendingSync() {
            pendingClipboardSync = false
            instance?.cancelSyncNotification()
        }
    }

    private var clipboardManager: ClipboardManager? = null
    private var lastClipboardHash: String? = null
    private var clipboardCallback: ((ByteArray) -> Unit)? = null

    override fun onCreate() {
        super.onCreate()
        instance = this
        clipboardManager = getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
        createNotificationChannel()
        android.util.Log.i(TAG, "Accessibility service created")
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Clipboard Sync",
                NotificationManager.IMPORTANCE_HIGH
            ).apply {
                description = "Notifications for clipboard sync requests"
                setShowBadge(true)
            }
            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }

    override fun onServiceConnected() {
        super.onServiceConnected()
        android.util.Log.i(TAG, "Accessibility service connected")

        // Configure what events to listen to
        serviceInfo = serviceInfo.apply {
            // We don't need to listen to specific events for clipboard
            // The service just needs to be running to have clipboard access
            eventTypes = AccessibilityEvent.TYPES_ALL_MASK
            feedbackType = AccessibilityServiceInfo.FEEDBACK_GENERIC
            notificationTimeout = 100
            flags = AccessibilityServiceInfo.FLAG_INCLUDE_NOT_IMPORTANT_VIEWS or
                    AccessibilityServiceInfo.FLAG_REPORT_VIEW_IDS
        }

        // Start clipboard monitoring
        startClipboardMonitoring()
    }

    override fun onDestroy() {
        super.onDestroy()
        instance = null
        android.util.Log.i(TAG, "Accessibility service destroyed")
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        if (event == null) return

        // Check for copy-related events
        when (event.eventType) {
            AccessibilityEvent.TYPE_VIEW_CLICKED -> {
                // Check if user clicked a "Copy" button/menu item
                val text = event.text?.joinToString(" ")?.lowercase() ?: ""
                val contentDesc = event.contentDescription?.toString()?.lowercase() ?: ""
                val className = event.className?.toString() ?: ""

                // Also check the source node
                val sourceText = try {
                    event.source?.text?.toString()?.lowercase() ?: ""
                } catch (e: Exception) { "" }

                val isCopyAction = listOf(text, contentDesc, sourceText).any { str ->
                    str.contains("copy") || str.contains("复制") ||
                    str.contains("拷贝") || str.contains("剪切") ||
                    str.contains("cut")
                }

                if (isCopyAction) {
                    android.util.Log.i(TAG, "Copy action detected: text=$text, desc=$contentDesc")
                    // Read clipboard immediately while we have user interaction context
                    android.os.Handler(mainLooper).postDelayed({
                        checkClipboardChanged()
                    }, 50)
                }

                // Recycle the source node
                try { event.source?.recycle() } catch (e: Exception) {}
            }

            AccessibilityEvent.TYPE_VIEW_LONG_CLICKED -> {
                // Long press often opens copy menu
                android.os.Handler(mainLooper).postDelayed({
                    checkClipboardChanged()
                }, 200)
            }

            AccessibilityEvent.TYPE_VIEW_TEXT_SELECTION_CHANGED -> {
                // Text selection changed, might be followed by copy
                android.os.Handler(mainLooper).postDelayed({
                    checkClipboardChanged()
                }, 150)
            }

            AccessibilityEvent.TYPE_WINDOW_CONTENT_CHANGED -> {
                // Window content changed - check if it's a popup menu with copy
                val text = event.text?.joinToString(" ")?.lowercase() ?: ""
                if (text.contains("copy") || text.contains("复制")) {
                    android.os.Handler(mainLooper).postDelayed({
                        checkClipboardChanged()
                    }, 100)
                }
            }

            AccessibilityEvent.TYPE_NOTIFICATION_STATE_CHANGED -> {
                // Some apps show "Copied to clipboard" notifications
                val text = event.text?.joinToString(" ")?.lowercase() ?: ""
                if (text.contains("copied") || text.contains("已复制") || text.contains("剪贴板")) {
                    android.util.Log.i(TAG, "Copy notification detected: $text")
                    android.os.Handler(mainLooper).postDelayed({
                        checkClipboardChanged()
                    }, 50)
                }
            }
        }
    }

    override fun onInterrupt() {
        android.util.Log.w(TAG, "Accessibility service interrupted")
    }

    /**
     * Set callback for clipboard changes.
     */
    fun setClipboardCallback(callback: ((ByteArray) -> Unit)?) {
        this.clipboardCallback = callback
    }

    private fun startClipboardMonitoring() {
        clipboardManager?.addPrimaryClipChangedListener {
            checkClipboardChanged()
        }
        android.util.Log.i(TAG, "Started clipboard monitoring via accessibility service")
    }

    private fun checkClipboardChanged() {
        try {
            val content = readClipboardContent()
            if (content == null) {
                android.util.Log.d(TAG, "checkClipboardChanged: clipboard empty or access denied")
                // Show notification to let user bring app to foreground
                showSyncNotification()
                return
            }

            val hash = computeHash(content)

            if (hash != lastClipboardHash) {
                lastClipboardHash = hash
                android.util.Log.i(TAG, "Clipboard changed, content size: ${content.size}, hash: ${hash.take(8)}...")

                // Cancel any pending notification since we got the content
                cancelSyncNotification()

                // Notify through callback
                clipboardCallback?.invoke(content)

                // Also broadcast for NearClipService to pick up
                sendClipboardChangedBroadcast(content)
                android.util.Log.i(TAG, "Clipboard content broadcast sent")
            }
        } catch (e: SecurityException) {
            android.util.Log.w(TAG, "Clipboard access denied (background): ${e.message}")
            showSyncNotification()
        } catch (e: Exception) {
            android.util.Log.e(TAG, "Error checking clipboard: ${e.message}")
        }
    }

    private fun showSyncNotification() {
        pendingClipboardSync = true

        val intent = Intent(this, com.nearclip.MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TOP
            action = ACTION_SYNC_CLIPBOARD
        }

        val pendingIntent = PendingIntent.getActivity(
            this,
            0,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_menu_share)
            .setContentTitle("Clipboard Changed")
            .setContentText("Tap to sync clipboard to other devices")
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setAutoCancel(true)
            .setContentIntent(pendingIntent)
            .build()

        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.notify(NOTIFICATION_ID, notification)
        android.util.Log.i(TAG, "Sync notification shown")
    }

    private fun cancelSyncNotification() {
        pendingClipboardSync = false
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.cancel(NOTIFICATION_ID)
    }

    /**
     * Read current clipboard content.
     * This works even when app is in background thanks to accessibility service.
     */
    fun readClipboardContent(): ByteArray? {
        return try {
            val cm = clipboardManager ?: return null
            if (!cm.hasPrimaryClip()) {
                return null
            }

            val clip = cm.primaryClip ?: return null
            if (clip.itemCount == 0) return null

            val item = clip.getItemAt(0)
            val text = item.coerceToText(this)

            if (text.isNotEmpty()) {
                text.toString().toByteArray(Charsets.UTF_8)
            } else {
                null
            }
        } catch (e: Exception) {
            android.util.Log.e(TAG, "Failed to read clipboard: ${e.message}")
            null
        }
    }

    private fun sendClipboardChangedBroadcast(content: ByteArray) {
        val intent = Intent(ACTION_CLIPBOARD_CHANGED).apply {
            putExtra(EXTRA_CLIPBOARD_CONTENT, content)
            setPackage(packageName)
        }
        sendBroadcast(intent)
    }

    private fun computeHash(content: ByteArray): String {
        val digest = MessageDigest.getInstance("SHA-256")
        val hashBytes = digest.digest(content)
        return hashBytes.joinToString("") { "%02x".format(it) }
    }
}

// Broadcast action and extras
const val ACTION_CLIPBOARD_CHANGED = "com.nearclip.CLIPBOARD_CHANGED"
const val EXTRA_CLIPBOARD_CONTENT = "clipboard_content"
const val ACTION_SYNC_CLIPBOARD = "com.nearclip.SYNC_CLIPBOARD"
