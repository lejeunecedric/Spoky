//! Discord synchronization logic
//! 
//! Handles syncing conversations and messages from Discord API.
//! Uses incremental sync with sync_checkpoints for efficiency.

use crate::db::Database;
use crate::models::{Conversation, Message, Protocol};
use crate::protocol::discord::DiscordError;

/// Sync conversations from Discord for an account
/// 
/// # Arguments
/// * `db` - Database connection
/// * `http` - Discord HTTP client
/// * `account_id` - Account ID to sync for
/// 
/// # Returns
/// Number of conversations synced
pub async fn sync_conversations(
    db: &Database,
    http: &serenity::http::Http,
    account_id: &str,
) -> Result<usize, DiscordError> {
    log::info!("Syncing conversations for account {}", account_id);

    // Get user's guilds (servers)
    let guilds = http.get_guilds(None, Some(100)).await?;
    let mut total_conversations = 0;

    for guild_info in guilds {
        let guild_id = guild_info.id;
        
        // Get channels for this guild
        let channels = http.get_channels(guild_id).await?;
        
        for channel in channels {
            if let Some(conversation) = channel_to_conversation(&channel, account_id
            ) {
                // Store in database
                // TODO: Implement conversation repository
                // db.conversations.upsert(conversation).await?;
                total_conversations += 1;
            }
        }
    }

    // Get DM channels
    let dm_channels = http.get_user_dm_channels().await?;
    for channel in dm_channels {
        if let Some(conversation) = dm_channel_to_conversation(&channel, account_id
        ) {
            // db.conversations.upsert(conversation).await?;
            total_conversations += 1;
        }
    }

    // Update sync checkpoint
    // TODO: Implement sync checkpoint tracking
    // db.sync_checkpoints.update(account_id, "conversations", now()).await?;

    log::info!("Synced {} conversations for account {}", total_conversations, account_id);
    Ok(total_conversations)
}

/// Sync messages for a specific conversation
/// 
/// # Arguments
/// * `db` - Database connection
/// * `http` - Discord HTTP client
/// * `conversation_id` - Discord channel ID
/// * `account_id` - Account ID
/// * `before` - Optional timestamp to fetch messages before
/// 
/// # Returns
/// Number of messages synced
pub async fn sync_messages(
    _db: &Database,
    http: &serenity::http::Http,
    conversation_id: &str,
    _account_id: &str,
    before: Option<i64>,
) -> Result<usize, DiscordError> {
    let channel_id = conversation_id.parse::<serenity::model::id::ChannelId>()
        .map_err(|_| DiscordError::ChannelNotFound(conversation_id.to_string()))?;

    let limit = 100; // Discord API max

    let messages = if let Some(before_ts) = before {
        let before_id = serenity::model::id::MessageId::new(before_ts as u64);
        channel_id.messages(
            http,
            serenity::builder::GetMessages::new()
                .before(before_id)
                .limit(limit)
        ).await?
    } else {
        channel_id.messages(
            http,
            serenity::builder::GetMessages::new()
                .limit(limit)
        ).await?
    };

    let count = messages.len();
    
    // Store messages in database
    for msg in messages {
        let _message = discord_message_to_message(&msg, conversation_id);
        // TODO: db.messages.upsert(message).await?;
    }

    log::info!("Synced {} messages for conversation {}", count, conversation_id);
    Ok(count)
}

/// Convert a Discord channel to Conversation model
fn channel_to_conversation(
    channel: &serenity::model::channel::Channel,
    account_id: &str,
) -> Option<Conversation> {
    match channel {
        serenity::model::channel::Channel::Guild(ch) => {
            Some(Conversation {
                id: ch.id.to_string(),
                protocol: Protocol::Discord,
                title: Some(ch.name.clone()),
                channel_type: "guild".to_string(),
                unread_count: 0,
                last_message_at: ch.last_message_id.map(|id| {
                    // Discord snowflake to timestamp (rough approximation)
                    ((id.get() >> 22) + 1420070400000) as i64
                }),
                last_message_preview: None,
                account_id: account_id.to_string(),
            })
        }
        _ => None, // Skip categories and other types for now
    }
}

/// Convert a Discord DM channel to Conversation model
fn dm_channel_to_conversation(
    channel: &serenity::model::channel::PrivateChannel,
    account_id: &str,
) -> Option<Conversation> {
    let name = channel.recipient.as_ref()
        .map(|u| u.name.clone())
        .unwrap_or_else(|| "Direct Message".to_string());

    Some(Conversation {
        id: channel.id.to_string(),
        protocol: Protocol::Discord,
        title: Some(name),
        channel_type: "dm".to_string(),
        unread_count: 0,
        last_message_at: channel.last_message_id.map(|id| {
            ((id.get() >> 22) + 1420070400000) as i64
        }),
        last_message_preview: None,
        account_id: account_id.to_string(),
    })
}

/// Convert a Discord message to Message model
fn discord_message_to_message(
    msg: &serenity::model::channel::Message,
    conversation_id: &str,
) -> Message {
    Message {
        id: msg.id.to_string(),
        conversation_id: conversation_id.to_string(),
        sender_id: msg.author.id.to_string(),
        sender_name: Some(msg.author.name.clone()),
        content: msg.content.clone(),
        content_type: crate::models::ContentType::Text,
        sent_at: msg.timestamp.unix_timestamp() * 1000,
        edited_at: msg.edited_timestamp.map(|t| t.unix_timestamp() * 1000),
        is_from_me: false,
        account_id: String::new(), // Set by caller
        reply_to_message_id: msg.referenced_message.as_ref().map(|m| m.id.to_string()),
    }
}
