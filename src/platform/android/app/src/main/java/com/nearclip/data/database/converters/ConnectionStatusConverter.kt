package com.nearclip.data.database.converters

import androidx.room.TypeConverter
import com.nearclip.data.model.ConnectionStatus

/**
 * 连接状态转换器
 */
class ConnectionStatusConverter {
    @TypeConverter
    fun fromConnectionStatus(connectionStatus: ConnectionStatus): String {
        return connectionStatus.value
    }

    @TypeConverter
    fun toConnectionStatus(value: String): ConnectionStatus {
        return ConnectionStatus.fromString(value)
    }
}