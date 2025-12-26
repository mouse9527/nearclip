package com.nearclip.data

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.nearclip.ffi.DevicePlatform
import com.nearclip.ffi.DeviceStatus
import com.nearclip.ffi.FfiDeviceInfo
import com.nearclip.ffi.FfiDeviceStorage
import org.json.JSONArray
import org.json.JSONObject

/**
 * Secure storage for sensitive data using Android Keystore.
 * Uses EncryptedSharedPreferences backed by AES-256 GCM encryption.
 */
class SecureStorage(private val context: Context) {

    private val masterKey: MasterKey by lazy {
        MasterKey.Builder(context)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()
    }

    private val encryptedPrefs: SharedPreferences by lazy {
        EncryptedSharedPreferences.create(
            context,
            PREFS_FILE_NAME,
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
    }

    companion object {
        private const val TAG = "SecureStorage"
        private const val PREFS_FILE_NAME = "nearclip_secure_prefs"
        private const val KEY_PAIRED_DEVICES = "paired_devices"
        private const val KEY_DEVICE_KEYS = "device_keys"
        private const val KEY_DATA_VERSION = "data_version"
        private const val CURRENT_DATA_VERSION = 1
    }

    /**
     * Result wrapper for storage operations.
     */
    sealed class StorageResult<out T> {
        data class Success<T>(val data: T) : StorageResult<T>()
        data class Error(val message: String, val exception: Exception? = null) : StorageResult<Nothing>()
    }

    init {
        migrateDataIfNeeded()
    }

    /**
     * Migrate data to current version if needed.
     */
    private fun migrateDataIfNeeded() {
        val currentVersion = encryptedPrefs.getInt(KEY_DATA_VERSION, 0)
        if (currentVersion < CURRENT_DATA_VERSION) {
            Log.i(TAG, "Migrating data from version $currentVersion to $CURRENT_DATA_VERSION")
            // Future migrations go here
            // when (currentVersion) {
            //     0 -> migrateV0ToV1()
            //     1 -> migrateV1ToV2()
            // }
            encryptedPrefs.edit()
                .putInt(KEY_DATA_VERSION, CURRENT_DATA_VERSION)
                .apply()
            Log.i(TAG, "Data migration completed")
        }
    }

    /**
     * Save paired devices securely.
     */
    fun savePairedDevices(devices: List<FfiDeviceInfo>) {
        val jsonArray = JSONArray()
        for (device in devices) {
            val obj = JSONObject().apply {
                put("id", device.id)
                put("name", device.name)
                put("platform", device.platform.name)
            }
            jsonArray.put(obj)
        }
        encryptedPrefs.edit()
            .putString(KEY_PAIRED_DEVICES, jsonArray.toString())
            .apply()
    }

    /**
     * Load paired devices from secure storage.
     * Returns StorageResult to properly handle errors.
     */
    fun loadPairedDevicesResult(): StorageResult<List<FfiDeviceInfo>> {
        val json = encryptedPrefs.getString(KEY_PAIRED_DEVICES, null)
            ?: return StorageResult.Success(emptyList())
        return try {
            val jsonArray = JSONArray(json)
            val devices = mutableListOf<FfiDeviceInfo>()
            for (i in 0 until jsonArray.length()) {
                val obj = jsonArray.getJSONObject(i)
                val platformStr = obj.getString("platform")
                val platform = try {
                    DevicePlatform.valueOf(platformStr)
                } catch (e: IllegalArgumentException) {
                    Log.w(TAG, "Unknown platform '$platformStr' for device, skipping")
                    continue
                }
                devices.add(
                    FfiDeviceInfo(
                        id = obj.getString("id"),
                        name = obj.getString("name"),
                        platform = platform,
                        status = DeviceStatus.DISCONNECTED
                    )
                )
            }
            StorageResult.Success(devices)
        } catch (e: Exception) {
            Log.e(TAG, "Failed to load paired devices", e)
            StorageResult.Error("Failed to load paired devices: ${e.message}", e)
        }
    }

    /**
     * Load paired devices from secure storage.
     * Returns empty list on error (for backward compatibility).
     */
    fun loadPairedDevices(): List<FfiDeviceInfo> {
        return when (val result = loadPairedDevicesResult()) {
            is StorageResult.Success -> result.data
            is StorageResult.Error -> {
                Log.w(TAG, "loadPairedDevices returning empty list due to error: ${result.message}")
                emptyList()
            }
        }
    }

    /**
     * Add a single paired device.
     */
    fun addPairedDevice(device: FfiDeviceInfo) {
        val devices = loadPairedDevices().toMutableList()
        // Remove if exists (update)
        devices.removeAll { it.id == device.id }
        devices.add(device)
        savePairedDevices(devices)
    }

    /**
     * Remove a paired device.
     */
    fun removePairedDevice(deviceId: String) {
        val devices = loadPairedDevices().toMutableList()
        devices.removeAll { it.id == deviceId }
        savePairedDevices(devices)
        // Also remove any stored keys for this device
        removeDeviceKeys(deviceId)
    }

    /**
     * Store encryption keys for a device.
     */
    fun saveDeviceKeys(deviceId: String, publicKey: ByteArray, privateKey: ByteArray? = null) {
        val keysJson = encryptedPrefs.getString(KEY_DEVICE_KEYS, null) ?: "{}"
        val keysObj = JSONObject(keysJson)

        val deviceKeysObj = JSONObject().apply {
            put("publicKey", android.util.Base64.encodeToString(publicKey, android.util.Base64.NO_WRAP))
            privateKey?.let {
                put("privateKey", android.util.Base64.encodeToString(it, android.util.Base64.NO_WRAP))
            }
        }
        keysObj.put(deviceId, deviceKeysObj)

        encryptedPrefs.edit()
            .putString(KEY_DEVICE_KEYS, keysObj.toString())
            .apply()
    }

    /**
     * Load encryption keys for a device.
     * Returns StorageResult to properly handle errors.
     */
    fun loadDeviceKeysResult(deviceId: String): StorageResult<Pair<ByteArray?, ByteArray?>> {
        val keysJson = encryptedPrefs.getString(KEY_DEVICE_KEYS, null)
            ?: return StorageResult.Success(Pair(null, null))
        return try {
            val keysObj = JSONObject(keysJson)
            if (!keysObj.has(deviceId)) return StorageResult.Success(Pair(null, null))

            val deviceKeysObj = keysObj.getJSONObject(deviceId)
            val publicKeyStr = deviceKeysObj.optString("publicKey", "")
            val privateKeyStr = deviceKeysObj.optString("privateKey", "")

            val publicKey = if (publicKeyStr.isNotEmpty()) {
                android.util.Base64.decode(publicKeyStr, android.util.Base64.NO_WRAP)
            } else null

            val privateKey = if (privateKeyStr.isNotEmpty()) {
                android.util.Base64.decode(privateKeyStr, android.util.Base64.NO_WRAP)
            } else null

            StorageResult.Success(Pair(publicKey, privateKey))
        } catch (e: Exception) {
            Log.e(TAG, "Failed to load device keys for $deviceId", e)
            StorageResult.Error("Failed to load device keys: ${e.message}", e)
        }
    }

    /**
     * Load encryption keys for a device.
     * Returns null pair on error (for backward compatibility).
     */
    fun loadDeviceKeys(deviceId: String): Pair<ByteArray?, ByteArray?> {
        return when (val result = loadDeviceKeysResult(deviceId)) {
            is StorageResult.Success -> result.data
            is StorageResult.Error -> {
                Log.w(TAG, "loadDeviceKeys returning null due to error: ${result.message}")
                Pair(null, null)
            }
        }
    }

    /**
     * Remove stored keys for a device.
     */
    private fun removeDeviceKeys(deviceId: String) {
        val keysJson = encryptedPrefs.getString(KEY_DEVICE_KEYS, null) ?: return
        try {
            val keysObj = JSONObject(keysJson)
            keysObj.remove(deviceId)
            encryptedPrefs.edit()
                .putString(KEY_DEVICE_KEYS, keysObj.toString())
                .apply()
            Log.d(TAG, "Removed keys for device: $deviceId")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to remove device keys for $deviceId", e)
        }
    }

    /**
     * Clear all secure storage.
     */
    fun clearAll() {
        encryptedPrefs.edit().clear().apply()
    }
}

/**
 * Implements FfiDeviceStorage interface for Rust FFI.
 * This allows Rust layer to control when devices are saved/loaded/removed.
 *
 * The dependency inversion pattern:
 * - Rust layer decides WHEN to save/load/delete devices
 * - This class implements HOW (using SecureStorage/EncryptedSharedPreferences)
 */
class DeviceStorageImpl(private val secureStorage: SecureStorage) : FfiDeviceStorage {

    companion object {
        private const val TAG = "DeviceStorageImpl"
    }

    /**
     * Save a paired device to persistent storage.
     * Called by Rust when a device is successfully paired and connected.
     */
    override fun saveDevice(device: FfiDeviceInfo) {
        secureStorage.addPairedDevice(device)
        Log.i(TAG, "Saved device '${device.name}' (${device.id})")
    }

    /**
     * Remove a paired device from persistent storage.
     * Called by Rust when a device is unpaired.
     */
    override fun removeDevice(deviceId: String) {
        secureStorage.removePairedDevice(deviceId)
        Log.i(TAG, "Removed device '$deviceId'")
    }

    /**
     * Load all paired devices from persistent storage.
     * Called by Rust during initialization.
     */
    override fun loadAllDevices(): List<FfiDeviceInfo> {
        val devices = secureStorage.loadPairedDevices()
        Log.i(TAG, "Loaded ${devices.size} devices")
        return devices
    }
}
