# CONFIG1: Miro Developer Portal Configuration

**Status**: ⏳ MANUAL STEP REQUIRED
**Priority**: P0 - Must complete before testing

---

## Action Required

Update Miro OAuth2.0 Redirect URI in Miro Developer Portal.

### Steps

1. **Open Miro Developer Portal**
   - Navigate to: https://miro.com/app/settings/user-profile/apps
   - Or: https://developers.miro.com/ → Your Apps

2. **Select Your App**
   - App Name: `miro-mcp-server` (or whatever you named it)
   - Client ID: `3458764647632208270`

3. **Update Redirect URI for OAuth 2.0**
   - Find section: "Redirect URI for OAuth 2.0"
   - **Remove old URI**: `https://miro-mcp.fly-agile.com/oauth/callback`
   - **Add new URIs** (both for future-proofing):
     1. `https://claude.ai/api/mcp/auth_callback`
     2. `https://claude.com/api/mcp/auth_callback`
   - Click "Add" for each URI
   - Click "Save" to apply changes

4. **Verify Configuration**
   - Confirm both URIs appear in the list
   - Note: Miro requires **exact matching** - no trailing slashes, correct protocol

### What This Does

**Before (ADR-004 Proxy OAuth)**:
- Miro redirects user back to our server: `fly-agile.com/oauth/callback`
- Our server exchanges code for token
- Our server stores encrypted token

**After (ADR-005 Resource Server)**:
- Miro redirects user to Claude: `claude.ai/api/mcp/auth_callback`
- Claude exchanges code for token
- Claude stores token securely
- Claude passes token to our server in API calls

### Verification

After updating, verify in Miro portal:
- ✅ `https://claude.ai/api/mcp/auth_callback` listed
- ✅ `https://claude.com/api/mcp/auth_callback` listed
- ✅ Old `https://miro-mcp.fly-agile.com/oauth/callback` removed (or keep if testing both)

### Security Note

The redirect URI is not a secret - it's part of OAuth security by **exact matching only**. Miro will reject authorization codes sent to any URI not registered here.

---

**Once completed, mark CONFIG1 as done and proceed with REMOVE1 (code cleanup).**
