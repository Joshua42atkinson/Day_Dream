# Ask Pete - Technical Manual & Developer Guide

## 1. Introduction

**Ask Pete** (formerly Daydream) is a sovereign, "Headless" Instructional Engine designed to bridge the gap between pedagogical rigor and narrative immersion. It is a "Gamutainment" system that transforms instructional prompts into immersive 3D/Audio-Visual worlds.

This manual serves as the definitive guide for the development team, outlining the project's philosophy, architecture, and development workflows.

---

## 2. Philosophy

### 2.1. The "Boilermaker Industrial" Aesthetic

The user interface follows the **Boilermaker Industrial** design language, inspired by Purdue University's engineering heritage.

- **Colors:** Boilermaker Black (`#121212`), Old Gold (`#CEB888`), Steam White (`#F0F0F0`).
- **Visual Metaphors:** Chamfered edges, mechanical switches, pressure gauges, steam pipes.
- **Typography:** Monospace fonts (JetBrains Mono) for "engineering" elements, Sans-serif (Inter) for UI labels.
- **Goal:** To make the user feel like they are operating a powerful, precision-engineered machine.

### 2.2. "AI as a Mirror" (Socratic Method)

The core pedagogical innovation is the **AI as a Mirror** concept.

- **Role:** The AI (Pete) acts not as an oracle, but as a Socratic tutor.

The system is divided into a "Brain" (Backend) and a "Body" (Frontend).

#### The Brain (Backend)

- **Technology:** Rust (Axum) + Bevy ECS.
- **Responsibility:** Authoritative state management, AI orchestration, Database persistence.
- **Components:**
  - **Axum:** Web server handling API requests.
  - **Bevy ECS:** Manages the narrative state and game logic.
  - **Gemma 3:** Local LLM inference (2B/270M) via Candle.
  - **Antigravity:** Enterprise cloud synchronization and vector storage.
  - **PostgreSQL:** Local persistent storage for user data.

#### The Body (Frontend)

- **Technology:** Rust (Leptos) compiled to WebAssembly (WASM).
- **Responsibility:** Presentation layer, UI rendering, user interaction.
- **Deployment:** Runs in the browser, requiring no installation for the user.

### 3.2. "Split-Brain" Architecture (Mobile)

To support mobile devices without compromising power:

- **Heavy Server:** The "Brain" runs on a powerful workstation (PC) handling AI and physics.
- **Light Client:** The Android app (built with Tauri v2) connects to the server via WebSocket/REST.
- **Communication:** The mobile client acts as a remote control, sending inputs and receiving state updates.

### 3.3. Key Patterns

- **Async/Sync Bridge:** Uses `bevy_defer` to safely communicate between the async web server (Axum) and the synchronous game engine (Bevy).
- **Blocking Compute Isolation:** Heavy tasks (AI inference) are wrapped in `tokio::task::spawn_blocking` to prevent freezing the web server.

---

## 4. Setup & Installation

### Prerequisites

- **Rust:** Stable toolchain (`rustup`).
- **WASM Target:** `rustup target add wasm32-unknown-unknown`.
- **Trunk:** `cargo install trunk`.
- **PostgreSQL:** Local instance or Docker container.
- **SQLx CLI:** `cargo install sqlx-cli`.

### Quick Start

1. **Database Setup:**

    ```bash
    docker-compose up -d db
    cd backend
    sqlx database create
    sqlx migrate run
    ```

2. **Run Backend:**

    ```bash
    cd backend
    cargo run
    ```

3. **Run Frontend:**

    ```bash
    cd frontend
    trunk serve
    ```

---

## 5. Contribution Guidelines

### Code Standards

- **Rust:** Follow the official Rust Style Guide. Use `cargo fmt` and `cargo clippy`.
- **Error Handling:** No `unwrap()` in production code. Use `Result` and `?`.
- **Async:** Use `tokio` idioms. Never block the async runtime.

### Workflow

1. **Fork & Clone.**
2. **Create Feature Branch.**
3. **Implement Changes.**
4. **Test:** `cargo test --workspace`.
5. **Submit PR:** Follow the Conventional Commits specification.

### Directory Structure

- `backend/`: Axum server and Bevy ECS logic.
- `frontend/`: Leptos WASM application.
- `common/`: Shared types and utilities.
- `src-tauri/`: Tauri configuration for mobile builds.
