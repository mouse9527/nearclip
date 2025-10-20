package com.nearclip.services.ble

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.fragment.app.Fragment

/**
 * BLE权限管理器
 * 负责处理Android 12+的新蓝牙权限模型
 */
class BlePermissionManager(private val context: Context) {

    companion object {
        private const val TAG = "BlePermissionManager"
        const val REQUEST_BLUETOOTH_PERMISSIONS = 1001
        const val REQUEST_LOCATION_PERMISSIONS = 1002

        // Android 12+ 新的蓝牙权限
        val BLUETOOTH_PERMISSIONS = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.BLUETOOTH_ADVERTISE
            )
        } else {
            arrayOf(
                Manifest.permission.BLUETOOTH,
                Manifest.permission.BLUETOOTH_ADMIN,
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        }

        // 位置权限（Android 10+需要用于BLE扫描）
        val LOCATION_PERMISSIONS = arrayOf(
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.ACCESS_COARSE_LOCATION
        )
    }

    /**
     * 检查是否拥有所有BLE权限
     */
    fun hasAllBluetoothPermissions(): Boolean {
        return BLUETOOTH_PERMISSIONS.all { permission ->
            ContextCompat.checkSelfPermission(context, permission) == PackageManager.PERMISSION_GRANTED
        }
    }

    /**
     * 检查是否拥有位置权限
     */
    fun hasLocationPermissions(): Boolean {
        return LOCATION_PERMISSIONS.all { permission ->
            ContextCompat.checkSelfPermission(context, permission) == PackageManager.PERMISSION_GRANTED
        }
    }

    /**
     * 检查是否有扫描权限
     */
    fun hasScanPermission(): Boolean {
        val scanPermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            Manifest.permission.BLUETOOTH_SCAN
        } else {
            Manifest.permission.ACCESS_FINE_LOCATION
        }
        return ContextCompat.checkSelfPermission(context, scanPermission) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查是否有连接权限
     */
    fun hasConnectPermission(): Boolean {
        val connectPermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            Manifest.permission.BLUETOOTH_CONNECT
        } else {
            Manifest.permission.BLUETOOTH
        }
        return ContextCompat.checkSelfPermission(context, connectPermission) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查是否有广播权限
     */
    fun hasAdvertisePermission(): Boolean {
        val advertisePermission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            Manifest.permission.BLUETOOTH_ADVERTISE
        } else {
            Manifest.permission.BLUETOOTH_ADMIN
        }
        return ContextCompat.checkSelfPermission(context, advertisePermission) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 获取缺少的权限列表
     */
    fun getMissingPermissions(): List<String> {
        val missingPermissions = mutableListOf<String>()

        // 检查蓝牙权限
        BLUETOOTH_PERMISSIONS.forEach { permission ->
            if (ContextCompat.checkSelfPermission(context, permission) != PackageManager.PERMISSION_GRANTED) {
                missingPermissions.add(permission)
            }
        }

        // 检查位置权限
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            LOCATION_PERMISSIONS.forEach { permission ->
                if (ContextCompat.checkSelfPermission(context, permission) != PackageManager.PERMISSION_GRANTED) {
                    missingPermissions.add(permission)
                }
            }
        }

        return missingPermissions
    }

    /**
     * 从Activity请求蓝牙权限
     */
    fun requestBluetoothPermissions(activity: Activity) {
        val missingPermissions = getMissingPermissions().filter { permission ->
            BLUETOOTH_PERMISSIONS.contains(permission)
        }.toTypedArray()

        if (missingPermissions.isNotEmpty()) {
            ActivityCompat.requestPermissions(
                activity,
                missingPermissions,
                REQUEST_BLUETOOTH_PERMISSIONS
            )
        }
    }

    /**
     * 从Fragment请求蓝牙权限
     */
    fun requestBluetoothPermissions(fragment: Fragment) {
        val missingPermissions = getMissingPermissions().filter { permission ->
            BLUETOOTH_PERMISSIONS.contains(permission)
        }.toTypedArray()

        if (missingPermissions.isNotEmpty()) {
            fragment.requestPermissions(
                missingPermissions,
                REQUEST_BLUETOOTH_PERMISSIONS
            )
        }
    }

    /**
     * 从Activity请求位置权限
     */
    fun requestLocationPermissions(activity: Activity) {
        val missingLocationPermissions = LOCATION_PERMISSIONS.filter { permission ->
            ContextCompat.checkSelfPermission(context, permission) != PackageManager.PERMISSION_GRANTED
        }.toTypedArray()

        if (missingLocationPermissions.isNotEmpty()) {
            ActivityCompat.requestPermissions(
                activity,
                missingLocationPermissions,
                REQUEST_LOCATION_PERMISSIONS
            )
        }
    }

    /**
     * 从Fragment请求位置权限
     */
    fun requestLocationPermissions(fragment: Fragment) {
        val missingLocationPermissions = LOCATION_PERMISSIONS.filter { permission ->
            ContextCompat.checkSelfPermission(context, permission) != PackageManager.PERMISSION_GRANTED
        }.toTypedArray()

        if (missingLocationPermissions.isNotEmpty()) {
            fragment.requestPermissions(
                missingLocationPermissions,
                REQUEST_LOCATION_PERMISSIONS
            )
        }
    }

    /**
     * 请求所有必需的权限
     */
    fun requestAllPermissions(activity: Activity) {
        val missingPermissions = getMissingPermissions().toTypedArray()

        if (missingPermissions.isNotEmpty()) {
            ActivityCompat.requestPermissions(
                activity,
                missingPermissions,
                REQUEST_BLUETOOTH_PERMISSIONS
            )
        }
    }

    /**
     * 请求所有必需的权限（Fragment版本）
     */
    fun requestAllPermissions(fragment: Fragment) {
        val missingPermissions = getMissingPermissions().toTypedArray()

        if (missingPermissions.isNotEmpty()) {
            fragment.requestPermissions(
                missingPermissions,
                REQUEST_BLUETOOTH_PERMISSIONS
            )
        }
    }

    /**
     * 检查权限请求结果
     */
    fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
        onAllGranted: () -> Unit,
        onSomeDenied: (deniedPermissions: List<String>) -> Unit
    ) {
        when (requestCode) {
            REQUEST_BLUETOOTH_PERMISSIONS,
            REQUEST_LOCATION_PERMISSIONS -> {
                val grantedPermissions = mutableListOf<String>()
                val deniedPermissions = mutableListOf<String>()

                permissions.forEachIndexed { index, permission ->
                    if (index < grantResults.size && grantResults[index] == PackageManager.PERMISSION_GRANTED) {
                        grantedPermissions.add(permission)
                    } else {
                        deniedPermissions.add(permission)
                    }
                }

                if (deniedPermissions.isEmpty()) {
                    onAllGranted()
                } else {
                    onSomeDenied(deniedPermissions)
                }
            }
        }
    }

    /**
     * 检查是否应该显示权限说明
     */
    fun shouldShowRequestPermissionRationale(activity: Activity, permission: String): Boolean {
        return ActivityCompat.shouldShowRequestPermissionRationale(activity, permission)
    }

    /**
     * 检查是否应该显示权限说明（Fragment版本）
     */
    fun shouldShowRequestPermissionRationale(fragment: Fragment, permission: String): Boolean {
        return fragment.shouldShowRequestPermissionRationale(permission)
    }

    /**
     * 获取权限说明文本
     */
    fun getPermissionRationaleText(permission: String): String {
        return when (permission) {
            Manifest.permission.BLUETOOTH_SCAN ->
                "NearClip需要蓝牙扫描权限来发现附近的设备"
            Manifest.permission.BLUETOOTH_CONNECT ->
                "NearClip需要蓝牙连接权限来与其他设备建立连接"
            Manifest.permission.BLUETOOTH_ADVERTISE ->
                "NearClip需要蓝牙广播权限来让其他设备发现本设备"
            Manifest.permission.ACCESS_FINE_LOCATION ->
                "NearClip需要位置权限来进行BLE设备扫描（系统要求）"
            Manifest.permission.ACCESS_COARSE_LOCATION ->
                "NearClip需要位置权限来进行BLE设备扫描（系统要求）"
            Manifest.permission.BLUETOOTH ->
                "NearClip需要蓝牙权限来使用蓝牙功能"
            Manifest.permission.BLUETOOTH_ADMIN ->
                "NearClip需要蓝牙管理权限来控制蓝牙设置"
            else -> "NearClip需要此权限来正常工作"
        }
    }

    /**
     * 获取所有权限的说明文本
     */
    fun getAllPermissionsRationaleText(): String {
        val rationales = mutableListOf<String>()

        if (!hasScanPermission()) {
            rationales.add(getPermissionRationaleText(
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    Manifest.permission.BLUETOOTH_SCAN
                } else {
                    Manifest.permission.ACCESS_FINE_LOCATION
                }
            ))
        }

        if (!hasConnectPermission()) {
            rationales.add(getPermissionRationaleText(
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    Manifest.permission.BLUETOOTH_CONNECT
                } else {
                    Manifest.permission.BLUETOOTH
                }
            ))
        }

        if (!hasAdvertisePermission()) {
            rationales.add(getPermissionRationaleText(
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    Manifest.permission.BLUETOOTH_ADVERTISE
                } else {
                    Manifest.permission.BLUETOOTH_ADMIN
                }
            ))
        }

        return rationales.joinToString("\n\n")
    }
}