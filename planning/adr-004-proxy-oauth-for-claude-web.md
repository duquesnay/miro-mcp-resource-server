# ADR-004: Proxy OAuth Pattern for Claude.ai Web Integration

**Status:** Accepted (Supersedes ADR-002/ADR-003 HTTP mode for Claude.ai web)
**Date:** 2025-11-11
**Context:** Miro MCP Server HTTP deployment for Claude.ai web custom connectors
**Decision Makers:** Solution Architect, Security Specialist

---

## Executive Summary

**What changed**: HTTP Resource Server → HTTP Proxy OAuth

**Why**: Claude.ai web custom connectors use convention-based OAuth URLs, ignoring RFC 9728 metadata discovery. The Resource Server pattern (ADR-002/ADR-003) doesn't work with Claude.ai web.

**How**: Server implements OAuth proxy endpoints (`/authorize`, `/callback`, `/token`) between Claude.ai and Miro, using encrypted cookies for stateless state management.

---

## Context

### What We Built (ADR-002/ADR-003 HTTP Mode)

**Architecture**: OAuth Resource Server (RFC 9728)
```
Claude.ai → Bearer token → MCP Server → Validates with Miro → Proxies API calls
```

**Implementation**:
- OAuth metadata endpoint: `/.well-known/oauth-protected-resource`
- Bearer token validation via Miro API
- In-memory LRU cache (5-minute TTL)
- WWW-Authenticate headers (RFC 6750 compliance)
- Successfully deployed and tested locally

**Assumption**: Claude Platform would discover `authorization_endpoint` from metadata and handle OAuth

---

### What We Discovered

**Testing with Claude.ai web custom connectors revealed**:

1. **Claude.ai ignores metadata discovery**
   - When Client ID entered, Claude redirects to: `{our-server}/authorize`
   - Does NOT use `authorization_endpoint` from `/.well-known/oauth-protected-resource`
   - Convention-based routing, not standards-based discovery

2. **Claude.ai expects Proxy OAuth pattern**
   - Server must implement: `/authorize`, `/callback`, `/token` endpoints
   - Server acts as OAuth proxy between Claude.ai (client) and Miro (provider)
   - This matches vault-server implementation (proven working pattern)

3. **Resource Server pattern incompatible**
   - Claude.ai doesn't send Bearer tokens to MCP server
   - Claude.ai delegates OAuth flow TO the MCP server
   - MCP server must obtain and manage tokens on behalf of Claude.ai

**Evidence**: When entering `MIRO_CLIENT_ID=3458764647516852398` in Claude.ai connector config, browser redirected to `http://localhost:3000/authorize` (our server), not Miro's authorization endpoint.

---

## Decision

**Switch to Proxy OAuth pattern for Claude.ai web integration.**

### New Architecture

```
User → Claude.ai Web
         ↓ (initiates OAuth)
Your MCP Server /authorize
         ↓ (redirects with PKCE)
Miro Authorization (user authorizes)
         ↓ (callback with code)
Your MCP Server /callback
         ↓ (exchange code for tokens with PKCE)
Miro Token Endpoint
         ↓ (returns access + refresh tokens)
Your MCP Server
         ↓ (encrypts tokens in cookie)
Returns cookie to Claude.ai
         ↓
Claude.ai makes MCP requests with cookie
         ↓
Your MCP Server (decrypts cookie → uses token)
         ↓
Miro API
```

### Key Components

**OAuth Proxy Endpoints**:
- `GET /authorize` - Initiates OAuth with Miro (generates PKCE, state)
- `GET /callback` - Handles Miro callback (validates state, exchanges code)
- `POST /token` - Token refresh endpoint (uses refresh token)

**State Management**:
- **Method**: Encrypted cookies (stateless, no database)
- **Contents**: Access token, refresh token, expiry, user info
- **Security**: AES-GCM encryption via `ring` crate
- **Lifetime**: 60 days (refresh token validity from Miro)

**PKCE Implementation**:
- Code verifier: 128-byte random (base64url encoded)
- Code challenge: SHA-256 hash of verifier
- Challenge method: `S256`
- Prevents authorization code interception

**Environment Variables**:
```bash
BASE_URL=https://your-mcp-server.scaleway.io  # For redirect URI
MIRO_CLIENT_ID=3458764647516852398
MIRO_CLIENT_SECRET=<from Miro app settings>
MIRO_ENCRYPTION_KEY=<generated 32-byte key>
PORT=3000
```

---

## Implementation Plan

### New Files (7 total)

**1. `src/oauth/mod.rs`**
- Module declaration
- Re-exports: `ProxyOAuthProvider`, `handle_authorize`, `handle_callback`, `handle_token`

**2. `src/oauth/proxy_provider.rs`**
- `ProxyOAuthProvider` struct
- Miro OAuth configuration
- PKCE generation
- Authorization URL construction
- Token exchange logic

**3. `src/oauth/endpoints.rs`**
- `GET /authorize` handler (redirects to Miro with PKCE)
- `GET /callback` handler (validates state, exchanges code, sets cookie)
- `POST /token` handler (refreshes token from cookie)

**4. `src/oauth/cookie_manager.rs`**
- Encrypt/decrypt cookie contents
- Cookie serialization/deserialization
- Cookie creation/parsing

**5. `src/oauth/pkce.rs`**
- Generate code verifier (128 random bytes)
- Generate code challenge (SHA-256 hash)
- Validate verifier/challenge pair

**6. `src/oauth/state.rs`**
- Generate cryptographically secure state
- Store state temporarily (in-memory map with TTL)
- Validate state on callback

**7. `src/oauth/types.rs`**
- `OAuthState` struct (state + PKCE verifier)
- `CookieData` struct (tokens + expiry + user info)
- `TokenResponse` struct (Miro token endpoint response)

### Modified Files (3 total)

**1. `src/bin/http-server-adr002.rs`** → Rename to `http-server.rs`
- Add OAuth routes: `.route("/authorize", get(handle_authorize))`
- Add callback route: `.route("/callback", get(handle_callback))`
- Add token route: `.route("/token", post(handle_token))`
- Update AppState with `ProxyOAuthProvider`

**2. `src/auth/metadata.rs`**
- Remove OAuth Protected Resource metadata
- Add OAuth Authorization Server metadata
- Update endpoints:
  - `authorization_endpoint`: `{BASE_URL}/authorize`
  - `token_endpoint`: `{BASE_URL}/token`
  - `response_types_supported`: `["code"]`
  - `grant_types_supported`: `["authorization_code", "refresh_token"]`
  - `code_challenge_methods_supported`: `["S256"]`

**3. `src/config.rs`**
- Add `BASE_URL` environment variable
- Add `MIRO_ENCRYPTION_KEY` environment variable
- Validation: `BASE_URL` must be HTTPS in production

### Dependencies to Add

```toml
[dependencies]
# Existing dependencies unchanged...

# New for Proxy OAuth
ring = "0.17"           # Cookie encryption (AES-GCM)
base64 = "0.22"         # Base64url encoding for PKCE
sha2 = "0.10"           # SHA-256 for PKCE challenge
rand = "0.8"            # Cryptographically secure random for state/PKCE
oauth2 = "4.4"          # OAuth client library (authorization code flow)
```

**Why `ring` is now needed**: Persistent cookie encryption (vs ADR-003's in-memory cache)

---

## Comparison: ADR-002/ADR-003 vs ADR-004

| Aspect | ADR-002/ADR-003 HTTP Mode | ADR-004 Proxy OAuth |
|--------|---------------------------|---------------------|
| **OAuth Role** | Resource Server | Proxy |
| **Token Source** | Bearer header from Claude | Direct OAuth with Miro |
| **OAuth Handler** | Claude Platform | MCP Server |
| **Token Storage** | In-memory cache (5min TTL) | Encrypted cookies (60 day TTL) |
| **State Management** | Stateless validation | Stateful (encrypted cookies) |
| **Encryption** | Not needed | Required (ring) |
| **PKCE** | N/A | Required |
| **Endpoints** | `/.well-known/oauth-protected-resource` | `/authorize`, `/callback`, `/token` |
| **Claude.ai Web** | ❌ Doesn't work (ignored metadata) | ✅ Works (convention-based routing) |
| **Complexity** | 150 LOC | 500 LOC |
| **Dependencies** | No ring | Needs ring |
| **Compilation** | Fast (~30s) | Slower (~2min) |
| **Scalability** | Stateless (scales easily) | Cookie-based (scales with sticky sessions) |

---

## Consequences

### Positive

✅ **Works with Claude.ai web** (primary requirement met)
- Matches Claude.ai's convention-based OAuth expectations
- Proven pattern (vault-server reference implementation)

✅ **Stateless scalability** (despite cookie state)
- Encrypted cookies remove need for database
- No server-side session storage
- Can deploy multiple instances (cookie decrypts anywhere)

✅ **Security via PKCE**
- Prevents authorization code interception
- No client secret exposed to browser
- State validation prevents CSRF

✅ **Long-lived sessions**
- 60-day refresh token validity
- Users don't re-authenticate frequently
- Transparent token refresh

### Negative

❌ **More complex implementation**
- 7 new files vs 0 for Resource Server
- OAuth flow logic in MCP server
- Cookie encryption/decryption overhead

❌ **Slower compilation**
- `ring` dependency adds ~1.5min to build
- Not as fast as ADR-002's approach

❌ **Server manages tokens**
- MCP server responsible for token security
- More attack surface than stateless validation
- Cookie encryption key must be protected

### Trade-offs

⚖️ **Cookie-based state** (vs database)
- PRO: No database needed (simpler deployment)
- PRO: Stateless from server perspective (cookie is portable)
- CON: Cookie size limits (~4KB max)
- CON: Client can delete cookie (forces re-authentication)

⚖️ **Multi-instance deployment** (sticky sessions)
- PRO: Cookie decrypts on any instance (shared encryption key)
- CON: State map (for CSRF validation) is instance-local
- SOLUTION: Keep state TTL short (5min), acceptable failure rate

---

## Security Considerations

### Threat Model

**Protected Against**:
- ✅ CSRF attacks → State parameter validation
- ✅ Code interception → PKCE (S256)
- ✅ Token theft from cookie → AES-GCM encryption
- ✅ Replay attacks → State nonce (single-use)
- ✅ Token theft in transit → HTTPS required

**Attack Vectors**:
- ⚠️ Cookie theft from client → Mitigated by HttpOnly, Secure, SameSite flags
- ⚠️ Encryption key compromise → All cookies decryptable (rotate key immediately)
- ⚠️ State map exhaustion → DoS possible (limit state storage, TTL eviction)

### Security Best Practices

**Cookie Settings**:
```rust
Cookie::build("miro_auth", encrypted_cookie_data)
    .http_only(true)      // No JavaScript access
    .secure(true)          // HTTPS only
    .same_site(SameSite::Strict)  // CSRF protection
    .max_age(Duration::days(60))  // Match refresh token validity
    .path("/")
```

**Encryption**:
- Algorithm: AES-256-GCM (authenticated encryption)
- Key derivation: Direct 32-byte key from `MIRO_ENCRYPTION_KEY`
- Nonce: Random 12-byte per encryption (prepended to ciphertext)
- Authentication: GCM tag prevents tampering

**State Management**:
- State: 32-byte random (base64url encoded)
- Storage: In-memory HashMap with 5-minute TTL
- Cleanup: Background task evicts expired states
- Limit: Max 10,000 states (prevents memory exhaustion)

---

## Migration Path

### From ADR-002/ADR-003 Implementation

**Step 1: Add dependencies**
- Update `Cargo.toml` with ring, oauth2, base64, sha2, rand

**Step 2: Create OAuth module**
- Implement 7 new files in `src/oauth/`
- Copy PKCE logic from vault-server reference

**Step 3: Update HTTP server**
- Add OAuth routes to `http-server.rs`
- Update AppState with `ProxyOAuthProvider`

**Step 4: Update metadata**
- Change metadata from Resource Server to Authorization Server
- Update endpoints in `metadata.rs`

**Step 5: Environment configuration**
- Add `BASE_URL` and `MIRO_ENCRYPTION_KEY`
- Update deployment config (Scaleway)

**Step 6: Testing**
- Test OAuth flow locally (curl + browser)
- Test with Claude.ai web connector
- Verify cookie encryption/decryption
- Validate PKCE implementation

**Step 7: Deprecate old code**
- Remove `src/auth/token_validator.rs` (no longer needed)
- Remove Bearer token validation logic
- Keep MiroClient unchanged (still uses tokens)

---

## Testing Strategy

### Local Testing

**1. OAuth Flow** (manual browser test):
```bash
# Start server
cargo run --bin http-server

# Visit in browser
http://localhost:3000/authorize

# Should redirect to Miro authorization
# After approval, callback should set cookie
# Verify cookie in browser DevTools
```

**2. Token Refresh** (automated test):
```rust
#[tokio::test]
async fn test_token_refresh() {
    // Simulate expired access token
    // Call /token endpoint with refresh token
    // Verify new access token received
    // Verify cookie updated
}
```

**3. Cookie Encryption** (unit test):
```rust
#[test]
fn test_cookie_encryption_roundtrip() {
    let original = CookieData { /* ... */ };
    let encrypted = encrypt_cookie(&original, &key)?;
    let decrypted = decrypt_cookie(&encrypted, &key)?;
    assert_eq!(original, decrypted);
}
```

### Integration Testing with Claude.ai

**Test Cases**:
1. Initial OAuth flow (user authorizes Miro)
2. MCP tool calls with valid cookie
3. Token refresh (after 1 hour expiry)
4. Cookie deletion (forces re-authentication)
5. Invalid state parameter (CSRF attempt)
6. Expired authorization code (>10min delay)

---

## Implementation Estimates

**Complexity Breakdown** (from solution architect analysis):

| Component | Complexity | Estimated Time |
|-----------|------------|----------------|
| PKCE implementation | 1.5 | 1 hour |
| State management | 1.5 | 1 hour |
| Cookie encryption | 2.0 | 1.5 hours |
| Proxy provider | 2.0 | 1.5 hours |
| OAuth endpoints | 2.5 | 2 hours |
| Metadata updates | 1.0 | 30 min |
| HTTP server integration | 1.5 | 1 hour |
| **Total** | **12.0** | **~8.5 hours** |

**Testing**: +2 hours (integration tests, manual Claude.ai testing)

**Total Estimate**: ~10-11 hours (1.5 days)

---

## References

### Standards
- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749) - Authorization Code Flow
- [PKCE RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636) - Proof Key for Code Exchange
- [OAuth Security BCP](https://datatracker.ietf.org/doc/html/draft-ietf-oauth-security-topics) - Security Best Practices

### API Documentation
- [Miro OAuth Guide](https://developers.miro.com/docs/getting-started-with-oauth)
- [Miro Token Endpoint](https://developers.miro.com/reference/exchange-authorization-code-with-access-token)

### Reference Implementations
- `vault-server` - Proven Proxy OAuth pattern for Claude.ai web
- Solution architect implementation plan (conversation context)

### Related ADRs
- **ADR-001** (Superseded): OAuth2 Stateless Architecture - Early Proxy OAuth concept
- **ADR-002** (Superseded for web): OAuth Resource Server - Doesn't work with Claude.ai
- **ADR-003** (Superseded for web): Dual-Mode Architecture - HTTP mode incompatible
- **ADR-004** (This document): Proxy OAuth for Claude.ai web integration

---

## Rollback Plan

**If Proxy OAuth fails in production**:

1. **Immediate**: Claude.ai web won't work (expected - Resource Server incompatible)
2. **Fallback**: No fallback possible - Claude.ai requires Proxy OAuth
3. **Alternative**: Use Claude Desktop with stdio mode (future ADR-005?)

**Monitoring**:
- Log OAuth flow completions (success rate)
- Track token refresh failures
- Monitor cookie decryption errors
- Alert on state validation failures (potential CSRF attacks)

---

## Next Steps

**Immediate** (this sprint):
1. Implement 7 new OAuth files
2. Update HTTP server with OAuth routes
3. Test OAuth flow locally
4. Test with Claude.ai web connector

**Follow-up** (next sprint):
1. Production deployment to Scaleway
2. HTTPS configuration
3. Monitor OAuth flow success rate
4. Document troubleshooting guide for users

**Future** (optional):
1. stdio mode for Claude Desktop (ADR-005?)
2. Multi-instance deployment testing (sticky sessions)
3. Cookie rotation strategy (security hardening)

---

## Decision Rationale

**Why switch from Resource Server to Proxy OAuth?**

**Empirical evidence**: Testing showed Claude.ai doesn't use metadata discovery - it uses convention-based routing (`/authorize`, `/callback`, `/token`). The Resource Server pattern (ADR-002/ADR-003) is architecturally sound but incompatible with Claude.ai's implementation.

**Proven pattern**: vault-server uses Proxy OAuth and works with Claude.ai web - this validates the approach.

**Trade-off acceptance**: More complexity and slower compilation are acceptable costs for the primary requirement (Claude.ai web integration).

**No alternative**: Resource Server won't work with Claude.ai web - this isn't a preference, it's a requirement.

---

**Status**: Accepted
**Supersedes**: ADR-002 (for Claude.ai web), ADR-003 HTTP mode (for Claude.ai web)
**Next Review**: After production deployment and 1 week of monitoring
