//! mDNS 设备发现模块
//!
//! 提供在局域网上发现其他 NearClip 设备的功能。

use crate::error::NetError;
use crate::mdns::advertise::{SERVICE_TYPE, TXT_DEVICE_ID, TXT_PUBKEY_HASH};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{broadcast, RwLock};
use tokio::task::JoinHandle;
use tracing::{debug, info, instrument, warn};

/// 发现的设备信息
///
/// 包含从 mDNS 服务发现中获取的设备详细信息。
///
/// # Example
///
/// ```
/// use nearclip_net::DiscoveredDevice;
/// use std::collections::HashSet;
/// use std::net::IpAddr;
/// use std::time::Instant;
///
/// let device = DiscoveredDevice {
///     device_id: "device-001".to_string(),
///     public_key_hash: "dGVzdC1oYXNo".to_string(),
///     addresses: HashSet::new(),
///     port: 12345,
///     fullname: "device-001._nearclip._tcp.local.".to_string(),
///     discovered_at: Instant::now(),
///     last_seen: Instant::now(),
/// };
///
/// assert_eq!(device.device_id, "device-001");
/// ```
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// 设备 ID
    pub device_id: String,
    /// 公钥哈希（Base64）
    pub public_key_hash: String,
    /// 设备 IP 地址列表
    pub addresses: HashSet<IpAddr>,
    /// 服务端口
    pub port: u16,
    /// 完整服务名
    pub fullname: String,
    /// 发现时间
    pub discovered_at: Instant,
    /// 最后更新时间
    pub last_seen: Instant,
}

impl PartialEq for DiscoveredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.device_id == other.device_id
            && self.public_key_hash == other.public_key_hash
            && self.addresses == other.addresses
            && self.port == other.port
            && self.fullname == other.fullname
    }
}

impl Eq for DiscoveredDevice {}

impl std::hash::Hash for DiscoveredDevice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.device_id.hash(state);
        self.fullname.hash(state);
    }
}

impl DiscoveredDevice {
    /// 从 mdns-sd ServiceInfo 创建 DiscoveredDevice
    ///
    /// # Arguments
    ///
    /// * `info` - mDNS 服务信息
    ///
    /// # Returns
    ///
    /// 成功返回 `DiscoveredDevice`，如果缺少必要的 TXT 记录或 device_id 为空则返回 None
    pub fn from_service_info(info: &mdns_sd::ServiceInfo) -> Option<Self> {
        let device_id = info.get_property_val_str(TXT_DEVICE_ID)?.to_string();

        // M4 Fix: 验证 device_id 不为空
        if device_id.is_empty() {
            return None;
        }

        let public_key_hash = info
            .get_property_val_str(TXT_PUBKEY_HASH)
            .unwrap_or("")
            .to_string();

        let addresses: HashSet<IpAddr> = info.get_addresses().iter().copied().collect();
        let now = Instant::now();

        Some(Self {
            device_id,
            public_key_hash,
            addresses,
            port: info.get_port(),
            fullname: info.get_fullname().to_string(),
            discovered_at: now,
            last_seen: now,
        })
    }

    /// 更新设备信息
    ///
    /// 保留原始发现时间，更新其他字段
    pub fn update_from(&mut self, other: &DiscoveredDevice) {
        self.public_key_hash = other.public_key_hash.clone();
        self.addresses = other.addresses.clone();
        self.port = other.port;
        // L1 Fix: 同步更新 fullname
        self.fullname = other.fullname.clone();
        self.last_seen = Instant::now();
    }
}

/// 发现事件
///
/// 表示设备发现过程中的各种事件。
#[derive(Debug, Clone)]
pub enum DiscoveryEvent {
    /// 发现新设备
    DeviceFound(DiscoveredDevice),
    /// 设备信息更新（如 IP 变化）
    DeviceUpdated(DiscoveredDevice),
    /// 设备离线
    DeviceLost {
        /// 设备 ID
        device_id: String,
        /// 完整服务名
        fullname: String,
    },
}

/// 事件广播通道容量
const EVENT_CHANNEL_CAPACITY: usize = 64;

/// mDNS 设备发现器
///
/// 在局域网上发现其他 NearClip 设备。
///
/// # Example
///
/// ```no_run
/// use nearclip_net::{MdnsDiscovery, DiscoveryEvent};
///
/// # async fn example() -> Result<(), nearclip_net::NetError> {
/// let mut discovery = MdnsDiscovery::new()?;
/// let mut events = discovery.subscribe();
///
/// discovery.start().await?;
///
/// // 处理发现事件
/// while let Ok(event) = events.recv().await {
///     match event {
///         DiscoveryEvent::DeviceFound(device) => {
///             println!("Found device: {}", device.device_id);
///         }
///         DiscoveryEvent::DeviceLost { device_id, .. } => {
///             println!("Lost device: {}", device_id);
///         }
///         _ => {}
///     }
/// }
///
/// discovery.stop().await?;
/// # Ok(())
/// # }
/// ```
pub struct MdnsDiscovery {
    /// mDNS 守护进程
    daemon: Arc<ServiceDaemon>,
    /// 已发现设备列表 (device_id -> DiscoveredDevice)
    devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
    /// M3 Fix: fullname 到 device_id 的反向查找表
    fullname_to_id: Arc<RwLock<HashMap<String, String>>>,
    /// 事件广播通道发送端
    event_tx: broadcast::Sender<DiscoveryEvent>,
    /// 浏览任务句柄
    browse_handle: Option<JoinHandle<()>>,
    /// 是否正在浏览
    is_browsing: bool,
}

impl MdnsDiscovery {
    /// 创建新的发现器
    ///
    /// # Returns
    ///
    /// 成功返回 `MdnsDiscovery`，失败返回 `NetError`
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_net::MdnsDiscovery;
    ///
    /// let discovery = MdnsDiscovery::new();
    /// assert!(discovery.is_ok());
    /// ```
    #[instrument]
    pub fn new() -> Result<Self, NetError> {
        let daemon = ServiceDaemon::new()
            .map_err(|e| NetError::Mdns(format!("Failed to create mDNS daemon: {}", e)))?;

        let (event_tx, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);

        debug!("Created mDNS discovery");

        Ok(Self {
            daemon: Arc::new(daemon),
            devices: Arc::new(RwLock::new(HashMap::new())),
            fullname_to_id: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            browse_handle: None,
            is_browsing: false,
        })
    }

    /// 开始发现设备
    ///
    /// 开始在局域网上搜索 NearClip 服务。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `NetError`
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<(), NetError> {
        if self.is_browsing {
            warn!("Discovery already started");
            return Ok(());
        }

        // 开始浏览服务
        let receiver = self
            .daemon
            .browse(SERVICE_TYPE)
            .map_err(|e| NetError::Mdns(format!("Failed to start browsing: {}", e)))?;

        let devices = Arc::clone(&self.devices);
        let fullname_to_id = Arc::clone(&self.fullname_to_id);
        let event_tx = self.event_tx.clone();

        // 启动异步任务处理事件
        let handle = tokio::spawn(async move {
            loop {
                // 使用 spawn_blocking 因为 recv() 是阻塞的
                let recv_clone = receiver.clone();
                let event_result =
                    tokio::task::spawn_blocking(move || recv_clone.recv()).await;

                match event_result {
                    Ok(Ok(service_event)) => {
                        match service_event {
                            ServiceEvent::ServiceResolved(info) => {
                                if let Some(device) = DiscoveredDevice::from_service_info(&info)
                                {
                                    let mut devices_guard = devices.write().await;
                                    let device_id = device.device_id.clone();

                                    if let Some(existing) = devices_guard.get_mut(&device_id) {
                                        // 设备已存在，更新信息
                                        existing.update_from(&device);
                                        let updated = existing.clone();
                                        drop(devices_guard);

                                        debug!(
                                            device_id = %device_id,
                                            addresses = ?updated.addresses,
                                            "Device updated"
                                        );

                                        // 发送更新事件（忽略发送失败，可能没有订阅者）
                                        let _ = event_tx.send(DiscoveryEvent::DeviceUpdated(updated));
                                    } else {
                                        // 新设备
                                        let fullname = device.fullname.clone();
                                        devices_guard.insert(device_id.clone(), device.clone());
                                        drop(devices_guard);

                                        // M3 Fix: 更新反向查找表
                                        let mut fullname_guard = fullname_to_id.write().await;
                                        fullname_guard.insert(fullname, device_id.clone());
                                        drop(fullname_guard);

                                        info!(
                                            device_id = %device_id,
                                            port = device.port,
                                            addresses = ?device.addresses,
                                            "Device discovered"
                                        );

                                        let _ = event_tx.send(DiscoveryEvent::DeviceFound(device));
                                    }
                                }
                            }
                            ServiceEvent::ServiceRemoved(_service_type, fullname) => {
                                // M3 Fix: 使用反向查找表 O(1) 查找
                                let mut fullname_guard = fullname_to_id.write().await;
                                let removed_device_id = fullname_guard.remove(&fullname);
                                drop(fullname_guard);

                                if let Some(device_id) = removed_device_id {
                                    let mut devices_guard = devices.write().await;
                                    devices_guard.remove(&device_id);
                                    drop(devices_guard);

                                    info!(
                                        device_id = %device_id,
                                        fullname = %fullname,
                                        "Device lost"
                                    );

                                    let _ = event_tx.send(DiscoveryEvent::DeviceLost {
                                        device_id,
                                        fullname,
                                    });
                                }
                            }
                            ServiceEvent::SearchStarted(service_type) => {
                                debug!(service_type = %service_type, "Search started");
                            }
                            ServiceEvent::SearchStopped(service_type) => {
                                debug!(service_type = %service_type, "Search stopped");
                                break;
                            }
                            ServiceEvent::ServiceFound(_service_type, _fullname) => {
                                // 服务发现的初始通知，等待 ServiceResolved 获取完整信息
                                debug!("Service found, waiting for resolution");
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        warn!(error = %e, "Error receiving mDNS event");
                        break;
                    }
                    Err(e) => {
                        warn!(error = %e, "Browse task error");
                        break;
                    }
                }
            }
        });

        self.browse_handle = Some(handle);
        self.is_browsing = true;

        info!(service_type = %SERVICE_TYPE, "mDNS discovery started");

        Ok(())
    }

    /// 停止发现设备
    ///
    /// 停止搜索 NearClip 服务。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `NetError`
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<(), NetError> {
        if !self.is_browsing {
            debug!("Discovery not running");
            return Ok(());
        }

        // 停止浏览
        self.daemon
            .stop_browse(SERVICE_TYPE)
            .map_err(|e| NetError::Mdns(format!("Failed to stop browsing: {}", e)))?;

        // 等待任务结束
        if let Some(handle) = self.browse_handle.take() {
            // 给任务一点时间来处理 SearchStopped 事件
            let _ = tokio::time::timeout(std::time::Duration::from_secs(1), handle).await;
        }

        self.is_browsing = false;

        info!("mDNS discovery stopped");

        Ok(())
    }

    /// 订阅发现事件
    ///
    /// 返回一个接收器，可以用来接收设备发现、更新和离线事件。
    ///
    /// # Returns
    ///
    /// 事件接收器
    pub fn subscribe(&self) -> broadcast::Receiver<DiscoveryEvent> {
        self.event_tx.subscribe()
    }

    /// 获取当前设备列表
    ///
    /// 返回所有已发现设备的快照。
    ///
    /// # Returns
    ///
    /// 设备列表
    pub async fn get_devices(&self) -> Vec<DiscoveredDevice> {
        let devices = self.devices.read().await;
        devices.values().cloned().collect()
    }

    /// 获取特定设备
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备 ID
    ///
    /// # Returns
    ///
    /// 如果找到设备返回 `Some(DiscoveredDevice)`，否则返回 `None`
    pub async fn get_device(&self, device_id: &str) -> Option<DiscoveredDevice> {
        let devices = self.devices.read().await;
        devices.get(device_id).cloned()
    }

    /// M1 Fix: 清除所有已发现的设备
    ///
    /// 用于在重新开始发现前清理陈旧数据
    pub async fn clear_devices(&self) {
        let mut devices = self.devices.write().await;
        devices.clear();
        drop(devices);

        let mut fullname_to_id = self.fullname_to_id.write().await;
        fullname_to_id.clear();
    }

    /// 是否正在发现
    ///
    /// # Returns
    ///
    /// 如果正在搜索设备返回 `true`
    pub fn is_browsing(&self) -> bool {
        self.is_browsing
    }
}

impl Drop for MdnsDiscovery {
    fn drop(&mut self) {
        if self.is_browsing {
            // 尝试停止浏览
            if let Err(e) = self.daemon.stop_browse(SERVICE_TYPE) {
                warn!(error = %e, "Failed to stop mDNS browsing during drop");
            } else {
                debug!("mDNS browsing stopped during drop");
            }
        }

        // M2 Fix: 确保取消后台任务
        if let Some(handle) = self.browse_handle.take() {
            handle.abort();
            debug!("Browse task aborted during drop");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovered_device_creation() {
        let device = DiscoveredDevice {
            device_id: "test-device".to_string(),
            public_key_hash: "dGVzdC1oYXNo".to_string(),
            addresses: HashSet::new(),
            port: 12345,
            fullname: "test-device._nearclip._tcp.local.".to_string(),
            discovered_at: Instant::now(),
            last_seen: Instant::now(),
        };

        assert_eq!(device.device_id, "test-device");
        assert_eq!(device.public_key_hash, "dGVzdC1oYXNo");
        assert_eq!(device.port, 12345);
    }

    #[test]
    fn test_discovered_device_equality() {
        let now = Instant::now();
        let device1 = DiscoveredDevice {
            device_id: "device-1".to_string(),
            public_key_hash: "hash".to_string(),
            addresses: HashSet::new(),
            port: 8080,
            fullname: "device-1._nearclip._tcp.local.".to_string(),
            discovered_at: now,
            last_seen: now,
        };

        let device2 = DiscoveredDevice {
            device_id: "device-1".to_string(),
            public_key_hash: "hash".to_string(),
            addresses: HashSet::new(),
            port: 8080,
            fullname: "device-1._nearclip._tcp.local.".to_string(),
            discovered_at: Instant::now(), // 不同时间
            last_seen: Instant::now(),
        };

        // 时间戳不影响相等性
        assert_eq!(device1, device2);
    }

    #[test]
    fn test_discovered_device_hash() {
        use std::collections::HashSet as DeviceSet;

        let device1 = DiscoveredDevice {
            device_id: "device-1".to_string(),
            public_key_hash: "hash".to_string(),
            addresses: HashSet::new(),
            port: 8080,
            fullname: "device-1._nearclip._tcp.local.".to_string(),
            discovered_at: Instant::now(),
            last_seen: Instant::now(),
        };

        let device2 = device1.clone();

        let mut set: DeviceSet<DiscoveredDevice> = DeviceSet::new();
        set.insert(device1);
        set.insert(device2);

        // 相同设备应该只保留一个
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_discovery_event_device_found() {
        let device = DiscoveredDevice {
            device_id: "test".to_string(),
            public_key_hash: "hash".to_string(),
            addresses: HashSet::new(),
            port: 8080,
            fullname: "test._nearclip._tcp.local.".to_string(),
            discovered_at: Instant::now(),
            last_seen: Instant::now(),
        };

        let event = DiscoveryEvent::DeviceFound(device.clone());

        if let DiscoveryEvent::DeviceFound(d) = event {
            assert_eq!(d.device_id, "test");
        } else {
            panic!("Expected DeviceFound event");
        }
    }

    #[test]
    fn test_discovery_event_device_lost() {
        let event = DiscoveryEvent::DeviceLost {
            device_id: "test".to_string(),
            fullname: "test._nearclip._tcp.local.".to_string(),
        };

        if let DiscoveryEvent::DeviceLost { device_id, fullname } = event {
            assert_eq!(device_id, "test");
            assert_eq!(fullname, "test._nearclip._tcp.local.");
        } else {
            panic!("Expected DeviceLost event");
        }
    }

    #[test]
    fn test_discovery_event_device_updated() {
        let device = DiscoveredDevice {
            device_id: "test".to_string(),
            public_key_hash: "hash".to_string(),
            addresses: HashSet::new(),
            port: 8080,
            fullname: "test._nearclip._tcp.local.".to_string(),
            discovered_at: Instant::now(),
            last_seen: Instant::now(),
        };

        let event = DiscoveryEvent::DeviceUpdated(device);

        if let DiscoveryEvent::DeviceUpdated(d) = event {
            assert_eq!(d.device_id, "test");
        } else {
            panic!("Expected DeviceUpdated event");
        }
    }

    #[test]
    fn test_mdns_discovery_new_success() {
        let discovery = MdnsDiscovery::new();
        assert!(discovery.is_ok());

        let discovery = discovery.unwrap();
        assert!(!discovery.is_browsing());
    }

    #[tokio::test]
    async fn test_mdns_discovery_get_devices_empty() {
        let discovery = MdnsDiscovery::new().unwrap();
        let devices = discovery.get_devices().await;
        assert!(devices.is_empty());
    }

    #[tokio::test]
    async fn test_mdns_discovery_get_device_not_found() {
        let discovery = MdnsDiscovery::new().unwrap();
        let device = discovery.get_device("nonexistent").await;
        assert!(device.is_none());
    }

    #[test]
    fn test_mdns_discovery_subscribe() {
        let discovery = MdnsDiscovery::new().unwrap();
        let _receiver = discovery.subscribe();
        // 订阅成功，不会 panic
    }
}
