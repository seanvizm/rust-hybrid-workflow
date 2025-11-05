# 🎯 Getting Started with Web UI

## Quick Setup (3 Steps)

### 1. Install Dependencies

```bash
# Install Trunk (WebAssembly build tool)
cargo install trunk

# Add WASM target to Rust
rustup target add wasm32-unknown-unknown
```

### 2. Run the Web UI

```bash
./run_web_ui.sh
```

Or manually:

```bash
# Terminal 1: Build frontend
trunk build --features web-ui

# Terminal 2: Start backend
cargo run --bin workflow-web-server --features web-server
```

### 3. Open Your Browser

Navigate to: **http://localhost:3000**

## 📸 What You'll See

### Workflow List Page
- All workflows from `workflows/` directory
- Click any "Run Workflow" button to execute

### Workflow Runner Page  
- Real-time execution status
- Expandable step-by-step results
- Each step shows:
  - ✅ Success/❌ Failure status
  - ⏱️ Execution time
  - 📄 Console output
  - 🏷️ Programming language used

## 🎨 Features

- ✅ Browse all available workflows
- ✅ One-click workflow execution  
- ✅ Step-by-step result visualization
- ✅ Expandable output details
- ✅ Beautiful, modern UI
- ✅ Responsive design (works on mobile)

## 🔧 Troubleshooting

**Problem**: Port 3000 already in use  
**Solution**: Edit `src/web/server.rs` and change the port number

**Problem**: CSS not loading  
**Solution**: Make sure `assets/` directory exists in project root

**Problem**: WASM build fails  
**Solution**: Run `rustup target add wasm32-unknown-unknown`

## 📚 More Info

See [WEB_UI.md](WEB_UI.md) for detailed documentation.
