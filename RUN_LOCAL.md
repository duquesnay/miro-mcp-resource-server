# Testing Miro MCP Server with Claude Desktop (Local)

## Quick Start

### 1. Start the ADR-002 Server

```bash
# Terminal 1: Start the server
PORT=3010 RUST_LOG=info cargo run --bin http-server-adr002
```

The server will start on `http://localhost:3010` with:
- OAuth metadata: `http://localhost:3010/.well-known/oauth-protected-resource`
- Health check: `http://localhost:3010/health`
- Protected endpoints: `/mcp/list_boards`, `/mcp/get_board/:board_id`

### 2. Configure Claude Desktop

Add this to your Claude Desktop MCP configuration (`~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "miro-local": {
      "url": "http://localhost:3010/mcp",
      "transport": {
        "type": "sse"
      }
    }
  }
}
```

**Note**: The actual configuration format for remote MCP in Claude Desktop may differ. Check Claude Desktop documentation for exact syntax.

### 3. Restart Claude Desktop

Quit and relaunch Claude Desktop to load the new MCP server configuration.

### 4. Test the OAuth Flow

In Claude Desktop, ask:

```
"Can you list my Miro boards?"
```

Claude should:
1. Discover OAuth metadata from `/.well-known/oauth-protected-resource`
2. Redirect you to Miro OAuth login
3. Get access token from Miro
4. Call your local server with `Authorization: Bearer <token>`
5. Your server validates token with Miro API (with LRU caching)
6. Returns board list

## Architecture (ADR-002 Resource Server)

```
┌─────────────┐          ┌──────────┐          ┌──────────────┐
│   Claude    │          │   Miro   │          │ Local Server │
│  Desktop    │          │  OAuth   │          │  (ADR-002)   │
└─────────────┘          └──────────┘          └──────────────┘
       │                       │                        │
       │ 1. Discover metadata  │                        │
       ├──────────────────────────────────────────────>│
       │   GET /.well-known/oauth-protected-resource   │
       │<──────────────────────────────────────────────┤
       │   {"authorization_servers": ["miro.com"]}     │
       │                       │                        │
       │ 2. Initiate OAuth     │                        │
       ├──────────────────────>│                        │
       │   Redirect to Miro    │                        │
       │                       │                        │
       │ 3. User approves      │                        │
       │<──────────────────────┤                        │
       │   Access token        │                        │
       │                       │                        │
       │ 4. Call MCP with Bearer token                 │
       ├──────────────────────────────────────────────>│
       │   Authorization: Bearer <token>               │
       │                       │                        │
       │                       │ 5. Validate token      │
       │                       │<───────────────────────┤
       │                       │   GET /v1/oauth-token  │
       │                       ├───────────────────────>│
       │                       │   {user_id, scopes}    │
       │                       │                        │
       │ 6. Return boards      │                        │
       │<──────────────────────────────────────────────┤
       │   {boards: [...]}     │                        │
```

## Troubleshooting

### Server not starting?
```bash
# Check if port 3010 is already in use
lsof -i :3010

# Try a different port
PORT=3020 cargo run --bin http-server-adr002
```

### Claude Desktop not discovering OAuth?
Check that the metadata endpoint returns correct JSON:
```bash
curl http://localhost:3010/.well-known/oauth-protected-resource
# Should return: {"protected_resources":[{"resource":"https://api.miro.com","authorization_servers":["https://miro.com/oauth"]}]}
```

### Token validation failing?
Check server logs for validation errors:
```bash
RUST_LOG=debug cargo run --bin http-server-adr002
```

## Production Deployment

Once local testing succeeds, deploy to Scaleway Containers:
- Update OAuth redirect URI in Miro Developer Portal
- Configure secrets in Scaleway Secret Manager
- Deploy with persistent container for LRU cache

See [planning/adr-002-oauth-resource-server-architecture.md](planning/adr-002-oauth-resource-server-architecture.md) for architecture details.
