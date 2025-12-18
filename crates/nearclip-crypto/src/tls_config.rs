//! TLS 1.3 配置模块
//!
//! 提供设备间安全通信所需的 TLS 服务端和客户端配置。
//! 使用自签名证书和 TOFU (Trust On First Use) 信任模型。
//!
//! # Example
//!
//! ```
//! use nearclip_crypto::{TlsCertificate, TlsServerConfig, TlsClientConfig};
//!
//! // 生成服务端证书
//! let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//!
//! // 创建服务端配置
//! let server_config = TlsServerConfig::new(&cert).unwrap();
//!
//! // 创建客户端配置（信任服务端证书）
//! let client_config = TlsClientConfig::new(cert.cert_der()).unwrap();
//! ```

use crate::CryptoError;
use rcgen::{CertificateParams, DnType, KeyPair, PKCS_ECDSA_P256_SHA256};
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::ring::default_provider,
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime},
    ClientConfig, DigitallySignedStruct, RootCertStore, ServerConfig, SignatureScheme,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, instrument, warn};
use zeroize::Zeroize;

/// 自签名 TLS 证书
///
/// 封装 rcgen 生成的 X.509 证书，用于配置 TLS 连接。
/// 使用 ECDSA P-256 签名算法，确保与 ECDH 密钥兼容。
///
/// # Example
///
/// ```
/// use nearclip_crypto::TlsCertificate;
///
/// // 生成支持 localhost 的证书
/// let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
///
/// // 获取证书字节用于传输或存储
/// let cert_der = cert.cert_der();
/// assert!(!cert_der.is_empty());
/// ```
#[derive(Clone)]
pub struct TlsCertificate {
    cert_der: Vec<u8>,
    key_der: Vec<u8>,
}

impl TlsCertificate {
    /// 生成新的自签名 TLS 证书
    ///
    /// 使用 ECDSA P-256 签名算法生成自签名证书。
    /// 证书有效期为 365 天。
    ///
    /// # Arguments
    ///
    /// * `subject_alt_names` - 证书的 Subject Alternative Names 列表
    ///   支持域名（如 "localhost"）和 IP 地址（如 "192.168.1.1"）
    ///
    /// # Returns
    ///
    /// 生成的 TLS 证书，或错误如果生成失败
    ///
    /// # Example
    ///
    /// ```
    /// use nearclip_crypto::TlsCertificate;
    ///
    /// let cert = TlsCertificate::generate(&[
    ///     "localhost".to_string(),
    ///     "127.0.0.1".to_string(),
    /// ]).unwrap();
    /// ```
    #[instrument(skip(subject_alt_names), fields(san_count = subject_alt_names.len()))]
    pub fn generate(subject_alt_names: &[String]) -> Result<Self, CryptoError> {
        // 验证 SAN 列表不为空
        if subject_alt_names.is_empty() {
            return Err(CryptoError::CertificateGeneration(
                "Subject Alternative Names cannot be empty".to_string(),
            ));
        }

        // 生成 ECDSA P-256 密钥对
        let key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256)
            .map_err(|e| CryptoError::CertificateGeneration(e.to_string()))?;

        // 配置证书参数
        let mut params = CertificateParams::new(subject_alt_names.to_vec())
            .map_err(|e| CryptoError::CertificateGeneration(e.to_string()))?;

        // 设置证书有效期为 365 天
        params.not_before = time::OffsetDateTime::now_utc();
        params.not_after = time::OffsetDateTime::now_utc() + Duration::from_secs(365 * 24 * 60 * 60);

        // 设置 Distinguished Name
        params
            .distinguished_name
            .push(DnType::CommonName, "NearClip Device");
        params
            .distinguished_name
            .push(DnType::OrganizationName, "NearClip");

        // 生成自签名证书
        let cert = params
            .self_signed(&key_pair)
            .map_err(|e| CryptoError::CertificateGeneration(e.to_string()))?;

        debug!("Generated self-signed TLS certificate with ECDSA P-256");

        Ok(Self {
            cert_der: cert.der().to_vec(),
            key_der: key_pair.serialize_der(),
        })
    }

    /// 获取证书 DER 编码字节
    ///
    /// 返回 X.509 证书的 DER 编码，用于传输给对端或存储。
    pub fn cert_der(&self) -> &[u8] {
        &self.cert_der
    }

    /// 获取私钥 DER 编码字节
    ///
    /// 返回私钥的 PKCS#8 DER 编码。
    ///
    /// **安全警告：** 私钥必须安全存储，不可记录到日志或明文传输。
    pub fn key_der(&self) -> &[u8] {
        &self.key_der
    }

    /// 获取证书 PEM 编码字符串
    ///
    /// 返回 X.509 证书的 PEM 编码，便于人类阅读和调试。
    pub fn cert_pem(&self) -> String {
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&self.cert_der);
        format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----",
            b64.chars()
                .collect::<Vec<_>>()
                .chunks(64)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl std::fmt::Debug for TlsCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsCertificate")
            .field("cert_der_len", &self.cert_der.len())
            .field("key_der_len", &self.key_der.len())
            .finish_non_exhaustive()
    }
}

impl Drop for TlsCertificate {
    fn drop(&mut self) {
        // 安全清零私钥材料，防止内存残留
        self.key_der.zeroize();
    }
}

/// TLS 服务端配置
///
/// 封装 rustls ServerConfig，用于创建 TLS 服务端。
/// 强制使用 TLS 1.3 协议。
///
/// # Example
///
/// ```
/// use nearclip_crypto::{TlsCertificate, TlsServerConfig};
///
/// let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
/// let server_config = TlsServerConfig::new(&cert).unwrap();
///
/// // 使用 config() 获取 Arc<ServerConfig> 供 TCP 监听使用
/// let config = server_config.config();
/// ```
pub struct TlsServerConfig {
    config: Arc<ServerConfig>,
}

impl TlsServerConfig {
    /// 从证书创建服务端 TLS 配置
    ///
    /// 配置强制使用 TLS 1.3，不启用客户端证书验证。
    /// 客户端身份验证由应用层通过 TOFU 模型处理。
    ///
    /// # Arguments
    ///
    /// * `cert` - 服务端证书
    ///
    /// # Returns
    ///
    /// TLS 服务端配置，或错误如果配置失败
    #[instrument(skip(cert))]
    pub fn new(cert: &TlsCertificate) -> Result<Self, CryptoError> {
        let cert_der = CertificateDer::from(cert.cert_der().to_vec());
        let key_der = PrivateKeyDer::try_from(cert.key_der().to_vec())
            .map_err(|e| CryptoError::TlsConfiguration(format!("Invalid private key: {}", e)))?;

        // 使用 ring 加密提供者，强制 TLS 1.3
        let config = ServerConfig::builder_with_provider(Arc::new(default_provider()))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| CryptoError::TlsConfiguration(e.to_string()))?
            .with_no_client_auth()
            .with_single_cert(vec![cert_der], key_der)
            .map_err(|e| CryptoError::TlsConfiguration(e.to_string()))?;

        debug!("Created TLS server configuration with TLS 1.3");

        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// 获取 rustls ServerConfig
    ///
    /// 返回 `Arc<ServerConfig>` 供 TCP 服务端使用。
    pub fn config(&self) -> Arc<ServerConfig> {
        Arc::clone(&self.config)
    }
}

impl std::fmt::Debug for TlsServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsServerConfig")
            .finish_non_exhaustive()
    }
}

/// TLS 客户端配置
///
/// 封装 rustls ClientConfig，用于创建 TLS 客户端连接。
/// 使用 TOFU (Trust On First Use) 信任模型。
///
/// # Example
///
/// ```
/// use nearclip_crypto::{TlsCertificate, TlsClientConfig};
///
/// // 服务端证书（通常通过配对流程获取）
/// let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
///
/// // 创建信任该服务端证书的客户端配置
/// let client_config = TlsClientConfig::new(server_cert.cert_der()).unwrap();
/// ```
pub struct TlsClientConfig {
    config: Arc<ClientConfig>,
}

impl TlsClientConfig {
    /// 创建信任指定证书的客户端 TLS 配置
    ///
    /// 使用 TOFU 信任模型，将指定的服务端证书添加到信任根。
    /// 只有提供的证书签发的服务端才能建立连接。
    ///
    /// # Arguments
    ///
    /// * `trusted_cert_der` - 信任的服务端证书 DER 编码字节
    ///
    /// # Returns
    ///
    /// TLS 客户端配置，或错误如果配置失败
    #[instrument(skip(trusted_cert_der), fields(cert_len = trusted_cert_der.len()))]
    pub fn new(trusted_cert_der: &[u8]) -> Result<Self, CryptoError> {
        let mut root_store = RootCertStore::empty();
        let cert = CertificateDer::from(trusted_cert_der.to_vec());

        root_store
            .add(cert)
            .map_err(|e| CryptoError::TlsConfiguration(format!("Invalid certificate: {}", e)))?;

        let config = ClientConfig::builder_with_provider(Arc::new(default_provider()))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| CryptoError::TlsConfiguration(e.to_string()))?
            .with_root_certificates(root_store)
            .with_no_client_auth();

        debug!("Created TLS client configuration with TOFU trust model");

        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// 获取 rustls ClientConfig
    ///
    /// 返回 `Arc<ClientConfig>` 供 TCP 客户端使用。
    pub fn config(&self) -> Arc<ClientConfig> {
        Arc::clone(&self.config)
    }

    /// 创建不验证证书的客户端 TLS 配置（仅用于测试）
    ///
    /// **警告**: 此配置不验证服务端证书，存在中间人攻击风险。
    /// 仅应用于开发和测试环境，不应在生产环境中使用。
    ///
    /// # Returns
    ///
    /// TLS 客户端配置，或错误如果配置失败
    #[instrument]
    pub fn new_insecure() -> Result<Self, CryptoError> {
        warn!("Creating insecure TLS client config - DO NOT USE IN PRODUCTION");

        let config = ClientConfig::builder_with_provider(Arc::new(default_provider()))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| CryptoError::TlsConfiguration(e.to_string()))?
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(InsecureServerCertVerifier))
            .with_no_client_auth();

        debug!("Created insecure TLS client configuration");

        Ok(Self {
            config: Arc::new(config),
        })
    }
}

/// 不验证服务端证书的验证器（仅用于测试）
///
/// **警告**: 此验证器接受所有服务端证书，存在严重安全风险。
#[derive(Debug)]
struct InsecureServerCertVerifier;

impl ServerCertVerifier for InsecureServerCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // 接受所有证书 - 仅用于测试
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ED25519,
        ]
    }
}

impl std::fmt::Debug for TlsClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsClientConfig")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_certificate() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]);
        assert!(cert.is_ok());

        let cert = cert.unwrap();
        assert!(!cert.cert_der().is_empty());
        assert!(!cert.key_der().is_empty());
    }

    #[test]
    fn test_generate_certificate_multiple_sans() {
        let cert = TlsCertificate::generate(&[
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "nearclip.local".to_string(),
        ]);
        assert!(cert.is_ok());
    }

    #[test]
    fn test_generate_certificate_empty_sans_fails() {
        let result = TlsCertificate::generate(&[]);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::CertificateGeneration(_))));
    }

    #[test]
    fn test_certificate_pem_format() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let pem = cert.cert_pem();

        assert!(pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(pem.ends_with("-----END CERTIFICATE-----"));
    }

    #[test]
    fn test_certificate_clone() {
        let cert1 = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let cert2 = cert1.clone();

        assert_eq!(cert1.cert_der(), cert2.cert_der());
        assert_eq!(cert1.key_der(), cert2.key_der());
    }

    #[test]
    fn test_certificate_debug_no_key_leak() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let debug_str = format!("{:?}", cert);

        // Debug 输出不应包含实际密钥内容
        assert!(!debug_str.contains(&hex::encode(cert.key_der())));
        assert!(debug_str.contains("cert_der_len"));
    }

    #[test]
    fn test_server_config_creation() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let server_config = TlsServerConfig::new(&cert);
        assert!(server_config.is_ok());
    }

    #[test]
    fn test_server_config_returns_arc() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let server_config = TlsServerConfig::new(&cert).unwrap();

        let config1 = server_config.config();
        let config2 = server_config.config();

        // 应该返回相同的 Arc
        assert!(Arc::ptr_eq(&config1, &config2));
    }

    #[test]
    fn test_client_config_creation() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let client_config = TlsClientConfig::new(cert.cert_der());
        assert!(client_config.is_ok());
    }

    #[test]
    fn test_client_config_invalid_cert() {
        let invalid_cert = vec![0u8; 100]; // 无效证书
        let result = TlsClientConfig::new(&invalid_cert);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::TlsConfiguration(_))));
    }

    #[test]
    fn test_client_config_returns_arc() {
        let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
        let client_config = TlsClientConfig::new(cert.cert_der()).unwrap();

        let config1 = client_config.config();
        let config2 = client_config.config();

        assert!(Arc::ptr_eq(&config1, &config2));
    }

    #[test]
    fn test_server_and_client_configs_compatible() {
        // 生成服务端证书
        let server_cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();

        // 创建服务端配置
        let server_config = TlsServerConfig::new(&server_cert);
        assert!(server_config.is_ok());

        // 使用服务端证书创建客户端配置
        let client_config = TlsClientConfig::new(server_cert.cert_der());
        assert!(client_config.is_ok());
    }

    #[test]
    fn test_tls_certificate_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TlsCertificate>();
    }

    #[test]
    fn test_tls_server_config_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TlsServerConfig>();
    }

    #[test]
    fn test_tls_client_config_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TlsClientConfig>();
    }
}
