# 开发计划

NearClip跨设备剪贴板同步工具的开发计划和任务管理。

## 计划概览

本项目采用严格的TDD（测试驱动开发）、Clean Architecture（清洁架构）和XP（极限编程）最佳实践进行开发。

## 快速导航

- [项目总览](../README.md) - 项目概述和技术架构
- [架构设计](../architecture/) - Clean Architecture和技术架构
- [开发指南](../guides/) - TDD、XP实践和编码标准
- [用户故事](stories/) - INVEST用户故事
- [具体任务](../tasks/) - 正交任务分解

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

本项目采用超细粒度的任务分解，每个任务设计为5-10分钟内可完成，确保开发过程的快速反馈和进度跟踪。任务按照功能模块和平台进行分解。

### 核心基础模块 (0100-0199)
- [Task 0101: 定义设备类型枚举](../tasks/0101-device-enum-definition.md) - 基础设备类型定义
- [Task 0102: 定义设备状态枚举](../tasks/0102-device-status-enum.md) - 设备连接状态管理
- [Task 0103: 定义设备能力枚举](../tasks/0103-device-capability-enum.md) - 设备能力标识

### 加密安全模块 (0200-0299)  
- [Task 0201: 定义加密算法枚举](../tasks/0201-encryption-algorithm-enum.md) - 支持的加密算法
- [Task 0202: 定义加密错误类型](../tasks/0202-encryption-error-types.md) - 加密相关错误处理
- [Task 0203: 实现加密密钥基础结构](../tasks/0203-encryption-key-basics.md) - 密钥基础结构

### 文本传输模块 (0300-0399)
- [Task 0301: 定义传输方向枚举](../tasks/0301-transfer-direction-enum.md) - 传输方向标识
- [Task 0302: 定义传输状态枚举](../tasks/0302-transfer-status-enum.md) - 传输过程状态
- [Task 0303: 定义传输优先级枚举](../tasks/0303-transfer-priority-enum.md) - 传输优先级管理

### 剪贴板同步模块 (0400-0499)
- [Task 0401: 定义剪贴板内容类型](../tasks/0401-clipboard-content-types.md) - 支持的内容类型
- [Task 0402: 实现内容变化检测算法](../tasks/0402-content-detection-algorithm.md) - 智能变化检测
- [Task 0403: 定义同步策略枚举](../tasks/0403-sync-strategy-enum.md) - 同步决策策略

### 设备管理模块 (0500-0599)
- [Task 0501: 定义设备元数据结构](../tasks/0501-device-metadata-struct.md) - 设备元数据管理
- [Task 0502: 实现设备存储接口](../tasks/0502-device-storage-interface.md) - 持久化存储接口
- [Task 0503: 实现设备状态跟踪](../tasks/0503-device-state-tracker.md) - 状态变更跟踪

### Android平台实现 (0600-0699)
- [Task 0601: 实现Android剪贴板权限检查](../tasks/0601-android-clipboard-permission.md) - 权限管理
- [Task 0602: 实现Android剪贴板监听器](../tasks/0602-android-clipboard-listener.md) - 内容监听
- [Task 0603: 实现Android权限请求对话框](../tasks/0603-android-permission-dialog.md) - 用户交互

### iOS平台实现 (0700-0799)
- [Task 0701: 实现iOS剪贴板访问权限](../tasks/0701-ios-clipboard-access.md) - 权限处理
- [Task 0702: 实现iOS剪贴板变化监听](../tasks/0702-ios-clipboard-monitor.md) - 变化监听
- [Task 0703: 实现iOS剪贴板内容过滤](../tasks/0703-ios-content-filter.md) - 内容过滤

### 桌面平台实现 (0800-0899)
- [Task 0801: 实现桌面端剪贴板监控基础](../tasks/0801-desktop-clipboard-monitor.md) - 跨平台基础
- [Task 0802: 实现Windows剪贴板监控](../tasks/0802-windows-clipboard-monitor.md) - Windows特定
- [Task 0803: 实现macOS剪贴板监控](../tasks/0803-macos-clipboard-monitor.md) - macOS特定
- [Task 0804: 实现Linux剪贴板监控](../tasks/0804-linux-clipboard-monitor.md) - Linux特定

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
│   └── ../tasks/                  # TDD开发任务
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