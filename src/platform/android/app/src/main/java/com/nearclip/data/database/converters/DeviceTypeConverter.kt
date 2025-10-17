package com.nearclip.data.database.converters

import androidx.room.TypeConverter
import com.nearclip.data.model.DeviceType

/**
 * 设备类型转换器
 */
class DeviceTypeConverter {
    @TypeConverter
    fun fromDeviceType(deviceType: DeviceType): String {
        return deviceType.value
    }

    @TypeConverter
    fun toDeviceType(value: String): DeviceType {
        return DeviceType.fromString(value)
    }
}