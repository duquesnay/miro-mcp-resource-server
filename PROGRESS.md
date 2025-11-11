# ADR-005 Resource Server Refactor - Progress

**Date**: 2025-11-11
**Status**: 80% Complete - Core Implementation Done

---

## ✅ Completed

### Phase 1: Cleanup (DONE)
- ✅ CONFIG1: Miro redirect URI documented (manual step)
- ✅ REMOVE1: Deleted ADR-004 OAuth proxy files
  - `src/oauth/` directory removed
  - `src/oauth_dcr.rs`, `src/oauth_session.rs` removed
  - `src/auth/token_store.rs` removed
- ✅ REMOVE2: Updated Cargo.toml
  - Removed: oauth2, ring, aes-gcm, rand, sha2
  - Added: jsonwebtoken v9.3
  - Simplified features: removed oauth-proxy

### Phase 2: Core Implementation (DONE)
- ✅ OAUTH1: Protected Resource Metadata
  - `src/auth/metadata.rs` created
  - RFC 9728 compliant
  - Points to Miro authorization server
  - Includes tests
- ✅ OAUTH3: JWT Token Validation
  - `src/auth/token_validator.rs` rewritten
  - JWT decoding with audience + expiry verification
  - LRU cache (5min TTL) preserved
  - Comprehensive tests
- ✅ OAUTH2: WWW-Authenticate header
  - Already implemented in middleware (line 160-164 of http_server.rs)
  - Returns 401 with RFC 6750 compliant header

---

## ⏳ Remaining (20%)

### Phase 3: Integration Cleanup (30 min)
- [ ] Remove oauth-proxy feature guards from `src/http_server.rs`
  - Remove `#[cfg(feature = "oauth-proxy")]` blocks (~15 locations)
  - Clean up AppStateADR002 struct (remove oauth_provider, cookie_manager fields)
  - Remove OAuth endpoint routes (authorize, callback, token)
- [ ] Add metadata endpoint route
  - `GET /.well-known/oauth-protected-resource` → returns ProtectedResourceMetadata
- [ ] Update TokenValidator initialization
  - Pass `base_url` from config for audience validation
- [ ] Fix remaining compilation errors

### Phase 4: Configuration (15 min)
- [ ] CONFIG2: Update `.env.production`
  - Remove: `MIRO_CLIENT_SECRET`, `MIRO_ENCRYPTION_KEY`, `MIRO_REDIRECT_URI`
  - Keep: `MIRO_CLIENT_ID`, `BASE_URL`, `MCP_SERVER_PORT`
- [ ] CONFIG3: Update deployment scripts
  - Remove secret injection for deleted env vars
  - Update comments about OAuth pattern

### Phase 5: Testing (Local)
- [ ] `cargo test` - Run unit tests
- [ ] `cargo build --release` - Build binary
- [ ] Test metadata endpoint manually: `curl localhost:8080/.well-known/oauth-protected-resource`

### Phase 6: Documentation (15 min)
- [ ] DOC3: Update CLAUDE.md
  - Remove ADR-004 proxy references
  - Document ADR-005 Resource Server pattern
- [ ] DOC4: Update README
  - Simpler architecture diagram
  - Update OAuth configuration steps

---

## Integration Notes

### Key Files Modified
```
✅ src/auth/mod.rs               - Updated exports
✅ src/auth/metadata.rs           - NEW (OAuth metadata)
✅ src/auth/token_validator.rs   - Rewritten (JWT validation)
✅ Cargo.toml                     - Simplified dependencies
⏳ src/http_server.rs             - Needs cleanup (remove oauth-proxy guards)
⏳ .env.production                - Needs config cleanup
⏳ scripts/deploy.sh              - Needs script updates
```

### Compilation Status
- **Current**: ~15 warnings about oauth-proxy feature
- **Remaining errors**: 0 (method name fixed, imports cleaned)
- **Next**: Remove feature guards, build should pass

---

## Next Steps

1. **Finish http_server.rs cleanup** (20 min)
2. **Update configs** (10 min)
3. **Test locally** (10 min)
4. **Commit and merge to main** (5 min)

**Estimated Time to Completion**: 45 minutes

---

## Manual Steps Required

### Before Testing
1. **Update Miro Developer Portal** (CONFIG1)
   - Add redirect URI: `https://claude.ai/api/mcp/auth_callback`
   - Remove old URI: `https://miro-mcp.fly-agile.com/oauth/callback`

### Before Deployment
2. **Update Scaleway Secret Manager**
   - Delete: `MIRO_CLIENT_SECRET`, `MIRO_ENCRYPTION_KEY`
   - Keep: Regular env vars

---

**Status**: Core implementation complete ✅  
**Next**: Integration cleanup (30 min remaining)
