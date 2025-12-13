//! NearClip Core Module
//!
//! Core coordination layer providing the main API for platform clients.
//! Orchestrates crypto, network, BLE, and sync modules.

#![allow(dead_code)]
#![allow(unused_variables)]

pub mod error;

// Re-export error types for convenience
pub use error::{NearClipError, Result};

// Future modules:
// mod manager;   // NearClipManager main class
// mod device;    // Device management
// mod config;    // Configuration management

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_reexport() {
        let err = NearClipError::Network("test".to_string());
        assert!(err.to_string().contains("Network"));
    }

    #[test]
    fn test_result_reexport() {
        fn test_fn() -> Result<i32> {
            Ok(42)
        }
        assert!(test_fn().is_ok());
    }
}
