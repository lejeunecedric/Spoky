//! Discord protocol implementation for Spoky
//!
//! Implements the ProtocolAdapter trait for Discord using the serenity crate.
//! Uses bot token authentication and Discord Gateway for real-time events.

pub mod adapter;
pub mod auth;
pub mod sync;

pub use adapter::DiscordAdapter;

use crate::models::{Conversation, Message, Protocol};
use crate::protocol::events::{ConversationEvent, MessageEvent, ProtocolEvent};
use crate::protocol::ProtocolError;
use serenity::async_trait;
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};
use tauri::AppHandle;
use thiserror::Error;

/// Discord-specific errors
#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("Invalid bot token format")]
    InvalidToken,
    #[error("Discord API error: {0}")]
    ApiError(String),
    #[error("Gateway connection failed: {0}")]
    GatewayError(String),
    #[error("Rate limited: retry after {0}s")]
    RateLimited(u64),
    #[error("Channel not found: {0}")]
    ChannelNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<DiscordError> for ProtocolError {
    fn from(e: DiscordError) -> Self {
        match e {
            DiscordError::InvalidToken => ProtocolError::InvalidCredentials,
            DiscordError::ApiError(msg) => ProtocolError::ProtocolError(msg),
            DiscordError::GatewayError(msg) => ProtocolError::ConnectionFailed(msg),
            DiscordError::RateLimited(_) => ProtocolError::RateLimited,
            DiscordError::ChannelNotFound(msg) => ProtocolError::ProtocolError(msg),
            DiscordError::PermissionDenied(msg) => ProtocolError::ProtocolError(msg),
        }
    }
}

impl From<serenity::Error> for DiscordError {
    fn from(e: serenity::Error) -> Self {
        match e {
            serenity::Error::Http(http_err) => {
                if let serenity::http::error::ErrorType::UnsuccessfulRequest(response) = &http_err {
                    if response.status_code == 429 {
                        return DiscordError::RateLimited(
                            response.retry_after.unwrap_or(0) as u64
                        );
                    }
                    if response.status_code == 401 {
                        return DiscordError::InvalidToken;
                    }
                    if response.status_code == 403 {
                        return DiscordError::PermissionDenied(
                            response.error.message.clone()
                        );
                    }
                }
                DiscordError::ApiError(http_err.to_string())
            }
            serenity::Error::Gateway(msg) => DiscordError::GatewayError(msg),
            _ => DiscordError::ApiError(e.to_string()),
        }
    }
}

/// Discord Gateway event handler
/// 
/// Receives real-time events from Discord Gateway and emits them to the frontend
/// via Tauri's event system.
pub struct DiscordEventHandler {
    /// App handle for emitting Tauri events to the frontend
    pub app_handle: AppHandle,
    /// The account ID this handler is associated with
    pub account_id: String,
    /// Bot user ID (set when Ready event is received)
    pub bot_user_id: std::sync::Arc<tokio::sync::RwLock<Option<serenity::model::id::UserId>>>,
}

impl DiscordEventHandler {
    /// Create a new event handler
    pub fn new(app_handle: AppHandle, account_id: String) -> Self {
        Self {
            app_handle,
            account_id,
            bot_user_id: std::sync::Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Convert a serenity Message to our Message model
    fn convert_message(&self, msg: &SerenityMessage, is_outgoing: bool) -> Message {
        Message {
            id: msg.id.to_string(),
            conversation_id: msg.channel_id.to_string(),
            sender_id: msg.author.id.to_string(),
            content: msg.content.clone(),
            sent_at: msg.timestamp.naive_utc().and_utc(),
            read: !is_outgoing, // Outgoing messages are considered read
        }
    }

    /// Emit a protocol event to the frontend
    fn emit_event(&self, event: ProtocolEvent) {
        if let Err(e) = self.app_handle.emit("protocol:event", event) {
            log::error!("Failed to emit protocol event: {}", e);
        }
    }
}

#[async_trait]
impl EventHandler for DiscordEventHandler {
    /// Called when the bot is ready/connected to the Gateway
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log::info!("Discord Gateway connected as {} (id: {})", ready.user.name, ready.user.id);
        
        // Store the bot user ID for outgoing message detection
        let mut bot_id = self.bot_user_id.write().await;
        *bot_id = Some(ready.user.id);
        
        // Emit connection status event
        let event = ProtocolEvent::ConnectionChanged(crate::protocol::events::ConnectionEvent {
            account_id: self.account_id.clone(),
            protocol: Protocol::Discord,
            status: crate::protocol::events::ConnectionStatus::Connected,
            message: Some(format!("Connected as {}", ready.user.name)),
        });
        
        self.emit_event(event);
    }

    /// Called when a new message is received
    async fn message(&self, ctx: Context, msg: SerenityMessage) {
        log::debug!("Discord message received via Gateway in channel {}", msg.channel_id);
        
        // Check if this is an outgoing message (sent by the bot)
        let bot_id = *self.bot_user_id.read().await;
        let is_outgoing = bot_id.map(|id| id == msg.author.id).unwrap_or(false);
        
        // Convert the serenity message to our Message model
        let message = self.convert_message(&msg, is_outgoing);
        
        // Emit MessageReceived event
        let event = ProtocolEvent::MessageReceived(MessageEvent {
            account_id: self.account_id.clone(),
            conversation_id: msg.channel_id.to_string(),
            message: message.clone(),
        });
        
        self.emit_event(event);
        
        // Also emit ConversationUpdated to refresh conversation list
        // Try to get channel info for conversation name
        let conversation_name = if let Some(guild_id) = msg.guild_id {
            // Try to get the channel name from the guild
            match msg.channel_id.to_channel(&ctx.http).await {
                Ok(channel) => {
                    match channel {
                        serenity::model::channel::Channel::Guild(guild_channel) => {
                            guild_channel.name.clone()
                        }
                        _ => "Unknown Channel".to_string(),
                    }
                }
                Err(_) => "Unknown Channel".to_string(),
            }
        } else {
            // DM channel
            msg.author.name.clone()
        };
        
        let conversation = Conversation {
            id: msg.channel_id.to_string(),
            protocol: Protocol::Discord,
            account_id: self.account_id.clone(),
            name: conversation_name,
            participants: vec![msg.author.id.to_string()],
            unread_count: if is_outgoing { 0 } else { 1 },
            last_message_at: Some(msg.timestamp.naive_utc().and_utc()),
        };
        
        let conversation_event = ProtocolEvent::ConversationUpdated(ConversationEvent {
            account_id: self.account_id.clone(),
            conversation,
        });
        
        self.emit_event(conversation_event);
        
        log::info!("Discord message received via Gateway from {}", msg.author.name);
    }
}
