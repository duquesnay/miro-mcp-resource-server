# Historical Architecture Decisions

This directory contains **historical/deprecated** Architecture Decision Records (ADRs) that were considered but not implemented, or superseded by later decisions.

## Archive Contents

### ADR-001: OAuth2 Stateless Architecture (Proxy OAuth - Early Concept)
**Status**: Not implemented
**Date**: Early exploration
**Why archived**: This described a Proxy OAuth pattern where our server would manage the full OAuth flow. This pattern was initially explored but ultimately not selected for this project.

### ADR-003: Dual-Mode Architecture
**Status**: Not implemented
**Date**: 2025-11-10
**Why archived**: Proposed supporting both stdio and HTTP modes. We chose to focus on HTTP-only deployment with Resource Server pattern.

### ADR-004: Proxy OAuth for Claude.ai Web
**Status**: Considered but not implemented
**Date**: 2025-11-11
**Why archived**: This described implementing OAuth proxy endpoints (`/authorize`, `/callback`, `/token`) where our server would handle the OAuth flow between Claude.ai and Miro. After analysis, we determined the **Resource Server pattern** (ADR-002/ADR-005) was simpler and sufficient for our needs.

## Current Architecture

**See**: `../ADR-002-oauth-resource-server-architecture.md` (or ADR-005 if renamed)

**Pattern**: OAuth Resource Server
**Implementation**: Claude handles OAuth with Miro, our server validates Bearer tokens
**Status**: Production implementation (current)

## Why These Were Not Implemented

1. **Complexity vs Value**: Proxy OAuth (ADR-001, ADR-004) adds significant complexity (~500 LOC vs ~150 LOC) without clear benefits for our use case
2. **Claude Platform Capabilities**: Claude.ai can handle OAuth flow directly - we don't need to proxy it
3. **Simpler is Better**: Resource Server pattern is architecturally simpler and follows RFC 9728 standards
4. **Deployment**: Resource Server pattern works with stateless container deployment (no session storage needed)

## Historical Context: Fork Evolution

This repository (`miro-mcp-resource-server`) is a **GitHub fork** of the main project that initially explored Proxy OAuth. The fork demonstrates:

- **Simpler is better**: Resource Server pattern is 70% less code than Proxy OAuth
- **Standards compliance**: RFC 9728 Protected Resource Metadata
- **Production deployment**: Successfully deployed to Scaleway Containers

The main project may have implemented Proxy OAuth (ADR-004), but this fork chose the Resource Server path.

---

**Last Updated**: 2025-11-12
**Maintained For**: Historical reference and architectural learning
