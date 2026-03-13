//! Protocol event types for real-time communication
//!
//! Events flow from protocol adapters → event channel → Tauri emit → frontend

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

/// Connection status change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEvent {
    pub account_id: String,
    pub protocol: Protocol,
    pub status: ConnectionStatus,
    pub message: Option<String>, // e.g., error description or connection info
}

/// New message received event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    pub account_id: String,
    pub conversation_id: String,
    pub message: Message,
}

/// Conversation created or updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEvent {
    pub account_id: String,
    pub conversation: Conversation,
}

/// Protocol error event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolErrorEvent {
    pub account_id: String,
    pub error: String,
    pub recoverable: bool,
}

/// Connection status states
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl ConnectionStatus {
    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionStatus::Disconnected => "disconnected",
            ConnectionStatus::Connecting => "connecting",
            ConnectionStatus::Connected => "connected",
            ConnectionStatus::Error => "error",
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected)
    }

    /// Check if connecting
    pub fn is_connecting(&self) -> bool {
        matches!(self, ConnectionStatus::Connecting)
    }
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
