# Feature Research

**Domain:** Unified messenger / multi-protocol chat app (WhatsApp, Signal, Discord bridging)
**Researched:** 2026-03-10
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Unified inbox | Core value proposition - one place for all chats | MEDIUM | Single list showing conversations from all protocols |
| Send/receive text messages | Basic messaging functionality | HIGH | Each protocol requires different bridge implementation |
| Read receipts | Users expect to know if messages were seen | MEDIUM | Supported by all three protocols but handled differently |
| Typing indicators | Real-time feedback expected in modern chat | MEDIUM | WebSocket/event-based; supported by all protocols |
| Notifications | Users need to know when new messages arrive | MEDIUM | Desktop notifications with protocol attribution |
| Basic media support | Images and files are standard | MEDIUM | WhatsApp/Signal: E2EE media; Discord: CDN links |
| Contact/conversation list | Users need to see who they can message | LOW | Unified list with protocol badges |
| Message history | Users expect to see past conversations | HIGH | Requires local storage; WhatsApp/Signal have E2EE |
| Search within chat | Users expect to find past messages | MEDIUM | Per-conversation search is minimum viable |
| Account connection | Users must link their existing accounts | HIGH | Each protocol has different auth method (QR, pairing, token) |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Ultra-lightweight footprint | Primary differentiator vs web-wrapper apps (Ferdium, Rambox) | HIGH | Tauri + Rust vs Electron; protocol bridges vs iframe web apps |
| On-device bridging | Privacy: messages never touch third-party servers | HIGH | Beeper-style local bridges vs cloud-hosted |
| Cross-network unified search | Search across ALL conversations regardless of protocol | HIGH | Beeper's "killer feature" - requires local message index |
| Smart inbox organization | Work/friends/family filtering across protocols | MEDIUM | Tags or folders that span all networks |
| Protocol-native UX | Each chat feels like native app, not bridge | MEDIUM | Preserve protocol-specific features (Discord threads, Signal disappearing messages) |
| Message scheduling | Send later across any network | MEDIUM | Beeper Plus feature; requires queue system |
| Voice note transcription | Read voice notes without listening | MEDIUM | Beeper Plus feature; requires local or API transcription |
| Incognito mode | Preview without sending read receipts | LOW | Local-only feature, no protocol changes needed |
| Local-first architecture | No cloud dependency, full data ownership | HIGH | All data stored locally, sync is optional |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Cloud sync | "I want my chats on all devices" | Breaks E2EE model for WhatsApp/Signal; adds server infrastructure; privacy concerns | Local-first with optional encrypted backup export |
| Multi-account per protocol | "I have work and personal WhatsApp" | Doubles bridge complexity; auth conflicts; RAM usage increases | Defer to v2; v1 focuses on single account per protocol |
| Voice/video calling | "I want to call from one app" | Extremely complex bridging; each protocol has different calling APIs; often requires native apps anyway | Deep-link to native app for calls (Beeper approach) |
| Plugin system | "I want to extend functionality" | Adds complexity before core is stable; security implications; maintenance burden | Build features into core; consider plugins only after PMF |
| Real-time presence sync | "Show my status on all networks" | Presence APIs vary wildly; creates ghost "online" states; privacy issues | Local presence only; don't broadcast status changes |
| Group management | "Create and manage groups" | Group APIs are most complex part of each protocol; many edge cases | v1 focuses on 1:1 and existing groups; management is v2 |

## Feature Dependencies

```
Account Connection (WhatsApp/Signal/Discord auth)
    └──requires──> Protocol Bridge Implementation
                        └──requires──> Local Message Storage

Unified Inbox
    └──requires──> Account Connection (all protocols)
    └──requires──> Conversation List Aggregation

Send Messages
    └──requires──> Account Connection
    └──requires──> Protocol Bridge (bidirectional)

Cross-Network Search
    └──requires──> Local Message Index
                        └──requires──> Local Message Storage

Notifications
    └──requires──> Unified Inbox
    └──requires──> Desktop Integration (Tauri)

Message Scheduling
    └──conflicts──> Real-time delivery expectation
    └──requires──> Local Queue System
```

### Dependency Notes

- **Account Connection requires Protocol Bridge:** Each protocol (WhatsApp, Signal, Discord) needs a working bridge before users can authenticate. WhatsApp requires whatsmeow library, Signal requires libsignal, Discord uses discordgo.
- **Cross-Network Search requires Local Message Index:** To search across all protocols efficiently, messages must be stored and indexed locally. This is Beeper's killer feature.
- **Message Scheduling conflicts with Real-time:** Users expect instant delivery; scheduling adds complexity. Defer to v1.x.
- **Local-first architecture enables On-device bridging:** Without cloud dependency, bridges run locally and preserve E2EE for WhatsApp/Signal.

## MVP Definition

### Launch With (v1)

Minimum viable product — what's needed to validate the concept.

- [x] **Account connection for WhatsApp** — QR code pairing via whatsmeow library
- [x] **Account connection for Signal** — Link as secondary device via libsignal
- [x] **Account connection for Discord** — OAuth or access token via discordgo
- [x] **Unified inbox** — Single conversation list from all three protocols
- [x] **Read messages** — View incoming text messages from all protocols
- [x] **Send text messages** — Reply and start new conversations via text
- [x] **Basic notifications** — Desktop notifications for new messages
- [x] **Protocol badges** — Visual indicator of which network each conversation is on
- [x] **Lightweight footprint** — Must be lighter than running 3 separate apps

### Add After Validation (v1.x)

Features to add once core is working.

- [ ] **Basic media support** — Images and files (send/receive) — trigger: users request media sharing
- [ ] **Read receipts** — Show when messages are read — trigger: users confused about delivery status
- [ ] **Typing indicators** — Show when contact is typing — trigger: users expect real-time feel
- [ ] **Search within conversation** — Find messages in current chat — trigger: users can't find past messages
- [ ] **Reactions** — Emoji reactions to messages — trigger: users expect reaction parity with native apps
- [ ] **Message history persistence** — Store messages locally between sessions — trigger: users lose context on restart

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] **Cross-network unified search** — Search across ALL conversations; requires full message index
- [ ] **Smart inbox organization** — Tags/folders spanning protocols; requires metadata layer
- [ ] **Message scheduling** — Send later feature; requires queue system
- [ ] **Voice note transcription** — Requires local Whisper or API integration
- [ ] **Group management** — Create/invite/kick in groups; most complex protocol feature
- [ ] **Multi-account per protocol** — Work + personal accounts; auth complexity
- [ ] **Mobile apps** — Desktop-first product; mobile is significant undertaking
- [ ] **Notion/Obsidian integration** — Memory features after messaging core works

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Account connection (all 3) | HIGH | HIGH | P1 |
| Unified inbox | HIGH | MEDIUM | P1 |
| Read messages | HIGH | MEDIUM | P1 |
| Send text messages | HIGH | HIGH | P1 |
| Basic notifications | HIGH | LOW | P1 |
| Protocol badges | MEDIUM | LOW | P1 |
| Lightweight footprint | HIGH | HIGH | P1 |
| Basic media support | MEDIUM | MEDIUM | P2 |
| Read receipts | MEDIUM | MEDIUM | P2 |
| Typing indicators | MEDIUM | MEDIUM | P2 |
| Search within conversation | MEDIUM | MEDIUM | P2 |
| Reactions | LOW | MEDIUM | P2 |
| Message history persistence | HIGH | MEDIUM | P2 |
| Cross-network unified search | HIGH | HIGH | P3 |
| Smart inbox organization | MEDIUM | MEDIUM | P3 |
| Message scheduling | LOW | MEDIUM | P3 |
| Voice note transcription | LOW | HIGH | P3 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Competitor Feature Analysis

| Feature | Beeper | Ferdium/Rambox | Spoky Approach |
|---------|--------|----------------|----------------|
| Architecture | Protocol bridges (Matrix) | Web wrappers (iframe) | Protocol bridges (direct) |
| RAM usage | ~200-400MB | ~500MB-1GB+ (Electron + web apps) | Target: <150MB (Tauri + Rust) |
| WhatsApp | ✅ Full bridge | ✅ Web wrapper | ✅ Bridge (whatsmeow) |
| Signal | ✅ Full bridge | ✅ Web wrapper | ✅ Bridge (libsignal) |
| Discord | ✅ Full bridge | ✅ Web wrapper | ✅ Bridge (discordgo) |
| E2EE preservation | ✅ On-device connections | ❌ No (web renders) | ✅ Direct protocol bridges |
| Unified search | ✅ Cross-network | ❌ Per-tab only | ❌ Defer to v2 |
| Message scheduling | ✅ Plus feature | ❌ | ❌ Defer to v1.x |
| Cloud sync | ✅ Optional | ❌ Per-service | ❌ Local-first |
| Customization | ❌ Limited | ✅ Extensive | ❌ Focus on core |
| Cost | Free (5 accounts) / $9.99/mo | Free / $5-10/mo | Free (self-hosted) |

## Protocol-Specific Feature Support

### WhatsApp (via mautrix-whatsapp/whatsmeow)

| Feature | Supported | Notes |
|---------|-----------|-------|
| Text messages | ✅ | Plain and formatted |
| Media/files | ✅ | Images, video, documents |
| Replies | ✅ | Quote messages |
| Reactions | ✅ | Emoji reactions |
| Polls | ✅ | Create and vote |
| Groups | ✅ | Read/write |
| Read receipts | ✅ | Blue ticks |
| Typing indicators | ✅ | Real-time |
| Location sharing | ✅ | Send/receive |
| Contact sharing | ✅ | Send/receive |
| Voice notes | ✅ | Audio messages |
| Disappearing messages | ✅ | Time-based deletion |
| Calls | ❌ | Must use native app |
| Broadcast lists | ❌ | Not supported on web |

### Signal (via mautrix-signal/libsignal)

| Feature | Supported | Notes |
|---------|-----------|-------|
| Text messages | ✅ | Plain and formatted |
| Media/files | ✅ | Images, files, gifs, stickers |
| Replies | ✅ | Quote messages |
| Reactions | ✅ | Emoji reactions |
| Polls | ✅ | Create and vote |
| Groups | ✅ | Full support including permissions |
| Read receipts | ✅ | Delivered/read status |
| Typing indicators | ✅ | Real-time |
| Voice notes | ✅ | Audio messages |
| Disappearing messages | ✅ | Core Signal feature |
| Contacts | ✅ | Profile sync |
| Payment messages | ❌ | Not bridged |
| Calls | ❌ | Must use native app |

### Discord (via mautrix-discord/discordgo)

| Feature | Supported | Notes |
|---------|-----------|-------|
| Text messages | ✅ | Plain and formatted (markdown) |
| Media/files | ✅ | CDN-based |
| Replies | ✅ | Quote messages |
| Threads | ✅ | Create and participate |
| Reactions | ✅ | Unicode and custom emojis |
| Custom emojis | ✅ | Display and react |
| Embeds | ✅ | Rich content preview |
| Groups/DMs | ✅ | Private chats |
| Servers/channels | ✅ | Guild support |
| Typing indicators | ✅ | Partial (DMs after first message) |
| Read receipts | ✅ | Own status only |
| Bot commands | ✅ | Execute slash commands |
| Voice channels | ❌ | Must use native app |
| Presence/status | ❌ | Not bridged |

## Sources

- **Beeper FAQ & Help Center** - https://beeper.com/faq, https://help.beeper.com — Official feature documentation for unified messenger (HIGH confidence)
- **mautrix-whatsapp ROADMAP.md** - https://github.com/mautrix/whatsapp/blob/main/ROADMAP.md — WhatsApp bridge feature support (HIGH confidence)
- **mautrix-signal ROADMAP.md** - https://github.com/mautrix/signal/blob/main/ROADMAP.md — Signal bridge feature support (HIGH confidence)
- **mautrix-discord ROADMAP.md** - https://github.com/mautrix/discord/blob/main/ROADMAP.md — Discord bridge feature support (HIGH confidence)
- **Ferdium Website** - https://ferdium.org — Web-wrapper approach reference (HIGH confidence)
- **Rambox Website** - https://rambox.app — Web-wrapper approach reference (HIGH confidence)
- **Beeper Blog: Universal Communication Bus** - https://blog.beeper.com/p/the-universal-communication-bus-42dfb9a141ad — Vision and user pain points (HIGH confidence)
- **Matrix.org Bridges** - https://matrix.org/ecosystem/bridges/ — Protocol bridge ecosystem (HIGH confidence)

---
*Feature research for: unified messenger / multi-protocol chat (WhatsApp, Signal, Discord)*
*Researched: 2026-03-10*
