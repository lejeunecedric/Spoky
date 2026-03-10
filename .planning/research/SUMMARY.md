# Project Research Summary

**Project:** Spoky
**Domain:** Multi-protocol desktop messenger (WhatsApp, Signal, Discord)
**Researched:** 2026-03-10
**Confidence:** MEDIUM-HIGH

## Executive Summary

Spoky is a unified desktop messenger that bridges WhatsApp, Signal, and Discord into a single lightweight application. Experts in this domain (Beeper, mautrix bridges) use protocol bridge architectures rather than web-wrappers to preserve end-to-end encryption and minimize resource usage. The recommended approach uses Tauri 2.0 for a lightweight desktop shell (~600KB vs Electron's ~150MB), Svelte 5 for a minimal frontend footprint, and sidecar processes for protocol bridges.

The primary risks are legal/tos-related: Discord prohibits self-bots, and WhatsApp/Signal have no official APIs for personal accounts. Mitigation requires using well-maintained community bridges (whatsapp-web.js, signal-cli, discord.js), aggressive rate limiting, and clear user documentation about risks. Technically, the biggest challenge is multi-protocol state synchronization—each protocol has different connection semantics that must be abstracted behind a unified interface from day one.

## Key Findings

### Recommended Stack

The stack prioritizes minimal resource footprint while enabling multi-protocol bridging. Tauri + Rust provides a secure, lightweight foundation. Svelte 5 compiles away at build time for minimal JavaScript overhead. Protocol bridges run as sidecar processes (Node.js binaries) communicating via JSON-RPC over stdin/stdout.

**Core technologies:**
- **Tauri 2.0+** — Desktop framework — uses system webview for <600KB apps, Rust core for safety
- **Svelte 5.25+** — UI framework — smallest bundle size of major frameworks, compile-time optimization
- **TypeScript 5.x** — Type system — essential for protocol message handling
- **Rust 1.77+** — Backend core — Tauri native, memory-safe, handles IPC and state
- **SQLite (via @tauri-apps/plugin-sql)** — Message storage — local-first, encrypted storage for messages
- **Tailwind CSS 4.x** — Styling — zero-runtime, dark mode built-in

**Protocol bridges (sidecars):**
- **whatsapp-web.js** — WhatsApp bridge — only maintained WhatsApp Web bridge (21K stars)
- **signal-cli** — Signal bridge — official libsignal wrapper, JSON-RPC interface
- **discord.js** — Discord bridge — official Discord library, 100% API coverage

### Expected Features

**Must have (table stakes) — v1:**
- Account connection (QR/Token auth for each protocol) — users cannot use the app without connecting
- Unified inbox — single conversation list from all three protocols
- Read/send text messages — basic messaging functionality
- Basic notifications — desktop alerts for new messages
- Protocol badges — visual indicator of which network each conversation is on
- Lightweight footprint — must be lighter than running 3 separate apps

**Should have (competitive) — v1.x:**
- Basic media support — images and files (send/receive)
- Read receipts — show when messages are read
- Typing indicators — real-time feedback
- Search within conversation — find past messages
- Message history persistence — store locally between sessions

**Defer (v2+):**
- Cross-network unified search — requires full message index
- Smart inbox organization — tags/folders spanning protocols
- Message scheduling — requires queue system
- Multi-account per protocol — auth complexity
- Voice/video calling — extremely complex bridging

### Architecture Approach

The architecture uses a **Protocol Adapter pattern** where each messaging service implements a common Rust trait. This abstraction allows the frontend to work with any protocol uniformly. Messages flow from protocol adapters through a unified message model, into SQLite storage, then to the frontend via Tauri IPC events.

**Major components:**
1. **WebView Frontend (Svelte)** — UI rendering, conversation views, settings; communicates via Tauri IPC
2. **Rust Core (Tauri)** — App orchestration, state management, protocol coordination, sidecar management
3. **Protocol Adapter Layer** — Common trait + registry for Discord/WhatsApp/Signal adapters
4. **Sidecar Processes** — Node.js binaries for WhatsApp (whatsapp-web.js), Signal (signal-cli), Discord (discord.js)
5. **Message Store (SQLite)** — Persistent storage for messages, conversations, account configs

### Critical Pitfalls

1. **Discord self-bot account termination** — Discord prohibits automating user accounts. Use Bot API only (accepts DM limitations) or document that Discord may be read-only/limited in v1.
2. **WhatsApp/Signal ban risk** — No official APIs for personal accounts. Use well-maintained community bridges, implement rate limiting, test with secondary accounts, document user risk.
3. **Multi-protocol state synchronization hell** — Each protocol has different connection semantics. Design protocol abstraction layer FIRST with granular connection states and message deduplication.
4. **Tauri IPC serialization blocking** — Large payloads freeze the UI. Stream data with pagination, store in SQLite and pass IDs through IPC, never pass media as base64 blobs.
5. **Protocol library abandonment** — Third-party bridges may break when protocols change. Design abstraction layer for library swapping, monitor GitHub issues, have fallback messaging.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Core Infrastructure
**Rationale:** Foundation required before any protocol work. Models and storage must exist before bridges can store messages.
**Delivers:** Working Tauri app with IPC, database, and unified data models
**Addresses:** Multi-protocol state sync hell pitfall (design protocol abstraction layer first)
**Tasks:** Tauri setup, unified Message/Conversation models, SQLite with migrations, basic Tauri commands

### Phase 2: Discord Protocol (Easiest First)
**Rationale:** Discord has the best-documented API and discord.js is mature. Get one protocol working before tackling WhatsApp's reverse-engineered bridge.
**Delivers:** Full Discord messaging (read/send) with real-time events
**Uses:** discord.js sidecar, Protocol Adapter trait
**Implements:** First protocol adapter, validates architecture patterns
**Avoids:** Protocol library abandonment (most stable bridge first)

### Phase 3: UI Foundation
**Rationale:** Need working protocol to build UI against. Avoid building UI for theoretical features.
**Delivers:** Conversation list, message view, send message UI, real-time updates
**Uses:** Svelte components, Tauri events for push updates
**Avoids:** WebView memory leaks (implement virtual scrolling from start)

### Phase 4: WhatsApp Protocol
**Rationale:** Most complex bridge due to reverse-engineered protocol. Requires QR code flow and careful rate limiting.
**Delivers:** WhatsApp messaging with QR code authentication
**Uses:** whatsapp-web.js sidecar, IPC for QR display
**Addresses:** WhatsApp ban risk pitfall (implement rate limiting, document risks)

### Phase 5: Signal Protocol
**Rationale:** Signal bridge is moderately complex. Links as secondary device like Signal Desktop.
**Delivers:** Signal messaging with device linking flow
**Uses:** signal-cli sidecar (GraalVM native preferred for smaller footprint)
**Addresses:** Signal third-party access pitfall

### Phase 6: Polish & Notifications
**Rationale:** Notifications and error handling require all protocols working.
**Delivers:** Desktop notifications, connection error handling, background sync
**Uses:** Tauri notification plugin, proper error states
**Avoids:** Reconnection storm pitfall (exponential backoff)

### Phase Ordering Rationale

- **Dependencies drive order:** Models and storage must exist before adapters can persist messages
- **Risk reduction:** Start with most stable protocol (Discord) to validate architecture before tackling fragile bridges (WhatsApp)
- **UI follows backend:** Build UI against working protocol rather than mock data
- **Pitfall mitigation:** Protocol abstraction layer designed in Phase 1 prevents state sync hell in later phases

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2 (Discord):** Discord Bot API limitations for DM access need clarification — can bots receive DMs from users who haven't messaged first?
- **Phase 4 (WhatsApp):** whatsapp-web.js multi-device support and QR flow needs hands-on validation
- **Phase 5 (Signal):** signal-cli GraalVM native build process needs documentation review

Phases with standard patterns (skip research-phase):
- **Phase 1 (Core Infrastructure):** Well-documented Tauri/SvelteKit patterns
- **Phase 3 (UI Foundation):** Standard Svelte component patterns, virtual scrolling is well-documented

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Tauri + SvelteKit integration verified from official docs; sidecar pattern documented |
| Features | HIGH | Feature landscape well-documented via Beeper and mautrix projects |
| Architecture | MEDIUM | Protocol adapter pattern is sound, but multi-protocol integration has limited examples |
| Pitfalls | HIGH | Discord ToS and WhatsApp ban risks are well-documented |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **Stack discrepancy:** ARCHITECTURE.md mentions Rust-native options (twilight for Discord, presage for Signal) while STACK.md recommends Node.js sidecars for all three. Recommend STACK.md approach (sidecars) for consistency and maintainability, but flag for architecture decision during planning.
- **Discord Bot API limitations:** Bots cannot initiate DMs. Need to determine if this is acceptable for v1 or if Discord should be limited to server channels.
- **WhatsApp rate limits:** Meta doesn't publish official rate limits for unofficial APIs. Need empirical testing during implementation.

## Sources

### Primary (HIGH confidence)
- **Tauri 2.0 Official Docs** — https://v2.tauri.app/ — architecture, IPC, sidecar patterns
- **SvelteKit Tauri Integration** — https://v2.tauri.app/start/frontend/sveltekit/ — SPA setup
- **Discord Self-Bots Policy** — https://support.discord.com/hc/en-us/articles/115002192352 — prohibition on user account automation
- **Beeper FAQ & Help Center** — https://beeper.com/faq, https://help.beeper.com — feature documentation for unified messenger

### Secondary (MEDIUM confidence)
- **whatsapp-web.js GitHub** — https://github.com/pedroslopez/whatsapp-web.js — WhatsApp bridge implementation
- **signal-cli GitHub** — https://github.com/AsamK/signal-cli — Signal bridge implementation
- **discord.js Docs** — https://discord.js.org — Discord library documentation
- **mautrix bridges** — https://github.com/mautrix/whatsapp, signal, discord — reference implementations for multi-protocol bridging

### Tertiary (LOW confidence)
- **WhatsApp ban risk** — Inferred from community reports; Meta doesn't document publicly
- **Signal third-party access** — Terms of Service is general; specific enforcement unclear

---
*Research completed: 2026-03-10*
*Ready for roadmap: yes*
