# 开发计划

NearClip跨设备剪贴板同步工具的开发计划和任务管理。

## 计划概览

本项目采用严格的TDD（测试驱动开发）、Clean Architecture（清洁架构）和XP（极限编程）最佳实践进行开发。

## 快速导航

- [项目总览](../README.md) - 项目概述和技术架构
- [架构设计](../architecture/) - Clean Architecture和技术架构
- [开发指南](../guides/) - TDD、XP实践和编码标准
- [用户故事](stories/) - INVEST用户故事
- [具体任务](tasks/) - 正交任务分解

## 开发方法论

### TDD (测试驱动开发)
- **红绿重构循环**: 严格遵循RED→GREEN→REFACTOR流程
- **测试优先**: 先写失败测试，再写最小实现代码
- **覆盖率要求**: 单元测试 > 90%，集成测试 > 80%

### Clean Architecture (清洁架构)
- **四层架构**: Entities → Use Cases → Interface Adapters → Frameworks
- **依赖倒置**: 内层不依赖外层，所有依赖指向内部
- **接口隔离**: 最小必要的接口，避免接口污染

### XP (极限编程)
- **结对编程**: 所有生产代码必须两人结对开发
- **持续集成**: 每次提交通过所有测试，快速反馈
- **简单设计**: KISS、YAGNI、DRY原则

## 开发里程碑

### 里程碑 1: 基础架构 (高优先级)
- **目标**: 建立混合传输架构，支持基本的设备发现和连接
- **预估时间**: 3-4周

### 里程碑 2: 安全基础 (高优先级)  
- **目标**: 建立端到端加密的配对机制
- **预估时间**: 2-3周

### 里程碑 3: 核心功能 (中优先级)
- **目标**: 实现手动文本传输和基础设备管理
- **预估时间**: 2-3周

### 里程碑 4: 增强功能 (低优先级)
- **目标**: 实现自动剪贴板同步
- **预估时间**: 2周

### 里程碑 5: 多端适配 (中优先级)
- **目标**: 完成各平台的特定适配和优化
- **预估时间**: 2-3周

## 任务结构

本项目采用细粒度的任务分解，每个任务设计为10分钟内可完成，确保开发过程的快速反馈和进度跟踪。

### Story 001: 设备发现
- [设备抽象层](tasks/0101-device-abstraction-layer.md) - 实现统一的设备抽象接口

### Story 002: 安全配对
- [加密密钥结构](tasks/0201-encryption-key-structure.md) - 实现AES-256-GCM加密密钥结构
- [密钥生成器](tasks/0202-key-generator.md) - 实现安全的随机密钥生成功能

### Story 003: 手动文本传输  
- [传输会话](tasks/0301-transfer-session.md) - 定义文本传输会话的基础结构
- [传输错误处理](tasks/0302-transfer-error-handling.md) - 实现传输过程中的错误处理机制
- [传输事件系统](tasks/0303-transfer-event-system.md) - 实现传输状态变化的事件通知
- [传输队列管理](tasks/0304-transfer-queue-management.md) - 实现多传输任务的排队调度
- [传输协议处理](tasks/0305-transfer-protocol-handling.md) - 定义传输数据格式和协议规范
- [传输进度跟踪](tasks/0306-transfer-progress-tracking.md) - 实现实时传输进度显示

### Story 004: 自动同步
- [剪贴板监控器](tasks/0401-clipboard-monitoring.md) - 实现系统剪贴板变化检测
- [同步策略](tasks/0402-sync-strategy.md) - 实现自动同步的决策逻辑
- [内容变化检测](tasks/0403-content-change-detection.md) - 实现智能内容变化检测算法
- [同步冲突处理](tasks/0404-sync-conflict-handling.md) - 解决多设备同时修改的冲突

### Story 005: 设备管理
- [设备信息结构](tasks/0501-device-info-structure.md) - 定义设备基本属性和元数据
- [设备存储管理](tasks/0502-device-storage-management.md) - 实现设备信息的持久化存储
- [设备状态管理](tasks/0503-device-state-management.md) - 跟踪和协调设备状态变化

## 任务设计原则

- **10分钟原则**: 每个任务都能在10分钟内完成，确保快速反馈
- **正交分解**: 任务之间尽量独立，减少依赖关系
- **TDD循环**: 每个任务都包含完整的RED→GREEN→REFACTOR流程
- **Clean Architecture**: 明确区分domain、infrastructure等层次
- **验收标准**: 每个任务都有明确的验收标准和测试覆盖率要求

## 文档结构

```
doc/
├── README.md                    # 项目总览  
├── CLAUDE.md                    # Claude开发指导
├── clean-architecture.md        # Clean Architecture设计
├── plan/                        # 开发计划 (本目录)
│   ├── README.md               # 计划总览 (本文件)
│   ├── stories/                # INVEST用户故事
│   │   ├── 001-device-discovery.md
│   │   ├── 002-secure-pairing.md
│   │   ├── 003-manual-text-transfer.md
│   │   ├── 004-automatic-sync.md
│   │   └── 005-device-management.md
│   └── tasks/                  # TDD开发任务
│       ├── 0101-device-abstraction-layer.md
│       ├── 0201-encryption-key-structure.md
│       ├── 0202-key-generator.md
│       ├── 0301-transfer-session.md
│       ├── 0302-transfer-error-handling.md
│       ├── 0303-transfer-event-system.md
│       ├── 0304-transfer-queue-management.md
│       ├── 0305-transfer-protocol-handling.md
│       ├── 0306-transfer-progress-tracking.md
│       ├── 0401-clipboard-monitoring.md
│       ├── 0402-sync-strategy.md
│       ├── 0403-content-change-detection.md
│       ├── 0404-sync-conflict-handling.md
│       ├── 0501-device-info-structure.md
│       ├── 0502-device-storage-management.md
│       └── 0503-device-state-management.md
└── guides/                     # 开发指南
    ├── tdd-guide.md            # TDD实施指南
    └── xp-practices.md         # XP实践指南
```

## 开始开发

1. 阅读项目总览了解技术架构
2. 查看架构设计理解Clean Architecture
3. 学习开发指南掌握TDD和XP实践
4. 按照用户故事和任务进行开发