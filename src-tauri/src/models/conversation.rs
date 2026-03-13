use crate::models::Protocol;
use serde::{Deserialize, Serialize};

/// Conversation data model representing a chat thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub protocol: Protocol,
    pub account_id: String,
    pub protocol_conversation_id: String, // original ID from protocol
    pub title: Option<String>,            // group name or contact name
    pub participants: Vec<String>,        // participant IDs as JSON array
    pub last_message_id: Option<String>,
    pub last_message_preview: Option<String>,
    pub last_message_at: Option<i64>, // Unix timestamp in milliseconds
    pub unread_count: i32,
    pub created_at: i64, // Unix timestamp in milliseconds
    pub updated_at: i64, // Unix timestamp in milliseconds
}

/// New conversation data for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConversation {
    pub protocol: Protocol,
    pub account_id: String,
    pub protocol_conversation_id: String,
    pub title: Option<String>,
    pub participants: Vec<String>,
}

/// Conversation update data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConversationUpdate {
    pub title: Option<Option<String>>,
    pub participants: Option<Vec<String>>,
    pub last_message_id: Option<Option<String>>,
    pub last_message_preview: Option<Option<String>>,
    pub last_message_at: Option<Option<i64>>,
    pub unread_count: Option<i32>,
}

impl Conversation {
    /// Create a new conversation with generated ID and timestamps
    pub fn create(new: NewConversation) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            protocol: new.protocol,
            account_id: new.account_id,
            protocol_conversation_id: new.protocol_conversation_id,
            title: new.title,
            participants: new.participants,
            last_message_id: None,
            last_message_preview: None,
            last_message_at: None,
            unread_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update conversation fields
    pub fn update(&mut self, update: ConversationUpdate) {
        let now = chrono::Utc::now().timestamp_millis();

        if let Some(title) = update.title {
            self.title = title;
        }

        if let Some(participants) = update.participants {
            self.participants = participants;
        }

        if let Some(last_message_id) = update.last_message_id {
            self.last_message_id = last_message_id;
        }

        if let Some(preview) = update.last_message_preview {
            self.last_message_preview = preview;
        }

        if let Some(last_message_at) = update.last_message_at {
            self.last_message_at = last_message_at;
        }

        if let Some(unread_count) = update.unread_count {
            self.unread_count = unread_count;
        }

        self.updated_at = now;
    }

    /// Increment unread count
    pub fn increment_unread(&mut self) {
        self.unread_count += 1;
        self.updated_at = chrono::Utc::now().timestamp_millis();
    }

    /// Mark as read (reset unread count)
    pub fn mark_as_read(&mut self) {
        self.unread_count = 0;
        self.updated_at = chrono::Utc::now().timestamp_millis();
    }

    /// Update last message info
    pub fn update_last_message(&mut self, message_id: String, preview: String, timestamp: i64) {
        self.last_message_id = Some(message_id);
        self.last_message_preview = Some(preview);
        self.last_message_at = Some(timestamp);
        self.updated_at = chrono::Utc::now().timestamp_millis();
    }
}
