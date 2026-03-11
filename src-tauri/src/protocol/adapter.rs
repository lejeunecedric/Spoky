use async_trait::async_trait;
use crate::models::{Account, Conversation, Message, NewMessage, Protocol};
use super::events::{ProtocolEvent, ConnectionStatus};

/// Trait that all protocol implementations must satisfy
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Return the protocol type this adapter handles
    fn protocol(&self) -> Protocol;

    /// Connect to the protocol service
    /// Returns initial connection status
    async fn connect(&mut self, account: &Account) -> Result<ConnectionStatus, ProtocolError>;

    /// Disconnect from the protocol service
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;

    /// Get current connection status
    fn connection_status(&self) -> ConnectionStatus;

    /// Fetch conversations from the protocol
    /// Used during initial sync or refresh
    async fn get_conversations(&self) -> Result<Vec<Conversation>, ProtocolError>;

    /// Fetch messages for a specific conversation
    async fn get_messages(
        &self,
        conversation_id: &str,
        before: Option<chrono::DateTime<chrono::Utc>>,
        limit: usize,
    ) -> Result<Vec<Message>, ProtocolError>;

    /// Send a message in a conversation
    async fn send_message(&self, conversation_id: &str, content: &str) -> Result<Message, ProtocolError>;

    /// Set callback for protocol events
    /// The adapter calls this when messages arrive, connection changes, etc.
    fn on_event(&mut self, callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>);
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Rate limited")]
    RateLimited,
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Factory for creating protocol adapters
pub trait ProtocolAdapterFactory: Send + Sync {
    fn create(&self) -> Box<dyn ProtocolAdapter>;
    fn protocol(&self) -> Protocol;
}