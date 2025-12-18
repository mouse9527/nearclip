//! NearClip 核心管理器模块
//!
//! 提供统一的 API 管理所有同步功能，协调底层模块。
//!
//! # 示例
//!
//! ```
//! use nearclip_core::{
//!     NearClipManager, NearClipConfig, NearClipCallback,
//!     DeviceInfo, NearClipError,
//! };
//! use std::sync::Arc;
//!
//! struct MyCallback;
//!
//! impl NearClipCallback for MyCallback {
//!     fn on_device_connected(&self, device: &DeviceInfo) {
//!         println!("Device connected: {}", device.name());
//!     }
//!     fn on_device_disconnected(&self, device_id: &str) {
//!         println!("Device disconnected: {}", device_id);
//!     }
//!     fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
//!         println!("Received {} bytes from {}", content.len(), from_device);
//!     }
//!     fn on_sync_error(&self, error: &NearClipError) {
//!         eprintln!("Sync error: {}", error);
//!     }
//! }
//!
//! let config = NearClipConfig::new("My Device");
//! let callback = Arc::new(MyCallback);
//! let manager = NearClipManager::new(config, callback).unwrap();
//!
//! assert!(!manager.is_running());
//! ```

use crate::config::NearClipConfig;
use crate::device::{DeviceInfo, DevicePlatform, DeviceStatus};
use crate::error::{NearClipError, Result};
use nearclip_crypto::{TlsCertificate, TlsClientConfig, TlsServerConfig};
use nearclip_net::{
    DiscoveredDevice, MdnsAdvertiser, MdnsDiscovery, MdnsServiceConfig,
    TcpClient, TcpClientConfig, TcpReadHalf, TcpServer, TcpServerConfig, TcpWriteHalf,
};
use nearclip_sync::{Channel, Message, MessageType, PairingPayload, ProtocolPlatform};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex as TokioMutex;
use tokio::task::JoinHandle;

// ============================================================
// 平台类型转换辅助函数
// ============================================================

/// 将 DevicePlatform 转换为 ProtocolPlatform
fn device_platform_to_protocol(platform: DevicePlatform) -> ProtocolPlatform {
    match platform {
        DevicePlatform::MacOS => ProtocolPlatform::MacOS,
        DevicePlatform::Android => ProtocolPlatform::Android,
        DevicePlatform::Unknown => ProtocolPlatform::Unknown,
    }
}

/// 将 ProtocolPlatform 转换为 DevicePlatform
fn protocol_platform_to_device(platform: ProtocolPlatform) -> DevicePlatform {
    match platform {
        ProtocolPlatform::MacOS => DevicePlatform::MacOS,
        ProtocolPlatform::Android => DevicePlatform::Android,
        ProtocolPlatform::Unknown => DevicePlatform::Unknown,
    }
}

// ============================================================
// 消息帧协议辅助函数
// ============================================================

/// 最大消息大小 (16 MB)
const MAX_MESSAGE_SIZE: u32 = 16 * 1024 * 1024;

/// 通过分离的写半连接发送带长度前缀的消息
///
/// 消息格式：[4字节长度(大端序)] + [消息数据]
async fn send_framed_message_split(writer: &mut TcpWriteHalf, msg: &Message) -> Result<()> {
    let data = msg.serialize()
        .map_err(|e| NearClipError::Sync(format!("Failed to serialize message: {}", e)))?;

    let len = data.len() as u32;
    if len > MAX_MESSAGE_SIZE {
        return Err(NearClipError::Sync(format!("Message too large: {} bytes", len)));
    }

    // 写入长度前缀 (4字节大端序)
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await
        .map_err(|e| NearClipError::Network(format!("Failed to write length prefix: {}", e)))?;

    // 写入消息数据
    writer.write_all(&data).await
        .map_err(|e| NearClipError::Network(format!("Failed to write message data: {}", e)))?;

    writer.flush().await
        .map_err(|e| NearClipError::Network(format!("Failed to flush: {}", e)))?;

    Ok(())
}

/// 通过分离的读半连接接收带长度前缀的消息
///
/// 返回 None 表示连接已关闭
async fn receive_framed_message_split(reader: &mut TcpReadHalf) -> Result<Option<Message>> {
    // 读取长度前缀 (4字节大端序)
    let mut len_buf = [0u8; 4];
    let n = reader.read(&mut len_buf).await
        .map_err(|e| NearClipError::Network(format!("Failed to read length prefix: {}", e)))?;

    if n == 0 {
        // 连接已关闭
        return Ok(None);
    }

    if n < 4 {
        // 部分读取，继续读取剩余字节
        let mut total = n;
        while total < 4 {
            let m = reader.read(&mut len_buf[total..]).await
                .map_err(|e| NearClipError::Network(format!("Failed to read length prefix: {}", e)))?;
            if m == 0 {
                return Ok(None);
            }
            total += m;
        }
    }

    let len = u32::from_be_bytes(len_buf);

    if len > MAX_MESSAGE_SIZE {
        return Err(NearClipError::Sync(format!("Message too large: {} bytes", len)));
    }

    // 读取消息数据
    let mut data = vec![0u8; len as usize];
    let mut total = 0;
    while total < len as usize {
        let n = reader.read(&mut data[total..]).await
            .map_err(|e| NearClipError::Network(format!("Failed to read message data: {}", e)))?;
        if n == 0 {
            return Err(NearClipError::Network("Connection closed while reading message".to_string()));
        }
        total += n;
    }

    // 反序列化消息
    let msg = Message::deserialize(&data)
        .map_err(|e| NearClipError::Sync(format!("Failed to deserialize message: {}", e)))?;

    Ok(Some(msg))
}

// ============================================================
// NearClipCallback - 回调接口
// ============================================================

/// NearClip 回调接口
///
/// 平台客户端实现此 trait 以接收事件通知。
///
/// # 示例
///
/// ```
/// use nearclip_core::{NearClipCallback, DeviceInfo, NearClipError};
///
/// struct LoggingCallback;
///
/// impl NearClipCallback for LoggingCallback {
///     fn on_device_connected(&self, device: &DeviceInfo) {
///         println!("Connected: {}", device.name());
///     }
///     fn on_device_disconnected(&self, device_id: &str) {
///         println!("Disconnected: {}", device_id);
///     }
///     fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
///         println!("Clipboard from {}: {} bytes", from_device, content.len());
///     }
///     fn on_sync_error(&self, error: &NearClipError) {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
pub trait NearClipCallback: Send + Sync {
    /// 设备连接成功时调用
    fn on_device_connected(&self, device: &DeviceInfo);

    /// 设备断开连接时调用
    fn on_device_disconnected(&self, device_id: &str);

    /// 收到剪贴板内容时调用
    fn on_clipboard_received(&self, content: &[u8], from_device: &str);

    /// 发生同步错误时调用
    fn on_sync_error(&self, error: &NearClipError);
}

// ============================================================
// NoOpCallback - 空回调实现
// ============================================================

/// 空回调实现
///
/// 不执行任何操作的回调，用于测试或不需要回调的场景。
#[derive(Debug, Default)]
pub struct NoOpCallback;

impl NearClipCallback for NoOpCallback {
    fn on_device_connected(&self, _device: &DeviceInfo) {}
    fn on_device_disconnected(&self, _device_id: &str) {}
    fn on_clipboard_received(&self, _content: &[u8], _from_device: &str) {}
    fn on_sync_error(&self, _error: &NearClipError) {}
}

// ============================================================
// ManagerState - 内部状态
// ============================================================

/// 管理器内部状态
#[derive(Default)]
struct ManagerState {
    /// 已配对设备列表
    paired_devices: HashMap<String, DeviceInfo>,
    /// 当前使用的通道
    current_channel: Option<Channel>,
}

// ============================================================
// NetworkServices - 网络服务组件
// ============================================================

/// 连接包装器，包含分离的读写半连接和接收任务
///
/// 使用分离的读写流避免发送时阻塞在接收任务持有的锁上
struct ConnectionHandle {
    /// 写半连接 (用于发送，受 mutex 保护以支持并发发送)
    writer: Arc<TokioMutex<TcpWriteHalf>>,
    /// 接收任务句柄 (读半连接由任务独占)
    recv_task: Option<JoinHandle<()>>,
}

/// 网络服务组件
///
/// 管理 TCP 服务器、mDNS 广播和发现服务。
struct NetworkServices {
    /// TLS 证书
    tls_cert: TlsCertificate,
    /// TCP 服务器 (Arc 包装以支持共享给 accept 任务)
    tcp_server: Option<Arc<TcpServer>>,
    /// 服务器端口
    server_port: u16,
    /// mDNS 广播器
    mdns_advertiser: Option<MdnsAdvertiser>,
    /// mDNS 发现器
    mdns_discovery: Option<MdnsDiscovery>,
    /// 连接接受任务
    accept_task: Option<JoinHandle<()>>,
    /// 发现事件处理任务
    discovery_task: Option<JoinHandle<()>>,
    /// 活跃连接 (device_id -> ConnectionHandle)
    connections: HashMap<String, ConnectionHandle>,
}

impl NetworkServices {
    fn new(tls_cert: TlsCertificate) -> Self {
        Self {
            tls_cert,
            tcp_server: None,
            server_port: 0,
            mdns_advertiser: None,
            mdns_discovery: None,
            accept_task: None,
            discovery_task: None,
            connections: HashMap::new(),
        }
    }
}

// ============================================================
// NearClipManager - 核心管理器
// ============================================================

/// NearClip 核心管理器
///
/// 提供统一的 API 管理所有同步功能。
///
/// # 生命周期
///
/// 1. 创建 `NearClipManager::new(config, callback)`
/// 2. 启动 `manager.start().await`
/// 3. 同步剪贴板 `manager.sync_clipboard(content).await`
/// 4. 停止 `manager.stop().await`
///
/// # 示例
///
/// ```
/// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
/// use std::sync::Arc;
///
/// let config = NearClipConfig::new("Test Device");
/// let callback = Arc::new(NoOpCallback);
/// let manager = NearClipManager::new(config, callback).unwrap();
///
/// assert!(!manager.is_running());
/// assert_eq!(manager.get_connected_devices().len(), 0);
/// ```
pub struct NearClipManager {
    /// 配置
    config: NearClipConfig,
    /// 设备 ID (用于 mDNS 广播和消息标识)
    device_id: String,
    /// 回调
    callback: Arc<dyn NearClipCallback>,
    /// 运行状态
    running: AtomicBool,
    /// 内部状态 (Arc 包装以支持共享给后台任务)
    state: Arc<RwLock<ManagerState>>,
    /// 网络服务 (需要 async 访问，使用 TokioMutex，Arc 包装以支持共享给后台任务)
    network: Arc<TokioMutex<Option<NetworkServices>>>,
}

impl NearClipManager {
    /// 创建新的管理器实例
    ///
    /// # 参数
    ///
    /// * `config` - 配置
    /// * `callback` - 回调实现
    ///
    /// # 错误
    ///
    /// 如果配置验证失败，返回错误。
    ///
    /// # 示例
    ///
    /// ```
    /// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
    /// use std::sync::Arc;
    ///
    /// let config = NearClipConfig::new("Device");
    /// let callback = Arc::new(NoOpCallback);
    /// let manager = NearClipManager::new(config, callback);
    /// assert!(manager.is_ok());
    /// ```
    pub fn new(config: NearClipConfig, callback: Arc<dyn NearClipCallback>) -> Result<Self> {
        config.validate()?;

        // 使用配置中的设备 ID，如果没有则生成新的
        let device_id = config
            .device_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().to_uppercase());

        tracing::info!(
            device_name = config.device_name(),
            device_id = %device_id,
            wifi = config.wifi_enabled(),
            ble = config.ble_enabled(),
            "Creating NearClipManager"
        );

        Ok(Self {
            config,
            device_id,
            callback,
            running: AtomicBool::new(false),
            state: Arc::new(RwLock::new(ManagerState::default())),
            network: Arc::new(TokioMutex::new(None)),
        })
    }

    /// 获取设备 ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// 启动服务
    ///
    /// 启动 mDNS 广播、TCP 服务器、BLE 广播等。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use nearclip_core::{NearClipManager, NearClipConfig, NoOpCallback};
    /// use std::sync::Arc;
    ///
    /// # async fn example() {
    /// let config = NearClipConfig::new("Device");
    /// let callback = Arc::new(NoOpCallback);
    /// let manager = NearClipManager::new(config, callback).unwrap();
    ///
    /// manager.start().await.unwrap();
    /// assert!(manager.is_running());
    ///
    /// manager.stop().await;
    /// assert!(!manager.is_running());
    /// # }
    /// ```
    pub async fn start(&self) -> Result<()> {
        if self.running.load(Ordering::Acquire) {
            tracing::warn!("Manager already running");
            return Ok(());
        }

        tracing::info!("Starting NearClipManager");

        if self.config.wifi_enabled() {
            // 1. 生成 TLS 证书
            let tls_cert = TlsCertificate::generate(&["nearclip.local".to_string()])
                .map_err(|e| NearClipError::Network(format!("Failed to generate TLS cert: {}", e)))?;

            let tls_server_config = TlsServerConfig::new(&tls_cert)
                .map_err(|e| NearClipError::Network(format!("Failed to create TLS config: {}", e)))?;

            // 2. 启动 TCP 服务器 (使用动态端口 0)
            let server_config = TcpServerConfig::new().with_port(0);
            let tcp_server = TcpServer::bind(server_config, tls_server_config.config())
                .await
                .map_err(|e| NearClipError::Network(format!("Failed to bind TCP server: {}", e)))?;

            let server_port = tcp_server.local_addr()
                .map_err(|e| NearClipError::Network(format!("Failed to get server address: {}", e)))?
                .port();

            tracing::info!(port = server_port, "TCP server started");

            // 3. 启动 mDNS 广播
            let pubkey_hash = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &tls_cert.cert_der()[..32.min(tls_cert.cert_der().len())]
            );
            let mdns_config = MdnsServiceConfig::new(
                self.device_id.clone(),
                pubkey_hash,
                server_port,
            );
            let mut mdns_advertiser = MdnsAdvertiser::new(mdns_config)
                .map_err(|e| NearClipError::Network(format!("Failed to create mDNS advertiser: {}", e)))?;
            mdns_advertiser.start().await
                .map_err(|e| NearClipError::Network(format!("Failed to start mDNS advertiser: {}", e)))?;

            tracing::info!(device_id = %self.device_id, "mDNS advertiser started");

            // 4. 启动 mDNS 发现
            let mut mdns_discovery = MdnsDiscovery::new()
                .map_err(|e| NearClipError::Network(format!("Failed to create mDNS discovery: {}", e)))?;
            mdns_discovery.start().await
                .map_err(|e| NearClipError::Network(format!("Failed to start mDNS discovery: {}", e)))?;

            tracing::info!("mDNS discovery started");

            // 包装 TCP 服务器为 Arc 以支持共享
            let tcp_server = Arc::new(tcp_server);
            let tcp_server_for_accept = tcp_server.clone();

            // 创建 accept 任务
            let network_for_accept = self.network.clone();
            let callback_for_accept = self.callback.clone();
            let state_for_accept = self.state.clone();

            let accept_task = tokio::spawn(async move {
                tracing::info!("Accept task started");
                loop {
                    match tcp_server_for_accept.accept().await {
                        Ok(conn) => {
                            let peer_addr = conn.peer_addr();
                            tracing::info!(peer = %peer_addr, "Incoming connection accepted");

                            // 使用 peer IP 作为临时标识，收到 PairingRequest 后会更新为真实设备 ID
                            let temp_device_id = format!("incoming_{}", peer_addr);

                            // 分离连接为读写半连接，避免读写死锁
                            let (reader, writer) = conn.into_split();
                            let writer = Arc::new(TokioMutex::new(writer));
                            let device_id_for_recv = temp_device_id.clone();
                            let callback_for_recv = callback_for_accept.clone();
                            let state_for_recv = state_for_accept.clone();
                            let network_for_recv = network_for_accept.clone();

                            // 启动接收任务 (独占读半连接，无需锁)
                            let recv_task = tokio::spawn(async move {
                                let mut reader = reader;
                                let mut actual_device_id = device_id_for_recv.clone();
                                tracing::info!(device_id = %device_id_for_recv, "Receive task started");
                                loop {
                                    let msg = receive_framed_message_split(&mut reader).await;

                                    match msg {
                                        Ok(Some(message)) => {
                                            tracing::debug!(
                                                device_id = %actual_device_id,
                                                msg_type = ?message.msg_type,
                                                from = %message.device_id,
                                                "Message received"
                                            );

                                            match message.msg_type {
                                                MessageType::ClipboardSync => {
                                                    tracing::info!(
                                                        from = %message.device_id,
                                                        size = message.payload.len(),
                                                        "Clipboard received"
                                                    );
                                                    callback_for_recv.on_clipboard_received(
                                                        &message.payload,
                                                        &message.device_id,
                                                    );
                                                }
                                                MessageType::PairingRequest => {
                                                    // 解析配对请求载荷
                                                    match PairingPayload::deserialize(&message.payload) {
                                                        Ok(payload) => {
                                                            tracing::info!(
                                                                from_id = %payload.device_id,
                                                                from_name = %payload.device_name,
                                                                platform = ?payload.platform,
                                                                "PairingRequest received, adding device"
                                                            );

                                                            // 创建新设备信息并添加到配对设备列表
                                                            let device = DeviceInfo::new(
                                                                payload.device_id.clone(),
                                                                payload.device_name.clone(),
                                                            )
                                                            .with_platform(protocol_platform_to_device(payload.platform))
                                                            .with_status(DeviceStatus::Connected);

                                                            // 更新连接映射中的设备 ID
                                                            let old_device_id = device_id_for_recv.clone();
                                                            let new_device_id = payload.device_id.clone();

                                                            // 将设备添加到配对列表
                                                            {
                                                                let mut state = state_for_recv.write().unwrap();
                                                                state.paired_devices.insert(new_device_id.clone(), device.clone());
                                                            }

                                                            // 更新连接映射：将旧的临时 ID 映射移到新的真实设备 ID
                                                            {
                                                                let mut network = network_for_recv.lock().await;
                                                                if let Some(ref mut services) = *network {
                                                                    if let Some(handle) = services.connections.remove(&old_device_id) {
                                                                        services.connections.insert(new_device_id.clone(), handle);
                                                                        tracing::info!(
                                                                            old_id = %old_device_id,
                                                                            new_id = %new_device_id,
                                                                            "Connection remapped to real device ID"
                                                                        );
                                                                    }
                                                                }
                                                            }

                                                            actual_device_id = new_device_id;

                                                            // 触发设备连接回调
                                                            callback_for_recv.on_device_connected(&device);
                                                        }
                                                        Err(e) => {
                                                            tracing::warn!(
                                                                error = %e,
                                                                "Failed to deserialize PairingRequest payload"
                                                            );
                                                        }
                                                    }
                                                }
                                                MessageType::Heartbeat => {
                                                    tracing::debug!(from = %message.device_id, "Heartbeat received");
                                                }
                                                MessageType::Ack => {
                                                    tracing::debug!(from = %message.device_id, "Ack received");
                                                }
                                                _ => {
                                                    tracing::debug!(
                                                        msg_type = ?message.msg_type,
                                                        "Unhandled message type"
                                                    );
                                                }
                                            }
                                        }
                                        Ok(None) => {
                                            tracing::info!(device_id = %actual_device_id, "Connection closed by peer");
                                            break;
                                        }
                                        Err(e) => {
                                            tracing::warn!(device_id = %actual_device_id, error = %e, "Receive error");
                                            break;
                                        }
                                    }
                                }
                                tracing::info!(device_id = %actual_device_id, "Receive task ended");
                            });

                            // 创建连接句柄
                            let handle = ConnectionHandle {
                                writer,
                                recv_task: Some(recv_task),
                            };

                            // 存储连接
                            {
                                let mut network = network_for_accept.lock().await;
                                if let Some(ref mut services) = *network {
                                    services.connections.insert(temp_device_id.clone(), handle);
                                    tracing::info!(device_id = %temp_device_id, "Connection stored");
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "Failed to accept connection");
                            // 继续尝试接受其他连接
                        }
                    }
                }
            });

            // 创建网络服务
            let mut network_services = NetworkServices::new(tls_cert);
            network_services.tcp_server = Some(tcp_server);
            network_services.server_port = server_port;
            network_services.mdns_advertiser = Some(mdns_advertiser);
            network_services.mdns_discovery = Some(mdns_discovery);
            network_services.accept_task = Some(accept_task);

            // 存储网络服务
            {
                let mut network = self.network.lock().await;
                *network = Some(network_services);
            }
        }

        self.running.store(true, Ordering::Release);

        // 设置初始通道
        {
            let mut state = self.state.write().unwrap();
            if self.config.wifi_enabled() {
                state.current_channel = Some(Channel::Wifi);
            } else if self.config.ble_enabled() {
                state.current_channel = Some(Channel::Ble);
            }
        }

        tracing::info!("NearClipManager started");
        Ok(())
    }

    /// 停止服务
    ///
    /// 停止所有后台服务，断开所有连接。
    pub async fn stop(&self) {
        if !self.running.load(Ordering::Acquire) {
            tracing::warn!("Manager not running");
            return;
        }

        tracing::info!("Stopping NearClipManager");

        // 停止网络服务
        {
            let mut network = self.network.lock().await;
            if let Some(ref mut services) = *network {
                // 1. 停止后台任务
                if let Some(handle) = services.accept_task.take() {
                    handle.abort();
                    tracing::debug!("Accept task stopped");
                }
                if let Some(handle) = services.discovery_task.take() {
                    handle.abort();
                    tracing::debug!("Discovery task stopped");
                }

                // 2. 关闭所有连接
                for (device_id, mut handle) in services.connections.drain() {
                    // 先停止接收任务
                    if let Some(recv_task) = handle.recv_task.take() {
                        recv_task.abort();
                        tracing::debug!(device_id = %device_id, "Receive task aborted");
                    }
                    // 写半连接在被 drop 时会关闭
                    drop(handle.writer);
                    tracing::debug!(device_id = %device_id, "Connection closed");
                }

                // 3. 停止 mDNS 发现
                if let Some(ref mut discovery) = services.mdns_discovery {
                    if let Err(e) = discovery.stop().await {
                        tracing::warn!(error = %e, "Failed to stop mDNS discovery");
                    }
                    tracing::debug!("mDNS discovery stopped");
                }

                // 4. 停止 mDNS 广播
                if let Some(ref mut advertiser) = services.mdns_advertiser {
                    if let Err(e) = advertiser.stop().await {
                        tracing::warn!(error = %e, "Failed to stop mDNS advertiser");
                    }
                    tracing::debug!("mDNS advertiser stopped");
                }

                // 5. TCP 服务器会在 drop 时自动关闭
                services.tcp_server = None;
                tracing::debug!("TCP server stopped");
            }
            *network = None;
        }

        // 断开所有设备（更新状态）
        // 注意：先收集需要回调的设备 ID，释放锁后再调用回调，避免死锁
        let disconnected_ids: Vec<String> = {
            let mut state = self.state.write().unwrap();
            let ids: Vec<String> = state
                .paired_devices
                .iter_mut()
                .filter_map(|(device_id, device)| {
                    if device.status().is_connected() {
                        device.set_status(DeviceStatus::Disconnected);
                        Some(device_id.clone())
                    } else {
                        None
                    }
                })
                .collect();
            state.current_channel = None;
            ids
        };

        // 在锁外调用回调
        for device_id in disconnected_ids {
            self.callback.on_device_disconnected(&device_id);
        }

        self.running.store(false, Ordering::Release);

        tracing::info!("NearClipManager stopped");
    }

    /// 同步剪贴板内容
    ///
    /// 将剪贴板内容发送到所有已连接设备。
    ///
    /// # 参数
    ///
    /// * `content` - 剪贴板内容
    ///
    /// # 错误
    ///
    /// - 管理器未运行
    /// - 没有可用通道
    /// - 发送失败
    pub async fn sync_clipboard(&self, content: &[u8]) -> Result<()> {
        tracing::info!(content_size = content.len(), "sync_clipboard called");

        if !self.running.load(Ordering::Acquire) {
            tracing::warn!("sync_clipboard: Manager not running");
            return Err(NearClipError::Sync("Manager not running".to_string()));
        }

        tracing::debug!("sync_clipboard: Manager is running, checking channel");

        let state = self.state.read().unwrap();
        let channel = match state.current_channel {
            Some(ch) => ch,
            None => {
                tracing::warn!("sync_clipboard: No channel available");
                return Err(NearClipError::Sync("No channel available".to_string()));
            }
        };
        drop(state);

        tracing::debug!(channel = ?channel, "sync_clipboard: Channel found");

        // 创建剪贴板同步消息
        let msg = Message::clipboard_sync(content, self.device_id.clone());

        tracing::debug!("sync_clipboard: Acquiring network lock");

        // 发送到所有活跃连接
        let network = self.network.lock().await;

        tracing::debug!("sync_clipboard: Network lock acquired");

        if let Some(ref services) = *network {
            let connection_ids: Vec<String> = services.connections.keys().cloned().collect();

            if connection_ids.is_empty() {
                tracing::debug!("No active connections, skipping sync");
                return Ok(());
            }

            tracing::info!(
                content_size = content.len(),
                channel = ?channel,
                connection_count = connection_ids.len(),
                "Syncing clipboard"
            );

            let mut failed_devices = Vec::new();

            for device_id in &connection_ids {
                if let Some(handle) = services.connections.get(device_id) {
                    let mut writer = handle.writer.lock().await;
                    match send_framed_message_split(&mut *writer, &msg).await {
                        Ok(_) => {
                            tracing::debug!(device_id = %device_id, "Sent clipboard");
                        }
                        Err(e) => {
                            tracing::error!(device_id = %device_id, error = %e, "Failed to send clipboard");
                            failed_devices.push(device_id.clone());
                        }
                    }
                }
            }

            drop(network);

            // 移除失败的连接
            if !failed_devices.is_empty() {
                let mut network = self.network.lock().await;
                if let Some(ref mut services) = *network {
                    for device_id in &failed_devices {
                        if let Some(mut handle) = services.connections.remove(device_id) {
                            // 停止接收任务
                            if let Some(recv_task) = handle.recv_task.take() {
                                recv_task.abort();
                            }
                        }
                        // 更新设备状态
                        let mut state = self.state.write().unwrap();
                        if let Some(device) = state.paired_devices.get_mut(device_id) {
                            device.set_status(DeviceStatus::Disconnected);
                        }
                        drop(state);
                        self.callback.on_device_disconnected(device_id);
                    }
                }
            }
        } else {
            tracing::warn!("sync_clipboard: No network services available");
        }

        tracing::info!("sync_clipboard completed");
        Ok(())
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    /// 获取配置
    pub fn config(&self) -> &NearClipConfig {
        &self.config
    }

    /// 获取当前通道
    pub fn current_channel(&self) -> Option<Channel> {
        self.state.read().unwrap().current_channel
    }

    // --------------------------------------------------------
    // 设备管理
    // --------------------------------------------------------

    /// 获取已配对设备列表
    pub fn get_paired_devices(&self) -> Vec<DeviceInfo> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .values()
            .cloned()
            .collect()
    }

    /// 获取已连接设备列表
    pub fn get_connected_devices(&self) -> Vec<DeviceInfo> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .values()
            .filter(|d| d.status().is_connected())
            .cloned()
            .collect()
    }

    /// 获取设备状态
    pub fn get_device_status(&self, device_id: &str) -> Option<DeviceStatus> {
        self.state
            .read()
            .unwrap()
            .paired_devices
            .get(device_id)
            .map(|d| d.status())
    }

    /// 添加已配对设备
    ///
    /// 内部方法，用于添加新配对的设备。
    pub fn add_paired_device(&self, device: DeviceInfo) {
        let device_id = device.id().to_string();
        tracing::info!(device_id = %device_id, "Adding paired device");

        self.state
            .write()
            .unwrap()
            .paired_devices
            .insert(device_id, device);
    }

    /// 移除已配对设备
    pub fn remove_paired_device(&self, device_id: &str) -> Option<DeviceInfo> {
        tracing::info!(device_id = %device_id, "Removing paired device");

        self.state
            .write()
            .unwrap()
            .paired_devices
            .remove(device_id)
    }

    /// 连接设备
    ///
    /// 尝试连接到指定设备。
    ///
    /// # 参数
    ///
    /// * `device_id` - 设备 ID
    ///
    /// # 错误
    ///
    /// - 管理器未运行
    /// - 设备未找到
    /// - 连接失败
    pub async fn connect_device(&self, device_id: &str) -> Result<()> {
        if !self.running.load(Ordering::Acquire) {
            return Err(NearClipError::Sync("Manager not running".to_string()));
        }

        {
            let state = self.state.read().unwrap();
            if !state.paired_devices.contains_key(device_id) {
                return Err(NearClipError::DeviceNotFound(device_id.to_string()));
            }
        }

        tracing::info!(device_id = %device_id, "Connecting to device");

        // 设置为连接中
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Connecting);
            }
        }

        // 从 mDNS 发现中获取设备地址
        tracing::debug!(device_id = %device_id, "Looking up device in mDNS discovery");
        let discovered_device = {
            let network = self.network.lock().await;
            tracing::debug!(device_id = %device_id, has_network = network.is_some(), "Got network lock");
            if let Some(ref services) = *network {
                if let Some(ref discovery) = services.mdns_discovery {
                    tracing::debug!(device_id = %device_id, "Calling discovery.get_device");
                    let result = discovery.get_device(device_id).await;
                    tracing::debug!(device_id = %device_id, found = result.is_some(), "Discovery lookup complete");
                    result
                } else {
                    tracing::warn!(device_id = %device_id, "No mDNS discovery service");
                    None
                }
            } else {
                tracing::warn!(device_id = %device_id, "No network services");
                None
            }
        };

        let discovered = match discovered_device {
            Some(d) => d,
            None => {
                // 设备未在网络上发现，重置状态
                let mut state = self.state.write().unwrap();
                if let Some(device) = state.paired_devices.get_mut(device_id) {
                    device.set_status(DeviceStatus::Disconnected);
                }
                return Err(NearClipError::Network(format!(
                    "Device {} not discovered on network", device_id
                )));
            }
        };

        // 获取设备的可用地址，优先使用 IPv4
        // 因为 IPv6 链路本地地址 (fe80::) 在跨设备连接时需要 scope_id
        let addr = {
            use std::net::IpAddr;

            // 优先选择 IPv4 地址
            let ipv4_addr = discovered.addresses.iter()
                .find(|a| matches!(a, IpAddr::V4(_)));

            // 如果没有 IPv4，尝试找非链路本地的 IPv6 地址
            let non_link_local_v6 = discovered.addresses.iter()
                .find(|a| match a {
                    IpAddr::V6(v6) => !v6.is_loopback() && !is_link_local_v6(v6),
                    _ => false,
                });

            ipv4_addr.or(non_link_local_v6).or_else(|| discovered.addresses.iter().next())
                .ok_or_else(|| {
                    NearClipError::Network(format!("No address found for device {}", device_id))
                })?
        };

        let socket_addr = SocketAddr::new(*addr, discovered.port);
        tracing::debug!(device_id = %device_id, addr = %socket_addr, "Connecting to device");

        // 创建 TLS 客户端配置
        // TODO: 实现 TOFU 模型 - 配对时保存对端证书并在连接时验证
        // 目前使用不验证证书的配置用于测试
        let tls_client_config = TlsClientConfig::new_insecure()
            .map_err(|e| NearClipError::Network(format!("Failed to create TLS client config: {}", e)))?;

        // 建立 TLS 连接
        let client_config = TcpClientConfig::new(socket_addr);
        let conn = TcpClient::connect(client_config, tls_client_config.config(), "nearclip.local")
            .await
            .map_err(|e| {
                // 连接失败，重置状态
                let mut state = self.state.write().unwrap();
                if let Some(device) = state.paired_devices.get_mut(device_id) {
                    device.set_status(DeviceStatus::Disconnected);
                }
                NearClipError::Network(format!("Failed to connect to device {}: {}", device_id, e))
            })?;

        tracing::info!(device_id = %device_id, "Connected to device");

        // 分离连接为读写半连接，避免读写死锁
        let (reader, writer) = conn.into_split();
        let writer = Arc::new(TokioMutex::new(writer));
        let device_id_for_recv = device_id.to_string();
        let callback_for_recv = self.callback.clone();

        // 启动接收任务 (独占读半连接，无需锁)
        let recv_task = tokio::spawn(async move {
            let mut reader = reader;
            tracing::info!(device_id = %device_id_for_recv, "Receive task started (outgoing connection)");
            loop {
                let msg = receive_framed_message_split(&mut reader).await;

                match msg {
                    Ok(Some(message)) => {
                        tracing::debug!(
                            device_id = %device_id_for_recv,
                            msg_type = ?message.msg_type,
                            from = %message.device_id,
                            "Message received"
                        );

                        match message.msg_type {
                            MessageType::ClipboardSync => {
                                tracing::info!(
                                    from = %message.device_id,
                                    size = message.payload.len(),
                                    "Clipboard received"
                                );
                                callback_for_recv.on_clipboard_received(
                                    &message.payload,
                                    &message.device_id,
                                );
                            }
                            MessageType::Heartbeat => {
                                tracing::debug!(from = %message.device_id, "Heartbeat received");
                            }
                            MessageType::Ack => {
                                tracing::debug!(from = %message.device_id, "Ack received");
                            }
                            _ => {
                                tracing::debug!(
                                    msg_type = ?message.msg_type,
                                    "Unhandled message type"
                                );
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::info!(device_id = %device_id_for_recv, "Connection closed by peer");
                        break;
                    }
                    Err(e) => {
                        tracing::warn!(device_id = %device_id_for_recv, error = %e, "Receive error");
                        break;
                    }
                }
            }
            tracing::info!(device_id = %device_id_for_recv, "Receive task ended");
        });

        // 创建连接句柄
        let handle = ConnectionHandle {
            writer,
            recv_task: Some(recv_task),
        };

        // 保存连接
        {
            let mut network = self.network.lock().await;
            if let Some(ref mut services) = *network {
                services.connections.insert(device_id.to_string(), handle);
            }
        }

        // 发送 PairingRequest，告诉对方自己的设备信息
        {
            // 获取本设备的平台信息
            let my_platform = if cfg!(target_os = "macos") {
                ProtocolPlatform::MacOS
            } else if cfg!(target_os = "android") {
                ProtocolPlatform::Android
            } else {
                ProtocolPlatform::Unknown
            };

            let pairing_payload = PairingPayload::new(
                self.device_id.clone(),
                self.config.device_name().to_string(),
                my_platform,
            );

            if let Ok(payload_bytes) = pairing_payload.serialize() {
                let pairing_msg = Message::pairing_request(payload_bytes, self.device_id.clone());

                let mut network = self.network.lock().await;
                if let Some(ref mut services) = *network {
                    if let Some(handle) = services.connections.get(device_id) {
                        let mut writer = handle.writer.lock().await;
                        if let Err(e) = send_framed_message_split(&mut writer, &pairing_msg).await {
                            tracing::warn!(device_id = %device_id, error = %e, "Failed to send PairingRequest");
                        } else {
                            tracing::info!(device_id = %device_id, "PairingRequest sent");
                        }
                    }
                }
            }
        }

        // 更新设备状态
        // 注意：先释放写锁再调用回调，避免回调中调用 get_connected_devices 导致死锁
        let device_for_callback = {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Connected);
                Some(device.clone())
            } else {
                None
            }
        };

        // 在锁外调用回调
        if let Some(device) = device_for_callback {
            self.callback.on_device_connected(&device);
        }

        Ok(())
    }

    /// 断开设备连接
    ///
    /// # 参数
    ///
    /// * `device_id` - 设备 ID
    pub async fn disconnect_device(&self, device_id: &str) -> Result<()> {
        {
            let state = self.state.read().unwrap();
            if !state.paired_devices.contains_key(device_id) {
                return Err(NearClipError::DeviceNotFound(device_id.to_string()));
            }
        }

        tracing::info!(device_id = %device_id, "Disconnecting device");

        // 关闭连接
        {
            let mut network = self.network.lock().await;
            if let Some(ref mut services) = *network {
                if let Some(mut handle) = services.connections.remove(device_id) {
                    // 先停止接收任务
                    if let Some(recv_task) = handle.recv_task.take() {
                        recv_task.abort();
                        tracing::debug!(device_id = %device_id, "Receive task aborted");
                    }
                    // 写半连接在被 drop 时会关闭
                    drop(handle.writer);
                    tracing::debug!(device_id = %device_id, "Connection closed");
                }
            }
        }

        // 更新设备状态
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Disconnected);
            }
        }

        self.callback.on_device_disconnected(device_id);

        Ok(())
    }

    // --------------------------------------------------------
    // 设备发现方法
    // --------------------------------------------------------

    /// 获取网络上发现的设备列表
    ///
    /// 返回通过 mDNS 发现的所有设备，无论是否已配对。
    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        let network = self.network.lock().await;
        if let Some(ref services) = *network {
            if let Some(ref discovery) = services.mdns_discovery {
                return discovery.get_devices().await;
            }
        }
        Vec::new()
    }

    /// 尝试连接所有已配对但未连接的发现设备
    ///
    /// 遍历发现的设备，如果设备已配对但未连接，尝试建立连接。
    /// 返回成功连接的设备数量。
    pub async fn try_connect_paired_devices(&self) -> usize {
        if !self.running.load(Ordering::Acquire) {
            return 0;
        }

        let discovered = self.get_discovered_devices().await;
        let paired_ids: Vec<String> = {
            let state = self.state.read().unwrap();
            state.paired_devices.keys().cloned().collect()
        };

        let mut connected = 0;
        for device in discovered {
            if paired_ids.contains(&device.device_id) {
                // 检查是否已连接
                let already_connected = {
                    let state = self.state.read().unwrap();
                    state.paired_devices
                        .get(&device.device_id)
                        .map(|d| d.status().is_connected())
                        .unwrap_or(false)
                };

                if !already_connected {
                    tracing::info!(device_id = %device.device_id, "Auto-connecting to discovered paired device");
                    match self.connect_device(&device.device_id).await {
                        Ok(_) => {
                            connected += 1;
                            tracing::info!(device_id = %device.device_id, "Auto-connected successfully");
                        }
                        Err(e) => {
                            tracing::warn!(device_id = %device.device_id, error = %e, "Auto-connect failed");
                        }
                    }
                }
            }
        }

        connected
    }

    // --------------------------------------------------------
    // 内部方法 - 用于底层模块调用
    // --------------------------------------------------------

    /// 处理收到的剪贴板内容
    ///
    /// 由底层模块调用，当收到远程剪贴板内容时。
    pub fn handle_clipboard_received(&self, content: &[u8], from_device: &str) {
        tracing::debug!(
            content_size = content.len(),
            from_device = %from_device,
            "Received clipboard content"
        );

        // 更新设备活动时间
        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(from_device) {
                device.touch();
            }
        }

        self.callback.on_clipboard_received(content, from_device);
    }

    /// 处理同步错误
    ///
    /// 由底层模块调用，当发生错误时。
    pub fn handle_sync_error(&self, error: NearClipError) {
        tracing::error!(error = %error, "Sync error occurred");
        self.callback.on_sync_error(&error);
    }

    /// 处理设备连接
    ///
    /// 由底层模块调用，当设备连接成功时。
    pub fn handle_device_connected(&self, device: DeviceInfo) {
        let device_id = device.id().to_string();
        tracing::info!(device_id = %device_id, "Device connected");

        {
            let mut state = self.state.write().unwrap();
            state.paired_devices.insert(device_id, device.clone());
        }

        self.callback.on_device_connected(&device);
    }

    /// 处理设备断开
    ///
    /// 由底层模块调用，当设备断开连接时。
    pub fn handle_device_disconnected(&self, device_id: &str) {
        tracing::info!(device_id = %device_id, "Device disconnected");

        {
            let mut state = self.state.write().unwrap();
            if let Some(device) = state.paired_devices.get_mut(device_id) {
                device.set_status(DeviceStatus::Disconnected);
            }
        }

        self.callback.on_device_disconnected(device_id);
    }
}

impl std::fmt::Debug for NearClipManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NearClipManager")
            .field("config", &self.config)
            .field("running", &self.is_running())
            .field("paired_devices", &self.get_paired_devices().len())
            .field("connected_devices", &self.get_connected_devices().len())
            .finish()
    }
}

/// 检查 IPv6 地址是否是链路本地地址
///
/// 链路本地地址 (fe80::/10) 只在本地链路上有效，
/// 跨设备连接时需要指定 scope_id，否则会失败。
fn is_link_local_v6(addr: &std::net::Ipv6Addr) -> bool {
    // 链路本地地址的前 10 位是 1111 1110 10 (fe80::/10)
    let segments = addr.segments();
    (segments[0] & 0xffc0) == 0xfe80
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::DevicePlatform;
    use std::sync::Mutex;

    // 测试回调，记录调用
    struct TestCallback {
        connected: Mutex<Vec<String>>,
        disconnected: Mutex<Vec<String>>,
        clipboard: Mutex<Vec<(Vec<u8>, String)>>,
        errors: Mutex<Vec<String>>,
    }

    impl TestCallback {
        fn new() -> Self {
            Self {
                connected: Mutex::new(Vec::new()),
                disconnected: Mutex::new(Vec::new()),
                clipboard: Mutex::new(Vec::new()),
                errors: Mutex::new(Vec::new()),
            }
        }

        fn connected_count(&self) -> usize {
            self.connected.lock().unwrap().len()
        }

        fn disconnected_count(&self) -> usize {
            self.disconnected.lock().unwrap().len()
        }

        fn clipboard_count(&self) -> usize {
            self.clipboard.lock().unwrap().len()
        }

        fn error_count(&self) -> usize {
            self.errors.lock().unwrap().len()
        }
    }

    impl NearClipCallback for TestCallback {
        fn on_device_connected(&self, device: &DeviceInfo) {
            self.connected.lock().unwrap().push(device.id().to_string());
        }

        fn on_device_disconnected(&self, device_id: &str) {
            self.disconnected.lock().unwrap().push(device_id.to_string());
        }

        fn on_clipboard_received(&self, content: &[u8], from_device: &str) {
            self.clipboard
                .lock()
                .unwrap()
                .push((content.to_vec(), from_device.to_string()));
        }

        fn on_sync_error(&self, error: &NearClipError) {
            self.errors.lock().unwrap().push(error.to_string());
        }
    }

    fn create_manager() -> NearClipManager {
        let config = NearClipConfig::new("Test Device");
        let callback = Arc::new(NoOpCallback);
        NearClipManager::new(config, callback).unwrap()
    }

    fn create_manager_with_callback() -> (NearClipManager, Arc<TestCallback>) {
        let config = NearClipConfig::new("Test Device");
        let callback = Arc::new(TestCallback::new());
        let manager = NearClipManager::new(config, callback.clone()).unwrap();
        (manager, callback)
    }

    // --------------------------------------------------------
    // 创建测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_new() {
        let manager = create_manager();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_manager_new_invalid_config() {
        let config = NearClipConfig::new(""); // 无效名称
        let callback = Arc::new(NoOpCallback);
        let result = NearClipManager::new(config, callback);
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_config() {
        let manager = create_manager();
        assert_eq!(manager.config().device_name(), "Test Device");
    }

    // --------------------------------------------------------
    // 生命周期测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_start_stop() {
        let manager = create_manager();

        assert!(!manager.is_running());

        manager.start().await.unwrap();
        assert!(manager.is_running());

        manager.stop().await;
        assert!(!manager.is_running());
    }

    #[tokio::test]
    async fn test_manager_start_twice() {
        let manager = create_manager();

        manager.start().await.unwrap();
        // 第二次启动不应该失败
        manager.start().await.unwrap();

        assert!(manager.is_running());
    }

    #[tokio::test]
    async fn test_manager_stop_when_not_running() {
        let manager = create_manager();
        // 未启动时停止不应该失败
        manager.stop().await;
        assert!(!manager.is_running());
    }

    // --------------------------------------------------------
    // 通道测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_current_channel_wifi() {
        let config = NearClipConfig::new("Test")
            .with_wifi_enabled(true)
            .with_ble_enabled(false);
        let callback = Arc::new(NoOpCallback);
        let manager = NearClipManager::new(config, callback).unwrap();

        manager.start().await.unwrap();
        assert_eq!(manager.current_channel(), Some(Channel::Wifi));
    }

    #[tokio::test]
    async fn test_manager_current_channel_ble() {
        let config = NearClipConfig::new("Test")
            .with_wifi_enabled(false)
            .with_ble_enabled(true);
        let callback = Arc::new(NoOpCallback);
        let manager = NearClipManager::new(config, callback).unwrap();

        manager.start().await.unwrap();
        assert_eq!(manager.current_channel(), Some(Channel::Ble));
    }

    #[test]
    fn test_manager_current_channel_not_running() {
        let manager = create_manager();
        assert_eq!(manager.current_channel(), None);
    }

    // --------------------------------------------------------
    // 设备管理测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_add_paired_device() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_platform(DevicePlatform::MacOS);
        manager.add_paired_device(device);

        assert_eq!(manager.get_paired_devices().len(), 1);
    }

    #[test]
    fn test_manager_remove_paired_device() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);
        assert_eq!(manager.get_paired_devices().len(), 1);

        let removed = manager.remove_paired_device("d1");
        assert!(removed.is_some());
        assert_eq!(manager.get_paired_devices().len(), 0);
    }

    #[test]
    fn test_manager_get_device_status() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Connected)
        );
        assert_eq!(manager.get_device_status("unknown"), None);
    }

    #[test]
    fn test_manager_get_connected_devices() {
        let manager = create_manager();

        let d1 = DeviceInfo::new("d1", "D1").with_status(DeviceStatus::Connected);
        let d2 = DeviceInfo::new("d2", "D2").with_status(DeviceStatus::Disconnected);
        let d3 = DeviceInfo::new("d3", "D3").with_status(DeviceStatus::Connected);

        manager.add_paired_device(d1);
        manager.add_paired_device(d2);
        manager.add_paired_device(d3);

        assert_eq!(manager.get_paired_devices().len(), 3);
        assert_eq!(manager.get_connected_devices().len(), 2);
    }

    // --------------------------------------------------------
    // 连接/断开测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_connect_device_not_discovered() {
        // Test that connect_device returns error when device is not discovered on network
        let (manager, _callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        manager.start().await.unwrap();

        // Device is paired but not discovered on network - should fail
        let result = manager.connect_device("d1").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(NearClipError::Network(_))));

        // Device status should be disconnected after failed connection attempt
        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Disconnected)
        );
    }

    #[tokio::test]
    async fn test_manager_connect_device_not_found() {
        let manager = create_manager();
        manager.start().await.unwrap();

        let result = manager.connect_device("unknown").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(NearClipError::DeviceNotFound(_))));
    }

    #[tokio::test]
    async fn test_manager_connect_device_not_running() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        let result = manager.connect_device("d1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_disconnect_device() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.disconnect_device("d1").await.unwrap();

        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Disconnected)
        );
        assert_eq!(callback.disconnected_count(), 1);
    }

    #[tokio::test]
    async fn test_manager_disconnect_device_not_found() {
        let manager = create_manager();

        let result = manager.disconnect_device("unknown").await;
        assert!(result.is_err());
    }

    // --------------------------------------------------------
    // 同步测试
    // --------------------------------------------------------

    #[tokio::test]
    async fn test_manager_sync_clipboard_not_running() {
        let manager = create_manager();

        let result = manager.sync_clipboard(b"test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_sync_clipboard_no_connected_devices() {
        let manager = create_manager();
        manager.start().await.unwrap();

        // 没有连接设备，应该成功但不做任何事
        let result = manager.sync_clipboard(b"test").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manager_sync_clipboard_with_devices() {
        let manager = create_manager();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.start().await.unwrap();

        let result = manager.sync_clipboard(b"test content").await;
        assert!(result.is_ok());
    }

    // --------------------------------------------------------
    // 回调测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_handle_clipboard_received() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1");
        manager.add_paired_device(device);

        manager.handle_clipboard_received(b"hello", "d1");

        assert_eq!(callback.clipboard_count(), 1);
    }

    #[test]
    fn test_manager_handle_sync_error() {
        let (manager, callback) = create_manager_with_callback();

        manager.handle_sync_error(NearClipError::Network("test".to_string()));

        assert_eq!(callback.error_count(), 1);
    }

    #[test]
    fn test_manager_handle_device_connected() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.handle_device_connected(device);

        assert_eq!(callback.connected_count(), 1);
        assert_eq!(manager.get_paired_devices().len(), 1);
    }

    #[test]
    fn test_manager_handle_device_disconnected() {
        let (manager, callback) = create_manager_with_callback();

        let device = DeviceInfo::new("d1", "Device 1")
            .with_status(DeviceStatus::Connected);
        manager.add_paired_device(device);

        manager.handle_device_disconnected("d1");

        assert_eq!(callback.disconnected_count(), 1);
        assert_eq!(
            manager.get_device_status("d1"),
            Some(DeviceStatus::Disconnected)
        );
    }

    // --------------------------------------------------------
    // NoOpCallback 测试
    // --------------------------------------------------------

    #[test]
    fn test_noop_callback() {
        let callback = NoOpCallback;
        let device = DeviceInfo::new("d1", "D1");
        let error = NearClipError::Network("test".to_string());

        // 这些不应该 panic
        callback.on_device_connected(&device);
        callback.on_device_disconnected("d1");
        callback.on_clipboard_received(b"test", "d1");
        callback.on_sync_error(&error);
    }

    #[test]
    fn test_noop_callback_debug() {
        let callback = NoOpCallback;
        let debug = format!("{:?}", callback);
        assert!(debug.contains("NoOpCallback"));
    }

    // --------------------------------------------------------
    // Debug 测试
    // --------------------------------------------------------

    #[test]
    fn test_manager_debug() {
        let manager = create_manager();
        let debug = format!("{:?}", manager);
        assert!(debug.contains("NearClipManager"));
        assert!(debug.contains("running"));
    }
}
