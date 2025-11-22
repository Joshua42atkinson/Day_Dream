Daydream: LDT Research Sandbox
A Purdue University Master's Capstone Project

Daydream is a next-generation instructional design platform and "creator's sandbox" designed to empower educators to build narrative-driven Intelligent Tutoring Systems (ITS) without coding expertise. Developed as a capstone artifact for the Purdue Learning Design and Technology (LDT) program, it utilizes a high-performance Rust/WASM architecture (Axum, Leptos, Bevy) and integrates "Privacy-by-Design" principles to ensure compliance with student data privacy standards.

## Quickstart

This project includes a setup script to automate the installation of dependencies.

1.  **Run the setup script:**
    ```sh
    chmod +x scripts/dev_setup.sh
    ./scripts/dev_setup.sh
    ```
    This script will:
    - Attempt to install required system dependencies for Debian-based systems and provide guidance for other platforms.
    - Add the `wasm32-unknown-unknown` Rust target.
    - Install necessary cargo tools (`cargo-binstall`, `sqlx-cli`, `cargo-leptos`).
    - Check for a Docker installation.
    - Create a `backend/.env` file from `backend/.env.example`.

2.  **Start the database:**
    - From the root of the project, run:
      ```sh
      sudo docker compose up -d db
      ```
    - This will start a PostgreSQL database container in the background.

## Running the Application

Once the setup is complete, you can run the entire application (both frontend and backend) with a single command from the project root:

```sh
cargo leptos watch
```

This will:
- Build the frontend and backend.
- Start the backend server.
- Start a file watcher that will automatically rebuild and reload the application when you make changes.

You can access the application at `http://127.0.0.1:3000`.
