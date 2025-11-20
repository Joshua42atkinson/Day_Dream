#!/bin/bash
# This script automates the setup of the development environment for the Daydream project.

# Exit immediately if a command exits with a non-zero status.
set -e

# Function to print messages
print_message() {
  echo "===================================================================================================="
  echo "$1"
  echo "===================================================================================================="
}

# 1. Install system dependencies
print_message "Installing system dependencies..."
case "$(uname -s)" in
  Linux)
    if [ -x "$(command -v apt-get)" ]; then
      echo "Debian-based system detected. Installing dependencies..."
      sudo apt-get update && sudo apt-get install -y libasound2-dev libudev-dev
    else
      echo "Non-Debian Linux system detected."
      echo "Please install the following dependencies manually:"
      echo " - alsa-lib-devel (or equivalent for libasound2)"
      echo " - libudev-devel (or equivalent)"
    fi
    ;;
  Darwin)
    echo "macOS detected."
    echo "Please ensure you have the command line tools installed ('xcode-select --install')."
    echo "The necessary libraries are typically included with the base system or XCode."
    ;;
  *)
    echo "Unsupported operating system: $(uname -s)"
    echo "Please install the equivalent of 'libasound2' and 'libudev' for your system."
    ;;
esac

# 2. Add wasm32-unknown-unknown target
print_message "Adding wasm32-unknown-unknown Rust target..."
rustup target add wasm32-unknown-unknown

# 3. Install required cargo tools
print_message "Installing required cargo tools (cargo-binstall, sqlx-cli, cargo-leptos)..."
cargo install cargo-binstall
cargo binstall sqlx-cli -y
cargo binstall cargo-leptos -y

# 4. Check for Docker
print_message "Checking for Docker installation..."
if ! [ -x "$(command -v docker)" ]; then
  echo "Docker is not installed. Please install Docker to run the database."
  echo "See https://docs.docker.com/engine/install/ for installation instructions."
  exit 1
fi
echo "Docker is installed."

# 5. Create .env file
print_message "Setting up the .env file..."
if [ -f "backend/.env.example" ]; then
  if [ ! -f "backend/.env" ]; then
    cp backend/.env.example backend/.env
    echo "Created backend/.env from backend/.env.example"
  else
    echo "backend/.env already exists. Skipping creation."
  fi
else
    echo "backend/.env.example not found. Please create a '.env' file in the 'backend' directory with the following content:"
    echo ""
    echo "DATABASE_URL=\"postgres://user:password@localhost:5432/daydream_db\""
    echo ""
fi

print_message "Development environment setup is complete!"
