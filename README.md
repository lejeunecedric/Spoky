# Spoky 💬

> One lightweight desktop app for all your messages. No more juggling heavy chat clients.

[![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust)](https://www.rust-lang.org)
[![Svelte](https://img.shields.io/badge/Svelte-FF3E00?logo=svelte)](https://svelte.dev)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## What is Spoky?

**Spoky** is an ultra-lightweight desktop communicator that brings your WhatsApp, Signal, and Discord conversations into one place. Built with a Rust-heavy core and Tauri shell, it delivers a fast, memory-efficient messaging experience without the bloat of running multiple heavy chat clients.

### The Problem

Modern messaging is fragmented. You need:
- Discord for gaming communities
- WhatsApp for family and international friends  
- Signal for privacy-conscious conversations

Running all three means 3x the RAM usage, 3x the startup time, and constant context switching.

### The Solution

One app. All your messages. Minimal resource usage.

```
┌─────────────────────────────────────┐
│  Spoky                              │
├──────────┬──────────────────────────┤
│          │  💬 Conversation View    │
│  📋      │  ─────────────────────   │
│  Unified │                          │
│  Inbox   │  Alice (WhatsApp)        │
│          │  > Hey! Are we still     │
│  • Discord│    on for tonight?      │
│  • Whats │                          │
│  • Signal│  You (Discord)           │
│          │  > Yeah, 8pm works!      │
│          │                          │
└──────────┴──────────────────────────┘
```

---

## ✨ Features

### Current (In Development)

- [x] **Protocol Abstraction Layer** — Event-driven adapter trait for multi-protocol support
- [ ] **Tauri + Svelte Foundation** — Modern, lightweight desktop shell
- [ ] **SQLite Database** — Full-featured schema for messages, conversations, and sync
- [ ] **Discord Integration** — Bot token authentication, channels, DMs
- [ ] **WhatsApp Integration** — QR code pairing
- [ ] **Signal Integration** — Device linking
- [ ] **Unified Inbox** — All conversations in one searchable list
- [ ] **Desktop Notifications** — Never miss a message

### Architecture Highlights

```rust
// Every protocol implements the same trait
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    async fn connect(&mut self, account: &Account) -> Result<ConnectionStatus>;
    async fn get_conversations(&self) -> Result<Vec<Conversation>>;
    async fn send_message(&self, conversation_id: &str, content: &str) -> Result<Message>;
    fn on_event(&mut self, callback: Box<dyn Fn(ProtocolEvent)>);
}
```

- **Event-Driven** — Real-time updates via Tauri IPC, no polling
- **Protocol Registry** — Manages active connections with thread-safe access
- **Extensible** — New protocols just implement the trait

---

## 🛠️ Tech Stack

| Layer | Technology | Why |
|-------|-----------|-----|
| **Frontend** | Svelte 5 + TypeScript | Reactive, minimal overhead |
| **Desktop** | Tauri 2.0 | Rust-powered, tiny bundle size |
| **Backend** | Rust | Memory safety, performance |
| **Database** | SQLite | Embedded, zero-config |
| **Protocols** | Custom adapters | Discord bot API, WhatsApp Web, Signal CLI |

---

## 🚀 Development Status

**Phase 1: Foundation** — In Progress (1/3 complete)

| Phase | Status | Description |
|-------|--------|-------------|
| 1. Foundation | 🚧 In Progress | App shell, database, protocol abstraction |
| 2. Discord | ⏳ Planned | First protocol integration |
| 3. WhatsApp | ⏳ Planned | QR auth, message sync |
| 4. Signal | ⏳ Planned | Device linking |
| 5. Media | ⏳ Planned | Images, attachments |
| 6. Notifications | ⏳ Planned | Desktop alerts |

See [ROADMAP.md](.planning/ROADMAP.md) for detailed planning.

---

## 📁 Project Structure

```
spoky/
├── src/                    # Svelte frontend
│   ├── routes/
│   └── components/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── models/         # Data structures
│   │   ├── protocol/       # Protocol abstraction
│   │   │   ├── adapter.rs  # ProtocolAdapter trait
│   │   │   ├── events.rs   # Event types
│   │   │   └── registry.rs # Connection management
│   │   └── main.rs
│   └── Cargo.toml
├── .planning/              # GSD project management
│   ├── phases/             # Phase-based planning
│   ├── ROADMAP.md
│   └── PROJECT.md
└── README.md
```

---

## 🏗️ Building from Source

### Prerequisites

- [Node.js](https://nodejs.org) 18+
- [Rust](https://rust-lang.org) 1.70+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/spoky.git
cd spoky

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Building for Production

```bash
# Build optimized release
npm run tauri build
```

Output will be in `src-tauri/target/release/bundle/`.

---

## 🤝 Contributing

This is a personal project for me and my friends, but contributions are welcome! 

Areas where help is appreciated:
- Protocol adapters (Discord, WhatsApp Web, Signal)
- UI/UX design for the unified inbox
- Performance optimization
- Documentation

Please read our [Contributing Guide](CONTRIBUTING.md) (coming soon).

---

## 📜 License

MIT License — see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app) — the Rust-based desktop framework
- Inspired by the frustration of running 3+ chat apps simultaneously
- For my friends who just want to chat without the RAM tax

---

<div align="center">

**[Download Latest Release](https://github.com/yourusername/spoky/releases)** •
**[Report Bug](https://github.com/yourusername/spoky/issues)** •
**[Request Feature](https://github.com/yourusername/spoky/issues)**

</div>
