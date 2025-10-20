package com.nearclip.services.ble

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.le.*
import android.content.Context
import android.content.pm.PackageManager
import android.os.ParcelUuid
import android.util.Log
import androidx.core.content.ContextCompat
import java.nio.charset.StandardCharsets
import java.util.*

/**
 * BLE广播器
 * 负责广播设备信息供其他设备发现
 */
class BleAdvertiser(private val context: Context) {

    companion object {
        private const val TAG = "BleAdvertiser"
        private val SERVICE_UUID = ParcelUuid.fromString("0000FE2C-0000-1000-8000-00805F9B34FB")
        private val ADVERTISING_INTERVAL = AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY
        private val TX_POWER_LEVEL = AdvertiseSettings.ADVERTISE_TX_POWER_HIGH
    }

    private val bluetoothAdapter = android.bluetooth.BluetoothAdapter.getDefaultAdapter()
    private val bluetoothLeAdvertiser: BluetoothLeAdvertiser? = bluetoothAdapter?.bluetoothLeAdvertiser
    private var isAdvertising = false
    private var advertiseCallback: AdvertiseCallback? = null

    /**
     * 检查BLE权限
     */
    fun hasRequiredPermissions(): Boolean {
        return ContextCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_ADVERTISE) == PackageManager.PERMISSION_GRANTED &&
               ContextCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_CONNECT) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查BLE是否可用
     */
    fun isBluetoothAvailable(): Boolean {
        return bluetoothAdapter != null && bluetoothAdapter.isEnabled
    }

    /**
     * 开始广播
     */
    @SuppressLint("MissingPermission")
    fun startAdvertising(deviceName: String = "NearClip-Android"): Result<Unit> {
        Log.d(TAG, "开始BLE广播: $deviceName")

        if (!hasRequiredPermissions()) {
            Log.e(TAG, "缺少广播权限")
            return Result.failure(SecurityException("缺少BLE广播权限"))
        }

        if (!isBluetoothAvailable()) {
            Log.e(TAG, "蓝牙不可用")
            return Result.failure(IllegalStateException("蓝牙不可用"))
        }

        if (isAdvertising) {
            Log.w(TAG, "已经在广播中")
            return Result.success(Unit)
        }

        try {
            bluetoothLeAdvertiser?.let { advertiser ->
                // 配置广播设置
                val settings = AdvertiseSettings.Builder()
                    .setAdvertiseMode(ADVERTISING_INTERVAL)
                    .setTxPowerLevel(TX_POWER_LEVEL)
                    .setConnectable(true)
                    .setTimeout(0) // 永久广播
                    .build()

                // 配置广播数据
                val advertiseData = AdvertiseData.Builder()
                    .setIncludeDeviceName(true)
                    .setIncludeTxPowerLevel(true)
                    .addServiceUuid(SERVICE_UUID)
                    .addServiceData(SERVICE_UUID, createServiceData(deviceName))
                    .build()

                // 配置扫描响应数据
                val scanResponseData = AdvertiseData.Builder()
                    .setIncludeDeviceName(true)
                    .addManufacturerData(0x004C, createManufacturerData(deviceName))
                    .build()

                advertiseCallback = createAdvertiseCallback()
                advertiser.startAdvertising(settings, advertiseData, scanResponseData, advertiseCallback)
                isAdvertising = true
                Log.d(TAG, "BLE广播已启动")
                return Result.success(Unit)
            } ?: run {
                Log.e(TAG, "BLE广播器不可用")
                return Result.failure(IllegalStateException("BLE广播器不可用"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "启动广播失败", e)
            return Result.failure(e)
        }
    }

    /**
     * 停止广播
     */
    fun stopAdvertising() {
        Log.d(TAG, "停止BLE广播")

        if (!isAdvertising) {
            return
        }

        try {
            advertiseCallback?.let { callback ->
                bluetoothLeAdvertiser?.stopAdvertising(callback)
            }
            isAdvertising = false
            advertiseCallback = null
            Log.d(TAG, "BLE广播已停止")
        } catch (e: Exception) {
            Log.e(TAG, "停止广播失败", e)
        }
    }

    /**
     * 创建广播回调
     */
    private fun createAdvertiseCallback(): AdvertiseCallback {
        return object : AdvertiseCallback() {
            override fun onStartSuccess(settingsInEffect: AdvertiseSettings) {
                Log.d(TAG, "广播启动成功")
                Log.d(TAG, "广播模式: ${settingsInEffect.mode}, TX功率: ${settingsInEffect.txPowerLevel}")
            }

            override fun onStartFailure(errorCode: Int) {
                Log.e(TAG, "广播启动失败，错误代码: $errorCode")
                val errorMessage = when (errorCode) {
                    ADVERTISE_FAILED_ALREADY_STARTED -> "广播已在进行中"
                    ADVERTISE_FAILED_DATA_TOO_LARGE -> "广播数据过大"
                    ADVERTISE_FAILED_FEATURE_UNSUPPORTED -> "不支持的功能"
                    ADVERTISE_FAILED_INTERNAL_ERROR -> "内部错误"
                    ADVERTISE_FAILED_TOO_MANY_ADVERTISERS -> "广播器数量过多"
                    else -> "未知错误: $errorCode"
                }
                Log.e(TAG, "广播失败: $errorMessage")
                isAdvertising = false
            }
        }
    }

    /**
     * 创建服务数据
     */
    private fun createServiceData(deviceName: String): ByteArray {
        val deviceId = bluetoothAdapter?.address ?: "unknown"
        val timestamp = System.currentTimeMillis()

        // 格式: deviceId|deviceName|timestamp
        val data = "$deviceId|$deviceName|$timestamp"
        return data.toByteArray(StandardCharsets.UTF_8)
    }

    /**
     * 创建制造商数据
     */
    private fun createManufacturerData(deviceName: String): ByteArray {
        // 使用Apple的制造商ID (0x004C) 来模拟iOS设备
        val deviceId = bluetoothAdapter?.address ?: "unknown"

        // 简单的数据结构
        val data = mutableMapOf<String, String>()
        data["id"] = deviceId
        data["name"] = deviceName
        data["type"] = "android"
        data["version"] = "1.0"

        // 简单的序列化
        val dataString = data.entries.joinToString(";") { "${it.key}=${it.value}" }
        return dataString.toByteArray(StandardCharsets.UTF_8)
    }

    /**
     * 获取广播状态
     */
    fun isAdvertising(): Boolean = isAdvertising

    /**
     * 清理资源
     */
    fun cleanup() {
        stopAdvertising()
    }
}