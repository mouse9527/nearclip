//! NearClip Core Library
//!
//! This library provides the core functionality for NearClip cross-platform
//! clipboard synchronization. It includes device discovery, secure communication,
//! and data synchronization capabilities.

#![warn(missing_docs)]
#![warn(clippy::all)]

// Include generated protobuf code
pub mod device_discovery;
pub mod data_sync;
pub mod error_handling;

// Core modules (will be implemented in future stories)
// pub mod ble;
// pub mod crypto;
// pub mod device_manager;
// pub mod sync_engine;
pub mod utils;
pub mod error;

// Re-export commonly used types
pub use device_discovery::*;
pub use data_sync::*;
pub use error_handling::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_initialization() {
        // Basic test to ensure library compiles and can be imported
        assert!(true);
    }
}