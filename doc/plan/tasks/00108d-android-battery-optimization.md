# Task 00108d: Android 电池优化

## 任务描述

实现Android平台的电池优化管理，智能调整设备发现策略以减少电池消耗。

## TDD开发要求

### RED阶段 - 编写失败的测试
```kotlin
class BatteryOptimizationManagerTest {
    @Test
    fun testBatteryOptimizationStatus() {
        // RED: 测试电池优化状态
        val manager = BatteryOptimizationManager(mockContext)
        
        assertFalse(manager.isBatteryOptimizationEnabled())
        assertEquals(5000L, manager.getOptimalScanInterval())
    }

    @Test
    fun testLowPowerModeDetection() {
        // RED: 测试低电量模式检测
        val manager = BatteryOptimizationManager(mockContext)
        
        assertFalse(manager.isLowPowerMode())
        assertTrue(manager.shouldReduceDiscoveryFrequency())
    }
}
```

### GREEN阶段 - 最小实现
```kotlin
class BatteryOptimizationManager(
    private val context: Context
) {
    private val powerManager: PowerManager by lazy {
        context.getSystemService(Context.POWER_SERVICE) as PowerManager
    }
    
    private val batteryManager: BatteryManager by lazy {
        context.getSystemService(Context.BATTERY_SERVICE) as BatteryManager
    }
    
    fun isBatteryOptimizationEnabled(): Boolean {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            !powerManager.isIgnoringBatteryOptimizations(context.packageName)
        } else {
            false
        }
    }
    
    fun isLowPowerMode(): Boolean {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            powerManager.isPowerSaveMode
        } else {
            false
        }
    }
    
    fun getBatteryLevel(): Int {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            batteryManager.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY)
        } else {
            getBatteryLevelLegacy()
        }
    }
    
    fun getOptimalScanInterval(): Long {
        return when {
            isLowPowerMode() -> 15000L // 15秒
            getBatteryLevel() < 20 -> 10000L // 10秒
            getBatteryLevel() < 50 -> 5000L // 5秒
            else -> 2000L // 2秒
        }
    }
    
    fun getOptimalScanDuration(): Long {
        return when {
            isLowPowerMode() -> 500L // 0.5秒
            getBatteryLevel() < 20 -> 1000L // 1秒
            getBatteryLevel() < 50 -> 2000L // 2秒
            else -> 5000L // 5秒
        }
    }
    
    fun shouldReduceDiscoveryFrequency(): Boolean {
        return isLowPowerMode() || getBatteryLevel() < 30
    }
    
    fun shouldEnableBackgroundDiscovery(): Boolean {
        return !isLowPowerMode() && getBatteryLevel() > 20 && !isBatteryOptimizationEnabled()
    }
    
    fun getBatteryStatus(): BatteryStatus {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            val status = batteryManager.getIntProperty(BatteryManager.BATTERY_PROPERTY_STATUS)
            when (status) {
                BatteryManager.BATTERY_STATUS_CHARGING -> BatteryStatus.Charging
                BatteryManager.BATTERY_STATUS_DISCHARGING -> BatteryStatus.Discharging
                BatteryManager.BATTERY_STATUS_NOT_CHARGING -> BatteryStatus.NotCharging
                BatteryManager.BATTERY_STATUS_FULL -> BatteryStatus.Full
                else -> BatteryStatus.Unknown
            }
        } else {
            BatteryStatus.Unknown
        }
    }
    
    suspend fun requestDisableBatteryOptimization(): Boolean {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            try {
                val intent = Intent().apply {
                    action = Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS
                    data = Uri.parse("package:${context.packageName}")
                    flags = Intent.FLAG_ACTIVITY_NEW_TASK
                }
                context.startActivity(intent)
                true
            } catch (e: Exception) {
                false
            }
        } else {
            true
        }
    }
    
    private fun getBatteryLevelLegacy(): Int {
        val intent = context.registerReceiver(
            null, 
            IntentFilter(Intent.ACTION_BATTERY_CHANGED)
        )
        val level = intent?.getIntExtra(BatteryManager.EXTRA_LEVEL, -1) ?: -1
        val scale = intent?.getIntExtra(BatteryManager.EXTRA_SCALE, -1) ?: -1
        
        return if (level != -1 && scale != -1) {
            (level * 100 / scale)
        } else {
            50 // 默认值
        }
    }
}

enum class BatteryStatus {
    Charging,
    Discharging,
    NotCharging,
    Full,
    Unknown
}

class DiscoveryStrategyManager(
    private val batteryManager: BatteryOptimizationManager
) {
    fun getOptimalDiscoveryStrategy(): DiscoveryStrategy {
        return when {
            batteryManager.isLowPowerMode() -> DiscoveryStrategy.PowerSaving
            batteryManager.getBatteryLevel() < 30 -> DiscoveryStrategy.Balanced
            else -> DiscoveryStrategy.Aggressive
        }
    }
    
    fun getScanParameters(): ScanParameters {
        val strategy = getOptimalDiscoveryStrategy()
        return when (strategy) {
            DiscoveryStrategy.Aggressive -> ScanParameters(
                interval = 2000L,
                duration = 5000L,
                mode = ScanSettings.SCAN_MODE_LOW_LATENCY
            )
            DiscoveryStrategy.Balanced -> ScanParameters(
                interval = 5000L,
                duration = 3000L,
                mode = ScanSettings.SCAN_MODE_BALANCED
            )
            DiscoveryStrategy.PowerSaving -> ScanParameters(
                interval = 15000L,
                duration = 1000L,
                mode = ScanSettings.SCAN_MODE_LOW_POWER
            )
        }
    }
}

data class ScanParameters(
    val interval: Long,
    val duration: Long,
    val mode: Int
)

enum class DiscoveryStrategy {
    Aggressive,
    Balanced,
    PowerSaving
}
```

### REFACTOR阶段
```kotlin
class BatteryOptimizationManager(
    private val context: Context,
    private val config: BatteryConfig = BatteryConfig.default()
) {
    // 添加电池状态监控
    // 添加历史电池使用分析
    // 添加用户偏好设置
}

data class BatteryConfig(
    val lowPowerThreshold: Int,
    val criticalBatteryThreshold: Int,
    val adaptiveScanning: Boolean,
    val backgroundDiscovery: Boolean,
    val powerSaveModePriority: Int
) {
    companion object {
        fun default() = BatteryConfig(
            lowPowerThreshold = 20,
            criticalBatteryThreshold = 10,
            adaptiveScanning = true,
            backgroundDiscovery = true,
            powerSaveModePriority = 1
        )
        
        fun performance() = BatteryConfig(
            lowPowerThreshold = 10,
            criticalBatteryThreshold = 5,
            adaptiveScanning = false,
            backgroundDiscovery = true,
            powerSaveModePriority = 0
        )
    }
}
```

## 验收标准
- [ ] 电池优化状态检测正确
- [ ] 智能扫描间隔调整
- [ ] 低电量模式适配
- [ ] 电池优化白名单请求
- [ ] 电池状态监控

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 前置任务
- [Task 00108c: Android 权限管理](00108c-android-permission-management.md)

## 后续任务
- [Task 00108e: Android 发现UI组件](00108e-android-discovery-ui.md)