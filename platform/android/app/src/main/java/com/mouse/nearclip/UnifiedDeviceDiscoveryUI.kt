package com.mouse.nearclip

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

@Composable
fun UnifiedDeviceDiscoveryScreen(
    discoveryState: UnifiedDiscoveryState,
    onDeviceSelected: (DiscoveredDevice) -> Unit,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxSize()
    ) {
        // Intelligent discovery header
        IntelligentDiscoveryHeader(
            isScanning = discoveryState.isScanning,
            strategy = discoveryState.strategy,
            deviceCount = discoveryState.discoveredDevices.size,
            onRefresh = onRefresh
        )
        
        // Unified device list
        if (discoveryState.discoveredDevices.isEmpty()) {
            EmptyStateView(
                isScanning = discoveryState.isScanning,
                onRefresh = onRefresh
            )
        } else {
            UnifiedDeviceListView(
                devices = discoveryState.discoveredDevices,
                onDeviceSelected = onDeviceSelected
            )
        }
    }
}

@Composable
fun IntelligentDiscoveryHeader(
    isScanning: Boolean,
    strategy: DiscoveryStrategy,
    deviceCount: Int,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        color = MaterialTheme.colorScheme.primaryContainer
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Text(
                        text = "设备发现",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Text(
                        text = getStrategyText(strategy, isScanning),
                        style = MaterialTheme.typography.bodySmall,
                        color = if (isScanning) 
                            MaterialTheme.colorScheme.primary 
                        else 
                            MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
                
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        text = "$deviceCount 个设备",
                        style = MaterialTheme.typography.bodyMedium
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    IconButton(onClick = onRefresh) {
                        Icon(
                            imageVector = Icons.Default.Refresh,
                            contentDescription = "刷新"
                        )
                    }
                }
            }
            
            // Strategy indicator
            StrategyIndicator(strategy = strategy)
        }
    }
}

@Composable
fun StrategyIndicator(
    strategy: DiscoveryStrategy,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier.padding(top = 4.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        val (icon, text, color) = when (strategy) {
            DiscoveryStrategy.Aggressive -> 
                Triple(Icons.Default.Wifi, "高性能模式", MaterialTheme.colorScheme.primary)
            DiscoveryStrategy.Balanced -> 
                Triple(Icons.Default.Bluetooth, "平衡模式", MaterialTheme.colorScheme.primary)
            DiscoveryStrategy.PowerSaving -> 
                Triple(Icons.Default.BatterySaver, "省电模式", MaterialTheme.colorScheme.secondary)
        }
        
        Icon(
            imageVector = icon,
            contentDescription = null,
            modifier = Modifier.size(16.dp),
            tint = color
        )
        Spacer(modifier = Modifier.width(4.dp))
        Text(
            text = text,
            style = MaterialTheme.typography.bodySmall,
            color = color
        )
    }
}

@Composable
fun UnifiedDeviceListView(
    devices: List<DiscoveredDevice>,
    onDeviceSelected: (DiscoveredDevice) -> Unit,
    modifier: Modifier = Modifier
) {
    LazyColumn(
        modifier = modifier.fillMaxSize()
    ) {
        items(
            items = devices,
            key = { it.id }
        ) { device ->
            UnifiedDeviceListItem(
                device = device,
                onClick = { onDeviceSelected(device) }
            )
        }
    }
}

@Composable
fun UnifiedDeviceListItem(
    device: DiscoveredDevice,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Card(
        modifier = modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp)
            .clickable(onClick = onClick),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Device icon
            DeviceIcon(
                deviceType = device.type,
                modifier = Modifier.size(48.dp)
            )
            
            Spacer(modifier = Modifier.width(16.dp))
            
            // Device info (doesn't show transport method)
            Column(
                modifier = Modifier.weight(1f)
            ) {
                Text(
                    text = device.name,
                    style = MaterialTheme.typography.titleSmall,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
                
                Row(
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    // Signal strength indicator
                    SignalStrengthIndicator(
                        rssi = device.rssi,
                        modifier = Modifier.padding(end = 8.dp)
                    )
                    
                    // Connection quality
                    val quality = calculateSignalQuality(device.rssi)
                    Text(
                        text = "${(quality * 100).toInt()}%",
                        style = MaterialTheme.typography.bodySmall,
                        color = when {
                            quality > 0.8f -> MaterialTheme.colorScheme.primary
                            quality > 0.5f -> MaterialTheme.colorScheme.secondary
                            else -> MaterialTheme.colorScheme.outline
                        }
                    )
                }
            }
            
            // Action button
            IconButton(
                onClick = onClick,
                modifier = Modifier.size(40.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.ConnectWithoutContact,
                    contentDescription = "连接设备",
                    tint = MaterialTheme.colorScheme.primary
                )
            }
        }
    }
}

@Composable
fun DeviceIcon(
    deviceType: DeviceType,
    modifier: Modifier = Modifier
) {
    val icon = when (deviceType) {
        DeviceType.NEARCLIP -> Icons.Default.Smartphone
        DeviceType.OTHER -> Icons.Default.DevicesOther
    }
    
    Icon(
        imageVector = icon,
        contentDescription = null,
        modifier = modifier,
        tint = MaterialTheme.colorScheme.primary
    )
}

@Composable
fun SignalStrengthIndicator(
    rssi: Int,
    modifier: Modifier = Modifier
) {
    val signalBars = when {
        rssi > -50 -> 4
        rssi > -60 -> 3
        rssi > -70 -> 2
        rssi > -80 -> 1
        else -> 0
    }
    
    Row(
        modifier = modifier,
        verticalAlignment = Alignment.Bottom
    ) {
        repeat(4) { index ->
            Box(
                modifier = Modifier
                    .width(3.dp)
                    .height(4.dp + (index * 2).dp)
                    .padding(horizontal = 0.5.dp)
                    .clip(RoundedCornerShape(1.dp))
                    .background(
                        color = if (index < signalBars) 
                            MaterialTheme.colorScheme.primary 
                        else 
                            MaterialTheme.colorScheme.onSurface.copy(alpha = 0.2f)
                    )
            )
        }
    }
}

@Composable
fun EmptyStateView(
    isScanning: Boolean,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            imageVector = Icons.Default.DevicesOther,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = getEmptyStateMessage(isScanning),
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        if (!isScanning) {
            Spacer(modifier = Modifier.height(8.dp))
            
            Text(
                text = "点击刷新按钮重新搜索",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

// Helper functions
fun getStrategyText(strategy: DiscoveryStrategy, isScanning: Boolean): String {
    return if (isScanning) {
        when (strategy) {
            DiscoveryStrategy.Aggressive -> "高性能搜索中..."
            DiscoveryStrategy.Balanced -> "平衡模式搜索中..."
            DiscoveryStrategy.PowerSaving -> "省电模式搜索中..."
        }
    } else {
        "搜索已停止"
    }
}

fun getEmptyStateMessage(isScanning: Boolean): String {
    return if (isScanning) "正在搜索设备..." else "未发现设备"
}

fun calculateSignalQuality(rssi: Int): Float {
    // RSSI ranges from -100 (weak) to -30 (strong)
    // Normalize to 0.0 to 1.0 range
    val normalized = when {
        rssi >= -30 -> 1.0f
        rssi <= -100 -> 0.0f
        else -> (rssi + 100) / 70.0f
    }
    return normalized.coerceIn(0.0f, 1.0f)
}

// Data classes for state management
data class UnifiedDiscoveryState(
    val isScanning: Boolean = false,
    val strategy: DiscoveryStrategy = DiscoveryStrategy.Balanced,
    val discoveredDevices: List<DiscoveredDevice> = emptyList()
)

// ViewModel for managing discovery state
class UnifiedDiscoveryViewModel : androidx.lifecycle.ViewModel() {
    private val _discoveryState = androidx.lifecycle.MutableLiveData<UnifiedDiscoveryState>()
    val discoveryState: androidx.lifecycle.LiveData<UnifiedDiscoveryState> = _discoveryState
    
    init {
        _discoveryState.value = UnifiedDiscoveryState()
    }
    
    fun updateDevices(devices: List<DiscoveredDevice>) {
        val currentState = _discoveryState.value ?: UnifiedDiscoveryState()
        _discoveryState.value = currentState.copy(
            discoveredDevices = devices.sortedByDescending { calculateSignalQuality(it.rssi) }
        )
    }
    
    fun setScanning(isScanning: Boolean) {
        val currentState = _discoveryState.value ?: UnifiedDiscoveryState()
        _discoveryState.value = currentState.copy(isScanning = isScanning)
    }
    
    fun setStrategy(strategy: DiscoveryStrategy) {
        val currentState = _discoveryState.value ?: UnifiedDiscoveryState()
        _discoveryState.value = currentState.copy(strategy = strategy)
    }
}