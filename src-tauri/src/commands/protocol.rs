use tauri::{AppHandle, State, Emitter};
use crate::protocol::registry::ProtocolRegistry;
use crate::protocol::events::{ProtocolEvent, ConnectionEvent, ConnectionStatus};
use crate::models::Protocol;

/// Connect a protocol account
#[tauri::command]
pub async fn connect_account(
    app: AppHandle,
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<String, String> {
    // In Phase 2, this will look up account from database
    // For now, return stub success
    log::info!("Connecting account: {}", account_id);
    
    // Emit event to frontend
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: account_id.clone(),
            protocol: Protocol::Discord, // Stub
            status: ConnectionStatus::Connecting,
            message: None,
        }
    )).map_err(|e| e.to_string())?;

    Ok(format!("Connecting account {}", account_id))
}

/// Disconnect a protocol account
#[tauri::command]
pub async fn disconnect_account(
    app: AppHandle,
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<String, String> {
    log::info!("Disconnecting account: {}", account_id);
    
    // Emit disconnect event
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: account_id.clone(),
            protocol: Protocol::Discord,
            status: ConnectionStatus::Disconnected,
            message: None,
        }
    )).map_err(|e| e.to_string())?;

    Ok(format!("Disconnected account {}", account_id))
}

/// Get connection status for an account
#[tauri::command]
pub async fn get_connection_status(
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<ConnectionStatus, String> {
    match registry.connection_status(&account_id).await {
        Some(status) => Ok(status),
        None => Ok(ConnectionStatus::Disconnected),
    }
}