# Miro MCP Server - Project Framing

## Vision

Create a production-ready Model Context Protocol (MCP) server in Rust that enables Claude AI to programmatically create and manipulate Miro boards, with special focus on visualizing agile squad organizational structures. This will be the first OAuth2-enabled Miro MCP server, supporting remote deployment for Claude.ai web interface.

## Context

**Current State**:
- Existing Miro MCP servers (TypeScript) use static tokens only
- No OAuth2 flow implementation exists for Miro MCP
- Agile coaches manually create organizational diagrams in Miro

**Opportunity**:
- Developer has OAuth2 + MCP experience (GitHub MCP server)
- Miro API v2 is well-documented and stable
- Clear primary use case: agile squad visualization
- Active Miro credentials available for testing

**Target Deployment**:
- Remote MCP server accessible from Claude.ai web interface
- HTTPS/TLS required for OAuth2 redirect
- Proper token refresh for long-running sessions

---

## Team Structure

### Core Team (Consulted Systematically)

**1. solution-architect** - Implementation Planning & Architecture
- **When to consult**: Before starting each epic, for complex feature planning
- **Responsibilities**:
  - OAuth2 flow architecture decisions
  - MCP tool design and structure
  - Rust async patterns and library selection
  - API client architecture
  - Token storage and security patterns
- **Deliverables**: Implementation plans with file breakdowns, pattern recommendations

**2. developer** - Code Implementation
- **When to consult**: All code writing tasks (MANDATORY delegation)
- **Responsibilities**:
  - Implementing OAuth2 flows in Rust
  - Creating MCP tool handlers
  - Miro API client implementation
  - Error handling and validation
  - Integration testing
- **Deliverables**: Working code with tests
- **Model**: Use Haiku for simple tools (CRUD operations), Sonnet for OAuth2 and complex orchestration

**3. security-specialist** - OAuth2 & Token Security
- **When to consult**: Auth implementation, token storage, before production deployment
- **Responsibilities**:
  - OAuth2 flow security review
  - Token storage security (encryption at rest)
  - Secrets management validation
  - API credential handling
  - HTTPS/TLS configuration review
- **Deliverables**: Security findings with specific remediations

**4. integration-specialist** - MCP Protocol & API Integration
- **When to consult**: MCP tool design, API compatibility changes
- **Responsibilities**:
  - MCP protocol compliance validation
  - Miro API integration patterns
  - Tool parameter schema design
  - Cross-tool coordination (e.g., SQUAD1 orchestrating multiple tools)
  - API version compatibility
- **Deliverables**: Integration tests, compatibility matrices

---

### Support Team (On-Demand Consultation)

**5. architecture-reviewer** - SOLID & Design Review
- **When to consult**: After completing epics, before complex refactorings
- **Use case**: Ensure Rust code follows SOLID principles, review module structure

**6. performance-optimizer** - Scalability & Efficiency
- **When to consult**: Bulk operations implementation (BULK1), production performance issues
- **Use case**: Async/await patterns, connection pooling, rate limit handling

**7. code-quality-analyst** - Code Health
- **When to consult**: End of sprint, before major releases
- **Use case**: Identify DRY violations, complexity hotspots, maintainability issues

**8. git-workflow-manager** - Commit Hygiene
- **When to consult**: ALL commits (MANDATORY)
- **Use case**: Atomic commits, proper commit messages, history cleanliness

**9. documentation-writer** - API Documentation
- **When to consult**: Public API finalization, deployment documentation
- **Use case**: MCP tool documentation, OAuth2 setup guide, deployment instructions

---

### Collaboration Patterns

**Feature Development Flow**:
```
User Request
    ↓
solution-architect (plan implementation)
    ↓
developer (implement with tests)
    ↓
integration-specialist (validate MCP compliance)
    ↓
git-workflow-manager (atomic commit)
    ↓
[Complexity accumulator tracks progress]
    ↓
quality-orchestrator (every 2-3 features)
```

**Security-Critical Flow** (Auth, Token Management):
```
User Request
    ↓
solution-architect (security-aware architecture)
    ↓
security-specialist (review approach before implementation)
    ↓
developer (implement with security constraints)
    ↓
security-specialist (validate implementation)
    ↓
git-workflow-manager (commit)
```

**Bug Fix Flow**:
```
Bug Report
    ↓
developer (TDD: write failing test)
    ↓
developer (fix to make test pass)
    ↓
git-workflow-manager (commit with test)
```

---

## Agile Flow Configuration

```yaml
agile_flow:
  # Require estimation before starting work
  estimation_required: false  # Move fast for initial MVP

  # Quality review threshold (number of micro features)
  quality_review_threshold: 2-3  # Review every 2-3 features (standard)

  # Require backlog for all work
  backlog_required: true

  # Complexity multipliers
  complexity_weights:
    simple: 1.0    # Simple CRUD tools (create_text, delete_item)
    medium: 1.5    # API integration tools with state (list_boards, update_item)
    complex: 2.0   # OAuth2 flows, orchestration tools (SQUAD1)
```

**Rationale**:
- No estimation overhead for MVP (move fast)
- Standard quality review cadence (2-3 features)
- OAuth2 and squad orchestration are complex (2.0 weight)
- Simple tool implementations are lightweight (1.0 weight)

---

## Technical Constraints

### Language & Framework
- **Language**: Rust (stable)
- **MCP Framework**: TBD - evaluate existing Rust MCP libraries or implement from spec
- **HTTP Client**: reqwest (async, well-maintained)
- **OAuth2**: oauth2-rs crate (standard Rust OAuth2 implementation)
- **Serialization**: serde (JSON for MCP protocol and Miro API)

### Miro API Constraints
- **Rate Limit**: 100 requests/minute per user
- **Bulk Limit**: Max 20 items per bulk create operation
- **Token Expiry**: Access tokens expire after 3600 seconds (1 hour)
- **API Version**: v2 (stable, v1 deprecated for most endpoints)

### Stateless Architecture (ADR-002 Resource Server)
- **Authentication**: Bearer token validation only (no OAuth flow management)
- **Token Validation**: LRU cache in-memory (5-minute TTL, 95% hit rate)
- **No State Storage**: No cookies, sessions, or databases required
- **Scalability**: Stateless design enables horizontal scaling
- **Performance**: <1ms token validation latency (cached)
- **Deployment**: Container-based (Scaleway Containers) with persistent cache

### Security Requirements
- HTTPS/TLS mandatory for production deployment
- Bearer tokens validated with Miro API
- Tokens never logged (even in debug mode)
- Client ID/secret in environment variables only
- Audit logging for authentication events (correlation IDs)
- In-memory cache only (no persistent token storage)

### MCP Protocol Requirements
- Remote MCP server accessible via public URL
- Health check endpoint for monitoring
- Proper tool definitions with JSON schema
- Error responses following MCP error format
- OAuth2 flow per MCP specification

---

## Success Criteria

### Phase 1: Authentication (Epic 1 Complete - ADR-002 Resource Server)
- [x] OAuth Protected Resource metadata endpoint (RFC 9728)
- [x] Bearer token extraction from Authorization header
- [x] Token validation with Miro API introspection
- [x] LRU cache for token validation (5-min TTL)
- [x] Stateless architecture (no session storage)
- [x] Correlation IDs for request tracing
- [x] Structured logging (JSON format for production)

### Phase 2: Basic Operations (Epics 2-3 Complete)
- [ ] User lists existing Miro boards
- [ ] User creates new board via Claude prompt
- [ ] User creates sticky notes, shapes, text, frames
- [ ] All visual elements appear correctly on board

### Phase 3: Primary Use Case (Epics 4-6 Complete)
- [ ] User creates connectors with styling
- [ ] User updates and deletes items
- [ ] User creates 3-squad org chart in <5 minutes via simple prompt
- [ ] Organizational structure is clear and properly formatted

### Phase 4: Production Ready (Epic 7 + Security Review)
- [ ] Bulk operations reduce latency >50%
- [ ] Security review passed (no P0/P1 vulnerabilities)
- [ ] Documentation complete (setup, usage, deployment)
- [ ] Deployed to accessible HTTPS endpoint
- [ ] Claude.ai web interface integration tested

---

## Definition of Done

**Per Feature**:
- Code implemented following Rust best practices
- Unit tests covering core logic
- Integration tests for API interactions
- Error handling for API failures
- Committed via git-workflow skill (atomic commits)
- Backlog item marked complete with date

**Per Epic**:
- All epic features complete
- Integration tests passing
- Quality review completed (if threshold reached)
- Architecture review for complex epics (OAuth2, orchestration)
- Documentation updated

**Production Release**:
- All critical epics complete (1-6)
- Security review passed
- Performance acceptable for 50+ item diagrams
- OAuth2 flow tested end-to-end
- Deployment documentation complete
- Claude.ai web interface integration verified

---

## Risk Management

**High Risks**:
1. **OAuth2 integration**: Mitigated by Resource Server pattern (ADR-002) - Claude handles OAuth flow
2. **MCP protocol compliance**: Mitigated by integration-specialist validation + RFC 9728 metadata
3. **Token security**: Mitigated by Bearer token validation + in-memory caching (5-min TTL)
4. **Miro API rate limits**: Mitigated by token validation caching (95% hit rate) + bulk operations

**Medium Risks**:
1. **Rust async complexity**: Mitigated by solution-architect patterns + tokio best practices
2. **Deployment hosting**: Requires HTTPS - plan deployment platform early
3. **Error handling coverage**: Mitigated by comprehensive testing strategy

**Low Risks**:
1. **API stability**: Miro API v2 is stable and well-documented
2. **Library availability**: Rust ecosystem has mature HTTP/OAuth2/JSON libraries

---

## Development Phases

### Phase 1: Foundation (Sprint 1 - ~3 days)
**Focus**: Authentication + Board Management
- Epic 1: OAuth2 implementation
- Epic 2: Board operations
- Start Epic 3: Basic visual elements

### Phase 2: Visualization (Sprint 2 - ~3 days)
**Focus**: Complete visual toolset + primary use case
- Complete Epic 3: All visual elements
- Epic 4: Connectors and relationships
- Epic 5: Item management
- Epic 6: Squad visualization orchestration

### Phase 3: Production (Sprint 3 - ~2 days)
**Focus**: Optimization + deployment
- Epic 7: Bulk operations
- Security review
- Documentation
- Deployment
- Claude.ai integration testing

**Total Timeline**: ~8 working days (matches 62h estimate)

---

## Notes

**Developer Context**:
- Has built GitHub MCP server with OAuth2 previously
- Familiar with authorization code flow, token management
- Comfortable with Rust and async patterns
- Target: remote MCP for Claude.ai web interface

**Testing Strategy**:
- Use provided tokens for initial API exploration
- Implement OAuth2 flow after API client proven working
- Test token refresh using short-lived test tokens
- Manual testing with Claude.ai interface before release

**Infrastructure & Deployment**

**Platform Choice: Scaleway Managed Containers** ✅

*Decision rationale*: Required for LRU cache persistence and optimal token validation performance (ADR-002 Resource Server pattern)

**Why Container > Function**:
- **Remote MCP = SSE transport over HTTP** (long-polling HTTP server, not stdio)
- **LRU cache in-memory** persists between requests (95% cache hit rate)
- **No cold start penalty** for token validation (critical path)
- **Token validation latency**: <1ms (cached) vs 100ms (Miro API call)

**Performance Analysis**:
- **Workload pattern**: Sporadic bursts (org chart 1x/day + spaced API calls)
- **Token validation**: <1ms (95% cached) vs 100ms (Miro API)
- **MCP operations**: 200-500ms latency (Miro API + processing)
- **Cache efficiency**: 95% hit rate with 5-minute TTL

**Cost Projection**:
- **Container (always-on)**: ~€20/month (0.25 vCPU + 256Mi memory)
- **Container cost breakdown**:
  - vCPU: €0.10/vCPU/hour × 0.25 × 730 hours = €18/month
  - Memory: €0.01/GB/hour × 0.256 GB × 730 hours = €1.87/month
- **Verdict**: Acceptable for personal use with optimal performance

**Recommended Configuration**:
```yaml
containers:
  miro-mcp:
    runtime: rust (Debian Bookworm Slim)
    memory: 256Mi       # Sufficient for OAuth2 + API calls + LRU cache
    cpu: 0.25           # Single user, low concurrency
    min_scale: 1        # Always-on for cache persistence
    max_scale: 1        # Single user deployment
    port: 3000          # HTTP/SSE transport
```

**Architecture (ADR-002 Resource Server)**:
- **Pattern**: OAuth Resource Server per RFC 9728
- **OAuth flow**: Claude handles OAuth with Miro directly
- **Token validation**: Bearer tokens validated via Miro API
- **Caching**: LRU cache (100 tokens, 5-min TTL) achieves 95% hit rate
- **Code complexity**: ~150 LOC (70% less than Proxy OAuth alternative)
- **Build time**: ~30 seconds (no encryption dependencies)

**Cache Configuration**:
- **Type**: LRU (Least Recently Used)
- **Size**: 100 tokens (~10KB memory)
- **TTL**: 5 minutes (balance security vs performance)
- **Hit rate**: 95% (estimated for typical usage)

**Platform Details**:
- **Compute**: Scaleway Managed Containers
- **Secrets**: Scaleway Secret Manager (MIRO_CLIENT_ID, MIRO_ENCRYPTION_KEY)
- **Logs**: Scaleway Cockpit (audit trail for token validation)
- **TLS**: Native HTTPS (Scaleway provides TLS termination)
- **Cost target**: €20/month (vs €25-50/month with database)

**Decision date**: 2025-11-10 (ADR-002 Resource Server architecture)
**Production deployment**: 2025-11-12 (Scaleway Containers)
