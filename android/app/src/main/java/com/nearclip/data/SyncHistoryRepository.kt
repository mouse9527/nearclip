package com.nearclip.data

import com.nearclip.ffi.FfiNearClipManager
import com.nearclip.ffi.FfiSyncHistoryEntry
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.text.SimpleDateFormat
import java.util.*

/**
 * Represents a single sync event in history.
 * View model wrapper for FfiSyncHistoryEntry.
 */
data class SyncRecord(
    val id: Long,
    val timestamp: Long,
    val direction: SyncDirection,
    val deviceId: String,
    val deviceName: String,
    val contentPreview: String,
    val contentSize: ULong,
    val success: Boolean,
    val errorMessage: String? = null
) {
    companion object {
        fun fromFfi(entry: FfiSyncHistoryEntry): SyncRecord = SyncRecord(
            id = entry.id,
            timestamp = entry.timestampMs,
            direction = if (entry.direction == "sent") SyncDirection.SENT else SyncDirection.RECEIVED,
            deviceId = entry.deviceId,
            deviceName = entry.deviceName,
            contentPreview = entry.contentPreview,
            contentSize = entry.contentSize,
            success = entry.success,
            errorMessage = entry.errorMessage
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
 * Repository for managing sync history - delegates to FFI layer.
 */
class SyncHistoryRepository {

    private var manager: FfiNearClipManager? = null
    private val _syncHistory = MutableStateFlow<List<SyncRecord>>(emptyList())

    /**
     * Flow of all sync records, sorted by timestamp (newest first).
     */
    val syncHistory: Flow<List<SyncRecord>> = _syncHistory.asStateFlow()

    /**
     * Set the FFI manager reference.
     */
    fun setManager(manager: FfiNearClipManager) {
        this.manager = manager
        loadHistory()
    }

    /**
     * Reload history from FFI layer.
     */
    fun loadHistory() {
        val mgr = manager ?: return
        try {
            val entries = mgr.getRecentHistory(50u)
            val records = entries.map { SyncRecord.fromFfi(it) }
            _syncHistory.value = records
            android.util.Log.d("SyncHistoryRepository", "Loaded ${records.size} records from FFI")
        } catch (e: Exception) {
            android.util.Log.e("SyncHistoryRepository", "Failed to load history: ${e.message}")
        }
    }

    /**
     * Add a new sync record to history.
     */
    private fun addEntry(entry: FfiSyncHistoryEntry) {
        val mgr = manager
        if (mgr == null) {
            android.util.Log.w("SyncHistoryRepository", "No manager set, cannot add entry")
            return
        }

        try {
            mgr.addHistoryEntry(entry)
            // Reload to get the updated list with proper IDs
            loadHistory()
            android.util.Log.d("SyncHistoryRepository", "Added record - ${entry.direction} ${if (entry.success) "success" else "failed"}")
        } catch (e: Exception) {
            android.util.Log.e("SyncHistoryRepository", "Failed to add entry: ${e.message}")
        }
    }

    /**
     * Clear all sync history.
     */
    fun clearHistory() {
        val mgr = manager
        if (mgr == null) {
            android.util.Log.w("SyncHistoryRepository", "No manager set, cannot clear history")
            return
        }

        try {
            mgr.clearAllHistory()
            _syncHistory.value = emptyList()
            android.util.Log.d("SyncHistoryRepository", "Cleared all history")
        } catch (e: Exception) {
            android.util.Log.e("SyncHistoryRepository", "Failed to clear history: ${e.message}")
        }
    }

    /**
     * Record a successful send operation.
     */
    fun recordSent(
        deviceId: String,
        deviceName: String,
        content: ByteArray
    ) {
        val preview = try {
            String(content, Charsets.UTF_8).take(100)
        } catch (e: Exception) {
            "[Binary data]"
        }

        val entry = FfiSyncHistoryEntry(
            id = 0,  // Will be assigned by FFI
            deviceId = deviceId,
            deviceName = deviceName,
            contentPreview = preview,
            contentSize = content.size.toULong(),
            direction = "sent",
            timestampMs = System.currentTimeMillis(),
            success = true,
            errorMessage = null
        )

        addEntry(entry)
    }

    /**
     * Record a successful receive operation.
     */
    fun recordReceived(
        deviceId: String,
        deviceName: String,
        content: ByteArray
    ) {
        val preview = try {
            String(content, Charsets.UTF_8).take(100)
        } catch (e: Exception) {
            "[Binary data]"
        }

        val entry = FfiSyncHistoryEntry(
            id = 0,
            deviceId = deviceId,
            deviceName = deviceName,
            contentPreview = preview,
            contentSize = content.size.toULong(),
            direction = "received",
            timestampMs = System.currentTimeMillis(),
            success = true,
            errorMessage = null
        )

        addEntry(entry)
    }

    /**
     * Record a failed sync operation.
     */
    fun recordError(
        direction: SyncDirection,
        deviceId: String,
        deviceName: String,
        errorMessage: String
    ) {
        val entry = FfiSyncHistoryEntry(
            id = 0,
            deviceId = deviceId,
            deviceName = deviceName,
            contentPreview = "",
            contentSize = 0u,
            direction = if (direction == SyncDirection.SENT) "sent" else "received",
            timestampMs = System.currentTimeMillis(),
            success = false,
            errorMessage = errorMessage
        )

        addEntry(entry)
    }
}
