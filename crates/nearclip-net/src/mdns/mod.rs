//! mDNS 模块
//!
//! 提供局域网设备发现功能，包括服务广播和发现。

mod advertise;
mod discovery;

pub use advertise::{
    MdnsAdvertiser, MdnsServiceConfig, SERVICE_TYPE, TXT_DEVICE_ID, TXT_PUBKEY_HASH,
};
pub use discovery::{DiscoveredDevice, DiscoveryEvent, MdnsDiscovery};
