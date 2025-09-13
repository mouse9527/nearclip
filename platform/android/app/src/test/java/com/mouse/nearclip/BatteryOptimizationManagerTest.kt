package com.mouse.nearclip

import org.junit.Test
import org.junit.Assert.*

class BatteryOptimizationManagerTest {
    
    @Test
    fun testBatteryConfigDefaults() {
        // RED: 测试默认配置
        val config = BatteryConfig.default()
        
        assertEquals(20, config.lowPowerThreshold)
        assertEquals(10, config.criticalBatteryThreshold)
        assertEquals(30, config.discoveryReductionThreshold)
        assertEquals(20, config.backgroundDiscoveryThreshold)
        assertTrue(config.backgroundDiscoveryEnabled)
        assertTrue(config.adaptiveScanning)
        assertEquals(1, config.powerSaveModePriority)
        
        // 扫描间隔配置
        assertEquals(2000L, config.normalScanInterval)
        assertEquals(5000L, config.lowPowerScanInterval)
        assertEquals(10000L, config.criticalBatteryScanInterval)
        assertEquals(15000L, config.powerSavingScanInterval)
        
        // 扫描持续时间配置
        assertEquals(5000L, config.normalScanDuration)
        assertEquals(3000L, config.lowPowerScanDuration)
        assertEquals(2000L, config.criticalBatteryScanDuration)
        assertEquals(1000L, config.powerSavingScanDuration)
    }
    
    @Test
    fun testBatteryConfigPerformance() {
        // RED: 测试性能配置
        val config = BatteryConfig.performance()
        
        assertEquals(10, config.lowPowerThreshold)
        assertEquals(5, config.criticalBatteryThreshold)
        assertEquals(15, config.discoveryReductionThreshold)
        assertEquals(10, config.backgroundDiscoveryThreshold)
        assertFalse(config.adaptiveScanning)
        assertEquals(0, config.powerSaveModePriority)
        
        // 更短的扫描间隔（更积极的发现）
        assertEquals(1000L, config.normalScanInterval)
        assertEquals(2000L, config.lowPowerScanInterval)
        assertEquals(5000L, config.criticalBatteryScanInterval)
        assertEquals(8000L, config.powerSavingScanInterval)
        
        // 更长的扫描持续时间
        assertEquals(8000L, config.normalScanDuration)
        assertEquals(5000L, config.lowPowerScanDuration)
        assertEquals(3000L, config.criticalBatteryScanDuration)
        assertEquals(2000L, config.powerSavingScanDuration)
    }
    
    @Test
    fun testBatteryConfigPowerSaving() {
        // RED: 测试省电配置
        val config = BatteryConfig.powerSaving()
        
        assertEquals(30, config.lowPowerThreshold)
        assertEquals(20, config.criticalBatteryThreshold)
        assertEquals(40, config.discoveryReductionThreshold)
        assertEquals(30, config.backgroundDiscoveryThreshold)
        assertFalse(config.backgroundDiscoveryEnabled)
        assertTrue(config.adaptiveScanning)
        assertEquals(2, config.powerSaveModePriority)
        
        // 更长的扫描间隔（更省电）
        assertEquals(5000L, config.normalScanInterval)
        assertEquals(10000L, config.lowPowerScanInterval)
        assertEquals(20000L, config.criticalBatteryScanInterval)
        assertEquals(30000L, config.powerSavingScanInterval)
        
        // 更短的扫描持续时间
        assertEquals(3000L, config.normalScanDuration)
        assertEquals(2000L, config.lowPowerScanDuration)
        assertEquals(1000L, config.criticalBatteryScanDuration)
        assertEquals(500L, config.powerSavingScanDuration)
    }
    
    @Test
    fun testDiscoveryStrategyEnum() {
        // RED: 测试发现策略枚举
        val strategies = DiscoveryStrategy.values()
        assertEquals(3, strategies.size)
        
        assertTrue(strategies.contains(DiscoveryStrategy.Aggressive))
        assertTrue(strategies.contains(DiscoveryStrategy.Balanced))
        assertTrue(strategies.contains(DiscoveryStrategy.PowerSaving))
    }
    
    @Test
    fun testBatteryStatusEnum() {
        // RED: 测试电池状态枚举
        val statuses = BatteryStatus.values()
        assertEquals(5, statuses.size)
        
        assertTrue(statuses.contains(BatteryStatus.Charging))
        assertTrue(statuses.contains(BatteryStatus.Discharging))
        assertTrue(statuses.contains(BatteryStatus.NotCharging))
        assertTrue(statuses.contains(BatteryStatus.Full))
        assertTrue(statuses.contains(BatteryStatus.Unknown))
    }
    
    @Test
    fun testScanParametersDataClass() {
        // RED: 测试扫描参数数据类
        val params = ScanParameters(
            interval = 2000L,
            duration = 5000L,
            mode = 2
        )
        
        assertEquals(2000L, params.interval)
        assertEquals(5000L, params.duration)
        assertEquals(2, params.mode)
        
        // 测试equals和hashCode
        val params2 = ScanParameters(2000L, 5000L, 2)
        assertEquals(params, params2)
        assertEquals(params.hashCode(), params2.hashCode())
    }
    
    @Test
    fun testScanParametersCopy() {
        // RED: 测试扫描参数复制
        val original = ScanParameters(1000L, 3000L, 1)
        val copied = original.copy(interval = 2000L)
        
        assertEquals(2000L, copied.interval)
        assertEquals(3000L, copied.duration)
        assertEquals(1, copied.mode)
    }
}