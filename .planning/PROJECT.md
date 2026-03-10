# Spoky

## What This Is

Spoky is an ultra-lightweight desktop communicator for me and my friends that brings WhatsApp, Signal, and Discord into one place. It is designed to reduce app sprawl and RAM usage by using a Tauri shell with a Rust-heavy core instead of multiple heavyweight chat clients.

## Core Value

One lightweight desktop app lets users read and send text messages across their main chat networks without juggling multiple heavy apps.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Users can connect one WhatsApp account, one Signal account, and one Discord account in the same desktop app.
- [ ] Users can browse a unified inbox that combines conversations from the supported protocols.
- [ ] Users can open conversations and send or reply to text messages from the app.
- [ ] The app stays lightweight enough to replace keeping multiple chat clients open.

### Out of Scope

- Voice and video calling — not part of the first release's core messaging value.
- Group management — defer until direct messaging workflow is solid.
- Multi-account support — keep v1 simple and lightweight.
- Mobile apps — desktop-first product.
- Plugin system — avoid added complexity before core messaging works.
- Cloud sync — local-first v1 focus.
- Notion and Obsidian integrations in v1 — memory features come after the messaging core is working.
- Mastodon and IRC in v1 — later protocol expansion after WhatsApp, Signal, and Discord.

## Context

The main problem is having too many chat apps open at once and paying the RAM cost for all of them. The first version is for a small, real user set: me and my friends. The intended v1 experience is to connect supported accounts, see conversations in a unified inbox, open a thread, and send or reply to text messages from one desktop app. Long term, the product should also connect conversations to memory tools such as Notion and Obsidian so users can save insights, link chats to notes or projects, and retrieve notes while chatting.

## Constraints

- **Tech stack**: Tauri desktop app with a Rust-heavy core — chosen to minimize resource usage versus heavier desktop stacks.
- **Performance**: Lightweight operation is a primary product constraint — the app should balance low RAM usage with fast startup.
- **Protocol scope**: v1 supports WhatsApp, Signal, and Discord only — later protocols are explicitly deferred.
- **UX**: v1 should use a normal desktop split-pane chat layout — compact mode can come later.
- **Functional scope**: v1 focuses on text messaging only — no voice/video, mobile, plugins, or cloud sync.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use Tauri with a Rust-heavy core | Lightweight desktop footprint is central to the product | — Pending |
| Start with WhatsApp, Signal, and Discord | These are the first three protocols explicitly required for v1 | — Pending |
| Use a unified inbox | Reduces app switching and delivers the main value immediately | — Pending |
| Support reading, replying, and starting text conversations in v1 where supported | v1 needs to be useful as a real daily messaging client, not just a viewer | — Pending |
| Defer Notion and Obsidian integrations until after messaging core is stable | Memory features matter, but not before the core communicator works | — Pending |

---
*Last updated: 2026-03-10 after initialization*
