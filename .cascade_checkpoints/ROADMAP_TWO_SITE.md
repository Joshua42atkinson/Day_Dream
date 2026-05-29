# Two-Site Architecture Roadmap
## playdaydream.com + greatrecycler.com

---

## Domain Division

### playdaydream.com — The Solo Engine (PLAY MODE)
> "Enter the Great Game. You are the Player."

**Core Loop:**
```
Character Creator → Deck Builder → Journey Author → Live Play → Trail Review
```

**Pages:**
| Route | Purpose |
|-------|---------|
| `/` | Landing: choose Create, Play, or Review |
| `/create/character` | Name, archetype, channel attunement sliders |
| `/create/deck` | Browse ARCANA library, build personal spell deck |
| `/create/journey` | Visual node editor: word-DAG + Mad Libs story slots |
| `/play/:id` | Live LitRPG session (existing swipe player, enhanced) |
| `/play/custom` | Test-run a journey from the editor |
| `/review` | SpellBook, Character Sheet evolution, Resonance threads |
| `/library` | Local save slots (multiple characters + journeys) |

**Key Principle:** Everything is local-first. No accounts. No cloud. Player owns their data.

---

### greatrecycler.com — The Discovery Hub (PUBLISH MODE)
> "The Great Recycler: transmute experience into shared wisdom."

**Core Loop:**
```
Browse Journeys → Preview → Import to playdaydream.com → Play → Rate/Reflect
```

**Pages:**
| Route | Purpose |
|-------|---------|
| `/` | Featured journeys, archetype filters, search |
| `/journey/:id` | Published journey page with preview + import button |
| `/author/:handle` | Author profile (optional, pseudonymous) |
| `/library` | Categorized browsing: by channel, stage, genre, age |
| `/about` | The Great Game philosophy, how to self-author |

**Key Principle:** Discovery without surveillance. No tracking. Optional attribution.

---

## Shared Data Format

Both sites speak **StoryGraph JSON** (already defined in `common/src/expert.rs`):

```json
{
  "id": "journey-slug",
  "title": "The Forest of Mirrors",
  "description": "A somatic exploration of cognitive bias...",
  "age_range": "10-14",
  "nodes": [
    {
      "id": "node-uuid",
      "title": "The Threshold",
      "content": "Story text with ---Depth question?",
      "subject_word": "Presence",
      "channel": "body",
      "depth": "What does your body feel right now?",
      "image_url": "/images/forest.png",
      "target_freq": 174.6,
      "choices": [
        {
          "id": "choice-uuid",
          "label": "Step forward",
          "leads_to": "next-node-uuid",
          "virtue": "courage"
        }
      ]
    }
  ],
  "connections": [...]
}
```

**Export Flow:**
1. Author finishes journey on playdaydream.com
2. Clicks "Export StoryGraph"
3. Gets `.json` file + optional card art `.png` assets
4. Can upload to greatrecycler.com (manual or API)
5. greatrecycler.com validates format, generates preview, lists publicly

---

## The Self-Authoring Trinity

### 1. Character Creator (`/create/character`)
```
┌─────────────────────────────────────┐
│  Who are you, Player?               │
│                                     │
│  Name: [________]                   │
│                                     │
│  Starting Archetype:                 │
│  ○ The Oracle   (Mind +10%)          │
│  ○ The Bard     (Heart +10%)        │
│  ○ The Cultivator (Body +10%)        │
│  ○ The Templar  (Action +10%)        │
│  ○ The Architect (Balanced)          │
│                                     │
│  Channel Attunement:                 │
│  Mind:   [████████░░] 80%            │
│  Heart:  [██████░░░░] 60%            │
│  Body:   [██████████] 100%           │
│  Action: [████░░░░░░] 40%            │
│                                     │
│  [Create Character]                 │
└─────────────────────────────────────┘
```
- Saved to localStorage as `dd_character`
- Channel attunements affect: card draw weights, story tone, archetype evolution

### 2. Spell Deck Builder (`/create/deck`)
```
┌─────────────────────────────────────┐
│  Your SpellBook                     │
│  12 / 20 spells selected             │
│                                     │
│  ARCANA Library:        Your Deck:  │
│  ┌─────────┐           ┌─────────┐ │
│  │Presence │           │Resilience│ │
│  │[Body] ◆ │  [+]  →   │[Action] ◆│ │
│  │★★      │           │★★★     │ │
│  └─────────┘           └─────────┘ │
│  ┌─────────┐           ┌─────────┐ │
│  │Bias     │           │Patience  │ │
│  │[Mind] ○ │  [+]  →   │[Body] △ │ │
│  │★★★     │           │★★      │ │
│  └─────────┘           └─────────┘ │
│                                     │
│  Synergy preview: Resilience +     │
│  Patience = "Steadfast" (+2 power)  │
│                                     │
│  [Save Deck]                        │
└─────────────────────────────────────┘
```
- Browse full ARCANA word library (filter by channel, stage, symbol)
- Select up to 20 spells for personal deck
- Hand size = 3-5 (randomly drawn from deck each session)
- Synergy preview shows when linked words are both in deck

### 3. Journey Author (`/create/journey`)
```
┌─────────────────────────────────────┐
│  Journey Author                     │
│                                     │
│  [Add Node]  [Connect]  [Test Run]  │
│                                     │
│        ┌─────────┐                 │
│        │Threshold│                 │
│        │Presence │                 │
│        └───┬─────┘                 │
│      ┌─────┴─────┐                  │
│      ▼           ▼                   │
│ ┌─────────┐  ┌─────────┐            │
│ │Mirror   │  │Sanctuary│            │
│ │Bias     │  │Withdraw │            │
│ └────┬────┘  └─────────┘            │
│      ▼                               │
│ ┌─────────┐                          │
│ │Crucible │                          │
│ │Resilience│                          │
│ └─────────┘                          │
│                                     │
│  Click node to edit:                 │
│  Word: [Presence]                   │
│  Story: "You stand before..."       │
│  Mad Libs: "In the [forest], a      │
│    [student] begins to [breathe]."  │
│  Depth: "What does presence feel      │
│    like in your body?"              │
│                                     │
│  [Save Journey] [Export JSON]       │
└─────────────────────────────────────┘
```
- Simpler than Leptos node editor — built for speed
- Mad Libs slots: Setting [◆], Subject [☆], Action [◇], Modifiers [△]
- Export produces StoryGraph JSON compatible with both sites
- "Test Run" immediately plays in `/play/custom`

---

## Enhanced Live Play (`/play/:id`)

### Current State
- Swipe-card narrative with ambient audio
- Double-tap for depth question
- End screen with trail + emergent class

### Target State
```
┌─────────────────────────────────────┐
│  [Character: "The Oracle" — Mind 85%]│  ← Top bar, swipe down to expand
│                                     │
│  ┌─────────────────────────────┐    │
│  │                             │    │
│  │    [Story card: Bias]       │    │
│  │    "The mirror whispers..." │    │
│  │                             │    │
│  │    ○ Swipe RIGHT: Examine │    │
│  │    ○ Swipe LEFT: Turn away│    │
│  │    ○ Swipe DOWN: Dig deeper│    │
│  │                             │    │
│  └─────────────────────────────┘    │
│                                     │
│  Your Hand:                          │
│  ┌─────┐ ┌─────┐ ┌─────┐          │
│  │Courage│ │Patience│ │Bias   │    │
│  │[Heart]│ │[Body]  │ │[Mind] │    │
│  │★★    │ │★★★   │ │★★★  │    │
│  └─────┘ └─────┘ └─────┘          │
│                                     │
│  [Cast a spell →]                   │
│                                     │
│  SpellBook: 12 words  [View]       │
└─────────────────────────────────────┘
```

**New Mechanics:**
1. **Character Sheet overlay** — Swipe down from top to see full stats
2. **Spell hand at bottom** — 3-5 cards from personal deck, swipe to cast
3. **Casting = playing a spell card** — Each card has: word, channel, symbol, power
4. **Synergy glow** — When two linked spells are in hand together, borders pulse
5. **Live mastery** — "Courage upgraded: Experienced → Owned" toast
6. **Recycler hint** — "Patience will return, evolved" when card goes to discard

---

## Data Model Expansion (React Side)

### Character Sheet (new)
```js
{
  name: "Player",
  archetype: "Oracle",        // starting choice
  attunement: {
    mind: 0.85,
    heart: 0.60,
    body: 0.40,
    action: 0.30
  },
  emergentClass: "The Oracle", // derived from play patterns
  wordsEncountered: 12,
  totalCasts: 47,
  createdAt: "2026-05-29"
}
```

### Spell Deck (new)
```js
{
  name: "My First Deck",
  spells: [
    { word: "Resilience", channel: "action", symbol: "◆", stage: "Outlaw" },
    { word: "Patience", channel: "body", symbol: "△", stage: "Hero" }
  ],
  synergies: [
    { pair: ["Resilience", "Patience"], name: "Steadfast", bonus: 2 }
  ]
}
```

### SpellBook (extend existing)
```js
// Currently in useStudentTrail.js — expand to full SpellBook
{
  entries: [
    {
      word: "Resilience",
      channel: "action",
      mastery: "Owned",       // Encountered | Experienced | Owned | Mastered
      timesCast: 7,
      resonanceWith: ["Patience", "Courage"]
    }
  ]
}
```

---

## greatrecycler.com Implementation Plan

**Phase 1: Static Showcase**
- Astro or Next.js static site
- Hardcoded featured journeys (same JSON format)
- Import button: downloads `.json` + instructions to upload to playdaydream.com

**Phase 2: Dynamic Discovery**
- Simple backend (optional): accepts StoryGraph JSON uploads
- Generates preview images from card art
- Search/filter by channel, stage, age range
- "Import to Play" button: opens playdaydream.com with URL-encoded JSON

**Phase 3: Community**
- Optional pseudonymous author profiles
- Rating system (1-5 stars, no text reviews to avoid moderation)
- "Most played this week" leaderboard (privacy-preserving, count only)

---

## Next Session Checklist

### High Priority (Character + Deck)
- [ ] Create `src/pages/CharacterCreator.jsx`
- [ ] Create `src/hooks/useCharacterSheet.js` (localStorage)
- [ ] Create `src/pages/DeckBuilder.jsx`
- [ ] Expand `src/data/curriculum.js` into full ARCANA word library (20+ words)
- [ ] Add character/deck to `useStudentTrail.js` integration

### Medium Priority (Journey Author)
- [ ] Create `src/pages/JourneyAuthor.jsx` (simplified React node editor)
- [ ] Mad Libs slot editor per node
- [ ] Export StoryGraph JSON button
- [ ] "Test Run" → `/play/custom` flow

### Lower Priority (Enhanced Play)
- [ ] Character Sheet overlay in Play
- [ ] Spell hand display at bottom
- [ ] Casting animation + synergy glow
- [ ] Live mastery toast notifications

### Infrastructure
- [ ] Set up greatrecycler.com repo or folder
- [ ] Create static Astro site scaffold
- [ ] Design featured journey showcase page

---

*Vision: Every person is the author of their own Great Game. 
playdaydream.com is the forge. greatrecycler.com is the library.*
