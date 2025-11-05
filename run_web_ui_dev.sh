#!/bin/bash

# Run the Hybrid Workflow Engine Web UI in Development Mode
# This script provides hot reloading for both frontend and backend

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
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

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

echo "🚀 Hybrid Workflow Engine - Web UI (Development Mode)"
echo "======================================================"
echo ""

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    print_warning "Trunk is not installed. Installing..."
    cargo install trunk
    print_success "Trunk installed successfully!"
fi

# Check if cargo-watch is installed for backend hot reload
if ! command -v cargo-watch &> /dev/null; then
    print_warning "cargo-watch is not installed. Installing..."
    cargo install cargo-watch
    print_success "cargo-watch installed successfully!"
fi

# Check if wasm target is added
if ! rustup target list | grep "wasm32-unknown-unknown (installed)" &> /dev/null; then
    print_warning "WASM target not found. Adding..."
    rustup target add wasm32-unknown-unknown
    print_success "WASM target added successfully!"
fi

echo ""
print_msg "Starting development servers with hot reloading..."
print_msg "Frontend: http://localhost:8080 (Trunk dev server)"
print_msg "Backend API: http://localhost:3000"
echo ""

# Sync CSS file to web-ui directory
print_msg "Syncing CSS file..."
cp assets/workflow-web.css web-ui/style.css
print_success "CSS file synced!"

# Check if port 3000 is already in use and kill it
if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null 2>&1; then
    print_warning "Port 3000 is already in use. Stopping existing process..."
    lsof -ti:3000 | xargs kill -9 2>/dev/null || true
    sleep 1
    print_success "Stopped existing server on port 3000"
fi

# Check if port 8080 is already in use and kill it
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1; then
    print_warning "Port 8080 is already in use. Stopping existing process..."
    lsof -ti:8080 | xargs kill -9 2>/dev/null || true
    sleep 1
    print_success "Stopped existing server on port 8080"
fi

print_warning "Note: Use the Trunk dev server at http://localhost:8080 for hot reloading"
print_warning "Press Ctrl+C to stop both servers"
echo ""

# Function to cleanup background processes on exit
cleanup() {
    print_msg "Stopping servers..."
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    kill $CSS_WATCHER_PID 2>/dev/null || true
    exit 0
}

trap cleanup INT TERM

# Start CSS file watcher in background
./watch_css.sh &
CSS_WATCHER_PID=$!

# Start the backend server with hot reload in background
print_msg "Starting backend server with hot reload..."
cargo watch -x 'run --bin workflow-web-server --features web-server' &
BACKEND_PID=$!

# Give backend a moment to start
sleep 2

# Start Trunk in watch mode (with hot reload)
print_msg "Starting Trunk dev server with hot reload..."
cd web-ui
trunk serve --port 8080 &
FRONTEND_PID=$!
cd ..

echo ""
print_success "Development servers started!"
echo ""
print_msg "🎨 Frontend (with hot reload): http://localhost:8080"
print_msg "🔧 Backend API: http://localhost:3000"
print_msg ""
print_msg "💡 Edit files and watch them auto-reload:"
print_msg "   - CSS: Edit assets/workflow-web.css (will auto-sync)"
print_msg "   - Frontend: web-ui/src/**/*.rs"
print_msg "   - Backend: src/bin/workflow-web-server/**/*.rs"
echo ""
print_warning "Press Ctrl+C to stop both servers"
echo ""

# Wait for either process to exit
wait
