//! NearClip Protocol Definitions
//!
//! This crate defines the message formats and protocols used for
//! device pairing and data synchronization.

pub mod pairing;

// Re-exports
pub use pairing::{
    PairingMessage, PairingRequest, PairingResponse,
    PairingConfirm, PairingRejected,
};
