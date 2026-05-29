# Session Summary — May 29, 2026 Evening
## Daydream LitRPG Platform Architecture

---

## Vision Statement

> A **sovereign, choose-your-own-adventure life LitRPG system**. The player IS the character. 
> They create themselves (Character Sheet), build their own spell deck (SpellBook), author their own 
> word-DAG journey (Journey Editor), then **play as themselves** inside their self-created world.
>
> Vocabulary is the fulcrum. The Great Game is the framework. Privacy is the default.

---

## Domain Strategy

| Domain | Role | Audience |
|--------|------|----------|
| **playdaydream.com** | **PLAY MODE** — The game engine. Character creator, deck builder, live play, trail review. | Players (students) |
| **greatrecycler.com** | **PUBLISH MODE** — Discovery hub. Published journeys, community decks, archetype leaderboards (privacy-preserving). | Authors, parents, community |

Both sites share the same StoryGraph JSON format. An author can:
1. Build on playdaydream.com (self-authoring)
2. Export their journey as StoryGraph JSON
3. Publish to greatrecycler.com for others to discover and play

---

## What Was Built Today

### 1. playdaydream.com Deployment Pipeline
- `vercel.json` — Vite build config, SPA rewrites, security headers
- `.github/workflows/deploy_playdaydream.yml` — CI/CD to Vercel on push to main
- PWA support via `vite-plugin-pwa` (service worker, manifest, offline play)
- 5 atmospheric placeholder images for demo adventures
- DNS configured: A record → `76.76.21.21`, CNAME `www` → `cname.vercel-dns.com`

### 2. Shareable Adventure URLs
- React Router with `/play/:adventureId` and `/custom` routes
- Home page uses `navigate()` instead of local state
- Direct links work: `playdaydream.com/play/bias-and-mirrors`

### 3. StoryGraph ↔ React Bridge
- `src/adapters/storygraph.js` — Converts Rust StoryGraph JSON → React curriculum format
- Channel auto-detection from `subject_word` with fallback table
- Virtue auto-detection from choice label keywords
- Depth question extraction from `content` using `---` delimiter
- File upload on home page: "Load StoryGraph JSON" → plays at `/custom`

### 4. Rust Data Model Expansion
- `StoryNode` gains: `channel: Option<String>`, `depth: Option<String>`
- `StoryChoice` gains: `virtue: Option<String>`
- `StoryGraph` gains: `description: Option<String>`, `age_range: Option<String>`
- All structs derive `Default` for forward-compatible struct literals
- Leptos property editor updated with channel dropdown, depth textarea, virtue input

### 5. Compilation Fixes
- Fixed all Rust struct-literal sites with `..Default::default()`
- `common` crate compiles clean
- `frontend` crate: zero errors in touched files; 9 pre-existing errors in `pearl_setup.rs` only

---

## Current Codebase State

### playdaydream/ (React Player)
```
src/
  main.jsx          → Entry point
  App.jsx           → Router: /, /play/:id, /custom
  adapters/
    storygraph.js   → Rust JSON → React curriculum adapter
  data/
    curriculum.js   → Hardcoded demo adventures (Bias & Mirrors, Feelings Garden)
    constants.js    → Channel enum, Mastery enum
  hooks/
    useSwipeGesture.js    → Mouse/touch drag, swipe detection, double-tap
    useAmbientDrone.js    → Web Audio API sine wave drone
    useStudentTrail.js    → localStorage persistence, trail, SpellBook, emergent class
  pages/
    Home.jsx        → Adventure picker + StoryGraph JSON upload
    Home.css        → Styling for cards + upload section
    Play.jsx        → Core game: swipe-card narrative, depth overlay, end screen
```

### Deployment
- **GitHub:** `Joshua42atkinson/Day_Dream` — `a493a01` pushed to `main`
- **Vercel:** Connected, builds clean, `day-dream-xxx.vercel.app` active
- **DNS:** Squarespace A + CNAME records set, domain ready for Vercel verification

### Not Yet Built (Next Session Priority)
1. **Character Creator** (`/create/character`) — Name, archetype, channel sliders
2. **Spell Deck Builder** (`/create/deck`) — Drag/tap word-spells into personal deck
3. **Journey Author** (`/create/journey`) — Visual node editor in React
4. **Enhanced Player** — Character Sheet overlay, spell hand display, live synergy
5. **SpellBook persistence** — Cross-session save/load, mastery tracking
6. **greatrecycler.com** — Discovery hub for published journeys

---

## Technical Notes for Next Session

### React App Architecture
- Vite + React 19 + vanilla CSS (no framework needed for PWA)
- `react-router-dom` for client-side routing
- All state in `localStorage` (sovereign, no backend required for solo play)
- PWA precaches all assets for offline play after first visit

### Data Flow
```
Author (Leptos tool) → StoryGraph JSON → playdaydream.com /custom (upload)
                                     → greatrecycler.com (publish for discovery)
                                     → playdaydream.com /play/:id (hardcoded or fetched)
```

### Key Files to Extend
- `src/App.jsx` — Add `/create/*` routes
- `src/hooks/useStudentTrail.js` — Expand to full Character Sheet
- `src/data/curriculum.js` — Add ARCANA word library (expand beyond 2 adventures)
- `src/adapters/storygraph.js` — Already handles full StoryGraph format

### Great Game Concepts to Surface in UI
- **Four Channels:** Mind (Sage/Wood), Heart (Mystic/Fire), Body (Healer/Water), Action (Builder/Earth)
- **Four Stages:** Hero ★, Outlaw ★★, EdgeLord ★★★, BestSelf ★★★★
- **Five Symbols:** ◆ Stone (noun), ◇ Spark (verb), △ Prism (adj), ○ Void (abstract), ☆ Star (key term)
- **Mastery Tiers:** Encountered 🔮 → Experienced ⚡ → Owned 🌟 → Mastered 👑
- **Emergent Classes:** Oracle, Bard, Cultivator, Templar, Architect, Hermeticist
- **The Recycler:** Spaced repetition through natural story replay

---

## Files Created/Modified Today

| File | Action | Purpose |
|------|--------|---------|
| `vercel.json` | Created | Vercel build + SPA rewrites |
| `.github/workflows/deploy_playdaydream.yml` | Created | CI/CD pipeline |
| `playdaydream/vite.config.js` | Modified | PWA plugin + manifest |
| `playdaydream/index.html` | Modified | Apple touch icon |
| `playdaydream/src/App.jsx` | Modified | React Router |
| `playdaydream/src/pages/Home.jsx` | Modified | Shareable URLs + file upload |
| `playdaydream/src/pages/Home.css` | Modified | Upload section styles |
| `playdaydream/src/adapters/storygraph.js` | Created | StoryGraph → curriculum adapter |
| `playdaydream/public/images/*.png` | Created | 5 placeholder images |
| `playdaydream/public/icons/*.png` | Created | PWA icons (192, 512) |
| `common/src/expert.rs` | Modified | channel, depth, virtue, age_range, description |
| `frontend/src/components/authoring/property_editor.rs` | Modified | New fields in Leptos editor |
| `frontend/src/components/authoring/node_canvas.rs` | Modified | ..Default::default() fixes |
| `frontend/src/pages/daydream.rs` | Modified | ..Default::default() fixes |

---

## Session Status

**Deployment:** ✅ playdaydream.com live on Vercel (DNS configured)
**PWA:** ✅ Offline-capable after first visit
**Shareable URLs:** ✅ /play/:id works
**StoryGraph Upload:** ✅ File upload → instant play
**Rust Model:** ✅ New fields backward-compatible
**Leptos Editor:** ✅ Channel, depth, virtue editable

**Next Gate:** Character Creator + Deck Builder + Journey Author (self-authoring trinity)

---

*Prepared for next session: solo LitRPG build phase*
*Domains: playdaydream.com (play) + greatrecycler.com (publish)*
*Framework: The Great Game by Joshua Atkinson*
