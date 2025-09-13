package com.nearclip.android.service

import android.bluetooth.BluetoothAdapter
import android.content.Context
import android.net.ConnectivityManager
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.test.runTest
import org.junit.Test
import org.junit.Assert.*
import org.mockito.Mockito

class UnifiedDiscoveryManagerTest {
    @Test
    fun testDeviceDeduplication() {
        // RED: 测试设备去重
        val mockContext = Mockito.mock(Context::class.java)
        val mockBluetoothAdapter = Mockito.mock(BluetoothAdapter::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)

        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)

        runBlocking {
            val devices = mutableListOf<UnifiedDevice>()
            val job = launch {
                manager.startDiscovery().collect { device ->
                    devices.add(device)
                }
            }

            // 模拟通过BLE和WiFi发现同一个设备
            manager.addMockDevice(BluetoothDevice("device-123", "Test Device"), "device-123")
            manager.addMockDevice(WiFiDevice("device-123", "Test Device"), "device-123")

            delay(100)
            job.cancel()

            assertEquals(1, devices.size) // 同一个设备只应出现一次
            assertEquals("device-123", devices[0].id)
        }
    }

    @Test
    fun testIntelligentTransportSelection() {
        // RED: 测试智能传输方式选择
        val mockContext = Mockito.mock(Context::class.java)
        val mockBluetoothAdapter = Mockito.mock(BluetoothAdapter::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)
        val mockNetworkInfo = Mockito.mock(android.net.NetworkInfo::class.java)

        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)

        // 模拟WiFi网络可用
        Mockito.`when`(mockNetworkInfo.isConnected).thenReturn(true)

        val selection = manager.selectOptimalTransport()
        assertEquals(TransportType.WIFI, selection)
    }

    @Test
    fun testDeviceMerging() {
        // RED: 测试设备信息合并
        val mockContext = Mockito.mock(Context::class.java)
        val mockBluetoothAdapter = Mockito.mock(BluetoothAdapter::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)

        val manager = UnifiedDiscoveryManager(mockContext, mockBluetoothAdapter, mockConnectivityManager)

        runBlocking {
            val devices = mutableListOf<UnifiedDevice>()
            val job = launch {
                manager.startDiscovery().collect { device ->
                    devices.add(device)
                }
            }

            // 先通过BLE发现
            manager.addMockDevice(BluetoothDevice("device-123", "Test Device"), "device-123")
            delay(50)

            // 再通过WiFi发现（应合并信息）
            manager.addMockDevice(WiFiDevice("device-123", "Test Device"), "device-123")
            delay(50)

            job.cancel()

            assertEquals(1, devices.size)
            assertTrue(devices[0].transports.contains(TransportType.BLE))
            assertTrue(devices[0].transports.contains(TransportType.WIFI))
        }
    }
}

// Mock device classes for testing
data class BluetoothDevice(val id: String, val name: String)
data class WiFiDevice(val id: String, val name: String)