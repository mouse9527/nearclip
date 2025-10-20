package com.nearclip.services.ble

import android.content.Context
import android.util.Log
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

/**
 * BLE服务工厂
 * 负责创建和管理BLE服务实例，实现单例模式
 */
object BleServiceFactory {

    private const val TAG = "BleServiceFactory"

    private var bleManager: BleManager? = null
    private val mutex = Mutex()

    /**
     * 获取BLE管理器实例（单例）
     */
    suspend fun getBleManager(context: Context): BleManager {
        return mutex.withLock {
            bleManager ?: createBleManager(context).also {
                bleManager = it
                Log.d(TAG, "创建新的BLE管理器实例")
            }
        }
    }

    /**
     * 获取BLE管理器实例（非挂起版本，如果未初始化则返回null）
     */
    fun getBleManagerOrNull(): BleManager? {
        return bleManager
    }

    /**
     * 创建BLE管理器实例
     */
    private fun createBleManager(context: Context): BleManager {
        Log.d(TAG, "创建BLE管理器")
        return BleManager(context.applicationContext)
    }

    /**
     * 销毁BLE管理器实例
     */
    suspend fun destroyBleManager() {
        mutex.withLock {
            bleManager?.let { manager ->
                Log.d(TAG, "销毁BLE管理器实例")
                manager.cleanup()
                bleManager = null
            }
        }
    }

    /**
     * 检查BLE管理器是否已初始化
     */
    fun isInitialized(): Boolean {
        return bleManager != null
    }

    /**
     * 强制重新初始化（用于测试或错误恢复）
     */
    suspend fun reinitialize(context: Context) {
        mutex.withLock {
            bleManager?.cleanup()
            bleManager = createBleManager(context)
            Log.d(TAG, "BLE管理器已重新初始化")
        }
    }

    /**
     * 获取权限管理器
     */
    fun getPermissionManager(context: Context): BlePermissionManager {
        return BlePermissionManager(context)
    }

    /**
     * 获取独立的扫描器（不通过管理器）
     */
    fun getScanner(context: Context): BleScanner {
        return BleScanner(context)
    }

    /**
     * 获取独立的广播器（不通过管理器）
     */
    fun getAdvertiser(context: Context): BleAdvertiser {
        return BleAdvertiser(context)
    }

    /**
     * 获取独立的连接管理器（不通过管理器）
     */
    fun getConnectionManager(context: Context): BleConnectionManager {
        return BleConnectionManager(context)
    }

    /**
     * 获取独立的设备发现监听器（不通过管理器）
     */
    fun getDiscoveryListener(scanner: BleScanner): DeviceDiscoveryListener {
        return DeviceDiscoveryListener(scanner)
    }

    /**
     * 创建测试用的BLE设备
     */
    fun createTestDevice(
        deviceId: String = "test-device-${System.currentTimeMillis()}",
        deviceName: String = "Test-NearClip",
        rssi: Int = -50,
        deviceType: BleDeviceType = BleDeviceType.NEARCLIP
    ): BleDevice {
        // 创建一个模拟的蓝牙设备
        val mockBluetoothDevice = MockBluetoothDevice(deviceId, deviceName)

        return BleDevice(
            deviceId = deviceId,
            deviceName = deviceName,
            deviceType = deviceType,
            rssi = rssi,
            timestamp = System.currentTimeMillis(),
            bluetoothDevice = mockBluetoothDevice
        )
    }

    /**
     * 创建测试消息
     */
    fun createTestMessage(
        messageId: String = "test-${System.currentTimeMillis()}",
        type: MessageType = MessageType.PING,
        payload: String = "Test message",
        sequenceNumber: Int = 0
    ): TestMessage {
        return TestMessage(
            messageId = messageId,
            type = type,
            payload = payload,
            timestamp = System.currentTimeMillis(),
            sequenceNumber = sequenceNumber
        )
    }

    /**
     * 模拟蓝牙设备类（用于测试）
     */
    private class MockBluetoothDevice(
        private val address: String,
        private val name: String
    ) : android.bluetooth.BluetoothDevice(null, address) {

        override fun getName(): String = name
        override fun getAddress(): String = address
        override fun getBluetoothClass(): android.bluetooth.BluetoothClass? = null
        override fun getBondState(): Int = android.bluetooth.BluetoothDevice.BOND_NONE
        override fun getType(): Int = android.bluetooth.BluetoothDevice.DEVICE_TYPE_LE
    }

    /**
     * 获取调试信息
     */
    suspend fun getDebugInfo(): Map<String, Any> {
        return mutex.withLock {
            mapOf(
                "managerInitialized" to (bleManager != null),
                "managerState" to (bleManager?.managerState?.value?.name ?: "null"),
                "isScanning" to (bleManager?.isScanning?.value ?: false),
                "isAdvertising" to (bleManager?.isAdvertising?.value ?: false),
                "discoveredDevicesCount" to (bleManager?.discoveredDevices?.value?.size ?: 0),
                "connectedDevicesCount" to (bleManager?.getConnectedDevices()?.size ?: 0)
            )
        }
    }
}