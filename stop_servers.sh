#!/bin/bash

# Stop all web UI servers

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_msg() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

echo "🛑 Stopping Hybrid Workflow Engine Servers"
echo "==========================================="
echo ""

# Stop processes on port 3000 (backend)
if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null 2>&1; then
    print_msg "Stopping backend server on port 3000..."
    lsof -ti:3000 | xargs kill -9 2>/dev/null
    print_success "Backend server stopped"
else
    print_msg "No server running on port 3000"
fi

# Stop processes on port 8080 (frontend dev server)
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1; then
    print_msg "Stopping frontend dev server on port 8080..."
    lsof -ti:8080 | xargs kill -9 2>/dev/null
    print_success "Frontend dev server stopped"
else
    print_msg "No server running on port 8080"
fi

# Stop any cargo-watch processes
if pgrep -f "cargo-watch" >/dev/null 2>&1; then
    print_msg "Stopping cargo-watch processes..."
    pkill -f "cargo-watch" 2>/dev/null
    print_success "cargo-watch stopped"
fi

# Stop any trunk processes
if pgrep -f "trunk serve" >/dev/null 2>&1; then
    print_msg "Stopping trunk serve processes..."
    pkill -f "trunk serve" 2>/dev/null
    print_success "trunk serve stopped"
fi

echo ""
print_success "All servers stopped!"
