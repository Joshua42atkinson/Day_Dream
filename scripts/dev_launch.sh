#!/bin/bash
# Exit immediately if a command exits with a non-zero status.
set -e

# --- 1. CONFIGURATION ---
LEPTOS_WATCH_PORT="3000"
RELOAD_PORT="3001"

print_message() {
  echo "===================================================================================================="
  echo "$1"
  echo "===================================================================================================="
}

# --- 2. EXECUTE ATOMIC ENVIRONMENT SETUP ---
print_message "Executing idempotent environment setup..."
# Run the existing setup script to ensure all dependencies and folders are present.
# This re-adds WASM targets, reinstalls cargo-leptos, and creates the public/ directory.
./scripts/dev_setup.sh

# --- 3. PORT CONFLICT RESOLUTION ---
# Stop any processes blocking the default development and reload ports (3000 and 3001).
# This is critical to bypass persistent server-start failures.
print_message "Checking for and terminating processes on ports ${LEPTOS_WATCH_PORT} and ${RELOAD_PORT}..."

# Find and kill processes on port 3000
# Note: 'lsof' might not be available in all minimal environments; '|| true' ensures the script doesn't stop.
if command -v lsof &> /dev/null; then
    if lsof -i :${LEPTOS_WATCH_PORT} -t; then
        print_message "Port ${LEPTOS_WATCH_PORT} occupied. Killing process."
        lsof -i :${LEPTOS_WATCH_PORT} -t | xargs kill -9 || true
    fi

    # Find and kill processes on port 3001
    if lsof -i :${RELOAD_PORT} -t; then
        print_message "Port ${RELOAD_PORT} occupied. Killing process."
        lsof -i :${RELOAD_PORT} -t | xargs kill -9 || true
    fi
fi

print_message "Ports confirmed clear (or check skipped). Starting application watcher."

# --- 4. LAUNCH APPLICATION ---
# Move to the project root directory
cd "$(dirname "$0")/.."

print_message "Launching cargo leptos watch from root..."
# This command launches the server and client in watch mode, using the fresh setup.
cargo leptos watch