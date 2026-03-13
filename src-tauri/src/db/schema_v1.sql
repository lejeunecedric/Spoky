-- Migration v1: Initial Schema
-- Creates all tables and indexes for Spoky v1

-- Accounts: store connected protocol accounts
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    protocol TEXT NOT NULL, -- 'discord', 'whatsapp', 'signal'
    credentials TEXT NOT NULL, -- encrypted credentials JSON
    display_name TEXT,
    connection_status TEXT NOT NULL DEFAULT 'disconnected', -- 'connected', 'connecting', 'disconnected', 'error'
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Conversations: unified conversation list across protocols
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
);

-- Messages: message content and metadata
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    protocol_message_id TEXT, -- original ID from protocol
    sender_id TEXT NOT NULL,
    sender_name TEXT,
    content TEXT NOT NULL,
    content_type TEXT DEFAULT 'text', -- 'text', 'image', 'file'
    is_from_me INTEGER DEFAULT 0,
    is_read INTEGER DEFAULT 0,
    reply_to_message_id TEXT,
    sent_at INTEGER NOT NULL,
    received_at INTEGER,
    edited_at INTEGER,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

-- Sync checkpoints: per-protocol sync state for incremental updates
CREATE TABLE IF NOT EXISTS sync_checkpoints (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    last_sync_at INTEGER,
    cursor TEXT, -- protocol-specific sync cursor
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

-- Message reactions: emoji reactions (future-ready for v1.x)
CREATE TABLE IF NOT EXISTS message_reactions (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    sender_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);

-- Read receipts: per-message read status (future-ready for v1.x)
CREATE TABLE IF NOT EXISTS read_receipts (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    reader_id TEXT NOT NULL,
    read_at INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_sent_at ON messages(sent_at);
CREATE INDEX IF NOT EXISTS idx_conversations_account_id ON conversations(account_id);
CREATE INDEX IF NOT EXISTS idx_conversations_last_message_at ON conversations(last_message_at DESC);
