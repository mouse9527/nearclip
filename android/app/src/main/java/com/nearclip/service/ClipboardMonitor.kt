package com.nearclip.service

import android.Manifest
import android.content.BroadcastReceiver
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import androidx.core.content.ContextCompat
import java.io.BufferedReader
import java.io.InputStreamReader
import java.security.MessageDigest
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicBoolean

/**
 * Monitors clipboard changes and triggers sync.
 *
 * On Android 10+, uses AccessibilityService to read clipboard in background.
 * Falls back to standard clipboard access when app is in foreground.
 *
 * If READ_LOGS permission is granted, monitors logcat to detect clipboard access
 * denial and automatically brings a floating activity to foreground to read clipboard.
 */
class ClipboardMonitor(
    private val context: Context,
    private val onClipboardChanged: (ByteArray) -> Unit
) {
    companion object {
        private const val TAG = "ClipboardMonitor"

        @Volatile
        private var instance: ClipboardMonitor? = null

        fun getInstance(): ClipboardMonitor? = instance
    }

    init {
        instance = this
    }
    private val clipboardManager = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
    private val handler = Handler(Looper.getMainLooper())

    // Track recent content hashes to prevent sync loops
    private val recentHashes = mutableSetOf<String>()
    private val maxRecentHashes = 10

    // Flag to temporarily ignore changes (when we write to clipboard)
    private var ignoreNextChange = false

    private val clipboardListener = ClipboardManager.OnPrimaryClipChangedListener {
        if (ignoreNextChange) {
            ignoreNextChange = false
            return@OnPrimaryClipChangedListener
        }
        handleClipboardChange()
    }

    // Broadcast receiver for accessibility service clipboard changes
    private val accessibilityClipboardReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            if (intent?.action == ACTION_CLIPBOARD_CHANGED) {
                val content = intent.getByteArrayExtra(EXTRA_CLIPBOARD_CONTENT) ?: return
                android.util.Log.i("ClipboardMonitor", "Received clipboard from accessibility service: ${content.size} bytes")

                if (ignoreNextChange) {
                    ignoreNextChange = false
                    return
                }

                // Check if this is content we recently synced (prevent loop)
                val hash = computeHash(content)
                synchronized(recentHashes) {
                    if (recentHashes.contains(hash)) {
                        return
                    }
                }

                onClipboardChanged(content)
            }
        }
    }

    private var isMonitoring = false
    private var isAccessibilityReceiverRegistered = false
    private val logcatMonitorRunning = AtomicBoolean(false)
    private val logcatExecutor = Executors.newSingleThreadExecutor()

    /**
     * Check if accessibility service is enabled for background clipboard access.
     */
    fun isAccessibilityServiceEnabled(): Boolean {
        return NearClipAccessibilityService.isEnabled(context)
    }

    /**
     * Check if READ_LOGS permission is granted (required for automatic clipboard sync).
     */
    fun hasReadLogsPermission(): Boolean {
        return ContextCompat.checkSelfPermission(context, Manifest.permission.READ_LOGS) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * Start monitoring clipboard changes.
     */
    fun startMonitoring() {
        if (isMonitoring) return

        // Standard clipboard listener (works when app is in foreground)
        clipboardManager.addPrimaryClipChangedListener(clipboardListener)

        // Register broadcast receiver for accessibility service
        if (!isAccessibilityReceiverRegistered) {
            val filter = IntentFilter(ACTION_CLIPBOARD_CHANGED)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                context.registerReceiver(accessibilityClipboardReceiver, filter, Context.RECEIVER_NOT_EXPORTED)
            } else {
                context.registerReceiver(accessibilityClipboardReceiver, filter)
            }
            isAccessibilityReceiverRegistered = true
            android.util.Log.i(TAG, "Registered accessibility clipboard receiver")
        }

        // Connect to accessibility service if available
        NearClipAccessibilityService.getInstance()?.setClipboardCallback { content ->
            handler.post {
                if (ignoreNextChange) {
                    ignoreNextChange = false
                    return@post
                }

                val hash = computeHash(content)
                synchronized(recentHashes) {
                    if (!recentHashes.contains(hash)) {
                        android.util.Log.i(TAG, "Clipboard from accessibility callback: ${content.size} bytes")
                        onClipboardChanged(content)
                    }
                }
            }
        }

        // Start logcat monitoring if READ_LOGS permission is granted (Android 10+)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q && hasReadLogsPermission()) {
            startLogcatMonitoring()
        }

        isMonitoring = true
        android.util.Log.i(TAG, "Clipboard monitoring started (accessibility: ${isAccessibilityServiceEnabled()}, readLogs: ${hasReadLogsPermission()})")
    }

    /**
     * Monitor logcat for clipboard access denial messages.
     * When detected, launch floating activity to read clipboard.
     */
    private fun startLogcatMonitoring() {
        if (logcatMonitorRunning.getAndSet(true)) {
            return // Already running
        }

        logcatExecutor.execute {
            try {
                val timeStamp = SimpleDateFormat("yyyy-MM-dd HH:mm:ss.SSS", Locale.US).format(Date())
                val packageName = context.packageName

                // Listen only for ClipboardService errors after now
                val logcatFilter = if (Build.VERSION.SDK_INT > 35) { // VANILLA_ICE_CREAM
                    "E ClipboardService"
                } else {
                    "ClipboardService:E"
                }

                android.util.Log.i(TAG, "Starting logcat monitoring for clipboard denial")

                val process = Runtime.getRuntime().exec(arrayOf("logcat", "-T", timeStamp, logcatFilter, "*:S"))
                val bufferedReader = BufferedReader(InputStreamReader(process.inputStream))

                var line: String?
                while (bufferedReader.readLine().also { line = it } != null && logcatMonitorRunning.get()) {
                    if (line?.contains(packageName) == true && line?.contains("Denying clipboard access") == true) {
                        android.util.Log.i(TAG, "Detected clipboard denial, launching floating activity")
                        handler.post {
                            launchFloatingActivity()
                        }
                    }
                }

                process.destroy()
            } catch (e: Exception) {
                android.util.Log.e(TAG, "Logcat monitoring error: ${e.message}")
            } finally {
                logcatMonitorRunning.set(false)
            }
        }
    }

    private fun stopLogcatMonitoring() {
        logcatMonitorRunning.set(false)
    }

    private fun launchFloatingActivity() {
        try {
            val intent = ClipboardFloatingActivity.getIntent(context, showToast = false)
            context.startActivity(intent)
        } catch (e: Exception) {
            android.util.Log.e(TAG, "Failed to launch floating activity: ${e.message}")
        }
    }

    /**
     * Stop monitoring clipboard changes.
     */
    fun stopMonitoring() {
        if (!isMonitoring) return

        clipboardManager.removePrimaryClipChangedListener(clipboardListener)

        // Unregister broadcast receiver
        if (isAccessibilityReceiverRegistered) {
            try {
                context.unregisterReceiver(accessibilityClipboardReceiver)
            } catch (e: Exception) {
                // Ignore if not registered
            }
            isAccessibilityReceiverRegistered = false
        }

        // Disconnect from accessibility service
        NearClipAccessibilityService.getInstance()?.setClipboardCallback(null)

        // Stop logcat monitoring
        stopLogcatMonitoring()

        isMonitoring = false
        android.util.Log.i(TAG, "Clipboard monitoring stopped")
    }

    /**
     * Mark content as remote (from another device).
     * This prevents sync loops by adding the hash to recent hashes.
     */
    fun markAsRemote(content: ByteArray) {
        val hash = computeHash(content)
        synchronized(recentHashes) {
            recentHashes.add(hash)
            // Trim old hashes
            while (recentHashes.size > maxRecentHashes) {
                recentHashes.remove(recentHashes.first())
            }
        }
    }

    /**
     * Set flag to ignore the next clipboard change.
     * Call this before writing to clipboard.
     */
    fun ignoreNextChange() {
        ignoreNextChange = true
        // Reset after a short delay in case write doesn't trigger listener
        handler.postDelayed({
            ignoreNextChange = false
        }, 500)
    }

    private fun handleClipboardChange() {
        try {
            // Read clipboard content
            val content = readClipboardContent() ?: return

            // Check if this is content we recently synced (prevent loop)
            val hash = computeHash(content)
            synchronized(recentHashes) {
                if (recentHashes.contains(hash)) {
                    // This is content from a remote device, don't sync back
                    return
                }
            }

            // Trigger sync callback
            onClipboardChanged(content)

        } catch (e: SecurityException) {
            // Cannot read clipboard - app not in foreground on Android 10+
            // This is expected behavior
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    /**
     * Read current clipboard content.
     * Uses accessibility service on Android 10+ when available.
     * Returns null if clipboard is empty or cannot be read.
     */
    fun readClipboardContent(): ByteArray? {
        // First, try accessibility service (works in background on Android 10+)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            NearClipAccessibilityService.getInstance()?.readClipboardContent()?.let { content ->
                android.util.Log.i("ClipboardMonitor", "Read clipboard via accessibility service: ${content.size} bytes")
                return content
            }
        }

        // Fallback to standard clipboard access (works in foreground)
        return try {
            if (!clipboardManager.hasPrimaryClip()) {
                return null
            }

            val clip = clipboardManager.primaryClip ?: return null
            if (clip.itemCount == 0) return null

            val item = clip.getItemAt(0)

            // Try to get text content
            val text = item.coerceToText(context)
            if (text.isNotEmpty()) {
                return text.toString().toByteArray(Charsets.UTF_8)
            }

            null
        } catch (e: SecurityException) {
            // Cannot read clipboard on Android 10+ when not in foreground
            android.util.Log.w("ClipboardMonitor", "Cannot read clipboard (not in foreground). Enable accessibility service for background access.")
            null
        }
    }

    /**
     * Manually trigger a sync of current clipboard content.
     * Useful for "sync now" button in UI.
     */
    fun syncCurrentClipboard() {
        android.util.Log.i("ClipboardMonitor", "syncCurrentClipboard called (accessibility: ${isAccessibilityServiceEnabled()})")
        val content = readClipboardContent()
        android.util.Log.i("ClipboardMonitor", "readClipboardContent returned: ${content?.size ?: "null"} bytes")

        if (content == null) {
            android.util.Log.w("ClipboardMonitor", "Cannot read clipboard. Please enable accessibility service or bring app to foreground.")
            return
        }

        val hash = computeHash(content)
        synchronized(recentHashes) {
            if (!recentHashes.contains(hash)) {
                android.util.Log.i("ClipboardMonitor", "Calling onClipboardChanged with ${content.size} bytes")
                onClipboardChanged(content)
            } else {
                android.util.Log.i("ClipboardMonitor", "Content already in recentHashes, skipping")
            }
        }
    }

    private fun computeHash(content: ByteArray): String {
        val digest = MessageDigest.getInstance("SHA-256")
        val hashBytes = digest.digest(content)
        return hashBytes.joinToString("") { "%02x".format(it) }
    }

    fun isMonitoring(): Boolean = isMonitoring
}
