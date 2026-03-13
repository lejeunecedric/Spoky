//! Database module for Spoky
//!
//! Handles SQLite database connections, schema definitions, and migrations.

pub mod migrations;
pub mod schema;

use tauri::Manager;
use tauri_plugin_sql::Connection;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Database state managed by Tauri
#[derive(Clone)]
pub struct Database {
    pub connection: Arc<RwLock<Connection>>,
}

impl Database {
    /// Create a new database wrapper
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(RwLock::new(connection)),
        }
    }

    /// Get the database connection
    pub async fn conn(&self,
    ) -> tokio::sync::RwLockReadGuard<'_, Connection> {
        self.connection.read().await
    }

    /// Get a mutable database connection
    pub async fn conn_mut(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, Connection> {
        self.connection.write().await
    }
}

/// Initialize the database for the application
pub async fn init_db(app: &tauri::AppHandle) -> Result<Database, String> {
    // Get app data directory for storing the database
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {:?}", e))?;

    // Ensure the directory exists
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    // Build database path
    let db_path = app_data_dir.join("spoky.db");
    let db_url = format!(
        "sqlite:{}",
        db_path.to_str().ok_or("Invalid database path")?
    );

    log::info!("Opening database at: {}", db_path.display());

    // Open database connection using Tauri SQL plugin
    let connection = Connection::open(&db_url)
        .await
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Run migrations
    migrations::run_migrations(&connection)
        .await
        .map_err(|e| format!("Migration failed: {}", e))?;

    log::info!("Database initialized successfully");

    Ok(Database::new(connection))
}

/// Get database state from app handle
pub fn get_db(app: &tauri::AppHandle) -> Result<Database, String> {
    app.state::<Database>()
        .inner()
        .clone()
        .ok_or_else(|| "Database not initialized".to_string())
}
