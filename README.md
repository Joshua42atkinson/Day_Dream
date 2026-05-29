# Daydream — Words Are Spells

**A Meaning-Making Engine Where Vocabulary Is Magic**

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/bevy-0.18-blue.svg)](https://bevyengine.org/)
[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

> **Letters spell words. Words cast spells. The student is a wizard learning to wield meaning itself.**

---

## What Is Daydream?

Daydream is a **sovereign, local-first educational engine** built on [Bevy ECS](https://bevyengine.org/) that teaches vocabulary through experiential narrative — not definitions.

Every word in a curriculum is a **spell card** with a Channel (Mind/Heart/Body/Action), a story moment, and a Socratic depth question. The student swipes through an interactive Choose-Your-Own-Adventure, building a **SpellBook** of mastered word-spells. Their journey reveals how they think — which channels they favor, which they avoid — reflected back as a living **Character Sheet**.

This is **VAAM**: Vocabulary Acquisition Autonomous Meaning. A word isn't defined by other words. It's defined by experience.

### The Core Mechanic

```
ENCOUNTER the word → EXPERIENCE it through story → OWN it as a spell
        ↑                                                    │
        └────────────── The Great Recycler ──────────────────┘
```

### The Triple Sandwich

Every slide delivers the word across three channels simultaneously:

| Layer | Channel | Experience |
|-------|---------|------------|
| **Card** (top) | Mind | *What is this word?* |
| **Story** (middle) | Heart | *What does it feel like?* |
| **Setting** (bottom) | Body | *What's the vibe right now?* |

When all three land, the student doesn't memorize — they **Flow**.

---

## Quick Start

```bash
# Build the engine
cargo build -p daydream-engine

# Run it
cargo run -p daydream-engine
```

**Controls:**
- `→` or `D` — Swipe Right (Yes / Accept)
- `←` or `A` — Swipe Left (No / Reject)
- `↓` or `S` or `Space` — Dig Deeper (VAAM depth prompt)
- Mouse drag also works for all three directions

---

## The Great Game Framework

Daydream's game design is powered by [*The Great Game*](2025_07_23%20great%20game%20book.md) — a self-mastery framework that maps directly to ECS mechanics:

| Game Element | Great Game Concept | ECS Implementation |
|---|---|---|
| Card elements | Four Channels of Consciousness | `Channel` enum (Mind 🟢, Heart 🟠, Body 🔵, Action 🟡) |
| Difficulty tiers | Four Stages | `Stage` enum (Hero → Outlaw → Edge Lord → Best Self) |
| Crafting | The Great Recycler | Core gameplay loop: encounter → experience → own |
| Collection | SpellBook | `SpellBook` resource with mastery levels |
| Synergies | Five Phases / Generation Cycle | `SynergyLinks` component on word entities |
| Player profile | Character Sheet | `CharacterSheet` resource with attunement scores |
| Classes | Emergent archetypes | Auto-detected from play patterns (Oracle, Bard, Templar...) |

---

## Architecture

```
engine/
├── Cargo.toml          # Bevy 0.18 + serde
└── src/
    ├── main.rs         # App setup, system registration
    ├── components.rs   # Full ECS type system (Channel, Stage, SpellPower, etc.)
    ├── dag.rs          # Curriculum DAG loading + demo data
    ├── input.rs        # Swipe gesture detection (mouse + keyboard)
    └── render.rs       # Triple Sandwich visuals + trail review
```

### Key Types

- **`WordCard`** — The spell entity. Word, depth prompt, themes.
- **`Channel`** — Mind/Heart/Body/Action with color derivation.
- **`Stage`** — Hero/Outlaw/EdgeLord/BestSelf with star indicators.
- **`SpellPower`** — Per-word mastery tracking (Encountered → Experienced → Owned → Mastered).
- **`CharacterSheet`** — Student profile with channel attunement and emergent class detection.
- **`SpellBook`** — Collection of all word-spells the student has engaged with.

---

## Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| **Framework** | ✅ Done | Great Game type system, demo curriculum, Triple Sandwich rendering |
| **Curriculum Authoring** | 🔲 Next | JSON loader, parent-facing authoring tool |
| **Persistence** | 🔲 | Save/load SpellBook + CharacterSheet across sessions |
| **Visual Polish** | 🔲 | Generated card art, animations, custom typography |
| **AI Narrator** | 🔲 | WebLLM + fine-tuned Gemma 4 E2B per curriculum |
| **Mobile** | 🔲 | WASM compile, PWA wrapper, touch swipe |

Full details: **[docs/MASTER_DESIGN_DOC.md](docs/MASTER_DESIGN_DOC.md)**

---

## Who This Is For

| User | Role |
|------|------|
| **Parent** | Authors word-DAG curricula. Chooses words, writes stories, sets moods. |
| **Child** | Plays through the story. Swipes to navigate. Builds a SpellBook. |
| **AI** | (Future) Narrates the journey. Fine-tuned Gemma model, runs on-device. |

---

## Privacy & Sovereignty

- ✅ **100% Local** — No cloud. No accounts. No telemetry.
- ✅ **On-Device AI** — Fine-tuned models run in-browser via WebLLM.
- ✅ **Open Source** — GPL-3.0. Auditable. Forkable. Forever.
- ✅ **COPPA/GDPR** — Privacy-first by architecture, not by policy.

---

## License

**GNU General Public License v3.0 (GPLv3)**

All derivatives must remain open source. Education should remain a public good.

---

**Built with ❤️ in Rust + Bevy** | **Privacy by Design** | **Words Are Spells**
