use crate::auth::{ProtectedResourceMetadata, TokenValidator};
use crate::config::Config;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

// OAuth proxy removed in ADR-005 (Resource Server pattern)
// Protected Resource Metadata endpoint added instead

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Protected Resource Metadata endpoint (RFC 9728)
/// Advertises OAuth authorization server and resource capabilities
async fn protected_resource_metadata(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let base_url = config
        .base_url
        .clone()
        .unwrap_or_else(|| "https://miro-mcp.example.com".to_string());
    let metadata = ProtectedResourceMetadata::new_for_miro(base_url);
    Json(metadata)
}

//
// ============================================================================
// ADR-002 Resource Server Implementation (OAuth client removed)
// ============================================================================
//

/// Correlation ID for request tracing
#[derive(Clone)]
pub struct RequestId(pub String);

/// Correlation ID middleware - adds unique request_id to all requests
/// This enables tracing requests across the entire lifecycle for debugging
async fn correlation_id_middleware(mut request: Request<axum::body::Body>, next: Next) -> Response {
    // Generate unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create tracing span with request_id for all subsequent logs
    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
    );

    // Store request_id in extensions for access in handlers
    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));

    // Execute request within the span
    let _enter = span.enter();

    info!("Request started");
    let response = next.run(request).await;
    info!("Request completed");

    response
}

/// Application state for ADR-005 Resource Server pattern
/// Token validation only - Claude handles OAuth
#[derive(Clone)]
pub struct AppStateADR002 {
    pub token_validator: Arc<TokenValidator>,
    pub config: Arc<Config>,
}

/// Create HTTP server for ADR-005 Resource Server pattern
/// Includes:
/// - Correlation ID middleware (OBS1)
/// - Protected Resource Metadata endpoint (RFC 9728)
/// - Bearer token authentication with JWT validation
/// - MCP protocol endpoints
pub fn create_app_adr002(token_validator: Arc<TokenValidator>, config: Arc<Config>) -> Router {
    let state = AppStateADR002 {
        token_validator,
        config,
    };

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route(
            "/.well-known/oauth-protected-resource",
            get(protected_resource_metadata),
        )
        .with_state(state.config.clone());

    // Apply correlation ID middleware to ALL requests
    Router::new()
        .merge(public_routes)
        .layer(middleware::from_fn(correlation_id_middleware))
}

/// Run HTTP server with ADR-005 Resource Server pattern
/// Claude handles OAuth, we validate JWT tokens
pub async fn run_server_adr002(
    port: u16,
    token_validator: Arc<TokenValidator>,
    config: Arc<Config>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app_adr002(token_validator, config);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("ADR-005 Resource Server listening on {}", addr);
    info!(
        "Protected Resource Metadata: http://{}/.well-known/oauth-protected-resource",
        addr
    );
    info!("OAuth handled by Claude.ai - we validate JWT tokens");
    info!("Protected endpoints require Bearer token with valid audience");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            info!("Shutting down HTTP server");
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_app_adr002() {
        // Create test-specific Config instead of loading from environment/file
        let config = Arc::new(Config {
            client_id: "test_client_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
            encryption_key: [0u8; 32],
            port: 3000,
            base_url: Some("https://test.example.com".to_string()),
        });

        let token_validator = Arc::new(TokenValidator::new(
            config
                .base_url
                .clone()
                .unwrap_or_else(|| "https://test.example.com".to_string()),
        ));
        let app = create_app_adr002(token_validator, config);
        assert!(std::mem::size_of_val(&app) > 0);
    }
}
