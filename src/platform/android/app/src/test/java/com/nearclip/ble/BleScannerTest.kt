package com.nearclip.ble

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.le.BluetoothLeScanner
import android.content.Context
import android.content.pm.PackageManager
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import app.cash.turbine.test
import com.nearclip.services.ble.*
import io.mockk.*
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.test.*

class BleScannerTest {

    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private lateinit var context: Context
    private lateinit var bluetoothAdapter: BluetoothAdapter
    private lateinit var bluetoothLeScanner: BluetoothLeScanner
    private lateinit var bleScanner: BleScanner

    @Before
    fun setUp() {
        context = mockk(relaxed = true)
        bluetoothAdapter = mockk(relaxed = true)
        bluetoothLeScanner = mockk(relaxed = true)

        mockkStatic(android.bluetooth.BluetoothAdapter::class)
        every { BluetoothAdapter.getDefaultAdapter() } returns bluetoothAdapter
        every { bluetoothAdapter.bluetoothLeScanner } returns bluetoothLeScanner
        every { bluetoothAdapter.isEnabled } returns true

        bleScanner = BleScanner(context)
    }

    @Test
    fun `hasRequiredPermissions should return true when all permissions are granted`() {
        // Given
        every {
            context.checkSelfPermission(Manifest.permission.BLUETOOTH_SCAN)
        } returns PackageManager.PERMISSION_GRANTED
        every {
            context.checkSelfPermission(Manifest.permission.BLUETOOTH_CONNECT)
        } returns PackageManager.PERMISSION_GRANTED
        every {
            context.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION)
        } returns PackageManager.PERMISSION_GRANTED

        // When
        val result = bleScanner.hasRequiredPermissions()

        // Then
        assertTrue(result)
    }

    @Test
    fun `hasRequiredPermissions should return false when some permissions are denied`() {
        // Given
        every {
            context.checkSelfPermission(Manifest.permission.BLUETOOTH_SCAN)
        } returns PackageManager.PERMISSION_DENIED
        every {
            context.checkSelfPermission(Manifest.permission.BLUETOOTH_CONNECT)
        } returns PackageManager.PERMISSION_GRANTED
        every {
            context.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION)
        } returns PackageManager.PERMISSION_GRANTED

        // When
        val result = bleScanner.hasRequiredPermissions()

        // Then
        assertFalse(result)
    }

    @Test
    fun `isBluetoothAvailable should return true when bluetooth is enabled`() {
        // Given
        every { bluetoothAdapter.isEnabled } returns true

        // When
        val result = bleScanner.isBluetoothAvailable()

        // Then
        assertTrue(result)
    }

    @Test
    fun `isBluetoothAvailable should return false when bluetooth is disabled`() {
        // Given
        every { bluetoothAdapter.isEnabled } returns false

        // When
        val result = bleScanner.isBluetoothAvailable()

        // Then
        assertFalse(result)
    }

    @Test
    fun `isBluetoothAvailable should return false when bluetooth adapter is null`() {
        // Given
        every { BluetoothAdapter.getDefaultAdapter() } returns null

        val scanner = BleScanner(context)

        // When
        val result = scanner.isBluetoothAvailable()

        // Then
        assertFalse(result)
    }

    @Test
    fun `startScanning should fail when permissions are missing`() = runTest {
        // Given
        every {
            context.checkSelfPermission(Manifest.permission.BLUETOOTH_SCAN)
        } returns PackageManager.PERMISSION_DENIED

        // When
        val result = bleScanner.startScanning()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is SecurityException)
    }

    @Test
    fun `startScanning should fail when bluetooth is not available`() = runTest {
        // Given
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns false

        // When
        val result = bleScanner.startScanning()

        // Then
        assertTrue(result.isFailure)
        assertTrue(result.exceptionOrNull() is IllegalStateException)
    }

    @Test
    fun `startScanning should succeed when permissions and bluetooth are available`() = runTest {
        // Given
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns true
        val scanCallbackSlot = slot<android.bluetooth.le.ScanCallback>()
        every {
            bluetoothLeScanner.startScan(any(), any(), capture(scanCallbackSlot))
        } just Runs

        // When
        val result = bleScanner.startScanning()

        // Then
        assertTrue(result.isSuccess)
        verify { bluetoothLeScanner.startScan(any(), any(), any()) }
    }

    @Test
    fun `startScanning should not start scanning when already scanning`() = runTest {
        // Given
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns true
        every {
            bluetoothLeScanner.startScan(any(), any(), any())
        } just Runs

        // First call
        bleScanner.startScanning()

        // When
        val result = bleScanner.startScanning()

        // Then
        assertTrue(result.isSuccess)
        verify(exactly = 1) { bluetoothLeScanner.startScan(any(), any(), any()) }
    }

    @Test
    fun `stopScanning should stop active scanning`() {
        // Given
        val scanCallbackSlot = slot<android.bluetooth.le.ScanCallback>()
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns true
        every {
            bluetoothLeScanner.startScan(any(), any(), capture(scanCallbackSlot))
        } just Runs
        every {
            bluetoothLeScanner.stopScan(scanCallbackSlot.captured)
        } just Runs

        // Start scanning first
        bleScanner.startScanning()

        // When
        bleScanner.stopScanning()

        // Then
        verify { bluetoothLeScanner.stopScan(any()) }
    }

    @Test
    fun `discoveredDevices flow should emit devices when scan results are received`() = runTest {
        // Given
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns true

        val scanCallbackSlot = slot<android.bluetooth.le.ScanCallback>()
        every {
            bluetoothLeScanner.startScan(any(), any(), capture(scanCallbackSlot))
        } just Runs

        // Create mock device
        val mockBluetoothDevice = mockk<android.bluetooth.BluetoothDevice>(relaxed = true)
        every { mockBluetoothDevice.address } returns "00:11:22:33:44:55"
        every { mockBluetoothDevice.name } returns "Test Device"

        val mockScanResult = mockk<android.bluetooth.le.ScanResult>(relaxed = true)
        every { mockScanResult.device } returns mockBluetoothDevice
        every { mockScanResult.rssi } returns -50
        every { mockScanResult.scanRecord } returns null

        // When
        bleScanner.startScanning()

        // Then
        bleScanner.discoveredDevices.test {
            // Simulate scan result
            scanCallbackSlot.captured.onScanResult(
                android.bluetooth.le.ScanSettings.CALLBACK_TYPE_ALL_MATCHES,
                mockScanResult
            )

            // Expect device emission
            val device = awaitItem()
            assertEquals("00:11:22:33:44:55", device.deviceId)
            assertEquals("Test Device", device.deviceName)
            assertEquals(-50, device.rssi)

            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `discoveredDevices flow should ignore devices without names`() = runTest {
        // Given
        every {
            context.checkSelfPermission(any())
        } returns PackageManager.PERMISSION_GRANTED
        every { bluetoothAdapter.isEnabled } returns true

        val scanCallbackSlot = slot<android.bluetooth.le.ScanCallback>()
        every {
            bluetoothLeScanner.startScan(any(), any(), capture(scanCallbackSlot))
        } just Runs

        // Create mock device without name
        val mockBluetoothDevice = mockk<android.bluetooth.BluetoothDevice>(relaxed = true)
        every { mockBluetoothDevice.address } returns "00:11:22:33:44:55"
        every { mockBluetoothDevice.name } returns null

        val mockScanResult = mockk<android.bluetooth.le.ScanResult>(relaxed = true)
        every { mockScanResult.device } returns mockBluetoothDevice
        every { mockScanResult.rssi } returns -50
        every { mockScanResult.scanRecord } returns null

        // When
        bleScanner.startScanning()

        // Then
        bleScanner.discoveredDevices.test {
            // Simulate scan result without name
            scanCallbackSlot.captured.onScanResult(
                android.bluetooth.le.ScanSettings.CALLBACK_TYPE_ALL_MATCHES,
                mockScanResult
            )

            // Expect no emission since device has no name
            expectNoEvents()

            cancelAndIgnoreRemainingEvents()
        }
    }

    @Test
    fun `cleanup should close the devices channel`() {
        // When
        bleScanner.cleanup()

        // Then
        bleScanner.discoveredDevices.test {
            // Channel should be closed
            awaitComplete()
        }
    }
}