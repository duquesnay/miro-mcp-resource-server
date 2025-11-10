# Bearer Token Authentication - Usage Guide

## Overview

The HTTP server now validates Bearer tokens on all protected routes using middleware. This guide shows how to add new MCP endpoints that leverage this authentication.

## Architecture

```
Client Request
    ↓
Authorization: Bearer <token>
    ↓
bearer_auth_middleware
    ├─ Extract token
    ├─ Validate with Miro API (cached)
    ├─ Store UserInfo in extensions
    └─ Pass to handler
        ↓
MCP Handler (receives UserInfo)
```

## Adding Protected Endpoints

### Step 1: Create Handler

Your handler receives `UserInfo` via Axum's `Extension` extractor:

```rust
use axum::{Extension, Json};
use miro_mcp_server::UserInfo;
use serde_json::Value;

async fn list_boards_handler(
    Extension(user_info): Extension<UserInfo>,
    // other extractors...
) -> Result<Json<Value>, StatusCode> {
    // user_info contains:
    // - user_id: String
    // - team_id: String
    // - scopes: Vec<String>

    info!("Listing boards for user: {}", user_info.user_id);

    // Use user_info to make Miro API calls
    // ...

    Ok(Json(json!({ "boards": [] })))
}
```

### Step 2: Add Route to Protected Router

In `src/http_server.rs`:

```rust
// Protected routes (require Bearer token validation)
let protected_routes = Router::new()
    .route("/mcp/boards/list", post(list_boards_handler))
    .route("/mcp/items/create", post(create_item_handler))
    .route("/mcp/items/update", post(update_item_handler))
    // ... more routes
    .layer(middleware::from_fn_with_state(
        state.clone(),
        bearer_auth_middleware,
    ));
```

**Important:** All routes in `protected_routes` automatically get Bearer token validation.

## Client Usage

### Valid Request

```bash
curl -X POST http://localhost:3010/mcp/boards/list \
  -H "Authorization: Bearer <valid_token>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response:** `200 OK` with board data

### Missing Token

```bash
curl -X POST http://localhost:3010/mcp/boards/list \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response:** `401 Unauthorized`

### Invalid Token

```bash
curl -X POST http://localhost:3010/mcp/boards/list \
  -H "Authorization: Bearer invalid_token" \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response:** `401 Unauthorized`

## UserInfo Structure

```rust
pub struct UserInfo {
    /// Miro user ID (e.g., "3458764628469116734")
    pub user_id: String,

    /// Miro team ID (e.g., "3458764628469116735")
    pub team_id: String,

    /// Granted scopes (e.g., ["boards:read", "boards:write"])
    pub scopes: Vec<String>,
}
```

### Checking Scopes

```rust
async fn create_board_handler(
    Extension(user_info): Extension<UserInfo>,
) -> Result<Json<Value>, StatusCode> {
    // Check if user has required scope
    if !user_info.scopes.contains(&"boards:write".to_string()) {
        warn!("User {} lacks boards:write scope", user_info.user_id);
        return Err(StatusCode::FORBIDDEN);
    }

    // Proceed with creation
    // ...
}
```

## Validation Flow

### 1. Cache Hit (Fast Path)

```
Request → extract_bearer_token() → TokenValidator.validate_token()
                                          ↓
                                    Check LRU cache
                                          ↓
                                    Cache hit (5 min TTL)
                                          ↓
                                    Return UserInfo
                                          ↓
                                    Handler (<1ms)
```

### 2. Cache Miss (Slow Path)

```
Request → extract_bearer_token() → TokenValidator.validate_token()
                                          ↓
                                    Cache miss or expired
                                          ↓
                                    Call Miro API (100-300ms)
                                          ↓
                                    Store in cache
                                          ↓
                                    Return UserInfo
                                          ↓
                                    Handler
```

### 3. Invalid Token

```
Request → extract_bearer_token() → TokenValidator.validate_token()
                                          ↓
                                    Call Miro API
                                          ↓
                                    401 Unauthorized
                                          ↓
                                    Return 401 to client
                                    (Handler never called)
```

## Error Handling

### Middleware Errors (Automatic 401)

The middleware handles these automatically:

1. **Missing Authorization header**
   - Error: `AuthError::NoToken`
   - Response: `401 Unauthorized`

2. **Invalid format** (e.g., "Basic xyz" instead of "Bearer xyz")
   - Error: `AuthError::InvalidTokenFormat`
   - Response: `401 Unauthorized`

3. **Token validation failed** (Miro API returns 401)
   - Error: `AuthError::TokenInvalid`
   - Response: `401 Unauthorized`

4. **Miro API error** (network, timeout, etc.)
   - Error: `AuthError::TokenValidationFailed`
   - Response: `401 Unauthorized`

### Handler Errors (Custom Logic)

```rust
async fn handler(
    Extension(user_info): Extension<UserInfo>,
) -> Result<Json<Value>, StatusCode> {
    // Authorization checks
    if !has_permission(&user_info) {
        return Err(StatusCode::FORBIDDEN); // 403
    }

    // Business logic errors
    match perform_operation().await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Operation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR) // 500
        }
    }
}
```

## Public vs Protected Routes

### Public Routes (No Auth Required)

```rust
let public_routes = Router::new()
    .route("/health", get(health_check))
    .route("/.well-known/oauth-protected-resource", get(oauth_metadata))
    .route("/oauth/authorize", get(oauth_authorize))
    .route("/oauth/callback", get(oauth_callback));
```

**Use case:**
- Health checks
- OAuth flow endpoints
- Public metadata

### Protected Routes (Bearer Token Required)

```rust
let protected_routes = Router::new()
    .route("/mcp/boards/list", post(list_boards_handler))
    // All routes here require Bearer tokens
    .layer(middleware::from_fn_with_state(
        state.clone(),
        bearer_auth_middleware,
    ));
```

**Use case:**
- MCP tool endpoints
- User-specific operations
- Miro API proxying

## Performance Characteristics

### Token Validation

| Scenario | Latency | Description |
|----------|---------|-------------|
| Cache hit | <1ms | Token found in LRU cache, not expired |
| Cache miss | 100-300ms | Call Miro API, cache result |
| Invalid token | 100-300ms | Call Miro API, receive 401 |

### Cache Configuration

- **Capacity:** 100 tokens (LRU eviction)
- **TTL:** 5 minutes per token
- **Thread-safe:** `Mutex<LruCache>`
- **Memory:** ~10-20 KB per cached entry

### Optimization Tips

1. **Reuse tokens:** Clients should reuse the same token across requests to benefit from caching
2. **Token lifetime:** Miro tokens typically valid for 1 hour, cache TTL is 5 minutes
3. **Concurrent requests:** Cache is thread-safe, multiple requests can share cache
4. **Capacity planning:** 100 token capacity supports ~100 concurrent users

## Testing

### Unit Tests (Middleware)

```rust
#[tokio::test]
async fn test_protected_route_requires_auth() {
    let app = create_test_app();

    // Request without Authorization header
    let response = app
        .oneshot(
            Request::builder()
                .uri("/mcp/boards/list")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
```

### Integration Tests (With Mock Miro API)

```rust
#[tokio::test]
async fn test_valid_token_passes_to_handler() {
    let mock_server = MockServer::start().await;

    // Mock Miro token endpoint
    Mock::given(method("GET"))
        .and(path("/v1/oauth-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "user_id": "user123",
            "team_id": "team456",
            "scopes": "boards:read boards:write"
        })))
        .mount(&mock_server)
        .await;

    let app = create_app_with_validator(
        TokenValidator::new_with_endpoint(mock_server.uri())
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/mcp/boards/list")
                .header("Authorization", "Bearer valid_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

## Troubleshooting

### "401 Unauthorized" on Valid Token

**Check:**
1. Token format: `Authorization: Bearer <token>` (case-sensitive)
2. Token not expired (Miro access tokens expire after 1 hour)
3. Miro API reachable (check network logs)
4. Token has required scopes

**Debug:**
```bash
RUST_LOG=miro_mcp_server=debug cargo run
# Watch for: "Token validation failed: ..."
```

### "Cache not working"

**Check:**
1. Same token used across requests (cache key is token)
2. Token not expired in cache (5 min TTL)
3. Cache capacity not exceeded (100 tokens)

**Monitor:**
```rust
let (used, capacity) = token_validator.cache_stats();
println!("Cache: {}/{} entries", used, capacity);
```

### "Performance issues"

**Symptoms:**
- High latency on protected endpoints
- Miro API rate limits

**Solutions:**
1. Check cache hit rate (should be >90% for typical usage)
2. Increase cache capacity if many unique users
3. Increase cache TTL if token churn is high
4. Implement request coalescing for concurrent requests

## Best Practices

### 1. Always Use Extension Extractor

```rust
// ✅ Good
async fn handler(Extension(user_info): Extension<UserInfo>) { }

// ❌ Bad - manually parsing extensions
async fn handler(req: Request) {
    let user_info = req.extensions().get::<UserInfo>().unwrap();
}
```

### 2. Check Scopes Early

```rust
async fn handler(Extension(user_info): Extension<UserInfo>) {
    // ✅ Good - fail fast
    if !has_required_scope(&user_info) {
        return Err(StatusCode::FORBIDDEN);
    }

    // Expensive operations here
}
```

### 3. Log Security Events

```rust
use tracing::{info, warn};

async fn handler(Extension(user_info): Extension<UserInfo>) {
    info!("User {} accessing protected resource", user_info.user_id);

    if !authorized {
        warn!("Unauthorized access attempt by user {}", user_info.user_id);
    }
}
```

### 4. Don't Log Tokens

```rust
// ✅ Good
warn!("Token validation failed for user {}", user_info.user_id);

// ❌ Bad - exposes token
warn!("Token validation failed: {}", token);
```

## Security Considerations

### What's Validated

✅ Token is well-formed (Bearer <token>)
✅ Token is valid with Miro API
✅ Token has user/team context
✅ Token scopes are available

### What's NOT Validated (Yet)

⚠️ Specific scope requirements per endpoint
⚠️ Rate limiting per user
⚠️ Token revocation checking

**Note:** Scope checking should be implemented in individual handlers based on operation requirements.

## Migration Guide

### From Cookie-Based Auth to Bearer Token

**Old approach (being phased out):**
```rust
// Read access token from cookie
let token = cookie_manager.get_token(&cookies)?;
```

**New approach:**
```rust
// Middleware automatically validates Bearer token
async fn handler(Extension(user_info): Extension<UserInfo>) {
    // Token already validated, user_info available
}
```

**Benefits:**
- Stateless (no cookie management)
- Standard OAuth2 flow
- Works with Claude.ai remote MCP
- Better security (no cookie CSRF risks)

---

**Implementation Status:** ✅ Complete (AUTH10)
**Next Steps:** Add MCP tool handlers (AUTH11)
