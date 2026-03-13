//! Account management commands
//!
//! Handles CRUD operations for accounts and connection management.

use tauri::{AppHandle, State};
use crate::db::Database;
use crate::models::{Account, NewAccount, Protocol};
use crate::protocol::discord::{DiscordAdapter, DiscordAdapterFactory};
use crate::protocol::discord::auth::{encrypt_token, generate_key};
use crate::protocol::registry::ProtocolRegistry;

/// Create a new account with encrypted credentials
/// 
/// # Arguments
/// * `db` - Database state
/// * `protocol` - Protocol type ("discord", "whatsapp", "signal")
/// * `credentials` - Raw credentials (e.g., Discord bot token)
/// * `display_name` - Optional display name for the account
#[tauri::command]
pub async fn create_account(
    db: State<'_, Database>,
    protocol: String,
    credentials: String,
    display_name: Option<String>,
) -> Result<Account, String> {
    log::info!("Creating {} account", protocol);

    // Parse protocol
    let protocol = protocol.parse::<Protocol>()
        .map_err(|e| format!("Invalid protocol: {}", e))?;

    // Encrypt credentials
    let key = generate_key();
    let encrypted = encrypt_token(&credentials, &key)
        .map_err(|e| format!("Failed to encrypt credentials: {}", e))?;

    // Create account record
    let new_account = NewAccount {
        protocol,
        credentials: encrypted,
        display_name,
    };

    let account = Account::create(new_account);

    // Insert into database
    let conn = db.conn().await;
    conn.execute(
        "INSERT INTO accounts (id, protocol, credentials, display_name, connection_status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        vec![
            account.id.clone().into(),
            account.protocol.to_string().into(),
            account.credentials.clone().into(),
            account.display_name.clone().into(),
            account.connection_status.to_string().into(),
            account.created_at.into(),
            account.updated_at.into(),
        ],
    )
    .await
    .map_err(|e| format!("Failed to insert account: {}", e))?;

    log::info!("Created account {} ({})", account.id, account.protocol);
    Ok(account)
}

/// Delete an account and all associated data
/// 
/// # Arguments
/// * `db` - Database state
/// * `account_id` - ID of account to delete
#[tauri::command]
pub async fn delete_account(
    db: State<'_, Database>,
    account_id: String,
) -> Result<(), String> {
    log::info!("Deleting account {}", account_id);

    // TODO: Disconnect if connected

    let conn = db.conn().await;
    
    // Delete messages first (foreign key constraint)
    conn.execute(
        "DELETE FROM messages WHERE account_id = ?1",
        vec![account_id.clone().into()],
    )
    .await
    .map_err(|e| format!("Failed to delete messages: {}", e))?;

    // Delete conversations
    conn.execute(
        "DELETE FROM conversations WHERE account_id = ?1",
        vec![account_id.clone().into()],
    )
    .await
    .map_err(|e| format!("Failed to delete conversations: {}", e))?;

    // Delete account
    conn.execute(
        "DELETE FROM accounts WHERE id = ?1",
        vec![account_id.clone().into()],
    )
    .await
    .map_err(|e| format!("Failed to delete account: {}", e))?;

    log::info!("Deleted account {}", account_id);
    Ok(())
}

/// Get all accounts
/// 
/// # Arguments
/// * `db` - Database state
#[tauri::command]
pub async fn get_accounts(
    db: State<'_, Database>,
) -> Result<Vec<Account>, String> {
    let conn = db.conn().await;
    
    let rows = conn
        .select(
            "SELECT id, protocol, credentials, display_name, connection_status, created_at, updated_at 
             FROM accounts 
             ORDER BY created_at DESC",
            vec![],
        )
        .await
        .map_err(|e| format!("Failed to query accounts: {}", e))?;

    let mut accounts = Vec::new();
    for row in rows {
        let account = Account {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol: row.get("protocol")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(Protocol::Discord),
            credentials: row.get("credentials").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            display_name: row.get("display_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            connection_status: row.get("connection_status")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::ConnectionStatus::Disconnected),
            created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
            updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
        };
        accounts.push(account);
    }

    log::debug!("Fetched {} accounts", accounts.len());
    Ok(accounts)
}

/// Get a single account by ID
/// 
/// # Arguments
/// * `db` - Database state
/// * `account_id` - Account ID to fetch
#[tauri::command]
pub async fn get_account(
    db: State<'_, Database>,
    account_id: String,
) -> Result<Option<Account>, String> {
    let conn = db.conn().await;
    
    let rows = conn
        .select(
            "SELECT id, protocol, credentials, display_name, connection_status, created_at, updated_at 
             FROM accounts 
             WHERE id = ?1",
            vec![account_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to query account: {}", e))?;

    if let Some(row) = rows.into_iter().next() {
        let account = Account {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol: row.get("protocol")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(Protocol::Discord),
            credentials: row.get("credentials").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            display_name: row.get("display_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            connection_status: row.get("connection_status")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::ConnectionStatus::Disconnected),
            created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
            updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
        };
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

/// Connect a Discord account
/// 
/// # Arguments
/// * `app` - AppHandle for emitting events
/// * `registry` - Protocol registry state
/// * `db` - Database state
/// * `account_id` - Account ID to connect
#[tauri::command]
pub async fn connect_discord_account(
    app: AppHandle,
    registry: State<'_, ProtocolRegistry>,
    db: State<'_, Database>,
    account_id: String,
) -> Result<String, String> {
    log::info!("Connecting Discord account {}", account_id);

    // Load account from database
    let conn = db.conn().await;
    let rows = conn
        .select(
            "SELECT id, protocol, credentials, display_name, connection_status, created_at, updated_at 
             FROM accounts 
             WHERE id = ?1",
            vec![account_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to load account: {}", e))?;

    let row = rows.into_iter().next()
        .ok_or_else(|| format!("Account {} not found", account_id))?;

    let account = Account {
        id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        protocol: row.get("protocol")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(Protocol::Discord),
        credentials: row.get("credentials").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        display_name: row.get("display_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
        connection_status: row.get("connection_status")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(crate::models::ConnectionStatus::Disconnected),
        created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
        updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
    };

    // Create Discord adapter factory
    let factory = DiscordAdapterFactory::new();

    // Connect via registry
    registry.connect_account(app, &account, &factory)
        .await
        .map_err(|e| format!("Failed to connect account: {}", e))?;

    // Update account status in database
    let _ = conn.execute(
        "UPDATE accounts SET connection_status = ?1, updated_at = ?2 WHERE id = ?3",
        vec![
            crate::models::ConnectionStatus::Connected.to_string().into(),
            chrono::Utc::now().timestamp_millis().into(),
            account_id.clone().into(),
        ],
    )
    .await;

    let result = format!("Discord account {} connected successfully", account_id);
    log::info!("{}", result);
    Ok(result)
}
