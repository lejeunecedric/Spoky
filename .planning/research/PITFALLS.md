# Domain Pitfalls

**Domain:** Tauri Multi-Protocol Messenger (WhatsApp, Signal, Discord)
**Researched:** 2026-03-10

## Critical Pitfalls

Mistakes that cause rewrites, account bans, or project failure.

### Pitfall 1: Discord Self-Bot Account Termination

**What goes wrong:** Automating a normal Discord user account (self-bot) violates Discord's Terms of Service and can result in immediate account termination.

**Why it happens:** Discord's API provides bot accounts for automation, but bot accounts cannot access DMs or act as regular users. Developers try to automate their personal account using token-based authentication to get full user functionality.

**Consequences:**
- Account termination without warning
- All associated data/guilds lost
- IP may be flagged for future monitoring

**Prevention:**
- **DO NOT automate user accounts** — Discord explicitly forbids this: "Automating normal user accounts (generally called 'self-bots') outside of the OAuth2/bot API is forbidden, and can result in an account termination if found."
- Use Discord's official Bot API only (requires server admin to add bot)
- For DM access, consider that bots cannot initiate DMs — users must DM first
- Alternative: Use Discord's official client and accept limited automation

**Detection:** Any use of a user token (not bot token) to authenticate API calls.

**Phase to address:** Phase 1 (Protocol Implementation) — Must decide Discord integration strategy before implementation.

**Source:** [Discord Self-Bots Policy](https://support.discord.com/hc/en-us/articles/115002192352-Automated-User-Accounts-Self-Bots) — HIGH confidence

---

### Pitfall 2: WhatsApp Unofficial API Ban Risk

**What goes wrong:** WhatsApp has no official API for personal accounts. Using reverse-engineered libraries (baileys, whatsmeow, whatsapp-web.js) risks account bans.

**Why it happens:** Meta only provides the WhatsApp Business API for commercial use. Personal account automation requires unofficial libraries that simulate the WhatsApp Web protocol.

**Consequences:**
- Account temporary or permanent ban
- Phone number blacklisted
- Bridge breaks when WhatsApp updates protocol (frequent)
- Library maintainers may abandon project after protocol changes

**Prevention:**
- Use `whatsmeow` (Go library used by mautrix-whatsapp) — most actively maintained
- Implement aggressive rate limiting (WhatsApp is sensitive to automation patterns)
- Never send bulk messages or auto-responses
- Keep session alive but don't poll aggressively
- Handle disconnections gracefully with exponential backoff
- Test with a secondary account first
- Document that users use at their own risk

**Detection:** 
- Sudden connection failures after WhatsApp updates
- Account locked messages
- Rate limit errors from WhatsApp servers

**Phase to address:** Phase 1 (Protocol Implementation) — Must establish risk acceptance and fallback strategy.

**Source:** Matrix mautrix-whatsapp documentation, community reports — MEDIUM confidence (Meta doesn't document ban criteria publicly)

---

### Pitfall 3: Signal Third-Party Access Prohibition

**What goes wrong:** Signal's Terms of Service forbid unauthorized access to their services and bulk/auto-messaging.

**Why it happens:** Signal has no official third-party client API. Bridges like mautrix-signal use signal-cli which is a reverse-engineered interface.

**Consequences:**
- Potential account suspension
- Protocol changes break functionality
- No official support when things break
- Registration Lock PIN complications

**Prevention:**
- Use `mautrix-signal` approach (Go-based, references libsignal)
- Respect Signal's rate limits and automation policies
- Implement proper device registration flow
- Handle Registration Lock PIN requirements
- Never auto-message or bulk send

**Detection:**
- CAPTCHA challenges during registration
- Connection drops after app updates
- Account verification failures

**Phase to address:** Phase 1 (Protocol Implementation) — Signal integration may need alternative approach.

**Source:** [Signal Terms of Service](https://signal.org/legal/) — HIGH confidence

---

### Pitfall 4: Multi-Protocol State Synchronization Hell

**What goes wrong:** Each protocol has different connection semantics, reconnection behavior, and message ordering. A unified inbox becomes a nightmare of race conditions and stale data.

**Why it happens:** Developers underestimate the complexity of:
- WhatsApp: Long-lived WebSocket with periodic keepalive
- Signal: Different connection model, tied to phone number verification
- Discord: Gateway WebSocket with heartbeats and resume capability

**Consequences:**
- Messages appear in wrong order
- Duplicate messages across reconnects
- UI shows "connected" when actually disconnected
- Memory leaks from orphaned connection handlers
- Dead messages sent while offline appear unexpectedly

**Prevention:**
- Design a **protocol abstraction layer** from day one
- Each protocol wrapper must implement:
  - `connect()` / `disconnect()` / `reconnect()`
  - `getConnectionState()` with granular states
  - `getMessages(since)` for catch-up
  - Event emitters for incoming messages
- Use message IDs for deduplication (each protocol has different ID format)
- Implement connection state machine per protocol
- Never assume "connected" means "can send immediately"

**Detection:**
- Messages appearing after "disconnect"
- UI state doesn't match actual connection
- Memory usage growing over time

**Phase to address:** Phase 1 (Core Architecture) — Define protocol interface before any implementation.

---

### Pitfall 5: Tauri IPC Serialization Blocking

**What goes wrong:** Passing large message histories or media through Tauri's IPC causes UI freezes because all data must serialize to JSON.

**Why it happens:** Tauri uses JSON-RPC-like protocol for IPC. Large payloads block the single IPC thread, freezing the WebView.

**Consequences:**
- App appears frozen during message sync
- Images/media take seconds to "load" even when local
- Poor UX on low-end machines
- OOM on very large conversations

**Prevention:**
- **Stream large data** — don't pass entire history at once
- Implement pagination in IPC calls
- Store messages in local database, pass only IDs through IPC
- Use Tauri's sidecar for heavy protocol processing if needed
- Consider `tauri-plugin-sql` for direct database access from frontend
- For media: pass file paths, not base64 blobs

**Detection:**
- UI freezes during initial sync
- Network tab shows multi-second IPC calls
- DevTools shows large JSON payloads

**Phase to address:** Phase 2 (Data Layer) — Database and IPC strategy must be designed together.

**Source:** [Tauri IPC Documentation](https://tauri.app/concept/inter-process-communication/) — HIGH confidence

---

## Moderate Pitfalls

### Pitfall 6: Discord Rate Limit Cascade Failure

**What goes wrong:** Hitting Discord's rate limits causes Cloudflare IP bans (10,000 invalid requests per 10 minutes).

**Why it happens:** 
- Global limit: 50 requests/second per bot
- Per-route limits vary and change
- Invalid requests (401, 403, 429) count toward ban threshold
- Retry logic can amplify the problem

**Prevention:**
- Parse `X-RateLimit-*` headers dynamically — **never hardcode limits**
- Implement request queue with per-bucket tracking
- Use `X-RateLimit-Bucket` to group related endpoints
- Back off on 429 responses using `retry_after` value
- Handle global rate limit (`X-RateLimit-Global: true`) specially
- Log invalid request counts to avoid Cloudflare ban

**Detection:**
- 429 responses in logs
- `X-RateLimit-Remaining: 0` approaching zero
- Sudden connection failures (Cloudflare ban)

**Phase to address:** Phase 1 (Protocol Implementation)

**Source:** [Discord Rate Limits Documentation](https://discord.com/developers/docs/topics/rate-limits) — HIGH confidence

---

### Pitfall 7: Credential Storage Security

**What goes wrong:** Storing auth tokens/credentials insecurely exposes user accounts if device is compromised.

**Why it happens:** Need to persist session data between app restarts, but simple file storage or localStorage is not secure.

**Prevention:**
- Use OS credential store via Tauri plugin
- Consider `tauri-plugin-stronghold` for encrypted storage
- Never log tokens or session data
- Clear credentials on explicit logout
- Implement session timeout if desired

**Detection:**
- Credentials visible in config files
- Tokens in debug logs
- Session survives OS logout

**Phase to address:** Phase 2 (Data Layer)

---

### Pitfall 8: WebView Memory Leaks in Chat UI

**What goes wrong:** Long chat histories with images/media cause WebView memory to grow unbounded.

**Why it happens:** 
- DOM nodes for old messages not garbage collected
- Image blobs retained in memory
- Event listeners not cleaned up

**Prevention:**
- Implement **virtual scrolling** — only render visible messages
- Lazy-load images/media on scroll
- Clean up event listeners on unmount
- Set maximum rendered message count
- Consider `loading="lazy"` for images

**Detection:**
- Memory usage grows as user scrolls history
- App becomes sluggish after extended use
- DevTools Memory panel shows detached DOM nodes

**Phase to address:** Phase 3 (UI Implementation)

---

### Pitfall 9: Protocol Library Abandonment

**What goes wrong:** The third-party library you depend on stops being maintained when the protocol changes.

**Why it happens:** WhatsApp/Signal/Discord update their protocols. Reverse-engineered libraries break. Maintainers may not have time to fix.

**Prevention:**
- Choose libraries with active communities (mautrix projects are well-maintained)
- Monitor library GitHub issues for breakage reports
- Design abstraction layer to allow library swapping
- Have fallback plan (e.g., "WhatsApp bridge temporarily unavailable")
- Consider contributing to upstream if relying heavily

**Detection:**
- Sudden spike in connection errors after platform updates
- GitHub issues reporting breakage
- No commits to library in months

**Phase to address:** Phase 1 (Protocol Implementation) — Choose libraries carefully.

---

### Pitfall 10: Missing Message Types Cause Crashes

**What goes wrong:** Receiving an unsupported message type (location, contact, sticker, reaction, etc.) crashes or corrupts the UI.

**Why it happens:** Each protocol has many message types. Developers implement text messages first, then encounter edge cases in production.

**Prevention:**
- Define a **unified message type enum** upfront
- Include `Unknown` variant for unsupported types
- Log unknown types for future implementation
- Never assume all messages have text content
- Handle missing fields gracefully (null checks everywhere)

**Detection:**
- Crashes when receiving stickers/reactions
- UI shows "undefined" or blank messages
- Error logs about unexpected message format

**Phase to address:** Phase 2 (Data Model)

---

## Minor Pitfalls

### Pitfall 11: Reconnection Storm

**What goes wrong:** When connection drops, aggressive reconnection attempts overwhelm the server or cause UI thrashing.

**Prevention:** Implement exponential backoff with jitter. Cap maximum retry attempts before requiring user action.

---

### Pitfall 12: Timezone Handling in Messages

**What goes wrong:** Messages appear in wrong order or with wrong timestamps due to timezone confusion.

**Prevention:** Store all timestamps in UTC. Convert to local only for display. Use ISO 8601 format.

---

### Pitfall 13: WhatsApp Multi-Device Sync Issues

**What goes wrong:** WhatsApp's multi-device feature means messages may sync from phone after being read elsewhere, causing duplicate "new message" notifications.

**Prevention:** Track last-seen timestamp per conversation. Mark messages as read based on timestamp, not just receipt.

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|----------------|------------|
| Protocol Selection | Discord self-bot prohibition | Accept limited Discord functionality or skip Discord v1 |
| Protocol Implementation | WhatsApp/Signal ban risk | Document user risk, test with secondary accounts |
| Core Architecture | State sync hell | Design protocol abstraction layer first |
| Data Layer | IPC blocking | Use database for storage, pass IDs through IPC |
| UI Implementation | Memory leaks | Virtual scrolling, lazy loading |
| Authentication | Credential exposure | Use OS credential store |
| Error Handling | Crash on unknown message types | Always have fallback handling |

---

## Risk Summary by Protocol

| Protocol | Official API? | Ban Risk | Maintenance Risk | Recommendation |
|----------|---------------|----------|------------------|----------------|
| **Discord** | Yes (Bot API) | High (if self-bot) | Low | Use Bot API only, accept DM limitations |
| **WhatsApp** | No (Business only) | Medium-High | Medium | Use whatsmeow, document risk |
| **Signal** | No | Medium | Medium | Use signal-cli approach, document risk |

---

## Sources

- [Discord Self-Bots Policy](https://support.discord.com/hc/en-us/articles/115002192352-Automated-User-Accounts-Self-Bots) — HIGH confidence
- [Discord Rate Limits Documentation](https://discord.com/developers/docs/topics/rate-limits) — HIGH confidence
- [Discord Terms of Service](https://discord.com/terms) — HIGH confidence
- [Signal Terms of Service](https://signal.org/legal/) — HIGH confidence
- [Matrix Bridges Overview](https://matrix.org/ecosystem/bridges/) — HIGH confidence
- [Tauri Security Documentation](https://tauri.app/security/) — HIGH confidence
- [Tauri IPC Documentation](https://tauri.app/concept/inter-process-communication/) — HIGH confidence
- [Tauri CSP Documentation](https://tauri.app/security/csp/) — HIGH confidence
- [Tauri Sidecar Documentation](https://tauri.app/develop/sidecar/) — HIGH confidence
- mautrix-whatsapp / mautrix-signal patterns — MEDIUM confidence (inferred from project documentation)
