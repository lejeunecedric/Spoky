//! Conversation management commands
//!
//! Handles fetching and managing conversations (channels, groups, DMs).

use tauri::State;
use crate::db::Database;
use crate::models::Conversation;
use crate::protocol::registry::ProtocolRegistry;

/// Get all conversations
/// 
/// Returns conversations ordered by most recent message first.
/// 
/// # Arguments
/// * `db` - Database state
#[tauri::command]
pub async fn get_conversations(
    db: State<'_, Database>,
) -> Result<Vec<Conversation>, String> {
    let conn = db.conn().await;
    
    let rows = conn
        .select(
            "SELECT id, protocol, account_id, protocol_conversation_id, title, participants,
                    last_message_id, last_message_at, last_message_preview,
                    unread_count, created_at, updated_at
             FROM conversations 
             ORDER BY last_message_at DESC NULLS LAST, updated_at DESC",
            vec![],
        )
        .await
        .map_err(|e| format!("Failed to fetch conversations: {}", e))?;

    let mut conversations = Vec::with_capacity(rows.len());
    for row in rows {
        let participants_json = row.get("participants")
            .and_then(|v| v.as_str())
            .unwrap_or("[]")
            .to_string();
        
        let participants: Vec<String> = serde_json::from_str(&participants_json)
            .unwrap_or_default();

        let conversation = Conversation {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol: row.get("protocol")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::Protocol::Discord),
            account_id: row.get("account_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol_conversation_id: row.get("protocol_conversation_id")
                .and_then(|v| v.as_str()).unwrap_or("").to_string(),
            title: row.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
            participants,
            last_message_id: row.get("last_message_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_preview: row.get("last_message_preview").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_at: row.get("last_message_at").and_then(|v| v.as_i64()),
            unread_count: row.get("unread_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
            updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
        };
        conversations.push(conversation);
    }

    log::debug!("Fetched {} conversations", conversations.len());
    Ok(conversations)
}

/// Get a single conversation by ID
/// 
/// # Arguments
/// * `db` - Database state
/// * `conversation_id` - Conversation ID to fetch
#[tauri::command]
pub async fn get_conversation(
    db: State<'_, Database>,
    conversation_id: String,
) -> Result<Option<Conversation>, String> {
    let conn = db.conn().await;
    
    let rows = conn
        .select(
            "SELECT id, protocol, account_id, protocol_conversation_id, title, participants,
                    last_message_id, last_message_at, last_message_preview,
                    unread_count, created_at, updated_at
             FROM conversations 
             WHERE id = ?1",
            vec![conversation_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to fetch conversation: {}", e))?;

    if let Some(row) = rows.into_iter().next() {
        let participants_json = row.get("participants")
            .and_then(|v| v.as_str())
            .unwrap_or("[]")
            .to_string();
        
        let participants: Vec<String> = serde_json::from_str(&participants_json)
            .unwrap_or_default();

        let conversation = Conversation {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol: row.get("protocol")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::Protocol::Discord),
            account_id: row.get("account_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol_conversation_id: row.get("protocol_conversation_id")
                .and_then(|v| v.as_str()).unwrap_or("").to_string(),
            title: row.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
            participants,
            last_message_id: row.get("last_message_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_preview: row.get("last_message_preview").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_at: row.get("last_message_at").and_then(|v| v.as_i64()),
            unread_count: row.get("unread_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
            updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
        };
        Ok(Some(conversation))
    } else {
        Ok(None)
    }
}

/// Mark a conversation as read
/// 
/// Resets the unread_count to 0 for the conversation.
/// 
/// # Arguments
/// * `db` - Database state
/// * `conversation_id` - Conversation to mark as read
#[tauri::command]
pub async fn mark_conversation_read(
    db: State<'_, Database>,
    conversation_id: String,
) -> Result<(), String> {
    let conn = db.conn().await;
    
    conn.execute(
        "UPDATE conversations SET unread_count = 0, updated_at = ?1 WHERE id = ?2",
        vec![
            chrono::Utc::now().timestamp_millis().into(),
            conversation_id.clone().into(),
        ],
    )
    .await
    .map_err(|e| format!("Failed to mark conversation as read: {}", e))?;

    log::debug!("Marked conversation {} as read", conversation_id);
    Ok(())
}

/// Sync conversations for an account
/// 
/// Fetches the latest conversation list from the protocol.
/// 
/// # Arguments
/// * `db` - Database state
/// * `registry` - Protocol registry
/// * `account_id` - Account to sync conversations for
#[tauri::command]
pub async fn sync_conversations(
    db: State<'_, Database>,
    registry: State<'_, ProtocolRegistry>,
    account_id: String,
) -> Result<usize, String> {
    log::info!("Syncing conversations for account {}", account_id);

    // Get adapter from registry
    let adapter = registry.get_adapter(&account_id)
        .await
        .ok_or_else(|| format!("Account {} not connected", account_id))?;

    // Fetch conversations from protocol
    let conversations = adapter
        .get_conversations()
        .await
        .map_err(|e| format!("Failed to fetch conversations from protocol: {}", e))?;

    let count = conversations.len();

    // Store conversations in database
    let conn = db.conn().await;
    for conv in conversations {
        let participants_json = serde_json::to_string(&conv.participants)
            .unwrap_or_else(|_| "[]".to_string());

        let _ = conn.execute(
            "INSERT OR REPLACE INTO conversations 
             (id, protocol, account_id, protocol_conversation_id, title, participants,
              last_message_id, last_message_at, last_message_preview, unread_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            vec![
                conv.id.clone().into(),
                conv.protocol.to_string().into(),
                conv.account_id.clone().into(),
                conv.protocol_conversation_id.clone().into(),
                conv.title.clone().into(),
                participants_json.into(),
                conv.last_message_id.clone().into(),
                conv.last_message_at.into(),
                conv.last_message_preview.clone().into(),
                conv.unread_count.into(),
                conv.created_at.into(),
                chrono::Utc::now().timestamp_millis().into(),
            ],
        ).await;
    }

    log::info!("Synced {} conversations for account {}", count, account_id);
    Ok(count)
}

/// Get conversations for a specific account
/// 
/// # Arguments
/// * `db` - Database state
/// * `account_id` - Account ID to filter by
#[tauri::command]
pub async fn get_conversations_for_account(
    db: State<'_, Database>,
    account_id: String,
) -> Result<Vec<Conversation>, String> {
    let conn = db.conn().await;
    
    let rows = conn
        .select(
            "SELECT id, protocol, account_id, protocol_conversation_id, title, participants,
                    last_message_id, last_message_at, last_message_preview,
                    unread_count, created_at, updated_at
             FROM conversations 
             WHERE account_id = ?1
             ORDER BY last_message_at DESC NULLS LAST, updated_at DESC",
            vec![account_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to fetch conversations: {}", e))?;

    let mut conversations = Vec::with_capacity(rows.len());
    for row in rows {
        let participants_json = row.get("participants")
            .and_then(|v| v.as_str())
            .unwrap_or("[]")
            .to_string();
        
        let participants: Vec<String> = serde_json::from_str(&participants_json)
            .unwrap_or_default();

        let conversation = Conversation {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol: row.get("protocol")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::Protocol::Discord),
            account_id: row.get("account_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol_conversation_id: row.get("protocol_conversation_id")
                .and_then(|v| v.as_str()).unwrap_or("").to_string(),
            title: row.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()),
            participants,
            last_message_id: row.get("last_message_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_preview: row.get("last_message_preview").and_then(|v| v.as_str()).map(|s| s.to_string()),
            last_message_at: row.get("last_message_at").and_then(|v| v.as_i64()),
            unread_count: row.get("unread_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            created_at: row.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0),
            updated_at: row.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0),
        };
        conversations.push(conversation);
    }

    log::debug!("Fetched {} conversations for account {}", conversations.len(), account_id);
    Ok(conversations)
}
