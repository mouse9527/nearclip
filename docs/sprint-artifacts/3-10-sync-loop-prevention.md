# Story 3.10: 实现同步循环防护

Status: done

## Story

As a 用户,
I want 系统防止剪贴板内容回传,
So that 不会无限循环同步.

## Acceptance Criteria

1. **Given** 收到远程剪贴板内容 **When** 写入本地剪贴板 **Then** 标记内容来源为远程
2. **And** 本地剪贴板变化检测忽略该内容
3. **And** 不会重新发送到源设备
4. **And** 测试验证防循环有效

## Tasks / Subtasks

- [x] Task 1: 定义内容来源枚举 (AC: 1)
  - [x] 1.1 创建 `crates/nearclip-sync/src/loop_guard.rs`
  - [x] 1.2 定义 `ContentOrigin` 枚举 (Local, Remote)
  - [x] 1.3 定义 `ContentSource` 结构体 (origin, device_id, timestamp)

- [x] Task 2: 实现内容指纹 (AC: 2, 3)
  - [x] 2.1 创建 `ContentFingerprint` 结构体
  - [x] 2.2 实现 `from_content(&[u8])` 使用哈希计算指纹
  - [x] 2.3 实现 `PartialEq` 和 `Hash` trait

- [x] Task 3: 实现循环防护配置 (AC: 1, 2)
  - [x] 3.1 创建 `LoopGuardConfig` 结构体
  - [x] 3.2 实现 `with_history_size()` 配置历史记录大小
  - [x] 3.3 实现 `with_expiry_duration()` 配置过期时间
  - [x] 3.4 实现 `validate()` 方法

- [x] Task 4: 实现循环防护核心 (AC: 1, 2, 3)
  - [x] 4.1 创建 `LoopGuard` 结构体
  - [x] 4.2 实现 `record_local(&self, content: &[u8])` 记录本地内容
  - [x] 4.3 实现 `record_remote(&self, content: &[u8], device_id: &str)` 记录远程内容
  - [x] 4.4 实现 `should_sync(&self, content: &[u8]) -> bool` 判断是否应该同步
  - [x] 4.5 实现 `is_from_remote(&self, content: &[u8]) -> bool` 判断是否来自远程

- [x] Task 5: 实现历史记录管理 (AC: 2)
  - [x] 5.1 实现 LRU 缓存存储历史记录
  - [x] 5.2 实现自动过期清理
  - [x] 5.3 实现 `clear()` 清空历史

- [x] Task 6: 导出模块 (AC: 1)
  - [x] 6.1 在 `lib.rs` 添加 `pub mod loop_guard;`
  - [x] 6.2 添加 re-exports

- [x] Task 7: 编写单元测试 (AC: 1, 2, 3, 4)
  - [x] 7.1 测试 `ContentFingerprint` 计算
  - [x] 7.2 测试 `LoopGuard` 创建
  - [x] 7.3 测试本地内容记录
  - [x] 7.4 测试远程内容记录
  - [x] 7.5 测试 `should_sync` 逻辑
  - [x] 7.6 测试历史过期

- [x] Task 8: 编写集成测试 (AC: 1, 2, 3, 4)
  - [x] 8.1 创建 `tests/loop_guard_integration.rs`
  - [x] 8.2 测试：本地内容应该同步
  - [x] 8.3 测试：远程内容不应该回传
  - [x] 8.4 测试：完整防循环流程

- [x] Task 9: 验证构建 (AC: 1, 2, 3, 4)
  - [x] 9.1 运行 `cargo build -p nearclip-sync` 确保无错误
  - [x] 9.2 运行 `cargo test -p nearclip-sync` 确保测试通过
  - [x] 9.3 运行 `cargo clippy -p nearclip-sync` 确保无警告

## Dev Notes

### 架构约束 (来自 architecture.md 和 project_context.md)

1. **内容指纹**: 使用 SHA256 前 16 字节作为指纹
2. **历史大小**: 默认保存最近 100 条记录
3. **过期时间**: 默认 60 秒后过期
4. **线程安全**: 使用 RwLock 保护内部状态

### 与其他 Story 的关系

- Story 3-5: 剪贴板发送 (发送前检查 should_sync)
- Story 3-6: 剪贴板接收 (接收后记录 record_remote)
- Story 3-11: 核心协调层 (整合防循环逻辑)

### 设计决策

1. **基于指纹**: 使用内容哈希而非内容本身，节省内存
2. **双重检查**: 同时检查指纹和时间戳
3. **LRU 缓存**: 自动淘汰旧记录
4. **设备追踪**: 记录内容来源设备，支持多设备场景

### 数据结构

```rust
// 内容来源
enum ContentOrigin {
    Local,              // 本地产生
    Remote(String),     // 远程设备 (device_id)
}

// 内容指纹
struct ContentFingerprint {
    hash: [u8; 16],     // SHA256 前 16 字节
}

// 历史记录条目
struct HistoryEntry {
    fingerprint: ContentFingerprint,
    origin: ContentOrigin,
    timestamp: u64,
}

// 循环防护配置
struct LoopGuardConfig {
    history_size: usize,    // 历史大小 (默认 100)
    expiry_secs: u64,       // 过期时间 (默认 60s)
}

// 循环防护
struct LoopGuard {
    config: LoopGuardConfig,
    history: RwLock<LruCache<ContentFingerprint, HistoryEntry>>,
}
```

### 防循环流程

```
[本地剪贴板变化]
       |
       v
[计算内容指纹]
       |
       v
[检查 should_sync()]
       |
   +---+---+
   |       |
  Yes      No (来自远程)
   |       |
   v       v
[发送]  [忽略]


[收到远程内容]
       |
       v
[record_remote()]
       |
       v
[写入本地剪贴板]
       |
       v
[本地变化检测触发]
       |
       v
[should_sync() 返回 false]
       |
       v
[不发送 - 循环被阻止]
```

## Checklist

- [x] All tasks completed
- [x] Unit tests passing (27 loop_guard unit tests)
- [x] Integration tests passing (54 integration tests)
- [x] Clippy warnings resolved
- [x] Story file updated to 'done'
