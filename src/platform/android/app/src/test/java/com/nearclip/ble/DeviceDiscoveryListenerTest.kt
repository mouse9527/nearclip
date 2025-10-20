package com.nearclip.ble

import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import app.cash.turbine.test
import com.nearclip.services.ble.*
import io.mockk.*
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.test.*

class DeviceDiscoveryListenerTest {

    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private lateinit var bleScanner: BleScanner
    private lateinit var discoveryListener: DeviceDiscoveryListener

    @Before
    fun setUp() {
        bleScanner = mockk(relaxed = true)
        discoveryListener = DeviceDiscoveryListener(bleScanner)
    }

    @Test
    fun `startDiscovery should start listening to scanner discoveries`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        // When
        discoveryListener.startDiscovery()

        // Then
        discoveryListener.discoveryEvents.test {
            assertEquals(DiscoveryEvent.DiscoveryStarted, awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `stopDiscovery should stop listening and send stopped event`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow
        discoveryListener.startDiscovery()

        // When
        discoveryListener.stopDiscovery()

        // Then
        discoveryListener.discoveryEvents.test {
            assertEquals(DiscoveryEvent.DiscoveryStopped, awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `new device should be added to discovered devices`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val testDevice = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device"
        )

        // When
        discoveryListener.startDiscovery()
        deviceFlow.emit(testDevice)

        // Then
        discoveryListener.discoveredDevices.test {
            val devices = awaitItem()
            assertEquals(1, devices.size)
            assertTrue(devices.containsKey("test-001"))
            assertEquals("Test-Device", devices["test-001"]?.deviceName)

            cancelAndIgnoreRemainingEvents()
        }

        discoveryListener.discoveryEvents.test {
            assertEquals(DiscoveryEvent.DiscoveryStarted, awaitItem())
            assertEquals(DiscoveryEvent.NewDeviceDiscovered(testDevice), awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `device with stronger signal should replace existing device`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val weakDevice = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device",
            rssi = -70
        )

        val strongDevice = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device",
            rssi = -50
        )

        // When
        discoveryListener.startDiscovery()
        deviceFlow.emit(weakDevice)
        deviceFlow.emit(strongDevice)

        // Then
        discoveryListener.discoveredDevices.test {
            val devices = awaitItem()
            assertEquals(1, devices.size)
            assertEquals(-50, devices["test-001"]?.rssi)

            cancelAndIgnoreRemainingEvents()
        }

        discoveryListener.discoveryEvents.test {
            assertEquals(DiscoveryEvent.DiscoveryStarted, awaitItem())
            assertEquals(DiscoveryEvent.NewDeviceDiscovered(weakDevice), awaitItem())
            assertEquals(DiscoveryEvent.DeviceUpdated(strongDevice), awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `device with small signal improvement should not replace existing device`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val device1 = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device",
            rssi = -60,
            timestamp = System.currentTimeMillis()
        )

        val device2 = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device",
            rssi = -58, // Only 2dBm improvement
            timestamp = System.currentTimeMillis() + 1000
        )

        // When
        discoveryListener.startDiscovery()
        deviceFlow.emit(device1)
        deviceFlow.emit(device2)

        // Then
        discoveryListener.discoveredDevices.test {
            val devices = awaitItem()
            assertEquals(1, devices.size)
            assertEquals(-60, devices["test-001"]?.rssi) // Should keep the original

            cancelAndIgnoreRemainingEvents()
        }

        discoveryListener.discoveryEvents.test {
            assertEquals(DiscoveryEvent.DiscoveryStarted, awaitItem())
            assertEquals(DiscoveryEvent.NewDeviceDiscovered(device1), awaitItem())
            // Should not emit DeviceUpdated event
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `removeDevice should remove device from discovered devices`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val testDevice = BleServiceFactory.createTestDevice(
            deviceId = "test-001",
            deviceName = "Test-Device"
        )

        discoveryListener.startDiscovery()
        deviceFlow.emit(testDevice)

        // Wait for device to be added
        discoveryListener.discoveredDevices.test {
            awaitItem() // Initial empty map
            awaitItem() // Map with device
            cancelAndIgnoreRemainingEvents()
        }

        // When
        discoveryListener.removeDevice("test-001")

        // Then
        discoveryListener.discoveredDevices.test {
            val devices = awaitItem()
            assertEquals(0, devices.size)

            cancelAndIgnoreRemainingEvents()
        }

        discoveryListener.discoveryEvents.test {
            // Skip initial events
            skipItems(2)
            assertEquals(DiscoveryEvent.DeviceRemoved(testDevice), awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `clearDevices should remove all devices`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val device1 = BleServiceFactory.createTestDevice(deviceId = "test-001")
        val device2 = BleServiceFactory.createTestDevice(deviceId = "test-002")

        discoveryListener.startDiscovery()
        deviceFlow.emit(device1)
        deviceFlow.emit(device2)

        // When
        discoveryListener.clearDevices()

        // Then
        discoveryListener.discoveredDevices.test {
            val devices = awaitItem()
            assertEquals(0, devices.size)

            cancelAndIgnoreRemainingEvents()
        }

        discoveryListener.discoveryEvents.test {
            // Skip initial events
            skipItems(3) // DiscoveryStarted + 2 NewDeviceDiscovered
            assertEquals(DiscoveryEvent.AllDevicesCleared, awaitItem())
            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `getNearClipDevices should filter NearClip devices`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val nearClipDevice = BleServiceFactory.createTestDevice(
            deviceId = "nearclip-001",
            deviceName = "NearClip-Android",
            deviceType = BleDeviceType.NEARCLIP
        )
        val regularDevice = BleServiceFactory.createTestDevice(
            deviceId = "regular-001",
            deviceName = "Regular-Device",
            deviceType = BleDeviceType.LE
        )

        discoveryListener.startDiscovery()
        deviceFlow.emit(nearClipDevice)
        deviceFlow.emit(regularDevice)

        // Wait for devices to be added
        discoveryListener.discoveredDevices.test {
            awaitItem() // Empty map
            awaitItem() // Map with nearClip device
            awaitItem() // Map with both devices
            cancelAndIgnoreRemainingEvents()
        }

        // When
        val nearClipDevices = discoveryListener.getNearClipDevices()

        // Then
        assertEquals(1, nearClipDevices.size)
        assertEquals("nearclip-001", nearClipDevices.first().deviceId)
        assertEquals("NearClip-Android", nearClipDevices.first().deviceName)
    }

    @Test
    fun `getDevice should return correct device`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val testDevice = BleServiceFactory.createTestDevice(deviceId = "test-001")

        discoveryListener.startDiscovery()
        deviceFlow.emit(testDevice)

        // Wait for device to be added
        discoveryListener.discoveredDevices.test {
            awaitItem() // Empty map
            awaitItem() // Map with device
            cancelAndIgnoreRemainingEvents()
        }

        // When
        val foundDevice = discoveryListener.getDevice("test-001")
        val notFoundDevice = discoveryListener.getDevice("nonexistent")

        // Then
        assertNotNull(foundDevice)
        assertEquals("test-001", foundDevice?.deviceId)
        assertNull(notFoundDevice)
    }

    @Test
    fun `getDiscoveryStats should return correct statistics`() = runTest {
        // Given
        val deviceFlow = kotlinx.coroutines.flow.MutableSharedFlow<BleDevice>()
        every { bleScanner.discoveredDevices } returns deviceFlow

        val nearClipDevice = BleServiceFactory.createTestDevice(
            deviceId = "nearclip-001",
            deviceType = BleDeviceType.NEARCLIP
        )
        val leDevice = BleServiceFactory.createTestDevice(
            deviceId = "le-001",
            deviceType = BleDeviceType.LE
        )
        val dualDevice = BleServiceFactory.createTestDevice(
            deviceId = "dual-001",
            deviceType = BleDeviceType.DUAL
        )
        val unknownDevice = BleServiceFactory.createTestDevice(
            deviceId = "unknown-001",
            deviceType = BleDeviceType.UNKNOWN
        )

        discoveryListener.startDiscovery()
        deviceFlow.emit(nearClipDevice)
        deviceFlow.emit(leDevice)
        deviceFlow.emit(dualDevice)
        deviceFlow.emit(unknownDevice)

        // Wait for all devices to be added
        discoveryListener.discoveredDevices.test {
            awaitItem() // Empty map
            awaitItem() // nearClip device
            awaitItem() // nearClip + le device
            awaitItem() // nearClip + le + dual device
            awaitItem() // all devices
            cancelAndIgnoreRemainingEvents()
        }

        // When
        val stats = discoveryListener.getDiscoveryStats()

        // Then
        assertEquals(4, stats.totalDevices)
        assertEquals(1, stats.nearClipDevices)
        assertEquals(1, stats.leDevices)
        assertEquals(1, stats.dualDevices)
        assertEquals(1, stats.unknownDevices)
        assertTrue(stats.lastDiscoveryTime > 0)
        assertNotNull(stats.getLastDiscoveryTimeString())
    }

    @Test
    fun `cleanup should close discovery events channel`() = runTest {
        // Given
        discoveryListener.startDiscovery()

        // When
        discoveryListener.cleanup()

        // Then
        discoveryListener.discoveryEvents.test {
            // Channel should be closed
            awaitComplete()
        }
    }
}