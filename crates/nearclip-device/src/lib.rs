//! NearClip Device Management
//!
//! This crate provides device discovery, pairing, storage, and state management.

mod error;
mod models;
mod store;
mod manager;
mod pairing;

pub use error::DeviceError;
pub use models::{PairedDevice, DiscoveredDevice, DevicePlatform, DiscoveryChannel};
pub use store::DeviceStore;
pub use manager::DeviceManager;
pub use pairing::{PairingManager, PairingState, PairingError};
