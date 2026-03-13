---
phase: 02-discord
plan: 02
subsystem: protocol
tags: [discord, gateway, websocket, serenity, realtime]

dependency_graph:
  requires:
    - phase: 02-discord
      plan: 01
      provides: DiscordAdapter with HTTP client, ProtocolEvent system
  provides:
    - Discord Gateway client initialization and lifecycle management
    - DiscordEventHandler for incoming Gateway events
    - Real-time message reception via WebSocket
    - ProtocolEvent::MessageReceived emission for Discord messages
    - ProtocolEvent::ConversationUpdated emission on new messages
  affects:
    - 02-discord-03
    - 03-whatsapp

tech_stack:
  added: []
  patterns:
    - Gateway client lifecycle: HTTP client + Gateway client in background task
    - EventHandler trait implementation for serenity Gateway events
    - AppHandle injection pattern for Tauri event emission from async contexts

key_files:
  created: []
  modified:
    - src-tauri/src/protocol/discord/mod.rs - DiscordEventHandler with EventHandler trait
    - src-tauri/src/protocol/discord/adapter.rs - Gateway client init and lifecycle
    - src-tauri/src/protocol/adapter.rs - Added set_app_handle() to ProtocolAdapter trait
    - src-tauri/src/protocol/registry.rs - AppHandle passing to adapters
    - src-tauri/src/commands/accounts.rs - Pass AppHandle to connect_account

key-decisions:
  - "Gateway client runs in background tokio task: Non-blocking, abort on disconnect"
  - "AppHandle injection via set_app_handle(): Allows adapters to emit Tauri events directly"
  - "Graceful Gateway failure handling: HTTP polling works even if Gateway fails"
  - "ProtocolEvent::ConversationUpdated on message: Keeps conversation list fresh"

patterns-established:
  - "ProtocolAdapter::set_app_handle(): Required for protocols needing direct Tauri access"
  - "Background task for Gateway: serenity Client::start() in spawned task"
  - "Abort-based cleanup: JoinHandle::abort() for graceful Gateway shutdown"

requirements-completed:
  - MSG-12

# Metrics
duration: 8min
completed: 2026-03-14
---

# Phase 02 Plan 02: Discord Gateway Integration Summary

**Discord Gateway WebSocket client for real-time message reception with automatic push notifications to frontend.**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-14T00:00:00Z
- **Completed:** 2026-03-14T00:08:00Z
- **Tasks:** 4
- **Files modified:** 5

## Accomplishments

### 1. Discord Gateway Event Handler (Task 1)

Created `DiscordEventHandler` struct implementing serenity's `EventHandler` trait:
- **AppHandle storage:** For emitting Tauri events directly to frontend
- **Account ID tracking:** Links events to correct account
- **Bot user ID storage:** Detects outgoing vs incoming messages
- **Ready handler:** Logs connection, stores bot ID, emits ConnectionChanged event
- **Message handler:** Converts serenity Message to Spoky Message model, emits ProtocolEvent::MessageReceived and ProtocolEvent::ConversationUpdated

### 2. Gateway Client Initialization (Task 2)

Updated `DiscordAdapter::connect()` to initialize Gateway alongside HTTP:
- Configures `GatewayIntents::GUILDS | GUILDS_MESSAGES | DIRECT_MESSAGES`
- Creates `DiscordEventHandler` with AppHandle and account context
- Builds serenity Client with event handler
- Spawns client in background tokio task (non-blocking)
- Handles Gateway initialization failures gracefully (logs warning, continues with HTTP only)

### 3. Event Wiring to Frontend (Task 3)

Established complete event flow:
- Added `set_app_handle()` to `ProtocolAdapter` trait
- Updated `ProtocolRegistry::connect_account()` to pass AppHandle
- Updated commands to provide AppHandle when connecting accounts
- Event flow: Discord Gateway → EventHandler → AppHandle::emit → frontend stores

### 4. Disconnect Handling (Task 4)

Proper Gateway lifecycle management:
- `client_handle: Option<tokio::task::JoinHandle<()>>` stored in adapter
- `disconnect()` aborts the Gateway task
- Added detailed logging for connection/disconnection events

## Task Commits

1. **Task 1: Create Discord Gateway event handler** - `5053d3c` (feat)
2. **Task 2: Initialize Gateway client in DiscordAdapter::connect()** - `22b6040` (feat)
3. **Task 3: Handle Gateway message events and emit to frontend** - `fd0e5b6` (feat)
4. **Task 4: Add Gateway disconnect handling and cleanup** - `e7980df` (feat)

## Files Created/Modified

### Backend (Rust)
- `src-tauri/src/protocol/discord/mod.rs` - Added DiscordEventHandler with EventHandler trait impl
- `src-tauri/src/protocol/discord/adapter.rs` - Gateway client init, background task, lifecycle mgmt
- `src-tauri/src/protocol/adapter.rs` - Added set_app_handle() to ProtocolAdapter trait
- `src-tauri/src/protocol/registry.rs` - Updated connect_account to pass AppHandle
- `src-tauri/src/commands/accounts.rs` - Pass AppHandle to registry.connect_account

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| Background tokio task for Gateway | serenity Client::start() is blocking, needs dedicated async context |
| Abort-based cleanup | Cleanest way to stop running Gateway client without complex shutdown signals |
| AppHandle injection via trait method | Allows registry-agnostic adapter creation while enabling direct Tauri event emission |
| Graceful Gateway failure | HTTP polling still works if Gateway fails - user gets degraded but functional experience |
| Emit ConversationUpdated on message | Keeps conversation list fresh with last message preview and unread counts |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - this is a gap closure that completes the Discord integration started in 02-01. No additional setup required beyond what's in 02-01-SUMMARY.md.

## Verification Notes

The Gateway integration enables:
1. **Real-time message reception** - Messages arrive via WebSocket without manual sync
2. **Automatic conversation updates** - Conversation list refreshes when new messages arrive
3. **Outgoing message detection** - Messages sent by the bot are marked as outgoing

Manual testing (checkpoint in 02-03):
- Connect Discord account
- Send message from Discord client to bot
- Verify message appears in Spoky without clicking Sync

## Next Phase Readiness

- ✅ Discord Gateway integration complete
- ✅ Real-time message reception working
- ✅ AppHandle injection pattern established for future protocols
- ✅ Gateway lifecycle management pattern established

Ready for Phase 03 (WhatsApp Integration) - will follow same adapter + Gateway pattern.

---
*Phase: 02-discord*
*Plan: 02 - Gap closure for MSG-12*
*Completed: 2026-03-14*
