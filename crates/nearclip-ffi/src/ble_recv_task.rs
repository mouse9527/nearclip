//! BLE receive task helper
//!
//! Provides a common implementation for BLE message receive tasks,
//! eliminating code duplication between on_ble_data_received and
//! on_ble_connection_changed.

use std::sync::Arc;
use nearclip_sync::{MessageType, PairingPayload};
use nearclip_transport::Transport;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::{FfiNearClipCallback, FfiDeviceInfo};
use nearclip_core::{DevicePlatform, DeviceStatus};
use nearclip_ble::BleController;

/// Spawn a BLE receive task with optional BleController for device ID remapping
///
/// This task continuously receives messages from the transport and
/// dispatches them to the appropriate callback methods.
///
/// When a PairingRequest is received, the real device_id from the message is used
/// to update the BleController mappings. This is important in peripheral mode
/// where the initial device_id is just the central's MAC address.
///
/// # Arguments
///
/// * `transport` - The BLE transport to receive from
/// * `callback` - The FFI callback to notify
/// * `device_id` - The initial device ID (may be a MAC address in peripheral mode)
/// * `ble_controller` - Optional BleController for updating device mappings
///
/// # Returns
///
/// A JoinHandle for the spawned task
pub fn spawn_ble_recv_task_with_controller(
    transport: Arc<dyn Transport>,
    callback: Arc<dyn FfiNearClipCallback>,
    device_id: String,
    ble_controller: Option<Arc<RwLock<Option<Arc<BleController>>>>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        tracing::info!(device_id = %device_id, "BLE receive task started");
        let mut current_device_id = device_id.clone();

        loop {
            match transport.recv().await {
                Ok(message) => {
                    tracing::debug!(
                        device_id = %current_device_id,
                        msg_type = ?message.msg_type,
                        "BLE message received"
                    );

                    match message.msg_type {
                        MessageType::PairingRequest | MessageType::PairingResponse => {
                            tracing::info!(
                                from = %message.device_id,
                                msg_type = ?message.msg_type,
                                "BLE pairing message received"
                            );

                            // Parse PairingPayload to get real device info
                            match PairingPayload::deserialize(&message.payload) {
                                Ok(pairing_info) => {
                                    let real_device_id = pairing_info.device_id.clone();
                                    let device_name = pairing_info.device_name.clone();
                                    let platform = match pairing_info.platform {
                                        nearclip_sync::ProtocolPlatform::MacOS => DevicePlatform::MacOS,
                                        nearclip_sync::ProtocolPlatform::Android => DevicePlatform::Android,
                                        nearclip_sync::ProtocolPlatform::Unknown => DevicePlatform::Unknown,
                                    };

                                    tracing::info!(
                                        old_device_id = %current_device_id,
                                        real_device_id = %real_device_id,
                                        device_name = %device_name,
                                        platform = ?platform,
                                        "Received pairing info, updating device ID mapping"
                                    );

                                    // Update BleController mapping if the device_id is different
                                    // (happens in peripheral mode where initial device_id is MAC address)
                                    if current_device_id != real_device_id {
                                        if let Some(ref controller_lock) = ble_controller {
                                            let controller = controller_lock.read().await;
                                            if let Some(ref controller) = *controller {
                                                // The current_device_id is being used as peripheral_uuid
                                                // We need to add the real device_id mapping
                                                controller.register_device_mapping(
                                                    &real_device_id,
                                                    &current_device_id,
                                                ).await;

                                                tracing::info!(
                                                    peripheral_uuid = %current_device_id,
                                                    device_id = %real_device_id,
                                                    "Updated BleController device ID mapping"
                                                );
                                            }
                                        }

                                        // Update the device info in the callback with real info
                                        let device_info = FfiDeviceInfo {
                                            id: real_device_id.clone(),
                                            name: device_name,
                                            platform,
                                            status: DeviceStatus::Connected,
                                        };
                                        callback.on_device_connected(device_info);

                                        current_device_id = real_device_id;
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!(
                                        error = %e,
                                        "Failed to deserialize PairingPayload"
                                    );
                                }
                            }
                        }
                        MessageType::ClipboardSync => {
                            tracing::info!(
                                from = %message.device_id,
                                size = message.payload.len(),
                                "BLE clipboard received"
                            );
                            callback.on_clipboard_received(
                                message.payload.clone(),
                                message.device_id.clone(),
                            );
                        }
                        MessageType::Unpair => {
                            tracing::info!(
                                from = %message.device_id,
                                "BLE unpair notification received"
                            );
                            callback.on_device_unpaired(message.device_id.clone());
                            break;
                        }
                        _ => {
                            tracing::debug!(
                                msg_type = ?message.msg_type,
                                "Unhandled BLE message type"
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        device_id = %current_device_id,
                        error = %e,
                        "BLE receive error, stopping task"
                    );
                    break;
                }
            }
        }
        tracing::info!(device_id = %current_device_id, "BLE receive task ended");
    })
}
