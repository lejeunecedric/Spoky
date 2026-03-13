pub mod account;
pub mod conversation;
pub mod message;

pub use account::{Account, AccountUpdate, NewAccount};
pub use conversation::{Conversation, ConversationUpdate, NewConversation};
pub use message::{Message, MessageReaction, MessageUpdate, NewMessage, ReadReceipt};

use serde::{Deserialize, Serialize};

/// Supported chat protocols
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Discord,
    WhatsApp,
    Signal,
}

impl Protocol {
    /// Get protocol as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::Discord => "discord",
            Protocol::WhatsApp => "whatsapp",
            Protocol::Signal => "signal",
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "discord" => Ok(Protocol::Discord),
            "whatsapp" => Ok(Protocol::WhatsApp),
            "signal" => Ok(Protocol::Signal),
            _ => Err(format!("Unknown protocol: {}", s)),
        }
    }
}

/// Connection status for accounts
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error,
}

impl ConnectionStatus {
    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionStatus::Connected => "connected",
            ConnectionStatus::Connecting => "connecting",
            ConnectionStatus::Disconnected => "disconnected",
            ConnectionStatus::Error => "error",
        }
    }
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ConnectionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "connected" => Ok(ConnectionStatus::Connected),
            "connecting" => Ok(ConnectionStatus::Connecting),
            "disconnected" => Ok(ConnectionStatus::Disconnected),
            "error" => Ok(ConnectionStatus::Error),
            _ => Err(format!("Unknown connection status: {}", s)),
        }
    }
}

/// Content types for messages
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Image,
    File,
}

impl ContentType {
    /// Get content type as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::File => "file",
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(ContentType::Text),
            "image" => Ok(ContentType::Image),
            "file" => Ok(ContentType::File),
            _ => Err(format!("Unknown content type: {}", s)),
        }
    }
}
