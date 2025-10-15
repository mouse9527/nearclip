# 测试策略

## 测试金字塔

```
E2E Tests
/        \
Integration Tests
/            \
Frontend Unit  Backend Unit
```

## 测试组织

### 前端测试

```
android/app/src/test/
├── ui/
│   ├── components/
│   │   ├── DeviceCardTest.kt
│   │   └── StatusIndicatorTest.kt
│   └── screens/
│       ├── HomeScreenTest.kt
│       └── DeviceDiscoveryScreenTest.kt
├── viewmodel/
│   └── NearClipViewModelTest.kt
└── utils/
    └── ExtensionsTest.kt
```

### 后端测试

```
android/app/src/test/
├── services/
│   ├── BluetoothManagerTest.kt
│   ├── SyncServiceTest.kt
│   └── SecurityServiceTest.kt
├── repository/
│   ├── DeviceRepositoryTest.kt
│   └── SyncRecordRepositoryTest.kt
└── database/
    └── AppDatabaseTest.kt
```

### E2E 测试

```
test/e2e/
├── DevicePairingTest.kt
├── TextSyncTest.kt
├── MultiDeviceSyncTest.kt
└── ErrorHandlingTest.kt
```

## 测试示例

### 前端组件测试

```kotlin
@Test
fun `DeviceCard displays device information correctly`() {
    val device = Device(
        deviceId = "test-device-1",
        deviceName = "Test Android",
        deviceType = "android",
        publicKey = "test-public-key",
        lastSeen = System.currentTimeMillis(),
        connectionStatus = "connected"
    )

    composeTestRule.setContent {
        DeviceCard(
            device = device,
            onConnect = {},
            onDisconnect = {}
        )
    }

    composeTestRule
        .onNodeWithText("Test Android")
        .assertIsDisplayed()

    composeTestRule
        .onNodeWithContentDescription("Disconnect")
        .assertIsDisplayed()
}
```

### 后端 API 测试

```kotlin
@Test
fun `SyncService broadcasts message to all target devices`() = runTest {
    val sourceDevice = Device("device-1", "Source", "android", "key1", 0, "connected")
    val targetDevice1 = Device("device-2", "Target1", "mac", "key2", 0, "connected")
    val targetDevice2 = Device("device-3", "Target2", "android", "key3", 0, "connected")

    val syncService = SyncService(
        bluetoothService = mockBluetoothService,
        storageService = mockStorageService
    )

    val result = syncService.broadcastSync(
        content = "Test message",
        targetDevices = listOf(targetDevice1, targetDevice2)
    )

    assertTrue(result.isSuccess)
    verify(mockBluetoothService, times(1)).sendMessage(targetDevice1, any())
    verify(mockBluetoothService, times(1)).sendMessage(targetDevice2, any())
    verify(mockStorageService, times(1)).saveSyncRecord(any())
}
```

### E2E 测试

```kotlin
@Test
fun `Complete device pairing and text sync flow`() = runTest {
    val androidDevice = createAndroidDevice()
    val macDevice = createMacDevice()

    // 建立连接
    val pairingResult = androidDevice.initiatePairing(macDevice)
    assertTrue(pairingResult.isSuccess)

    // 等待配对完成
    delay(1000)
    assertTrue(androidDevice.isPairedWith(macDevice))
    assertTrue(macDevice.isPairedWith(androidDevice))

    // 测试文本同步
    val testContent = "Hello from Android!"
    androidDevice.simulateClipboardCopy(testContent)

    // 等待同步完成
    delay(2000)

    // 验证 Mac 设备收到同步内容
    val syncedContent = macDevice.getClipboardContent()
    assertEquals(testContent, syncedContent)
}
```
