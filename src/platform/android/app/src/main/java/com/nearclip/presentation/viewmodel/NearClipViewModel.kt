package com.nearclip.presentation.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.repository.DeviceRepository
import com.nearclip.services.PermissionManager
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import javax.inject.Inject

/**
 * NearClip主ViewModel
 * 管理应用的核心状态和业务逻辑
 */
@HiltViewModel
class NearClipViewModel @Inject constructor(
    private val deviceRepository: DeviceRepository,
    private val permissionManager: PermissionManager
) : BaseViewModel() {

    // UI状态数据类
    data class NearClipUiState(
        val isLoading: Boolean = false,
        val discoveredDevices: List<Device> = emptyList(),
        val connectedDevices: List<Device> = emptyList(),
        val isDiscovering: Boolean = false,
        val errorMessage: String? = null,
        val hasPermissions: Boolean = false,
        val selectedDevice: Device? = null
    )

    // UI状态流
    private val _uiState = MutableStateFlow(NearClipUiState())
    val uiState: StateFlow<NearClipUiState> = _uiState.asStateFlow()

    init {
        // 监听权限状态变化
        viewModelScope.launch {
            permissionManager.permissionStates.collect { permissions ->
                val hasAllPermissions = permissions.values.all { it }
                _uiState.update { it.copy(hasPermissions = hasAllPermissions) }
            }
        }

        // 监听设备列表变化
        viewModelScope.launch {
            combine(
                deviceRepository.getAllDevices(),
                deviceRepository.getConnectedDevices()
            ) { allDevices, connectedDevices ->
                _uiState.update {
                    it.copy(
                        discoveredDevices = allDevices,
                        connectedDevices = connectedDevices
                    )
                }
            }
        }
    }

    /**
     * 开始设备发现
     */
    fun startDeviceDiscovery() {
        if (!permissionManager.areBluetoothPermissionsGranted()) {
            _uiState.update { it.copy(errorMessage = "需要蓝牙权限才能发现设备") }
            return
        }

        launchSafely {
            _uiState.update { it.copy(isDiscovering = true, errorMessage = null) }

            // 这里将来会调用Rust FFI进行实际的设备发现
            // 目前先模拟设备发现逻辑
            simulateDeviceDiscovery()
        }
    }

    /**
     * 停止设备发现
     */
    fun stopDeviceDiscovery() {
        launchSafely {
            _uiState.update { it.copy(isDiscovering = false) }
            // 这里将来会调用Rust FFI停止设备发现
        }
    }

    /**
     * 连接到设备
     */
    fun connectToDevice(device: Device) {
        if (!permissionManager.areBluetoothPermissionsGranted()) {
            _uiState.update { it.copy(errorMessage = "需要蓝牙权限才能连接设备") }
            return
        }

        launchSafely {
            // 这里将来会调用Rust FFI进行实际的设备连接
            deviceRepository.updateDeviceConnectionStatus(device.deviceId, true)

            _uiState.update {
                it.copy(selectedDevice = device, errorMessage = null)
            }
        }
    }

    /**
     * 断开设备连接
     */
    fun disconnectFromDevice(device: Device) {
        launchSafely {
            deviceRepository.updateDeviceConnectionStatus(device.deviceId, false)

            _uiState.update {
                it.copy(
                    selectedDevice = if (_uiState.value.selectedDevice?.deviceId == device.deviceId) null
                        else _uiState.value.selectedDevice
                )
            }
        }
    }

    /**
     * 选择设备
     */
    fun selectDevice(device: Device) {
        _uiState.update { it.copy(selectedDevice = device) }
    }

    /**
     * 清除错误消息
     */
    fun clearErrorMessage() {
        _uiState.update { it.copy(errorMessage = null) }
        clearError()
    }

    /**
     * 获取设备总数
     */
    fun getDeviceCount(): Int {
        return _uiState.value.discoveredDevices.size
    }

    /**
     * 获取连接的设备数量
     */
    fun getConnectedDeviceCount(): Int {
        return _uiState.value.connectedDevices.size
    }

    /**
     * 模拟设备发现（临时实现，将来会被Rust FFI替代）
     */
    private fun simulateDeviceDiscovery() {
        viewModelScope.launch {
            kotlinx.coroutines.delay(2000) // 模拟发现延迟

            val mockDevice = Device(
                deviceId = "mock-device-1",
                deviceName = "Mock Android Device",
                deviceType = Device.DeviceType.ANDROID,
                publicKey = "mock-public-key-12345",
                lastSeen = System.currentTimeMillis(),
                connectionStatus = ConnectionStatus.DISCONNECTED
            )

            deviceRepository.insertDevice(mockDevice)

            _uiState.update { it.copy(isDiscovering = false) }
        }
    }

    /**
     * 检查是否有所有权限
     */
    fun hasAllPermissions(): Boolean {
        return permissionManager.areAllPermissionsGranted()
    }

    /**
     * 检查蓝牙权限
     */
    fun hasBluetoothPermissions(): Boolean {
        return permissionManager.areBluetoothPermissionsGranted()
    }

    /**
     * 检查剪贴板权限
     */
    fun hasClipboardPermissions(): Boolean {
        return permissionManager.areClipboardPermissionsGranted()
    }
}