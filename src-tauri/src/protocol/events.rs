use crate::models::{Conversation, Message, Protocol};
use serde::{Deserialize, Serialize};

/// Events emitted by protocol adapters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ProtocolEvent {
    ConnectionChanged(ConnectionEvent),
    MessageReceived(MessageEvent),
    ConversationUpdated(ConversationEvent),
    Error(ProtocolErrorEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEvent {
    pub account_id: String,
    pub protocol: Protocol,
    pub status: ConnectionStatus,
    pub message: Option<String>, // e.g., error description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    pub account_id: String,
    pub conversation_id: String,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEvent {
    pub account_id: String,
    pub conversation: Conversation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolErrorEvent {
    pub account_id: String,
    pub error: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}
