# Local Testing Guide - Miro MCP Server

Complete guide for testing the OAuth2 flow and MCP server locally before deploying to Scaleway Functions.

## Prerequisites

- Rust toolchain (stable)
- Miro account with developer access
- OpenSSL (for generating encryption keys)
- Web browser with DevTools

## 1. Miro OAuth App Configuration

### Create or Update Miro OAuth App

1. Go to [Miro Developer Portal](https://miro.com/app/settings/user-profile/apps)
2. Select your existing Miro app (or create a new one)
3. Add redirect URI for local testing:
   ```
   http://localhost:3000/oauth/callback
   ```
4. Note your credentials:
   - **Client ID**: Found on app settings page
   - **Client Secret**: Found on app settings page (keep this secret!)
5. Ensure the following scopes are enabled:
   - `boards:read` - Read board data
   - `boards:write` - Create and modify boards

**Important**: Keep both local (`http://localhost:3000/oauth/callback`) and production redirect URIs registered in your Miro app.

## 2. Environment Setup

### Create .env File

```bash
# Copy the example file
cp .env.example .env

# Generate encryption key
openssl rand -hex 32

# Edit .env with your values
```

### Configure .env

Open `.env` and fill in:

```env
# From Miro Developer Portal
MIRO_CLIENT_ID=3458764647516852398
MIRO_CLIENT_SECRET=your_secret_here

# Local testing redirect URI
MIRO_REDIRECT_URI=http://localhost:3000/oauth/callback

# Generate fresh key with: openssl rand -hex 32
MIRO_ENCRYPTION_KEY=abc123def456...  # 64-character hex string

# Server port (default: 3000)
MCP_SERVER_PORT=3000
```

**Security Checklist**:
- ✅ `.env` is in `.gitignore`
- ✅ Encryption key is 64 hex characters (32 bytes)
- ✅ Client secret never committed to version control
- ✅ Different keys for development and production

## 3. Build and Run Server

### Start the Server

```bash
# From project root
cargo run
```

**Expected output**:
```
Starting Miro MCP Server
Configuration loaded successfully
OAuth HTTP server started on port 3000
MCP server initialized
OAuth callback URL: http://127.0.0.1:3000/oauth/callback
```

### Verify Server Health

```bash
# In another terminal
curl http://localhost:3000/health
```

**Expected response**:
```
OK
```

## 4. Test OAuth2 Flow

### Step 1: Initiate Authorization

Open in browser:
```
http://localhost:3000/oauth/authorize
```

**What happens**:
1. Server generates PKCE code challenge and CSRF token
2. Creates encrypted cookie `miro_oauth_state` with:
   - CSRF token
   - PKCE verifier
   - Creation timestamp (for 10-minute expiry)
3. Redirects to Miro authorization page

**Verify in DevTools** (Application → Cookies → localhost:3000):
- Cookie name: `miro_oauth_state`
- Attributes: `HttpOnly`, `Secure` (if HTTPS), `SameSite=Lax`
- Value: Long encrypted string (base64-encoded AES-256-GCM ciphertext)

### Step 2: Authorize on Miro

On Miro authorization page:
1. Review requested permissions (boards:read, boards:write)
2. Click **Allow** to authorize the app
3. Miro redirects back to: `http://localhost:3000/oauth/callback?code=...&state=...`

### Step 3: Token Exchange

**What happens automatically**:
1. Server extracts `miro_oauth_state` cookie from request
2. Decrypts and validates:
   - Cookie not expired (< 10 minutes old)
   - CSRF token matches `state` query parameter
3. Extracts PKCE verifier from cookie
4. Exchanges authorization code for access token using PKCE verifier
5. Saves encrypted access token to `~/.local/share/miro-mcp-server/tokens.enc`
6. Shows success page

**Expected browser output**:
```
✓ Authorization Successful!
Your Miro account has been connected.
You can now close this window and return to Claude.
```

**Server logs** (check terminal):
```
Received OAuth callback with code
Redirecting to Miro authorization URL with encrypted state cookie
OAuth tokens saved successfully
```

## 5. Verification Steps

### Verify Cookie Encryption

**Check cookie state**:
1. Open DevTools → Application → Cookies
2. Find `miro_oauth_state` cookie
3. Copy encrypted value (starts with base64 characters)

**Attempt to decode** (should fail without key):
```bash
echo "cookie_value_here" | base64 -d
# Should show binary gibberish (AES-256-GCM ciphertext)
```

**Security expectation**: Cookie contents are encrypted and cannot be read/modified without the `MIRO_ENCRYPTION_KEY`.

### Verify Token Storage

**Check token file**:
```bash
cat ~/.local/share/miro-mcp-server/tokens.enc
```

**Expected**: Encrypted JSON blob (not human-readable)

**Check file permissions**:
```bash
ls -la ~/.local/share/miro-mcp-server/tokens.enc
```

**Expected**: `rw-------` (600) - only readable by owner

### Verify OAuth State Expiry

**Test expired state**:
1. Initiate authorization: `http://localhost:3000/oauth/authorize`
2. Copy `miro_oauth_state` cookie value
3. Wait 11 minutes (TTL is 10 minutes)
4. Manually complete callback with old state
5. Should get error: "Cookie validation failed: OAuth state expired"

### Verify CSRF Protection

**Test CSRF attack**:
1. Initiate authorization: `http://localhost:3000/oauth/authorize`
2. Intercept redirect to Miro (copy URL)
3. Modify `state` parameter in URL
4. Complete authorization with modified state
5. Should get error: "Cookie validation failed: CSRF token mismatch"

## 6. Test MCP Tools

After successful OAuth, test MCP tools via stdin/stdout:

```bash
# MCP tools are available via stdio transport
# Use Claude Desktop or MCP client to test:
# - list_boards
# - create_board
# - create_sticky_note
# etc.
```

**Note**: Full MCP testing requires MCP client integration (Claude Desktop or test harness).

## 7. Troubleshooting

### Error: "OAuth state cookie not found"

**Cause**: Cookie not sent or browser blocking cookies

**Fix**:
1. Check browser allows cookies for localhost
2. Verify `miro_oauth_state` cookie exists in DevTools
3. Ensure no browser extensions blocking cookies

### Error: "OAuth state expired"

**Cause**: More than 10 minutes between `/authorize` and `/callback`

**Fix**:
1. Complete OAuth flow faster (< 10 minutes)
2. If testing expiry, this is expected behavior

### Error: "CSRF token mismatch"

**Cause**: State parameter tampering or cookie-state mismatch

**Fix**:
1. Don't modify URLs during OAuth flow
2. Clear cookies and restart flow
3. Check for reverse proxy/middleware modifying state

### Error: "Failed to load configuration"

**Cause**: Missing or invalid `.env` file

**Fix**:
1. Verify `.env` file exists in project root
2. Check all required variables are set
3. Validate encryption key format: 64 hex characters
4. Ensure no extra whitespace in values

### Error: "Invalid redirect URI"

**Cause**: Miro app not configured with local redirect URI

**Fix**:
1. Go to Miro Developer Portal
2. Add `http://localhost:3000/oauth/callback` to allowed redirect URIs
3. Save configuration
4. Restart OAuth flow

### Server won't start - "Address already in use"

**Cause**: Port 3000 already occupied

**Fix**:
```bash
# Check what's using port 3000
lsof -i :3000

# Kill process or change port in .env
MCP_SERVER_PORT=3001
```

### Encryption key error

**Cause**: Invalid encryption key format

**Fix**:
```bash
# Regenerate valid key (should output 64 hex characters)
openssl rand -hex 32

# Update MIRO_ENCRYPTION_KEY in .env
# Format: 64 characters from [0-9a-f]
```

## 8. Testing Checklist

Before deploying to Scaleway Functions:

- [ ] OAuth authorization flow completes successfully
- [ ] Encrypted cookies created with correct attributes
- [ ] CSRF token validation works (reject mismatched state)
- [ ] State expiry enforced (reject >10-minute-old cookies)
- [ ] Token exchange successful with PKCE verifier
- [ ] Access token saved encrypted to disk
- [ ] Token file has correct permissions (600)
- [ ] Health check endpoint responds
- [ ] Server logs show no errors
- [ ] Browser DevTools shows encrypted cookies
- [ ] Manual CSRF attack blocked
- [ ] Manual expired state rejected

## 9. Cookie Inspection

### View Cookie Details in Browser

**Chrome/Edge DevTools**:
1. Open DevTools (F12)
2. Application tab → Cookies → http://localhost:3000
3. Find `miro_oauth_state`
4. Check attributes: HttpOnly, Secure, SameSite

**Firefox DevTools**:
1. Open DevTools (F12)
2. Storage tab → Cookies → http://localhost:3000
3. Select `miro_oauth_state`
4. View attributes in right panel

**Safari DevTools**:
1. Develop → Show Web Inspector
2. Storage tab → Cookies → localhost
3. Find `miro_oauth_state`

### Expected Cookie Attributes

```
Name: miro_oauth_state
Value: <long base64-encoded encrypted string>
Domain: localhost
Path: /
Expires: Session (deleted on browser close)
HttpOnly: ✓ (JavaScript cannot access)
Secure: ✓ (HTTPS only - may be ✗ for localhost HTTP)
SameSite: Lax (sent on navigation, not on cross-site requests)
```

## 10. Next Steps

After successful local testing:

1. **Document findings**: Note any issues encountered
2. **Prepare for production**:
   - Generate production encryption key
   - Configure Scaleway Functions redirect URI
   - Set up Scaleway Secret Manager
3. **Proceed with AUTH5**: Access token cookie storage
4. **Deploy to Scaleway**: See DEPLOY2 in backlog

## Security Notes

**Local Testing Security**:
- Use HTTP for localhost (Secure cookie flag will be omitted)
- Generate fresh encryption key for each environment
- Never use production credentials for local testing
- Clear browser cookies after testing

**Production Security**:
- HTTPS required (Secure cookie flag mandatory)
- Rotate encryption keys periodically
- Use Scaleway Secret Manager for credentials
- Enable audit logging (Cockpit)
- Monitor OAuth events for anomalies

## References

- [ADR-001: OAuth2 Stateless Architecture](planning/adr-001-oauth2-stateless-architecture.md)
- [Miro REST API Documentation](https://developers.miro.com/docs/rest-api-overview)
- [OAuth 2.0 RFC 6749](https://tools.ietf.org/html/rfc6749)
- [PKCE RFC 7636](https://tools.ietf.org/html/rfc7636)
