package com.nearclip.services.ble

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.le.*
import android.content.Context
import android.content.pm.PackageManager
import android.os.ParcelUuid
import android.util.Log
import androidx.core.content.ContextCompat
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.receiveAsFlow
import java.util.*

/**
 * BLE设备扫描器
 * 负责扫描附近的BLE设备并提供设备发现回调
 */
class BleScanner(private val context: Context) {

    companion object {
        private const val TAG = "BleScanner"
        private val SERVICE_UUID = ParcelUuid.fromString("0000FE2C-0000-1000-8000-00805F9B34FB")
    }

    private val bluetoothAdapter = android.bluetooth.BluetoothAdapter.getDefaultAdapter()
    private val bluetoothLeScanner: BluetoothLeScanner? = bluetoothAdapter?.bluetoothLeScanner
    private val _discoveredDevices = Channel<BleDevice>(capacity = Channel.UNLIMITED)
    private var isScanning = false
    private var scanCallback: ScanCallback? = null

    val discoveredDevices: Flow<BleDevice> = _discoveredDevices.receiveAsFlow()

    /**
     * 检查BLE权限
     */
    fun hasRequiredPermissions(): Boolean {
        return ContextCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_SCAN) == PackageManager.PERMISSION_GRANTED &&
               ContextCompat.checkSelfPermission(context, Manifest.permission.BLUETOOTH_CONNECT) == PackageManager.PERMISSION_GRANTED &&
               ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * 检查BLE是否可用
     */
    fun isBluetoothAvailable(): Boolean {
        return bluetoothAdapter != null && bluetoothAdapter.isEnabled
    }

    /**
     * 开始扫描BLE设备
     */
    @SuppressLint("MissingPermission")
    fun startScanning(): Result<Unit> {
        Log.d(TAG, "开始BLE扫描")

        if (!hasRequiredPermissions()) {
            Log.e(TAG, "缺少必要的权限")
            return Result.failure(SecurityException("缺少BLE权限"))
        }

        if (!isBluetoothAvailable()) {
            Log.e(TAG, "蓝牙不可用")
            return Result.failure(IllegalStateException("蓝牙不可用"))
        }

        if (isScanning) {
            Log.w(TAG, "已经在扫描中")
            return Result.success(Unit)
        }

        try {
            bluetoothLeScanner?.let { scanner ->
                scanCallback = createScanCallback()

                // 配置扫描设置
                val scanSettings = ScanSettings.Builder()
                    .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
                    .setCallbackType(ScanSettings.CALLBACK_TYPE_ALL_MATCHES)
                    .setMatchMode(ScanSettings.MATCH_MODE_AGGRESSIVE)
                    .setNumOfMatches(ScanSettings.MATCH_NUM_MAX_ADVERTISEMENT)
                    .build()

                // 配置扫描过滤器
                val filters = mutableListOf<ScanFilter>()
                // 可以添加特定服务的过滤器
                // filters.add(ScanFilter.Builder().setServiceUuid(SERVICE_UUID).build())

                scanner.startScan(filters, scanSettings, scanCallback)
                isScanning = true
                Log.d(TAG, "BLE扫描已启动")
                return Result.success(Unit)
            } ?: run {
                Log.e(TAG, "BLE扫描器不可用")
                return Result.failure(IllegalStateException("BLE扫描器不可用"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "启动扫描失败", e)
            return Result.failure(e)
        }
    }

    /**
     * 停止扫描
     */
    fun stopScanning() {
        Log.d(TAG, "停止BLE扫描")

        if (!isScanning) {
            return
        }

        try {
            scanCallback?.let { callback ->
                bluetoothLeScanner?.stopScan(callback)
            }
            isScanning = false
            scanCallback = null
            Log.d(TAG, "BLE扫描已停止")
        } catch (e: Exception) {
            Log.e(TAG, "停止扫描失败", e)
        }
    }

    /**
     * 创建扫描回调
     */
    private fun createScanCallback(): ScanCallback {
        return object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                handleScanResult(result)
            }

            override fun onBatchScanResults(results: MutableList<ScanResult>) {
                results.forEach { handleScanResult(it) }
            }

            override fun onScanFailed(errorCode: Int) {
                Log.e(TAG, "扫描失败，错误代码: $errorCode")
                val errorMessage = when (errorCode) {
                    SCAN_FAILED_ALREADY_STARTED -> "扫描已在进行中"
                    SCAN_FAILED_APPLICATION_REGISTRATION_FAILED -> "应用注册失败"
                    SCAN_FAILED_INTERNAL_ERROR -> "内部错误"
                    SCAN_FAILED_FEATURE_UNSUPPORTED -> "不支持的功能"
                    SCAN_FAILED_OUT_OF_HARDWARE_RESOURCES -> "硬件资源不足"
                    else -> "未知错误: $errorCode"
                }
                Log.e(TAG, "扫描失败: $errorMessage")
            }
        }
    }

    /**
     * 处理扫描结果
     */
    private fun handleScanResult(result: ScanResult) {
        val device = result.device
        val rssi = result.rssi
        val scanRecord = result.scanRecord

        if (device.name == null && scanRecord?.deviceName == null) {
            // 忽略没有名称的设备
            return
        }

        val deviceName = device.name ?: scanRecord?.deviceName ?: "未知设备"
        val deviceId = device.address
        val deviceType = when {
            scanRecord?.serviceUuids?.contains(SERVICE_UUID) == true -> BleDeviceType.NEARCLIP
            device.name?.contains("NearClip", true) == true -> BleDeviceType.NEARCLIP
            device.bluetoothType == android.bluetooth.BluetoothDevice.DEVICE_TYPE_DUAL -> BleDeviceType.DUAL
            device.bluetoothType == android.bluetooth.BluetoothDevice.DEVICE_TYPE_LE -> BleDeviceType.LE
            else -> BleDeviceType.UNKNOWN
        }

        val bleDevice = BleDevice(
            deviceId = deviceId,
            deviceName = deviceName,
            deviceType = deviceType,
            rssi = rssi,
            timestamp = System.currentTimeMillis(),
            bluetoothDevice = device
        )

        Log.d(TAG, "发现设备: $deviceName (${device.address}), RSSI: $rssi, 类型: $deviceType")

        try {
            _discoveredDevices.trySend(bleDevice)
        } catch (e: Exception) {
            Log.e(TAG, "发送设备信息失败", e)
        }
    }

    /**
     * 清理资源
     */
    fun cleanup() {
        stopScanning()
        _discoveredDevices.close()
    }
}

/**
 * BLE设备信息
 */
data class BleDevice(
    val deviceId: String,
    val deviceName: String,
    val deviceType: BleDeviceType,
    val rssi: Int,
    val timestamp: Long,
    val bluetoothDevice: android.bluetooth.BluetoothDevice
)

/**
 * BLE设备类型
 */
enum class BleDeviceType {
    UNKNOWN,
    LE,           // 低功耗设备
    DUAL,         // 双模设备
    NEARCLIP      // NearClip设备
}