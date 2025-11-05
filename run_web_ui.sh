#!/bin/bash

# Run the Hybrid Workflow Engine Web UI
# This script starts both the backend server and frontend dev server

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_msg() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo "🚀 Hybrid Workflow Engine - Web UI"
echo "===================================="
echo ""

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    print_warning "Trunk is not installed. Installing..."
    cargo install trunk
    print_success "Trunk installed successfully!"
fi

# Check if wasm target is added
if ! rustup target list | grep "wasm32-unknown-unknown (installed)" &> /dev/null; then
    print_warning "WASM target not found. Adding..."
    rustup target add wasm32-unknown-unknown
    print_success "WASM target added successfully!"
fi

echo ""
print_msg "Building the web UI..."
echo ""

# Build the frontend from web-ui directory
cd web-ui
trunk build
cd ..

print_success "Frontend built successfully!"
echo ""
print_msg "Starting the backend server..."
echo ""

# Start the backend server
cargo run --bin workflow-web-server --features web-server

