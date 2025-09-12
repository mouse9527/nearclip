# Task 0106: 实现智能传输选择器 (TDD版本)

## 任务描述

按照TDD原则实现智能传输选择器，根据网络环境、设备状态和用户偏好自动选择最佳的传输方式（WiFi或BLE）。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/transport_selector_tests.rs
#[cfg(test)]
mod transport_selector_tests {
    use super::*;
    
    #[test]
    fn test_transport_selector_creation() {
        // RED: 测试传输选择器创建
        let selector = TransportSelector::new();
        
        // 应该有默认的传输选择策略
        assert_eq!(selector.get_strategy(), SelectionStrategy::Balanced);
        assert_eq!(selector.get_available_transports().len(), 0);
    }
    
    #[tokio::test]
    async fn test_wifi_preferred_when_available() {
        // RED: 测试WiFi优先选择逻辑
        let mut selector = TransportSelector::new();
        
        // 配置上下文：WiFi可用且质量良好
        let context = ContextInfo {
            network_context: NetworkContext {
                wifi_available: true,
                wifi_quality: NetworkQuality::Good,
                ble_available: true,
                ble_quality: NetworkQuality::Excellent,
            },
            battery_context: BatteryContext {
                battery_level: 80,
                is_charging: false,
            },
            user_preferences: UserPreferences::default(),
        };
        
        let available_transports = vec![
            TransportType::Wifi,
            TransportType::Ble,
        ];
        
        let result = selector.select_best_transport(&available_transports, &context).await;
        
        assert!(result.is_ok());
        let selection = result.unwrap();
        assert_eq!(selection.selected_transport, TransportType::Wifi);
        assert_eq!(selection.reason, SelectionReason::BetterQuality);
    }
    
    #[tokio::test]
    async fn test_ble_fallback_when_wifi_unavailable() {
        // RED: 测试WiFi不可用时BLE回退逻辑
        let mut selector = TransportSelector::new();
        
        // 配置上下文：WiFi不可用，BLE可用
        let context = ContextInfo {
            network_context: NetworkContext {
                wifi_available: false,
                wifi_quality: NetworkQuality::Poor,
                ble_available: true,
                ble_quality: NetworkQuality::Good,
            },
            battery_context: BatteryContext {
                battery_level: 50,
                is_charging: false,
            },
            user_preferences: UserPreferences::default(),
        };
        
        let available_transports = vec![
            TransportType::Wifi,
            TransportType::Ble,
        ];
        
        let result = selector.select_best_transport(&available_transports, &context).await;
        
        assert!(result.is_ok());
        let selection = result.unwrap();
        assert_eq!(selection.selected_transport, TransportType::Ble);
        assert_eq!(selection.reason, SelectionReason::Fallback);
    }
    
    #[tokio::test]
    async fn test_battery_aware_selection() {
        // RED: 测试电池感知的选择逻辑
        let mut selector = TransportSelector::new();
        
        // 配置上下文：低电量状态，优先BLE
        let context = ContextInfo {
            network_context: NetworkContext {
                wifi_available: true,
                wifi_quality: NetworkQuality::Good,
                ble_available: true,
                ble_quality: NetworkQuality::Good,
            },
            battery_context: BatteryContext {
                battery_level: 15, // 低电量
                is_charging: false,
            },
            user_preferences: UserPreferences::default(),
        };
        
        let available_transports = vec![
            TransportType::Wifi,
            TransportType::Ble,
        ];
        
        let result = selector.select_best_transport(&available_transports, &context).await;
        
        assert!(result.is_ok());
        let selection = result.unwrap();
        assert_eq!(selection.selected_transport, TransportType::Ble);
        assert_eq!(selection.reason, SelectionReason::BatteryConservation);
    }
    
    #[tokio::test]
    async fn test_user_preference_override() {
        // RED: 测试用户偏好覆盖自动选择
        let mut selector = TransportSelector::new();
        
        // 配置上下文：用户明确偏好BLE
        let context = ContextInfo {
            network_context: NetworkContext {
                wifi_available: true,
                wifi_quality: NetworkQuality::Excellent,
                ble_available: true,
                ble_quality: NetworkQuality::Good,
            },
            battery_context: BatteryContext {
                battery_level: 80,
                is_charging: false,
            },
            user_preferences: UserPreferences {
                preferred_transport: Some(TransportType::Ble),
                ..Default::default()
            },
        };
        
        let available_transports = vec![
            TransportType::Wifi,
            TransportType::Ble,
        ];
        
        let result = selector.select_best_transport(&available_transports, &context).await;
        
        assert!(result.is_ok());
        let selection = result.unwrap();
        assert_eq!(selection.selected_transport, TransportType::Ble);
        assert_eq!(selection.reason, SelectionReason::UserPreference);
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
#[derive(Debug, Clone, PartialEq)]
pub enum TransportType {
    Wifi,
    Ble,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NetworkQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStrategy {
    Performance,   // 优先性能
    Battery,       // 优先省电
    Balanced,      // 平衡策略
    Custom,       // 自定义策略
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionReason {
    BetterQuality,
    Fallback,
    BatteryConservation,
    UserPreference,
    NetworkCondition,
    Default,
}

#[derive(Debug)]
pub struct SelectionResult {
    pub selected_transport: TransportType,
    pub reason: SelectionReason,
    pub confidence: f32,
    pub alternative: Option<TransportType>,
}

#[derive(Debug, Clone)]
pub struct NetworkContext {
    pub wifi_available: bool,
    pub wifi_quality: NetworkQuality,
    pub ble_available: bool,
    pub ble_quality: NetworkQuality,
}

#[derive(Debug, Clone)]
pub struct BatteryContext {
    pub battery_level: u8,
    pub is_charging: bool,
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub preferred_transport: Option<TransportType>,
    pub battery_saving_mode: bool,
    pub performance_mode: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_transport: None,
            battery_saving_mode: false,
            performance_mode: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextInfo {
    pub network_context: NetworkContext,
    pub battery_context: BatteryContext,
    pub user_preferences: UserPreferences,
}

pub struct TransportSelector {
    strategy: SelectionStrategy,
    available_transports: Vec<TransportType>,
}

impl TransportSelector {
    pub fn new() -> Self {
        Self {
            strategy: SelectionStrategy::Balanced,
            available_transports: Vec::new(),
        }
    }
    
    pub fn get_strategy(&self) -> SelectionStrategy {
        self.strategy
    }
    
    pub fn get_available_transports(&self) -> &[TransportType] {
        &self.available_transports
    }
    
    pub async fn select_best_transport(
        &self,
        available_transports: &[TransportType],
        context: &ContextInfo,
    ) -> Result<SelectionResult, SelectionError> {
        // 用户偏好优先
        if let Some(preferred) = &context.user_preferences.preferred_transport {
            if available_transports.contains(preferred) {
                return Ok(SelectionResult {
                    selected_transport: preferred.clone(),
                    reason: SelectionReason::UserPreference,
                    confidence: 1.0,
                    alternative: None,
                });
            }
        }
        
        // WiFi优先策略
        if available_transports.contains(&TransportType::Wifi) && 
           context.network_context.wifi_available {
            return Ok(SelectionResult {
                selected_transport: TransportType::Wifi,
                reason: SelectionReason::BetterQuality,
                confidence: 0.8,
                alternative: Some(TransportType::Ble),
            });
        }
        
        // BLE回退
        if available_transports.contains(&TransportType::Ble) && 
           context.network_context.ble_available {
            return Ok(SelectionResult {
                selected_transport: TransportType::Ble,
                reason: SelectionReason::Fallback,
                confidence: 0.6,
                alternative: None,
            });
        }
        
        // 低电量优先BLE
        if context.battery_context.battery_level < 20 && 
           available_transports.contains(&TransportType::Ble) {
            return Ok(SelectionResult {
                selected_transport: TransportType::Ble,
                reason: SelectionReason::BatteryConservation,
                confidence: 0.7,
                alternative: None,
            });
        }
        
        Err(SelectionError::NoSuitableTransport)
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
#[derive(Debug)]
pub struct TransportSelector {
    strategy: SelectionStrategy,
    scoring_weights: ScoringWeights,
    selection_rules: Vec<SelectionRule>,
}

#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub quality_weight: f32,
    pub battery_weight: f32,
    pub latency_weight: f32,
    pub bandwidth_weight: f32,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            quality_weight: 0.4,
            battery_weight: 0.3,
            latency_weight: 0.2,
            bandwidth_weight: 0.1,
        }
    }
}

#[derive(Debug)]
pub struct SelectionRule {
    pub name: String,
    pub condition: Box<dyn Fn(&ContextInfo) -> bool + Send + Sync>,
    pub action: Box<dyn Fn(&[TransportType]) -> Option<TransportType> + Send + Sync>,
    pub priority: u32,
}

impl TransportSelector {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self {
            strategy: SelectionStrategy::Balanced,
            scoring_weights: ScoringWeights::default(),
            selection_rules: Self::create_default_rules(),
        }
    }
    
    pub fn with_strategy(mut self, strategy: SelectionStrategy) -> Self {
        self.strategy = strategy;
        self.scoring_weights = Self::get_weights_for_strategy(&strategy);
        self
    }
    
    fn get_weights_for_strategy(strategy: &SelectionStrategy) -> ScoringWeights {
        match strategy {
            SelectionStrategy::Performance => ScoringWeights {
                quality_weight: 0.5,
                battery_weight: 0.1,
                latency_weight: 0.3,
                bandwidth_weight: 0.1,
            },
            SelectionStrategy::Battery => ScoringWeights {
                quality_weight: 0.2,
                battery_weight: 0.6,
                latency_weight: 0.1,
                bandwidth_weight: 0.1,
            },
            SelectionStrategy::Balanced => ScoringWeights::default(),
            SelectionStrategy::Custom => ScoringWeights::default(),
        }
    }
    
    fn create_default_rules() -> Vec<SelectionRule> {
        vec![
            SelectionRule {
                name: "User Preference".to_string(),
                condition: Box::new(|ctx| ctx.user_preferences.preferred_transport.is_some()),
                action: Box::new(|transports| {
                    ctx.user_preferences.preferred_transport.clone()
                        .filter(|pref| transports.contains(pref))
                }),
                priority: 100,
            },
            SelectionRule {
                name: "Low Battery".to_string(),
                condition: Box::new(|ctx| ctx.battery_context.battery_level < 20 && !ctx.battery_context.is_charging),
                action: Box::new(|transports| {
                    transports.iter()
                        .find(|&t| t == &TransportType::Ble)
                        .cloned()
                }),
                priority: 90,
            },
            SelectionRule {
                name: "WiFi Preferred".to_string(),
                condition: Box::new(|ctx| ctx.network_context.wifi_available),
                action: Box::new(|transports| {
                    transports.iter()
                        .find(|&t| t == &TransportType::Wifi)
                        .cloned()
                }),
                priority: 80,
            },
        ]
    }
    
    pub async fn select_best_transport(
        &self,
        available_transports: &[TransportType],
        context: &ContextInfo,
    ) -> Result<SelectionResult, SelectionError> {
        if available_transports.is_empty() {
            return Err(SelectionError::NoAvailableTransports);
        }
        
        // 应用选择规则
        for rule in &self.selection_rules {
            if (rule.condition)(context) {
                if let Some(selected) = (rule.action)(available_transports) {
                    return Ok(SelectionResult {
                        selected_transport: selected,
                        reason: self.map_rule_to_reason(&rule.name),
                        confidence: self.calculate_confidence(&selected, context),
                        alternative: self.find_alternative(&selected, available_transports),
                    });
                }
            }
        }
        
        // 基于评分的智能选择
        self.select_by_scoring(available_transports, context).await
    }
    
    async fn select_by_scoring(
        &self,
        available_transports: &[TransportType],
        context: &ContextInfo,
    ) -> Result<SelectionResult, SelectionError> {
        let mut scored_transports: Vec<_> = available_transports.iter()
            .map(|transport| {
                let score = self.calculate_transport_score(transport, context);
                (transport, score)
            })
            .collect();
        
        scored_transports.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        if let Some((best_transport, best_score)) = scored_transports.first() {
            let alternative = scored_transports.get(1).map(|(t, _)| *t);
            
            Ok(SelectionResult {
                selected_transport: (*best_transport).clone(),
                reason: SelectionReason::BetterQuality,
                confidence: *best_score,
                alternative,
            })
        } else {
            Err(SelectionError::NoSuitableTransport)
        }
    }
    
    fn calculate_transport_score(&self, transport: &TransportType, context: &ContextInfo) -> f32 {
        let quality_score = self.calculate_quality_score(transport, context);
        let battery_score = self.calculate_battery_score(transport, context);
        let latency_score = self.calculate_latency_score(transport, context);
        let bandwidth_score = self.calculate_bandwidth_score(transport, context);
        
        quality_score * self.scoring_weights.quality_weight +
        battery_score * self.scoring_weights.battery_weight +
        latency_score * self.scoring_weights.latency_weight +
        bandwidth_score * self.scoring_weights.bandwidth_weight
    }
    
    fn calculate_quality_score(&self, transport: &TransportType, context: &ContextInfo) -> f32 {
        match transport {
            TransportType::Wifi if context.network_context.wifi_available => {
                match context.network_context.wifi_quality {
                    NetworkQuality::Excellent => 1.0,
                    NetworkQuality::Good => 0.8,
                    NetworkQuality::Fair => 0.6,
                    NetworkQuality::Poor => 0.3,
                }
            },
            TransportType::Ble if context.network_context.ble_available => {
                match context.network_context.ble_quality {
                    NetworkQuality::Excellent => 0.9,
                    NetworkQuality::Good => 0.7,
                    NetworkQuality::Fair => 0.5,
                    NetworkQuality::Poor => 0.2,
                }
            },
            _ => 0.0,
        }
    }
    
    fn calculate_battery_score(&self, transport: &TransportType, context: &ContextInfo) -> f32 {
        let battery_level = context.battery_context.battery_level;
        let is_charging = context.battery_context.is_charging;
        
        match transport {
            TransportType::Ble => {
                // BLE更省电
                if is_charging { 0.8 } else { 0.9 }
            },
            TransportType::Wifi => {
                // WiFi耗电更多，但充电时影响较小
                if is_charging { 0.9 } else { 0.6 }
            },
            _ => 0.5,
        }
    }
}
```

### 必须编写的测试类型

#### 1. 单元测试 (Unit Tests)
- 选择器配置测试
- 策略应用测试
- 评分计算测试
- 规则引擎测试
- 上下文处理测试

#### 2. 集成测试 (Integration Tests)
- 与传输层集成测试
- 与上下文提供者集成测试
- 与用户偏好系统集成测试

#### 3. 场景测试 (Scenario Tests)
- 网络变化场景测试
- 电池变化场景测试
- 用户交互场景测试
- 错误恢复场景测试

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%
- **集成测试覆盖率**: > 85%
- **场景测试覆盖率**: > 80%

## Clean Architecture要求

### 依赖倒置原则
传输选择器应该作为Use Case，依赖domain层的接口：

```rust
// rust-core/application/use_cases/transport_selection.rs
pub struct TransportSelector {
    context_provider: Arc<dyn ContextProvider>,
    transport_factory: Arc<dyn TransportFactory>,
    user_preferences: Arc<dyn UserPreferencesRepository>,
}
```

### 接口隔离原则
定义选择器相关的接口：

```rust
// rust-core/domain/interfaces/selection.rs
#[async_trait]
pub trait ContextProvider: Send + Sync {
    async fn get_network_context(&self) -> Result<NetworkContext, ContextError>;
    async fn get_battery_context(&self) -> Result<BatteryContext, ContextError>;
    async fn get_user_preferences(&self) -> Result<UserPreferences, ContextError>;
}

#[async_trait]
pub trait TransportSelector: Send + Sync {
    async fn select_best_transport(&self, context: &ContextInfo) -> Result<SelectionResult, SelectionError>;
    fn get_selection_strategy(&self) -> SelectionStrategy;
}
```

## XP实践要求

### 1. 结对编程
- 所有选择器代码必须两人结对完成
- 一人负责评分算法，另一人负责规则引擎
- 定期切换角色

### 2. 持续集成
- 每次提交都必须通过所有选择器测试
- 自动化构建和测试
- 快速反馈循环

### 3. 代码审查
- 所有代码必须经过同行审查
- 检查TDD流程是否正确遵循
- 确保选择逻辑的合理性

### 4. 重构勇气
- 定期重构以保持代码质量
- 消除选择逻辑中的重复代码
- 提高代码可读性

### 5. 简单设计
- 只实现当前需要的选择功能
- 避免过度设计选择算法
- 保持选择逻辑简单明了

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 遵循Clean Architecture原则
- [ ] 通过代码审查
- [ ] 集成测试通过
- [ ] 场景测试通过
- [ ] 文档更新完整

## 依赖任务

- Task 0103: 实现传输层抽象
- Task 0104: 实现mDNS发现
- Task 0105: 实现BLE发现

## 后续任务

- Task 0107: 实现设备连接管理
- Task 0108: 实现安全配对机制
- Task 0109: 实现混合传输管理