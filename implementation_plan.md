# Master Design & Implementation Plan: Daydream Rebirth
### *The Branching Somatic Sandbox (Choose-Your-Own-Adventure slide engine)*

This document serves as the **Master Design & Implementation Plan** (Milestone 1) to transition the Daydream project from a complex, over-engineered 3D VR environment into a premium, responsive, **branching somatic slide graph engine**.

---

## User Review Required

> [!IMPORTANT]
> **Pedagogical Shift:**
> We are collapsing the 3D NPC sandbox into a 2D directed-graph slide canvas. The student moves slide-by-slide. Each slide represents a conceptual sanctuary containing exactly:
> *   **One Story** (2-3 sentences of context)
> *   **One Song** (Reference frequency/pitch)
> *   **One Image** (Soft-ambient backdrop)
> *   **One Subject Word** (Vocabulary-as-a-Mechanic / VAAM)
> 
> Unlocking a slide's branching choices requires matching the reference tone via the PLING! pitch gate (sing/hum).

---

## Proposed Technical Changes

```
day_dream/
  ├── common/
  │    └── src/expert.rs        <── Enriched StoryNode & StoryChoice models
  ├── backend/
  │    └── src/
  │         ├── main.rs         <── Cleaned Axum endpoints, Bevy command handlers
  │         └── ai/             <── Stripped legacy Candle engine placeholders
  └── frontend/
       └── src/
            ├── models.rs       <── Client model updates
            ├── components/
            │    └── authoring/ <── Extended PropertyEditor & NodeCanvas
            └── pages/
                 └── daydream.rs <── Beautiful Somatic Adventure Player
```

---

### 1. Data Model Upgrade (`common`)

We will enrich `common/src/expert.rs` to represent a clean, branching slide node.

#### [MODIFY] [expert.rs](file:///home/joshua/Workflow/Other/Day_Dream/common/src/expert.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryChoice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub leads_to: String,              // Destination StoryNode ID
    pub pitch_gate: Option<f32>,       // PLING! pitch gate (target Hz)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub content: String,               // Story segment
    pub x: f64,
    pub y: f64,
    // Somatic & Media fields:
    pub subject_word: String,          // active vocabulary word (VAAM)
    pub image_url: Option<String>,      // Background illustration
    pub audio_url: Option<String>,      // backing music or reference sound
    pub target_freq: Option<f32>,       // reference pitch for somatic gate
    pub choices: Vec<StoryChoice>,     // branching choices
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryGraph {
    pub id: String,
    pub title: String,
    pub nodes: Vec<StoryNode>,
    pub connections: Vec<Connection>,
}
```

---

### 2. Frontend Models (`frontend`)

We will synchronize the Leptos client models.

#### [MODIFY] [models.rs](file:///home/joshua/Workflow/Other/Day_Dream/frontend/src/models.rs)
- Remove `GameTurn` and redundant structures.
- Import standard `StoryNode`, `StoryChoice`, and `StoryGraph` directly from the shared `common` crate.

---

### 3. Visual Canvas & Property Editor (`frontend/components/authoring/`)

We will upgrade the visual authoring suite to allow parents (SMEs) to easily build branching curriculum graphs.

#### [MODIFY] [property_editor.rs](file:///home/joshua/Workflow/Other/Day_Dream/frontend/src/components/authoring/property_editor.rs)
- Update inputs to support the new somatic fields:
  - **Subject Word** (Input textbox)
  - **Image Path** (Input textbox)
  - **Audio Path** (Input textbox)
  - **Target Frequency** (Numeric input slider/box)
- Build a sub-form to add, edit, and delete multiple `StoryChoice` items, including their descriptive labels, target destinations, and pitch requirements.

#### [MODIFY] [node_canvas.rs](file:///home/joshua/Workflow/Other/Day_Dream/frontend/src/components/authoring/node_canvas.rs)
- Adapt the node dragging and drawing logic to support ports mapped directly to choices.
- Ensure the toolbar, save handler, and background grid utilize our enriched HSL theme tokens (`slate-950`, `cyan-500`, etc.).

---

### 4. The Somatic Adventure Player (`frontend/pages/`)

This is the gameplay engine, built directly inside your student console.

#### [MODIFY] [daydream.rs](file:///home/joshua/Workflow/Other/Day_Dream/frontend/src/pages/daydream.rs)
- Rebuild the page into a swipeable slide layout with slow, anti-dopamine transition states:
  - **Intro**: Slowly fades in the semi-transparent background image, plays the tone using Web Audio API, and shows the context text.
  - **Mic Gate**: Renders a beautiful glassmorphic "somatic breathing" needle using standard browser microphone detection to match `target_freq` (if required).
  - **Choose**: Unlocks choice buttons that update character state when selected.

---

### 5. Bevy ECS Integration (`backend`)

We will wire choice selections directly into your Bevy character engine to log state.

#### [MODIFY] [main.rs](file:///home/joshua/Workflow/Other/Day_Dream/backend/src/main.rs)
- Clear redundant Candle inference endpoints.
- Update `/api/quest/action` to push a choice event into the Bevy world.
- Ensure systems like `sync_yarn_to_story_progress` read the selected choice data to update `VirtueTopology` (valor, compassion, self-efficacy) and `CognitiveLoad` state variables.

---

## Verification Plan

### Automated Verification
*   **Compile Tests**: Run `cargo check --workspace` to ensure all Rust workspaces are fully aligned with the new data schemas.
*   **Wasm Compiling**: Run `cargo leptos build` to verify the Leptos frontend compiles cleanly.

### Manual Verification
1.  **Authoring Check**: Open the visual Node Canvas at `/authoring`. Create a node, add two choices, assign a backing audio frequency, and click "Save Graph."
2.  **Gameplay Check**: Open the gameplay console. Play through a node, sing/hum to match the pitch gate, select a choice, and verify that the console moves to the next node and logs the updated virtues correctly.
