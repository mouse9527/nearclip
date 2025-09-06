package com.nearclip.android.ble

import android.app.Service
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.BluetoothLeAdvertiser
import android.bluetooth.le.BluetoothLeScanner
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanFilter
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Intent
import android.os.Binder
import android.os.IBinder
import android.os.ParcelUuid
import android.util.Log
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.util.UUID
import javax.inject.Inject

@AndroidEntryPoint
class BleService : Service() {
    
    companion object {
        private const val TAG = "BleService"
        // NearClip 服务 UUID
        val NEARCLIP_SERVICE_UUID: UUID = UUID.fromString("12345678-1234-5678-9012-123456789abc")
    }
    
    @Inject
    lateinit var bluetoothManager: BluetoothManager
    
    private val bluetoothAdapter: BluetoothAdapter? by lazy {
        bluetoothManager.adapter
    }
    
    private val bluetoothLeScanner: BluetoothLeScanner? by lazy {
        bluetoothAdapter?.bluetoothLeScanner
    }
    
    private val bluetoothLeAdvertiser: BluetoothLeAdvertiser? by lazy {
        bluetoothAdapter?.bluetoothLeAdvertiser
    }
    
    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.Main)
    
    // 扫描状态
    private val _isScanning = MutableStateFlow(false)
    val isScanning: StateFlow<Boolean> = _isScanning.asStateFlow()
    
    // 发现的设备列表
    private val _discoveredDevices = MutableStateFlow<List<NearClipDevice>>(emptyList())
    val discoveredDevices: StateFlow<List<NearClipDevice>> = _discoveredDevices.asStateFlow()
    
    // 扫描错误
    private val _scanError = MutableStateFlow<String?>(null)
    val scanError: StateFlow<String?> = _scanError.asStateFlow()
    
    // 广播状态
    private val _isAdvertising = MutableStateFlow(false)
    val isAdvertising: StateFlow<Boolean> = _isAdvertising.asStateFlow()
    
    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            super.onScanResult(callbackType, result)
            handleScanResult(result)
        }
        
        override fun onBatchScanResults(results: MutableList<ScanResult>) {
            super.onBatchScanResults(results)
            results.forEach { handleScanResult(it) }
        }
        
        override fun onScanFailed(errorCode: Int) {
            super.onScanFailed(errorCode)
            val errorMessage = when (errorCode) {
                SCAN_FAILED_ALREADY_STARTED -> "扫描已经开始"
                SCAN_FAILED_APPLICATION_REGISTRATION_FAILED -> "应用注册失败"
                SCAN_FAILED_FEATURE_UNSUPPORTED -> "设备不支持 BLE 扫描"
                SCAN_FAILED_INTERNAL_ERROR -> "内部错误"
                SCAN_FAILED_OUT_OF_HARDWARE_RESOURCES -> "硬件资源不足"
                SCAN_FAILED_SCANNING_TOO_FREQUENTLY -> "扫描过于频繁"
                else -> "未知错误: $errorCode"
            }
            Log.e(TAG, "BLE 扫描失败: $errorMessage")
            serviceScope.launch {
                _scanError.value = errorMessage
                _isScanning.value = false
            }
        }
    }
    
    private val advertiseCallback = object : AdvertiseCallback() {
        override fun onStartSuccess(settingsInEffect: AdvertiseSettings?) {
            super.onStartSuccess(settingsInEffect)
            Log.d(TAG, "BLE 广播启动成功")
            serviceScope.launch {
                _isAdvertising.value = true
            }
        }
        
        override fun onStartFailure(errorCode: Int) {
            super.onStartFailure(errorCode)
            val errorMessage = when (errorCode) {
                ADVERTISE_FAILED_ALREADY_STARTED -> "广播已经开始"
                ADVERTISE_FAILED_DATA_TOO_LARGE -> "广播数据过大"
                ADVERTISE_FAILED_FEATURE_UNSUPPORTED -> "设备不支持 BLE 广播"
                ADVERTISE_FAILED_INTERNAL_ERROR -> "内部错误"
                ADVERTISE_FAILED_TOO_MANY_ADVERTISERS -> "广播器过多"
                else -> "未知错误: $errorCode"
            }
            Log.e(TAG, "BLE 广播启动失败: $errorMessage")
            serviceScope.launch {
                _isAdvertising.value = false
            }
        }
    }
    
    private fun handleScanResult(result: ScanResult) {
        val device = result.device
        val rssi = result.rssi
        val scanRecord = result.scanRecord
        
        Log.d(TAG, "发现设备: ${device.name ?: "未知"} (${device.address}) RSSI: $rssi")
        
        // 检查是否是 NearClip 设备
        val serviceUuids = scanRecord?.serviceUuids
        Log.d(TAG, "设备服务UUID: $serviceUuids")
        
        val isNearClipDevice = serviceUuids?.any { 
            it.uuid == NEARCLIP_SERVICE_UUID 
        } ?: false
        
        Log.d(TAG, "是否为NearClip设备: $isNearClipDevice")
        
        // 临时显示所有设备用于调试
        if (true) { // 改为 isNearClipDevice 来恢复过滤
            val nearClipDevice = NearClipDevice(
                address = device.address,
                name = device.name ?: "未知设备",
                rssi = rssi,
                isConnectable = true // 默认为可连接
            )
            
            serviceScope.launch {
                val currentDevices = _discoveredDevices.value.toMutableList()
                val existingIndex = currentDevices.indexOfFirst { it.address == device.address }
                
                if (existingIndex >= 0) {
                    // 更新现有设备信息
                    currentDevices[existingIndex] = nearClipDevice
                } else {
                    // 添加新设备
                    currentDevices.add(nearClipDevice)
                }
                
                _discoveredDevices.value = currentDevices
            }
            
            Log.d(TAG, "发现 NearClip 设备: ${nearClipDevice.name} (${nearClipDevice.address})")
        }
    }
    
    fun startScan() {
        if (_isScanning.value) {
            Log.w(TAG, "扫描已在进行中")
            return
        }
        
        val adapter = bluetoothAdapter
        val scanner = bluetoothLeScanner
        
        if (adapter == null || !adapter.isEnabled) {
            serviceScope.launch {
                _scanError.value = "蓝牙未启用"
            }
            return
        }
        
        if (scanner == null) {
            serviceScope.launch {
                _scanError.value = "设备不支持 BLE"
            }
            return
        }
        
        // 清除之前的错误和设备列表
        serviceScope.launch {
            _scanError.value = null
            _discoveredDevices.value = emptyList()
        }
        
        // 配置扫描过滤器 - 临时移除过滤器以查看所有设备
        // val scanFilter = ScanFilter.Builder()
        //     .setServiceUuid(ParcelUuid(NEARCLIP_SERVICE_UUID))
        //     .build()
        
        // 配置扫描设置
        val scanSettings = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
            .setCallbackType(ScanSettings.CALLBACK_TYPE_ALL_MATCHES)
            .setMatchMode(ScanSettings.MATCH_MODE_AGGRESSIVE)
            .setNumOfMatches(ScanSettings.MATCH_NUM_MAX_ADVERTISEMENT)
            .setReportDelay(0L)
            .build()
        
        try {
            // 临时不使用过滤器，扫描所有设备
            scanner.startScan(emptyList(), scanSettings, scanCallback)
            serviceScope.launch {
                _isScanning.value = true
            }
            Log.d(TAG, "开始 BLE 扫描")
        } catch (e: SecurityException) {
            Log.e(TAG, "缺少蓝牙权限", e)
            serviceScope.launch {
                _scanError.value = "缺少蓝牙权限"
            }
        } catch (e: Exception) {
            Log.e(TAG, "启动扫描失败", e)
            serviceScope.launch {
                _scanError.value = "启动扫描失败: ${e.message}"
            }
        }
    }
    
    fun stopScan() {
        if (!_isScanning.value) {
            Log.w(TAG, "扫描未在进行中")
            return
        }
        
        try {
            bluetoothLeScanner?.stopScan(scanCallback)
            serviceScope.launch {
                _isScanning.value = false
            }
            Log.d(TAG, "停止 BLE 扫描")
        } catch (e: SecurityException) {
            Log.e(TAG, "停止扫描时缺少权限", e)
        } catch (e: Exception) {
            Log.e(TAG, "停止扫描失败", e)
        }
    }
    
    fun startAdvertising() {
        if (_isAdvertising.value) {
            Log.w(TAG, "广播已在进行中")
            return
        }
        
        val adapter = bluetoothAdapter
        val advertiser = bluetoothLeAdvertiser
        
        if (adapter == null || !adapter.isEnabled) {
            Log.e(TAG, "蓝牙未启用，无法开始广播")
            return
        }
        
        if (advertiser == null) {
            Log.e(TAG, "设备不支持 BLE 广播")
            return
        }
        
        // 配置广播设置
        val settings = AdvertiseSettings.Builder()
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_MEDIUM)
            .setConnectable(true)
            .setTimeout(0) // 持续广播
            .build()
        
        // 配置广播数据
        val data = AdvertiseData.Builder()
            .setIncludeDeviceName(false)
            .setIncludeTxPowerLevel(false)
            .addServiceUuid(ParcelUuid(NEARCLIP_SERVICE_UUID))
            .build()
        
        try {
            advertiser.startAdvertising(settings, data, advertiseCallback)
            Log.d(TAG, "开始 BLE 广播")
        } catch (e: SecurityException) {
            Log.e(TAG, "缺少蓝牙广播权限", e)
        } catch (e: Exception) {
            Log.e(TAG, "启动广播失败", e)
        }
    }
    
    fun stopAdvertising() {
        if (!_isAdvertising.value) {
            Log.w(TAG, "广播未在进行中")
            return
        }
        
        try {
            bluetoothLeAdvertiser?.stopAdvertising(advertiseCallback)
            serviceScope.launch {
                _isAdvertising.value = false
            }
            Log.d(TAG, "停止 BLE 广播")
        } catch (e: SecurityException) {
            Log.e(TAG, "停止广播时缺少权限", e)
        } catch (e: Exception) {
            Log.e(TAG, "停止广播失败", e)
        }
    }
    
    inner class LocalBinder : Binder() {
        fun getService(): BleService = this@BleService
    }
    
    private val binder = LocalBinder()
    
    override fun onBind(intent: Intent): IBinder {
        return binder
    }
    
    override fun onDestroy() {
        super.onDestroy()
        stopScan()
        stopAdvertising()
    }
}