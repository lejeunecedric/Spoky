---
phase: 02-discord
plan: 03
subsystem: ui

tags: [discord, reply, new-conversation, dm, ui, svelte]

dependency_graph:
  requires:
    - phase: 02-discord
      plan: 02
      provides: Discord Gateway integration, ProtocolEvent system
  provides:
    - Reply button on messages in ConversationView
    - Reply preview UI in MessageInput with cancel option
    - reply_to_message_id support in send_message command
    - create_dm_conversation backend command
    - New conversation modal in ConversationList
    - User ID input for creating Discord DMs
  affects:
    - 03-whatsapp
    - 04-signal

tech_stack:
  added: []
  patterns:
    - "Reply UI pattern: replyTo state passed from parent, cleared on send or cancel"
    - "Hover actions: reply button appears on message hover"
    - "Modal pattern: backdrop click to close, stopPropagation on content"

key_files:
  created: []
  modified:
    - src-tauri/src/protocol/adapter.rs - Added reply_to_message_id and create_dm_conversation to trait
    - src-tauri/src/protocol/discord/adapter.rs - Implemented reply logic and DM creation
    - src-tauri/src/commands/messages.rs - Updated send_message command with reply parameter
    - src-tauri/src/commands/conversations.rs - Added create_dm_conversation command
    - src-tauri/src/main.rs - Registered create_dm_conversation command
    - src/lib/components/MessageInput.svelte - Added reply UI with preview and cancel
    - src/lib/components/ConversationView.svelte - Added reply button on message hover
    - src/lib/components/ConversationList.svelte - Added new conversation modal
    - src/lib/stores/messages.ts - Updated send() to accept replyToMessageId
    - src/routes/+page.svelte - Wired up reply state between components

key-decisions:
  - "Reply via reply_to_message_id: Discord API supports message references for replies"
  - "Discord DM creation via create_private_channel: Serenity provides this API"
  - "Modal for new conversations: Simple user ID input for Discord v1"
  - "Escape key cancels reply: Keyboard shortcut for quick dismissal"

patterns-established:
  - "Reply state management: Lift state to parent (+page.svelte), pass via props"
  - "Message action buttons: Show on hover for cleaner UI"
  - "Modal form pattern: Form fields, validation, error display, loading state"

requirements-completed:
  - MSG-05
  - MSG-06

metrics:
  duration: 5min
  completed: 2026-03-14
---

# Phase 02 Plan 03: Gap Closure for MSG-05, MSG-06 Summary

**Reply UI with message preview and New Conversation modal for creating Discord DMs.**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-13T23:59:48Z
- **Completed:** 2026-03-14T00:05:45Z
- **Tasks:** 4
- **Files modified:** 10

## Accomplishments

### 1. Reply Support Backend (Task 1)

Extended the full message sending stack to support replies:
- **ProtocolAdapter trait:** Added `reply_to_message_id: Option<&str>` parameter to `send_message()`
- **DiscordAdapter:** Implemented reply logic using serenity's `CreateMessage::reference_message()`
- **send_message command:** Accepts optional `reply_to_message_id` parameter and passes to adapter

### 2. Reply UI Components (Task 2)

Created complete reply UI flow:
- **MessageInput.svelte:** Added `replyingTo` and `onCancelReply` props
  - Reply preview shows sender name and truncated message content
  - Cancel button (×) and Escape key to dismiss
  - Visual indicator with left blue border
- **ConversationView.svelte:** Added reply button on message hover
  - Hover state tracking for showing actions
  - `onReply` callback prop for parent communication
  - Reply button styled to match message bubble
- **+page.svelte:** Wired reply state between components
  - `replyingTo` state managed at page level
  - `handleReply()` and `cancelReply()` functions
- **messages.ts:** Updated `send()` to accept optional `replyToMessageId`

### 3. Create DM Backend (Task 3)

Added backend support for new Discord DM conversations:
- **ProtocolAdapter trait:** Added `create_dm_conversation()` method
- **DiscordAdapter:** Implemented using `Http::create_private_channel()`
  - Parses user ID as u64
  - Creates DM channel via Discord API
  - Converts to Spoky Conversation model
- **create_dm_conversation command:** 
  - Gets adapter from registry
  - Creates DM via protocol
  - Saves to database with conflict handling
  - Returns complete Conversation object

### 4. New Conversation UI (Task 4)

Added UI for creating new DM conversations:
- **ConversationList header:** + button opens modal
- **Modal design:**
  - Discord account selection dropdown
  - User ID input field with help text
  - Error message display
  - Cancel/Create buttons with loading state
- **Integration:**
  - Calls `create_dm_conversation` command
  - Auto-selects new conversation on success
  - Refreshes conversation list

## Task Commits

1. **Task 1: Add reply_to_message_id to send_message command** - `c0bcffc` (feat)
2. **Task 2: Create reply UI components** - `c768375` (feat)
3. **Task 3: Create create_dm_conversation backend command** - `daf15e5` (feat)
4. **Task 4: Create New Conversation UI** - `158a968` (feat)

## Files Created/Modified

### Backend (Rust)
- `src-tauri/src/protocol/adapter.rs` - Extended trait with reply and DM creation methods
- `src-tauri/src/protocol/discord/adapter.rs` - Implemented reply and DM creation logic
- `src-tauri/src/commands/messages.rs` - Updated send_message command signature
- `src-tauri/src/commands/conversations.rs` - Added create_dm_conversation command
- `src-tauri/src/main.rs` - Registered new command in invoke handler

### Frontend (Svelte/TypeScript)
- `src/lib/components/MessageInput.svelte` - Reply preview UI and state handling
- `src/lib/components/ConversationView.svelte` - Reply button on message hover
- `src/lib/components/ConversationList.svelte` - New conversation modal
- `src/lib/stores/messages.ts` - Updated send() with reply support
- `src/routes/+page.svelte` - Wired reply state between components

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| Pass reply_to_message_id as Option<&str> | Matches Discord API optional reference pattern |
| Use serenity's MessageReference for replies | Native Discord reply threading support |
| Lift reply state to page level | Allows coordination between ConversationView and MessageInput |
| Show reply button on hover | Cleaner UI, doesn't clutter message display |
| User ID input for new DMs (v1) | Simplest approach, Discord doesn't provide user search API for bots |
| Auto-select new conversation | Immediate feedback, user can start messaging right away |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

**None** - this completes the Discord integration. No additional setup beyond what's in 02-01-SUMMARY.md.

## Verification Notes

### Reply Flow
1. Open a conversation
2. Hover over any message → Reply button appears
3. Click Reply → Reply preview shows in MessageInput
4. Type message and send → Message sent as reply to original
5. Press Escape or click × → Cancel reply mode

### New Conversation Flow
1. Click + button in ConversationList header
2. Select Discord account (or pre-selected if only one)
3. Enter Discord User ID
4. Click Create → New DM appears in list and is selected
5. Can immediately send messages to new conversation

## Next Phase Readiness

- ✅ Reply functionality complete (MSG-05)
- ✅ New conversation creation complete (MSG-06)
- ✅ Phase 2 Discord integration 100% complete

Ready for Phase 03 (WhatsApp Integration) - will follow same adapter pattern with protocol-specific implementations.

---
*Phase: 02-discord*
*Plan: 03 - Gap closure for MSG-05, MSG-06*
*Completed: 2026-03-14*
