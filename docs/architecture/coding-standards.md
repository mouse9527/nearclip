# 编码标准

## 关键的全栈规则

- **类型安全：** 在 Android 和 Mac 平台都使用强类型系统
- **消息验证：** 所有接收到的消息必须验证格式、签名和时间戳
- **错误处理：** 所有网络操作必须包含适当的错误处理和重试逻辑
- **资源管理：** BLE 连接和数据库连接必须正确管理生命周期
- **日志记录：** 使用统一的日志格式，不记录敏感信息
- **状态管理：** UI 状态更新必须通过状态管理器进行
- **加密安全：** 所有设备间通信必须端到端加密

## 命名约定

| 元素 | 前端 | 后端 | 示例 |
|------|------|------|------|
| 组件 | PascalCase | - | `DeviceCard.kt` |
| 服务 | PascalCase | PascalCase | `BluetoothService.kt` |
| 函数 | camelCase | camelCase | `startDeviceDiscovery()` |
| 数据类 | PascalCase | PascalCase | `SyncRecord.kt` |
| 常量 | UPPER_SNAKE_CASE | UPPER_SNAKE_CASE | `MAX_RETRY_COUNT` |
