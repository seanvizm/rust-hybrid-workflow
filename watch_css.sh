#!/bin/bash

# Watch CSS file and auto-sync to web-ui directory
# This runs in the background during development

SOURCE_CSS="assets/workflow-web.css"
TARGET_CSS="web-ui/style.css"

echo "👀 Watching $SOURCE_CSS for changes..."

# Get initial checksum
LAST_CHECKSUM=$(shasum "$SOURCE_CSS" 2>/dev/null || echo "")

while true; do
    # Wait a bit before checking again
    sleep 1
    
    # Get current checksum
    CURRENT_CHECKSUM=$(shasum "$SOURCE_CSS" 2>/dev/null || echo "")
    
    # If file changed, sync it
    if [ "$CURRENT_CHECKSUM" != "$LAST_CHECKSUM" ]; then
        echo "🎨 CSS file changed, syncing to web-ui..."
        cp "$SOURCE_CSS" "$TARGET_CSS"
        echo "✅ CSS synced! Trunk will auto-reload."
        LAST_CHECKSUM="$CURRENT_CHECKSUM"
    fi
done
