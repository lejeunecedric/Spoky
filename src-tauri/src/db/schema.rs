/// Database schema definitions for Spoky
///
/// All tables use TEXT for UUIDs and INTEGER for timestamps (Unix epoch in milliseconds)
/// BOOLEAN is stored as INTEGER (0 = false, 1 = true)

/// Accounts: store connected protocol accounts
pub const CREATE_ACCOUNTS_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    protocol TEXT NOT NULL, -- 'discord', 'whatsapp', 'signal'
    credentials TEXT NOT NULL, -- encrypted credentials JSON
    display_name TEXT,
    connection_status TEXT NOT NULL DEFAULT 'disconnected', -- 'connected', 'connecting', 'disconnected', 'error'
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
  )
"#;

/// Conversations: unified conversation list across protocols
pub const CREATE_CONVERSATIONS_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    protocol TEXT NOT NULL,
    account_id TEXT NOT NULL,
    protocol_conversation_id TEXT NOT NULL, -- original ID from protocol
    title TEXT, -- group name or contact name
    participants TEXT, -- JSON array of participant IDs
    last_message_id TEXT,
    last_message_preview TEXT,
    last_message_at INTEGER,
    unread_count INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
  )
"#;

/// Messages: message content and metadata
pub const CREATE_MESSAGES_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    protocol_message_id TEXT, -- original ID from protocol
    sender_id TEXT NOT NULL,
    sender_name TEXT,
    content TEXT NOT NULL,
    content_type TEXT DEFAULT 'text', -- 'text', 'image', 'file'
    is_from_me BOOLEAN DEFAULT 0,
    is_read BOOLEAN DEFAULT 0,
    reply_to_message_id TEXT,
    sent_at INTEGER NOT NULL,
    received_at INTEGER,
    edited_at INTEGER,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
  )
"#;

/// Sync checkpoints: per-protocol sync state for incremental updates
pub const CREATE_SYNC_CHECKPOINTS_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS sync_checkpoints (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    last_sync_at INTEGER,
    cursor TEXT, -- protocol-specific sync cursor
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
  )
"#;

/// Message reactions: emoji reactions (future-ready for v1.x)
pub const CREATE_MESSAGE_REACTIONS_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS message_reactions (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    sender_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
  )
"#;

/// Read receipts: per-message read status (future-ready for v1.x)
pub const CREATE_READ_RECEIPTS_TABLE: &str = r#"
  CREATE TABLE IF NOT EXISTS read_receipts (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    reader_id TEXT NOT NULL,
    read_at INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
  )
"#;

// Performance indexes

/// Index for loading conversation history efficiently
pub const IDX_MESSAGES_CONVERSATION_ID: &str = r#"
  CREATE INDEX IF NOT EXISTS idx_messages_conversation_id 
  ON messages(conversation_id)
"#;

/// Index for ordering messages by time
pub const IDX_MESSAGES_SENT_AT: &str = r#"
  CREATE INDEX IF NOT EXISTS idx_messages_sent_at 
  ON messages(sent_at)
"#;

/// Index for filtering conversations by account
pub const IDX_CONVERSATIONS_ACCOUNT_ID: &str = r#"
  CREATE INDEX IF NOT EXISTS idx_conversations_account_id 
  ON conversations(account_id)
"#;

/// Index for sorting conversation list by last message
pub const IDX_CONVERSATIONS_LAST_MESSAGE_AT: &str = r#"
  CREATE INDEX IF NOT EXISTS idx_conversations_last_message_at 
  ON conversations(last_message_at DESC)
"#;

/// All table creation statements in order of dependencies
pub const ALL_TABLES: &[&str] = &[
    CREATE_ACCOUNTS_TABLE,
    CREATE_CONVERSATIONS_TABLE,
    CREATE_MESSAGES_TABLE,
    CREATE_SYNC_CHECKPOINTS_TABLE,
    CREATE_MESSAGE_REACTIONS_TABLE,
    CREATE_READ_RECEIPTS_TABLE,
];

/// All index creation statements
pub const ALL_INDEXES: &[&str] = &[
    IDX_MESSAGES_CONVERSATION_ID,
    IDX_MESSAGES_SENT_AT,
    IDX_CONVERSATIONS_ACCOUNT_ID,
    IDX_CONVERSATIONS_LAST_MESSAGE_AT,
];
