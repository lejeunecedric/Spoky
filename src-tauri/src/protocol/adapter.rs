//! Protocol adapter trait definition
//!
//! All protocol implementations (Discord, WhatsApp, Signal) must implement this trait.

use async_trait::async_trait;
use crate::models::{Account, Conversation, Message, Protocol};
use super::events::{ConnectionStatus, ProtocolEvent};
use super::ProtocolError;

use tauri::AppHandle;

/// Trait that all protocol implementations must satisfy
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Return the protocol type this adapter handles
    fn protocol(&self) -> Protocol;

    /// Set the app handle for Tauri event emission
    /// Must be called before connect() for adapters that use direct event emission
    fn set_app_handle(&mut self, app_handle: AppHandle);

    /// Connect to the protocol service
    /// Returns initial connection status
    async fn connect(
        &mut self,
        account: &Account,
    ) -> Result<ConnectionStatus, ProtocolError>;

    /// Disconnect from the protocol service
    async fn disconnect(&mut self,
    ) -> Result<(), ProtocolError>;

    /// Get current connection status
    fn connection_status(&self) -> ConnectionStatus;

    /// Fetch conversations from the protocol
    /// Used during initial sync or refresh
    async fn get_conversations(
        &self,
    ) -> Result<Vec<Conversation>, ProtocolError>;

    /// Fetch messages for a specific conversation
    async fn get_messages(
        &self,
        conversation_id: &str,
        before: Option<i64>, // Unix timestamp in milliseconds
        limit: usize,
    ) -> Result<Vec<Message>, ProtocolError>;

    /// Send a message in a conversation
    /// 
    /// # Arguments
    /// * `conversation_id` - Target conversation ID
    /// * `content` - Message content
    /// * `reply_to_message_id` - Optional message ID to reply to
    async fn send_message(
        &self,
        conversation_id: &str,
        content: &str,
        reply_to_message_id: Option<&str>,
    ) -> Result<Message, ProtocolError>;

    /// Create a new direct message conversation with a user
    /// 
    /// # Arguments
    /// * `user_id` - The user ID to create a DM with
    async fn create_dm_conversation(
        &self,
        user_id: &str,
    ) -> Result<Conversation, ProtocolError>;

    /// Set callback for protocol events
    /// The adapter calls this when messages arrive, connection changes, etc.
    fn on_event(
        &mut self,
        callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>,
    );

    /// Get account ID associated with this adapter
    fn account_id(&self) -> Option<&str>;
}

/// Factory for creating protocol adapters
pub trait ProtocolAdapterFactory: Send + Sync {
    /// Create a new adapter instance
    fn create(&self,
    ) -> Box<dyn ProtocolAdapter>;

    /// Get the protocol type this factory creates
    fn protocol(&self) -> Protocol;
}

/// Adapter information for UI display
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub protocol: Protocol,
    pub name: String,
    pub version: String,
    pub features: Vec<AdapterFeature>,
}

/// Features supported by an adapter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterFeature {
    TextMessages,
    MediaMessages,
    Reactions,
    Replies,
    GroupChats,
    DirectMessages,
    Presence,
    TypingIndicators,
}
