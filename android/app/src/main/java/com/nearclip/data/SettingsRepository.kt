package com.nearclip.data

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.*
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import java.io.IOException

// Extension to create DataStore
val Context.settingsDataStore: DataStore<Preferences> by preferencesDataStore(name = "nearclip_settings")

/**
 * User-configurable settings for NearClip.
 */
data class NearClipSettings(
    val wifiEnabled: Boolean = true,
    val bleEnabled: Boolean = true,
    val autoConnect: Boolean = true,
    val syncNotifications: Boolean = true
)

/**
 * Repository for managing NearClip settings using DataStore.
 */
class SettingsRepository(private val dataStore: DataStore<Preferences>) {

    private object PreferencesKeys {
        val WIFI_ENABLED = booleanPreferencesKey("wifi_enabled")
        val BLE_ENABLED = booleanPreferencesKey("ble_enabled")
        val AUTO_CONNECT = booleanPreferencesKey("auto_connect")
        val SYNC_NOTIFICATIONS = booleanPreferencesKey("sync_notifications")
    }

    /**
     * Flow of current settings.
     */
    val settings: Flow<NearClipSettings> = dataStore.data
        .catch { exception ->
            if (exception is IOException) {
                emit(emptyPreferences())
            } else {
                throw exception
            }
        }
        .map { preferences ->
            NearClipSettings(
                wifiEnabled = preferences[PreferencesKeys.WIFI_ENABLED] ?: true,
                bleEnabled = preferences[PreferencesKeys.BLE_ENABLED] ?: true,
                autoConnect = preferences[PreferencesKeys.AUTO_CONNECT] ?: true,
                syncNotifications = preferences[PreferencesKeys.SYNC_NOTIFICATIONS] ?: true
            )
        }

    /**
     * Update WiFi sync enabled setting.
     */
    suspend fun setWifiEnabled(enabled: Boolean) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.WIFI_ENABLED] = enabled
        }
    }

    /**
     * Update BLE sync enabled setting.
     */
    suspend fun setBleEnabled(enabled: Boolean) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.BLE_ENABLED] = enabled
        }
    }

    /**
     * Update auto-connect setting.
     */
    suspend fun setAutoConnect(enabled: Boolean) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.AUTO_CONNECT] = enabled
        }
    }

    /**
     * Update sync notifications setting.
     */
    suspend fun setSyncNotifications(enabled: Boolean) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.SYNC_NOTIFICATIONS] = enabled
        }
    }

    /**
     * Update all settings at once.
     */
    suspend fun updateSettings(settings: NearClipSettings) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.WIFI_ENABLED] = settings.wifiEnabled
            preferences[PreferencesKeys.BLE_ENABLED] = settings.bleEnabled
            preferences[PreferencesKeys.AUTO_CONNECT] = settings.autoConnect
            preferences[PreferencesKeys.SYNC_NOTIFICATIONS] = settings.syncNotifications
        }
    }
}
