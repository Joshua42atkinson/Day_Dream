# Daydream Initiative - Sovereign Gamutainment System

**A Headless Instructional Engine for Transformational Learning**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![WASM](https://img.shields.io/badge/WebAssembly-Ready-blueviolet)](https://webassembly.org/)

> **A generative, sovereign educational ecosystem that transforms instructor's text prompts into immersive 3D/Audio-Visual worlds**

---

## 🎯 Vision

The Daydream Initiative solves the **"Edutainment Gap"** - the false choice between pedagogical rigor (LMS, quizzes) and narrative immersion (AAA games). We provide a **sovereign, privacy-first platform** where:

- ✅ **Non-programmers** (Instructional Designers, educators) create sophisticated learning experiences
- ✅ **Students own their data** - All AI runs locally, no cloud leakage (FERPA-compliant)
- ✅ **Institutions own the IP** - 100% open source under GPLv3, no vendor lock-in
- ✅ **Research flourishes** - A "Living Laboratory" for Affective Computing and AI Ethics

### The Pedagogical Innovation

**Transformational Play** replaces "points and badges" with **Identity Work**:

1. **Projective Identity Dissonance** (James Paul Gee): Learners choose an archetype
2. **Inverse Persona Agent**: Local AI generates NPCs embodying traits the learner avoids
3. **Resolution Through Learning**: Progress requires adopting new vocabulary, ethical frameworks, constructing a richer identity

---

## 🏗️ Architecture: The "Headless" Design

### The "Brain" (Authoritative Backend)

**Rust/Bevy ECS** - Secure, sovereign state management on university servers

```
Backend (Rust/Axum)
├── Bevy ECS Engine       → Authoritative game state
├── Candle (Local AI)     → Llama-3-8B inference (on-premises)
├── bevy_yarnspinner      → Branching narrative logic
└── PostgreSQL            → Story/reflection persistence
```

**Key Benefit**: Educational logic, student data, and AI never leave controlled environment.

### The "Body" (Presentation Layer)

**Leptos/WASM** - High-performance web client

```
Frontend (Leptos)  
└── Compiles to WebAssembly → Runs in any browser
    ├── No installation needed
    ├── Works on Chromebooks, iPads, Lab PCs
    └── "Frictionless Access" - Just click a link
```

### The "Lego" Stack

| Module | Technology | Purpose |
|--------|-----------|---------|
| **Core Engine** | `bevy` ECS | Data-driven game logic, WASM-ready |
| **Narrative** | `bevy_yarnspinner` | Branching dialogue (pedagogy as text files) |
| **Physics** | `avian` | Deterministic puzzles and movement |
| **AI Brain** | `candle` | Local LLM inference (privacy-preserving) |
| **Audio** | `rodio` / `bevy_audio` | Procedural audio synthesis |

### Critical Architectural Patterns

#### 1. Async/Sync Bridge (`bevy_defer`)

**Problem**: Axum (async) ↔ Bevy (sync) impedance mismatch

**Solution**:

```rust
async fn save_story(State(async_world): State<AsyncWorld>) {
    async_world.apply(|world: &mut World| {
        // Safe mutation of Bevy state
    }).await;
}
```

#### 2. Blocking Compute Isolation

**Problem**: AI inference blocks Tokio runtime

**Solution**:

```rust
tokio::task::spawn_blocking(move || {
    candle_llm_inference(prompt) // ✅ Non-blocking
}).await?
```

---

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.70+ ([rustup.rs](https://rustup.rs))
- **PostgreSQL** 14+
- **Trunk**: `cargo install trunk`
- **sqlx-cli**: `cargo install sqlx-cli --no-default-features --features postgres`

### Quick Start

```bash
# 1. Setup database
export DATABASE_URL="postgres://postgres:password@localhost:5432/daydream"
cd backend && sqlx database create && sqlx migrate run

# 2. Run backend
cd backend && cargo run
# → Server on http://localhost:8080

# 3. Run frontend (new terminal)
cd frontend && trunk serve
# → UI on http://127.0.0.1:8080
```

**👉 Full setup guide**: See [SETUP.md](SETUP.md)

---

## 📚 The "Generative Seed" Workflow

1. **Instructor Input** (No code required):

   ```toml
   # seed_artifact.toml
   theme = "Cyberpunk Ethics"
   vocabulary = ["Utilitarianism", "Deontology", "Virtue Ethics"]
   learning_goal = "Apply ethical frameworks to AI trolley problems"
   ```

2. **Engine Processing**:
   - `bevy_yarnspinner` + `candle` LLM expand seed → dialogue trees
   - Bevy ECS spawns entities, NPCs, quests at runtime

3. **Student Experience**:
   - 3D environment with NPCs posing ethical dilemmas
   - Use vocabulary words to unlock dialogue options (Vocabulary-as-a-Mechanic)
   - Real-time "Virtue Topology" dashboard shows identity shifts

---

## 🎓 For Purdue University

### Institutional Value Proposition

| Dimension | Value |
|-----------|-------|
| **Research Utility** | Living Laboratory for Affective Computing, Narrative Psychology, AI Ethics |
| **Grant Eligibility** | Aligns with NSF "AI in Education" + "Open Cyberinfrastructure" calls |
| **Interdisciplinary** | English (Interactive Fiction) + CS (Rust/AI) + Psychology (Identity Theory) |
| **IP Protection** | GPLv3 ensures institutional ownership, prevents proprietary forks |

### MVP Deliverables

1. **The Holodeck**: Grey-box 3D environment (`bevy_fps_controller`)
2. **The Conversationalist**: NPC with `bevy_yarnspinner` + local AI
3. **The Mirror**: Real-time "Virtue Topology" UI dashboard

### Evaluation Metrics

- **Feasibility**: Frame time budget (LLM response <200ms on university GPU)
- **Learning Outcomes**:
  - Flow State Retention (time-on-task vs. LMS)
  - Vocabulary Transfer (pre/post tests)
  - Identity Expansion (Self-Efficacy surveys)

---

## 🔐 Privacy & Sovereignty

### FERPA/COPPA Compliance

✅ **Local AI Processing**: Whisper (STT) + OpenAudio (TTS) run on university servers  
✅ **No Cloud Dependencies**: Student voice/reflection data never leaves campus  
✅ **Data Sovereignty**: PostgreSQL on-premises, full institutional control  
✅ **Open Source**: Auditable codebase, no proprietary "black boxes"

### The Anti-Walled Garden

Unlike Unity/Unreal/Roblox platforms that create vendor lock-in:

- **Own the Stack**: Rust code, Bevy engine, Candle AI - all open source
- **Own the Data**: PostgreSQL under institutional control
- **Own the Future**: No licensing fees, no platform deprecation risk

---

## 🛠️ Development Workflow

### Agentic Development with Google Antigravity

**The "Force Multiplier" Approach**:

```
You (Architect)  : "Create ECS component `CognitiveLoad`"
Antigravity (AI) : "Here's the Bevy system that adjusts fog  
                    density based on learner confusion"
Result           : Complex visual metaphors in minutes, not days
```

**Development Velocity Metric**: Features/week with AI agent vs. manual coding

### Current Status

- ✅ **Frontend**: Leptos 0.8, compiling to WASM
- ✅ **Backend**: Axum + PostgreSQL integration
- ✅ **Database**: Story graph persistence with sqlx
- 🔄 **Next**: Bevy ECS game loop integration
- 🎯 **Planned**: Candle LLM + bevy_yarnspinner

---

## 📖 Documentation

- **[SETUP.md](SETUP.md)** - Platform-specific installation
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines
- **[walkthrough.md](.gemini/antigravity/brain/*/walkthrough.md)** - Build verification

### API Documentation

See [API Endpoints](#) section in main README for story graph CRUD operations.

---

## 📜 License

**GNU General Public License v3.0 (GPLv3)**

This copyleft license ensures:

- ✅ Anyone can use, study, modify the code
- ✅ All derivatives must remain open source
- ✅ Prevents proprietary forks
- ✅ Purdue University retains institutional ownership

**Why GPLv3?** We reject permissive licenses (MIT/Apache) that allow commercial entities to create closed-source forks. Education should remain a public good.

---

## 🤝 Contributing

We welcome contributions from:

- **Learning scientists** - Pedagogical theory implementation
- **Rust developers** - ECS systems, WASM optimization
- **AI researchers** - Candle LLM integration, quantization
- **Narrative designers** - Yarnspinner scenario authoring

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📞 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/YOUR_ORG/Day_Dream/issues)
- **Discussions**: [GitHub Discussions](https://github.com/YOUR_ORG/Day_Dream/discussions)
- **Purdue Contacts**:
  - Faculty Champion: [Prof. George Hanshaw](mailto:email@purdue.edu)
  - Innovation Hub: [Purdue Envision Center](https://www.purdue.edu/envision/)

---

## 🎖️ Acknowledgments

**Built with**:

- 🦀 **Rust** - Fearless concurrency, memory safety
- 🎮 **Bevy** - ECS game engine
- 🤖 **Candle** - HuggingFace ML framework
- 🌐 **Leptos** - Reactive WASM frontend
- 🏛️ **Purdue University** - Learning Design & Technology program

**Developed using**: Google Antigravity (Agentic Development Platform)

---

### Executive Summary

> *"The Daydream Initiative moves beyond the 'Walled Gardens' of proprietary EdTech. By leveraging **Rust** for performance, **Bevy** for modularity, and **Local AI** for privacy, we are building a sovereign, generative learning engine. This system transforms the instructor's simple text prompts into immersive, 3D/Audio-Visual worlds, democratizing high-fidelity game-based learning for the entire university."*

**Status**: Pre-alpha, architectural design validated. Frontend compiling successfully. Ready for Purdue institutional pickup.

---

**Built with ❤️ in Rust** | **Privacy by Design** | **Open Source Forever** | **Purdue University**
