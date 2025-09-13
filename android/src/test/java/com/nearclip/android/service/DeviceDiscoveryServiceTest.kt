package com.nearclip.android.service

import org.junit.Test
import org.junit.Assert.*

class DeviceDiscoveryServiceTest {
    @Test
    fun testServiceLifecycle() {
        // RED: 测试服务生命周期
        val service = DeviceDiscoveryService()

        assertFalse(service.isRunning)

        service.onCreate()
        service.onStartCommand(null, 0, 0)

        assertTrue(service.isRunning)

        service.onDestroy()
        assertFalse(service.isRunning)
    }

    @Test
    fun testBackgroundDiscovery() {
        // RED: 测试后台发现
        val service = DeviceDiscoveryService()

        service.onCreate()
        assertTrue(service.canRunInBackground())

        service.startBackgroundDiscovery()
        assertTrue(service.isDiscoveringInBackground())
    }
}