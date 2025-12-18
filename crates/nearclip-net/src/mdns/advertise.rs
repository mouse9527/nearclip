//! mDNS 服务广播模块
//!
//! 提供设备在局域网上广播自己存在的功能。

use crate::error::NetError;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, instrument, warn};

/// NearClip mDNS 服务类型
///
/// 格式遵循 DNS-SD 规范：`_<service>._<protocol>.local.`
pub const SERVICE_TYPE: &str = "_nearclip._tcp.local.";

/// TXT 记录键：设备 ID
pub const TXT_DEVICE_ID: &str = "id";

/// TXT 记录键：公钥哈希
pub const TXT_PUBKEY_HASH: &str = "pk";

/// mDNS 服务配置
///
/// 包含广播服务所需的所有配置信息。
///
/// # Example
///
/// ```
/// use nearclip_net::MdnsServiceConfig;
///
/// let config = MdnsServiceConfig::new(
///     "device-001".to_string(),
///     "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
///     12345,
/// );
///
/// assert_eq!(config.device_id(), "device-001");
/// assert_eq!(config.port(), 12345);
/// ```
#[derive(Debug, Clone)]
pub struct MdnsServiceConfig {
    /// 设备唯一标识符
    device_id: String,
    /// 公钥哈希（Base64 编码）
    public_key_hash: String,
    /// 服务监听端口
    port: u16,
    /// 可选的主机名（不含 .local. 后缀）
    hostname: Option<String>,
}

impl MdnsServiceConfig {
    /// 创建新的服务配置
    ///
    /// # Arguments
    ///
    /// * `device_id` - 设备唯一标识符
    /// * `public_key_hash` - 公钥的 Base64 编码哈希
    /// * `port` - 服务监听端口
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_net::MdnsServiceConfig;
    ///
    /// let config = MdnsServiceConfig::new(
    ///     "my-device".to_string(),
    ///     "aGFzaA==".to_string(),
    ///     8080,
    /// );
    /// ```
    pub fn new(device_id: String, public_key_hash: String, port: u16) -> Self {
        Self {
            device_id,
            public_key_hash,
            port,
            hostname: None,
        }
    }

    /// 设置自定义主机名
    ///
    /// 如果不设置，将使用设备 ID 作为主机名。
    /// 主机名只能包含字母、数字和连字符，且不能以连字符开头或结尾。
    ///
    /// # Panics
    ///
    /// 不会 panic，无效主机名会在 `MdnsAdvertiser::new()` 时返回错误。
    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    /// 验证主机名格式是否有效
    ///
    /// DNS 主机名规则：
    /// - 只能包含字母、数字和连字符
    /// - 不能以连字符开头或结尾
    /// - 长度不超过 63 个字符（单个标签）
    fn validate_hostname(hostname: &str) -> Result<(), NetError> {
        // 移除 .local. 后缀进行验证
        let name = hostname
            .trim_end_matches('.')
            .trim_end_matches(".local")
            .trim_end_matches('.');

        if name.is_empty() {
            return Err(NetError::Configuration(
                "hostname cannot be empty".to_string(),
            ));
        }

        if name.len() > 63 {
            return Err(NetError::Configuration(format!(
                "hostname label '{}' exceeds 63 characters",
                name
            )));
        }

        if name.starts_with('-') || name.ends_with('-') {
            return Err(NetError::Configuration(format!(
                "hostname '{}' cannot start or end with hyphen",
                name
            )));
        }

        // 检查是否只包含有效字符
        for ch in name.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '-' {
                return Err(NetError::Configuration(format!(
                    "hostname '{}' contains invalid character '{}'",
                    name, ch
                )));
            }
        }

        Ok(())
    }

    /// 获取设备 ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// 获取公钥哈希
    pub fn public_key_hash(&self) -> &str {
        &self.public_key_hash
    }

    /// 获取端口
    pub fn port(&self) -> u16 {
        self.port
    }

    /// 获取主机名
    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), NetError> {
        if self.device_id.is_empty() {
            return Err(NetError::Configuration("device_id cannot be empty".to_string()));
        }

        // 验证 device_id 作为默认主机名是否有效
        Self::validate_hostname(&self.device_id)?;

        if self.public_key_hash.is_empty() {
            return Err(NetError::Configuration(
                "public_key_hash cannot be empty".to_string(),
            ));
        }
        if self.port == 0 {
            return Err(NetError::Configuration("port cannot be 0".to_string()));
        }

        // 如果设置了自定义主机名，也要验证
        if let Some(ref hostname) = self.hostname {
            Self::validate_hostname(hostname)?;
        }

        // 验证 TXT 记录长度不超过 DNS 限制
        // DNS TXT 记录格式: 每条记录 = 1字节长度前缀 + "key=value"
        // 单条 TXT 记录最大 255 字节（不含长度前缀）
        let txt_record_1_len = TXT_DEVICE_ID.len() + 1 + self.device_id.len(); // "id=<device_id>"
        let txt_record_2_len = TXT_PUBKEY_HASH.len() + 1 + self.public_key_hash.len(); // "pk=<hash>"

        if txt_record_1_len > 255 {
            return Err(NetError::Configuration(format!(
                "TXT record 'id' length {} exceeds 255 bytes limit",
                txt_record_1_len
            )));
        }
        if txt_record_2_len > 255 {
            return Err(NetError::Configuration(format!(
                "TXT record 'pk' length {} exceeds 255 bytes limit",
                txt_record_2_len
            )));
        }

        Ok(())
    }

    /// 构建 TXT 记录属性数组
    pub(crate) fn build_txt_properties(&self) -> Vec<(String, String)> {
        vec![
            (TXT_DEVICE_ID.to_string(), self.device_id.clone()),
            (TXT_PUBKEY_HASH.to_string(), self.public_key_hash.clone()),
        ]
    }
}

/// mDNS 服务广播器
///
/// 负责在局域网上广播设备存在，使其他 NearClip 设备可以发现。
///
/// # Example
///
/// ```no_run
/// use nearclip_net::{MdnsAdvertiser, MdnsServiceConfig};
///
/// # async fn example() -> Result<(), nearclip_net::NetError> {
/// let config = MdnsServiceConfig::new(
///     "device-001".to_string(),
///     "dGVzdC1wdWJrZXktaGFzaA==".to_string(),
///     12345,
/// );
///
/// let mut advertiser = MdnsAdvertiser::new(config)?;
/// advertiser.start().await?;
///
/// // 广播运行中...
///
/// advertiser.stop().await?;
/// # Ok(())
/// # }
/// ```
pub struct MdnsAdvertiser {
    /// mDNS 服务守护进程
    daemon: Arc<Mutex<ServiceDaemon>>,
    /// 服务配置
    config: MdnsServiceConfig,
    /// 已注册服务的完整名称
    service_fullname: Option<String>,
}

impl MdnsAdvertiser {
    /// 创建新的广播器
    ///
    /// # Arguments
    ///
    /// * `config` - mDNS 服务配置
    ///
    /// # Returns
    ///
    /// 成功返回 `MdnsAdvertiser`，失败返回 `NetError`
    #[instrument(skip(config), fields(device_id = %config.device_id))]
    pub fn new(config: MdnsServiceConfig) -> Result<Self, NetError> {
        // 验证配置
        config.validate()?;

        let daemon =
            ServiceDaemon::new().map_err(|e| NetError::Mdns(format!("Failed to create daemon: {}", e)))?;

        debug!(device_id = %config.device_id, "Created mDNS advertiser");

        Ok(Self {
            daemon: Arc::new(Mutex::new(daemon)),
            config,
            service_fullname: None,
        })
    }

    /// 启动服务广播
    ///
    /// 在局域网上注册 mDNS 服务，使其他设备可以发现。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `NetError`
    #[instrument(skip(self), fields(device_id = %self.config.device_id))]
    pub async fn start(&mut self) -> Result<(), NetError> {
        // 如果已经在广播，先停止
        if self.service_fullname.is_some() {
            warn!("Service already advertising, stopping first");
            self.stop().await?;
        }

        let daemon = self.daemon.lock().await;

        // 构建主机名
        let hostname = self
            .config
            .hostname
            .clone()
            .unwrap_or_else(|| format!("{}.local.", self.config.device_id));

        // 确保主机名以 .local. 结尾
        let hostname = if hostname.ends_with(".local.") {
            hostname
        } else if hostname.ends_with(".local") {
            format!("{}.", hostname)
        } else {
            format!("{}.local.", hostname)
        };

        // 构建 TXT 属性
        let properties = self.config.build_txt_properties();
        let properties_refs: Vec<(&str, &str)> = properties
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        // 创建服务信息
        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &self.config.device_id,
            &hostname,
            (),
            self.config.port,
            &properties_refs[..],
        )
        .map_err(|e| NetError::ServiceRegistration(format!("Failed to create service info: {}", e)))?
        .enable_addr_auto();  // 启用地址自动发现

        let fullname = service_info.get_fullname().to_string();

        // 注册服务
        daemon
            .register(service_info)
            .map_err(|e| NetError::ServiceRegistration(format!("Failed to register service: {}", e)))?;

        self.service_fullname = Some(fullname.clone());

        info!(
            service_name = %fullname,
            port = self.config.port,
            device_id = %self.config.device_id,
            "mDNS service registered"
        );

        Ok(())
    }

    /// 停止服务广播
    ///
    /// 从局域网上注销 mDNS 服务。
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`，失败返回 `NetError`
    #[instrument(skip(self), fields(device_id = %self.config.device_id))]
    pub async fn stop(&mut self) -> Result<(), NetError> {
        if let Some(fullname) = self.service_fullname.take() {
            let daemon = self.daemon.lock().await;

            daemon
                .unregister(&fullname)
                .map_err(|e| NetError::Mdns(format!("Failed to unregister service: {}", e)))?;

            info!(service_name = %fullname, "mDNS service unregistered");
        } else {
            debug!("No service to unregister");
        }

        Ok(())
    }

    /// 检查是否正在广播
    ///
    /// # Returns
    ///
    /// 如果服务正在广播返回 `true`
    pub fn is_advertising(&self) -> bool {
        self.service_fullname.is_some()
    }

    /// 获取服务完整名称
    ///
    /// # Returns
    ///
    /// 如果正在广播，返回服务的完整 DNS 名称
    pub fn service_fullname(&self) -> Option<&str> {
        self.service_fullname.as_deref()
    }

    /// 获取当前配置
    pub fn config(&self) -> &MdnsServiceConfig {
        &self.config
    }
}

impl Drop for MdnsAdvertiser {
    fn drop(&mut self) {
        // 如果服务仍在广播，尝试注销
        if let Some(fullname) = self.service_fullname.take() {
            // 尝试同步��取锁并注销服务
            // 注意：Drop 不能是 async，所以使用 try_lock
            if let Ok(daemon) = self.daemon.try_lock() {
                if let Err(e) = daemon.unregister(&fullname) {
                    // 在 Drop 中只能记录警告，不能返回错误
                    warn!(
                        service_name = %fullname,
                        error = %e,
                        "Failed to unregister mDNS service during drop"
                    );
                } else {
                    debug!(service_name = %fullname, "mDNS service unregistered during drop");
                }
            } else {
                warn!(
                    service_name = %fullname,
                    "Could not acquire lock to unregister mDNS service during drop"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config_new() {
        let config = MdnsServiceConfig::new(
            "test-device".to_string(),
            "dGVzdC1oYXNo".to_string(),
            12345,
        );

        assert_eq!(config.device_id(), "test-device");
        assert_eq!(config.public_key_hash(), "dGVzdC1oYXNo");
        assert_eq!(config.port(), 12345);
        assert!(config.hostname().is_none());
    }

    #[test]
    fn test_service_config_with_hostname() {
        let config = MdnsServiceConfig::new(
            "test-device".to_string(),
            "dGVzdC1oYXNo".to_string(),
            12345,
        )
        .with_hostname("custom-host".to_string());

        assert_eq!(config.hostname(), Some("custom-host"));
    }

    #[test]
    fn test_service_config_validate_success() {
        let config = MdnsServiceConfig::new(
            "test-device".to_string(),
            "dGVzdC1oYXNo".to_string(),
            12345,
        );

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_service_config_validate_empty_device_id() {
        let config = MdnsServiceConfig::new(String::new(), "hash".to_string(), 12345);

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_id"));
    }

    #[test]
    fn test_service_config_validate_empty_pubkey_hash() {
        let config = MdnsServiceConfig::new("device".to_string(), String::new(), 12345);

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("public_key_hash"));
    }

    #[test]
    fn test_service_config_validate_zero_port() {
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 0);

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("port"));
    }

    #[test]
    fn test_service_config_validate_txt_too_long() {
        // 创建一个超长的公钥哈希 (pk=<hash> 需要 <= 255 字节, pk= 占 3 字节)
        let long_hash = "a".repeat(253); // 3 + 253 = 256 > 255
        let config = MdnsServiceConfig::new("device".to_string(), long_hash, 12345);

        let result = config.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("255 bytes"), "Error: {}", err_msg);
        assert!(err_msg.contains("'pk'"), "Error should mention 'pk': {}", err_msg);
    }

    #[test]
    fn test_service_config_validate_device_id_too_long_for_txt() {
        // 注意：device_id 也用作主机名，主机名标签限制为 63 字符
        // 所以这个测试只能验证主机名长度限制
        // TXT 记录长度限制 (255 字节) 对于 device_id 来说不太可能触发
        // 因为主机名 63 字符限制更严格
        let long_id = "d".repeat(64); // 超过主机名 63 字符限制
        let config = MdnsServiceConfig::new(long_id, "hash".to_string(), 12345);

        let result = config.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("63 characters"), "Error: {}", err_msg);
    }

    #[test]
    fn test_service_config_validate_invalid_hostname() {
        // 主机名以连字符开头
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 12345)
            .with_hostname("-invalid".to_string());
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hyphen"));

        // 主机名包含非法字符
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 12345)
            .with_hostname("invalid_host".to_string());
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid character"));

        // 主机名太长
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 12345)
            .with_hostname("a".repeat(64));
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("63 characters"));
    }

    #[test]
    fn test_service_config_validate_valid_hostname() {
        // 有效的主机名
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 12345)
            .with_hostname("my-device-123".to_string());
        assert!(config.validate().is_ok());

        // 带 .local. 后缀的有效主机名
        let config = MdnsServiceConfig::new("device".to_string(), "hash".to_string(), 12345)
            .with_hostname("my-device.local.".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_service_config_validate_device_id_as_hostname() {
        // 设备 ID 包含非法主机名字符应该失败
        let config = MdnsServiceConfig::new("device_with_underscore".to_string(), "hash".to_string(), 12345);
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid character"));
    }

    #[test]
    fn test_build_txt_properties() {
        let config = MdnsServiceConfig::new(
            "my-device".to_string(),
            "my-hash".to_string(),
            8080,
        );

        let properties = config.build_txt_properties();

        assert_eq!(properties.len(), 2);
        assert!(properties.iter().any(|(k, v)| k == TXT_DEVICE_ID && v == "my-device"));
        assert!(properties.iter().any(|(k, v)| k == TXT_PUBKEY_HASH && v == "my-hash"));
    }

    #[test]
    fn test_service_type_format() {
        assert_eq!(SERVICE_TYPE, "_nearclip._tcp.local.");
        assert!(SERVICE_TYPE.starts_with('_'));
        assert!(SERVICE_TYPE.contains("._tcp."));
        assert!(SERVICE_TYPE.ends_with(".local."));
    }

    #[test]
    fn test_txt_record_keys() {
        assert_eq!(TXT_DEVICE_ID, "id");
        assert_eq!(TXT_PUBKEY_HASH, "pk");
    }

    #[test]
    fn test_advertiser_new_success() {
        let config = MdnsServiceConfig::new(
            "test-device".to_string(),
            "dGVzdC1oYXNo".to_string(),
            12345,
        );

        let result = MdnsAdvertiser::new(config);
        assert!(result.is_ok());

        let advertiser = result.unwrap();
        assert!(!advertiser.is_advertising());
        assert!(advertiser.service_fullname().is_none());
    }

    #[test]
    fn test_advertiser_new_invalid_config() {
        let config = MdnsServiceConfig::new(String::new(), "hash".to_string(), 12345);

        let result = MdnsAdvertiser::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_advertiser_config_accessor() {
        let config = MdnsServiceConfig::new(
            "test-device".to_string(),
            "hash".to_string(),
            8080,
        );

        let advertiser = MdnsAdvertiser::new(config).unwrap();
        assert_eq!(advertiser.config().device_id(), "test-device");
        assert_eq!(advertiser.config().port(), 8080);
    }
}
