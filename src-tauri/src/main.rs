// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod db;
mod models;

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
            // Initialize database on app startup
            let app_handle = app.handle().clone();

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
        .invoke_handler(tauri::generate_handler![greet, test_db_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
