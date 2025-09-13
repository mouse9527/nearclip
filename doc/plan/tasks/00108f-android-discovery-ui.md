# Task 00108f: Android 发现UI组件

## 任务描述

实现Android平台设备发现的UI组件，使用Jetpack Compose创建设备列表界面和状态指示器。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class DeviceDiscoveryScreenTest {
    @Test
    fun testDeviceListDisplay() {
        // RED: 测试设备列表显示
        composeTestRule.setContent {
            DeviceDiscoveryScreen(
                discoveryState = MockDiscoveryState(),
                onDeviceSelected = {},
                onRefresh = {}
            )
        }
        
        composeTestRule.onNodeWithText("Test Device").assertExists()
        composeTestRule.onNodeWithText("Connected").assertExists()
    }

    @Test
    fun testDiscoveryStatusIndicator() {
        // RED: 测试发现状态指示器
        composeTestRule.setContent {
            DiscoveryStatusIndicator(
                isScanning = true,
                discoveredDeviceCount = 3
            )
        }
        
        composeTestRule.onNodeWithText("发现中...").assertExists()
        composeTestRule.onNodeWithText("3个设备").assertExists()
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
@Composable
fun DeviceDiscoveryScreen(
    discoveryState: DiscoveryState,
    onDeviceSelected: (Device) -> Unit,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxSize()
    ) {
        // 顶部状态栏
        DiscoveryStatusHeader(
            isScanning = discoveryState.isScanning,
            deviceCount = discoveryState.discoveredDevices.size,
            onRefresh = onRefresh
        )
        
        // 设备列表
        if (discoveryState.discoveredDevices.isEmpty()) {
            EmptyStateView(
                isScanning = discoveryState.isScanning,
                onRefresh = onRefresh
            )
        } else {
            DeviceListView(
                devices = discoveryState.discoveredDevices,
                onDeviceSelected = onDeviceSelected
            )
        }
    }
}

@Composable
fun DiscoveryStatusHeader(
    isScanning: Boolean,
    deviceCount: Int,
    onRefresh: () -> Unit,
    modifier: Modifier = Modifier
) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        color = MaterialTheme.colorScheme.primaryContainer
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = "设备发现",
                    style = MaterialTheme.typography.titleMedium
                )
                Text(
                    text = if (isScanning) "发现中..." else "已停止",
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
    }
}

@Composable
fun DeviceListView(
    devices: List<Device>,
    onDeviceSelected: (Device) -> Unit,
    modifier: Modifier = Modifier
) {
    LazyColumn(
        modifier = modifier.fillMaxSize()
    ) {
        items(
            items = devices,
            key = { it.id }
        ) { device ->
            DeviceListItem(
                device = device,
                onClick = { onDeviceSelected(device) }
            )
        }
    }
}

@Composable
fun DeviceListItem(
    device: Device,
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
            
            // 设备信息
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
                    DeviceStatusBadge(
                        status = device.status,
                        modifier = Modifier.padding(end = 8.dp)
                    )
                    
                    Text(
                        text = device.transportType.displayName,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
            
            // 信号强度
            SignalStrengthIndicator(
                rssi = device.rssi,
                modifier = Modifier.padding(start = 8.dp)
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
class DiscoveryViewModel : ViewModel() {
    private val _discoveryState = MutableStateFlow(DiscoveryState())
    val discoveryState: StateFlow<DiscoveryState> = _discoveryState.asStateFlow()
    
    private val discoveryManager: DeviceDiscoveryManager = // 注入或创建
    
    fun startDiscovery() {
        viewModelScope.launch {
            discoveryManager.startDiscovery().collect { event ->
                when (event) {
                    is DiscoveryEvent.DeviceDiscovered -> {
                        _discoveryState.value = _discoveryState.value.copy(
                            isScanning = true,
                            discoveredDevices = _discoveryState.value.discoveredDevices + event.device
                        )
                    }
                    is DiscoveryEvent.DeviceLost -> {
                        _discoveryState.value = _discoveryState.value.copy(
                            discoveredDevices = _discoveryState.value.discoveredDevices.filter { 
                                it.id != event.deviceId 
                            }
                        )
                    }
                    is DiscoveryEvent.ScanStarted -> {
                        _discoveryState.value = _discoveryState.value.copy(isScanning = true)
                    }
                    is DiscoveryEvent.ScanStopped -> {
                        _discoveryState.value = _discoveryState.value.copy(isScanning = false)
                    }
                }
            }
        }
    }
    
    fun stopDiscovery() {
        discoveryManager.stopDiscovery()
    }
    
    fun refreshDevices() {
        stopDiscovery()
        startDiscovery()
    }
}

data class DiscoveryState(
    val isScanning: Boolean = false,
    val discoveredDevices: List<Device> = emptyList()
)
```

### REFACTOR阶段
```kotlin
// 添加设备分组功能
// 添加搜索过滤功能
// 添加设备详情对话框
// 添加排序选项
```

## 验收标准
- [ ] 设备列表正确显示
- [ ] 状态指示器工作正常
- [ ] 刷新功能可用
- [ ] 空状态处理正确
- [ ] 设备选择功能正常

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108d: Android 电池优化](00108d-android-battery-optimization.md)

## 后续任务
- [Task 00108f: Android 后台服务](00108f-android-background-service.md)