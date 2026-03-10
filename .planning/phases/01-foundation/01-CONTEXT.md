# Phase 1: Foundation - Context

**Gathered:** 2026-03-10
**Status:** Ready for planning

<domain>
## Phase Boundary

Build the app infrastructure: Tauri shell, SQLite database, unified data models, and protocol abstraction layer. This is the foundation that all subsequent phases build on. No protocol integration or messaging functionality — those come in later phases.

</domain>

<decisions>
## Implementation Decisions

### Project Structure
- Standard Tauri 2.0 layout with Svelte in a sibling folder (not inside src-tauri)
- Single Cargo.toml for v1 — defer workspace/monorepo until complexity warrants it
- Frontend code in `/src` directory at project root
- Rust core in `/src-tauri`

### Database Schema
- Full-featured schema from day one to avoid migrations later:
  - `accounts` — protocol, credentials (encrypted), connection status
  - `conversations` — protocol reference, participants, last message, unread count
  - `messages` — conversation reference, sender, content, timestamp, read status
  - `sync_checkpoints` — per-protocol sync cursor for incremental updates
  - `message_reactions` — emoji reactions (future-ready)
  - `read_receipts` — per-message read status (future-ready)
- Use SQLite via @tauri-apps/plugin-sql
- Migrations managed in Rust

### Protocol Abstraction
- Event-based adapter trait in Rust:
  - Core methods: `connect()`, `disconnect()`, `get_conversations()`, `send_message()`
  - Event callbacks: `on_message(callback)`, `on_connection_change(callback)`
- Each protocol (Discord, WhatsApp, Signal) will implement this trait
- Trait lives in core crate; implementations in separate modules
- Callbacks push events through Tauri IPC to frontend

### Empty State UI
- Setup wizard on first launch
- Guides user through connecting all three protocols (WhatsApp, Signal, Discord)
- Can skip and connect later
- Friendly, welcoming tone — this is for friends and personal use

### Claude's Discretion
- Exact Tauri IPC command names and event names
- Database migration file structure and naming
- Exact trait method signatures (sync vs async, error types)
- Empty state UI styling and copy

</decisions>

<specifics>
## Specific Ideas

- User wants a setup wizard that walks through connecting all three accounts on first launch
- Full schema from day one means fewer migrations when adding reactions/receipts later

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
None — greenfield project.

### Established Patterns
None — this phase establishes the patterns for all future phases.

### Integration Points
- Future phases will implement the ProtocolAdapter trait defined here
- UI components will consume events pushed through the IPC layer
- Database schema will be extended, not redesigned, in later phases

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 01-foundation*
*Context gathered: 2026-03-10*
