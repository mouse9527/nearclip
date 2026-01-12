package com.nearclip

import android.app.Application
import android.os.Build
import android.util.Log
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.data.DeviceStorageImpl
import com.nearclip.data.SecureStorage
import com.nearclip.data.SyncRetryStrategy
import com.nearclip.data.settingsDataStore
import com.nearclip.ffi.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.json.JSONException
import org.json.JSONObject

class ConnectionManager(application: Application) : AndroidViewModel(application), FfiNearClipCallback {

    companion object {
        private const val TAG = "ConnectionManager"
        const val MAX_PAIRED_DEVICES = 5
        private const val PREFS_NAME = "nearclip_prefs"
        private const val KEY_PAUSED_DEVICES = "paused_device_ids"
        private const val KEY_DEVICE_ID = "nearclip_device_id"
    }

    private var manager: FfiNearClipManager? = null
    private val secureStorage = SecureStorage(application)
    private val prefs = application.getSharedPreferences(PREFS_NAME, android.content.Context.MODE_PRIVATE)

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

    private val _pausedDeviceIds = MutableStateFlow<Set<String>>(emptySet())
    val pausedDeviceIds: StateFlow<Set<String>> = _pausedDeviceIds.asStateFlow()

    /** Pending content for "wait for device" strategy */
    private var pendingContent: ByteArray? = null

    /** Whether we can add more devices (haven't reached the limit) */
    val canAddMoreDevices: Boolean
        get() = _pairedDevices.value.size < MAX_PAIRED_DEVICES

    /** Cached default retry strategy - updated via flow collection */
    private var _cachedRetryStrategy: SyncRetryStrategy = SyncRetryStrategy.WAIT_FOR_DEVICE

    /** Get the default retry strategy from cache (non-blocking) */
    val defaultRetryStrategy: SyncRetryStrategy
        get() = _cachedRetryStrategy

    /** Start observing settings changes */
    private fun observeRetryStrategySetting() {
        viewModelScope.launch {
            getApplication<Application>().settingsDataStore.data
                .map { prefs ->
                    val value = prefs[androidx.datastore.preferences.core.stringPreferencesKey("default_retry_strategy")]
                        ?: SyncRetryStrategy.WAIT_FOR_DEVICE.value
                    SyncRetryStrategy.fromValue(value)
                }
                .collect { strategy ->
                    _cachedRetryStrategy = strategy
                }
        }
    }

    init {
        // Load paused devices from preferences
        _pausedDeviceIds.value = prefs.getStringSet(KEY_PAUSED_DEVICES, emptySet()) ?: emptySet()
        // Start observing retry strategy setting (non-blocking)
        observeRetryStrategySetting()
        initializeManager()
    }

    private fun initializeManager() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                // Load persisted device ID (empty string means auto-generate)
                val persistedDeviceId = prefs.getString(KEY_DEVICE_ID, "") ?: ""

                val config = FfiNearClipConfig(
                    deviceName = "${Build.MANUFACTURER} ${Build.MODEL}",
                    deviceId = persistedDeviceId,
                    wifiEnabled = true,
                    bleEnabled = true,
                    autoConnect = true,
                    connectionTimeoutSecs = 30u,
                    heartbeatIntervalSecs = 5u,
                    maxRetries = 3u
                )
                manager = FfiNearClipManager(config, this@ConnectionManager)

                // Save generated device ID if it was newly created
                if (persistedDeviceId.isEmpty()) {
                    val generatedId = manager?.getDeviceId()
                    if (!generatedId.isNullOrEmpty()) {
                        prefs.edit().putString(KEY_DEVICE_ID, generatedId).apply()
                        Log.i(TAG, "Saved new device ID: $generatedId")
                    }
                }

                // Register device storage with FFI manager
                // This will load paired devices from storage automatically
                val deviceStorage = DeviceStorageImpl(secureStorage)
                manager?.setDeviceStorage(deviceStorage)
                Log.i(TAG, "Device storage registered with FFI manager")

                val paired = manager?.getPairedDevices() ?: emptyList()
                val connected = manager?.getConnectedDevices() ?: emptyList()
                withContext(Dispatchers.Main) {
                    _pairedDevices.value = paired
                    _connectedDevices.value = connected
                }
            } catch (e: Exception) {
                withContext(Dispatchers.Main) {
                    _lastError.value = "Failed to initialize: ${e.message}"
                }
            }
        }
    }

    fun start() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                manager?.start()
                val running = manager?.isRunning() ?: false
                val paired = manager?.getPairedDevices() ?: emptyList()
                val connected = manager?.getConnectedDevices() ?: emptyList()
                withContext(Dispatchers.Main) {
                    _isRunning.value = running
                    _pairedDevices.value = paired
                    _connectedDevices.value = connected
                }
            } catch (e: NearClipException) {
                withContext(Dispatchers.Main) {
                    _lastError.value = "Start failed: ${e.message}"
                }
            }
        }
    }

    fun stop() {
        viewModelScope.launch(Dispatchers.IO) {
            manager?.stop()
            val paired = manager?.getPairedDevices() ?: emptyList()
            val connected = manager?.getConnectedDevices() ?: emptyList()
            withContext(Dispatchers.Main) {
                _isRunning.value = false
                _pairedDevices.value = paired
                _connectedDevices.value = connected
            }
        }
    }

    fun connectDevice(deviceId: String) {
        Log.i(TAG, "connectDevice() called with deviceId=$deviceId, manager=${manager != null}")
        viewModelScope.launch(Dispatchers.IO) {
            try {
                Log.i(TAG, "Attempting to connect to device: $deviceId")
                manager?.connectDevice(deviceId)
                Log.i(TAG, "connectDevice() completed for $deviceId")
                val paired = manager?.getPairedDevices() ?: emptyList()
                val connected = manager?.getConnectedDevices() ?: emptyList()
                withContext(Dispatchers.Main) {
                    _pairedDevices.value = paired
                    _connectedDevices.value = connected
                }
            } catch (e: NearClipException) {
                withContext(Dispatchers.Main) {
                    _lastError.value = "Connect failed: ${e.message}"
                }
            }
        }
    }

    fun disconnectDevice(deviceId: String) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                manager?.disconnectDevice(deviceId)
                val paired = manager?.getPairedDevices() ?: emptyList()
                val connected = manager?.getConnectedDevices() ?: emptyList()
                withContext(Dispatchers.Main) {
                    _pairedDevices.value = paired
                    _connectedDevices.value = connected
                }
            } catch (e: NearClipException) {
                withContext(Dispatchers.Main) {
                    _lastError.value = "Disconnect failed: ${e.message}"
                }
            }
        }
    }

    fun syncClipboard(content: ByteArray) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                manager?.syncClipboard(content)
            } catch (e: NearClipException) {
                withContext(Dispatchers.Main) {
                    _lastError.value = "Sync failed: ${e.message}"
                }
            }
        }
    }

    /**
     * Add a device from a pairing code (JSON string with QR code data).
     * Uses Rust FFI to parse, validate, and pair with the device.
     * @return the name of the added device
     * @throws IllegalArgumentException if the code is invalid
     * @throws IllegalStateException if the manager is not initialized or pairing fails
     */
    suspend fun addDeviceFromCode(code: String): String {
        // Check device limit first
        if (_pairedDevices.value.size >= MAX_PAIRED_DEVICES) {
            throw IllegalStateException("Maximum $MAX_PAIRED_DEVICES devices reached. Remove a device to add a new one.")
        }

        // Ensure manager is initialized
        val mgr = manager
            ?: throw IllegalStateException("Connection manager not initialized")

        // Run FFI calls on IO dispatcher
        val device = withContext(Dispatchers.IO) {
            try {
                // Use Rust's pairWithQrCode which handles:
                // 1. JSON parsing and validation (including ECDH public key)
                // 2. Device info extraction
                // 3. Adding to memory
                // 4. Attempting connection (WiFi + BLE, with timeout)
                // 5. Saving to storage via FfiDeviceStorage on success
                val pairedDevice = mgr.pairWithQrCode(code)
                Log.i(TAG, "Device paired successfully via QR code: ${pairedDevice.name} (${pairedDevice.id})")
                pairedDevice
            } catch (e: NearClipException) {
                Log.e(TAG, "QR code pairing failed: ${e.message}", e)
                throw IllegalStateException("Failed to pair device: ${e.message}")
            } catch (e: Exception) {
                Log.e(TAG, "Unexpected error during QR code pairing: ${e.message}", e)
                throw IllegalArgumentException("Invalid pairing code: ${e.message}")
            }
        }

        // Refresh devices
        withContext(Dispatchers.IO) {
            val paired = manager?.getPairedDevices() ?: emptyList()
            val connected = manager?.getConnectedDevices() ?: emptyList()
            withContext(Dispatchers.Main) {
                _pairedDevices.value = paired
                _connectedDevices.value = connected
            }
        }

        return device.name
    }

    /**
     * Remove a paired device.
     * Rust layer handles both manager and storage removal via FfiDeviceStorage.
     */
    fun removeDevice(deviceId: String) {
        Log.i(TAG, "removeDevice() called with deviceId=$deviceId")
        viewModelScope.launch(Dispatchers.IO) {
            Log.i(TAG, "removeDevice() coroutine started, manager=${manager != null}")

            // Unpair device (Rust handles both disconnection and storage removal via FfiDeviceStorage)
            try {
                Log.i(TAG, "Calling unpairDevice($deviceId)")
                manager?.unpairDevice(deviceId)
                Log.i(TAG, "Unpaired device: $deviceId")
            } catch (e: Exception) {
                Log.e(TAG, "Failed to unpair device: $deviceId", e)
            }

            // Remove from paused list
            val currentPaused = _pausedDeviceIds.value.toMutableSet()
            if (currentPaused.remove(deviceId)) {
                withContext(Dispatchers.Main) {
                    _pausedDeviceIds.value = currentPaused
                }
                prefs.edit().putStringSet(KEY_PAUSED_DEVICES, currentPaused).apply()
            }

            // Refresh devices
            val paired = manager?.getPairedDevices() ?: emptyList()
            val connected = manager?.getConnectedDevices() ?: emptyList()
            withContext(Dispatchers.Main) {
                _pairedDevices.value = paired
                _connectedDevices.value = connected
            }
        }
    }

    /**
     * Pause syncing for a specific device.
     * Paused devices won't receive clipboard sync.
     */
    fun pauseDevice(deviceId: String) {
        val currentPaused = _pausedDeviceIds.value.toMutableSet()
        currentPaused.add(deviceId)
        _pausedDeviceIds.value = currentPaused
        prefs.edit().putStringSet(KEY_PAUSED_DEVICES, currentPaused).apply()
        Log.i(TAG, "Device paused: $deviceId")
    }

    /**
     * Resume syncing for a specific device.
     */
    fun resumeDevice(deviceId: String) {
        val currentPaused = _pausedDeviceIds.value.toMutableSet()
        currentPaused.remove(deviceId)
        _pausedDeviceIds.value = currentPaused
        prefs.edit().putStringSet(KEY_PAUSED_DEVICES, currentPaused).apply()
        Log.i(TAG, "Device resumed: $deviceId")
    }

    /**
     * Check if a device is paused.
     */
    fun isDevicePaused(deviceId: String): Boolean {
        return _pausedDeviceIds.value.contains(deviceId)
    }

    fun generatePairingCode(): String {
        return try {
            // Use Rust FFI to generate QR code data (includes ECDH public key)
            manager?.generateQrCode() ?: run {
                // Fallback to simple JSON (without public key - INSECURE)
                Log.w(TAG, "Manager not initialized, using fallback pairing code")
                val deviceId = java.util.UUID.randomUUID().toString()
                val json = JSONObject().apply {
                    put("id", deviceId)
                    put("name", "${Build.MANUFACTURER} ${Build.MODEL}")
                    put("platform", "ANDROID")
                }
                json.toString()
            }
        } catch (e: Exception) {
            Log.e(TAG, "Failed to generate QR code: ${e.message}", e)
            // Fallback to simple JSON
            val deviceId = java.util.UUID.randomUUID().toString()
            val json = JSONObject().apply {
                put("id", deviceId)
                put("name", "${Build.MANUFACTURER} ${Build.MODEL}")
                put("platform", "ANDROID")
            }
            json.toString()
        }
    }

    fun refreshDevices() {
        viewModelScope.launch(Dispatchers.IO) {
            val paired = manager?.getPairedDevices() ?: emptyList()
            val connected = manager?.getConnectedDevices() ?: emptyList()
            withContext(Dispatchers.Main) {
                _pairedDevices.value = paired
                _connectedDevices.value = connected
            }
        }
    }

    /**
     * Refresh device state from an external service's manager.
     * Use this when connecting/disconnecting through NearClipService.
     */
    fun refreshFromService(service: com.nearclip.service.NearClipService?) {
        viewModelScope.launch(Dispatchers.IO) {
            val paired = service?.getPairedDevices() ?: emptyList()
            val connected = service?.getConnectedDevices() ?: emptyList()
            withContext(Dispatchers.Main) {
                _pairedDevices.value = paired
                _connectedDevices.value = connected
                _isRunning.value = service?.isRunning() ?: false
            }
        }
    }

    // MARK: - Retry Strategy Execution

    /**
     * Execute the discard strategy - clear pending content.
     */
    fun executeDiscardStrategy() {
        pendingContent = null
        _lastError.value = null
        Log.i(TAG, "Retry strategy: Discarded failed sync content")
    }

    /**
     * Execute the wait for device strategy - queue content for later.
     */
    fun executeWaitForDeviceStrategy(content: ByteArray? = null) {
        if (content != null) {
            pendingContent = content
        }
        Log.i(TAG, "Retry strategy: Content queued, waiting for device reconnection")
    }

    /**
     * Execute the continue retry strategy - retry sync immediately.
     * Note: This requires a callback to be set up by the service.
     */
    var onRetryRequested: (() -> Unit)? = null

    fun executeContinueRetryStrategy() {
        Log.i(TAG, "Retry strategy: Continuing retry")
        onRetryRequested?.invoke()
    }

    /**
     * Apply the default retry strategy for the given content.
     */
    fun applyDefaultRetryStrategy(content: ByteArray) {
        when (defaultRetryStrategy) {
            SyncRetryStrategy.DISCARD -> executeDiscardStrategy()
            SyncRetryStrategy.WAIT_FOR_DEVICE -> executeWaitForDeviceStrategy(content)
            SyncRetryStrategy.CONTINUE_RETRY -> executeContinueRetryStrategy()
        }
    }

    /**
     * Send pending content when a device reconnects.
     */
    private fun sendPendingContentIfNeeded() {
        val content = pendingContent ?: return
        if (_connectedDevices.value.isEmpty()) return

        Log.i(TAG, "Sending pending content to reconnected device(s)")
        viewModelScope.launch(Dispatchers.IO) {
            try {
                manager?.syncClipboard(content)
                // Only clear pending content on successful sync
                pendingContent = null
                Log.i(TAG, "Pending content sent successfully: ${content.size} bytes")
            } catch (e: NearClipException) {
                // Keep pending content for next attempt
                Log.w(TAG, "Failed to send pending content, will retry later: ${e.message}")
            }
        }
    }

    // FfiNearClipCallback implementation

    override fun onDeviceConnected(device: FfiDeviceInfo) {
        refreshDevices()
        // Send pending content if using "wait for device" strategy
        sendPendingContentIfNeeded()
    }

    override fun onDeviceDisconnected(deviceId: String) {
        refreshDevices()
    }

    override fun onDeviceUnpaired(deviceId: String) {
        Log.d(TAG, "Device unpaired by remote: $deviceId")
        // Note: Storage removal is handled by Rust layer via FfiDeviceStorage
        // Refresh device lists
        refreshDevices()
    }

    override fun onPairingRejected(deviceId: String, reason: String) {
        Log.w(TAG, "Pairing rejected by device: $deviceId, reason: $reason")
        // Remove from FFI manager (this will also remove from storage via FfiDeviceStorage)
        manager?.removePairedDevice(deviceId)
        Log.d(TAG, "Removed rejected device $deviceId")
        // Refresh device lists
        refreshDevices()
    }

    override fun onClipboardReceived(content: ByteArray, fromDevice: String) {
        _lastReceivedClipboard.value = Pair(content, fromDevice)
    }

    override fun onSyncError(errorMessage: String) {
        _lastError.value = errorMessage
    }

    override fun onDeviceDiscovered(device: FfiDiscoveredDevice) {
        Log.d(TAG, "Device discovered: ${device.peripheralUuid}, name=${device.deviceName}")
        // This is called by the Rust BleController when a device is discovered via BLE
        // We could auto-connect to paired devices here if needed
    }

    override fun onDeviceLost(peripheralUuid: String) {
        Log.d(TAG, "Device lost: $peripheralUuid")
        // Device is no longer visible via BLE scanning
    }

    override fun onCleared() {
        super.onCleared()
        manager?.stop()
        manager?.destroy()
    }
}
