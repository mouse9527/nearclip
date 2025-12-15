package com.nearclip.service

import android.content.ClipboardManager
import android.content.Context
import android.os.Build
import android.os.Handler
import android.os.Looper
import java.security.MessageDigest

/**
 * Monitors clipboard changes and triggers sync.
 *
 * Note: On Android 10+, clipboard can only be read when:
 * - App is in foreground
 * - App has input focus
 * - App is the default input method editor
 *
 * This monitor works best when combined with a foreground service
 * and user interaction triggers.
 */
class ClipboardMonitor(
    private val context: Context,
    private val onClipboardChanged: (ByteArray) -> Unit
) {
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

    private var isMonitoring = false

    /**
     * Start monitoring clipboard changes.
     */
    fun startMonitoring() {
        if (isMonitoring) return

        clipboardManager.addPrimaryClipChangedListener(clipboardListener)
        isMonitoring = true
    }

    /**
     * Stop monitoring clipboard changes.
     */
    fun stopMonitoring() {
        if (!isMonitoring) return

        clipboardManager.removePrimaryClipChangedListener(clipboardListener)
        isMonitoring = false
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
     * Returns null if clipboard is empty or cannot be read.
     */
    fun readClipboardContent(): ByteArray? {
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
            null
        }
    }

    /**
     * Manually trigger a sync of current clipboard content.
     * Useful for "sync now" button in UI.
     */
    fun syncCurrentClipboard() {
        val content = readClipboardContent()
        if (content != null) {
            val hash = computeHash(content)
            synchronized(recentHashes) {
                if (!recentHashes.contains(hash)) {
                    onClipboardChanged(content)
                }
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
