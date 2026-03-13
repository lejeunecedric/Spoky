//! Discord adapter implementing the ProtocolAdapter trait

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{Account, Conversation, Message, NewMessage, Protocol};
use crate::protocol::adapter::{ProtocolAdapter, ProtocolAdapterFactory};
use crate::protocol::events::{ProtocolEvent, ConnectionStatus, ConnectionEvent, MessageEvent};
use crate::protocol::ProtocolError;

use super::DiscordError;

/// Discord protocol adapter
/// 
/// Uses serenity crate for Discord API and Gateway communication.
/// Manages bot connection and converts Discord structures to Spoky models.
pub struct DiscordAdapter {
    /// Serenity HTTP client for REST API calls
    http_client: Option<Arc<serenity::http::Http>>,
    /// Gateway client for real-time events
    client: Option<serenity::Client>,
    /// Current connection status
    status: ConnectionStatus,
    /// Event callback for sending events to frontend
    event_callback: Option<Box<dyn Fn(ProtocolEvent) + Send + Sync>>,
    /// Associated account ID
    account_id: Option<String>,
    /// Bot token (encrypted at rest, decrypted in memory)
    bot_token: Option<String>,
}

impl DiscordAdapter {
    /// Create a new Discord adapter
    pub fn new() -> Self {
        Self {
            http_client: None,
            client: None,
            status: ConnectionStatus::Disconnected,
            event_callback: None,
            account_id: None,
            bot_token: None,
        }
    }

    /// Get the HTTP client if connected
    fn http(&self) -> Result<&Arc<serenity::http::Http>>, DiscordError> {
        self.http_client.as_ref()
            .ok_or(DiscordError::GatewayError("Not connected".to_string()))
    }

    /// Send a protocol event through the callback
    fn emit_event(&self, event: ProtocolEvent) {
        if let Some(ref callback) = self.event_callback {
            callback(event);
        }
    }

    /// Emit connection status change event
    fn emit_connection_change(&self, status: ConnectionStatus, message: Option<String>) {
        if let Some(ref account_id) = self.account_id {
            let event = ProtocolEvent::ConnectionChanged(ConnectionEvent {
                account_id: account_id.clone(),
                protocol: Protocol::Discord,
                status,
                message,
            });
            self.emit_event(event);
        }
    }

    /// Convert a Discord channel to Spoky Conversation model
    fn channel_to_conversation(&self, channel: &serenity::model::channel::Channel) -> Conversation {
        let (id, name, channel_type) = match channel {
            serenity::model::channel::Channel::Guild(channel) => {
                (channel.id.to_string(), channel.name.clone(), "guild")
            }
            serenity::model::channel::Channel::Private(channel) => {
                let name = channel.recipient.as_ref()
                    .map(|u| u.name.clone())
                    .unwrap_or_else(|| "Direct Message".to_string());
                (channel.id.to_string(), name, "dm")
            }
            serenity::model::channel::Channel::Category(_) => {
                return Conversation {
                    id: "0".to_string(),
                    protocol: Protocol::Discord,
                    title: None,
                    channel_type: "category".to_string(),
                    unread_count: 0,
                    last_message_at: None,
                    last_message_preview: None,
                    account_id: self.account_id.clone().unwrap_or_default(),
                };
            }
            _ => (channel.id().to_string(), "Unknown".to_string(), "unknown"),
        };

        Conversation {
            id,
            protocol: Protocol::Discord,
            title: Some(name),
            channel_type: channel_type.to_string(),
            unread_count: 0,
            last_message_at: None,
            last_message_preview: None,
            account_id: self.account_id.clone().unwrap_or_default(),
        }
    }

    /// Convert a Discord message to Spoky Message model
    fn discord_message_to_message(&self, msg: &serenity::model::channel::Message) -> Message {
        Message {
            id: msg.id.to_string(),
            conversation_id: msg.channel_id.to_string(),
            sender_id: msg.author.id.to_string(),
            sender_name: Some(msg.author.name.clone()),
            content: msg.content.clone(),
            content_type: crate::models::ContentType::Text,
            sent_at: msg.timestamp.unix_timestamp() * 1000, // Convert to milliseconds
            edited_at: msg.edited_timestamp.map(|t| t.unix_timestamp() * 1000),
            is_from_me: false, // Will be set based on bot user comparison
            account_id: self.account_id.clone().unwrap_or_default(),
            reply_to_message_id: msg.referenced_message.as_ref().map(|m| m.id.to_string()),
        }
    }
}

impl Default for DiscordAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProtocolAdapter for DiscordAdapter {
    fn protocol(&self) -> Protocol {
        Protocol::Discord
    }

    async fn connect(
        &mut self,
        account: &Account,
    ) -> Result<ConnectionStatus, ProtocolError> {
        // Decrypt bot token from account credentials
        // For now, credentials are stored as base64-encoded encrypted token
        // TODO: Implement proper decryption using crypto module
        let encrypted_token = base64::decode(&account.credentials)
            .map_err(|_| ProtocolError::InvalidCredentials)?;
        
        // Decrypt token (placeholder - implement proper crypto)
        let bot_token = String::from_utf8(encrypted_token)
            .map_err(|_| ProtocolError::InvalidCredentials)?;

        self.bot_token = Some(bot_token.clone());
        self.account_id = Some(account.id.clone());
        self.status = ConnectionStatus::Connecting;

        // Create HTTP client
        let http = serenity::http::Http::new(&bot_token);
        
        // Verify token by fetching current user
        match http.get_current_user().await {
            Ok(user) => {
                log::info!("Connected to Discord as {}", user.name);
                self.http_client = Some(Arc::new(http));
                self.status = ConnectionStatus::Connected;
                self.emit_connection_change(
                    ConnectionStatus::Connected,
                    Some(format!("Connected as {}", user.name)),
                );
            }
            Err(e) => {
                let err_msg = format!("Failed to authenticate: {}", e);
                log::error!("{}", err_msg);
                self.status = ConnectionStatus::Error;
                self.emit_connection_change(ConnectionStatus::Error, Some(err_msg));
                return Err(DiscordError::InvalidToken.into());
            }
        }

        Ok(self.status)
    }

    async fn disconnect(&mut self,
    ) -> Result<(), ProtocolError> {
        if self.client.is_some() {
            // Graceful shutdown of gateway client
            if let Some(client) = self.client.take() {
                // serenity Client doesn't have explicit shutdown, 
                // it stops when dropped
                drop(client);
            }
        }

        self.http_client = None;
        self.bot_token = None;
        self.status = ConnectionStatus::Disconnected;
        
        self.emit_connection_change(ConnectionStatus::Disconnected, None);
        
        log::info!("Discord account {} disconnected", 
            self.account_id.as_deref().unwrap_or("unknown"));
        
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.status
    }

    async fn get_conversations(
        &self,
    ) -> Result<Vec<Conversation>, ProtocolError> {
        let http = self.http()?;

        // Get user's guilds (servers)
        let guilds = http.get_guilds(None, Some(100)).await
            .map_err(DiscordError::from)?;

        let mut conversations = Vec::new();

        // For each guild, get text channels
        for guild_info in guilds {
            let guild_id = guild_info.id;
            
            let channels = http.get_channels(guild_id).await
                .map_err(DiscordError::from)?;

            for channel in channels {
                // Only include text channels and direct messages
                if matches!(
                    channel,
                    serenity::model::channel::Channel::Guild(_)
                        | serenity::model::channel::Channel::Private(_)
                ) {
                    conversations.push(self.channel_to_conversation(&channel));
                }
            }
        }

        // Also get DM channels
        let dm_channels = http.get_user_dm_channels().await
            .map_err(DiscordError::from)?;

        for channel in dm_channels {
            conversations.push(self.channel_to_conversation(
                &serenity::model::channel::Channel::Private(channel),
            ));
        }

        log::info!("Fetched {} conversations from Discord", conversations.len());
        Ok(conversations)
    }

    async fn get_messages(
        &self,
        conversation_id: &str,
        before: Option<i64>,
        limit: usize,
    ) -> Result<Vec<Message>, ProtocolError> {
        let http = self.http()?;
        
        let channel_id = conversation_id.parse::<serenity::model::id::ChannelId>()
            .map_err(|_| DiscordError::ChannelNotFound(conversation_id.to_string()))?;

        let limit = limit.min(100); // Discord API max is 100

        let messages = if let Some(before_ts) = before {
            // Convert timestamp to message ID
            let before_id = serenity::model::id::MessageId::new(before_ts as u64);
            channel_id.messages(&http,
                serenity::builder::GetMessages::new()
                    .before(before_id)
                    .limit(limit as u8)
            ).await
        } else {
            channel_id.messages(
                &http,
                serenity::builder::GetMessages::new()
                    .limit(limit as u8)
            ).await
        }.map_err(DiscordError::from)?;

        let messages: Vec<Message> = messages
            .iter()
            .map(|m| self.discord_message_to_message(m))
            .collect();

        Ok(messages)
    }

    async fn send_message(
        &self,
        conversation_id: &str,
        content: &str,
    ) -> Result<Message, ProtocolError> {
        let http = self.http()?;
        
        let channel_id = conversation_id.parse::<serenity::model::id::ChannelId>()
            .map_err(|_| DiscordError::ChannelNotFound(conversation_id.to_string()))?;

        let sent_msg = channel_id.say(&http,
            content
        ).await.map_err(DiscordError::from)?;

        let message = self.discord_message_to_message(&sent_msg);
        
        log::info!("Sent message {} to channel {}", sent_msg.id, channel_id);
        
        Ok(message)
    }

    fn on_event(
        &mut self,
        callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>,
    ) {
        self.event_callback = Some(callback);
    }

    fn account_id(&self) -> Option<&str> {
        self.account_id.as_deref()
    }
}

/// Factory for creating Discord adapters
pub struct DiscordAdapterFactory;

impl DiscordAdapterFactory {
    /// Create a new Discord adapter factory
    pub fn new() -> Self {
        Self
    }
}

impl Default for DiscordAdapterFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtocolAdapterFactory for DiscordAdapterFactory {
    fn create(&self,
    ) -> Box<dyn ProtocolAdapter> {
        Box::new(DiscordAdapter::new())
    }

    fn protocol(&self) -> Protocol {
        Protocol::Discord
    }
}
