# 🌐 Hybrid Workflow Engine - Web UI

Interactive web interface for running and monitoring hybrid workflows built with Leptos and Rust.

## ✨ Features

- 📋 **Workflow List View** - Browse all available workflows in your `workflows/` directory
- ▶️ **One-Click Execution** - Run workflows with a single button click
- 📊 **Step-by-Step Results** - View detailed execution results for each workflow step
- 🎨 **Beautiful UI** - Modern, responsive design with real-time updates
- ⚡ **Fast Performance** - WebAssembly-powered frontend for blazing-fast interactions

## 🚀 Quick Start

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Trunk (for building the WebAssembly frontend)

```bash
# Install the wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Running the Web UI

1. **Start the backend server:**

```bash
cargo run --bin workflow-web-server --features web-server
```

The server will start at `http://localhost:3000`

2. **Build and serve the frontend (in a separate terminal):**

```bash
cd web-ui
trunk serve
```

Or use the provided script:

```bash
./run_web_ui.sh
```

3. **Open your browser:**

Navigate to `http://localhost:3000`

## 📖 Usage

### Workflow List Page

- View all available workflows from the `workflows/` directory
- Each workflow card shows:
  - Workflow name and description
  - File path
  - Language badges
  - "Run Workflow" button

### Workflow Runner Page

- Click "Run Workflow" to execute the selected workflow
- View real-time execution status
- Expand/collapse individual step results
- See step-by-step output with:
  - Step number and name
  - Programming language used
  - Execution duration
  - Console output
  - Success/failure status

### Step Details

Click on any step card to expand and view:
- Full console output
- Error messages (if any)
- Execution timing
- Data passed between steps

## 🏗️ Architecture

```
┌─────────────────┐
│   Leptos App    │  (WebAssembly Frontend)
│   (Browser)     │
└────────┬────────┘
         │ REST API
         ▼
┌─────────────────┐
│   Axum Server   │  (Rust Backend)
│   Port 3000     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Workflow Engine │  (Core Rust Logic)
│ Multi-Language  │
│   Execution     │
└─────────────────┘
```

## 🎨 UI Components

### WorkflowList Component
- Fetches and displays all available workflows
- Responsive grid layout
- Loading states and error handling

### WorkflowRunner Component
- Executes selected workflow
- Real-time status updates
- Expandable step details
- Error display with stack traces

## 🔧 Development

### Project Structure

```
src/web/
├── mod.rs              # Module exports
├── app.rs              # Main Leptos app
├── server.rs           # Axum backend server
├── components/         # UI components
│   ├── mod.rs
│   ├── workflow_list.rs
│   └── workflow_runner.rs
└── api/                # API types
    └── mod.rs

assets/
└── workflow-web.css    # Styles

web-ui/
├── index.html          # Trunk entry point
└── Trunk.toml          # Trunk configuration
```

### API Endpoints

- `GET /api/workflows` - List all workflows
- `POST /api/workflows/:name/run` - Execute a workflow

### Building for Production

```bash
# Build the WASM frontend
trunk build --release

# Build the server
cargo build --release --bin workflow-web-server --features web-server

# Run the production server
./target/release/workflow-web-server
```

## 🎯 Roadmap

- [ ] Real-time streaming of step execution (WebSockets)
- [ ] Workflow editing in the browser
- [ ] Execution history and logs
- [ ] Workflow scheduling interface
- [ ] Performance metrics and charts
- [ ] Multi-user support with authentication
- [ ] Dark mode theme
- [ ] Export results as JSON/CSV

## 🐛 Troubleshooting

### Port Already in Use

If port 3000 is already in use, modify the server.rs file to use a different port.

### WASM Build Errors

Make sure you have the wasm target installed:
```bash
rustup target add wasm32-unknown-unknown
```

### CSS Not Loading

Ensure the `assets/` directory is in the project root and the server is configured to serve static files.

## 📝 License

Same as the main project - Apache License 2.0
