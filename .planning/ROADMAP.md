# Roadmap: Spoky

## Overview

Spoky delivers a unified messaging experience by progressively integrating three chat protocols—Discord, WhatsApp, and Signal—into a single lightweight desktop app. The journey starts with foundational infrastructure, then validates the architecture with Discord (most stable API), expands to WhatsApp and Signal, adds media support, and finishes with notifications for a complete messaging experience.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Foundation** - App infrastructure, database, and protocol abstraction layer
- [ ] **Phase 2: Discord Integration** - First working protocol with core messaging features
- [ ] **Phase 3: WhatsApp Integration** - QR code authentication and protocol expansion
- [ ] **Phase 4: Signal Integration** - Device linking to complete protocol coverage
- [ ] **Phase 5: Media Support** - Image and file sharing across all protocols
- [ ] **Phase 6: Notifications** - Desktop alerts and notification preferences

## Phase Details

### Phase 1: Foundation
**Goal**: Working app infrastructure ready for protocol integration
**Depends on**: Nothing (first phase)
**Requirements**: MSG-11
**Success Criteria** (what must be TRUE):
  1. User can launch the app and see an empty state UI
  2. App stores and retrieves data persistently between sessions
  3. Protocol adapter architecture is ready for Discord integration
**Plans**: TBD

Plans:
- [ ] 01-01: [TBD during planning]

### Phase 2: Discord Integration
**Goal**: User can connect Discord and message with Discord contacts
**Depends on**: Phase 1
**Requirements**: ACCT-03, ACCT-04, ACCT-05, ACCT-06, ACCT-07, MSG-01, MSG-02, MSG-03, MSG-04, MSG-05, MSG-06, MSG-12, UI-01, UI-02, UI-03, UI-04
**Success Criteria** (what must be TRUE):
  1. User can connect Discord account via bot token
  2. User sees all Discord conversations in unified list with protocol badges
  3. User can open a conversation and view message history
  4. User can send and reply to messages in Discord conversations
  5. User can disconnect and reconnect Discord account
**Plans**: TBD

Plans:
- [ ] 02-01: [TBD during planning]

### Phase 3: WhatsApp Integration
**Goal**: User can connect WhatsApp and message with WhatsApp contacts
**Depends on**: Phase 2
**Requirements**: ACCT-01, UI-05
**Success Criteria** (what must be TRUE):
  1. User can connect WhatsApp account via QR code scan
  2. User sees WhatsApp conversations alongside Discord in unified list
  3. User can message with WhatsApp contacts using existing messaging features
  4. User sees helpful error messages when WhatsApp connection fails
**Plans**: TBD

Plans:
- [ ] 03-01: [TBD during planning]

### Phase 4: Signal Integration
**Goal**: User can connect Signal and message with Signal contacts
**Depends on**: Phase 3
**Requirements**: ACCT-02
**Success Criteria** (what must be TRUE):
  1. User can connect Signal account via device linking
  2. User sees Signal conversations alongside Discord and WhatsApp in unified list
  3. User can message with Signal contacts using existing messaging features
**Plans**: TBD

Plans:
- [ ] 04-01: [TBD during planning]

### Phase 5: Media Support
**Goal**: User can share images and files in conversations
**Depends on**: Phase 4
**Requirements**: MSG-07, MSG-08, MSG-09, MSG-10, UI-06
**Success Criteria** (what must be TRUE):
  1. User can send images in any conversation
  2. User can send file attachments in any conversation
  3. User can view received images inline in conversation view
  4. User can download received file attachments
**Plans**: TBD

Plans:
- [ ] 05-01: [TBD during planning]

### Phase 6: Notifications
**Goal**: User receives timely alerts for new messages
**Depends on**: Phase 5
**Requirements**: NTFY-01, NTFY-02, NTFY-03, NTFY-04, NTFY-05
**Success Criteria** (what must be TRUE):
  1. User receives desktop notification when new message arrives
  2. User can click notification to open that conversation
  3. User can enable or disable notifications per account
  4. User can enable do-not-disturb mode to suppress all notifications
**Plans**: TBD

Plans:
- [ ] 06-01: [TBD during planning]

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation | 0/TBD | Not started | - |
| 2. Discord Integration | 0/TBD | Not started | - |
| 3. WhatsApp Integration | 0/TBD | Not started | - |
| 4. Signal Integration | 0/TBD | Not started | - |
| 5. Media Support | 0/TBD | Not started | - |
| 6. Notifications | 0/TBD | Not started | - |

---
*Roadmap created: 2026-03-10*
