package com.nearclip.service

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.os.Handler
import android.os.Looper

/**
 * Writes content to the system clipboard.
 *
 * Works in conjunction with ClipboardMonitor to prevent sync loops
 * by marking content as remote before writing.
 */
class ClipboardWriter(
    private val context: Context,
    private val clipboardMonitor: ClipboardMonitor?
) {
    private val clipboardManager = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
    private val handler = Handler(Looper.getMainLooper())

    /**
     * Write text content to clipboard.
     * Marks the content as remote to prevent sync loop.
     *
     * @param content The content to write
     * @param label Optional label for the clipboard data
     */
    fun writeText(content: ByteArray, label: String = "NearClip") {
        // Must run on main thread
        handler.post {
            try {
                // Mark as remote before writing to prevent sync loop
                clipboardMonitor?.markAsRemote(content)
                clipboardMonitor?.ignoreNextChange()

                // Convert bytes to string
                val text = content.toString(Charsets.UTF_8)

                // Create clip data and set to clipboard
                val clip = ClipData.newPlainText(label, text)
                clipboardManager.setPrimaryClip(clip)

            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    /**
     * Write text content to clipboard synchronously.
     * Must be called from main thread.
     */
    fun writeTextSync(content: ByteArray, label: String = "NearClip") {
        try {
            clipboardMonitor?.markAsRemote(content)
            clipboardMonitor?.ignoreNextChange()

            val text = content.toString(Charsets.UTF_8)
            val clip = ClipData.newPlainText(label, text)
            clipboardManager.setPrimaryClip(clip)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
