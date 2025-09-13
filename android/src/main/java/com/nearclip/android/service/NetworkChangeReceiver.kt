package com.nearclip.android.service

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.net.wifi.WifiManager

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