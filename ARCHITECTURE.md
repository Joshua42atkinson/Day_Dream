# ASK PETE: SYSTEM ARCHITECTURE & CONTENT STRATEGY

**Mission:** To operationalize "Physical AI" in education through a Rust-powered, Local-First ecosystem that balances Cognitive Load (Coal) with Mastery (Steam).

## 1. THE SHARED BRAIN (CORE LIBRARY)

**Crate Name:** `ask_pete_core`
**Path:** `/core`

This is the shared Rust library used by ALL interfaces. It ensures that the physics of learning are identical whether you are on a Server, a Desktop, or a Phone.

- **`models/`**: Defines the `TrainCar` (Lesson), `Cargo` (Vocab), and `StudentState` structs.
- **`physics/`**: The `CognitiveLoad` calculator.
  - *Logic:* Inputs text complexity + user engine power. Outputs "Load Rating" (0-100).
- **`ai/`**: The Gemma 3 Harness.
  - *Tech:* `candle-core` or `burn`.
  - *Function:* Loads quantized Gemma models (2B/8B) into memory for local inference.
- **`db/`**: The LanceDB Schema.
  - *Function:* Defines how vectors and journals are stored locally.

## 2. INTERFACE A: THE TOWER (Research Server)

**Metaphor:** Mission Control / The Federal Reserve
**User:** Researchers (VSEEL), Dept Heads, Admins.
**Tech Stack:** Axum (Backend), SQLx (Postgres), Antigravity (Google Cloud Sync).

### Content Organization

#### The "Big Data" Lake (Telemetry)

*Purpose:* Aggregates anonymized data to validate the "Coal Economy."
*Data Streams:*

- `Coal_Burn_Rate`: Time spent vs. Nodes completed.
- `Steam_Generation`: Mastery events and Quiz scores.
- `Sentiment_Vectors`: Anonymized emotional tone from reflection journals.

#### The Experiment Control Board

- **A/B Testing:** Configures which "Track" (Curriculum) is pushed to which Student Cohort.
- **Economy Governor:** Global variables (e.g., "Daily Coal Cap") that researchers can tweak to test scarcity.

#### The "NotebookLM" Pipeline

- **Function:** Automated exporter that formats telemetry into JSON/CSV specifically structure for Google NotebookLM.
- **Goal:** Generates the "Academic Impact Reports" automatically.

## 3. INTERFACE B: THE TRAIN YARD (Instructional Designer)

**Metaphor:** The Mechanic's Shop / Industrial Blueprint
**User:** Teachers, Instructional Designers.
**Tech Stack:** Bevy UI (Desktop Native - Windows/Mac/Linux).

### Content Organization

#### The Graph Editor (The Map)

- **Visuals:** Node-based editor (like Blender nodes).
- **Components:**
  - *Stations (Nodes):* The Learning Units.
  - *Tracks (Edges):* Logic gates (e.g., "IF score > 80 THEN branch left").
  - *Switches:* Adaptive routing logic.

#### The Weigh Station (The AI Mechanic)

- **Tech:** Runs Local Gemma 8B.
- **Features:**
  - *Real-Time Gauge:* As the teacher types, a needle moves from Green (Safe) to Red (Overload) based on vocabulary weight.
  - *The Inspector:* Sidebar chat where the AI suggests: "This narrative arc is broken," or "This word is Tier 3; suggest a scaffold."

#### The Parts Bin (Asset Library)

- **Vocabulary Bank:** Drag-and-drop words with pre-calculated cognitive weights.
- **Archetype Hooks:** Pre-written narrative beats for "Hero," "Sage," and "Creator" personas.

## 4. INTERFACE C: THE SCENARIO (Student View)

**Metaphor:** The Cockpit / First-Person RPG
**User:** The Learner (The Train).
**Tech Stack:** Bevy (WASM/Web), Oxide (Voice).

### Content Organization

#### The Narrative Engine (The View)

- **Dialogue System:** Chat-based interaction with NPCs (Local Gemma 2B Lite).
- **Voice Loop:** Voice-to-Text input for accessibility and immersion.

#### The Dashboard (The Gauges)

- **Coal Gauge:** Visualizes remaining mental energy. Stops "doomscrolling."
- **Steam Gauge:** Visualizes mastery currency.
- **The Backpack:** Inventory of "Vocabulary Tools" collected from the Train Yard.

#### The Reflection Cabin (The Privacy Moat)

- **Feature:** A local-only text editor.
- **The Soothe Protocol:** If `Keystroke_Jitter` is high (anxiety), the UI shifts colors and offers a "Parking Brake" (pause).

## 5. INTERFACE D: THE COMMUTER (Android Extension)

**Metaphor:** The Fitness Tracker / Geocaching Tool
**User:** The Student (On-the-go).
**Tech Stack:** PWA (Progressive Web App) + Web Geolocation API.

### Content Organization

#### The Physical AI Engine

- **GPS Logic:** "Unlock Node X by walking to Coordinates Y." Uses HTML5 Geolocation.
- **Interval Training:** Converts physical steps into "Coal" regeneration.

#### The Audio Review Loop

- **Feature:** Generates audio summaries of previous lessons for listening while walking.

## 6. DATA FLOW ARCHITECTURE (The Nervous System)

### The Hybrid Sovereign Model

We balance Privacy (Local) with Research (Cloud).

#### Local Sovereignty (The Device)

- **Store:** LanceDB (Embedded).
- **Data:** Raw Journal Entries, Private Reflections, Voice Logs.
- **Rule:** This data NEVER leaves the device. Gemma 3 analyzes it locally to provide feedback.

#### Enterprise Scale (The Cloud)

- **Store:** Google Antigravity (Vector Infrastructure).
- **Data:** "Steam" Transactions (Grades), "Coal" Burn Rates (Effort), Curriculum Maps.
- **Rule:** This is the public record of academic achievement.

#### The Sync Bridge

When a student completes a Node:

1. Local device generates a Zero-Knowledge Proof of completion.
2. Device sends the Proof + Steam Value to The Tower.
3. The Tower updates the global leaderboard/research dataset.

## 7. AI ORCHESTRATION (The Conductor)

| Task | AI Model | Location | Rationale |
| :--- | :--- | :--- | :--- |
| **NPC Dialogue** | Gemma 2B (Quantized) | Local (Student) | Low latency, works offline. |
| **Cognitive Weighing** | Gemma 8B | Local (Designer) | Needs nuance but must be free/unlimited. |
| **Curriculum Analysis** | Gemma 8B | Local (Designer) | Deep reading of lesson plans. |
| **"Stall" Rescue** | Gemini Pro | Cloud (API) | Only called when a student is stuck and needs "Oracle" level help. |
| **Data Synthesis** | NotebookLM | Cloud (Server) | Used by researchers to write reports. |
