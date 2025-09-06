package com.nearclip.android.permissions

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.content.ContextCompat
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class PermissionManager @Inject constructor(
    @ApplicationContext private val context: Context
) {
    
    /**
     * 获取所需的蓝牙权限列表
     */
    fun getRequiredBluetoothPermissions(): List<String> {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            // Android 12+ 需要新的蓝牙权限
            listOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_ADVERTISE,
                Manifest.permission.BLUETOOTH_CONNECT
            )
        } else {
            // Android 11 及以下版本需要位置权限
            listOf(
                Manifest.permission.BLUETOOTH,
                Manifest.permission.BLUETOOTH_ADMIN,
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        }
    }
    
    /**
     * 检查是否已授予所有必需的蓝牙权限
     */
    fun hasAllBluetoothPermissions(): Boolean {
        return getRequiredBluetoothPermissions().all { permission ->
            ContextCompat.checkSelfPermission(context, permission) == PackageManager.PERMISSION_GRANTED
        }
    }
    
    /**
     * 获取缺失的权限列表
     */
    fun getMissingBluetoothPermissions(): List<String> {
        return getRequiredBluetoothPermissions().filter { permission ->
            ContextCompat.checkSelfPermission(context, permission) != PackageManager.PERMISSION_GRANTED
        }
    }
    
    /**
     * 检查蓝牙是否已启用
     */
    fun isBluetoothEnabled(): Boolean {
        val bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
        return bluetoothAdapter?.isEnabled == true
    }
    
    /**
     * 检查设备是否支持 BLE
     */
    fun isBleSupported(): Boolean {
        return context.packageManager.hasSystemFeature(PackageManager.FEATURE_BLUETOOTH_LE)
    }
    
    /**
     * 获取权限状态描述
     */
    fun getPermissionStatusDescription(): PermissionStatus {
        return when {
            !isBleSupported() -> PermissionStatus.BLE_NOT_SUPPORTED
            !isBluetoothEnabled() -> PermissionStatus.BLUETOOTH_DISABLED
            !hasAllBluetoothPermissions() -> PermissionStatus.PERMISSIONS_MISSING
            else -> PermissionStatus.ALL_GRANTED
        }
    }
}

/**
 * 权限状态枚举
 */
enum class PermissionStatus {
    ALL_GRANTED,
    PERMISSIONS_MISSING,
    BLUETOOTH_DISABLED,
    BLE_NOT_SUPPORTED;
    
    fun getDescription(): String {
        return when (this) {
            ALL_GRANTED -> "所有权限已授予"
            PERMISSIONS_MISSING -> "缺少蓝牙权限"
            BLUETOOTH_DISABLED -> "蓝牙未启用"
            BLE_NOT_SUPPORTED -> "设备不支持 BLE"
        }
    }
    
    fun isReady(): Boolean {
        return this == ALL_GRANTED
    }
}