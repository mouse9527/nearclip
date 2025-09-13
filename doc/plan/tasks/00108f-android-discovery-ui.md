# Task 00108f: Android 统一设备发现UI

## 任务描述

实现Android平台统一设备发现的UI组件，使用Jetpack Compose创建透明的设备发现界面。用户无需区分BLE和WiFi传输方式，系统自动合并和显示设备。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class UnifiedDeviceDiscoveryScreenTest {
    @Test
    fun testUnifiedDeviceDisplay() {
        // RED: 测试统一设备显示（不区分传输方式）
        composeTestRule.setContent {
            UnifiedDeviceDiscoveryScreen(
                discoveryState = MockUnifiedDiscoveryState(),
                onDeviceSelected = {},
                onRefresh = {}
            )
        }
        
        composeTestRule.onNodeWithText("My Phone").assertExists()
        // 应该不显示传输方式信息
        composeTestRule.onNodeWithText("BLE").assertDoesNotExist()
        composeTestRule.onNodeWithText("WiFi").assertDoesNotExist()
    }

    @Test
    fun testDeviceMergingDisplay() {
        // RED: 测试设备合并显示
        composeTestRule.setContent {
            UnifiedDeviceDiscoveryScreen(
                discoveryState = MockMergedDiscoveryState(),
                onDeviceSelected = {},
                onRefresh = {}
            )
        }
        
        // 同一个设备即使通过多种方式发现也只显示一次
        composeTestRule.onAllNodesWithText("My Phone").fetchSemanticsNodes()
            .let { nodes -> assertEquals(1, nodes.size) }
    }

    @Test
    fun testIntelligentDiscoveryStatus() {
        // RED: 测试智能发现状态显示
        composeTestRule.setContent {
            DiscoveryStatusHeader(
                isScanning = true,
                strategy = DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY,
                deviceCount = 2
            )
        }
        
        composeTestRule.onNodeWithText("WiFi优先").assertExists()
        composeTestRule.onNodeWithText("2个设备").assertExists()
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
@Composable
fun UnifiedDeviceDiscoveryScreen(
    discoveryState: UnifiedDiscoveryState,
    onDeviceSelected: (UnifiedDevice) -> Unit,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxSize()
    ) {
        // 智能发现状态栏
        IntelligentDiscoveryHeader(
            isScanning = discoveryState.isScanning,
            strategy = discoveryState.strategy,
            deviceCount = discoveryState.discoveredDevices.size,
            onRefresh = onRefresh
        )
        
        // 统一设备列表
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
            
            // 策略指示器
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
            DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY -> 
                Triple(Icons.Default.Wifi, "WiFi优先", MaterialTheme.colorScheme.primary)
            DiscoveryStrategy.BLE_PRIMARY_WIFI_SECONDARY -> 
                Triple(Icons.Default.Bluetooth, "BLE优先", MaterialTheme.colorScheme.primary)
            DiscoveryStrategy.BLE_ONLY -> 
                Triple(Icons.Default.Bluetooth, "仅BLE", MaterialTheme.colorScheme.secondary)
            DiscoveryStrategy.WIFI_ONLY -> 
                Triple(Icons.Default.Wifi, "仅WiFi", MaterialTheme.colorScheme.secondary)
            DiscoveryStrategy.NONE -> 
                Triple(Icons.Default.Error, "无传输", MaterialTheme.colorScheme.error)
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

private fun getStrategyText(strategy: DiscoveryStrategy, isScanning: Boolean): String {
    return if (isScanning) {
        when (strategy) {
            DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY -> "WiFi优先搜索中..."
            DiscoveryStrategy.BLE_PRIMARY_WIFI_SECONDARY -> "BLE优先搜索中..."
            DiscoveryStrategy.BLE_ONLY -> "BLE搜索中..."
            DiscoveryStrategy.WIFI_ONLY -> "WiFi搜索中..."
            DiscoveryStrategy.NONE -> "搜索已停止"
        }
    } else {
        "搜索已停止"
    }
}

@Composable
fun UnifiedDeviceListView(
    devices: List<UnifiedDevice>,
    onDeviceSelected: (UnifiedDevice) -> Unit,
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
    device: UnifiedDevice,
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
            // 设备图标
            DeviceIcon(
                deviceType = device.type,
                modifier = Modifier.size(48.dp)
            )
            
            Spacer(modifier = Modifier.width(16.dp))
            
            // 设备信息（不显示传输方式）
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
                    // 设备状态
                    DeviceStatusBadge(
                        status = device.status,
                        modifier = Modifier.padding(end = 8.dp)
                    )
                    
                    // 可用传输方式图标（小型指示器）
                    TransportAvailabilityIndicator(
                        transports = device.transports,
                        modifier = Modifier.padding(end = 8.dp)
                    )
                    
                    // 连接质量
                    ConnectionQualityIndicator(
                        quality = device.quality,
                        modifier = Modifier.padding(end = 8.dp)
                    )
                }
            }
            
            // 操作按钮
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
fun TransportAvailabilityIndicator(
    transports: Set<TransportType>,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier,
        verticalAlignment = Alignment.CenterVertically
    ) {
        if (transports.contains(TransportType.WIFI)) {
            Icon(
                imageVector = Icons.Default.Wifi,
                contentDescription = "WiFi可用",
                modifier = Modifier.size(14.dp),
                tint = MaterialTheme.colorScheme.primary
            )
        }
        if (transports.contains(TransportType.BLE)) {
            Spacer(modifier = Modifier.width(2.dp))
            Icon(
                imageVector = Icons.Default.Bluetooth,
                contentDescription = "BLE可用",
                modifier = Modifier.size(14.dp),
                tint = MaterialTheme.colorScheme.primary
            )
        }
    }
}

@Composable
fun ConnectionQualityIndicator(
    quality: Float,
    modifier: Modifier = Modifier
) {
    val color = when {
        quality > 0.8f -> MaterialTheme.colorScheme.primary
        quality > 0.5f -> MaterialTheme.colorScheme.secondary
        else -> MaterialTheme.colorScheme.outline
    }
    
    Text(
        text = "${(quality * 100).toInt()}%",
        style = MaterialTheme.typography.bodySmall,
        color = color,
        modifier = modifier
    )
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
            text = if (isScanning) "正在搜索设备..." else "未发现设备",
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

@Composable
fun DeviceIcon(
    deviceType: DeviceType,
    modifier: Modifier = Modifier
) {
    val icon = when (deviceType) {
        DeviceType.Phone -> Icons.Default.Smartphone
        DeviceType.Tablet -> Icons.Default.Tablet
        DeviceType.Desktop -> Icons.Default.Computer
        DeviceType.Laptop -> Icons.Default.Laptop
        DeviceType.Watch -> Icons.Default.Watch
        DeviceType.TV -> Icons.Default.Tv
        DeviceType.Unknown -> Icons.Default.DevicesOther
    }
    
    Icon(
        imageVector = icon,
        contentDescription = null,
        modifier = modifier,
        tint = MaterialTheme.colorScheme.primary
    )
}

@Composable
fun DeviceStatusBadge(
    status: DeviceStatus,
    modifier: Modifier = Modifier
) {
    val (text, color) = when (status) {
        DeviceStatus.Connected -> "已连接" to MaterialTheme.colorScheme.primary
        DeviceStatus.Connecting -> "连接中" to MaterialTheme.colorScheme.secondary
        DeviceStatus.Disconnected -> "未连接" to MaterialTheme.colorScheme.outline
        DeviceStatus.Unknown -> "未知" to MaterialTheme.colorScheme.outline
    }
    
    Surface(
        modifier = modifier,
        shape = RoundedCornerShape(4.dp),
        color = color.copy(alpha = 0.12f)
    ) {
        Text(
            text = text,
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 2.dp),
            style = MaterialTheme.typography.bodySmall,
            color = color
        )
    }
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

// ViewModel和状态管理
class UnifiedDiscoveryViewModel : ViewModel() {
    private val _discoveryState = MutableStateFlow(UnifiedDiscoveryState())
    val discoveryState: StateFlow<UnifiedDiscoveryState> = _discoveryState.asStateFlow()
    
    private val unifiedDiscoveryManager: UnifiedDiscoveryManager = // 注入或创建
    
    fun startDiscovery() {
        viewModelScope.launch {
            unifiedDiscoveryManager.startDiscovery().collect { event ->
                when (event) {
                    is UnifiedDiscoveryEvent.DeviceDiscovered -> {
                        // 智能合并设备
                        val currentDevices = _discoveryState.value.discoveredDevices
                        val existingDevice = currentDevices.find { it.id == event.device.id }
                        
                        val updatedDevices = if (existingDevice != null) {
                            // 合并设备信息
                            val mergedDevice = mergeDeviceInfo(existingDevice, event.device)
                            currentDevices.map { if (it.id == mergedDevice.id) mergedDevice else it }
                        } else {
                            currentDevices + event.device
                        }
                        
                        _discoveryState.value = _discoveryState.value.copy(
                            isScanning = true,
                            discoveredDevices = updatedDevices.sortedByDescending { it.quality }
                        )
                    }
                    is UnifiedDiscoveryEvent.DeviceLost -> {
                        _discoveryState.value = _discoveryState.value.copy(
                            discoveredDevices = _discoveryState.value.discoveredDevices.filter { 
                                it.id != event.deviceId 
                            }
                        )
                    }
                    is UnifiedDiscoveryEvent.StrategyChanged -> {
                        _discoveryState.value = _discoveryState.value.copy(
                            strategy = event.newStrategy
                        )
                    }
                    is UnifiedDiscoveryEvent.ScanStarted -> {
                        _discoveryState.value = _discoveryState.value.copy(isScanning = true)
                    }
                    is UnifiedDiscoveryEvent.ScanStopped -> {
                        _discoveryState.value = _discoveryState.value.copy(isScanning = false)
                    }
                }
            }
        }
    }
    
    fun stopDiscovery() {
        unifiedDiscoveryManager.stopDiscovery()
    }
    
    fun refreshDevices() {
        stopDiscovery()
        startDiscovery()
    }
    
    private fun mergeDeviceInfo(existing: UnifiedDevice, new: UnifiedDevice): UnifiedDevice {
        return existing.copy(
            transports = existing.transports + new.transports,
            quality = maxOf(existing.quality, new.quality),
            lastSeen = maxOf(existing.lastSeen, new.lastSeen),
            attributes = existing.attributes + new.attributes
        )
    }
}

data class UnifiedDiscoveryState(
    val isScanning: Boolean = false,
    val strategy: DiscoveryStrategy = DiscoveryStrategy.WIFI_PRIMARY_BLE_SECONDARY,
    val discoveredDevices: List<UnifiedDevice> = emptyList()
)

// 统一事件类型
sealed class UnifiedDiscoveryEvent {
    data class DeviceDiscovered(val device: UnifiedDevice) : UnifiedDiscoveryEvent()
    data class DeviceLost(val deviceId: String) : UnifiedDiscoveryEvent()
    data class StrategyChanged(val newStrategy: DiscoveryStrategy) : UnifiedDiscoveryEvent()
    object ScanStarted : UnifiedDiscoveryEvent()
    object ScanStopped : UnifiedDiscoveryEvent()
}
```

### REFACTOR阶段
```kotlin
// 添加设备分组功能
// 添加搜索过滤功能
// 添加设备详情对话框
// 添加排序选项
```

## 验收标准
- [ ] 统一设备列表正确显示（不区分传输方式）
- [ ] 智能发现状态指示器显示当前策略
- [ ] 同一设备通过多种传输方式发现时只显示一次
- [ ] 设备质量评分和传输可用性指示器工作正常
- [ ] 刷新功能可用
- [ ] 空状态处理正确
- [ ] 设备选择功能正常
- [ ] 策略切换时状态实时更新

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108h: Android 统一设备发现管理](00108h-android-unified-discovery.md)

## 后续任务
- [Task 00108i: Android 设备连接管理](00108i-android-device-connection.md)