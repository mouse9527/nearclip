package com.nearclip.data

import android.content.Context
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.nearclip.ffi.DevicePlatform
import com.nearclip.ffi.DeviceStatus
import com.nearclip.ffi.FfiDeviceInfo
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
        private const val PREFS_FILE_NAME = "nearclip_secure_prefs"
        private const val KEY_PAIRED_DEVICES = "paired_devices"
        private const val KEY_DEVICE_KEYS = "device_keys"
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
     */
    fun loadPairedDevices(): List<FfiDeviceInfo> {
        val json = encryptedPrefs.getString(KEY_PAIRED_DEVICES, null) ?: return emptyList()
        return try {
            val jsonArray = JSONArray(json)
            val devices = mutableListOf<FfiDeviceInfo>()
            for (i in 0 until jsonArray.length()) {
                val obj = jsonArray.getJSONObject(i)
                devices.add(
                    FfiDeviceInfo(
                        id = obj.getString("id"),
                        name = obj.getString("name"),
                        platform = DevicePlatform.valueOf(obj.getString("platform")),
                        status = DeviceStatus.DISCONNECTED
                    )
                )
            }
            devices
        } catch (e: Exception) {
            e.printStackTrace()
            emptyList()
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
     */
    fun loadDeviceKeys(deviceId: String): Pair<ByteArray?, ByteArray?> {
        val keysJson = encryptedPrefs.getString(KEY_DEVICE_KEYS, null) ?: return Pair(null, null)
        return try {
            val keysObj = JSONObject(keysJson)
            if (!keysObj.has(deviceId)) return Pair(null, null)

            val deviceKeysObj = keysObj.getJSONObject(deviceId)
            val publicKey = deviceKeysObj.optString("publicKey", null)?.let {
                android.util.Base64.decode(it, android.util.Base64.NO_WRAP)
            }
            val privateKey = deviceKeysObj.optString("privateKey", null)?.let {
                android.util.Base64.decode(it, android.util.Base64.NO_WRAP)
            }
            Pair(publicKey, privateKey)
        } catch (e: Exception) {
            e.printStackTrace()
            Pair(null, null)
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
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    /**
     * Clear all secure storage.
     */
    fun clearAll() {
        encryptedPrefs.edit().clear().apply()
    }
}
