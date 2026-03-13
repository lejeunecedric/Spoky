// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod db;
mod models;
mod protocol;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Tauri IPC is working.", name)
}

#[tauri::command]
async fn test_db_connection(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Try to get database from app state
    match db::get_db(&app_handle) {
        Ok(_) => Ok("✅ Database connection successful!".to_string()),
        Err(e) => Err(format!("❌ Database error: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Create event channel for protocol events
            let (event_tx, mut event_rx) =
                tokio::sync::mpsc::channel::<protocol::events::ProtocolEvent>(100);

            // Create protocol registry
            let registry = protocol::registry::ProtocolRegistry::new(event_tx);
            app.manage(registry);

            // Spawn event forwarder task
            // Receives from channel and emits to frontend via Tauri
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = event_rx.recv().await {
                    if let Err(e) = app_handle_clone.emit("protocol:event", &event) {
                        log::error!("Failed to emit protocol event: {}", e);
                    }
                }
            });

            // Initialize database on app startup
            tauri::async_runtime::spawn(async move {
                match db::init_db(&app_handle).await {
                    Ok(database) => {
                        // Store database in app state for later use
                        app_handle.manage(database);
                        log::info!("Database initialized and stored in app state");
                    }
                    Err(e) => {
                        log::error!("Failed to initialize database: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            test_db_connection,
            commands::protocol::connect_account,
            commands::protocol::disconnect_account,
            commands::protocol::get_connection_status,
            commands::protocol::get_connected_accounts,
            commands::protocol::test_protocol_events,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
