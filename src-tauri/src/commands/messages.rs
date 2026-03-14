//! Message management commands
//!
//! Handles fetching and sending messages via protocol adapters.

use tauri::State;
use crate::db::Database;
use crate::models::{Message, NewMessage};
use crate::protocol::registry::ProtocolRegistry;

/// Get messages for a conversation
/// 
/// Fetches messages from the database for display.
/// In the future, may also sync with protocol if needed.
/// 
/// # Arguments
/// * `db` - Database state
/// * `conversation_id` - Conversation ID to fetch messages for
/// * `before` - Optional timestamp to fetch messages before (for pagination)
/// * `limit` - Maximum number of messages to fetch (default: 50)
#[tauri::command]
pub async fn get_messages(
    db: State<'_, Database>,
    conversation_id: String,
    before: Option<i64>,
    limit: Option<i64>,
) -> Result<Vec<Message>, String> {
    let limit = limit.unwrap_or(50).min(100) as usize;

    let conn = db.conn().await;
    
    let rows = if let Some(before_ts) = before {
        conn.select(
            "SELECT id, conversation_id, protocol_message_id, sender_id, sender_name, 
                    content, content_type, is_from_me, is_read, reply_to_message_id,
                    sent_at, received_at, edited_at
             FROM messages 
             WHERE conversation_id = ?1 AND sent_at < ?2
             ORDER BY sent_at DESC
             LIMIT ?3",
            vec![conversation_id.clone().into(), before_ts.into(), limit as i64],
        )
        .await
    } else {
        conn.select(
            "SELECT id, conversation_id, protocol_message_id, sender_id, sender_name, 
                    content, content_type, is_from_me, is_read, reply_to_message_id,
                    sent_at, received_at, edited_at
             FROM messages 
             WHERE conversation_id = ?1
             ORDER BY sent_at DESC
             LIMIT ?2",
            vec![conversation_id.clone().into(), limit as i64],
        )
        .await
    }.map_err(|e| format!("Failed to fetch messages: {}", e))?;

    let mut messages = Vec::with_capacity(rows.len());
    for row in rows {
        let message = Message {
            id: row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            conversation_id: row.get("conversation_id")
                .and_then(|v| v.as_str()).unwrap_or("").to_string(),
            protocol_message_id: row.get("protocol_message_id")
                .and_then(|v| v.as_str()).map(|s| s.to_string()),
            sender_id: row.get("sender_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            sender_name: row.get("sender_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            content: row.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            content_type: row.get("content_type")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::models::ContentType::Text),
            is_from_me: row.get("is_from_me").and_then(|v| v.as_bool()).unwrap_or(false),
            is_read: row.get("is_read").and_then(|v| v.as_bool()).unwrap_or(false),
            reply_to_message_id: row.get("reply_to_message_id")
                .and_then(|v| v.as_str()).map(|s| s.to_string()),
            sent_at: row.get("sent_at").and_then(|v| v.as_i64()).unwrap_or(0),
            received_at: row.get("received_at").and_then(|v| v.as_i64()),
            edited_at: row.get("edited_at").and_then(|v| v.as_i64()),
        };
        messages.push(message);
    }

    // Reverse to get chronological order (oldest first)
    messages.reverse();

    log::debug!("Fetched {} messages for conversation {}", messages.len(), conversation_id);
    Ok(messages)
}

/// Send a message in a conversation
/// 
/// Sends the message via the protocol adapter and stores the result.
/// 
/// # Arguments
/// * `db` - Database state
/// * `registry` - Protocol registry for adapter access
/// * `conversation_id` - Target conversation ID
/// * `content` - Message content to send
/// * `reply_to_message_id` - Optional message ID to reply to
#[tauri::command]
pub async fn send_message(
    db: State<'_, Database>,
    registry: State<'_, ProtocolRegistry>,
    conversation_id: String,
    content: String,
    reply_to_message_id: Option<String>,
) -> Result<Message, String> {
    log::info!("Sending message to conversation {}", conversation_id);

    // Get conversation to find associated account
    let conn = db.conn().await;
    let conv_rows = conn
        .select(
            "SELECT account_id FROM conversations WHERE id = ?1",
            vec![conversation_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to fetch conversation: {}", e))?;

    let account_id = conv_rows
        .into_iter()
        .next()
        .and_then(|row| row.get("account_id").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

    // Get adapter from registry
    let adapter = registry.get_adapter(&account_id)
        .await
        .ok_or_else(|| format!("Account {} not connected", account_id))?;

    // Send message via adapter
    let sent_message = adapter
        .send_message(&conversation_id, &content, reply_to_message_id.as_deref())
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    // Store message in database
    let conn = db.conn().await;
    conn.execute(
        "INSERT INTO messages (id, conversation_id, protocol_message_id, sender_id, sender_name, 
                               content, content_type, is_from_me, is_read, reply_to_message_id,
                               sent_at, received_at, edited_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        vec![
            sent_message.id.clone().into(),
            sent_message.conversation_id.clone().into(),
            sent_message.protocol_message_id.clone().into(),
            sent_message.sender_id.clone().into(),
            sent_message.sender_name.clone().into(),
            sent_message.content.clone().into(),
            sent_message.content_type.to_string().into(),
            true.into(), // is_from_me
            true.into(), // is_read - messages from me are automatically read
            sent_message.reply_to_message_id.clone().into(),
            sent_message.sent_at.into(),
            chrono::Utc::now().timestamp_millis().into(),
            sent_message.edited_at.into(),
        ],
    )
    .await
    .map_err(|e| format!("Failed to store message: {}", e))?;

    // Update conversation's last_message info
    let _ = conn.execute(
        "UPDATE conversations SET last_message_id = ?1, last_message_preview = ?2, 
                                  last_message_at = ?3, updated_at = ?4
         WHERE id = ?5",
        vec![
            sent_message.id.clone().into(),
            content[..content.len().min(100)].to_string().into(),
            sent_message.sent_at.into(),
            chrono::Utc::now().timestamp_millis().into(),
            conversation_id.clone().into(),
        ],
    ).await;

    log::info!("Sent message {} to conversation {}", sent_message.id, conversation_id);
    Ok(sent_message)
}

/// Sync messages for a conversation
/// 
/// Fetches latest messages from the protocol and updates the database.
/// 
/// # Arguments
/// * `db` - Database state
/// * `registry` - Protocol registry
/// * `conversation_id` - Conversation to sync
/// * `before` - Optional timestamp to sync messages before
#[tauri::command]
pub async fn sync_messages(
    db: State<'_, Database>,
    registry: State<'_, ProtocolRegistry>,
    conversation_id: String,
    before: Option<i64>,
) -> Result<usize, String> {
    log::info!("Syncing messages for conversation {}", conversation_id);

    // Get conversation to find associated account
    let conn = db.conn().await;
    let conv_rows = conn
        .select(
            "SELECT account_id FROM conversations WHERE id = ?1",
            vec![conversation_id.clone().into()],
        )
        .await
        .map_err(|e| format!("Failed to fetch conversation: {}", e))?;

    let account_id = conv_rows
        .into_iter()
        .next()
        .and_then(|row| row.get("account_id").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

    // Get adapter from registry
    let adapter = registry.get_adapter(&account_id)
        .await
        .ok_or_else(|| format!("Account {} not connected", account_id))?;

    // Fetch messages from protocol
    let messages = adapter
        .get_messages(&conversation_id, before, 100)
        .await
        .map_err(|e| format!("Failed to fetch messages from protocol: {}", e))?;

    let count = messages.len();

    // Store messages in database
    let conn = db.conn().await;
    for message in messages {
        let _ = conn.execute(
            "INSERT OR REPLACE INTO messages 
             (id, conversation_id, protocol_message_id, sender_id, sender_name, content, 
              content_type, is_from_me, is_read, reply_to_message_id, sent_at, received_at, edited_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            vec![
                message.id.clone().into(),
                message.conversation_id.clone().into(),
                message.protocol_message_id.clone().into(),
                message.sender_id.clone().into(),
                message.sender_name.clone().into(),
                message.content.clone().into(),
                message.content_type.to_string().into(),
                message.is_from_me.into(),
                message.is_read.into(),
                message.reply_to_message_id.clone().into(),
                message.sent_at.into(),
                message.received_at.into(),
                message.edited_at.into(),
            ],
        ).await;
    }

    log::info!("Synced {} messages for conversation {}", count, conversation_id);
    Ok(count)
}
