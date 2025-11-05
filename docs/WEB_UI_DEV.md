# Web UI Development Guide

## Quick Start

### 1. Production Mode (No Hot Reload)
```bash
./run_web_ui.sh
```
- Builds the frontend once
- Starts the backend server at http://localhost:3000
- Use this for production-like testing

### 2. Development Mode (With Hot Reload) ⚡
```bash
./run_web_ui_dev.sh
```
- **Frontend hot reload**: Automatically rebuilds and reloads when you change Rust/CSS/HTML files
- **Backend hot reload**: Automatically restarts server when you change backend code
- Frontend dev server: http://localhost:8080
- Backend API server: http://localhost:3000

**⚠️ IMPORTANT: Use http://localhost:8080 for development - it has hot reload!**

## First Time Setup

The dev script will automatically install required tools:
- `trunk` - Frontend dev server and WASM builder
- `cargo-watch` - Backend hot reload tool
- `wasm32-unknown-unknown` target

## Getting Started with Hot Reload

### Step 1: Start the Development Server

```bash
./run_web_ui_dev.sh
```

### Step 2: Open the CORRECT URL

- ✅ **USE THIS:** http://localhost:8080 (Trunk dev server with hot reload)
- ❌ **NOT THIS:** http://localhost:3000 (backend only, no hot reload)

### Step 3: Verify Hot Reload is Working

1. Open browser DevTools (F12)
2. Go to the Console tab
3. You should see: `[TRUNK] WebSocket connection established`

If you see this message, hot reload is working! 🎉

## What Gets Hot Reloaded?

### Frontend (Automatic reload on save):
- ✅ Rust code in `web-ui/src/`
- ✅ CSS in `assets/workflow-web.css` (auto-synced via watch script)
- ✅ HTML in `web-ui/index.html`
- ✅ Components in `web-ui/src/components/`

### Backend (Automatic restart on save):
- ✅ Server code in `src/bin/workflow-web-server/`
- ✅ API routes and handlers
- ✅ Core workflow engine code

## Testing Hot Reload

### Test 1: CSS Changes (Instant)

1. Open `assets/workflow-web.css`
2. Change a color, e.g., `--primary-color: #1e40af;` to `--primary-color: #ff0000;`
3. Save the file
4. Watch the terminal - you should see:
   ```
   🎨 CSS file changed, syncing to web-ui...
   ✅ CSS synced! Trunk will auto-reload.
   INFO 📦 starting build
   INFO ✅ success
   ```
5. The browser automatically refreshes with the new color!

### Test 2: Rust Frontend Changes (~2-5s)

1. Open `web-ui/src/app.rs`
2. Change the title or any text
3. Save the file
4. Watch the terminal compile and rebuild
5. Browser refreshes automatically!

### Test 3: Backend Changes (~2-5s)

1. Open `src/bin/workflow-web-server/main.rs`
2. Make a small change (e.g., modify a log message)
3. Save the file
4. Backend automatically restarts
5. Refresh browser to see changes

## How It Works

### Behind the Scenes

- **Trunk (port 8080):** Watches files, rebuilds WASM, injects WebSocket for hot reload
- **Backend (port 3000):** Serves API endpoints, auto-restarts with cargo-watch
- **CSS Watcher:** Monitors `assets/workflow-web.css` and syncs to `web-ui/style.css`
- **Proxy:** Trunk proxies `/api/*` requests to port 3000

### Hot Reload Flow

When you edit a file:
1. File watcher detects the change
2. Trunk rebuilds the WASM module (for frontend changes)
3. Sends WebSocket message to browser
4. Browser automatically reloads

## Performance Tips

- **First build:** ~30s (compiling from scratch)
- **Incremental rebuilds:** ~2-5s (only changed files)
- **CSS changes:** Instant (no recompilation needed)
- **Browser DevTools:** Keep console open to see hot reload events

## Troubleshooting

### Browser Not Auto-Refreshing

**Problem:** Files change but browser doesn't reload

**Solutions:**
1. ✅ Make sure you're on http://localhost:8080 (not 3000!)
2. ✅ Check browser console for `[TRUNK] WebSocket connection established`
3. ✅ Try hard refresh: Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows/Linux)
4. ✅ Restart servers: `./stop_servers.sh` then `./run_web_ui_dev.sh`

### "Address already in use" Error

**Problem:** Ports 3000 or 8080 are occupied

**Solution:**
```bash
./stop_servers.sh
./run_web_ui_dev.sh
```

This will kill any processes on ports 3000 and 8080, then restart the servers.

### "cargo-watch not found"

**Problem:** cargo-watch is not installed

**Solution:**
The script will auto-install it. If it fails:
```bash
cargo install cargo-watch
```

### Frontend Not Updating

**Problem:** CSS or Rust changes don't appear

**Solutions:**
- ✅ Ensure you're on http://localhost:8080 (not 3000)
- ✅ Check terminal for build errors
- ✅ For CSS: Make sure you're editing `assets/workflow-web.css` (not `web-ui/style.css`)
- ✅ Try hard refresh: Cmd+Shift+R or Ctrl+Shift+R

### Backend Not Restarting

**Problem:** Server changes don't take effect

**Solutions:**
- ✅ Check terminal for compile errors
- ✅ Ensure changes are saved
- ✅ Verify `cargo-watch` is running (check terminal output)

### Slow Recompilation

**Problem:** Builds take too long

**Tips:**
- First compile is always slow (~30s)
- Incremental rebuilds should be fast (~2-5s)
- CSS changes are instant
- Consider using `cargo check` for syntax validation during development

## Stopping the Servers

Press **Ctrl+C** in the terminal running `./run_web_ui_dev.sh`

This will automatically stop:
- Frontend dev server (Trunk on port 8080)
- Backend server (on port 3000)
- CSS file watcher

Or use the dedicated script:
```bash
./stop_servers.sh
```
