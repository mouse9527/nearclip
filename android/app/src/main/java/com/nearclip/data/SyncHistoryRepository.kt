package com.nearclip.data

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import org.json.JSONArray
import org.json.JSONObject
import java.text.SimpleDateFormat
import java.util.*

private val Context.syncHistoryDataStore: DataStore<Preferences> by preferencesDataStore(name = "sync_history")

/**
 * Represents a single sync event in history.
 */
data class SyncRecord(
    val id: String = UUID.randomUUID().toString(),
    val timestamp: Long = System.currentTimeMillis(),
    val direction: SyncDirection,
    val deviceId: String,
    val deviceName: String,
    val contentPreview: String,
    val contentSize: Int,
    val success: Boolean,
    val errorMessage: String? = null
) {
    fun toJson(): JSONObject = JSONObject().apply {
        put("id", id)
        put("timestamp", timestamp)
        put("direction", direction.name)
        put("deviceId", deviceId)
        put("deviceName", deviceName)
        put("contentPreview", contentPreview)
        put("contentSize", contentSize)
        put("success", success)
        put("errorMessage", errorMessage ?: JSONObject.NULL)
    }

    companion object {
        fun fromJson(json: JSONObject): SyncRecord = SyncRecord(
            id = json.optString("id", UUID.randomUUID().toString()),
            timestamp = json.optLong("timestamp", System.currentTimeMillis()),
            direction = try {
                SyncDirection.valueOf(json.optString("direction", "RECEIVED"))
            } catch (e: Exception) {
                SyncDirection.RECEIVED
            },
            deviceId = json.optString("deviceId", ""),
            deviceName = json.optString("deviceName", "Unknown"),
            contentPreview = json.optString("contentPreview", ""),
            contentSize = json.optInt("contentSize", 0),
            success = json.optBoolean("success", true),
            errorMessage = if (json.isNull("errorMessage")) null else json.optString("errorMessage")
        )
    }

    /**
     * Format timestamp as relative time string (e.g., "2 minutes ago")
     */
    fun getRelativeTime(): String {
        val now = System.currentTimeMillis()
        val diff = now - timestamp

        return when {
            diff < 60_000 -> "刚刚"
            diff < 3600_000 -> "${diff / 60_000} 分钟前"
            diff < 86400_000 -> "${diff / 3600_000} 小时前"
            diff < 604800_000 -> "${diff / 86400_000} 天前"
            else -> {
                val sdf = SimpleDateFormat("MM-dd HH:mm", Locale.getDefault())
                sdf.format(Date(timestamp))
            }
        }
    }
}

enum class SyncDirection {
    SENT,      // Sent to other device
    RECEIVED   // Received from other device
}

/**
 * Repository for managing sync history.
 */
class SyncHistoryRepository(private val context: Context) {

    companion object {
        private val SYNC_HISTORY_KEY = stringPreferencesKey("sync_history_json")
        private const val MAX_HISTORY_SIZE = 50  // Keep last 50 records
    }

    /**
     * Flow of all sync records, sorted by timestamp (newest first).
     */
    val syncHistory: Flow<List<SyncRecord>> = context.syncHistoryDataStore.data.map { prefs ->
        val json = prefs[SYNC_HISTORY_KEY] ?: "[]"
        parseHistory(json)
    }

    /**
     * Add a new sync record to history.
     */
    suspend fun addRecord(record: SyncRecord) {
        context.syncHistoryDataStore.edit { prefs ->
            val currentJson = prefs[SYNC_HISTORY_KEY] ?: "[]"
            val records = parseHistory(currentJson).toMutableList()

            // Add new record at the beginning
            records.add(0, record)

            // Trim to max size
            while (records.size > MAX_HISTORY_SIZE) {
                records.removeAt(records.size - 1)
            }

            prefs[SYNC_HISTORY_KEY] = serializeHistory(records)
        }
    }

    /**
     * Clear all sync history.
     */
    suspend fun clearHistory() {
        context.syncHistoryDataStore.edit { prefs ->
            prefs[SYNC_HISTORY_KEY] = "[]"
        }
    }

    /**
     * Record a successful send operation.
     */
    suspend fun recordSent(
        deviceId: String,
        deviceName: String,
        content: ByteArray
    ) {
        val preview = try {
            String(content, Charsets.UTF_8).take(100)
        } catch (e: Exception) {
            "[Binary data]"
        }

        addRecord(
            SyncRecord(
                direction = SyncDirection.SENT,
                deviceId = deviceId,
                deviceName = deviceName,
                contentPreview = preview,
                contentSize = content.size,
                success = true
            )
        )
    }

    /**
     * Record a successful receive operation.
     */
    suspend fun recordReceived(
        deviceId: String,
        deviceName: String,
        content: ByteArray
    ) {
        val preview = try {
            String(content, Charsets.UTF_8).take(100)
        } catch (e: Exception) {
            "[Binary data]"
        }

        addRecord(
            SyncRecord(
                direction = SyncDirection.RECEIVED,
                deviceId = deviceId,
                deviceName = deviceName,
                contentPreview = preview,
                contentSize = content.size,
                success = true
            )
        )
    }

    /**
     * Record a failed sync operation.
     */
    suspend fun recordError(
        direction: SyncDirection,
        deviceId: String,
        deviceName: String,
        errorMessage: String
    ) {
        addRecord(
            SyncRecord(
                direction = direction,
                deviceId = deviceId,
                deviceName = deviceName,
                contentPreview = "",
                contentSize = 0,
                success = false,
                errorMessage = errorMessage
            )
        )
    }

    private fun parseHistory(json: String): List<SyncRecord> {
        return try {
            val array = JSONArray(json)
            (0 until array.length()).map { i ->
                SyncRecord.fromJson(array.getJSONObject(i))
            }
        } catch (e: Exception) {
            android.util.Log.e("SyncHistoryRepository", "Failed to parse history: ${e.message}")
            emptyList()
        }
    }

    private fun serializeHistory(records: List<SyncRecord>): String {
        val array = JSONArray()
        records.forEach { record ->
            array.put(record.toJson())
        }
        return array.toString()
    }
}
