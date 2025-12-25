//! BLE receive task helper
//!
//! Provides a common implementation for BLE message receive tasks,
//! eliminating code duplication between on_ble_data_received and
//! on_ble_connection_changed.

use std::sync::Arc;
use nearclip_sync::MessageType;
use nearclip_transport::Transport;
use tokio::task::JoinHandle;

use crate::FfiNearClipCallback;

/// Spawn a BLE receive task for a transport
///
/// This task continuously receives messages from the transport and
/// dispatches them to the appropriate callback methods.
///
/// # Arguments
///
/// * `transport` - The BLE transport to receive from
/// * `callback` - The FFI callback to notify
/// * `device_id` - The device ID for logging
///
/// # Returns
///
/// A JoinHandle for the spawned task
pub fn spawn_ble_recv_task(
    transport: Arc<dyn Transport>,
    callback: Arc<dyn FfiNearClipCallback>,
    device_id: String,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        tracing::info!(device_id = %device_id, "BLE receive task started");
        loop {
            match transport.recv().await {
                Ok(message) => {
                    tracing::debug!(
                        device_id = %device_id,
                        msg_type = ?message.msg_type,
                        "BLE message received"
                    );

                    match message.msg_type {
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
                        device_id = %device_id,
                        error = %e,
                        "BLE receive error, stopping task"
                    );
                    break;
                }
            }
        }
        tracing::info!(device_id = %device_id, "BLE receive task ended");
    })
}
