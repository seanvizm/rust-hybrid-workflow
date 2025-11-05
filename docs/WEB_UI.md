# 🌐 Hybrid Workflow Engine - Web UI

Interactive web interface for running and monitoring hybrid workflows built with Leptos and Rust.

## ✨ Features

- 📋 **Workflow List View** - Browse all available workflows in your `workflows/` directory
- ▶️ **One-Click Execution** - Run workflows with a single button click
- 📊 **Step-by-Step Results** - View detailed execution results for each workflow step
- 🎨 **Beautiful UI** - Modern, responsive design with mystical blue theme
- ⚡ **Fast Performance** - WebAssembly-powered frontend for blazing-fast interactions
- 🔥 **Hot Reload** - Auto-refresh on code changes during development

---

## 🚀 Quick Start (3 Steps)

### 1. Install Dependencies

```bash
# Install Trunk (WebAssembly build tool)
cargo install trunk

# Add WASM target to Rust
rustup target add wasm32-unknown-unknown
```

### 2. Run the Web UI

**Production Mode:**
```bash
./run_web_ui.sh
```

**Development Mode (with hot reload):**
```bash
./run_web_ui_dev.sh
```

Or manually:
```bash
# Terminal 1: Build frontend
cd web-ui
trunk build

# Terminal 2: Start backend
cargo run --bin workflow-web-server --features web-server
```

### 3. Open Your Browser

- **Production**: http://localhost:3000
- **Development**: http://localhost:8080 (with hot reload)

---

## 📸 What You'll See

### Workflow List Page
- All workflows from `workflows/` directory displayed as cards
- Each card shows:
  - Workflow name and description
  - File path
  - Language badges (Lua, Python, Shell)
  - "Run Workflow" button

### Workflow Runner Page  
- Real-time execution status
- Expandable step-by-step results
- Each step shows:
  - ✅ Success/❌ Failure status
  - ⏱️ Execution time
  - 📄 Console output
  - 🏷️ Programming language used

### Step Details

Click on any step card to expand and view:
- Full console output
- Error messages (if any)
- Execution timing
- Data passed between steps

---

## 🛠️ Development Guide

### Running in Development Mode

For the best development experience with hot reloading:

```bash
./run_web_ui_dev.sh
```

Then open **http://localhost:8080** (not 3000) to get:
- ✅ Automatic browser refresh on file changes
- ✅ Fast incremental rebuilds (~2-5s)
- ✅ CSS changes applied instantly
- ✅ Backend auto-restart

See [WEB_UI_DEV.md](WEB_UI_DEV.md) for complete hot reload documentation.

### What Gets Hot Reloaded?

**Frontend (auto-refresh in browser):**
- ✅ Rust code in `web-ui/src/`
- ✅ CSS in `assets/workflow-web.css`
- ✅ HTML in `web-ui/index.html`
- ✅ Components in `web-ui/src/components/`

**Backend (auto-restart):**
- ✅ Server code in `src/bin/workflow-web-server/`
- ✅ API routes and handlers
- ✅ Core workflow engine code

### Project Structure

```
web-ui/
├── Cargo.toml          # Standalone package (no heavy dependencies)
├── index.html          # HTML template
├── Trunk.toml          # Trunk dev server config
├── style.css           # CSS (synced from assets/)
└── src/
    ├── lib.rs          # WASM entry point
    ├── app.rs          # Main Leptos app component
    └── components/
        ├── mod.rs
        ├── workflow_list.rs    # Workflow grid display
        └── workflow_runner.rs  # Execution & results UI

src/bin/workflow-web-server/
├── main.rs             # Axum server
└── api.rs              # API types & serialization

assets/
└── workflow-web.css    # Mystical blue theme styling
```

### Architecture

```
┌─────────────────────┐
│   Leptos App        │  WebAssembly Frontend
│   (Browser)         │  Port 8080 (dev) / 3000 (prod)
└──────────┬──────────┘
           │ REST API (/api/*)
           │
           ▼
┌─────────────────────┐
│   Axum Server       │  Rust Backend
│   Port 3000         │  Serves API + static files
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Workflow Engine     │  Core Rust Logic
│ mlua + PyO3         │  Multi-language execution
│ + wasmtime          │  (Lua, Python, Shell)
└─────────────────────┘
```

**Development Architecture:**
- Trunk dev server (8080) proxies `/api/*` to backend (3000)
- WebSocket connection for hot reload
- CSS watcher auto-syncs `assets/workflow-web.css` → `web-ui/style.css`

---

## 🔌 API Endpoints

### GET /api/workflows

List all available workflows.

**Response:**
```json
[
  {
    "name": "comprehensive_workflow",
    "path": "workflows/comprehensive_workflow.lua",
    "description": "Multi-language pipeline demo"
  }
]
```

### POST /api/workflows/:name/run

Execute a workflow by name.

**Request:** None (workflow name in URL)

**Response:**
```json
{
  "workflow_name": "comprehensive_workflow",
  "status": "Success",
  "steps": [
    {
      "step_number": 1,
      "step_name": "lua_config",
      "language": "lua",
      "output": "{\"config\": {...}}",
      "duration_ms": 45,
      "status": "Success"
    }
  ],
  "total_duration_ms": 1250
}
```

---

## 🎨 UI Components

### WorkflowList Component (`workflow_list.rs`)
- Fetches workflows from `/api/workflows`
- Displays in responsive grid layout
- Loading states and error handling
- Navigation to runner on button click

### WorkflowRunner Component (`workflow_runner.rs`)
- Accepts workflow name from URL params
- Posts to `/api/workflows/{name}/run`
- Real-time status updates
- Expandable step cards with color-coded status
- Error display with full stack traces

### Styling (`workflow-web.css`)
- Mystical blue theme (`#1e40af`)
- CSS custom properties for easy theming
- Responsive design (mobile-friendly)
- Smooth animations and transitions

---

## 🏗️ Building for Production

```bash
# Build the WASM frontend
cd web-ui
trunk build --release

# Build the server
cargo build --release --bin workflow-web-server --features web-server

# Run the production server
./target/release/workflow-web-server
```

The production server serves the built frontend from the `pkg/` directory.

---

## 🔧 Troubleshooting

### Port Already in Use

**Problem:** Port 3000 or 8080 is occupied

**Solution:**
```bash
./stop_servers.sh
./run_web_ui_dev.sh
```

### WASM Build Errors

**Problem:** `wasm32-unknown-unknown` target not found

**Solution:**
```bash
rustup target add wasm32-unknown-unknown
```

### CSS Not Loading (MIME Type Error)

**Problem:** Browser shows "non CSS MIME types not allowed"

**Solution:** This is already fixed! CSS is copied to `web-ui/style.css` and synced automatically during development.

### Frontend Not Updating

**Problem:** Code changes don't appear in browser

**Solution:**
- ✅ Make sure you're on http://localhost:8080 (not 3000) in dev mode
- ✅ Check terminal for build errors
- ✅ For CSS: Edit `assets/workflow-web.css` (not `web-ui/style.css`)
- ✅ Try hard refresh: Cmd+Shift+R or Ctrl+Shift+R

### Backend Not Restarting

**Problem:** Server changes don't take effect

**Solution:**
- ✅ Check terminal for compile errors
- ✅ Ensure changes are saved
- ✅ Verify `cargo-watch` is running (check terminal output)

### "cargo-watch not found"

**Problem:** cargo-watch is not installed

**Solution:**
```bash
cargo install cargo-watch
```

The dev script will auto-install it, but you can install manually if needed.

---

## 🎯 Roadmap

- [x] Workflow list view
- [x] One-click execution
- [x] Step-by-step results
- [x] Hot reload in development
- [x] Responsive design
- [x] Error handling and display
- [ ] Real-time streaming of step execution (WebSockets)
- [ ] Workflow editing in the browser
- [ ] Execution history and logs
- [ ] Workflow scheduling interface
- [ ] Performance metrics and charts
- [ ] Multi-user support with authentication
- [ ] Dark mode theme toggle
- [ ] Export results as JSON/CSV
- [ ] Workflow templates library

---

## 📚 Additional Documentation

- [WEB_UI_DEV.md](WEB_UI_DEV.md) - Complete development guide with hot reload details
- [TESTING.md](../TESTING.md) - Testing the workflow engine
- [RUST_TESTS.md](../RUST_TESTS.md) - Rust testing documentation

---

## 📝 License

Same as the main project - Apache License 2.0
