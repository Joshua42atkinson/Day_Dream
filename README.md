# The Daydream Initiative

Welcome to the Daydream Initiative, a "Creator's Sandbox" for Instructional Designers to build narrative-driven, AI-powered learning experiences. This project is a full-stack web application built entirely in Rust, using Leptos for the frontend and Axum for the backend.

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
