package com.nearclip.services

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.provider.Settings
import androidx.appcompat.app.AlertDialog
import androidx.fragment.app.Fragment
import kotlinx.coroutines.suspendCancellableCoroutine
import javax.inject.Inject
import javax.inject.Singleton

/**
 * 权限请求处理器
 * 处理权限请求逻辑、显示说明对话框和引导用户到设置页面
 */
@Singleton
class PermissionRequestHandler @Inject constructor(
    private val permissionManager: PermissionManager
) {

    /**
     * 请求所有必需权限
     * @param caller Activity或Fragment实例
     * @return 是否所有权限都已授予
     */
    suspend fun requestAllPermissions(caller: Any): Boolean {
        return if (permissionManager.areAllPermissionsGranted()) {
            true
        } else {
            when (caller) {
                is Activity -> requestPermissionsFromActivity(caller)
                is Fragment -> requestPermissionsFromFragment(caller)
                else -> throw IllegalArgumentException("Caller must be Activity or Fragment")
            }
        }
    }

    /**
     * 请求蓝牙权限
     */
    suspend fun requestBluetoothPermissions(caller: Any): Boolean {
        return if (permissionManager.areBluetoothPermissionsGranted()) {
            true
        } else {
            when (caller) {
                is Activity -> requestPermissionsFromActivity(caller, permissionManager.BLUETOOTH_PERMISSIONS)
                is Fragment -> requestPermissionsFromFragment(caller, permissionManager.BLUETOOTH_PERMISSIONS)
                else -> throw IllegalArgumentException("Caller must be Activity or Fragment")
            }
        }
    }

    /**
     * 请求剪贴板权限
     */
    suspend fun requestClipboardPermissions(caller: Any): Boolean {
        return if (permissionManager.areClipboardPermissionsGranted()) {
            true
        } else {
            when (caller) {
                is Activity -> requestPermissionsFromActivity(caller, permissionManager.CLIPBOARD_PERMISSIONS)
                is Fragment -> requestPermissionsFromFragment(caller, permissionManager.CLIPBOARD_PERMISSIONS)
                else -> throw IllegalArgumentException("Caller must be Activity or Fragment")
            }
        }
    }

    /**
     * 从Activity请求权限
     */
    private suspend fun requestPermissionsFromActivity(
        activity: Activity,
        permissions: Array<String> = permissionManager.getDeniedPermissions()
    ): Boolean = suspendCancellableCoroutine { continuation ->

        // 检查是否需要显示权限说明
        val shouldShowRationale = permissions.any {
            permissionManager.shouldShowPermissionRationale(activity, it)
        }

        if (shouldShowRationale) {
            showPermissionRationaleDialog(activity, permissions) { shouldRequest ->
                if (shouldRequest) {
                    requestPermissions(activity, permissions) { granted ->
                        continuation.resume(granted)
                    }
                } else {
                    continuation.resume(false)
                }
            }
        } else {
            requestPermissions(activity, permissions) { granted ->
                continuation.resume(granted)
            }
        }
    }

    /**
     * 从Fragment请求权限
     */
    private suspend fun requestPermissionsFromFragment(
        fragment: Fragment,
        permissions: Array<String> = permissionManager.getDeniedPermissions()
    ): Boolean = suspendCancellableCoroutine { continuation ->

        val shouldShowRationale = permissions.any {
            permissionManager.shouldShowPermissionRationale(fragment.requireActivity(), it)
        }

        if (shouldShowRationale) {
            showPermissionRationaleDialog(fragment.requireContext(), permissions) { shouldRequest ->
                if (shouldRequest) {
                    requestPermissions(fragment, permissions) { granted ->
                        continuation.resume(granted)
                    }
                } else {
                    continuation.resume(false)
                }
            }
        } else {
            requestPermissions(fragment, permissions) { granted ->
                continuation.resume(granted)
            }
        }
    }

    /**
     * 显示权限说明对话框
     */
    private fun showPermissionRationaleDialog(
        context: Context,
        permissions: Array<String>,
        onResult: (Boolean) -> Unit
    ) {
        val permissionNames = permissions.map { permissionManager.getPermissionDisplayName(it) }
        val message = "NearClip 需要以下权限来正常工作：\n\n${permissionNames.joinToString("\n")}\n\n请在设置中授予权限以继续使用应用。"

        AlertDialog.Builder(context)
            .setTitle("权限请求")
            .setMessage(message)
            .setPositiveButton("授予权限") { _, _ ->
                onResult(true)
            }
            .setNegativeButton("取消") { _, _ ->
                onResult(false)
            }
            .setCancelable(false)
            .show()
    }

    /**
     * 显示权限被拒绝的对话框
     */
    fun showPermissionDeniedDialog(context: Context, onResult: (Boolean) -> Unit) {
        AlertDialog.Builder(context)
            .setTitle("权限被拒绝")
            .setMessage("某些权限被拒绝，NearClip可能无法正常工作。\n\n您可以在设置中重新授予权限。")
            .setPositiveButton("去设置") { _, _ ->
                openAppSettings(context)
                onResult(true)
            }
            .setNegativeButton("稍后再说") { _, _ ->
                onResult(false)
            }
            .setCancelable(false)
            .show()
    }

    /**
     * 打开应用设置页面
     */
    private fun openAppSettings(context: Context) {
        val intent = Intent(Settings.ACTION_APPLICATION_DETAILS_SETTINGS).apply {
            data = Uri.fromParts("package", context.packageName, null)
        }
        context.startActivity(intent)
    }

    /**
     * 实际请求权限的方法
     */
    private fun requestPermissions(
        activity: Activity,
        permissions: Array<String>,
        onResult: (Boolean) -> Unit
    ) {
        val launcher = permissionManager.createPermissionLauncher(activity)
        launcher.launch(permissions)
    }

    private fun requestPermissions(
        fragment: Fragment,
        permissions: Array<String>,
        onResult: (Boolean) -> Unit
    ) {
        val launcher = permissionManager.createPermissionLauncher(fragment)
        launcher.launch(permissions)
    }
}