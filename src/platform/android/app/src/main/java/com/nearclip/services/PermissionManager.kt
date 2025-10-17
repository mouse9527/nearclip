package com.nearclip.services

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.fragment.app.Fragment
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import javax.inject.Inject
import javax.inject.Singleton

/**
 * 权限管理器
 * 负责检查和请求应用所需的所有权限
 */
@Singleton
class PermissionManager @Inject constructor(
    private val context: Context
) {
    private val _permissionStates = MutableStateFlow<Map<String, Boolean>>(emptyMap())
    val permissionStates: StateFlow<Map<String, Boolean>> = _permissionStates.asStateFlow()

    companion object {
        // 蓝牙权限
        private val BLUETOOTH_PERMISSIONS = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.ACCESS_COARSE_LOCATION
            )
        } else {
            arrayOf(
                Manifest.permission.BLUETOOTH,
                Manifest.permission.BLUETOOTH_ADMIN,
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.ACCESS_COARSE_LOCATION
            )
        }

        // 剪贴板权限
        private val CLIPBOARD_PERMISSIONS = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            arrayOf(
                Manifest.permission.READ_CLIPBOARD,
                Manifest.permission.WRITE_CLIPBOARD
            )
        } else {
            emptyArray()
        }

        // 通知权限
        private val NOTIFICATION_PERMISSION = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            Manifest.permission.POST_NOTIFICATIONS
        } else {
            null
        }

        // 网络权限 (正常权限，无需请求)
        private val NETWORK_PERMISSIONS = arrayOf(
            Manifest.permission.INTERNET,
            Manifest.permission.ACCESS_NETWORK_STATE
        )

        val ALL_PERMISSIONS = BLUETOOTH_PERMISSIONS + CLIPBOARD_PERMISSIONS + NETWORK_PERMISSIONS +
                (NOTIFICATION_PERMISSION?.let { arrayOf(it) } ?: emptyArray())
    }

    /**
     * 检查特定权限是否已授予
     */
    fun isPermissionGranted(permission: String): Boolean {
        return ContextCompat.checkSelfPermission(context, permission) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查所有权限是否已授予
     */
    fun areAllPermissionsGranted(): Boolean {
        return ALL_PERMISSIONS.all { isPermissionGranted(it) }
    }

    /**
     * 检查蓝牙权限是否已授予
     */
    fun areBluetoothPermissionsGranted(): Boolean {
        return BLUETOOTH_PERMISSIONS.all { isPermissionGranted(it) }
    }

    /**
     * 检查剪贴板权限是否已授予
     */
    fun areClipboardPermissionsGranted(): Boolean {
        return if (CLIPBOARD_PERMISSIONS.isNotEmpty()) {
            CLIPBOARD_PERMISSIONS.all { isPermissionGranted(it) }
        } else {
            true // Android 12 以下不需要剪贴板权限
        }
    }

    /**
     * 检查通知权限是否已授予
     */
    fun isNotificationPermissionGranted(): Boolean {
        return NOTIFICATION_PERMISSION?.let { isPermissionGranted(it) } ?: true
    }

    /**
     * 获取需要请求的权限列表
     */
    fun getDeniedPermissions(): Array<String> {
        return ALL_PERMISSIONS.filter { !isPermissionGranted(it) }.toTypedArray()
    }

    /**
     * 更新权限状态
     */
    fun updatePermissionStates() {
        val states = ALL_PERMISSIONS.associateWith { isPermissionGranted(it) }
        _permissionStates.value = states
    }

    /**
     * 创建权限请求启动器 (用于Activity)
     */
    fun createPermissionLauncher(activity: Activity): ActivityResultLauncher<Array<String>> {
        return activity.registerForActivityResult(
            ActivityResultContracts.RequestMultiplePermissions()
        ) { permissions ->
            updatePermissionStates()
        }
    }

    /**
     * 创建权限请求启动器 (用于Fragment)
     */
    fun createPermissionLauncher(fragment: Fragment): ActivityResultLauncher<Array<String>> {
        return fragment.registerForActivityResult(
            ActivityResultContracts.RequestMultiplePermissions()
        ) { permissions ->
            updatePermissionStates()
        }
    }

    /**
     * 检查是否需要显示权限说明
     */
    fun shouldShowPermissionRationale(activity: Activity, permission: String): Boolean {
        return ActivityCompat.shouldShowRequestPermissionRationale(activity, permission)
    }

    /**
     * 获取权限的显示名称
     */
    fun getPermissionDisplayName(permission: String): String {
        return when (permission) {
            Manifest.permission.BLUETOOTH -> "蓝牙"
            Manifest.permission.BLUETOOTH_ADMIN -> "蓝牙管理"
            Manifest.permission.BLUETOOTH_SCAN -> "蓝牙扫描"
            Manifest.permission.BLUETOOTH_CONNECT -> "蓝牙连接"
            Manifest.permission.ACCESS_FINE_LOCATION -> "精确位置"
            Manifest.permission.ACCESS_COARSE_LOCATION -> "大致位置"
            Manifest.permission.READ_CLIPBOARD -> "读取剪贴板"
            Manifest.permission.WRITE_CLIPBOARD -> "写入剪贴板"
            Manifest.permission.POST_NOTIFICATIONS -> "通知"
            Manifest.permission.INTERNET -> "网络访问"
            Manifest.permission.ACCESS_NETWORK_STATE -> "网络状态"
            else -> permission
        }
    }

    init {
        updatePermissionStates()
    }
}