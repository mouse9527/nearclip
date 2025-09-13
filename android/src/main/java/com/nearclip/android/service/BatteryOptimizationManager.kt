package com.nearclip.android.service

import android.content.Context
import android.os.PowerManager

class BatteryOptimizationManager(private val context: Context) {

    fun shouldEnableBackgroundDiscovery(): Boolean {
        val powerManager = context.getSystemService(Context.POWER_SERVICE) as PowerManager

        // 检查是否在省电模式
        if (powerManager.isPowerSaveMode) {
            return false
        }

        // 检查设备是否在充电
        if (!powerManager.isInteractive) {
            // 屏幕关闭时降低频率
            return true
        }

        return true
    }

    fun isBatteryOptimizationEnabled(): Boolean {
        val powerManager = context.getSystemService(Context.POWER_SERVICE) as PowerManager
        return powerManager.isIgnoringBatteryOptimizations(context.packageName)
    }
}