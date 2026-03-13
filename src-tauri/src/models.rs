use serde::{Deserialize, Serialize};

/// Supported messaging protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Protocol {
    Discord,
    WhatsApp,
    Signal,
}

/// User account for a protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub protocol: Protocol,
    pub name: String,
    pub credentials: String, // Encrypted credentials
    pub connected: bool,
}

/// A conversation (chat/channel/DM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub protocol: Protocol,
    pub account_id: String,
    pub name: String,
    pub participants: Vec<String>,
    pub unread_count: i32,
    pub last_message_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub read: bool,
}

/// Data for creating a new message
#[derive(Debug, Clone)]
pub struct NewMessage {
    pub conversation_id: String,
    pub content: String,
}
