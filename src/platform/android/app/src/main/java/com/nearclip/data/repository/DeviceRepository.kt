package com.nearclip.data.repository

import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow

/**
 * 设备数据仓库接口
 * 负责管理设备数据的CRUD操作
 */
interface DeviceRepository {
    /**
     * 获取所有设备
     */
    fun getAllDevices(): Flow<List<Device>>

    /**
     * 根据ID获取设备
     */
    suspend fun getDeviceById(deviceId: String): Device?

    /**
     * 添加新设备
     */
    suspend fun insertDevice(device: Device)

    /**
     * 更新设备信息
     */
    suspend fun updateDevice(device: Device)

    /**
     * 删除设备
     */
    suspend fun deleteDevice(deviceId: String)

    /**
     * 获取已连接的设备
     */
    fun getConnectedDevices(): Flow<List<Device>>

    /**
     * 根据设备类型获取设备
     */
    fun getDevicesByType(deviceType: Device.DeviceType): Flow<List<Device>>

    /**
     * 更新设备连接状态
     */
    suspend fun updateDeviceConnectionStatus(deviceId: String, isConnected: Boolean)

    /**
     * 更新设备最后连接时间
     */
    suspend fun updateDeviceLastSeen(deviceId: String, timestamp: Long)
}