# Ask Pete: The Instructional Design Sandbox

## A Purdue University Capstone Project

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![WASM](https://img.shields.io/badge/WebAssembly-Ready-blueviolet)](https://webassembly.org/)

> **"The Train Yard for the Mind"** — A sovereign, AI-assisted authoring tool for non-linear curriculum design.

---

## 🚂 Vision

**Ask Pete** bridges the gap between "Digital Native" software engineers and "Pedagogical Traditionalists." It transforms the abstract cognitive science of **Cognitive Load Theory** into a tangible, visual metaphor: **The Train Yard**.

- **The Train (Student)**: Has limited capacity (Cognitive Load).
- **The Cargo (Vocabulary)**: Knowledge that must be carried.
- **The Tracks (Curriculum)**: The non-linear path to mastery.
- **Pete (The Station Master)**: An AI persona that helps instructors build safe, efficient learning journeys.

### Key Features

- ✅ **Hybrid Sovereign AI**: **Gemma 3** (Local) for privacy + **Gemini 3 Ultra** (Cloud) for reasoning.
- ✅ **The Coal Economy**: "Compute Token Scarcity" teaches resource management.
- ✅ **The Weigh Station**: AI analyzes vocabulary words and assigns "Cognitive Weight" (1-100).
- ✅ **The Sandbox**: A GameLit RPG environment where students "burn Coal" to generate "Steam" (Mastery).
- ✅ **Open Source**: Apache 2.0 Licensed. Owned by the community, for the community.

---

## 📦 Installation

### Windows (Pre-built)

1. Download the latest `AskPete_Setup.exe` from the [Releases](https://github.com/Joshua42atkinson/Ask_Pete/releases) page.
2. Run the installer.
3. Launch "Ask Pete" from your desktop.

### Building from Source

**Prerequisites:**

- [Rust](https://rustup.rs/) (1.75+)
- [PostgreSQL](https://www.postgresql.org/) (14+)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [SQLx CLI](https://github.com/launchbadge/sqlx) (`cargo install sqlx-cli --no-default-features --features postgres`)

**Steps:**

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/Joshua42atkinson/Ask_Pete.git
    cd Ask_Pete
    ```

2. **Setup Database:**

    ```bash
    # Start Postgres (ensure user=postgres, password=password, db=daydream_db)
    # Or update backend/.env with your credentials
    cd backend
    sqlx database create
    sqlx migrate run
    ```

3. **Download AI Models:**
    - Download **Gemma 3** (2B or 270M) from HuggingFace.
    - Place it in `backend/models/`.

4. **Run the Backend:**

    ```bash
    cd backend
    cargo run
    # Server running at http://localhost:3000
    ```

5. **Run the Frontend:**

    ```bash
    # Open a new terminal
    cd frontend
    trunk serve
    # App running at http://localhost:8080
    ```

---

## 📚 Documentation

- **[Vision Document](docs/vision_document.md)**: The philosophical foundation.
- **[Research Agenda](docs/capstone_research/research_agenda.md)**: The theoretical framework.
- **[Cognitive Load Framework](docs/cognitive_load_framework.md)**: The physics of learning.
- **[Migration Protocol](docs/migration_protocol.md)**: From Daydream to Ask Pete.

---

## 🤝 Contributing

We welcome contributions from Instructional Designers, Rust Developers, and AI Researchers.
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ at Purdue University**
*Department of Learning Design & Technology*
