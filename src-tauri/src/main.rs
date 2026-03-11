// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Emitter};

mod commands;
mod models;
mod protocol;

use protocol::registry::ProtocolRegistry;
use protocol::events::ProtocolEvent;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Tauri IPC is working.", name)
}

#[tauri::command]
async fn test_db_connection(
    db: tauri::State<'tauri_plugin_sql::DbInstance>,
) -> Result<String, String> {
    // Test database connection - from Plan 01-02
    db.0.execute("SELECT 1", vec![])
        .await
        .map_err(|e| e.to_string())?;
    Ok("Database connection successful".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            // Create event channel
            let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<ProtocolEvent>(100);
            
            // Create protocol registry
            let registry = ProtocolRegistry::new(event_tx);
            app.manage(registry);

            // Spawn event forwarder task
            // Bridges the mpsc channel to Tauri's event system
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = event_rx.recv().await {
                    let _ = app_handle.emit("protocol:event", event);
                }
            });

            log::info!("Spoky application started successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            test_db_connection,
            commands::protocol::connect_account,
            commands::protocol::disconnect_account,
            commands::protocol::get_connection_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}