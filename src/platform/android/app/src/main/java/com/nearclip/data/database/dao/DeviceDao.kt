package com.nearclip.data.database.dao

import androidx.room.*
import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow

/**
 * 设备数据访问对象
 */
@Dao
interface DeviceDao {

    /**
     * 获取所有设备
     */
    @Query("SELECT * FROM devices ORDER BY lastSeen DESC")
    fun getAllDevices(): Flow<List<Device>>

    /**
     * 根据ID获取设备
     */
    @Query("SELECT * FROM devices WHERE deviceId = :deviceId")
    suspend fun getDeviceById(deviceId: String): Device?

    /**
     * 插入新设备
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertDevice(device: Device): Long

    /**
     * 更新设备信息
     */
    @Update
    suspend fun updateDevice(device: Device)

    /**
     * 删除设备
     */
    @Delete
    suspend fun deleteDevice(device: Device)

    /**
     * 根据ID删除设备
     */
    @Query("DELETE FROM devices WHERE deviceId = :deviceId")
    suspend fun deleteDeviceById(deviceId: String)

    /**
     * 获取已连接的设备
     */
    @Query("SELECT * FROM devices WHERE connectionStatus = 'CONNECTED' ORDER BY lastSeen DESC")
    fun getConnectedDevices(): Flow<List<Device>>

    /**
     * 根据设备类型获取设备
     */
    @Query("SELECT * FROM devices WHERE deviceType = :deviceType ORDER BY lastSeen DESC")
    fun getDevicesByType(deviceType: String): Flow<List<Device>>

    /**
     * 更新设备连接状态
     */
    @Query("UPDATE devices SET connectionStatus = :connectionStatus, updatedAt = :timestamp WHERE deviceId = :deviceId")
    suspend fun updateDeviceConnectionStatus(deviceId: String, connectionStatus: String, timestamp: Long)

    /**
     * 更新设备最后连接时间
     */
    @Query("UPDATE devices SET lastSeen = :lastSeen, updatedAt = :timestamp WHERE deviceId = :deviceId")
    suspend fun updateDeviceLastSeen(deviceId: String, lastSeen: Long, timestamp: Long)

    /**
     * 获取已配对的设备
     */
    @Query("SELECT * FROM devices WHERE isPaired = 1 ORDER BY lastSeen DESC")
    fun getPairedDevices(): Flow<List<Device>>

    /**
     * 设置设备配对状态
     */
    @Query("UPDATE devices SET isPaired = :isPaired, updatedAt = :timestamp WHERE deviceId = :deviceId")
    suspend fun setDevicePaired(deviceId: String, isPaired: Boolean, timestamp: Long)

    /**
     * 获取最近连接的设备
     */
    @Query("SELECT * FROM devices WHERE connectionStatus = 'CONNECTED' ORDER BY lastSeen DESC LIMIT 1")
    suspend fun getLastConnectedDevice(): Device?

    /**
     * 删除所有设备
     */
    @Query("DELETE FROM devices")
    suspend fun deleteAllDevices()

    /**
     * 获取设备总数
     */
    @Query("SELECT COUNT(*) FROM devices")
    suspend fun getDeviceCount(): Int
}