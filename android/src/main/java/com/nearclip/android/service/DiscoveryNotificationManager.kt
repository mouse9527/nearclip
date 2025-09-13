package com.nearclip.android.service

import android.app.Notification
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import androidx.core.app.NotificationCompat

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
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentIntent(pendingIntent)
            .addAction(
                android.R.drawable.ic_menu_close_clear_cancel,
                "停止",
                stopPendingIntent
            )
            .setOngoing(isDiscovering)
            .setOnlyAlertOnce(true)
            .build()
    }
}