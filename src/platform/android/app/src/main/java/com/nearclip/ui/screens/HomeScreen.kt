package com.nearclip.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.nearclip.presentation.viewmodel.NearClipViewModel
import com.nearclip.ui.components.*
import com.nearclip.data.model.Device

/**
 * 首页屏幕
 */
@Composable
fun HomeScreen(
    onNavigateToDevices: () -> Unit,
    viewModel: NearClipViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        // 标题和描述
        HeaderSection()

        Spacer(modifier = Modifier.height(24.dp))

        // 状态概览
        StatusOverviewSection(
            deviceCount = viewModel.getDeviceCount(),
            connectedCount = viewModel.getConnectedDeviceCount(),
            hasPermissions = viewModel.hasAllPermissions(),
            onNavigateToDevices = onNavigateToDevices
        )

        Spacer(modifier = Modifier.height(24.dp))

        // 权限状态
        if (!viewModel.hasAllPermissions()) {
            PermissionStatusSection()
        }

        Spacer(modifier = Modifier.height(24.dp))

        // 最近连接的设备
        if (uiState.connectedDevices.isNotEmpty()) {
            RecentDevicesSection(
                devices = uiState.connectedDevices.take(3),
                onDeviceClick = { viewModel.selectDevice(it) }
            )
        }

        // 错误消息
        uiState.errorMessage?.let { error ->
            Spacer(modifier = Modifier.height(16.dp))
            ErrorCard(
                message = error,
                onDismiss = { viewModel.clearErrorMessage() }
            )
        }
    }
}

/**
 * 头部区域
 */
@Composable
private fun HeaderSection() {
    Column {
        Text(
            text = "NearClip",
            style = MaterialTheme.typography.headlineLarge,
            fontWeight = FontWeight.Bold,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth()
        )

        Text(
            text = "隐私优先的剪贴板同步工具",
            style = MaterialTheme.typography.bodyLarge,
            textAlign = TextAlign.Center,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = "在您的Android和Mac设备间安全同步剪贴板内容",
            style = MaterialTheme.typography.bodyMedium,
            textAlign = TextAlign.Center,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

/**
 * 状态概览区域
 */
@Composable
private fun StatusOverviewSection(
    deviceCount: Int,
    connectedCount: Int,
    hasPermissions: Boolean,
    onNavigateToDevices: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Text(
                        text = "设备总数",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Text(
                        text = "$deviceCount",
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold
                    )
                }

                Column(
                    horizontalAlignment = Alignment.End
                ) {
                    Text(
                        text = "已连接",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Text(
                        text = "$connectedCount",
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                        color = if (connectedCount > 0)
                            MaterialTheme.colorScheme.primary
                        else
                            MaterialTheme.colorScheme.error
                    )
                    )
                }
            }

            Spacer(modifier = Modifier.height(12.dp))

            Divider()

            Spacer(modifier = Modifier.height(12.dp))

            // 快速操作按钮
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                if (!hasPermissions) {
                    Button(
                        onClick = { /* 将在权限管理中实现 */ },
                        modifier = Modifier.weight(1f)
                    ) {
                        Text("授予权限")
                    }
                }

                OutlinedButton(
                    onClick = onNavigateToDevices,
                    modifier = Modifier.weight(1f)
                ) {
                    Text("查看设备")
                }
            }
        }
    }
}

/**
 * 权限状态区域
 */
@Composable
private fun PermissionStatusSection() {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.errorContainer
        )
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = androidx.compose.material.icons.Icons.Warning,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onErrorContainer
            )

            Spacer(modifier = Modifier.width(12.dp))

            Column {
                Text(
                    text = "需要权限",
                    style = MaterialTheme.typography.bodyMedium,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onErrorContainer
                )
                Text(
                    text = "NearClip需要蓝牙和位置权限才能正常工作",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onErrorContainer
                )
            }
        }
    }
}

/**
 * 最近设备区域
 */
@Composable
private fun RecentDevicesSection(
    devices: List<Device>,
    onDeviceClick: (Device) -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Text(
                text = "最近连接的设备",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            Spacer(modifier = Modifier.height(12.dp))

            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(devices) { device ->
                    DeviceListItem(
                        device = device,
                        onClick = { onDeviceClick(device) }
                    )
                }
            }
        }
    }
}