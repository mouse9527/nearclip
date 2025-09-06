package com.nearclip.android.ui.discovery

import android.app.Application
import android.bluetooth.BluetoothAdapter
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.android.ble.BleService
import com.nearclip.android.ble.NearClipDevice
import com.nearclip.android.permissions.PermissionManager
import com.nearclip.android.permissions.PermissionStatus
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class DeviceDiscoveryViewModel @Inject constructor(
    application: Application,
    private val permissionManager: PermissionManager
) : AndroidViewModel(application) {
    
    companion object {
        private const val TAG = "DeviceDiscoveryVM"
    }
    
    private var bleService: BleService? = null
    private var isBound = false
    
    // UI 状态
    private val _uiState = MutableStateFlow(DeviceDiscoveryUiState())
    val uiState: StateFlow<DeviceDiscoveryUiState> = _uiState.asStateFlow()
    
    // 权限状态
    private val _permissionStatus = MutableStateFlow(PermissionStatus.PERMISSIONS_MISSING)
    val permissionStatus: StateFlow<PermissionStatus> = _permissionStatus.asStateFlow()
    
    // 服务连接
    private val serviceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            val binder = service as BleService.LocalBinder
            bleService = binder.getService()
            isBound = true
            
            // 监听服务状态
            observeServiceState()
        }
        
        override fun onServiceDisconnected(name: ComponentName?) {
            bleService = null
            isBound = false
        }
    }
    
    init {
        bindBleService()
        checkPermissions()
    }
    
    private fun bindBleService() {
        val intent = Intent(getApplication(), BleService::class.java)
        getApplication<Application>().bindService(intent, serviceConnection, Context.BIND_AUTO_CREATE)
    }
    
    private fun observeServiceState() {
        val service = bleService ?: return
        Log.d(TAG, "开始观察服务状态")
        
        viewModelScope.launch {
            combine(
                service.isScanning,
                service.discoveredDevices,
                service.scanError
            ) { isScanning, devices, error ->
                Log.d(TAG, "状态更新: isScanning=$isScanning, devices=${devices.size}, error=$error")
                _uiState.value = _uiState.value.copy(
                    isScanning = isScanning,
                    discoveredDevices = devices,
                    errorMessage = error
                )
            }.collect { /* 确保流被收集 */ }
        }
    }
    
    fun checkPermissions() {
        _permissionStatus.value = permissionManager.getPermissionStatusDescription()
    }
    
    fun startScan() {
        Log.d("DeviceDiscoveryVM", "startScan() called")
        
        if (!permissionManager.hasAllBluetoothPermissions()) {
            Log.w("DeviceDiscoveryVM", "Missing bluetooth permissions")
            _uiState.value = _uiState.value.copy(
                errorMessage = "缺少蓝牙权限，请先授予权限"
            )
            return
        }
        
        if (!permissionManager.isBluetoothEnabled()) {
            Log.w("DeviceDiscoveryVM", "Bluetooth not enabled")
            _uiState.value = _uiState.value.copy(
                showBluetoothEnableDialog = true
            )
            return
        }
        
        if (bleService == null) {
            Log.e("DeviceDiscoveryVM", "BLE service is null, binding service...")
            bindBleService()
            _uiState.value = _uiState.value.copy(
                errorMessage = "正在连接蓝牙服务，请稍后重试"
            )
            return
        }
        
        Log.d("DeviceDiscoveryVM", "Starting BLE scan...")
        bleService?.startScan()
    }
    
    fun stopScan() {
        bleService?.stopScan()
    }
    
    fun enableBluetooth() {
        val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
        // 这里需要在 Activity 中处理
        _uiState.value = _uiState.value.copy(
            bluetoothEnableIntent = enableBtIntent
        )
    }
    
    fun dismissBluetoothDialog() {
        _uiState.value = _uiState.value.copy(
            showBluetoothEnableDialog = false
        )
    }
    
    fun clearError() {
        _uiState.value = _uiState.value.copy(
            errorMessage = null
        )
    }
    
    fun onDeviceSelected(device: NearClipDevice) {
        // TODO: 实现设备连接逻辑
        _uiState.value = _uiState.value.copy(
            selectedDevice = device
        )
    }
    
    fun getRequiredPermissions(): List<String> {
        return permissionManager.getRequiredBluetoothPermissions()
    }
    
    override fun onCleared() {
        super.onCleared()
        if (isBound) {
            getApplication<Application>().unbindService(serviceConnection)
            isBound = false
        }
    }
}

/**
 * 设备发现 UI 状态
 */
data class DeviceDiscoveryUiState(
    val isScanning: Boolean = false,
    val discoveredDevices: List<NearClipDevice> = emptyList(),
    val selectedDevice: NearClipDevice? = null,
    val errorMessage: String? = null,
    val showBluetoothEnableDialog: Boolean = false,
    val bluetoothEnableIntent: Intent? = null
) {
    val hasDevices: Boolean get() = discoveredDevices.isNotEmpty()
    val canStartScan: Boolean get() = !isScanning
}