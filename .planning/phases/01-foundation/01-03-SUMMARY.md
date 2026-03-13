---
phase: 01-foundation
plan: 03
name: Protocol Adapter Architecture
subsystem: protocol
tags: [protocol, adapter, events, registry, trait]
dependency_graph:
  requires: [01-01, 01-02]
  provides: [02-01]
  affects: []
tech_stack:
  added:
    - async-trait (async trait methods)
    - thiserror (error handling)
  patterns:
    - Trait-based protocol abstraction
    - Event-driven communication
    - Producer-consumer pattern (mpsc channel)
    - Registry pattern for connection management
key_files:
  created:
    - src-tauri/src/protocol/mod.rs
    - src-tauri/src/protocol/adapter.rs
    - src-tauri/src/protocol/events.rs
    - src-tauri/src/protocol/registry.rs
    - src-tauri/src/commands/mod.rs
    - src-tauri/src/commands/protocol.rs
  modified:
    - src-tauri/Cargo.toml
    - src-tauri/src/main.rs
    - src-tauri/capabilities/default.json
decisions:
  - Event flow: Adapter → callback → mpsc channel → Tauri emit → frontend
  - Thread-safe registry using Arc<RwLock<HashMap>>
  - Connection status: Disconnected, Connecting, Connected, Error
  - ProtocolEvent enum with tagged serialization for frontend
metrics:
  duration: 12
  completed_date: "2026-03-11"
---

# Phase 01 Plan 03: Protocol Adapter Architecture Summary

**One-liner:** Implemented trait-based protocol abstraction with event-driven communication and connection registry.

## What Was Built

### ProtocolAdapter Trait

Core interface that all protocol implementations (Discord, WhatsApp, Signal) must satisfy:

```rust
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    fn protocol(&self) -> Protocol;
    async fn connect(&mut self, account: &Account) -> Result<ConnectionStatus, ProtocolError>;
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;
    fn connection_status(&self) -> ConnectionStatus;
    async fn get_conversations(&self) -> Result<Vec<Conversation>, ProtocolError>;
    async fn get_messages(&self, conversation_id: &str, before: Option<i64>, limit: usize) -> Result<Vec<Message>, ProtocolError>;
    async fn send_message(&self, conversation_id: &str, content: &str) -> Result<Message, ProtocolError>;
    fn on_event(&mut self, callback: Box<dyn Fn(ProtocolEvent) + Send + Sync>);
}
```

### Event System

Events flow from protocols to frontend through a decoupled channel:

**Event Types:**
- `ProtocolEvent::ConnectionChanged` - Connection status updates
- `ProtocolEvent::MessageReceived` - New message from protocol
- `ProtocolEvent::ConversationUpdated` - New or updated conversation
- `ProtocolEvent::Error` - Protocol errors

**Flow:**
```
ProtocolAdapter → on_event callback → mpsc channel → Tauri emit → Frontend
```

### ProtocolRegistry

Thread-safe connection manager with:
- `connect_account()` - Create adapter, set callback, connect, store
- `disconnect_account()` - Remove and disconnect adapter
- `connection_status()` - Query adapter status
- `connected_accounts()` - List active connections
- `disconnect_all()` - Graceful shutdown

Uses `Arc<RwLock<HashMap<String, Box<dyn ProtocolAdapter>>>>` for concurrent access.

### Tauri Commands

Frontend can call:
- `connect_account(account_id)` - Connect a protocol account
- `disconnect_account(account_id)` - Disconnect an account
- `get_connection_status(account_id)` - Check connection state
- `get_connected_accounts()` - List all connected accounts
- `test_protocol_events()` - Test event emission

### StubAdapter

Test implementation of ProtocolAdapter that:
- Simulates connection lifecycle
- Emits events to validate event flow
- Returns empty data for conversations/messages
- Useful for testing UI without real protocols

## Key Features

| Feature | Implementation |
|---------|----------------|
| **Async Trait** | async-trait crate for async methods in traits |
| **Thread Safety** | Send + Sync bounds on ProtocolAdapter |
| **Object Safety** | Box<dyn ProtocolAdapter> for storage |
| **Event Decoupling** | mpsc channel prevents blocking protocol threads |
| **Error Handling** | ProtocolError enum with thiserror |
| **Connection States** | 4-state lifecycle: Disconnected → Connecting → Connected/Error |

## Integration

main.rs setup:
1. Create mpsc channel for events
2. Create ProtocolRegistry with channel sender
3. Add registry to Tauri state
4. Spawn event forwarder task (channel → Tauri emit)
5. Register IPC commands

Frontend listens via:
```javascript
listen('protocol:event', (event) => {
  // Handle ProtocolEvent
});
```

## Architecture Benefits

- **Protocol Isolation** - Each protocol implements the same trait
- **Event-Driven** - No polling, real-time updates via callbacks
- **Thread-Safe** - Multiple protocols can run concurrently
- **Extensible** - Easy to add new protocols by implementing trait
- **Testable** - StubAdapter enables testing without real services

## Next Steps

Phase 02 Discord Integration:
- Implement DiscordAdapter (Discord-specific ProtocolAdapter)
- Add Discord bot token authentication
- Sync conversations and messages from Discord API
- Send messages to Discord channels

## Commits

- `1c109da`: feat(01-03): implement protocol adapter architecture

## Self-Check: PASSED

- [x] ProtocolAdapter trait with all required methods
- [x] ProtocolEvent enum with 4 event types
- [x] ProtocolRegistry with thread-safe access
- [x] Event channel bridging adapters to frontend
- [x] Tauri commands for protocol operations
- [x] StubAdapter for testing
- [x] Event:default permission added
- [x] All modules integrated in main.rs
- [x] Dependencies added (async-trait, thiserror)
- [x] Committed to git
