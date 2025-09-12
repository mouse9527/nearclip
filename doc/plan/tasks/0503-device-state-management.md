# Task 0503: 实现设备状态管理 (TDD版本)

## 任务描述

按照TDD原则实现设备状态管理，跟踪和协调设备的状态变化。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// tests/unit/device_state_manager_tests.rs
#[cfg(test)]
mod device_state_manager_tests {
    use super::*;
    
    #[test]
    fn test_state_manager_creation() {
        // RED: 测试状态管理器创建
        let manager = DeviceStateManager::new();
        
        assert_eq!(manager.device_count(), 0);
        assert!(manager.is_empty());
    }
    
    #[test]
    fn test_device_state_tracking() {
        // RED: 测试设备状态跟踪
        let mut manager = DeviceStateManager::new();
        let device = DeviceInfo::new("device-001", "Test Device", DeviceType::Mobile);
        
        // 注册设备
        manager.register_device(device).unwrap();
        
        assert_eq!(manager.device_count(), 1);
        assert_eq!(manager.get_device_state("device-001"), Some(&DeviceState::Offline));
        
        // 更新状态
        manager.update_device_state("device-001", DeviceState::Online).unwrap();
        assert_eq!(manager.get_device_state("device-001"), Some(&DeviceState::Online));
    }
    
    #[test]
    fn test_state_transition_validation() {
        // RED: 测试状态转换验证
        let mut manager = DeviceStateManager::new();
        let device = DeviceInfo::new("device-001", "Test Device", DeviceType::Mobile);
        
        manager.register_device(device).unwrap();
        
        // 无效的状态转换
        let result = manager.update_device_state("device-001", DeviceState::Error);
        assert!(matches!(result, Err(StateError::InvalidTransition)));
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```rust
// 只写刚好让测试通过的代码，不多不少
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    Offline,
    Online,
    Connecting,
    Disconnecting,
    Error,
}

#[derive(Debug)]
pub struct DeviceStateManager {
    devices: HashMap<String, DeviceState>,
    state_rules: StateTransitionRules,
}

#[derive(Debug)]
struct StateTransitionRules {
    allowed_transitions: HashMap<DeviceState, Vec<DeviceState>>,
}

impl DeviceStateManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            state_rules: StateTransitionRules::new(),
        }
    }
    
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
    
    pub fn register_device(&mut self, device: DeviceInfo) -> Result<(), StateError> {
        self.devices.insert(device.id().to_string(), DeviceState::Offline);
        Ok(())
    }
    
    pub fn get_device_state(&self, device_id: &str) -> Option<&DeviceState> {
        self.devices.get(device_id)
    }
    
    pub fn update_device_state(&mut self, device_id: &str, new_state: DeviceState) -> Result<(), StateError> {
        let current_state = self.devices.get(device_id)
            .ok_or(StateError::DeviceNotFound)?;
        
        if !self.state_rules.is_valid_transition(current_state, &new_state) {
            return Err(StateError::InvalidTransition);
        }
        
        self.devices.insert(device_id.to_string(), new_state);
        Ok(())
    }
}

impl StateTransitionRules {
    fn new() -> Self {
        let mut allowed = HashMap::new();
        allowed.insert(DeviceState::Offline, vec![DeviceState::Connecting]);
        allowed.insert(DeviceState::Connecting, vec![DeviceState::Online, DeviceState::Offline]);
        allowed.insert(DeviceState::Online, vec![DeviceState::Disconnecting, DeviceState::Offline]);
        allowed.insert(DeviceState::Disconnecting, vec![DeviceState::Offline]);
        
        Self { allowed_transitions: allowed }
    }
    
    fn is_valid_transition(&self, from: &DeviceState, to: &DeviceState) -> bool {
        if let Some(allowed) = self.allowed_transitions.get(from) {
            allowed.contains(to)
        } else {
            false
        }
    }
}

// 从前面任务导入
use super::device_info::{DeviceInfo, DeviceType};

#[derive(Debug)]
pub enum StateError {
    DeviceNotFound,
    InvalidTransition,
    DeviceAlreadyRegistered,
}
```

#### 3. REFACTOR阶段 - 重构代码
```rust
// 重构以消除重复，提高代码质量
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    Offline,
    Online,
    Connecting,
    Disconnecting,
    Busy,
    Maintenance,
    Error(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: DeviceState,
    pub to: DeviceState,
    pub timestamp: SystemTime,
    pub reason: Option<String>,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct DeviceStateHistory {
    pub device_id: String,
    pub transitions: VecDeque<StateTransition>,
    pub current_state: DeviceState,
    pub state_duration: HashMap<DeviceState, Duration>,
    pub last_updated: SystemTime,
}

#[derive(Debug)]
pub struct DeviceStateManager {
    devices: HashMap<String, DeviceStateHistory>,
    state_machine: StateMachine,
    event_bus: StateEventBus,
    config: StateManagerConfig,
    metrics: StateMetrics,
}

#[derive(Debug)]
pub struct StateMachine {
    transition_rules: StateTransitionRules,
    state_handlers: HashMap<DeviceState, Box<dyn StateHandler>>,
    automatic_transitions: HashMap<DeviceState, Vec<AutomaticTransition>>,
}

#[derive(Debug)]
pub struct StateTransitionRules {
    allowed_transitions: HashMap<(DeviceState, DeviceState), TransitionRule>,
    global_rules: Vec<GlobalTransitionRule>,
}

#[derive(Debug, Clone)]
pub struct TransitionRule {
    pub is_allowed: bool,
    pub condition: Option<TransitionCondition>,
    pub timeout: Option<Duration>,
    pub fallback_state: Option<DeviceState>,
}

#[derive(Debug, Clone)]
pub enum TransitionCondition {
    Always,
    NetworkAvailable,
    BatteryAbove(f32),
    UserConfirmation,
    Custom(Box<dyn Fn() -> bool>),
}

#[derive(Debug, Clone)]
pub struct AutomaticTransition {
    pub trigger: TransitionTrigger,
    pub target_state: DeviceState,
    pub condition: TransitionCondition,
}

#[derive(Debug, Clone)]
pub enum TransitionTrigger {
    Timeout(Duration),
    HeartbeatMissed(u32),
    ErrorDetected,
    UserAction(String),
    NetworkChange,
    BatteryLow(f32),
}

#[derive(Debug)]
pub struct StateEventBus {
    subscribers: HashMap<String, Vec<Box<dyn StateEventHandler>>>,
    event_history: VecDeque<StateEvent>,
}

#[derive(Debug, Clone)]
pub struct StateEvent {
    pub event_id: String,
    pub device_id: String,
    pub event_type: StateEventType,
    pub old_state: DeviceState,
    pub new_state: DeviceState,
    pub timestamp: SystemTime,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateEventType {
    StateChanged,
    StateTimeout,
    StateError,
    HeartbeatReceived,
    ConnectionLost,
    MaintenanceRequired,
}

#[derive(Debug, Clone)]
pub struct StateManagerConfig {
    pub max_history_size: usize,
    pub enable_auto_recovery: bool,
    pub heartbeat_timeout: Duration,
    pub connection_timeout: Duration,
    pub maintenance_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct StateMetrics {
    pub total_transitions: u64,
    pub state_timeouts: u32,
    pub recovery_attempts: u32,
    pub average_state_duration: HashMap<DeviceState, Duration>,
    pub device_uptime: HashMap<String, Duration>,
}

pub trait StateHandler {
    fn on_enter(&mut self, device_id: &str, state: &DeviceState) -> Result<(), StateError>;
    fn on_exit(&mut self, device_id: &str, state: &DeviceState) -> Result<(), StateError>;
    fn on_timeout(&mut self, device_id: &str, state: &DeviceState) -> Result<(), StateError>;
}

pub trait StateEventHandler {
    fn handle_event(&mut self, event: &StateEvent) -> Result<(), StateError>;
}

#[derive(Debug)]
pub enum StateError {
    DeviceNotFound,
    InvalidTransition(String),
    TransitionTimeout,
    StateHandlerError(String),
    EventBusError(String),
    ConfigurationError(String),
    ConcurrentModification,
}

impl DeviceStateManager {
    // 重构后的代码，保持测试绿色
    pub fn new() -> Self {
        Self::with_config(StateManagerConfig::default())
    }
    
    pub fn with_config(config: StateManagerConfig) -> Self {
        Self {
            devices: HashMap::new(),
            state_machine: StateMachine::new(),
            event_bus: StateEventBus::new(),
            config,
            metrics: StateMetrics::new(),
        }
    }
    
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
    
    pub fn register_device(&mut self, device: DeviceInfo) -> Result<(), StateError> {
        let device_id = device.id().to_string();
        
        if self.devices.contains_key(&device_id) {
            return Err(StateError::DeviceNotFound);
        }
        
        let history = DeviceStateHistory::new(device_id.clone(), DeviceState::Offline);
        self.devices.insert(device_id, history);
        
        // 发送注册事件
        self.event_bus.publish_event(StateEvent {
            event_id: Uuid::new_v4().to_string(),
            device_id: device.id().to_string(),
            event_type: StateEventType::StateChanged,
            old_state: DeviceState::Unknown,
            new_state: DeviceState::Offline,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        })?;
        
        Ok(())
    }
    
    pub fn unregister_device(&mut self, device_id: &str) -> Result<(), StateError> {
        let history = self.devices.remove(device_id)
            .ok_or(StateError::DeviceNotFound)?;
        
        // 清理相关的指标和事件
        self.metrics.device_uptime.remove(device_id);
        
        // 发送注销事件
        self.event_bus.publish_event(StateEvent {
            event_id: Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            event_type: StateEventType::StateChanged,
            old_state: history.current_state,
            new_state: DeviceState::Unknown,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        })?;
        
        Ok(())
    }
    
    pub fn get_device_state(&self, device_id: &str) -> Option<&DeviceState> {
        self.devices.get(device_id).map(|h| &h.current_state)
    }
    
    pub fn get_device_history(&self, device_id: &str) -> Option<&DeviceStateHistory> {
        self.devices.get(device_id)
    }
    
    pub fn update_device_state(&mut self, device_id: &str, new_state: DeviceState) -> Result<(), StateError> {
        let history = self.devices.get_mut(device_id)
            .ok_or(StateError::DeviceNotFound)?;
        
        let old_state = history.current_state.clone();
        
        // 验证状态转换
        self.state_machine.validate_transition(&old_state, &new_state)?;
        
        // 执行状态退出处理
        self.state_machine.on_state_exit(device_id, &old_state)?;
        
        // 更新状态历史
        let transition = StateTransition {
            from: old_state.clone(),
            to: new_state.clone(),
            timestamp: SystemTime::now(),
            reason: None,
            duration: Duration::ZERO,
        };
        
        history.add_transition(transition);
        history.current_state = new_state.clone();
        history.last_updated = SystemTime::now();
        
        // 执行状态进入处理
        self.state_machine.on_state_enter(device_id, &new_state)?;
        
        // 更新指标
        self.metrics.record_transition(&old_state, &new_state);
        
        // 发送状态变化事件
        self.event_bus.publish_event(StateEvent {
            event_id: Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            event_type: StateEventType::StateChanged,
            old_state,
            new_state,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        })?;
        
        // 检查自动转换
        self.check_automatic_transitions(device_id)?;
        
        Ok(())
    }
    
    pub fn force_update_device_state(&mut self, device_id: &str, new_state: DeviceState, reason: String) -> Result<(), StateError> {
        let history = self.devices.get_mut(device_id)
            .ok_or(StateError::DeviceNotFound)?;
        
        let old_state = history.current_state.clone();
        
        // 强制更新，跳过验证
        let transition = StateTransition {
            from: old_state.clone(),
            to: new_state.clone(),
            timestamp: SystemTime::now(),
            reason: Some(reason.clone()),
            duration: Duration::ZERO,
        };
        
        history.add_transition(transition);
        history.current_state = new_state.clone();
        history.last_updated = SystemTime::now();
        
        // 发送强制更新事件
        let mut metadata = HashMap::new();
        metadata.insert("reason".to_string(), reason);
        
        self.event_bus.publish_event(StateEvent {
            event_id: Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            event_type: StateEventType::StateChanged,
            old_state,
            new_state,
            timestamp: SystemTime::now(),
            metadata,
        })?;
        
        Ok(())
    }
    
    pub fn add_state_handler<H>(&mut self, state: DeviceState, handler: H)
    where
        H: StateHandler + 'static,
    {
        self.state_machine.add_handler(state, Box::new(handler));
    }
    
    pub fn add_event_handler<H>(&mut self, handler: H)
    where
        H: StateEventHandler + 'static,
    {
        self.event_bus.add_handler(Box::new(handler));
    }
    
    pub fn subscribe_to_device_events(&mut self, device_id: String, handler: Box<dyn StateEventHandler>) -> Result<(), StateError> {
        self.event_bus.subscribe_to_device(device_id, handler)
    }
    
    pub fn get_online_devices(&self) -> Vec<&String> {
        self.devices
            .iter()
            .filter(|(_, history)| matches!(history.current_state, DeviceState::Online))
            .map(|(device_id, _)| device_id)
            .collect()
    }
    
    pub fn get_devices_by_state(&self, state: &DeviceState) -> Vec<&String> {
        self.devices
            .iter()
            .filter(|(_, history)| history.current_state == *state)
            .map(|(device_id, _)| device_id)
            .collect()
    }
    
    pub fn start_heartbeat_monitor(&mut self) {
        // 启动心跳监控线程
        std::thread::spawn(move || {
            // 实现心跳监控逻辑
        });
    }
    
    pub fn process_heartbeat(&mut self, device_id: &str) -> Result<(), StateError> {
        let history = self.devices.get_mut(device_id)
            .ok_or(StateError::DeviceNotFound)?;
        
        // 更新最后心跳时间
        history.last_updated = SystemTime::now();
        
        // 发送心跳事件
        self.event_bus.publish_event(StateEvent {
            event_id: Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            event_type: StateEventType::HeartbeatReceived,
            old_state: history.current_state.clone(),
            new_state: history.current_state.clone(),
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        })?;
        
        Ok(())
    }
    
    pub fn get_metrics(&self) -> &StateMetrics {
        &self.metrics
    }
    
    pub fn get_device_uptime(&self, device_id: &str) -> Option<Duration> {
        self.metrics.device_uptime.get(device_id).copied()
    }
    
    fn check_automatic_transitions(&mut self, device_id: &str) -> Result<(), StateError> {
        let current_state = self.get_device_state(device_id)?.clone();
        
        if let Some(transitions) = self.state_machine.automatic_transitions.get(&current_state) {
            for transition in transitions.iter() {
                if self.should_trigger_automatic_transition(transition) {
                    self.update_device_state(device_id, transition.target_state.clone())?;
                }
            }
        }
        
        Ok(())
    }
    
    fn should_trigger_automatic_transition(&self, transition: &AutomaticTransition) -> bool {
        match &transition.condition {
            TransitionCondition::Always => true,
            // 其他条件检查...
            _ => true, // 简化实现
        }
    }
}

impl DeviceStateHistory {
    fn new(device_id: String, initial_state: DeviceState) -> Self {
        let now = SystemTime::now();
        let mut state_duration = HashMap::new();
        state_duration.insert(initial_state.clone(), Duration::ZERO);
        
        Self {
            device_id,
            transitions: VecDeque::new(),
            current_state: initial_state,
            state_duration,
            last_updated: now,
        }
    }
    
    fn add_transition(&mut self, transition: StateTransition) {
        // 更新状态持续时间
        if let Some(duration) = self.state_duration.get_mut(&transition.from) {
            *duration += transition.duration;
        }
        
        self.transitions.push_back(transition);
        
        // 限制历史记录大小
        if self.transitions.len() > 100 {
            self.transitions.pop_front();
        }
    }
}

impl StateMachine {
    fn new() -> Self {
        Self {
            transition_rules: StateTransitionRules::default(),
            state_handlers: HashMap::new(),
            automatic_transitions: HashMap::new(),
        }
    }
    
    fn validate_transition(&self, from: &DeviceState, to: &DeviceState) -> Result<(), StateError> {
        self.transition_rules.validate(from, to)
    }
    
    fn on_state_enter(&mut self, device_id: &str, state: &DeviceState) -> Result<(), StateError> {
        if let Some(handler) = self.state_handlers.get_mut(state) {
            handler.on_enter(device_id, state)
        } else {
            Ok(())
        }
    }
    
    fn on_state_exit(&mut self, device_id: &str, state: &DeviceState) -> Result<(), StateError> {
        if let Some(handler) = self.state_handlers.get_mut(state) {
            handler.on_exit(device_id, state)
        } else {
            Ok(())
        }
    }
    
    fn add_handler(&mut self, state: DeviceState, handler: Box<dyn StateHandler>) {
        self.state_handlers.insert(state, handler);
    }
}

impl StateTransitionRules {
    fn default() -> Self {
        let mut allowed = HashMap::new();
        
        // 定义所有允许的状态转换
        allowed.insert(
            (DeviceState::Offline, DeviceState::Connecting),
            TransitionRule {
                is_allowed: true,
                condition: Some(TransitionCondition::NetworkAvailable),
                timeout: Some(Duration::from_secs(30)),
                fallback_state: Some(DeviceState::Offline),
            }
        );
        
        allowed.insert(
            (DeviceState::Connecting, DeviceState::Online),
            TransitionRule {
                is_allowed: true,
                condition: None,
                timeout: None,
                fallback_state: None,
            }
        );
        
        allowed.insert(
            (DeviceState::Connecting, DeviceState::Offline),
            TransitionRule {
                is_allowed: true,
                condition: None,
                timeout: None,
                fallback_state: None,
            }
        );
        
        allowed.insert(
            (DeviceState::Online, DeviceState::Disconnecting),
            TransitionRule {
                is_allowed: true,
                condition: None,
                timeout: None,
                fallback_state: None,
            }
        );
        
        allowed.insert(
            (DeviceState::Online, DeviceState::Offline),
            TransitionRule {
                is_allowed: true,
                condition: None,
                timeout: None,
                fallback_state: None,
            }
        );
        
        allowed.insert(
            (DeviceState::Disconnecting, DeviceState::Offline),
            TransitionRule {
                is_allowed: true,
                condition: None,
                timeout: None,
                fallback_state: None,
            }
        );
        
        Self {
            allowed_transitions: allowed,
            global_rules: Vec::new(),
        }
    }
    
    fn validate(&self, from: &DeviceState, to: &DeviceState) -> Result<(), StateError> {
        if let Some(rule) = self.allowed_transitions.get(&(from.clone(), to.clone())) {
            if rule.is_allowed {
                Ok(())
            } else {
                Err(StateError::InvalidTransition(format!(
                    "Transition from {:?} to {:?} is not allowed", from, to
                )))
            }
        } else {
            Err(StateError::InvalidTransition(format!(
                "No transition rule defined from {:?} to {:?}", from, to
            )))
        }
    }
}

impl StateEventBus {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            event_history: VecDeque::with_capacity(1000),
        }
    }
    
    fn publish_event(&mut self, event: StateEvent) -> Result<(), StateError> {
        // 通知所有订阅者
        if let Some(handlers) = self.subscribers.get_mut("*") {
            for handler in handlers.iter_mut() {
                if let Err(e) = handler.handle_event(&event) {
                    return Err(StateError::EventBusError(e.to_string()));
                }
            }
        }
        
        // 通知设备特定订阅者
        if let Some(handlers) = self.subscribers.get_mut(&event.device_id) {
            for handler in handlers.iter_mut() {
                if let Err(e) = handler.handle_event(&event) {
                    return Err(StateError::EventBusError(e.to_string()));
                }
            }
        }
        
        // 记录事件历史
        self.event_history.push_back(event);
        if self.event_history.len() > 1000 {
            self.event_history.pop_front();
        }
        
        Ok(())
    }
    
    fn add_handler(&mut self, handler: Box<dyn StateEventHandler>) {
        self.subscribers
            .entry("*".to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }
    
    fn subscribe_to_device(&mut self, device_id: String, handler: Box<dyn StateEventHandler>) -> Result<(), StateError> {
        self.subscribers
            .entry(device_id)
            .or_insert_with(Vec::new)
            .push(handler);
        Ok(())
    }
}

impl StateMetrics {
    fn new() -> Self {
        Self {
            total_transitions: 0,
            state_timeouts: 0,
            recovery_attempts: 0,
            average_state_duration: HashMap::new(),
            device_uptime: HashMap::new(),
        }
    }
    
    fn record_transition(&mut self, from: &DeviceState, to: &DeviceState) {
        self.total_transitions += 1;
        
        // 更新平均状态持续时间
        let entry = self.average_state_duration.entry(from.clone()).or_insert(Duration::ZERO);
        *entry = Duration::from_secs((entry.as_secs() + 1) / 2); // 简化的平均计算
    }
}

impl Default for StateManagerConfig {
    fn default() -> Self {
        Self {
            max_history_size: 100,
            enable_auto_recovery: true,
            heartbeat_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(30),
            maintenance_interval: Duration::from_secs(3600),
        }
    }
}

// 从前面任务导入
use super::device_info::{DeviceInfo, DeviceType};
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为domain层的状态管理：

```rust
// rust-core/domain/device/state.rs
pub struct DeviceStateManager {
    // 设备状态管理实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查

## 依赖任务

- [设备存储管理](0502-device-storage-management.md)

## 后续任务

- [Task 0504: 实现设备信任机制](0504-device-trust-mechanism.md)
- [Task 0505: 实现设备生命周期管理](0505-device-lifecycle.md)
- [Task 0506: 实现设备群组管理](0506-device-group-management.md)