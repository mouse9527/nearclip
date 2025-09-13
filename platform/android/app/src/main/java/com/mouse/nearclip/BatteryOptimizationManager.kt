package com.mouse.nearclip

import android.content.Context
import android.os.Build
import android.os.PowerManager
import android.content.Intent
import android.net.Uri
import android.provider.Settings
import android.os.BatteryManager
import android.content.IntentFilter
import android.content.BroadcastReceiver
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class BatteryOptimizationManager(
    private val context: Context,
    private val config: BatteryConfig = BatteryConfig.default()
) {
    private val powerManager: PowerManager by lazy {
        context.getSystemService(Context.POWER_SERVICE) as PowerManager
    }
    
    private val batteryManager: BatteryManager by lazy {
        context.getSystemService(Context.BATTERY_SERVICE) as BatteryManager
    }
    
    // REFACTOR: 添加电池状态监控
    private val _batteryState = MutableStateFlow(BatteryState())
    val batteryState: StateFlow<BatteryState> = _batteryState.asStateFlow()
    
    // REFACTOR: 添加历史使用记录分析
    private val batteryHistory = mutableListOf<BatterySnapshot>()
    private val maxHistorySize = 100
    
    init {
        if (config.adaptiveScanning) {
            startBatteryMonitoring()
        }
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
            isLowPowerMode() -> config.powerSavingScanInterval
            getBatteryLevel() < config.criticalBatteryThreshold -> config.criticalBatteryScanInterval
            getBatteryLevel() < config.lowPowerThreshold -> config.lowPowerScanInterval
            else -> config.normalScanInterval
        }
    }
    
    fun getOptimalScanDuration(): Long {
        return when {
            isLowPowerMode() -> config.powerSavingScanDuration
            getBatteryLevel() < config.criticalBatteryThreshold -> config.criticalBatteryScanDuration
            getBatteryLevel() < config.lowPowerThreshold -> config.lowPowerScanDuration
            else -> config.normalScanDuration
        }
    }
    
    fun shouldReduceDiscoveryFrequency(): Boolean {
        return isLowPowerMode() || getBatteryLevel() < config.discoveryReductionThreshold
    }
    
    fun shouldEnableBackgroundDiscovery(): Boolean {
        return !isLowPowerMode() && 
               getBatteryLevel() > config.backgroundDiscoveryThreshold && 
               !isBatteryOptimizationEnabled() &&
               config.backgroundDiscoveryEnabled
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
    
    fun getOptimalDiscoveryStrategy(): DiscoveryStrategy {
        // REFACTOR: 使用智能策略选择，考虑历史使用模式
        val currentLevel = getBatteryLevel()
        val trend = analyzeBatteryTrend()
        
        return when {
            isLowPowerMode() -> DiscoveryStrategy.PowerSaving
            currentLevel < config.criticalBatteryThreshold || trend == BatteryTrend.RAPID_DRAIN -> 
                DiscoveryStrategy.PowerSaving
            currentLevel < config.lowPowerThreshold || trend == BatteryTrend.DRAINING -> 
                DiscoveryStrategy.Balanced
            else -> DiscoveryStrategy.Aggressive
        }
    }
    
    fun getScanParameters(): ScanParameters {
        val strategy = getOptimalDiscoveryStrategy()
        return when (strategy) {
            DiscoveryStrategy.Aggressive -> ScanParameters(
                interval = config.normalScanInterval,
                duration = config.normalScanDuration,
                mode = android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_LATENCY
            )
            DiscoveryStrategy.Balanced -> ScanParameters(
                interval = config.lowPowerScanInterval,
                duration = config.lowPowerScanDuration,
                mode = android.bluetooth.le.ScanSettings.SCAN_MODE_BALANCED
            )
            DiscoveryStrategy.PowerSaving -> ScanParameters(
                interval = config.powerSavingScanInterval,
                duration = config.powerSavingScanDuration,
                mode = android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_POWER
            )
        }
    }
    
    // REFACTOR: 添加电池状态监控功能
    private fun startBatteryMonitoring() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            val batteryReceiver = object : BroadcastReceiver() {
                override fun onReceive(context: Context?, intent: Intent?) {
                    updateBatteryState()
                }
            }
            
            val filter = IntentFilter().apply {
                addAction(Intent.ACTION_BATTERY_CHANGED)
                addAction(Intent.ACTION_POWER_CONNECTED)
                addAction(Intent.ACTION_POWER_DISCONNECTED)
            }
            
            context.registerReceiver(batteryReceiver, filter)
        }
    }
    
    private fun updateBatteryState() {
        val currentState = BatteryState(
            level = getBatteryLevel(),
            status = getBatteryStatus(),
            isLowPowerMode = isLowPowerMode(),
            isBatteryOptimizationEnabled = isBatteryOptimizationEnabled(),
            timestamp = System.currentTimeMillis()
        )
        
        _batteryState.value = currentState
        
        // 添加到历史记录
        addToHistory(currentState)
    }
    
    private fun addToHistory(state: BatteryState) {
        batteryHistory.add(BatterySnapshot(state, System.currentTimeMillis()))
        
        // 保持历史记录大小限制
        if (batteryHistory.size > maxHistorySize) {
            batteryHistory.removeAt(0)
        }
    }
    
    // REFACTOR: 添加电池使用趋势分析
    private fun analyzeBatteryTrend(): BatteryTrend {
        if (batteryHistory.size < 5) return BatteryTrend.STABLE
        
        val recent = batteryHistory.takeLast(5)
        val levels = recent.map { it.state.level }
        
        val averageDrain = levels.zipWithNext().map { (prev, curr) -> prev - curr }.average()
        
        return when {
            averageDrain > 2.0 -> BatteryTrend.RAPID_DRAIN
            averageDrain > 0.5 -> BatteryTrend.DRAINING
            averageDrain < -0.5 -> BatteryTrend.CHARGING
            else -> BatteryTrend.STABLE
        }
    }
    
    // REFACTOR: 添加性能指标
    fun getPerformanceMetrics(): BatteryMetrics {
        return BatteryMetrics(
            currentLevel = getBatteryLevel(),
            optimalScanInterval = getOptimalScanInterval(),
            optimalScanDuration = getOptimalScanDuration(),
            discoveryStrategy = getOptimalDiscoveryStrategy(),
            batteryTrend = analyzeBatteryTrend(),
            shouldReduceFrequency = shouldReduceDiscoveryFrequency(),
            shouldEnableBackgroundDiscovery = shouldEnableBackgroundDiscovery()
        )
    }
    
    // REFACTOR: 添加智能配置调整
    fun adjustConfigForCurrentConditions(): BatteryConfig {
        val trend = analyzeBatteryTrend()
        val currentLevel = getBatteryLevel()
        
        return when {
            trend == BatteryTrend.RAPID_DRAIN || currentLevel < config.criticalBatteryThreshold ->
                BatteryConfig.powerSaving()
            trend == BatteryTrend.DRAINING || currentLevel < config.lowPowerThreshold ->
                BatteryConfig(lowPowerThreshold = config.lowPowerThreshold,
                            criticalBatteryThreshold = config.criticalBatteryThreshold,
                            discoveryReductionThreshold = config.discoveryReductionThreshold,
                            backgroundDiscoveryThreshold = config.backgroundDiscoveryThreshold + 10,
                            backgroundDiscoveryEnabled = false,
                            adaptiveScanning = true,
                            powerSaveModePriority = 2)
            else ->
                config
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

// REFACTOR: 添加新的数据类
data class BatteryState(
    val level: Int = 50,
    val status: BatteryStatus = BatteryStatus.Unknown,
    val isLowPowerMode: Boolean = false,
    val isBatteryOptimizationEnabled: Boolean = false,
    val timestamp: Long = System.currentTimeMillis()
)

data class BatterySnapshot(
    val state: BatteryState,
    val timestamp: Long
)

data class BatteryMetrics(
    val currentLevel: Int,
    val optimalScanInterval: Long,
    val optimalScanDuration: Long,
    val discoveryStrategy: DiscoveryStrategy,
    val batteryTrend: BatteryTrend,
    val shouldReduceFrequency: Boolean,
    val shouldEnableBackgroundDiscovery: Boolean
)

enum class BatteryTrend {
    CHARGING,
    STABLE,
    DRAINING,
    RAPID_DRAIN
}

enum class BatteryStatus {
    Charging,
    Discharging,
    NotCharging,
    Full,
    Unknown
}

enum class DiscoveryStrategy {
    Aggressive,
    Balanced,
    PowerSaving
}

data class ScanParameters(
    val interval: Long,
    val duration: Long,
    val mode: Int
)

data class BatteryConfig(
    val lowPowerThreshold: Int = 20,
    val criticalBatteryThreshold: Int = 10,
    val discoveryReductionThreshold: Int = 30,
    val backgroundDiscoveryThreshold: Int = 20,
    val backgroundDiscoveryEnabled: Boolean = true,
    val adaptiveScanning: Boolean = true,
    val powerSaveModePriority: Int = 1,
    
    // 扫描间隔配置（毫秒）
    val normalScanInterval: Long = 2000L,
    val lowPowerScanInterval: Long = 5000L,
    val criticalBatteryScanInterval: Long = 10000L,
    val powerSavingScanInterval: Long = 15000L,
    
    // 扫描持续时间配置（毫秒）
    val normalScanDuration: Long = 5000L,
    val lowPowerScanDuration: Long = 3000L,
    val criticalBatteryScanDuration: Long = 2000L,
    val powerSavingScanDuration: Long = 1000L
) {
    companion object {
        fun default() = BatteryConfig()
        
        fun performance() = BatteryConfig(
            lowPowerThreshold = 10,
            criticalBatteryThreshold = 5,
            discoveryReductionThreshold = 15,
            backgroundDiscoveryThreshold = 10,
            adaptiveScanning = false,
            powerSaveModePriority = 0,
            normalScanInterval = 1000L,
            lowPowerScanInterval = 2000L,
            criticalBatteryScanInterval = 5000L,
            powerSavingScanInterval = 8000L,
            normalScanDuration = 8000L,
            lowPowerScanDuration = 5000L,
            criticalBatteryScanDuration = 3000L,
            powerSavingScanDuration = 2000L
        )
        
        fun powerSaving() = BatteryConfig(
            lowPowerThreshold = 30,
            criticalBatteryThreshold = 20,
            discoveryReductionThreshold = 40,
            backgroundDiscoveryThreshold = 30,
            backgroundDiscoveryEnabled = false,
            adaptiveScanning = true,
            powerSaveModePriority = 2,
            normalScanInterval = 5000L,
            lowPowerScanInterval = 10000L,
            criticalBatteryScanInterval = 20000L,
            powerSavingScanInterval = 30000L,
            normalScanDuration = 3000L,
            lowPowerScanDuration = 2000L,
            criticalBatteryScanDuration = 1000L,
            powerSavingScanDuration = 500L
        )
    }
}