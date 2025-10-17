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

/**
 * 设置屏幕
 */
@Composable
fun SettingsScreen(
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
                    Text("设置")
                },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            imageVector = androidx.compose.material.icons.Icons.ArrowBack,
                            contentDescription = "返回"
                        )
                    }
                }
            )
        }
    ) { paddingValues ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // 应用信息
            SettingsSection(title = "应用信息") {
                SettingsItem(
                    title = "应用版本",
                    subtitle = "1.0.0",
                    icon = androidx.compose.material.icons.Icons.Info
                )

                SettingsItem(
                    title = "技术栈",
                    subtitle = "Kotlin + Jetpack Compose + Rust",
                    icon = androidx.compose.material.icons.Icons.Code
                )
            }

            Divider()

            // 权限状态
            SettingsSection(title = "权限状态") {
                SettingsItem(
                    title = "蓝牙权限",
                    subtitle = if (viewModel.hasBluetoothPermissions()) "已授予" else "未授予",
                    icon = androidx.compose.material.icons.Icons.Bluetooth,
                    status = if (viewModel.hasBluetoothPermissions())
                        SettingsItemStatus.GRANTED
                    else
                        SettingsItemStatus.DENIED
                )

                SettingsItem(
                    title = "位置权限",
                    subtitle = if (viewModel.hasBluetoothPermissions()) "已授予" else "未授予",
                    icon = androidx.compose.material.icons.Icons.LocationOn,
                    status = if (viewModel.hasBluetoothPermissions())
                        SettingsItemStatus.GRANTED
                    else
                        SettingsItemStatus.DENIED
                )

                if (viewModel.hasClipboardPermissions()) {
                    SettingsItem(
                        title = "剪贴板权限",
                        subtitle = "已授予",
                        icon = androidx.compose.material.icons.Icons.ContentCopy,
                        status = SettingsItemStatus.GRANTED
                    )
                }
            }

            Divider()

            // 功能设置
            SettingsSection(title = "功能设置") {
                SettingsItem(
                    title = "自动同步",
                    subtitle = "自动同步剪贴板内容到所有已连接设备",
                    icon = androidx.compose.material.icons.Icons.Sync,
                    trailing = {
                        Switch(
                            checked = false, // 将来保存到Preferences
                            onCheckedChange = { /* 将来实现 */ }
                        )
                    }
                )

                SettingsItem(
                    title = "通知提醒",
                    subtitle = "设备连接和同步时显示通知",
                    icon = androidx.compose.material.icons.Icons.Notifications,
                    trailing = {
                        Switch(
                            checked = true, // 将来保存到Preferences
                            onCheckedChange = { /* 将来实现 */ }
                        )
                    }
                )

                SettingsItem(
                    title = "加密传输",
                    subtitle = "所有数据传输都经过端到端加密",
                    icon = androidx.compose.material.icons.Icons.Security,
                    status = SettingsItemStatus.INFO,
                    enabled = false
                )
            }

            Divider()

            // 关于
            SettingsSection(title = "关于") {
                SettingsItem(
                    title = "关于NearClip",
                    subtitle = "隐私优先的P2P剪贴板同步工具",
                    icon = androidx.compose.material.icons.Icons.Info,
                    trailing = {
                        Icon(
                            imageVector = androidx.compose.material.icons.Icons.ArrowForward,
                            contentDescription = "打开"
                        )
                    }
                )

                SettingsItem(
                    title = "帮助与支持",
                    subtitle = "查看使用指南和常见问题",
                    icon = androidx.compose.material.icons.Icons.Help,
                    trailing = {
                        Icon(
                            imageVector = androidx.compose.material.icons.Icons.ArrowForward,
                            contentDescription = "打开"
                        )
                    }
                )

                SettingsItem(
                    title = "隐私政策",
                    subtitle = "了解我们的隐私保护措施",
                    icon = androidx.compose.material.icons.Icons.PrivacyTip,
                    trailing = {
                        Icon(
                            imageVector = androidx.compose.material.icons.Icons.ArrowForward,
                            contentDescription = "打开"
                        )
                    }
                )
            }
        }
    }
}

/**
 * 设置项状态
 */
object SettingsItemStatus {
    const val GRANTED = "granted"
    const val DENIED = "denied"
    const val INFO = "info"
    const val WARNING = "warning"
}

/**
 * 设置项组件
 */
@Composable
private fun SettingsItem(
    title: String,
    subtitle: String,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    status: String? = null,
    trailing: @Composable (() -> Unit)? = null,
    enabled: Boolean = true,
    onClick: (() -> Unit)? = null
) {
    Card(
        modifier = Modifier
            .fillMaxWidth(),
        enabled = enabled,
        onClick = onClick ?: {}
    ) {
        Row(
            modifier = Modifier
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                modifier = Modifier.size(24.dp),
                tint = if (enabled)
                    MaterialTheme.colorScheme.onSurface
                else
                    MaterialTheme.colorScheme.onSurfaceVariant
            )

            Spacer(modifier = Modifier.width(16.dp))

            Column(
                modifier = Modifier.weight(1f)
            ) {
                Text(
                    text = title,
                    style = MaterialTheme.typography.bodyLarge,
                    color = if (enabled)
                        MaterialTheme.colorScheme.onSurface
                    else
                        MaterialTheme.colorScheme.onSurfaceVariant
                )

                Text(
                    text = subtitle,
                    style = MaterialTheme.typography.bodyMedium,
                    color = if (enabled)
                        MaterialTheme.colorScheme.onSurfaceVariant
                    else
                        MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.6f)
                )
            }

            if (status != null) {
                StatusChip(status = status)
            }

            if (trailing != null) {
                trailing()
            }
        }
    }
}

/**
 * 状态芯片组件
 */
@Composable
private fun StatusChip(status: String) {
    Surface(
        shape = MaterialTheme.shapes.small,
        color = when (status) {
            SettingsItemStatus.GRANTED -> MaterialTheme.colorScheme.primaryContainer
            SettingsItemStatus.DENIED -> MaterialTheme.colorScheme.errorContainer
            SettingsItemStatus.INFO -> MaterialTheme.colorScheme.secondaryContainer
            SettingsItemStatus.WARNING -> MaterialTheme.colorScheme.tertiaryContainer
            else -> MaterialTheme.colorScheme.surfaceVariant
        }
    ) {
        Text(
            text = when (status) {
                SettingsItemStatus.GRANTED -> "已授予"
                SettingsItemStatus.DENIED -> "未授予"
                SettingsItemStatus.INFO -> "已启用"
                SettingsItemStatus.WARNING -> "注意"
                else -> status
            },
            style = MaterialTheme.typography.bodySmall,
            color = when (status) {
                SettingsItemStatus.GRANTED -> MaterialTheme.colorScheme.onPrimaryContainer
                SettingsItemStatus.DENIED -> MaterialTheme.colorScheme.onErrorContainer
                SettingsItemStatus.INFO -> MaterialTheme.colorScheme.onSecondaryContainer
                SettingsItemStatus.WARNING -> MaterialTheme.colorScheme.onTertiaryContainer
                else -> MaterialTheme.colorScheme.onSurfaceVariant
            },
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
        )
    }
}

/**
 * 设置分组标题
 */
@Composable
private fun SettingsSection(
    title: String,
    content: @Composable () -> Unit
) {
    Column {
        Text(
            text = title,
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.primary,
            modifier = Modifier.padding(bottom = 8.dp)
        )

        content()
    }
}