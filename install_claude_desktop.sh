#!/usr/bin/env bash
#
# Install Remote MCP Configuration for Claude Desktop
#

set -e

CONFIG_DIR="$HOME/Library/Application Support/Claude"
CONFIG_FILE="$CONFIG_DIR/claude_desktop_config.json"
BACKUP_FILE="$CONFIG_DIR/claude_desktop_config.backup.$(date +%Y%m%d_%H%M%S).json"

echo "🔧 Installing Miro MCP Remote Server Configuration"
echo ""

# Create config directory if it doesn't exist
mkdir -p "$CONFIG_DIR"

# Backup existing config if it exists
if [ -f "$CONFIG_FILE" ]; then
    echo "📦 Backing up existing config to: $BACKUP_FILE"
    cp "$CONFIG_FILE" "$BACKUP_FILE"
fi

# Read existing config or create empty object
if [ -f "$CONFIG_FILE" ]; then
    EXISTING_CONFIG=$(cat "$CONFIG_FILE")
else
    EXISTING_CONFIG='{}'
fi

# Add miro-local to mcpServers
NEW_CONFIG=$(echo "$EXISTING_CONFIG" | jq '.mcpServers["miro-local"] = {
  "url": "http://localhost:3010",
  "transport": "sse"
}')

# Write new config
echo "$NEW_CONFIG" > "$CONFIG_FILE"

echo "✅ Configuration installed!"
echo ""
echo "📝 Config file: $CONFIG_FILE"
echo ""
echo "Next steps:"
echo "  1. Restart Claude Desktop (Cmd+Q and reopen)"
echo "  2. Ask Claude: 'List my Miro boards'"
echo "  3. Monitor server logs for authentication requests"
echo ""
echo "🔍 OAuth metadata: http://localhost:3010/.well-known/oauth-protected-resource"
echo "🏥 Health check: http://localhost:3010/health"
