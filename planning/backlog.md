# Miro MCP Server - Product Backlog

## Completed
- [x] AUTH1: User authenticates with Miro securely via OAuth2 ✅ 2025-11-10
- [x] AUTH2: System refreshes tokens automatically (vs manual re-auth) ✅ 2025-11-10
- [x] BOARD1: User lists accessible Miro boards programmatically ✅ 2025-11-10
- [x] BOARD2: User creates new boards via Claude prompts ✅ 2025-11-10
- [x] VIS1: User creates sticky notes with custom content and styling ✅ 2025-11-10
- [x] VIS2: User creates shapes for organizational structure ✅ 2025-11-10
- [x] VIS3: User creates text elements on boards ✅ 2025-11-10
- [x] VIS4: User creates frames for grouping related content ✅ 2025-11-10
- [x] ITEM1: User lists board items filtered by type ✅ 2025-11-10
- [x] ITEM2: User updates item properties dynamically ✅ 2025-11-10
- [x] ITEM3: User removes items from boards ✅ 2025-11-10
- [x] REL1: User connects items with styled arrows/lines ✅ 2025-11-10
- [x] REL2: User adds captions to connectors ✅ 2025-11-10
- [x] BULK1: User creates multiple items efficiently (vs individual API calls) ✅ 2025-11-10
- [x] TECH1: MCP server responds to protocol requests (vs crashing on tools/list) ✅ 2025-11-10
- [x] LAYER2: User understands item stacking order when reading/creating items ✅ 2025-11-10
- [x] FRAME1: User creates items directly in frames (vs manual move after creation) ✅ 2025-11-10
- [x] FRAME2: User moves items between frames for reorganization ✅ 2025-11-10
- [x] FRAME3: User filters items by containing frame ✅ 2025-11-10
- [x] FRAME4: User removes items from frames to board root ✅ 2025-11-10
- [x] TECH2: Developer modifies parent construction in single location (vs 5 duplications) ✅ 2025-11-10
- [x] TEST1: Parent filtering verified through integration tests (vs unit-only coverage) ✅ 2025-11-10
- [x] TECH4: System validates sort_by values explicitly (vs silent failures) ✅ 2025-11-10
- [x] DEPLOY1: Developer deploys to Scaleway in <5min (vs manual local setup) ✅ 2025-11-10
- [x] CI1: Developer receives automated test feedback on every push (vs manual local testing) ✅ 2025-11-10
- [x] TECH3: Developer adds complex items via builder pattern (vs 9-parameter functions) ✅ 2025-11-10
- [x] TECH5: Developer adds new tools without modifying routing (vs hardcoded match) ✅ 2025-11-10
- [x] AUTH3: User completes OAuth flow in browser from Claude Desktop (vs manual token management) ✅ 2025-11-10
- [x] AUTH4: Developer adds OAuth state via encrypted cookies (vs in-memory HashMap) ✅ 2025-11-10

## In Progress

- [ ] **AUTH5**: User's access token stored in encrypted cookies (vs server-side storage)

## Blocked
- [🚫] LAYER1.1: User controls z-order stacking (bring to front, send to back) ⚠️ Web SDK only
- [🚫] LAYER1.2: User manages organizational layers (visibility, locking) ⚠️ UI-only feature

## Planned

### Critical (ADR-001 Implementation - Blocks Serverless Deployment)

- [ ] **AUTH5**: User's access token stored in encrypted cookies (vs server-side storage)
  - **Outcome**: System remains stateless and secure with cookie-based token storage
  - **Acceptance Criteria**:
    - Store access token in encrypted httpOnly cookie
    - Set 1-hour expiration matching token lifetime
    - Implement Secure and SameSite=strict attributes
    - Clear oauth_state cookie after successful exchange
    - Middleware extracts and validates token from cookie
    - Test token theft protection (XSS, CSRF scenarios)
  - **Dependencies**: AUTH4 (cookie encryption infrastructure)
  - **Complexity**: 1.5 (builds on AUTH4 pattern)

### High Priority (Production Readiness)

- [ ] **DEPLOY2**: Serverless platform configured for Pattern B architecture
  - **Outcome**: Production deployment infrastructure ready with cost-effective serverless platform
  - **Acceptance Criteria**:
    - Evaluate AWS Lambda vs Cloudflare Workers vs Vercel Functions
    - Document cost comparison for 1-100 users scale
    - Set up basic deployment pipeline
    - Configure environment variables (CLIENT_ID, SECRET, ENCRYPTION_KEY)
    - Verify HTTPS certificate configuration
    - Test cold start performance (<500ms)
  - **Dependencies**: AUTH4, AUTH5 (stateless implementation complete)
  - **Complexity**: 1.5 (deployment setup)

- [ ] **TEST2**: Stateless authentication verified through comprehensive integration tests
  - **Outcome**: Prevent regressions in security-critical stateless cookie implementation
  - **Acceptance Criteria**:
    - Test PKCE validation (wrong verifier rejected)
    - Test state validation (CSRF attack blocked)
    - Test expired state (10-min timeout enforced)
    - Test expired access token (1-hour refresh)
    - Test cold start simulation (state persists in cookies)
    - Test concurrent auth flows (no state collision)
  - **Dependencies**: AUTH4, AUTH5
  - **Complexity**: 1.5 (complex security test scenarios)

### Medium Priority (Operational Excellence)

- [ ] **DEPLOY3**: Developer monitors production auth via structured logs and CloudWatch queries
  - **Outcome**: Audit trail and debugging capability for authentication events
  - **Acceptance Criteria**:
    - Implement structured logging for auth events (initiate, callback, refresh)
    - Log session IDs for audit trail
    - Set up CloudWatch Insights queries (failed auth, token refresh rate)
    - Document emergency revocation procedure
    - Create dashboard for auth metrics
  - **Dependencies**: DEPLOY2 (serverless platform configured)
  - **Complexity**: 1.0 (observability setup)

### Documentation

- [ ] **DOC1**: Developer understands stateless OAuth2 pattern through comprehensive documentation
  - **Outcome**: Future maintainers grasp architecture and security trade-offs
  - **Acceptance Criteria**:
    - Document Pattern B architecture (PKCE + encrypted cookies)
    - Explain why stateless vs database (ADR-001 rationale)
    - Document cookie encryption implementation
    - Provide example flows (authorization, callback, token use)
    - Document migration path to database (if needed >100 users)
    - Link to ADR-001 and industry references
  - **Dependencies**: AUTH4, AUTH5 (implementation complete)
  - **Complexity**: 0.5 (documentation only)
