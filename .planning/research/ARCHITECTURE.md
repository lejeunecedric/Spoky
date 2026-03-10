# Architecture Research

**Domain:** Tauri + Rust Multi-Protocol Messenger
**Researched:** 2026-03-10
**Confidence:** MEDIUM (Protocol implementations are well-documented, but multi-protocol integration patterns have limited examples)

## Standard Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           WebView Process (Frontend)                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐ │
│  │   UI Components │  │  State Manager  │  │   Unified Inbox View    │ │
│  │   (Svelte/React)│  │   (Zustand/etc) │  │   (Conversation List)   │ │
│  └────────┬────────┘  └────────┬────────┘  └────────────┬────────────┘ │
└───────────┼─────────────────────┼───────────────────────┼──────────────┘
            │                     │                       │
            │    IPC (Commands/Events)                    │
            ▼                     ▼                       ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Core Process (Rust)                               │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Protocol Adapter Layer                        │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │   │
│  │  │  Adapter    │  │  Adapter    │  │      Adapter            │  │   │
│  │  │  Trait      │  │  Registry   │  │      Router             │  │   │
│  │  │  (common)   │  │  (manager)  │  │      (dispatch)         │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │ Discord     │  │ WhatsApp    │  │ Signal      │  │  Message    │   │
│  │ (twilight)  │  │ (sidecar)   │  │ (presage)   │  │  Store      │   │
│  │ Rust crate  │  │ Go binary   │  │ Rust crate  │  │  (SQLite)   │   │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
└─────────┼────────────────┼────────────────┼────────────────┼──────────┘
          │                │                │                │
          │                │                │                │
          ▼                ▼                ▼                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Protocol Services (External)                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                      │
│  │ Discord API │  │ WhatsApp    │  │ Signal      │                      │
│  │ (WebSocket) │  │ Web API     │  │ Servers     │                      │
│  └─────────────┘  └─────────────┘  └─────────────┘                      │
└─────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Implementation |
|-----------|----------------|----------------|
| WebView (Frontend) | UI rendering, user interactions, display conversations | Svelte/React with Tauri API |
| Core Process | App orchestration, state management, protocol coordination | Tauri Rust core |
| Protocol Adapter Trait | Common interface for all protocols | Rust trait with async methods |
| Discord Adapter | Connect to Discord via official API | `twilight` Rust crate ecosystem |
| WhatsApp Adapter | Connect to WhatsApp Web API | Sidecar with Go `whatsmeow` library |
| Signal Adapter | Connect to Signal network | `presage` Rust crate |
| Message Store | Persistent storage for messages, conversations | SQLite via `rusqlite` or `sqlx` |
| Adapter Registry | Manage adapter lifecycle, routing | Rust module in core |

## Recommended Project Structure

```
src/
├── src-tauri/                    # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Library root
│   │   ├── commands/            # Tauri IPC commands
│   │   │   ├── mod.rs
│   │   │   ├── conversations.rs  # Fetch/send messages
│   │   │   ├── accounts.rs       # Account management
│   │   │   └── protocol.rs       # Protocol-specific commands
│   │   ├── adapters/             # Protocol adapters
│   │   │   ├── mod.rs
│   │   │   ├── traits.rs         # ProtocolAdapter trait
│   │   │   ├── discord/          # Discord implementation
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs     # twilight client wrapper
│   │   │   │   └── events.rs     # Event handlers
│   │   │   ├── whatsapp/         # WhatsApp sidecar interface
│   │   │   │   ├── mod.rs
│   │   │   │   ├── sidecar.rs    # Process management
│   │   │   │   └── protocol.rs   # IPC with Go binary
│   │   │   └── signal/           # Signal implementation
│   │   │       ├── mod.rs
│   │   │       └── client.rs     # presage wrapper
│   │   ├── models/               # Shared data models
│   │   │   ├── mod.rs
│   │   │   ├── message.rs        # Unified Message struct
│   │   │   ├── conversation.rs   # Unified Conversation struct
│   │   │   └── account.rs        # Account config models
│   │   ├── store/                # Persistence layer
│   │   │   ├── mod.rs
│   │   │   ├── database.rs       # SQLite setup
│   │   │   └── migrations/       # DB migrations
│   │   └── events.rs             # Event definitions for frontend
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                          # Frontend (Svelte/Vue/React)
│   ├── lib/
│   │   ├── components/           # UI components
│   │   ├── stores/               # State management
│   │   └── api/                  # Tauri invoke wrappers
│   ├── routes/
│   └── app.html
└── sidecars/                     # External protocol binaries
    └── whatsapp-bridge/          # Go sidecar for WhatsApp
        ├── main.go
        └── go.mod
```

### Structure Rationale

- **adapters/**: Each protocol is isolated. Adding new protocols means adding a new directory, not modifying existing code.
- **commands/**: Tauri IPC handlers organized by domain. Keeps frontend-backend contract clear.
- **models/**: Unified types that all adapters convert to/from. This is the "common language" of the app.
- **sidecars/**: External binaries for protocols without Rust libraries. Bundled with the app.

## Architectural Patterns

### Pattern 1: Protocol Adapter Trait

**What:** A common Rust trait that all protocol implementations must implement. This abstraction layer allows the rest of the app to work with any protocol without knowing implementation details.

**When to use:** Essential for multi-protocol architecture. Enables swapping protocols, testing with mocks, and adding new protocols without touching core logic.

**Trade-offs:** Adds indirection layer; protocol-specific features must be abstracted or exposed via optional methods.

```rust
// adapters/traits.rs
use async_trait::async_trait;
use crate::models::{Message, Conversation, Account};

#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Unique identifier for this protocol
    fn protocol_name(&self) -> &'static str;
    
    /// Connect and authenticate with the service
    async fn connect(&mut self, account: Account) -> Result<(), AdapterError>;
    
    /// Disconnect from the service
    async fn disconnect(&mut self) -> Result<(), AdapterError>;
    
    /// Fetch conversations (channels, DMs, etc.)
    async fn get_conversations(&self) -> Result<Vec<Conversation>, AdapterError>;
    
    /// Fetch messages for a conversation
    async fn get_messages(
        &self, 
        conversation_id: &str, 
        limit: u32
    ) -> Result<Vec<Message>, AdapterError>;
    
    /// Send a text message
    async fn send_message(
        &self, 
        conversation_id: &str, 
        content: &str
    ) -> Result<Message, AdapterError>;
    
    /// Check if currently connected
    fn is_connected(&self) -> bool;
}

// Adapter registry manages all protocol instances
pub struct AdapterRegistry {
    adapters: HashMap<String, Box<dyn ProtocolAdapter>>,
}
```

### Pattern 2: Unified Message Model

**What:** A single data structure that represents messages from any protocol. Each adapter converts protocol-specific formats to this unified model.

**When to use:** Required for unified inbox. Without this, frontend must handle multiple message formats.

**Trade-offs:** Loses protocol-specific features (Discord embeds, Signal reactions) unless added to unified model.

```rust
// models/message.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique ID (protocol-specific format preserved)
    pub id: String,
    
    /// Protocol that produced this message
    pub protocol: Protocol,
    
    /// Conversation/channel this message belongs to
    pub conversation_id: String,
    
    /// Sender information
    pub sender: Sender,
    
    /// Message content (text, for v1)
    pub content: String,
    
    /// Timestamp (UTC)
    pub timestamp: i64,
    
    /// Was this message sent by the current user?
    pub is_own: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Discord,
    WhatsApp,
    Signal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    pub id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
}
```

### Pattern 3: Sidecar Bridge Pattern

**What:** For protocols without Rust libraries, run an external binary (sidecar) that handles the protocol, communicating with the Rust core via IPC (stdin/stdout JSON) or local WebSocket.

**When to use:** WhatsApp (only mature library is Go). Keeps protocol complexity isolated while staying in-process.

**Trade-offs:** Additional binary to bundle; inter-process communication overhead; separate build pipeline.

```rust
// adapters/whatsapp/sidecar.rs
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
enum SidecarRequest {
    Connect { phone_number: String },
    GetChats,
    SendMessage { chat_id: String, text: String },
}

#[derive(Deserialize)]
enum SidecarResponse {
    Connected,
    Chats(Vec<WhatsAppChat>),
    MessageSent { id: String },
    Error(String),
}

pub struct WhatsAppSidecar {
    child: Option<CommandChild>,
}

impl WhatsAppSidecar {
    pub async fn spawn(app: &tauri::AppHandle) -> Result<Self, Error> {
        let sidecar = app.shell().sidecar("whatsapp-bridge")?;
        let (mut rx, child) = sidecar.spawn()?;
        
        // Spawn task to handle incoming events
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    // Parse and emit to frontend
                }
            }
        });
        
        Ok(Self { child: Some(child) })
    }
    
    pub async fn send_request(&mut self, req: SidecarRequest) -> Result<SidecarResponse, Error> {
        // Write JSON to stdin, read response
    }
}
```

## Data Flow

### Incoming Message Flow

```
[Protocol Server]
       │
       ▼
[Protocol Adapter] receives event/webhook
       │
       ▼ converts to unified Message
[Adapter Router] determines which adapter
       │
       ▼
[Message Store] persists to SQLite
       │
       ▼ emits Tauri event
[Frontend State] receives via listener
       │
       ▼
[UI Component] renders in conversation
```

### Outgoing Message Flow

```
[User Input] in UI
       │
       ▼ invoke Tauri command
[Rust Command Handler] receives request
       │
       ▼ routes to adapter
[Protocol Adapter] sends via protocol
       │
       ▼
[Protocol Server] delivers message
       │
       ▼ confirmation/event
[Message Store] marks as sent
       │
       ▼
[Frontend] updates message status
```

### State Management

```
[Frontend Store (Zustand/Svelte stores)]
       │
       │  Tauri invoke/emit
       ▼
[Rust Core Process]
       │
       │  Adapter trait calls
       ▼
[Protocol Adapters] (stateless, forward to service)
       │
       │
       ▼
[Message Store] (SQLite - single source of truth)
```

### Key Data Flows

1. **Initial Sync:** On app start, adapters fetch recent conversations/messages, store in SQLite, emit to frontend for display.

2. **Real-time Updates:** Each adapter maintains persistent connection (WebSocket/polling). Incoming messages flow through adapter → store → frontend event.

3. **Account Management:** Account credentials stored encrypted in SQLite. Adapters read from store on connect, persist session tokens.

## Protocol Adapter Design

### Discord Adapter (Native Rust)

**Library:** `twilight` ecosystem (twilight-gateway, twilight-http, twilight-model)

**Implementation approach:**
- Use `twilight-gateway` for WebSocket connection to Discord
- Use `twilight-http` for REST API calls
- Use `twilight-cache-inmemory` for state tracking

```rust
// adapters/discord/client.rs
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub struct DiscordAdapter {
    http: HttpClient,
    shard: Option<Shard>,
    intents: Intents,
}

impl DiscordAdapter {
    pub fn new(token: String) -> Self {
        let intents = Intents::GUILD_MESSAGES 
            | Intents::DIRECT_MESSAGES 
            | Intents::MESSAGE_CONTENT;
        
        Self {
            http: HttpClient::new(token),
            shard: None,
            intents,
        }
    }
}

#[async_trait]
impl ProtocolAdapter for DiscordAdapter {
    fn protocol_name(&self) -> &'static str { "discord" }
    
    async fn connect(&mut self, account: Account) -> Result<()> {
        let (shard, events) = twilight_gateway::create_recommended(
            &self.http,
            twilight_gateway::Config::new(account.token, self.intents)
        ).await?;
        
        self.shard = Some(shard);
        
        // Spawn event handler task
        tokio::spawn(handle_discord_events(events));
        
        Ok(())
    }
    
    // ... implement other trait methods
}
```

**Authentication:** Bot token or user OAuth2. For personal use, can use user token (against TOS) or create a bot application.

### WhatsApp Adapter (Sidecar)

**Library:** `whatsmeow` (Go) - most mature WhatsApp Web API library

**Implementation approach:**
- Compile Go binary that wraps whatsmeow
- Binary exposes JSON-RPC over stdin/stdout
- Rust sidecar manager spawns process and handles IPC

**Sidecar protocol (JSON over stdin/stdout):**

```json
// Request (Rust → Go)
{"id": 1, "method": "connect", "params": {}}
{"id": 2, "method": "get_chats", "params": {}}
{"id": 3, "method": "send_message", "params": {"chat": "1234567890@s.whatsapp.net", "text": "Hello"}}

// Response (Go → Rust)
{"id": 1, "result": {"status": "connected", "phone": "+1234567890"}}
{"id": 2, "result": {"chats": [...]}}

// Event (Go → Rust, unsolicited)
{"event": "message", "data": {"from": "...", "text": "...", "timestamp": 1234567890}}
```

**Authentication:** QR code scan (like WhatsApp Web). Sidecar generates QR, Rust displays in UI, sidecar receives scan confirmation.

### Signal Adapter (Native Rust)

**Library:** `presage` (Rust) - wraps `libsignal-service-rs`

**Implementation approach:**
- Use presage's `Manager` struct for all operations
- Store session in SQLite (presage-store-sqlite)
- Link as secondary device (like Signal Desktop)

```rust
// adapters/signal/client.rs
use presage::Manager;
use presage_store_sqlite::SqliteStore;

pub struct SignalAdapter {
    manager: Option<Manager<SqliteStore>>,
}

impl SignalAdapter {
    pub fn new(db_path: &Path) -> Result<Self> {
        let store = SqliteStore::open(db_path)?;
        Ok(Self { manager: None })
    }
    
    pub async fn link_device(&mut self, device_name: &str) -> Result<QrCode> {
        let store = SqliteStore::open(db_path)?;
        let (manager, registration) = Manager::link_secondary_device(
            store,
            device_name.to_string(),
        ).await?;
        
        // registration contains QR code data
        self.manager = Some(manager);
        Ok(registration.qr_code)
    }
}

#[async_trait]
impl ProtocolAdapter for SignalAdapter {
    fn protocol_name(&self) -> &'static str { "signal" }
    
    // ... implement trait using presage Manager
}
```

**Authentication:** Link as secondary device (scan QR from phone Signal app). Same flow as Signal Desktop.

**License Note:** `presage` is AGPL-3.0, which requires derivative works to be open-source. Consider licensing implications.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 1-5 users (v1 target) | Current architecture is optimal. All protocol handlers in single process, SQLite for storage. |
| 10-100 users | Add message pagination, lazy-load conversations. Consider connection pooling for SQLite. |
| 100+ users | Split protocol adapters into separate processes (true microservices). Use PostgreSQL. Add message queue for events. |

### Scaling Priorities

1. **First bottleneck:** Protocol connection management. Each adapter maintains persistent connections. For single-user v1, this is fine. For multiple simultaneous accounts, consider connection pooling per protocol.

2. **Second bottleneck:** Message storage. SQLite works well up to ~100K messages. After that, consider:
   - Message archiving (old messages to cold storage)
   - Per-account databases
   - Migration to PostgreSQL

## Anti-Patterns

### Anti-Pattern 1: Protocol-Specific Types in Core

**What people do:** Pass Discord `Message` structs or WhatsApp `Chat` objects directly through the app.

**Why it's wrong:** Frontend becomes tightly coupled to each protocol. Adding a new protocol requires frontend changes.

**Do this instead:** Convert all protocol types to unified `Message`/`Conversation` types at the adapter boundary. The rest of the app only knows about unified types.

### Anti-Pattern 2: Frontend Manages Protocol Connections

**What people do:** Frontend makes direct HTTP/WebSocket calls to protocol servers.

**Why it's wrong:** Exposes credentials in WebView. Bypasses Rust security layer. Can't run background sync when window closed.

**Do this instead:** All protocol communication happens in Rust Core. Frontend only calls Tauri commands.

### Anti-Pattern 3: Synchronous Protocol Operations

**What people do:** Block UI thread waiting for protocol responses.

**Why it's wrong:** Protocol operations are network-bound. Blocking freezes the UI.

**Do this instead:** All adapter methods are async. Frontend shows loading states. Use Tauri's async command support:

```rust
#[tauri::command]
async fn send_message(
    protocol: String,
    conversation_id: String,
    content: String,
    state: State<'_, AdapterRegistry>,
) -> Result<Message, String> {
    let adapter = state.get(&protocol)?;
    adapter.send_message(&conversation_id, &content).await
        .map_err(|e| e.to_string())
}
```

### Anti-Pattern 4: Storing Credentials in Plain Text

**What people do:** Store tokens/passwords directly in config files.

**Why it's wrong:** Anyone with file access can read credentials.

**Do this instead:** Use OS credential store via `keyring` crate, or encrypt with `keychain`/`secret-service`. For development, use `.env` files that are git-ignored.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| Discord Gateway | WebSocket via twilight-gateway | Requires bot token or user OAuth |
| Discord REST API | HTTP via twilight-http | Rate-limited; use twilight's built-in handling |
| WhatsApp Web | WebSocket via whatsmeow sidecar | No official API; reverse-engineered |
| Signal Servers | WebSocket via presage/libsignal | Uses Signal's official protocol |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Frontend ↔ Core | Tauri IPC (commands/events) | Commands for requests, events for push |
| Core ↔ Adapters | Trait method calls | Async methods, futures |
| Core ↔ Store | Direct function calls | Sync or async SQLite operations |
| Core ↔ Sidecar | stdin/stdout JSON | Spawned process, async I/O |

## Build Order Recommendations

Based on dependency analysis, recommended implementation order:

### Phase 1: Core Infrastructure
1. **Tauri project setup** - Scaffold with chosen frontend framework
2. **Message/Conversation models** - Define unified types first
3. **SQLite store** - Set up database with migrations
4. **Basic Tauri commands** - Hello world IPC validation

### Phase 2: First Protocol (Discord - easiest)
1. **twilight integration** - Add dependencies, create basic client
2. **Discord adapter** - Implement ProtocolAdapter trait
3. **Connection flow** - Bot token storage, gateway connection
4. **Message sync** - Fetch and display messages

**Why Discord first:**
- Native Rust library (no sidecar complexity)
- Well-documented API
- Easy bot token creation
- Instant feedback loop

### Phase 3: UI Foundation
1. **Conversation list** - Display all conversations
2. **Message view** - Display messages in selected conversation
3. **Send message** - Text input and send button
4. **Real-time updates** - Handle incoming message events

### Phase 4: Remaining Protocols
1. **Signal adapter** - presage integration, QR linking
2. **WhatsApp sidecar** - Go binary, IPC bridge
3. **Protocol switching** - UI for selecting active account

### Phase 5: Polish
1. **Error handling** - Connection failures, retry logic
2. **Background sync** - Messages when window closed
3. **Notifications** - Desktop notifications for new messages

## Sources

- Tauri Architecture Documentation: https://tauri.app/concept/architecture/
- Tauri Sidecar Guide: https://tauri.app/develop/sidecar/
- Twilight Discord Library: https://github.com/twilight-rs/twilight
- whatsmeow (WhatsApp): https://github.com/tulir/whatsmeow
- mautrix-whatsapp (reference implementation): https://github.com/mautrix/whatsapp
- mautrix-signal (reference implementation): https://github.com/mautrix/signal
- mautrix-discord (reference implementation): https://github.com/mautrix/discord
- presage (Signal Rust): https://github.com/whisperfish/presage

---
*Architecture research for: Tauri multi-protocol messenger*
*Researched: 2026-03-10*
