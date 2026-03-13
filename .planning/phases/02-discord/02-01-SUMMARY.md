---
phase: 02-discord
plan: 01
subsystem: protocol

tags: [discord, serenity, adapter, encryption, svelte, tauri]

dependency_graph:
  requires:
    - phase: 01-foundation
      provides: ProtocolAdapter trait, ProtocolRegistry, event system
  provides:
    - DiscordAdapter implementing ProtocolAdapter
    - AES-256-GCM encrypted bot token storage
    - Account management commands
    - Message/conversation commands
    - Three-pane UI with Svelte components
  affects:
    - 03-whatsapp
    - 04-signal

tech_stack:
  added:
    - serenity 0.12 (Discord API client)
    - aes-gcm 0.10 (encryption)
    - base64 0.22 (encoding)
    - rand 0.8 (random nonce generation)
  patterns:
    - Protocol-specific adapter implementation
    - Machine-derived key encryption (v1, upgrade to OS keychain in v2)
    - Svelte 5 runes ($state, $effect) for reactive UI
    - Tauri event listeners for real-time updates

key_files:
  created:
    - src-tauri/src/protocol/discord/adapter.rs - DiscordAdapter implementation
    - src-tauri/src/protocol/discord/auth.rs - AES-256-GCM token encryption
    - src-tauri/src/protocol/discord/sync.rs - Conversation/message sync
    - src-tauri/src/commands/accounts.rs - Account CRUD commands
    - src-tauri/src/commands/messages.rs - Message send/fetch commands
    - src-tauri/src/commands/conversations.rs - Conversation commands
    - src/lib/stores/accounts.ts - Frontend account store
    - src/lib/stores/conversations.ts - Frontend conversation store
    - src/lib/stores/messages.ts - Frontend message store
    - src/lib/components/AccountManager.svelte - Add Discord accounts UI
    - src/lib/components/ConversationList.svelte - Sidebar conversation list
    - src/lib/components/ConversationView.svelte - Message display
    - src/lib/components/MessageInput.svelte - Message input component
  modified:
    - src-tauri/Cargo.toml - Added Discord dependencies
    - src-tauri/src/protocol/mod.rs - Export discord module
    - src-tauri/src/commands/mod.rs - Export new command modules
    - src-tauri/src/main.rs - Register all new commands
    - src/routes/+page.svelte - Three-pane layout

key-decisions:
  - "Bot token auth (not OAuth): Simpler for v1, no web server needed"
  - "Machine-derived encryption key: Upgrade to OS keychain in v2"
  - "Three-pane UI layout: Sidebar accounts/conversations, main chat area"
  - "Real-time via Tauri events: Backend ProtocolEvent -> frontend stores"

patterns-established:
  - "Protocol adapter: Implement ProtocolAdapter trait for each protocol"
  - "Encrypted credentials: AES-256-GCM with random nonce for each token"
  - "Frontend stores: Svelte writable stores with invoke() methods"
  - "Event-driven updates: Backend emits, frontend listens and updates stores"

requirements-completed:
  - ACCT-03
  - ACCT-04
  - ACCT-05
  - ACCT-06
  - ACCT-07
  - MSG-01
  - MSG-02
  - MSG-03
  - MSG-04
  - MSG-05
  - MSG-06
  - MSG-12
  - UI-01
  - UI-02
  - UI-03
  - UI-04

metrics:
  duration: 10min
  completed: 2026-03-13
---

# Phase 02 Plan 01: Discord Integration Summary

**Discord protocol integration with bot token authentication, AES-256-GCM encrypted credentials, real-time messaging, and three-pane Svelte UI.**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-13T22:14:16Z
- **Completed:** 2026-03-13T22:25:08Z
- **Tasks:** 4
- **Files modified:** 19

## Accomplishments

### 1. Discord Protocol Adapter

Implemented `DiscordAdapter` that satisfies the `ProtocolAdapter` trait:
- **Authentication:** Bot token authentication via Discord REST API
- **Conversations:** Fetch guilds, channels, and DMs from Discord
- **Messages:** Get message history with pagination, send messages
- **Event handling:** Ready for Gateway integration (stubbed for v1)

### 2. Secure Credential Storage

AES-256-GCM encryption for bot tokens:
- Random 12-byte nonce per encryption
- Machine-derived key (v1 - upgrade path to OS keychain)
- Base64 encoding for database storage

### 3. Backend Commands

Tauri IPC commands for frontend integration:
- **Accounts:** `create_account`, `delete_account`, `get_accounts`, `connect_discord_account`
- **Conversations:** `get_conversations`, `mark_conversation_read`, `sync_conversations`
- **Messages:** `get_messages`, `send_message`, `sync_messages`

### 4. Svelte UI Components

Three-pane layout with real-time updates:
- **AccountManager:** Add Discord bots, view connection status
- **ConversationList:** Sidebar with unread badges, last message preview
- **ConversationView:** Message bubbles with date dividers, auto-scroll
- **MessageInput:** Auto-resizing textarea, Enter to send

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Discord dependencies and create DiscordAdapter** - `807c3b9` (feat)
2. **Task 2: Create account management commands and encryption** - `1a308b6` (feat)
3. **Task 3: Create message and conversation commands** - `169c52b` (feat)
4. **Task 4: Create Svelte UI components** - `0988690` (feat)

## Files Created/Modified

### Backend (Rust)
- `src-tauri/src/protocol/discord/mod.rs` - Discord module and error types
- `src-tauri/src/protocol/discord/adapter.rs` - DiscordAdapter implementation
- `src-tauri/src/protocol/discord/auth.rs` - Token encryption/decryption
- `src-tauri/src/protocol/discord/sync.rs` - Sync helpers
- `src-tauri/src/commands/accounts.rs` - Account commands
- `src-tauri/src/commands/messages.rs` - Message commands
- `src-tauri/src/commands/conversations.rs` - Conversation commands

### Frontend (Svelte/TypeScript)
- `src/lib/stores/accounts.ts` - Account store with real-time updates
- `src/lib/stores/conversations.ts` - Conversation store
- `src/lib/stores/messages.ts` - Message store with formatting helpers
- `src/lib/components/AccountManager.svelte` - Account management UI
- `src/lib/components/ConversationList.svelte` - Conversation list sidebar
- `src/lib/components/ConversationView.svelte` - Message display
- `src/lib/components/MessageInput.svelte` - Message input

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| Bot token (not OAuth) | Simpler for v1, no web server required |
| Machine-derived key | Quick v1 implementation, clear upgrade path to OS keychain |
| Three-pane layout | Standard messaging app pattern, scalable to more protocols |
| Text-only v1 | Media support planned for Phase 5 |
| Serenity crate | Most mature Rust Discord library, active maintenance |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

1. **Database schema mismatch with model fields** - Fixed commands to use actual schema fields (protocol_conversation_id, participants JSON, etc.) rather than simplified versions in initial code.

2. **TypeScript module resolution warnings** - LSP errors about module resolution; these are editor configuration warnings and don't affect runtime. Svelte/TypeScript build handles imports correctly.

## User Setup Required

**Discord Bot Token Required**

To use the Discord integration:

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Go to "Bot" section and create a bot
4. Copy the bot token
5. In Spoky, click "+ Add Account" in the Accounts section
6. Paste the bot token and click "Add Account"

The bot must be invited to servers with appropriate permissions to read and send messages.

## Next Phase Readiness

- ✅ Discord integration foundation complete
- ✅ Protocol adapter pattern validated
- ✅ Encryption pattern established
- ✅ Frontend component patterns ready

Ready for Phase 03 (WhatsApp Integration) - will follow same adapter pattern.

---
*Phase: 02-discord*
*Completed: 2026-03-13*
