package com.mouse.nearclip

import android.app.Activity
import android.content.pm.PackageManager
import android.Manifest
import android.os.Build
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

class PermissionManager(
    private val activity: Activity,
    private val permissionConfig: PermissionConfig = PermissionConfig.default()
) {
    private val permissionGroupManager = PermissionGroupManager()
    
    suspend fun requestRequiredPermissions(): PermissionResult {
        val missingPermissions = getMissingPermissions()
        
        if (missingPermissions.isEmpty()) {
            return PermissionResult.Granted
        }
        
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            PermissionResult.Requested(0)
        } else {
            PermissionResult.Granted // Pre-Marshmallow permissions granted at install
        }
    }
    
    fun hasRequiredPermissions(): Boolean {
        return getMissingPermissions().isEmpty()
    }
    
    fun shouldShowPermissionRationale(): Boolean {
        return getAllRequiredPermissions().any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    fun shouldShowBluetoothPermissionRationale(): Boolean {
        return permissionGroupManager.getBluetoothPermissions().any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    fun shouldShowLocationPermissionRationale(): Boolean {
        return permissionGroupManager.getLocationPermissions().any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    fun shouldShowWiFiPermissionRationale(): Boolean {
        return permissionGroupManager.getWiFiPermissions().any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    private fun getAllRequiredPermissions(): Array<String> {
        val permissions = mutableListOf<String>()
        
        if (permissionConfig.enableBluetooth) {
            permissions.addAll(permissionGroupManager.getBluetoothPermissions())
        }
        
        if (permissionConfig.enableLocation) {
            permissions.addAll(permissionGroupManager.getLocationPermissions())
        }
        
        if (permissionConfig.enableWiFi) {
            permissions.addAll(permissionGroupManager.getWiFiPermissions())
        }
        
        if (permissionConfig.enableNetwork) {
            permissions.addAll(permissionGroupManager.getNetworkPermissions())
        }
        
        return permissions.toTypedArray()
    }
    
    private fun getMissingPermissions(): Array<String> {
        return getAllRequiredPermissions().filter { permission ->
            activity.checkSelfPermission(permission) != PackageManager.PERMISSION_GRANTED
        }.toTypedArray()
    }
    
    // 添加权限状态持久化方法
    fun getPermissionStatusMap(): Map<String, Boolean> {
        return getAllRequiredPermissions().associateWith { permission ->
            activity.checkSelfPermission(permission) == PackageManager.PERMISSION_GRANTED
        }
    }
    
    // 添加防重复权限请求逻辑
    private var lastRequestTime = 0L
    private val requestCooldown = 1000L // 1 second cooldown
    
    fun canRequestPermissions(): Boolean {
        return System.currentTimeMillis() - lastRequestTime >= requestCooldown
    }
    
    fun markPermissionRequestTime() {
        lastRequestTime = System.currentTimeMillis()
    }
}

data class PermissionConfig(
    val enableBluetooth: Boolean,
    val enableLocation: Boolean,
    val enableWiFi: Boolean,
    val enableNetwork: Boolean,
    val showRationaleDialog: Boolean
) {
    companion object {
        fun default() = PermissionConfig(
            enableBluetooth = true,
            enableLocation = true,
            enableWiFi = true,
            enableNetwork = true,
            showRationaleDialog = true
        )
        
        fun bluetoothOnly() = PermissionConfig(
            enableBluetooth = true,
            enableLocation = false,
            enableWiFi = false,
            enableNetwork = false,
            showRationaleDialog = true
        )
        
        fun networkOnly() = PermissionConfig(
            enableBluetooth = false,
            enableLocation = false,
            enableWiFi = true,
            enableNetwork = true,
            showRationaleDialog = true
        )
    }
}

class PermissionGroupManager {
    fun getBluetoothPermissions(): Array<String> {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.BLUETOOTH_ADVERTISE
            )
        } else {
            arrayOf(Manifest.permission.BLUETOOTH, Manifest.permission.BLUETOOTH_ADMIN)
        }
    }
    
    fun getLocationPermissions(): Array<String> {
        return arrayOf(
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.ACCESS_COARSE_LOCATION
        )
    }
    
    fun getWiFiPermissions(): Array<String> {
        return arrayOf(
            Manifest.permission.ACCESS_WIFI_STATE,
            Manifest.permission.CHANGE_WIFI_STATE,
            Manifest.permission.CHANGE_NETWORK_STATE
        )
    }
    
    fun getNetworkPermissions(): Array<String> {
        return arrayOf(
            Manifest.permission.INTERNET,
            Manifest.permission.ACCESS_NETWORK_STATE
        )
    }
}

sealed class PermissionResult {
    object Granted : PermissionResult()
    data class Requested(val requestCode: Int) : PermissionResult()
    data class Denied(val deniedPermissions: List<String>) : PermissionResult()
    object Cancelled : PermissionResult()
}