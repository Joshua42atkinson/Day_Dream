# The Daydream Initiative

Welcome to the Daydream Initiative, a "Creator's Sandbox" for Instructional Designers to build narrative-driven, AI-powered learning experiences. This project is a full-stack web application built entirely in Rust, using Leptos for the frontend and Axum for the backend.

##  Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust & Cargo:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **Docker & Docker Compose:** [https://docs.docker.com/get-docker/](https://docs.docker.com/get-docker/)
- **`cargo-leptos`:** The CLI tool for building and serving Leptos applications.
  ```sh
  cargo install cargo-leptos
  ```
  > **Note:** The installation of `cargo-leptos` can be slow and has been observed to time out in some environments. If you encounter issues, please try the command again.

- **System Dependencies (for Bevy on Linux):**
  ```sh
  sudo apt-get update
  sudo apt-get install -y libasound2-dev libudev-dev
  ```

## Project Setup

1.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2.  **Set up environment variables:**
    - Navigate to the `backend` directory.
    - Copy the example `.env` file:
      ```sh
      cp .env.example .env
      ```
    - The default `DATABASE_URL` in the `.env` file is configured to work with the included Docker Compose setup. No changes are needed if you are using Docker for the database.

3.  **Start the database:**
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
