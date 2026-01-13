# Task 1.1 macOS 代码删除清单

## 目标
将 `macos/NearClip/Sources/NearClip/BleManager.swift` 从 **1153 行** 简化到 **~250 行**

---

## 删除项清单

### 1. 数据重组器类（80 行）✅ 优先级最高

**位置**: 第 1037-1079 行

**删除内容**:
```swift
class DataReassembler {
    private var chunks: [Int: Data] = [:]
    private var totalChunks: Int = 0
    private var messageId: UInt16 = 0
    private var lastActivityTime: Date = Date()
    private let timeout: TimeInterval = 30.0

    var isTimedOut: Bool {
        return Date().timeIntervalSince(lastActivityTime) > timeout
    }

    func reset() { ... }
    func addChunk(_ data: Data, sequence: Int, total: Int, messageId: UInt16) -> Data? { ... }
}
```

**原因**: Rust `nearclip-ble` crate 已有 `Reassembler` 实现

**影响**: 需要更新使用此类的代码

---

### 2. 数据分片器类（72 行）✅ 优先级最高

**位置**: 第 1083-1153 行

**删除内容**:
```swift
class DataChunker {
    private var messageIdCounter: UInt16 = 0

    func createChunks(from data: Data, maxPayloadSize: Int) -> [Data] { ... }
    static func parseChunk(_ data: Data) -> (...) { ... }
}
```

**原因**: Rust `nearclip-ble` crate 已有 `Chunker` 实现

**影响**: 需要更新使用此类的代码

---

### 3. 数据分片/重组常量（2 行）

**位置**: 第 1034-1035 行

**删除内容**:
```swift
// Chunk header size (Rust format): [messageId: 2 bytes][sequence: 2 bytes][total: 2 bytes][payloadLength: 2 bytes]
private let kChunkHeaderSize = 8
```

**原因**: 此常量只被 DataReassembler/DataChunker 使用

---

### 4. 连接限流属性（6 行）✅ 优先级高

**位置**: 第 93-98 行

**删除内容**:
```swift
// Track peripherals we're currently connecting to for discovery (read device_id then disconnect)
private var pendingDiscoveryConnections: Set<UUID> = []

// Throttle discovery connections - track last connection attempt time
private var lastDiscoveryAttempt: [UUID: Date] = [:]
private let discoveryThrottleInterval: TimeInterval = 30.0 // 30 seconds between discovery attempts for same device
private let maxConcurrentDiscovery = 2 // Max concurrent discovery connections
```

**原因**: 连接限流应由 Rust BleController 管理

**影响**: 需要删除所有引用这些属性的代码

---

### 5. 连接限流逻辑（多处）✅ 优先级高

**位置**: 在 `didDiscover` 回调中（第 655-678 行）

**删除代码段 #1** (第 655-656 行):
```swift
if pendingDiscoveryConnections.contains(peripheral.identifier) {
    return // 已经在连接队列中
}
```

**删除代码段 #2** (第 665-670 行):
```swift
if let lastAttempt = lastDiscoveryAttempt[peripheral.identifier],
   now.timeIntervalSince(lastAttempt) < discoveryThrottleInterval {
    // 在限流周期内，跳过
    return
}
```

**删除代码段 #3** (第 671-674 行):
```swift
if pendingDiscoveryConnections.count >= maxConcurrentDiscovery {
    // 已达到最大并发发现连接数
    return
}
```

**删除代码段 #4** (第 676-678 行):
```swift
NSLog("BLE: Auto-connecting to %@ to read device ID (pending: %d)", peripheralUuid, pendingDiscoveryConnections.count)
pendingDiscoveryConnections.insert(peripheral.identifier)
lastDiscoveryAttempt[peripheral.identifier] = now
```

---

### 6. 发现连接相关逻辑（多处）

**位置 1**: `didConnect` 回调中（第 684 行）
```swift
let isDiscoveryConnection = pendingDiscoveryConnections.contains(peripheral.identifier)
```
→ 删除此变量及其所有使用

**位置 2**: `didDisconnectPeripheral` 回调中（第 706, 714 行）
```swift
pendingDiscoveryConnections.remove(peripheral.identifier)  // 第 706 行
let wasDiscoveryConnection = pendingDiscoveryConnections.remove(peripheral.identifier) != nil  // 第 714 行
```
→ 删除这些清理代码

**位置 3**: `didDiscoverCharacteristics` 回调中（第 757 行）
```swift
if pendingDiscoveryConnections.contains(peripheral.identifier) {
    // 只读取 deviceId 特征
}
```
→ 删除此条件分支

**位置 4**: `didUpdateValueFor` 回调中（第 830 行）
```swift
let isDiscoveryConnection = pendingDiscoveryConnections.contains(peripheral.identifier)
```
→ 删除此变量及相关逻辑

---

### 7. 自动重组逻辑（需精确定位）

**需要查找的模式**:
- 使用 `DataReassembler` 实例的代码
- 调用 `addChunk()` 方法的代码
- 存储 `reassemblers` 字典的代码

**预计删除**: ~30-50 行

---

### 8. 自动分片逻辑（需精确定位）

**需要查找的模式**:
- 使用 `DataChunker` 实例的代码
- 调用 `createChunks()` 方法的代码
- 分片发送循环代码

**预计删除**: ~30-50 行

---

### 9. 其他业务逻辑（待确定）

**可能需要删除的**:
- [ ] 自动重连逻辑（如果有）
- [ ] 健康监控/心跳逻辑（如果有）
- [ ] 设备管理逻辑（应该在 Rust 层）

---

## 总计预估

| 项目 | 行数 |
|------|------|
| DataReassembler 类 | 80 |
| DataChunker 类 | 72 |
| 分片/重组常量 | 2 |
| 连接限流属性 | 6 |
| 连接限流逻辑 | ~40 |
| 发现连接逻辑 | ~30 |
| 自动重组调用 | ~40 |
| 自动分片调用 | ~40 |
| 其他业务逻辑 | ~50 |
| **总计** | **~360 行** |

**剩余代码**: 1153 - 360 = **~793 行**

**进一步简化目标**: 简化代理方法和清理注释，目标 **~250 行**

---

## 实施策略

### Phase 1: 删除独立的类（简单）
1. 删除 `DataReassembler` 类
2. 删除 `DataChunker` 类
3. 删除相关常量

### Phase 2: 删除连接限流（中等）
4. 删除连接限流属性
5. 删除 `didDiscover` 中的限流逻辑
6. 清理其他回调中的 `pendingDiscoveryConnections` 引用

### Phase 3: 删除数据处理调用（复杂）
7. 查找并删除 `DataReassembler` 使用
8. 查找并删除 `DataChunker` 使用
9. 更新数据接收/发送方法为原始字节传递

### Phase 4: 简化和清理（收尾）
10. 简化代理方法（只保留必要的转发）
11. 删除冗余注释和调试日志
12. 格式化代码

---

## 验证步骤

### 编译验证
```bash
cd macos/NearClip
xcodebuild -project NearClip.xcodeproj -scheme NearClip -configuration Debug clean build
```

### 行数验证
```bash
wc -l macos/NearClip/Sources/NearClip/BleManager.swift
# 应该 < 300 行
```

### 功能验证
- [ ] BLE 扫描正常
- [ ] BLE 连接正常
- [ ] 数据接收回调正常（原始字节）
- [ ] 数据发送方法正常（原始字节）
- [ ] Peripheral 模式正常

---

## 下一步

1. ✅ 创建此删除清单
2. ⏳ 查找所有 `DataReassembler` 和 `DataChunker` 的使用位置
3. ⏳ 备份原始文件
4. ⏳ 开始删除（从 Phase 1 开始）
5. ⏳ 逐步测试每个 Phase
6. ⏳ 最终验证
