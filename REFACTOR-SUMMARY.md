# Resource Server Pattern Refactor - Summary

**Date**: 2025-11-11
**Branch**: `feat/resource-server-pattern`
**Status**: Ready to implement

---

## TL;DR

We built a **complex OAuth proxy** (ADR-004) when a **simple Resource Server** pattern (ADR-005) would work.

**Savings**: 85% less code, 66% fewer secrets, 60-70% faster to production.

---

## What We're Changing

### FROM: Authorization Server Pattern (ADR-004)
```
User → Our Server (/oauth/authorize) → Miro OAuth
              ↓
Miro → Our Server (/oauth/callback) → Store encrypted tokens
              ↓
User → Our Server (reads stored tokens) → Miro API
```

**Complexity**: ~1000 LOC + encryption + token management

### TO: Resource Server Pattern (ADR-005)
```
User → Claude → Miro OAuth (redirect to claude.ai/api/mcp/auth_callback)
              ↓
         Claude stores tokens
              ↓
User → Claude → Our Server (with access token) → Miro API validates
```

**Complexity**: ~150 LOC (metadata + token validation only)

---

## Why We're Changing

### Discovery Timeline
1. **Built ADR-004**: Implemented complex OAuth proxy thinking it was required
2. **Deployment investigation**: Compared with vault-server for Scaleway setup
3. **Vault-server pattern**: Uses Claude's callback URL `claude.ai/api/mcp/auth_callback`
4. **Miro verification**: Confirmed Miro accepts external redirect URIs
5. **MCP spec check**: RFC 9728 explicitly supports Resource Server pattern

### Key Findings
- ✅ Miro accepts `claude.ai/api/mcp/auth_callback` as redirect URI
- ✅ MCP OAuth 2.1 spec supports Resource Server pattern (RFC 9728)
- ✅ Multiple MCP servers successfully use this pattern (vault-server, etc.)
- ✅ Claude.ai Pro/Team/Enterprise supports OAuth flow natively

---

## Implementation Plan

### Phase 1: Configuration (15 min)
- Update Miro app: Change redirect URI to `claude.ai/api/mcp/auth_callback`

### Phase 2: Code Cleanup (30 min)
- Delete OAuth proxy files (`oauth.rs`, `token_store.rs`, `cookie_manager.rs`, `pkce.rs`)
- Remove OAuth dependencies from `Cargo.toml`

### Phase 3: Metadata Endpoint (1-2 hours)
- **OAUTH1**: Implement `/.well-known/oauth-protected-resource`
- **OAUTH2**: Return 401 with `WWW-Authenticate` header

### Phase 4: Token Validation (2-3 hours)
- **OAUTH3**: Validate Bearer tokens (verify audience, expiry)

### Phase 5: Config Updates (30 min)
- Remove secrets from `.env.production` and Scaleway
- Update deployment scripts

### Phase 6: Testing (1-2 hours)
- **TEST4**: End-to-end OAuth flow with Claude.ai
- **TEST5**: Token validation edge cases

### Phase 7: Documentation (1 hour)
- Update CLAUDE.md and README with simplified architecture

**Total Estimate**: 0.5-1 day (vs 2-3 days remaining for ADR-004)

---

## Before vs After Comparison

| Aspect | ADR-004 (Authorization Server) | ADR-005 (Resource Server) |
|--------|--------------------------------|---------------------------|
| **Lines of Code** | ~1000 LOC | ~150 LOC (85% reduction) |
| **Secrets Managed** | 3 (client_secret, encryption_key, tokens) | 0 on our server (66% reduction) |
| **OAuth Endpoints** | 3 (`/authorize`, `/callback`, `/token`) | 1 (`/.well-known/oauth-protected-resource`) |
| **Token Storage** | AES-256-GCM encryption required | None (Claude handles it) |
| **PKCE Implementation** | Custom implementation needed | Claude handles it |
| **State Management** | Encrypted cookies | None needed |
| **Token Refresh** | Manual implementation | Claude handles it |
| **Implementation Time** | 2-3 days remaining | 0.5-1 day total |
| **Debugging Complexity** | High (cookies, PKCE, state) | Low (token validation only) |
| **Security Surface** | Larger (3 secrets, encryption) | Smaller (token validation) |

---

## File Changes Overview

### DELETE (ADR-004 artifacts)
```
src/auth/oauth.rs              # ~200 LOC
src/auth/token_store.rs        # ~100 LOC
src/auth/cookie_manager.rs     # ~100 LOC
src/auth/pkce.rs               # ~50 LOC
```

### CREATE (ADR-005 implementation)
```
src/auth/metadata.rs           # ~50 LOC - Protected Resource Metadata
src/auth/token_validation.rs  # ~80 LOC - JWT validation
```

### MODIFY
```
src/http_server.rs             # Add metadata endpoint route
src/auth/middleware.rs         # Add 401 + WWW-Authenticate
.env.production                # Remove secrets
scripts/deploy.sh              # Remove secret injection
Cargo.toml                     # Remove OAuth dependencies
```

---

## Risk Assessment

### Risks: LOW
- Pattern validated by multiple MCP servers (vault-server, etc.)
- MCP specification explicitly supports this (RFC 9728)
- Miro confirmed to accept external redirect URIs
- Can rollback to ADR-004 if needed (branch still exists)

### Rollback Plan
If Resource Server pattern fails:
1. Return to `main` branch (ADR-004 work preserved)
2. Continue AUTH10-14 implementation
3. Document failure reason for future reference

---

## Success Criteria

### Must Have (P0)
- ✅ Miro redirect URI updated to `claude.ai/api/mcp/auth_callback`
- ✅ Metadata endpoint returns correct JSON (RFC 9728)
- ✅ 401 responses include WWW-Authenticate header
- ✅ Token validation verifies audience claim
- ✅ End-to-end OAuth flow works with Claude.ai
- ✅ All MCP tools work with Claude-provided tokens

### Nice to Have
- Token validation caching (5min TTL for performance)
- Comprehensive error messages for debugging
- Documentation with architecture diagrams

---

## Next Steps

### Immediate Actions
1. **Review ADR-005** - Read [planning/ADR-005-resource-server-with-claude-oauth.md](planning/ADR-005-resource-server-with-claude-oauth.md)
2. **Review Backlog** - Read [planning/REFACTOR-BACKLOG.md](planning/REFACTOR-BACKLOG.md)
3. **Start Implementation** - Begin with CONFIG1 (Miro redirect URI update)

### Implementation Order
```
CONFIG1 (Miro portal)
  → REMOVE1-2 (cleanup)
  → OAUTH1-2 (metadata)
  → OAUTH3 (validation)
  → CONFIG2-3 (env vars)
  → TEST4-5 (verification)
  → DOC3-4 (documentation)
```

### Merge Strategy
Once complete and tested:
1. Create PR from `feat/resource-server-pattern` to `main`
2. PR description references ADR-005 and explains simplification
3. Merge and close ADR-004 work items in backlog
4. Update main backlog with "Completed via ADR-005 refactor"

---

## References

- **ADR-005**: [planning/ADR-005-resource-server-with-claude-oauth.md](planning/ADR-005-resource-server-with-claude-oauth.md)
- **Refactor Backlog**: [planning/REFACTOR-BACKLOG.md](planning/REFACTOR-BACKLOG.md)
- **MCP Authorization Spec**: https://modelcontextprotocol.io/specification/2025-06-18/basic/authorization
- **RFC 9728** (Protected Resource Metadata): https://datatracker.ietf.org/doc/html/rfc9728
- **Vault-server reference**: `/Users/guillaume/dev/tools/vault-server/.env.production`

---

## Questions?

**Q: Why didn't we do this from the start?**
A: Lack of awareness. No reference implementations reviewed before starting. ADR-004 seemed like the only path.

**Q: What if Resource Server doesn't work?**
A: Rollback to `main` branch (ADR-004 preserved). Low risk - pattern validated by multiple projects.

**Q: When can we deploy to production?**
A: 0.5-1 day after starting OAUTH1 implementation. Much faster than 2-3 days remaining for ADR-004.

**Q: What about token refresh?**
A: Claude handles it transparently. We just validate tokens on each request.

---

**Ready to start? Begin with CONFIG1 in REFACTOR-BACKLOG.md** 🚀
