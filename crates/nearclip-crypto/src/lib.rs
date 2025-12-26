//! NearClip Crypto Module
//!
//! Cryptographic primitives for secure device pairing and communication.
//! Includes ECDH key exchange, TLS 1.3 configuration, key management,
//! and paired device persistence.
//!
//! # ECDH Key Exchange
//!
//! ```
//! use nearclip_crypto::EcdhKeyPair;
//!
//! // Device A generates a keypair
//! let device_a = EcdhKeyPair::generate();
//!
//! // Device B generates a keypair
//! let device_b = EcdhKeyPair::generate();
//!
//! // Exchange public keys and compute shared secret
//! let shared_a = device_a.compute_shared_secret(&device_b.public_key_bytes()).unwrap();
//! let shared_b = device_b.compute_shared_secret(&device_a.public_key_bytes()).unwrap();
//!
//! // Both devices now have the same shared secret
//! assert_eq!(shared_a, shared_b);
//! ```
//!
//! # TLS 1.3 Configuration
//!
//! ```
//! use nearclip_crypto::{TlsCertificate, TlsServerConfig, TlsClientConfig};
//!
//! // Generate server certificate
//! let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
//!
//! // Create server config
//! let server_config = TlsServerConfig::new(&cert).unwrap();
//!
//! // Create client config (trusting server cert)
//! let client_config = TlsClientConfig::new(cert.cert_der()).unwrap();
//! ```

pub mod cipher;
pub mod device_store;
pub mod keypair;
pub mod pairing;
pub mod qrcode_parser;
pub mod tls_config;

// Re-export main types for convenience
pub use cipher::{Aes256Gcm, CipherError};
pub use device_store::{DeviceStore, FileDeviceStore, FileDeviceStoreConfig};
pub use keypair::{CryptoError, EcdhKeyPair};
pub use pairing::{
    ConnectionInfo, PairedDevice, PairingData, PairingSession, QrCodeConfig,
    QrCodeErrorCorrection, QrCodeGenerator, PAIRING_DATA_VERSION,
};
pub use qrcode_parser::QrCodeParser;
pub use tls_config::{TlsCertificate, TlsClientConfig, TlsServerConfig};

// Future modules:
// - KeychainDeviceStore (macOS Keychain integration)
// - KeystoreDeviceStore (Android Keystore integration)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_reexport() {
        // Verify EcdhKeyPair can be used from crate root
        let keypair = EcdhKeyPair::generate();
        assert_eq!(keypair.private_key_bytes().len(), 32);
    }

    #[test]
    fn test_crypto_error_reexport() {
        // Verify CryptoError can be used from crate root
        let err = CryptoError::InvalidPrivateKey("test".to_string());
        assert!(err.to_string().contains("Invalid private key"));
    }
}
