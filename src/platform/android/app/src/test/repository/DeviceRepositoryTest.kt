package com.nearclip.data.repository

import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.nearclip.data.database.NearClipDatabase
import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.model.DeviceType
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

@ExperimentalCoroutinesApi
@RunWith(AndroidJUnit4::class)
class DeviceRepositoryTest {

    private lateinit var database: NearClipDatabase
    private lateinit var deviceDao: com.nearclip.data.database.dao.DeviceDao
    private lateinit var repository: DeviceRepositoryImpl

    // 测试数据
    private val testDevice = Device(
        deviceId = "test-device-1",
        deviceName = "Test Device",
        deviceType = DeviceType.ANDROID,
        publicKey = "test-public-key",
        lastSeen = System.currentTimeMillis(),
        connectionStatus = ConnectionStatus.DISCONNECTED
    )

    @Before
    fun setup() {
        // 创建内存数据库用于测试
        database = Room.inMemoryDatabaseBuilder(
            ApplicationProvider.getApplicationContext(),
            NearClipDatabase::class.java
        ).allowMainThreadQueries().build()

        deviceDao = database.deviceDao()
        repository = DeviceRepositoryImpl(deviceDao)
    }

    @After
    fun tearDown() {
        database.close()
    }

    @Test
    fun `insertDevice should add device to database`() = runTest {
        // When
        repository.insertDevice(testDevice)

        // Then
        val devices = repository.getAllDevices().first()
        assertEquals(1, devices.size)
        assertEquals(testDevice.deviceId, devices.first().deviceId)
    }

    @Test
    fun `getDeviceById should return correct device`() = runTest {
        // Given
        repository.insertDevice(testDevice)

        // When
        val device = repository.getDeviceById("test-device-1")

        // Then
        assertNotNull(device)
        assertEquals("Test Device", device?.deviceName)
    }

    @Test
    fun `getDeviceById should return null for non-existent device`() = runTest {
        // When
        val device = repository.getDeviceById("non-existent-id")

        // Then
        assertEquals(null, device)
    }

    @Test
    fun `updateDeviceConnectionStatus should update connection status`() = runTest {
        // Given
        repository.insertDevice(testDevice)

        // When
        repository.updateDeviceConnectionStatus("test-device-1", true)

        // Then
        val connectedDevices = repository.getConnectedDevices().first()
        assertEquals(1, connectedDevices.size)
        assertTrue(connectedDevices.first().connectionStatus == ConnectionStatus.CONNECTED)
    }

    @Test
    fun `deleteDevice should remove device from database`() = runTest {
        // Given
        repository.insertDevice(testDevice)

        // When
        repository.deleteDevice("test-device-1")

        // Then
        val devices = repository.getAllDevices().first()
        assertEquals(0, devices.size)
    }

    @Test
    fun `updateDevice should modify existing device`() = runTest {
        // Given
        repository.insertDevice(testDevice)
        val updatedDevice = testDevice.copy(
            deviceName = "Updated Device Name",
            lastSeen = System.currentTimeMillis() + 1000
        )

        // When
        repository.updateDevice(updatedDevice)

        // Then
        val device = repository.getDeviceById("test-device-1")
        assertNotNull(device)
        assertEquals("Updated Device Name", device?.deviceName)
    }

    @Test
    fun `getAllDevices should return all devices in database`() = runTest {
        // Given
        val device2 = testDevice.copy(deviceId = "test-device-2", deviceName = "Device 2")
        repository.insertDevice(testDevice)
        repository.insertDevice(device2)

        // When
        val devices = repository.getAllDevices().first()

        // Then
        assertEquals(2, devices.size)
        assertTrue(devices.any { it.deviceId == "test-device-1" })
        assertTrue(devices.any { it.deviceId == "test-device-2" })
    }

    @Test
    fun `getConnectedDevices should return only connected devices`() = runTest {
        // Given
        val device2 = testDevice.copy(deviceId = "test-device-2", deviceName = "Device 2")
        repository.insertDevice(testDevice)
        repository.insertDevice(device2)

        // Connect only the first device
        repository.updateDeviceConnectionStatus("test-device-1", true)

        // When
        val connectedDevices = repository.getConnectedDevices().first()

        // Then
        assertEquals(1, connectedDevices.size)
        assertEquals("test-device-1", connectedDevices.first().deviceId)
        assertTrue(connectedDevices.first().connectionStatus == ConnectionStatus.CONNECTED)
    }
}