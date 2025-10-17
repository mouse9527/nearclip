package com.nearclip.ui.components

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.nearclip.data.model.Device
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.data.model.DeviceType
import com.nearclip.ui.theme.*

/**
 * 设备卡片组件
 * 显示设备信息和连接状态
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DeviceCard(
    device: Device,
    onConnect: () -> Unit,
    onDisconnect: () -> Unit,
    onCardClick: () -> Unit,
    modifier: Modifier = Modifier,
    isSelected: Boolean = false
) {
    Card(
        onClick = onCardClick,
        modifier = modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        shape = RoundedCornerShape(12.dp),
        elevation = CardDefaults.cardElevation(
            defaultElevation = if (isSelected) 8.dp else 4.dp
        ),
        colors = CardDefaults.cardColors(
            containerColor = if (isSelected) {
                MaterialTheme.colorScheme.primaryContainer
            } else {
                MaterialTheme.colorScheme.surface
            }
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        ) {
            // 设备信息行
            Row(
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically
            ) {
                // 设备图标
                DeviceIcon(
                    deviceType = device.deviceType,
                    connectionStatus = device.connectionStatus,
                    modifier = Modifier.size(48.dp)
                )

                Spacer(modifier = Modifier.width(16.dp))

                // 设备信息
                Column(
                    modifier = Modifier.weight(1f)
                ) {
                    Text(
                        text = device.deviceName,
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                        color = if (isSelected) {
                            MaterialTheme.colorScheme.onPrimaryContainer
                        } else {
                            MaterialTheme.colorScheme.onSurface
                        },
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis
                    )

                    Spacer(modifier = Modifier.height(4.dp))

                    Text(
                        text = "设备ID: ${device.deviceId.take(8)}...",
                        style = MaterialTheme.typography.bodySmall,
                        color = if (isSelected) {
                            MaterialTheme.colorScheme.onPrimaryContainer.copy(alpha = 0.7f)
                        } else {
                            MaterialTheme.colorScheme.onSurfaceVariant
                        }
                    )

                    Spacer(modifier = Modifier.height(2.dp))

                    Text(
                        text = "最后连接: ${formatLastSeen(device.lastSeen)}",
                        style = MaterialTheme.typography.bodySmall,
                        color = if (isSelected) {
                            MaterialTheme.colorScheme.onPrimaryContainer.copy(alpha = 0.7f)
                        } else {
                            MaterialTheme.colorScheme.onSurfaceVariant
                        }
                    )
                }

                // 连接状态指示器
                StatusIndicator(
                    status = device.connectionStatus,
                    modifier = Modifier.size(12.dp)
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            // 操作按钮行
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                when (device.connectionStatus) {
                    ConnectionStatus.CONNECTED -> {
                        OutlinedButton(
                            onClick = onDisconnect,
                            modifier = Modifier.weight(1f)
                        ) {
                            Icon(
                                imageVector = Icons.Default.Close,
                                contentDescription = "断开连接",
                                modifier = Modifier.size(16.dp)
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            Text("断开")
                        }
                    }
                    ConnectionStatus.CONNECTING -> {
                        Button(
                            onClick = { /* 连接中，禁用点击 */ },
                            enabled = false,
                            modifier = Modifier.weight(1f)
                        ) {
                            CircularProgressIndicator(
                                modifier = Modifier.size(16.dp),
                                strokeWidth = 2.dp,
                                color = MaterialTheme.colorScheme.onPrimary
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            Text("连接中...")
                        }
                    }
                    else -> { // DISCONNECTED, ERROR
                        Button(
                            onClick = onConnect,
                            modifier = Modifier.weight(1f)
                        ) {
                            Icon(
                                imageVector = Icons.Default.Bluetooth,
                                contentDescription = "连接",
                                modifier = Modifier.size(16.dp)
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            Text("连接")
                        }
                    }
                }

                // 更多选项按钮
                IconButton(
                    onClick = { /* 显示更多选项 */ }
                ) {
                    Icon(
                        imageVector = Icons.Default.MoreVert,
                        contentDescription = "更多选项"
                    )
                }
            }
        }
    }
}

/**
 * 设备图标组件
 */
@Composable
private fun DeviceIcon(
    deviceType: DeviceType,
    connectionStatus: ConnectionStatus,
    modifier: Modifier = Modifier
) {
    val icon = when (deviceType) {
        DeviceType.ANDROID -> Icons.Default.Android
        DeviceType.IOS -> Icons.Default.PhoneIphone
        DeviceType.MAC -> Icons.Default.LaptopMac
        DeviceType.WINDOWS -> Icons.Default.Laptop
        DeviceType.UNKNOWN -> Icons.Default.DeviceUnknown
    }

    val iconColor = when (connectionStatus) {
        ConnectionStatus.CONNECTED -> ConnectedColor
        ConnectionStatus.CONNECTING -> ConnectingColor
        ConnectionStatus.ERROR -> ErrorColor
        else -> DisconnectedColor
    }

    Surface(
        modifier = modifier,
        shape = RoundedCornerShape(8.dp),
        color = iconColor.copy(alpha = 0.1f)
    ) {
        Icon(
            imageVector = icon,
            contentDescription = "设备类型",
            modifier = Modifier.padding(12.dp),
            tint = iconColor
        )
    }
}

/**
 * 状态指示器组件
 */
@Composable
fun StatusIndicator(
    status: ConnectionStatus,
    modifier: Modifier = Modifier
) {
    val color = when (status) {
        ConnectionStatus.CONNECTED -> ConnectedColor
        ConnectionStatus.CONNECTING -> ConnectingColor
        ConnectionStatus.ERROR -> ErrorColor
        else -> DisconnectedColor
    }

    Surface(
        modifier = modifier,
        shape = RoundedCornerShape(6.dp),
        color = color
    ) {
        // 状态指示器只是一个有颜色的圆点
    }
}

/**
 * 格式化最后连接时间
 */
private fun formatLastSeen(timestamp: Long): String {
    val now = System.currentTimeMillis()
    val diff = now - timestamp

    return when {
        diff < 60_000 -> "刚刚"
        diff < 3600_000 -> "${diff / 60_000}分钟前"
        diff < 86400_000 -> "${diff / 3600_000}小时前"
        else -> "${diff / 86400_000}天前"
    }
}