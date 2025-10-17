package com.nearclip.data.repository

import com.nearclip.data.database.NearClipDatabase
import com.nearclip.data.database.dao.DeviceDao
import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow
import javax.inject.Inject
import javax.inject.Singleton

/**
 * 设备数据仓库实现
 */
@Singleton
class DeviceRepositoryImpl @Inject constructor(
    private val database: NearClipDatabase,
    private val deviceDao: DeviceDao
) : DeviceRepository {

    override fun getAllDevices(): Flow<List<Device>> {
        return deviceDao.getAllDevices()
    }

    override suspend fun getDeviceById(deviceId: String): Device? {
        return deviceDao.getDeviceById(deviceId)
    }

    override suspend fun insertDevice(device: Device) {
        deviceDao.insertDevice(device)
    }

    override suspend fun updateDevice(device: Device) {
        deviceDao.updateDevice(device)
    }

    override suspend fun deleteDevice(deviceId: String) {
        deviceDao.deleteDeviceById(deviceId)
    }

    override fun getConnectedDevices(): Flow<List<Device>> {
        return deviceDao.getConnectedDevices()
    }

    override fun getDevicesByType(deviceType: Device.DeviceType): Flow<List<Device>> {
        return deviceDao.getDevicesByType(deviceType.value)
    }

    override suspend fun updateDeviceConnectionStatus(deviceId: String, isConnected: Boolean) {
        val status = if (isConnected) "CONNECTED" else "DISCONNECTED"
        val timestamp = System.currentTimeMillis()
        deviceDao.updateDeviceConnectionStatus(deviceId, status, timestamp)
    }

    override suspend fun updateDeviceLastSeen(deviceId: String, timestamp: Long) {
        deviceDao.updateDeviceLastSeen(deviceId, timestamp, System.currentTimeMillis())
    }
}