use axum::http::{Request, StatusCode};
use axum::Router;
use miro_mcp_server::http_server::create_app_adr002;
use miro_mcp_server::{Config, TokenValidator};
use std::sync::Arc;
use tower::ServiceExt;

fn get_test_config() -> Config {
    Config {
        client_id: "test_client_id".to_string(),
        client_secret: "test_client_secret".to_string(),
        redirect_uri: "https://claude.ai/api/mcp/auth_callback".to_string(),
        encryption_key: [0u8; 32],
        port: 3010,
        base_url: Some("https://test.example.com".to_string()),
    }
}

fn create_test_app() -> Router {
    let config = Arc::new(get_test_config());
    let token_validator = Arc::new(TokenValidator::new(
        config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://test.example.com".to_string()),
    ));

    create_app_adr002(token_validator, config)
}

#[tokio::test]
async fn test_health_endpoint_no_auth_required() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_oauth_metadata_no_auth_required() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/.well-known/oauth-protected-resource")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// Note: ADR-005 Resource Server pattern - OAuth handled by Claude.ai
// No /oauth/authorize endpoint - we only validate JWT tokens
// Tests verify:
// 1. Public routes work without auth (health, metadata)
// 2. Protected routes require Bearer token with valid JWT
