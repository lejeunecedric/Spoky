# Requirements: Spoky

**Defined:** 2026-03-10
**Core Value:** One lightweight desktop app lets users read and send text messages across their main chat networks without juggling multiple heavy apps.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Account Management

- [ ] **ACCT-01**: User can connect one WhatsApp account via QR code scan
- [ ] **ACCT-02**: User can connect one Signal account via device linking
- [ ] **ACCT-03**: User can connect one Discord account via bot token
- [ ] **ACCT-04**: User sees connection status for each protocol (connected, connecting, disconnected, error)
- [ ] **ACCT-05**: User can disconnect an account
- [ ] **ACCT-06**: User can reconnect a disconnected account
- [ ] **ACCT-07**: Account credentials are stored securely between sessions

### Messaging

- [ ] **MSG-01**: User sees unified conversation list combining all connected protocols
- [ ] **MSG-02**: User sees protocol badge on each conversation indicating its network
- [ ] **MSG-03**: User can open a conversation and view message history
- [ ] **MSG-04**: User can send a text message in an open conversation
- [ ] **MSG-05**: User can reply to a specific message
- [ ] **MSG-06**: User can start a new text conversation (where protocol supports)
- [ ] **MSG-07**: User can send an image in a conversation
- [ ] **MSG-08**: User can send a file attachment in a conversation
- [ ] **MSG-09**: User can view received images in a conversation
- [ ] **MSG-10**: User can download received file attachments
- [ ] **MSG-11**: Messages persist locally between app sessions
- [ ] **MSG-12**: New messages appear in real-time without manual refresh

### Notifications

- [ ] **NTFY-01**: User receives desktop notification when a new message arrives
- [ ] **NTFY-02**: User can click a notification to open that conversation
- [ ] **NTFY-03**: User can enable/disable notifications per account
- [ ] **NTFY-04**: User can enable/disable notification sound
- [ ] **NTFY-05**: User can enable do not disturb mode (suppress all notifications)

### UI

- [ ] **UI-01**: App displays conversation list in a sidebar
- [ ] **UI-02**: App displays selected conversation in main pane
- [ ] **UI-03**: App displays message input field in conversation view
- [ ] **UI-04**: App displays connection status indicator for each protocol
- [ ] **UI-05**: App displays error states when protocol connection fails
- [ ] **UI-06**: User can attach image or file via message input

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Enhanced Messaging

- **MSG-13**: User can search within a conversation
- **MSG-14**: User can search across all conversations
- **MSG-15**: User sees read receipts for sent messages
- **MSG-16**: User sees typing indicators when others are typing
- **MSG-17**: User can react to messages with emoji

### Enhanced UI

- **UI-07**: User can toggle dark mode
- **UI-08**: User can use keyboard shortcuts for common actions
- **UI-09**: App has compact/floating window mode

### Multi-Account

- **ACCT-08**: User can connect multiple accounts per protocol

### Memory Integration

- **MEM-01**: User can save conversation snippet to Notion
- **MEM-02**: User can save conversation snippet to Obsidian
- **MEM-03**: User can link a conversation to a Notion page
- **MEM-04**: User can link a conversation to an Obsidian note

### Additional Protocols

- **PROTO-01**: User can connect a Mastodon account
- **PROTO-02**: User can connect an IRC account

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Voice/video calling | Extremely complex bridging; not core to lightweight messaging value |
| Group management (create, admin, permissions) | Defer until direct messaging workflow is solid |
| Mobile apps | Desktop-first product; mobile adds significant complexity |
| Plugin system | Avoid added complexity before core messaging works |
| Cloud sync | Local-first v1 focus; cloud sync breaks E2EE for WhatsApp/Signal |
| Multi-account per protocol | Doubles complexity; single account sufficient for v1 users |
| End-to-end encryption key management UI | Protocols handle E2EE transparently; no user action needed |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| ACCT-01 | — | Pending |
| ACCT-02 | — | Pending |
| ACCT-03 | — | Pending |
| ACCT-04 | — | Pending |
| ACCT-05 | — | Pending |
| ACCT-06 | — | Pending |
| ACCT-07 | — | Pending |
| MSG-01 | — | Pending |
| MSG-02 | — | Pending |
| MSG-03 | — | Pending |
| MSG-04 | — | Pending |
| MSG-05 | — | Pending |
| MSG-06 | — | Pending |
| MSG-07 | — | Pending |
| MSG-08 | — | Pending |
| MSG-09 | — | Pending |
| MSG-10 | — | Pending |
| MSG-11 | — | Pending |
| MSG-12 | — | Pending |
| NTFY-01 | — | Pending |
| NTFY-02 | — | Pending |
| NTFY-03 | — | Pending |
| NTFY-04 | — | Pending |
| NTFY-05 | — | Pending |
| UI-01 | — | Pending |
| UI-02 | — | Pending |
| UI-03 | — | Pending |
| UI-04 | — | Pending |
| UI-05 | — | Pending |
| UI-06 | — | Pending |

**Coverage:**
- v1 requirements: 30 total
- Mapped to phases: 0
- Unmapped: 30 ⚠️

---
*Requirements defined: 2026-03-10*
*Last updated: 2026-03-10 after initial definition*
