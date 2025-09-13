# Task 00108c: Android 权限管理

## 任务描述

实现Android平台的权限管理系统，处理蓝牙、WiFi、位置等运行时权限请求和状态管理。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class PermissionManagerTest {
    @Test
    fun testPermissionRequest() {
        // RED: 测试权限请求
        val manager = PermissionManager(mockActivity)
        
        runBlocking {
            val result = manager.requestRequiredPermissions()
            assertTrue(result is PermissionResult.Requested)
        }
    }

    @Test
    fun testPermissionStatusCheck() {
        // RED: 测试权限状态检查
        val manager = PermissionManager(mockActivity)
        
        assertFalse(manager.hasRequiredPermissions())
        assertFalse(manager.shouldShowBluetoothPermissionRationale())
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class PermissionManager(
    private val activity: Activity
) {
    private val requiredPermissions = arrayOf(
        Manifest.permission.BLUETOOTH_SCAN,
        Manifest.permission.BLUETOOTH_CONNECT,
        Manifest.permission.BLUETOOTH_ADVERTISE,
        Manifest.permission.ACCESS_FINE_LOCATION,
        Manifest.permission.ACCESS_WIFI_STATE,
        Manifest.permission.CHANGE_WIFI_STATE,
        Manifest.permission.INTERNET
    )
    
    suspend fun requestRequiredPermissions(): PermissionResult {
        val missingPermissions = getMissingPermissions()
        
        if (missingPermissions.isEmpty()) {
            return PermissionResult.Granted
        }
        
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            requestPermissionsCompat(missingPermissions)
        } else {
            PermissionResult.Granted // Pre-Marshmallow permissions granted at install
        }
    }
    
    fun hasRequiredPermissions(): Boolean {
        return getMissingPermissions().isEmpty()
    }
    
    fun shouldShowPermissionRationale(): Boolean {
        return requiredPermissions.any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    fun shouldShowBluetoothPermissionRationale(): Boolean {
        val bluetoothPermissions = arrayOf(
            Manifest.permission.BLUETOOTH_SCAN,
            Manifest.permission.BLUETOOTH_CONNECT,
            Manifest.permission.BLUETOOTH_ADVERTISE
        )
        return bluetoothPermissions.any { permission ->
            activity.shouldShowRequestPermissionRationale(permission)
        }
    }
    
    fun shouldShowLocationPermissionRationale(): Boolean {
        return activity.shouldShowRequestPermissionRationale(Manifest.permission.ACCESS_FINE_LOCATION)
    }
    
    private fun getMissingPermissions(): Array<String> {
        return requiredPermissions.filter { permission ->
            activity.checkSelfPermission(permission) != PackageManager.PERMISSION_GRANTED
        }.toTypedArray()
    }
    
    private suspend fun requestPermissionsCompat(permissions: Array<String>): PermissionResult {
        return suspendCancellableCoroutine { continuation ->
            val requestKey = UUID.randomUUID().toString()
            
            val callback = object : ActivityResultCallback<ActivityResult> {
                override fun onActivityResult(result: ActivityResult) {
                    if (result.resultCode == Activity.RESULT_OK) {
                        val grantResults = result.data?.getIntArrayExtra("grantResults") ?: IntArray(0)
                        val allGranted = grantResults.all { it == PackageManager.PERMISSION_GRANTED }
                        
                        if (allGranted) {
                            continuation.resume(PermissionResult.Granted)
                        } else {
                            continuation.resume(PermissionResult.Denied(getDeniedPermissions()))
                        }
                    } else {
                        continuation.resume(PermissionResult.Cancelled)
                    }
                    activity.unregisterActivityResultCallback(requestKey)
                }
            }
            
            activity.registerActivityResultCallback(requestKey, callback)
            
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                activity.requestPermissions(permissions, 0)
            }
        }
    }
    
    private fun getDeniedPermissions(): List<String> {
        return requiredPermissions.filter { permission ->
            activity.checkSelfPermission(permission) != PackageManager.PERMISSION_GRANTED
        }
    }
}

sealed class PermissionResult {
    object Granted : PermissionResult()
    data class Requested(val requestCode: Int) : PermissionResult()
    data class Denied(val deniedPermissions: List<String>) : PermissionResult()
    object Cancelled : PermissionResult()
}

class PermissionRequestHandler : ActivityResultCaller {
    private var permissionCallback: ((Boolean, Map<String, Boolean>) -> Unit)? = null
    
    fun requestPermissions(
        permissions: Array<String>,
        callback: (granted: Boolean, results: Map<String, Boolean>) -> Unit
    ) {
        permissionCallback = callback
        // 实际权限请求逻辑
    }
    
    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        val results = permissions.zip(grantResults.map { it == PackageManager.PERMISSION_GRANTED }).toMap()
        val allGranted = results.values.all { it }
        permissionCallback?.invoke(allGranted, results)
    }
}
```

### REFACTOR阶段
```kotlin
class PermissionManager(
    private val activity: Activity,
    private val permissionConfig: PermissionConfig = PermissionConfig.default()
) {
    // 添加权限组管理
    // 添加权限状态持久化
    // 添加权限请求防重复逻辑
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
        }
        
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
        return arrayOf(Manifest.permission.ACCESS_FINE_LOCATION)
    }
    
    fun getWiFiPermissions(): Array<String> {
        return arrayOf(
            Manifest.permission.ACCESS_WIFI_STATE,
            Manifest.permission.CHANGE_WIFI_STATE
        )
    }
}
```

## 验收标准
- [ ] 权限状态检查正确
- [ ] 权限请求功能正常
- [ ] 支持权限合理性说明
- [ ] 适配不同Android版本
- [ ] 错误处理机制完善

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108b: Android WiFi 设备发现核心](00108b-android-wifi-discovery-core.md)

## 后续任务
- [Task 00108d: Android 电池优化](00108d-android-battery-optimization.md)