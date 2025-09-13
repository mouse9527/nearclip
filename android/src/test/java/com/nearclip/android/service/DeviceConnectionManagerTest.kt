package com.nearclip.android.service

import android.content.Context
import android.net.ConnectivityManager
import kotlinx.coroutines.runBlocking
import org.junit.Test
import org.junit.Assert.*
import org.mockito.Mockito

class DeviceConnectionManagerTest {
    @Test
    fun testDeviceConnection() {
        // RED: 测试设备连接
        val mockContext = Mockito.mock(Context::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)

        val manager = DeviceConnectionManager(mockContext, mockConnectivityManager)
        val device = createMockUnifiedDevice()

        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            assertEquals(ConnectionState.Connected, manager.getConnectionState(device.id))
        }
    }

    @Test
    fun testAutomaticTransportSelection() {
        // RED: 测试自动传输选择
        val mockContext = Mockito.mock(Context::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)
        val mockNetworkInfo = Mockito.mock(android.net.NetworkInfo::class.java)

        val manager = DeviceConnectionManager(mockContext, mockConnectivityManager)
        val device = createMockUnifiedDevice(
            transports = setOf(TransportType.WIFI, TransportType.BLE)
        )

        // 模拟WiFi网络可用
        Mockito.`when`(mockNetworkInfo.isConnected).thenReturn(true)

        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            // 应选择WiFi作为优先传输
            assertEquals(TransportType.WIFI, manager.getActiveTransport(device.id))
        }
    }

    @Test
    fun testTransportFallback() {
        // RED: 测试传输回退机制
        val mockContext = Mockito.mock(Context::class.java)
        val mockConnectivityManager = Mockito.mock(ConnectivityManager::class.java)

        val manager = DeviceConnectionManager(mockContext, mockConnectivityManager)
        val device = createMockUnifiedDevice(
            transports = setOf(TransportType.WIFI, TransportType.BLE)
        )

        // 先尝试WiFi连接（模拟失败）
        manager.simulateConnectionFailure(TransportType.WIFI)

        runBlocking {
            val connectionResult = manager.connectToDevice(device)
            assertTrue(connectionResult.isSuccess)
            // 应自动回退到BLE
            assertEquals(TransportType.BLE, manager.getActiveTransport(device.id))
        }
    }

    // Helper methods for creating mock devices
    private fun createMockUnifiedDevice(
        transports: Set<TransportType> = setOf(TransportType.BLE)
    ): UnifiedDevice {
        return UnifiedDevice(
            id = "test-device-123",
            name = "Test Device",
            type = DeviceType.PHONE,
            transports = transports,
            quality = 0.8f,
            lastSeen = System.currentTimeMillis(),
            attributes = emptyMap()
        )
    }
}