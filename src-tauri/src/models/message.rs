use crate::models::ContentType;
use serde::{Deserialize, Serialize};

/// Message data model representing a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub protocol_message_id: Option<String>, // original ID from protocol
    pub sender_id: String,
    pub sender_name: Option<String>,
    pub content: String,
    pub content_type: ContentType,
    pub is_from_me: bool,
    pub is_read: bool,
    pub reply_to_message_id: Option<String>,
    pub sent_at: i64,             // Unix timestamp in milliseconds
    pub received_at: Option<i64>, // Unix timestamp in milliseconds
    pub edited_at: Option<i64>,   // Unix timestamp in milliseconds
}

/// New message data for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub conversation_id: String,
    pub protocol_message_id: Option<String>,
    pub sender_id: String,
    pub sender_name: Option<String>,
    pub content: String,
    pub content_type: ContentType,
    pub is_from_me: bool,
    pub reply_to_message_id: Option<String>,
    pub sent_at: i64,
}

/// Message update data (for edits)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageUpdate {
    pub content: Option<String>,
    pub is_read: Option<bool>,
}

/// Reaction to a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub id: String,
    pub message_id: String,
    pub sender_id: String,
    pub emoji: String,
    pub created_at: i64, // Unix timestamp in milliseconds
}

/// Read receipt for a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadReceipt {
    pub id: String,
    pub message_id: String,
    pub reader_id: String,
    pub read_at: i64, // Unix timestamp in milliseconds
}

impl Message {
    /// Create a new message with generated ID
    pub fn create(new: NewMessage) -> Self {
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            conversation_id: new.conversation_id,
            protocol_message_id: new.protocol_message_id,
            sender_id: new.sender_id,
            sender_name: new.sender_name,
            content: new.content,
            content_type: new.content_type,
            is_from_me: new.is_from_me,
            is_read: new.is_from_me, // Messages from me are automatically read
            reply_to_message_id: new.reply_to_message_id,
            sent_at: new.sent_at,
            received_at: Some(chrono::Utc::now().timestamp_millis()),
            edited_at: None,
        }
    }

    /// Update message content (for edits)
    pub fn update(&mut self, update: MessageUpdate) {
        let now = chrono::Utc::now().timestamp_millis();

        if let Some(content) = update.content {
            self.content = content;
            self.edited_at = Some(now);
        }

        if let Some(is_read) = update.is_read {
            self.is_read = is_read;
        }
    }

    /// Mark message as read
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }

    /// Check if message has been edited
    pub fn is_edited(&self) -> bool {
        self.edited_at.is_some()
    }

    /// Get a preview of the message (first 100 chars)
    pub fn preview(&self) -> String {
        if self.content.len() > 100 {
            format!("{}...", &self.content[..100])
        } else {
            self.content.clone()
        }
    }
}
