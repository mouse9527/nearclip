package com.nearclip.android.ble

import android.os.Parcelable
import kotlinx.parcelize.Parcelize

/**
 * 表示一个 NearClip 设备的数据类
 */
@Parcelize
data class NearClipDevice(
    val address: String,
    val name: String,
    val rssi: Int,
    val isConnectable: Boolean,
    val lastSeen: Long = System.currentTimeMillis()
) : Parcelable {
    
    /**
     * 获取信号强度描述
     */
    fun getSignalStrengthDescription(): String {
        return when {
            rssi >= -50 -> "很强"
            rssi >= -70 -> "强"
            rssi >= -80 -> "中等"
            rssi >= -90 -> "弱"
            else -> "很弱"
        }
    }
    
    /**
     * 获取信号强度等级 (1-5)
     */
    fun getSignalStrengthLevel(): Int {
        return when {
            rssi >= -50 -> 5
            rssi >= -70 -> 4
            rssi >= -80 -> 3
            rssi >= -90 -> 2
            else -> 1
        }
    }
    
    /**
     * 检查设备是否在线（最近 30 秒内有信号）
     */
    fun isOnline(): Boolean {
        return System.currentTimeMillis() - lastSeen < 30_000
    }
}