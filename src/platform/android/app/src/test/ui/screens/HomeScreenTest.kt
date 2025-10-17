package com.nearclip.ui.screens

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.model.DeviceType
import com.nearclip.presentation.viewmodel.NearClipViewModel
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Rule
import org.junit.Test
import kotlin.test.assertTrue

class HomeScreenTest {

    @get:Rule
    val composeTestRule = createComposeRule()

    private val mockViewModel = mockk<NearClipViewModel>()

    private val testDevice = Device(
        deviceId = "test-device-1",
        deviceName = "Test Android Device",
        deviceType = DeviceType.ANDROID,
        publicKey = "test-public-key-123",
        lastSeen = System.currentTimeMillis(),
        connectionStatus = ConnectionStatus.CONNECTED
    )

    @Test
    fun homeScreen_should_display_app_title() {
        // Given
        val mockUiState = MutableStateFlow(
            NearClipViewModel.NearClipUiState(
                isLoading = false,
                discoveredDevices = listOf(testDevice),
                connectedDevices = listOf(testDevice),
                isDiscovering = false,
                errorMessage = null,
                hasPermissions = true,
                selectedDevice = null
            )
        )

        every { mockViewModel.uiState } returns mockUiState
        every { mockViewModel.getDeviceCount() } returns 1
        every { mockViewModel.getConnectedDeviceCount() } returns 1
        every { mockViewModel.hasAllPermissions() } returns true

        // When
        composeTestRule.setContent {
            HomeScreen(
                onNavigateToDevices = { }
            )
        }

        // Then
        composeTestRule
            .onNodeWithText("NearClip")
            .assertIsDisplayed()
    }

    @Test
    fun homeScreen_should_display_device_count() {
        // Given
        val mockUiState = MutableStateFlow(
            NearClipViewModel.NearClipUiState(
                isLoading = false,
                discoveredDevices = listOf(testDevice),
                connectedDevices = listOf(testDevice),
                isDiscovering = false,
                errorMessage = null,
                hasPermissions = true,
                selectedDevice = null
            )
        )

        every { mockViewModel.uiState } returns mockUiState
        every { mockViewModel.getDeviceCount() } returns 1
        every { mockViewModel.getConnectedDeviceCount() } returns 1
        every { mockViewModel.hasAllPermissions() } returns true

        // When
        composeTestRule.setContent {
            HomeScreen(
                onNavigateToDevices = { }
            )
        }

        // Then
        composeTestRule
            .onNodeWithText("设备总数")
            .assertIsDisplayed()

        composeTestRule
            .onNodeWithText("1")
            .assertIsDisplayed()
    }

    @Test
    fun homeScreen_should_display_connected_device_count() {
        // Given
        val mockUiState = MutableStateFlow(
            NearClipViewModel.NearClipUiState(
                isLoading = false,
                discoveredDevices = listOf(testDevice),
                connectedDevices = listOf(testDevice),
                isDiscovering = false,
                errorMessage = null,
                hasPermissions = true,
                selectedDevice = null
            )
        )

        every { mockViewModel.uiState } returns mockUiState
        every { mockViewModel.getDeviceCount() } returns 1
        every { mockViewModel.getConnectedDeviceCount() } returns 1
        every { mockViewModel.hasAllPermissions() } returns true

        // When
        composeTestRule.setContent {
            HomeScreen(
                onNavigateToDevices = { }
            )
        }

        // Then
        composeTestRule
            .onNodeWithText("已连接")
            .assertIsDisplayed()

        composeTestRule
            .onNodeWithText("1")
            .assertIsDisplayed()
    }

    @Test
    fun homeScreen_should_display_permission_warning_when_permissions_missing() {
        // Given
        val mockUiState = MutableStateFlow(
            NearClipViewModel.NearClipUiState(
                isLoading = false,
                discoveredDevices = emptyList(),
                connectedDevices = emptyList(),
                isDiscovering = false,
                errorMessage = null,
                hasPermissions = false,
                selectedDevice = null
            )
        )

        every { mockViewModel.uiState } returns mockUiState
        every { mockViewModel.getDeviceCount() } returns 0
        every { mockViewModel.getConnectedDeviceCount() } returns 0
        every { mockViewModel.hasAllPermissions() } returns false

        // When
        composeTestRule.setContent {
            HomeScreen(
                onNavigateToDevices = { }
            )
        }

        // Then
        composeTestRule
            .onNodeWithText("需要权限")
            .assertIsDisplayed()
    }

    @Test
    fun homeScreen_should_display_recent_devices_when_connected() {
        // Given
        val mockUiState = MutableStateFlow(
            NearClipViewModel.NearClipUiState(
                isLoading = false,
                discoveredDevices = listOf(testDevice),
                connectedDevices = listOf(testDevice),
                isDiscovering = false,
                errorMessage = null,
                hasPermissions = true,
                selectedDevice = null
            )
        )

        every { mockViewModel.uiState } returns mockUiState
        every { mockViewModel.getDeviceCount() } returns 1
        every { mockViewModel.getConnectedDeviceCount() } returns 1
        every { mockViewModel.hasAllPermissions() } returns true

        // When
        composeTestRule.setContent {
            HomeScreen(
                onNavigateToDevices = { }
            )
        }

        // Then
        composeTestRule
            .onNodeWithText("最近连接的设备")
            .assertIsDisplayed()

        composeTestRule
            .onNodeWithText("Test Android Device")
            .assertIsDisplayed()
    }
}