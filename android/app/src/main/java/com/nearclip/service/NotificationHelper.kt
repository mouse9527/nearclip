package com.nearclip.service

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.os.Build
import androidx.core.app.NotificationCompat
import com.nearclip.MainActivity
import com.nearclip.data.settingsDataStore
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking

/**
 * Helper class for showing sync-related notifications.
 */
class NotificationHelper(private val context: Context) {

    companion object {
        const val SYNC_NOTIFICATION_CHANNEL_ID = "nearclip_sync_notifications"
        const val ACTION_RETRY_SYNC = "com.nearclip.action.RETRY_SYNC"
        const val ACTION_DISCARD_SYNC = "com.nearclip.action.DISCARD_SYNC"
        const val ACTION_WAIT_SYNC = "com.nearclip.action.WAIT_SYNC"
        private const val SYNC_SUCCESS_NOTIFICATION_ID = 1000
        private const val SYNC_FAILURE_NOTIFICATION_ID = 1001
    }

    private val notificationManager = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager

    init {
        createNotificationChannel()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                SYNC_NOTIFICATION_CHANNEL_ID,
                "Sync Notifications",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Notifications when clipboard is synced"
                setShowBadge(false)
                enableVibration(false)
                enableLights(false)
            }
            notificationManager.createNotificationChannel(channel)
        }
    }

    /**
     * Show a notification when clipboard sync succeeds.
     * @param fromDevice Name of the device that sent the clipboard content
     * @param contentPreview Optional preview of the content (first few characters)
     */
    fun showSyncSuccessNotification(fromDevice: String, contentPreview: String? = null) {
        // Check if sync notifications are enabled
        if (!isSyncNotificationsEnabled()) {
            return
        }

        // Intent to open app
        val openIntent = Intent(context, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP
        }
        val pendingIntent = PendingIntent.getActivity(
            context, 0, openIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val contentText = if (!contentPreview.isNullOrEmpty()) {
            val truncated = if (contentPreview.length > 50) {
                contentPreview.take(50) + "..."
            } else {
                contentPreview
            }
            "From $fromDevice: \"$truncated\""
        } else {
            "Synced from $fromDevice"
        }

        val notification = NotificationCompat.Builder(context, SYNC_NOTIFICATION_CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_menu_share)
            .setContentTitle("Clipboard Synced")
            .setContentText(contentText)
            .setAutoCancel(true)
            .setTimeoutAfter(3000) // Auto-dismiss after 3 seconds
            .setContentIntent(pendingIntent)
            .setPriority(NotificationCompat.PRIORITY_LOW)
            .setCategory(NotificationCompat.CATEGORY_STATUS)
            .build()

        // Use unique ID to allow multiple notifications
        val notificationId = SYNC_SUCCESS_NOTIFICATION_ID + (System.currentTimeMillis() % 100).toInt()
        notificationManager.notify(notificationId, notification)
    }

    /**
     * Show a notification when clipboard sync fails.
     * @param toDevice Name of the device we tried to sync to (optional)
     * @param reason The failure reason
     */
    fun showSyncFailureNotification(toDevice: String? = null, reason: String) {
        // Check if sync notifications are enabled
        if (!isSyncNotificationsEnabled()) {
            return
        }

        val openIntent = Intent(context, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP
        }
        val pendingIntent = PendingIntent.getActivity(
            context, 0, openIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        // Retry action - triggers sync of current clipboard
        val retryIntent = Intent(context, NearClipService::class.java).apply {
            action = ACTION_RETRY_SYNC
        }
        val retryPendingIntent = PendingIntent.getService(
            context, 1, retryIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        // Wait for device action
        val waitIntent = Intent(context, NearClipService::class.java).apply {
            action = ACTION_WAIT_SYNC
        }
        val waitPendingIntent = PendingIntent.getService(
            context, 2, waitIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        // Discard action
        val discardIntent = Intent(context, NearClipService::class.java).apply {
            action = ACTION_DISCARD_SYNC
        }
        val discardPendingIntent = PendingIntent.getService(
            context, 3, discardIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val contentText = if (toDevice != null) {
            "Failed to sync to $toDevice: $reason"
        } else {
            "Sync failed: $reason"
        }

        val notification = NotificationCompat.Builder(context, SYNC_NOTIFICATION_CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_dialog_alert)
            .setContentTitle("Sync Failed")
            .setContentText(contentText)
            .setAutoCancel(true)
            .setContentIntent(pendingIntent)
            .addAction(
                android.R.drawable.ic_menu_rotate,
                "Retry",
                retryPendingIntent
            )
            .addAction(
                android.R.drawable.ic_popup_sync,
                "Wait",
                waitPendingIntent
            )
            .addAction(
                android.R.drawable.ic_menu_close_clear_cancel,
                "Discard",
                discardPendingIntent
            )
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setCategory(NotificationCompat.CATEGORY_ERROR)
            .build()

        notificationManager.notify(SYNC_FAILURE_NOTIFICATION_ID, notification)
    }

    /**
     * Check if sync notifications are enabled in settings.
     */
    private fun isSyncNotificationsEnabled(): Boolean {
        return try {
            runBlocking {
                val preferences = context.settingsDataStore.data.first()
                preferences[androidx.datastore.preferences.core.booleanPreferencesKey("sync_notifications")] ?: true
            }
        } catch (e: Exception) {
            true // Default to enabled if reading fails
        }
    }
}
