package com.nearclip

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.nearclip.ui.navigation.NearClipNavigation
import com.nearclip.ui.theme.NearClipTheme
import com.nearclip.core.NearClipCore
import timber.log.Timber

/**
 * NearClip 主活动
 *
 * 应用程序的入口点，负责权限管理和导航设置
 */
class MainActivity : ComponentActivity() {

    private lateinit var nearClipCore: NearClipCore

    @OptIn(ExperimentalPermissionsApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        // 获取核心实例
        nearClipCore = (application as NearClipApplication).getCoreInstance()

        setContent {
            NearClipTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    // 权限检查
                    val bluetoothPermissionState = rememberPermissionState(
                        android.Manifest.permission.BLUETOOTH_SCAN
                    )
                    val locationPermissionState = rememberPermissionState(
                        android.Manifest.permission.ACCESS_FINE_LOCATION
                    )

                    // 如果权限已授予，显示主界面
                    if (bluetoothPermissionState.status.isGranted &&
                        locationPermissionState.status.isGranted) {
                        NearClipNavigation(
                            nearClipCore = nearClipCore,
                            onPermissionRequest = { /* 权限已授予 */ }
                        )
                    } else {
                        // 请求权限
                        PermissionRequestScreen(
                            onRequestBluetoothPermission = {
                                bluetoothPermissionState.launchPermissionRequest()
                            },
                            onRequestLocationPermission = {
                                locationPermissionState.launchPermissionRequest()
                            }
                        )
                    }
                }
            }
        }
    }

    override fun onResume() {
        super.onResume()

        try {
            // 启动 NearClip 核心服务
            nearClipCore.start()
            Timber.d("NearClip services started in MainActivity")
        } catch (e: Exception) {
            Timber.e(e, "Failed to start NearClip services")
        }
    }

    override fun onPause() {
        super.onPause()

        try {
            // 暂停某些服务以节省电池
            // 注意：不停止所有服务，因为可能需要后台同步
            nearClipCore.pauseBackgroundTasks()
            Timber.d("NearClip background tasks paused in MainActivity")
        } catch (e: Exception) {
            Timber.e(e, "Failed to pause NearClip services")
        }
    }

    override fun onDestroy() {
        super.onDestroy()

        try {
            // 清理资源
            nearClipCore.cleanup()
            Timber.d("NearClip resources cleaned up in MainActivity")
        } catch (e: Exception) {
            Timber.e(e, "Failed to cleanup NearClip services")
        }
    }
}