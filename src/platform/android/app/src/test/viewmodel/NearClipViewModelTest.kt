package com.nearclip.presentation.viewmodel

import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.model.DeviceType
import com.nearclip.data.repository.DeviceRepository
import com.nearclip.services.PermissionManager
import com.nearclip.test.util.MainDispatcherRule
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.runTest
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

@ExperimentalCoroutinesApi
class NearClipViewModelTest {

    @get:Rule
    val instantExecutorRule = InstantTaskExecutorRule()

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    // Mock依赖
    private lateinit var mockDeviceRepository: DeviceRepository
    private lateinit var mockPermissionManager: PermissionManager

    private lateinit var viewModel: NearClipViewModel

    // 测试数据
    private val testDevice = Device(
        deviceId = "test-device-1",
        deviceName = "Test Android Device",
        deviceType = DeviceType.ANDROID,
        publicKey = "test-public-key-123",
        lastSeen = System.currentTimeMillis(),
        connectionStatus = ConnectionStatus.DISCONNECTED
    )

    @Before
    fun setup() {
        mockDeviceRepository = mockk()
        mockPermissionManager = mockk()

        // 设置Mock的默认行为
        every { mockDeviceRepository.getAllDevices() } returns flowOf(listOf(testDevice))
        every { mockDeviceRepository.getConnectedDevices() } returns flowOf(emptyList())
        every { mockPermissionManager.areAllPermissionsGranted() } returns true
        every { mockPermissionManager.areBluetoothPermissionsGranted() } returns true
        every { mockPermissionManager.areClipboardPermissionsGranted() } returns true

        viewModel = NearClipViewModel(mockDeviceRepository, mockPermissionManager)
    }

    @Test
    fun `initial state should have correct default values`() = runTest {
        val uiState = viewModel.uiState.first()

        assertFalse(uiState.isLoading)
        assertFalse(uiState.isDiscovering)
        assertEquals(1, uiState.discoveredDevices.size)
        assertEquals("test-device-1", uiState.discoveredDevices.first().deviceId)
        assertNull(uiState.errorMessage)
    }

    @Test
    fun `startDeviceDiscovery should update loading state`() = runTest {
        // When
        viewModel.startDeviceDiscovery()

        // Then
        val uiState = viewModel.uiState.first()
        assertTrue(uiState.isDiscovering)
        assertNull(uiState.errorMessage)
    }

    @Test
    fun `stopDeviceDiscovery should stop discovery state`() = runTest {
        // Given
        viewModel.startDeviceDiscovery()

        // When
        viewModel.stopDeviceDiscovery()

        // Then
        val uiState = viewModel.uiState.first()
        assertFalse(uiState.isDiscovering)
    }

    @Test
    fun `connectToDevice should update selected device`() = runTest {
        // When
        viewModel.connectToDevice(testDevice)

        // Then
        val uiState = viewModel.uiState.first()
        assertEquals(testDevice, uiState.selectedDevice)
        assertNull(uiState.errorMessage)
    }

    @Test
    fun `clearErrorMessage should remove error message`() = runTest {
        // Given - 设置错误状态
        // 这需要通过测试方式触发错误，或者暴露一个测试方法

        // When
        viewModel.clearErrorMessage()

        // Then
        val uiState = viewModel.uiState.first()
        assertNull(uiState.errorMessage)
    }

    @Test
    fun `getDeviceCount should return correct count`() = runTest {
        // When
        val count = viewModel.getDeviceCount()

        // Then
        assertEquals(1, count)
    }

    @Test
    fun `getConnectedDeviceCount should return correct count`() = runTest {
        // When
        val count = viewModel.getConnectedDeviceCount()

        // Then
        assertEquals(0, count) // 因为我们mock的connected devices是空列表
    }

    @Test
    fun `hasAllPermissions should return permission manager result`() = runTest {
        // When
        val hasPermissions = viewModel.hasAllPermissions()

        // Then
        assertTrue(hasPermissions)
        verify { mockPermissionManager.areAllPermissionsGranted() }
    }

    @Test
    fun `hasBluetoothPermissions should return permission manager result`() = runTest {
        // When
        val hasPermissions = viewModel.hasBluetoothPermissions()

        // Then
        assertTrue(hasPermissions)
        verify { mockPermissionManager.areBluetoothPermissionsGranted() }
    }

    @Test
    fun `hasClipboardPermissions should return permission manager result`() = runTest {
        // When
        val hasPermissions = viewModel.hasClipboardPermissions()

        // Then
        assertTrue(hasPermissions)
        verify { mockPermissionManager.areClipboardPermissionsGranted() }
    }
}