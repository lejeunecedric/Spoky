//! Tauri commands for protocol operations

use tauri::{AppHandle, State};
use crate::models::{Account, Protocol};
use crate::protocol::events::{ConnectionStatus, ProtocolEvent, ConnectionEvent};
use crate::protocol::registry::ProtocolRegistry;

/// Connect a protocol account
#[tauri::command]
pub async fn connect_account(
    app: AppHandle,
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<String, String> {
    log::info!("Connecting account: {}", account_id);
    
    // In Phase 2, this will:
    // 1. Load account from database
    // 2. Create appropriate adapter factory based on protocol
    // 3. Call registry.connect_account()
    
    // For now, emit connecting event to demonstrate event flow
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: account_id.clone(),
            protocol: Protocol::Discord, // Stub - will be dynamic
            status: ConnectionStatus::Connecting,
            message: Some("Initiating connection...".to_string()),
        }
    )).map_err(|e| format!("Failed to emit event: {}", e))?;
    
    // Simulate connection delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Emit connected event
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: account_id.clone(),
            protocol: Protocol::Discord,
            status: ConnectionStatus::Connected,
            message: Some("Successfully connected".to_string()),
        }
    )).map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(format!("Account {} connected", account_id))
}

/// Disconnect a protocol account
#[tauri::command]
pub async fn disconnect_account(
    app: AppHandle,
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<String, String> {
    log::info!("Disconnecting account: {}", account_id);
    
    // In Phase 2, this will call registry.disconnect_account()
    
    // Emit disconnecting event
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: account_id.clone(),
            protocol: Protocol::Discord,
            status: ConnectionStatus::Disconnected,
            message: Some("Disconnecting...".to_string()),
        }
    )).map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(format!("Account {} disconnected", account_id))
}

/// Get connection status for an account
#[tauri::command]
pub async fn get_connection_status(
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<ConnectionStatus, String> {
    match registry.connection_status(&account_id).await {
        Some(status) => {
            log::debug!("Connection status for {}: {:?}", account_id, status);
            Ok(status)
        }
        None => {
            log::debug!("Account {} not found, returning disconnected", account_id);
            Ok(ConnectionStatus::Disconnected)
        }
    }
}

/// Get all connected account IDs
#[tauri::command]
pub async fn get_connected_accounts(
    registry: State<'_, ProtocolRegistry>,
) -> Result<Vec<String>, String> {
    let accounts = registry.connected_accounts().await;
    log::debug!("Connected accounts: {:?}", accounts);
    Ok(accounts)
}

/// Test the protocol event system
#[tauri::command]
pub async fn test_protocol_events(
    app: AppHandle,
) -> Result<String, String> {
    log::info!("Testing protocol event emission");
    
    // Emit a test connection event
    app.emit("protocol:event", ProtocolEvent::ConnectionChanged(
        ConnectionEvent {
            account_id: "test-account".to_string(),
            protocol: Protocol::Discord,
            status: ConnectionStatus::Connected,
            message: Some("Test event from protocol system".to_string()),
        }
    )).map_err(|e| format!("Failed to emit test event: {}", e))?;
    
    Ok("Test event emitted. Check frontend console.".to_string())
}
