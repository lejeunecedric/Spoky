use tauri_plugin_sql::Connection;
use crate::db::schema::{ALL_TABLES, ALL_INDEXES};

/// Represents a single database migration
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

/// All migrations in order of application
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial_schema",
        sql: include_str!("schema_v1.sql"),
    },
];

/// Run all pending migrations on the database connection
pub async fn run_migrations(db: &Connection) -> Result<(), String> {
    // Create migrations tracking table if it doesn't exist
    db.execute(
        r#"
        CREATE TABLE IF NOT EXISTS __migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at INTEGER NOT NULL
        )
        "#,
        Vec::new(),
    )
    .await
    .map_err(|e| format!("Failed to create migrations table: {}", e))?;

    // Get current schema version
    let current_version: i64 = db
        .query(
            "SELECT COALESCE(MAX(version), 0) as version FROM __migrations",
            Vec::new(),
        )
        .await
        .map_err(|e| format!("Failed to query migrations: {}", e))?
        .try_into()
        .map_err(|e| format!("Failed to parse version: {:?}", e))?;

    // Apply pending migrations
    for migration in MIGRATIONS {
        if migration.version > current_version {
            log::info!(
                "Applying migration {}: {}",
                migration.version,
                migration.name
            );

            // Execute migration SQL
            db.execute(migration.sql, Vec::new())
                .await
                .map_err(|e| {
                    format!(
                        "Migration {} failed: {}",
                        migration.version, e
                    )
                })?;

            // Record migration as applied
            let now = chrono::Utc::now().timestamp_millis();
            db.execute(
                "INSERT INTO __migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
                vec![
                    tauri_plugin_sql::Value::Integer(migration.version),
                    tauri_plugin_sql::Value::Text(migration.name.to_string()),
                    tauri_plugin_sql::Value::Integer(now),
                ],
            )
            .await
            .map_err(|e| {
                format!(
                    "Failed to record migration {}: {}",
                    migration.version, e
                )
            })?;

            log::info!("Migration {} applied successfully", migration.version);
        }
    }

    Ok(())
}

/// Initialize database with full schema (for fresh installs)
pub async fn init_database(db: &Connection) -> Result<(), String> {
    log::info!("Initializing database schema...");

    // Create all tables
    for table_sql in ALL_TABLES {
        db.execute(table_sql, Vec::new())
            .await
            .map_err(|e| format!("Failed to create table: {}", e))?;
    }

    // Create all indexes
    for index_sql in ALL_INDEXES {
        db.execute(index_sql, Vec::new())
            .await
            .map_err(|e| format!("Failed to create index: {}", e))?;
    }

    log::info!("Database schema initialized successfully");
    Ok(())
}

/// Get current schema version
pub async fn get_schema_version(db: &Connection) -> Result<i64, String> {
    // Check if migrations table exists
    let table_exists: Result<Vec<tauri_plugin_sql::Row>, _> = db
        .query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='__migrations'",
            Vec::new(),
        )
        .await;

    match table_exists {
        Ok(rows) if !rows.is_empty() => {
            // Table exists, get version
            db.query(
                "SELECT COALESCE(MAX(version), 0) as version FROM __migrations",
                Vec::new(),
            )
            .await
            .map_err(|e| format!("Failed to query version: {}", e))?
            .try_into()
            .map_err(|e| format!("Failed to parse version: {:?}", e))
        }
        _ => Ok(0), // No migrations table = version 0
    }
}
