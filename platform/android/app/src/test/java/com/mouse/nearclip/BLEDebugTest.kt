package com.mouse.nearclip

import android.bluetooth.BluetoothAdapter
import android.content.Context
import kotlinx.coroutines.runBlocking
import org.junit.Test
import org.junit.Assert.*
import org.junit.Before
import org.mockito.Mock
import org.mockito.Mockito.`when`
import org.mockito.MockitoAnnotations

class BLEDebugTest {
    @Mock
    private lateinit var mockContext: Context
    
    @Mock
    private lateinit var mockBluetoothAdapter: BluetoothAdapter
    
    @Before
    fun setup() {
        MockitoAnnotations.openMocks(this)
        `when`(mockBluetoothAdapter.isEnabled).thenReturn(true)
    }
    
    @Test
    fun testBLEScannerCreation() {
        // Test basic BLE scanner creation
        val scanner = BLEScannerManager(mockContext, mockBluetoothAdapter)
        assertNotNull(scanner)
        assertFalse(scanner.isScanning())
    }
    
    @Test
    fun testBLEScanConfigurations() {
        // Test different scan configurations
        val defaultConfig = BLEScanConfig.default()
        assertEquals(android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_POWER, defaultConfig.scanMode)
        assertEquals(5000L, defaultConfig.scanInterval)
        assertEquals(1000L, defaultConfig.scanWindow)
        
        val aggressiveConfig = BLEScanConfig.aggressive()
        assertEquals(android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_LATENCY, aggressiveConfig.scanMode)
        assertEquals(1000L, aggressiveConfig.scanInterval)
        assertEquals(500L, aggressiveConfig.scanWindow)
        
        val powerSavingConfig = BLEScanConfig.powerSaving()
        assertEquals(android.bluetooth.le.ScanSettings.SCAN_MODE_OPPORTUNISTIC, powerSavingConfig.scanMode)
        assertEquals(10000L, powerSavingConfig.scanInterval)
        assertEquals(500L, powerSavingConfig.scanWindow)
    }
    
    @Test
    fun testDeviceTypeHandling() {
        // Test device type enum functionality
        assertEquals(DeviceType.NEARCLIP, DeviceType.NEARCLIP)
        assertEquals(DeviceType.OTHER, DeviceType.OTHER)
        
        val device = DiscoveredDevice(
            id = "test-device-id",
            name = "Test Device",
            type = DeviceType.NEARCLIP,
            transport = TransportType.BLE,
            rssi = -75,
            lastSeen = System.currentTimeMillis()
        )
        
        assertEquals("test-device-id", device.id)
        assertEquals("Test Device", device.name)
        assertEquals(DeviceType.NEARCLIP, device.type)
        assertEquals(TransportType.BLE, device.transport)
        assertEquals(-75, device.rssi)
    }
    
    @Test
    fun testBLEDiscoveryErrorHandling() {
        // Test error types and messages
        val disabledError = BLEDiscoveryError.BluetoothDisabled
        assertEquals("Bluetooth is disabled", disabledError.message)
        
        val notSupportedError = BLEDiscoveryError.BluetoothNotSupported
        assertEquals("Bluetooth is not supported", notSupportedError.message)
        
        val permissionError = BLEDiscoveryError.PermissionDenied
        assertEquals("Bluetooth permission denied", permissionError.message)
        
        val scanFailedError = BLEDiscoveryError.ScanFailed(1)
        assertEquals("Scan failed with code: 1", scanFailedError.message)
    }
    
    @Test
    fun testScanningWorkflow() {
        // Test complete scanning workflow
        val scanner = BLEScannerManager(mockContext, mockBluetoothAdapter)
        
        runBlocking {
            // Test start scanning
            val startResult = scanner.startScan()
            assertTrue(startResult.isSuccess)
            assertTrue(scanner.isScanning())
            
            // Test stop scanning
            val stopResult = scanner.stopScan()
            assertTrue(stopResult.isSuccess)
            assertFalse(scanner.isScanning())
        }
        
        // Test device management
        assertEquals(0, scanner.getDiscoveredDevices().size)
        scanner.clearDiscoveredDevices()
        assertEquals(0, scanner.getDiscoveredDevices().size)
    }
}