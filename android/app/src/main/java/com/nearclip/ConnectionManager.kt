package com.nearclip

import android.app.Application
import android.os.Build
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.data.SecureStorage
import com.nearclip.ffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import org.json.JSONObject

class ConnectionManager(application: Application) : AndroidViewModel(application), FfiNearClipCallback {

    private var manager: FfiNearClipManager? = null
    private val secureStorage = SecureStorage(application)

    private val _isRunning = MutableStateFlow(false)
    val isRunning: StateFlow<Boolean> = _isRunning.asStateFlow()

    private val _pairedDevices = MutableStateFlow<List<FfiDeviceInfo>>(emptyList())
    val pairedDevices: StateFlow<List<FfiDeviceInfo>> = _pairedDevices.asStateFlow()

    private val _connectedDevices = MutableStateFlow<List<FfiDeviceInfo>>(emptyList())
    val connectedDevices: StateFlow<List<FfiDeviceInfo>> = _connectedDevices.asStateFlow()

    private val _lastError = MutableStateFlow<String?>(null)
    val lastError: StateFlow<String?> = _lastError.asStateFlow()

    private val _lastReceivedClipboard = MutableStateFlow<Pair<ByteArray, String>?>(null)
    val lastReceivedClipboard: StateFlow<Pair<ByteArray, String>?> = _lastReceivedClipboard.asStateFlow()

    init {
        initializeManager()
    }

    private fun initializeManager() {
        try {
            val config = FfiNearClipConfig(
                deviceName = "${Build.MANUFACTURER} ${Build.MODEL}",
                wifiEnabled = true,
                bleEnabled = true,
                autoConnect = true,
                connectionTimeoutSecs = 30u,
                heartbeatIntervalSecs = 5u,
                maxRetries = 3u
            )
            manager = FfiNearClipManager(config, this)

            // Load paired devices from secure storage
            loadPairedDevicesFromStorage()

            refreshDevices()
        } catch (e: Exception) {
            _lastError.value = "Failed to initialize: ${e.message}"
        }
    }

    private fun loadPairedDevicesFromStorage() {
        val storedDevices = secureStorage.loadPairedDevices()
        for (device in storedDevices) {
            try {
                manager?.addPairedDevice(device)
            } catch (e: Exception) {
                // Ignore if device already exists
            }
        }
    }

    fun start() {
        viewModelScope.launch {
            try {
                manager?.start()
                _isRunning.value = manager?.isRunning() ?: false
                refreshDevices()
            } catch (e: NearClipException) {
                _lastError.value = "Start failed: ${e.message}"
            }
        }
    }

    fun stop() {
        manager?.stop()
        _isRunning.value = false
        refreshDevices()
    }

    fun connectDevice(deviceId: String) {
        viewModelScope.launch {
            try {
                manager?.connectDevice(deviceId)
                refreshDevices()
            } catch (e: NearClipException) {
                _lastError.value = "Connect failed: ${e.message}"
            }
        }
    }

    fun disconnectDevice(deviceId: String) {
        viewModelScope.launch {
            try {
                manager?.disconnectDevice(deviceId)
                refreshDevices()
            } catch (e: NearClipException) {
                _lastError.value = "Disconnect failed: ${e.message}"
            }
        }
    }

    fun syncClipboard(content: ByteArray) {
        viewModelScope.launch {
            try {
                manager?.syncClipboard(content)
            } catch (e: NearClipException) {
                _lastError.value = "Sync failed: ${e.message}"
            }
        }
    }

    fun addDeviceFromCode(code: String) {
        try {
            val json = JSONObject(code)
            val device = FfiDeviceInfo(
                id = json.getString("id"),
                name = json.getString("name"),
                platform = DevicePlatform.valueOf(json.getString("platform")),
                status = DeviceStatus.DISCONNECTED
            )
            manager?.addPairedDevice(device)
            // Persist to secure storage
            secureStorage.addPairedDevice(device)
            refreshDevices()
        } catch (e: Exception) {
            throw IllegalArgumentException("Invalid pairing code: ${e.message}")
        }
    }

    fun removeDevice(deviceId: String) {
        manager?.removePairedDevice(deviceId)
        // Remove from secure storage
        secureStorage.removePairedDevice(deviceId)
        refreshDevices()
    }

    fun generatePairingCode(): String {
        val deviceId = java.util.UUID.randomUUID().toString()
        val json = JSONObject().apply {
            put("id", deviceId)
            put("name", "${Build.MANUFACTURER} ${Build.MODEL}")
            put("platform", "ANDROID")
        }
        return json.toString()
    }

    private fun refreshDevices() {
        _pairedDevices.value = manager?.getPairedDevices() ?: emptyList()
        _connectedDevices.value = manager?.getConnectedDevices() ?: emptyList()
    }

    // FfiNearClipCallback implementation

    override fun onDeviceConnected(device: FfiDeviceInfo) {
        refreshDevices()
    }

    override fun onDeviceDisconnected(deviceId: String) {
        refreshDevices()
    }

    override fun onClipboardReceived(content: ByteArray, fromDevice: String) {
        _lastReceivedClipboard.value = Pair(content, fromDevice)
    }

    override fun onSyncError(errorMessage: String) {
        _lastError.value = errorMessage
    }

    override fun onCleared() {
        super.onCleared()
        manager?.stop()
        manager?.destroy()
    }
}
