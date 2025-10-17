package com.nearclip.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.nearclip.data.model.ConnectionStatus
import com.nearclip.ui.theme.*

/**
 * 状态指示器组件
 * 用于显示连接状态的视觉指示器
 */
@Composable
fun StatusIndicator(
    status: ConnectionStatus,
    modifier: Modifier = Modifier,
    size: Int = 12
) {
    val color = when (status) {
        ConnectionStatus.CONNECTED -> ConnectedColor
        ConnectionStatus.CONNECTING -> ConnectingColor
        ConnectionStatus.ERROR -> ErrorColor
        else -> DisconnectedColor
    }

    val sizeDp = size.dp

    // 使用Box来创建圆点指示器
    androidx.compose.foundation.layout.Box(
        modifier = modifier
            .size(sizeDp)
            .clip(CircleShape)
            .background(color)
    )
}

/**
 * 带标签的状态指示器
 * 显示状态文字和指示器
 */
@Composable
fun StatusIndicatorWithLabel(
    status: ConnectionStatus,
    modifier: Modifier = Modifier,
    showLabel: Boolean = true
) {
    val statusText = when (status) {
        ConnectionStatus.CONNECTED -> "已连接"
        ConnectionStatus.CONNECTING -> "连接中"
        ConnectionStatus.ERROR -> "错误"
        else -> "未连接"
    }

    val statusColor = when (status) {
        ConnectionStatus.CONNECTED -> ConnectedColor
        ConnectionStatus.CONNECTING -> ConnectingColor
        ConnectionStatus.ERROR -> ErrorColor
        else -> DisconnectedColor
    }

    androidx.compose.foundation.layout.Row(
        modifier = modifier,
        verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
    ) {
        StatusIndicator(
            status = status,
            modifier = androidx.compose.ui.Modifier.padding(end = 8.dp)
        )

        if (showLabel) {
            androidx.compose.material3.Text(
                text = statusText,
                style = MaterialTheme.typography.bodySmall,
                color = statusColor
            )
        }
    }
}