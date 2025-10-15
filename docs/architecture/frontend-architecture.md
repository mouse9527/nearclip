# 前端架构

## 组件架构

### Android Compose 组件组织

```
app/src/main/java/com/nearclip/
├── ui/
│   ├── components/
│   │   ├── DeviceCard.kt
│   │   ├── StatusIndicator.kt
│   │   ├── QRCodeDisplay.kt
│   │   └── SyncProgressBar.kt
│   ├── screens/
│   │   ├── HomeScreen.kt
│   │   ├── DeviceDiscoveryScreen.kt
│   │   ├── DeviceManagementScreen.kt
│   │   └── SettingsScreen.kt
│   ├── theme/
│   │   ├── Color.kt
│   │   ├── Theme.kt
│   │   └── Type.kt
│   └── navigation/
│       └── Navigation.kt
```

### 组件模板

```kotlin
@Composable
fun DeviceCard(
    device: Device,
    onConnect: (String) -> Unit,
    onDisconnect: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    Card(
        modifier = modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // 设备图标
            Icon(
                imageVector = when (device.deviceType) {
                    "android" -> Icons.Default.Android
                    "mac" -> Icons.Default.Computer
                    else -> Icons.Default.DeviceUnknown
                },
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = when (device.connectionStatus) {
                    "connected" -> MaterialTheme.colorScheme.primary
                    "disconnected" -> MaterialTheme.colorScheme.onSurfaceVariant
                    else -> MaterialTheme.colorScheme.tertiary
                }
            )

            Spacer(modifier = Modifier.width(16.dp))

            // 设备信息
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = device.deviceName,
                    style = MaterialTheme.typography.titleMedium
                )
                Text(
                    text = "${device.deviceType.uppercase()} • ${getRelativeTimeString(device.lastSeen)}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            // 连接状态和操作按钮
            when (device.connectionStatus) {
                "connected" -> {
                    IconButton(onClick = { onDisconnect(device.deviceId) }) {
                        Icon(
                            imageVector = Icons.Default.BluetoothConnected,
                            contentDescription = "Disconnect"
                        )
                    }
                }
                "disconnected" -> {
                    OutlinedButton(
                        onClick = { onConnect(device.deviceId) }
                    ) {
                        Text("Connect")
                    }
                }
                else -> {
                    CircularProgressIndicator(
                        modifier = Modifier.size(24.dp),
                        strokeWidth = 2.dp
                    )
                }
            }
        }
    }
}
```

## 状态管理架构

### 状态结构

```kotlin
data class NearClipUiState(
    val connectedDevices: List<Device> = emptyList(),
    val discoveredDevices: List<Device> = emptyList(),
    val isScanning: Boolean = false,
    val isAdvertising: Boolean = false,
    val lastSyncStatus: SyncStatus? = null,
    val errorMessage: String? = null,
    val isLoading: Boolean = false
) {
    val hasConnectedDevices: Boolean
        get() = connectedDevices.isNotEmpty()

    val canSync: Boolean
        get() = hasConnectedDevices && !isLoading
}
```

### 状态管理模式

```kotlin
class NearClipViewModel : ViewModel() {
    private val _uiState = MutableStateFlow(NearClipUiState())
    val uiState: StateFlow<NearClipUiState> = _uiState.asStateFlow()

    private val bluetoothManager: BluetoothManager = TODO()
    private val syncService: SyncService = TODO()

    fun startDeviceDiscovery() {
        viewModelScope.launch {
            _uiState.update { it.copy(isScanning = true) }

            bluetoothManager.startDiscovery()
                .catch { error ->
                    _uiState.update {
                        it.copy(
                            isScanning = false,
                            errorMessage = "设备发现失败: ${error.message}"
                        )
                    }
                }
                .collect { devices ->
                    _uiState.update {
                        it.copy(
                            discoveredDevices = devices,
                            isScanning = false
                        )
                    }
                }
        }
    }

    fun connectToDevice(deviceId: String) {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }

            try {
                bluetoothManager.connectToDevice(deviceId)
                _uiState.update {
                    it.copy(
                        isLoading = false,
                        errorMessage = null
                    )
                }
            } catch (error: Exception) {
                _uiState.update {
                    it.copy(
                        isLoading = false,
                        errorMessage = "连接失败: ${error.message}"
                    )
                }
            }
        }
    }
}
```

## 路由架构

### 路由组织

```kotlin
sealed class Screen(val route: String) {
    object Home : Screen("home")
    object DeviceDiscovery : Screen("discovery")
    object DeviceManagement : Screen("management")
    object Settings : Screen("settings")
    object QRCode : Screen("qrcode")
}

@Composable
fun NearClipNavigation(
    navController: NavHostController = rememberNavController()
) {
    NavHost(
        navController = navController,
        startDestination = Screen.Home.route
    ) {
        composable(Screen.Home.route) {
            HomeScreen(
                onNavigateToDiscovery = {
                    navController.navigate(Screen.DeviceDiscovery.route)
                },
                onNavigateToManagement = {
                    navController.navigate(Screen.DeviceManagement.route)
                },
                onNavigateToSettings = {
                    navController.navigate(Screen.Settings.route)
                }
            )
        }

        composable(Screen.DeviceDiscovery.route) {
            DeviceDiscoveryScreen(
                onNavigateBack = { navController.popBackStack() },
                onNavigateToQRCode = {
                    navController.navigate(Screen.QRCode.route)
                }
            )
        }

        composable(Screen.QRCode.route) {
            QRCodeScreen(
                onNavigateBack = { navController.popBackStack() }
            )
        }
    }
}
```

## 前端服务层

### API 客户端设置

```kotlin
class BluetoothServiceImpl : BluetoothService {
    private val bluetoothAdapter: BluetoothAdapter? = TODO()
    private val bleScanner: BluetoothLeScanner? = TODO()
    private val gattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    // 处理连接成功
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    // 处理连接断开
                }
            }
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            // 处理特征值读取
        }

        override fun onCharacteristicWrite(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            // 处理特征值写入
        }
    }

    override suspend fun startDiscovery(): Flow<Device> = callbackFlow {
        val scanCallback = object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                val device = mapScanResultToDevice(result)
                trySend(device)
            }
        }

        bleScanner?.startScan(scanCallback)

        awaitClose {
            bleScanner?.stopScan(scanCallback)
        }
    }

    override suspend fun connectToDevice(device: Device): Boolean {
        return suspendCoroutine { continuation ->
            val bluetoothDevice = bluetoothAdapter?.getRemoteDevice(device.deviceId)
            bluetoothDevice?.connectGatt(context, false, gattCallback)
                ?.let { gatt ->
                    // 处理连接结果
                    continuation.resume(true)
                } ?: continuation.resume(false)
        }
    }
}
```

### 服务示例

```kotlin
class SyncServiceImpl(
    private val bluetoothService: BluetoothService,
    private val storageService: StorageService
) : SyncService {

    override suspend fun broadcastSync(
        content: String,
        targetDevices: List<Device>
    ): Result<Unit> {
        return try {
            val syncMessage = SyncMessage(
                syncId = UUID.randomUUID().toString(),
                sourceDeviceId = getCurrentDeviceId(),
                content = content,
                contentType = detectContentType(content),
                timestamp = System.currentTimeMillis(),
                targetDevices = targetDevices.map { it.deviceId }
            )

            // 存储同步记录
            storageService.saveSyncRecord(syncMessage)

            // 广播到所有目标设备
            targetDevices.forEach { device ->
                bluetoothService.sendMessage(device, syncMessage)
            }

            Result.success(Unit)
        } catch (error: Exception) {
            Result.failure(error)
        }
    }

    override suspend fun handleIncomingSync(message: SyncMessage): Result<Unit> {
        return try {
            // 验证消息来源
            if (!isValidSource(message.sourceDeviceId)) {
                return Result.failure(SecurityException("Unknown device"))
            }

            // 注入到粘贴板
            injectToClipboard(message.content)

            // 确认接收
            val ackMessage = AckMessage(
                originalMessageId = message.syncId,
                deviceId = getCurrentDeviceId(),
                status = "success"
            )

            bluetoothService.sendMessage(
                getDeviceById(message.sourceDeviceId)!!,
                ackMessage
            )

            Result.success(Unit)
        } catch (error: Exception) {
            Result.failure(error)
        }
    }
}
```
