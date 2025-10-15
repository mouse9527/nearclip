# 数据库架构

## Android Room 数据库架构

```sql
-- 设备表
CREATE TABLE devices (
    device_id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    device_type TEXT NOT NULL CHECK (device_type IN ('android', 'mac')),
    public_key TEXT NOT NULL,
    last_seen INTEGER NOT NULL,
    connection_status TEXT NOT NULL DEFAULT 'disconnected',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- 同步记录表
CREATE TABLE sync_records (
    sync_id TEXT PRIMARY KEY,
    source_device_id TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL CHECK (content_type IN ('text', 'url')),
    timestamp INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    FOREIGN KEY (source_device_id) REFERENCES devices(device_id)
);

-- 设备同步关联表
CREATE TABLE device_sync_targets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sync_id TEXT NOT NULL,
    target_device_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    FOREIGN KEY (sync_id) REFERENCES sync_records(sync_id),
    FOREIGN KEY (target_device_id) REFERENCES devices(device_id),
    UNIQUE(sync_id, target_device_id)
);

-- 配对请求表
CREATE TABLE pairing_requests (
    request_id TEXT PRIMARY KEY,
    initiator_device_id TEXT NOT NULL,
    target_device_id TEXT NOT NULL,
    pairing_code TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    expires_at INTEGER NOT NULL,
    FOREIGN KEY (initiator_device_id) REFERENCES devices(device_id),
    FOREIGN KEY (target_device_id) REFERENCES devices(device_id)
);

-- 索引
CREATE INDEX idx_devices_last_seen ON devices(last_seen);
CREATE INDEX idx_sync_records_timestamp ON sync_records(timestamp);
CREATE INDEX idx_pairing_requests_timestamp ON pairing_requests(timestamp);
CREATE INDEX idx_pairing_requests_status ON pairing_requests(status);
```

## macOS Core Data 模型

```swift
// 设备实体
@objc(Device)
public class Device: NSManagedObject {
    @NSManaged public var deviceId: String
    @NSManaged public var deviceName: String
    @NSManaged public var deviceType: String
    @NSManaged public var publicKey: String
    @NSManaged public var lastSeen: Date
    @NSManaged public var connectionStatus: String
    @NSManaged public var createdAt: Date
    @NSManaged public var updatedAt: Date
    @NSManaged public var syncRecords: NSSet?
    @NSManaged public var pairingRequests: NSSet?
}

// 同步记录实体
@objc(SyncRecord)
public class SyncRecord: NSManagedObject {
    @NSManaged public var syncId: String
    @NSManaged public var sourceDevice: Device
    @NSManaged public var content: String
    @NSManaged public var contentType: String
    @NSManaged public var timestamp: Date
    @NSManaged public var status: String
    @NSManaged public var targetDevices: NSSet?
}

// 配对请求实体
@objc(PairingRequest)
public class PairingRequest: NSManagedObject {
    @NSManaged public var requestId: String
    @NSManaged public var initiatorDevice: Device
    @NSManaged public var targetDevice: Device
    @NSManaged public var pairingCode: String
    @NSManaged public var timestamp: Date
    @NSManaged public var status: String
    @NSManaged public var expiresAt: Date
}
```
