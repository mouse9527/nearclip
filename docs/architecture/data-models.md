# 数据模型

## Device 设备模型

**目的：** 表示 NearClip 网络中的设备信息，包括设备标识、能力和连接状态。

**关键属性：**
- deviceId: String - 设备唯一标识符
- deviceName: String - 用户友好的设备名称
- deviceType: DeviceType - 设备类型（Android/Mac）
- publicKey: String - 设备公钥，用于加密验证
- lastSeen: Timestamp - 最后在线时间
- connectionStatus: ConnectionStatus - 连接状态

### TypeScript 接口

```typescript
interface Device {
  deviceId: string;
  deviceName: string;
  deviceType: 'android' | 'mac';
  publicKey: string;
  lastSeen: number;
  connectionStatus: 'connected' | 'disconnected' | 'pairing' | 'error';
}

enum DeviceType {
  ANDROID = 'android',
  MAC = 'mac'
}

enum ConnectionStatus {
  CONNECTED = 'connected',
  DISCONNECTED = 'disconnected',
  PAIRING = 'pairing',
  ERROR = 'error'
}
```

### 关系关系

- Device 1:n SyncRecord - 设备可以有多个同步记录
- Device 1:n PairingRequest - 设备可以发起或接收多个配对请求

## SyncRecord 同步记录模型

**目的：** 记录粘贴板同步操作的详细信息，用于冲突解决和历史追踪。

**Key Attributes:**
- syncId: String - 同步操作唯一标识符
- sourceDeviceId: String - 源设备 ID
- content: String - 同步的文本内容
- contentType: ContentType - 内容类型（文本/链接）
- timestamp: Timestamp - 同步时间戳
- status: SyncStatus - 同步状态

### TypeScript 接口

```typescript
interface SyncRecord {
  syncId: string;
  sourceDeviceId: string;
  targetDeviceIds: string[];
  content: string;
  contentType: 'text' | 'url';
  timestamp: number;
  status: 'pending' | 'completed' | 'failed';
}

enum ContentType {
  TEXT = 'text',
  URL = 'url'
}

enum SyncStatus {
  PENDING = 'pending',
  COMPLETED = 'completed',
  FAILED = 'failed'
}
```

### 关系关系

- SyncRecord n:1 Device - 同步记录关联到源设备和目标设备

## PairingRequest 配对请求模型

**目的：** 管理设备配对过程中的请求和验证信息。

**Key Attributes:**
- requestId: String - 配对请求唯一标识符
- initiatorDeviceId: String - 发起配对的设备 ID
- targetDeviceId: String - 目标设备 ID
- pairingCode: String - 配对码或 QR 码内容
- timestamp: Timestamp - 请求时间戳
- status: PairingStatus - 配对状态

### TypeScript 接口

```typescript
interface PairingRequest {
  requestId: string;
  initiatorDeviceId: string;
  targetDeviceId: string;
  pairingCode: string;
  timestamp: number;
  status: 'pending' | 'accepted' | 'rejected' | 'expired';
}

enum PairingStatus {
  PENDING = 'pending',
  ACCEPTED = 'accepted',
  REJECTED = 'rejected',
  EXPIRED = 'expired'
}
```

### 关系关系

- PairingRequest 1:1 Device - 配对请求关联到发起设备和目标设备
