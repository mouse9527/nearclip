//! NearClip Core Library
//!
//! 跨平台剪贴板同步的核心功能库
//! 提供加密、BLE 通信、设备管理等核心功能

pub mod crypto;
pub mod ble;
pub mod ffi;
pub mod message;
pub mod device;
pub mod sync;
pub mod error;

// 重新导出主要的公共接口
pub use crypto::CryptoService;
pub use ble::BLEManager;
pub use device::Device;
pub use sync::{SyncManager, SyncData};
pub use error::{NearClipError, Result};

use std::sync::Arc;
use tokio::sync::RwLock;

/// NearClip 核心实例
///
/// 这是 NearClip 的主要入口点，管理所有核心组件
pub struct NearClipCore {
    crypto_service: Arc<CryptoService>,
    ble_manager: Arc<BLEManager>,
    sync_manager: Arc<RwLock<SyncManager>>,
}

impl NearClipCore {
    /// 创建新的 NearClip 核心实例
    pub async fn new() -> Result<Self> {
        // 初始化加密服务
        let crypto_service = Arc::new(CryptoService::new()?);

        // 初始化 BLE 管理器
        let ble_manager = Arc::new(BLEManager::new(
            "6e400001-b5a3-f393-e0a9-e50e24dcca9e".to_string() // NearClip Service UUID
        ));

        // 初始化同步管理器
        let sync_manager = Arc::new(RwLock::new(SyncManager::new(
            crypto_service.clone(),
            ble_manager.clone()
        )));

        Ok(NearClipCore {
            crypto_service,
            ble_manager,
            sync_manager,
        })
    }

    /// 获取加密服务实例
    pub fn crypto_service(&self) -> &Arc<CryptoService> {
        &self.crypto_service
    }

    /// 获取 BLE 管理器实例
    pub fn ble_manager(&self) -> &Arc<BLEManager> {
        &self.ble_manager
    }

    /// 获取同步管理器实例
    pub fn sync_manager(&self) -> &Arc<RwLock<SyncManager>> {
        &self.sync_manager
    }

    /// 启动 NearClip 服务
    pub async fn start(&self) -> Result<()> {
        log::info!("Starting NearClip core services");

        // 启动设备发现
        self.ble_manager.start_advertising(&[]).await?;

        // 启动同步管理器
        let mut sync_manager = self.sync_manager.write().await;
        sync_manager.start().await?;

        log::info!("NearClip core services started successfully");
        Ok(())
    }

    /// 停止 NearClip 服务
    pub async fn stop(&self) -> Result<()> {
        log::info!("Stopping NearClip core services");

        // 停止 BLE 广播
        self.ble_manager.stop_advertising().await?;

        // 停止同步管理器
        let mut sync_manager = self.sync_manager.write().await;
        sync_manager.stop().await?;

        log::info!("NearClip core services stopped");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nearclip_core_creation() {
        let core = NearClipCore::new().await;
        assert!(core.is_ok());
    }

    #[tokio::test]
    async fn test_service_start_stop() {
        let core = NearClipCore::new().await.unwrap();

        // 启动服务
        assert!(core.start().await.is_ok());

        // 停止服务
        assert!(core.stop().await.is_ok());
    }
}