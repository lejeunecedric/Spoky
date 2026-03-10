# Stack Research

**Domain:** Tauri multi-protocol desktop messenger (WhatsApp, Signal, Discord)
**Researched:** 2026-03-10
**Confidence:** HIGH (official docs verified) / MEDIUM (protocol libraries)

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Tauri | 2.0+ | Desktop framework | Uses system webview for <600KB apps; Rust core for safety; official security audits; mobile-ready for future expansion |
| Svelte | 5.25+ | UI framework | Smallest bundle size of major frameworks; compile-time optimization; runes API for reactivity; ideal for lightweight apps |
| SvelteKit | 2.20+ | Meta-framework | File-based routing; adapter-static for SPA mode; first-class Tauri support |
| TypeScript | 5.x | Type system | Essential for protocol message handling; Svelte 5 has excellent TS support |
| Rust | 1.77+ | Backend core | Tauri native; memory-safe; handles protocol state management and IPC |

### Protocol Bridges (Sidecar Architecture)

| Library | Version | Protocol | Why |
|---------|---------|----------|-----|
| whatsapp-web.js | 1.34+ | WhatsApp | Only maintained WhatsApp Web bridge; 21K stars; multi-device support; Puppeteer-based |
| signal-cli | 0.14+ | Signal | Official libsignal wrapper; JSON-RPC interface; Java-based (GraalVM native available) |
| discord.js | 14.15+ | Discord | Official Discord library; 100% API coverage; mature ecosystem; Node.js native |

### Storage & State

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| @tauri-apps/plugin-store | 2.x | Settings & config | JSON key-value store; persists to file; async; works from both Rust and JS |
| @tauri-apps/plugin-sql | 2.x | Message cache | SQLite for structured message storage; migrations support; sqlx-based |

### Styling

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Tailwind CSS | 4.x | Utility-first CSS | Zero-runtime; SvelteKit integration; dark mode built-in; minimal bundle impact |

### Tauri Plugins (Required)

| Plugin | Purpose | Notes |
|--------|---------|-------|
| shell | Sidecar management | Required to spawn Node.js sidecar processes |
| store | Persistent settings | App configuration, account state |
| sql | Message storage | Local message cache (optional for v1) |
| single-instance | Prevent duplicates | Ensure only one app instance runs |
| notification | Desktop alerts | Message notifications |
| window-state | Window persistence | Remember position/size |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri Desktop App                       │
├────────────────────────┬────────────────────────────────────┤
│   SvelteKit Frontend   │         Rust Core (Tauri)          │
│   (SPA, TypeScript)    │   - IPC layer                       │
│   - Unified inbox UI   │   - State management                │
│   - Chat views         │   - Sidecar orchestration           │
│   - Settings UI        │   - Message aggregation             │
├────────────────────────┴────────────────────────────────────┤
│                    Tauri Shell Plugin                       │
│              (Sidecar Process Management)                    │
├──────────────┬──────────────┬───────────────────────────────┤
│ Node Sidecar │ Signal CLI   │ Node Sidecar                  │
│ (WhatsApp)   │ (GraalVM)    │ (Discord)                     │
│ whatsapp-    │ signal-cli   │ discord.js                    │
│ web.js       │ JSON-RPC     │                               │
└──────────────┴──────────────┴───────────────────────────────┘
```

### Sidecar Pattern Rationale

**Why sidecars instead of pure Rust?**

1. **WhatsApp**: No Rust library exists. WhatsApp Web protocol is undocumented and changes frequently. whatsapp-web.js is the only actively maintained bridge (21K+ stars).

2. **Signal**: libsignal (Rust) exists but only handles cryptography. signal-cli wraps the full Signal service layer including account management, group sync, and message delivery.

3. **Discord**: discord.js is the official library with 100% API coverage. No Rust alternative has equivalent maturity.

**Trade-off accepted**: Sidecars add complexity but are the only viable path. Using `@yao-pkg/pkg` bundles Node.js runtime into ~40MB binaries per platform.

## Installation

```bash
# Create Tauri project with SvelteKit
npm create tauri-app@latest spoky -- --template svelte-ts

# Frontend dependencies
cd spoky
npm install
npm install -D @sveltejs/adapter-static
npm install tailwindcss @tailwindcss/vite

# Tauri plugins
npm run tauri add shell
npm run tauri add store
npm run tauri add sql
npm run tauri add single-instance
npm run tauri add notification
npm run tauri add window-state

# Sidecar project setup
mkdir -p sidecar
cd sidecar
npm init -y
npm install whatsapp-web.js discord.js qrcode-terminal
npm install -D @yao-pkg/pkg @types/node
```

### Sidecar Package Scripts

```json
{
  "scripts": {
    "build": "pkg . --output ../src-tauri/binaries/spoky-bridge",
    "build:all": "pkg . --targets node18-linux-x64,node18-macos-x64,node18-win-x64 --output ../src-tauri/binaries/spoky-bridge"
  }
}
```

### SvelteKit Configuration

```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({ fallback: 'index.html' })
  }
};
```

```typescript
// src/routes/+layout.ts
export const ssr = false; // Required for Tauri APIs
```

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Desktop | Tauri | Electron | Electron bundles Chromium (~150MB); Tauri uses system webview (<600KB) |
| Desktop | Tauri | Neutralino | Less mature; smaller ecosystem; weaker IPC story |
| Frontend | Svelte | React | React runtime larger; Svelte compiles away for smaller bundles |
| Frontend | Svelte | Vue | Vue 3 good, but Svelte 5 runes are more ergonomic for reactivity |
| WhatsApp | whatsapp-web.js | Baileys (TS) | Baileys has more frequent breaking changes; whatsapp-web.js is more stable |
| Signal | signal-cli | libsignal (Rust) | libsignal only provides crypto primitives; signal-cli has full service layer |
| Discord | discord.js | serenity (Rust) | serenity is bot-focused; discord.js better for user account simulation |
| Storage | plugin-sql | plugin-store | SQL overkill for v1; plugin-store simpler for settings/config |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Electron | Massive bundle size (~150MB); contradicts "lightweight" goal | Tauri |
| Next.js | SSR-incompatible with Tauri; overkill for desktop | SvelteKit with adapter-static |
| libsignal (Rust) directly | Only crypto primitives, not full Signal client | signal-cli via JSON-RPC |
| Any SQLite wrapper outside Tauri | Breaks Tauri security model | @tauri-apps/plugin-sql |
| Global state libraries (Redux, etc.) | Svelte 5 runes provide built-in reactivity | $state, $derived runes |
| Puppeteer in main process | whatsapp-web.js spawns its own; conflicts likely | Let sidecar manage its Puppeteer |

## Stack Patterns by Variant

**If RAM is critical priority:**
- Use signal-cli GraalVM native build (~15MB) instead of JVM
- Consider lazy-loading sidecars only when account is connected
- Cache messages aggressively in SQLite to reduce polling

**If development speed is priority:**
- Run sidecars as separate Node processes during development
- Use `npm run dev` for each sidecar with hot reload
- Bundle for production only

**If multi-platform builds needed:**
- GitHub Actions matrix for linux-x64, macos-x64, macos-arm64, windows-x64
- Each platform needs its own sidecar binary with target triple suffix
- signal-cli provides pre-built binaries for all platforms

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Tauri 2.x | Rust 1.77+ | Minimum Rust version requirement |
| SvelteKit 2.x | Svelte 5.x | Svelte 4 not supported |
| discord.js 14.x | Node 22.12+ | Requires modern Node features |
| whatsapp-web.js | Node 18+ | Puppeteer dependency |
| signal-cli | Java/JRE 25+ | Or GraalVM native binary |
| Tailwind 4.x | Vite 5+ | Uses new Vite plugin architecture |

## Confidence Assessment

| Area | Confidence | Reason |
|------|------------|--------|
| Tauri + SvelteKit | HIGH | Official Tauri docs verify integration; tested pattern |
| Tailwind integration | HIGH | Official Tailwind docs for SvelteKit |
| Protocol libraries | MEDIUM | Third-party libraries; API stability varies |
| Sidecar architecture | MEDIUM | Well-documented pattern but adds complexity |
| Signal integration | MEDIUM | signal-cli is mature but requires Java runtime |

## Sources

- **Tauri 2.0 Official Docs** — https://v2.tauri.app/ (HIGH confidence, verified Jan 2026)
- **SvelteKit Tauri Integration** — https://v2.tauri.app/start/frontend/sveltekit/ (HIGH confidence)
- **Tauri Node.js Sidecar Guide** — https://v2.tauri.app/learn/sidecar-nodejs/ (HIGH confidence)
- **whatsapp-web.js GitHub** — https://github.com/pedroslopez/whatsapp-web.js (MEDIUM confidence, third-party)
- **signal-cli GitHub** — https://github.com/AsamK/signal-cli (MEDIUM confidence, unofficial but mature)
- **libsignal GitHub** — https://github.com/signalapp/libsignal (HIGH confidence, official but incomplete for our use)
- **discord.js Docs** — https://discord.js.org/docs/packages/discord.js/14.15.0 (HIGH confidence, official)
- **Tailwind SvelteKit Guide** — https://tailwindcss.com/docs/installation/framework-guides/sveltekit (HIGH confidence)
- **@tauri-apps/plugin-store** — https://v2.tauri.app/plugin/store/ (HIGH confidence)
- **@tauri-apps/plugin-sql** — https://v2.tauri.app/plugin/sql/ (HIGH confidence)

---
*Stack research for: Spoky multi-protocol messenger*
*Researched: 2026-03-10*
