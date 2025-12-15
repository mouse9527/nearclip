package com.nearclip

import android.app.Application
import android.os.Build
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.data.SecureStorage
import com.nearclip.ffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import org.json.JSONException
import org.json.JSONObject

class ConnectionManager(application: Application) : AndroidViewModel(application), FfiNearClipCallback {

    companion object {
        private const val TAG = "ConnectionManager"
    }

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
        val result = secureStorage.loadPairedDevicesResult()
        when (result) {
            is SecureStorage.StorageResult.Success -> {
                for (device in result.data) {
                    try {
                        manager?.addPairedDevice(device)
                        Log.d(TAG, "Loaded paired device: ${device.name} (${device.id})")
                    } catch (e: NearClipException) {
                        // Device already exists in manager - this is expected
                        Log.d(TAG, "Device ${device.id} already exists in manager")
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to add device ${device.id} to manager", e)
                    }
                }
            }
            is SecureStorage.StorageResult.Error -> {
                Log.e(TAG, "Failed to load paired devices from storage: ${result.message}")
                _lastError.value = "Failed to load saved devices: ${result.message}"
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

    /**
     * Add a device from a pairing code (JSON string).
     * @throws IllegalArgumentException if the code is invalid or missing required fields
     * @throws IllegalStateException if the manager is not initialized
     */
    fun addDeviceFromCode(code: String) {
        // Parse and validate JSON
        val json = try {
            JSONObject(code)
        } catch (e: JSONException) {
            throw IllegalArgumentException("Invalid pairing code format: not valid JSON")
        }

        // Validate required fields
        val id = json.optString("id", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'id' field")
        val name = json.optString("name", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'name' field")
        val platformStr = json.optString("platform", "").takeIf { it.isNotEmpty() }
            ?: throw IllegalArgumentException("Invalid pairing code: missing 'platform' field")

        // Validate platform enum
        val platform = try {
            DevicePlatform.valueOf(platformStr)
        } catch (e: IllegalArgumentException) {
            val validPlatforms = DevicePlatform.values().joinToString(", ") { it.name }
            throw IllegalArgumentException("Invalid pairing code: unknown platform '$platformStr'. Valid: $validPlatforms")
        }

        val device = FfiDeviceInfo(
            id = id,
            name = name,
            platform = platform,
            status = DeviceStatus.DISCONNECTED
        )

        // Ensure manager is initialized
        val mgr = manager
            ?: throw IllegalStateException("Connection manager not initialized")

        // Add to manager first - this is the source of truth
        try {
            mgr.addPairedDevice(device)
            Log.i(TAG, "Added paired device to manager: ${device.name} (${device.id})")
        } catch (e: NearClipException) {
            throw IllegalStateException("Failed to add device to manager: ${e.message}")
        }

        // Persist to secure storage (best effort - manager is already updated)
        try {
            secureStorage.addPairedDevice(device)
            Log.i(TAG, "Persisted device to secure storage: ${device.id}")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to persist device to storage (device added to manager)", e)
            // Don't fail - device is already in manager and will work for this session
        }

        refreshDevices()
    }

    /**
     * Remove a paired device.
     * Removes from both manager and secure storage.
     */
    fun removeDevice(deviceId: String) {
        var managerSuccess = false
        var storageSuccess = false

        // Remove from manager first
        try {
            manager?.removePairedDevice(deviceId)
            managerSuccess = true
            Log.i(TAG, "Removed device from manager: $deviceId")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to remove device from manager: $deviceId", e)
        }

        // Remove from secure storage
        try {
            secureStorage.removePairedDevice(deviceId)
            storageSuccess = true
            Log.i(TAG, "Removed device from secure storage: $deviceId")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to remove device from secure storage: $deviceId", e)
        }

        // Log if partial removal occurred
        if (managerSuccess != storageSuccess) {
            Log.w(TAG, "Partial device removal - manager: $managerSuccess, storage: $storageSuccess")
        }

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
