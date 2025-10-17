package com.nearclip.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.nearclip.presentation.viewmodel.NearClipViewModel
import com.nearclip.ui.components.*
import com.nearclip.data.model.Device

/**
 * 设备列表屏幕
 */
@Composable
fun DeviceListScreen(
    onNavigateBack: () -> Unit,
    viewModel: NearClipViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val scaffoldState = rememberScaffoldState()

    Scaffold(
        scaffoldState = scaffoldState,
        topBar = {
            TopAppBar(
                title = {
                    Text("设备列表")
                },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            imageVector = androidx.compose.material.icons.Icons.ArrowBack,
                            contentDescription = "返回"
                        )
                    }
                },
                actions = {
                    // 设备发现按钮
                    IconButton(
                        onClick = {
                            if (uiState.isDiscovering) {
                                viewModel.stopDeviceDiscovery()
                            } else {
                                viewModel.startDeviceDiscovery()
                            }
                        }
                    ) {
                        Icon(
                            imageVector = if (uiState.isDiscovering)
                                androidx.compose.material.icons.Icons.SearchOff
                            else
                                androidx.compose.material.icons.Icons.Search,
                            contentDescription = if (uiState.isDiscovering) "停止发现" else "开始发现"
                        )
                    }
                }
            )
        }
    ) { paddingValues ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            if (uiState.isLoading && uiState.discoveredDevices.isEmpty()) {
                CircularProgressIndicator(
                    modifier = Modifier.align(Alignment.Center)
                )
            } else {
                DeviceListContent(
                    devices = uiState.discoveredDevices,
                    isLoading = uiState.isDiscovering,
                    selectedDevice = uiState.selectedDevice,
                    onDeviceClick = { viewModel.selectDevice(it) }
                )
            }
        }
    }
}

/**
 * 设备列表内容
 */
@Composable
private fun DeviceListContent(
    devices: List<Device>,
    isLoading: Boolean,
    selectedDevice: Device?,
    onDeviceClick: (Device) -> Unit
) {
    Column {
        // 发现状态指示器
        if (isLoading) {
            LinearProgressIndicator(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = 8.dp)
            )
            Text(
                text = "正在搜索设备...",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.primary,
                modifier = Modifier.padding(bottom = 16.dp)
            )
        }

        // 设备列表
        if (devices.isEmpty()) {
            EmptyStateMessage(
                message = if (isLoading) "搜索中..." else "未发现设备",
                description = if (isLoading) "请稍候，正在搜索附近的NearClip设备" else "请确保其他设备已开启NearClip并处于可发现状态"
            )
        } else {
            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(devices) { device ->
                    DeviceListItem(
                        device = device,
                        onClick = { onDeviceClick(device) },
                        isSelected = selectedDevice?.deviceId == device.deviceId
                    )
                }
            }
        }

        // 错误消息
        uiState.errorMessage?.let { error ->
            Spacer(modifier = Modifier.height(16.dp))
            ErrorCard(
                message = error,
                onDismiss = { /* 将在ViewModel中处理 */ }
            )
        }
    }
}

/**
 * 空状态消息
 */
@Composable
private fun EmptyStateMessage(
    message: String,
    description: String
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            imageVector = androidx.compose.material.icons.Icons.Devices,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(16.dp))

        Text(
            text = message,
            style = MaterialTheme.typography.headlineSmall,
            textAlign = TextAlign.Center,
            color = MaterialTheme.colorScheme.onSurface
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = description,
            style = MaterialTheme.typography.bodyMedium,
            textAlign = TextAlign.Center,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}