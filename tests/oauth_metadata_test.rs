use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use miro_mcp_server::http_server::create_app_adr002;
use miro_mcp_server::{Config, TokenValidator};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

fn get_test_config() -> Config {
    Config {
        client_id: "test_client_id".to_string(),
        client_secret: "test_client_secret".to_string(),
        redirect_uri: "https://claude.ai/api/mcp/auth_callback".to_string(),
        encryption_key: [0u8; 32],
        port: 3000,
        base_url: Some("https://test.example.com".to_string()),
    }
}

/// Test that metadata endpoint returns RFC 9728 Protected Resource Metadata
#[tokio::test]
async fn test_metadata_endpoint_returns_rfc9728_format() {
    let config = Arc::new(get_test_config());
    let token_validator = Arc::new(TokenValidator::new(
        config.base_url.clone().unwrap_or_else(|| "https://test.example.com".to_string())
    ));
    let app = create_app_adr002(token_validator, config.clone());

    // Make request to metadata endpoint
    let response = app
        .oneshot(
            Request::builder()
                .uri("/.well-known/oauth-protected-resource")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    // Parse response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let metadata: Value = serde_json::from_slice(&body).unwrap();

    // Verify RFC 9728 required fields
    assert!(
        metadata.get("resource").is_some(),
        "Missing 'resource' field (RFC 9728 required)"
    );
    assert!(
        metadata.get("authorization_servers").is_some(),
        "Missing 'authorization_servers' field (RFC 9728 required)"
    );

    // Verify values are correct (ADR-005: Resource Server pattern)
    assert_eq!(
        metadata["resource"].as_str().unwrap(),
        "https://test.example.com",
        "Resource should be our MCP server URL"
    );

    let auth_servers = metadata["authorization_servers"]
        .as_array()
        .expect("authorization_servers should be array");
    assert_eq!(
        auth_servers.len(),
        1,
        "Should have exactly one authorization server"
    );
    assert_eq!(
        auth_servers[0].as_str().unwrap(),
        "https://miro.com",
        "Authorization server should be Miro (not our server)"
    );

    // Verify optional fields are present and correct
    assert!(
        metadata.get("scopes_supported").is_some(),
        "Should include scopes_supported"
    );
    let scopes = metadata["scopes_supported"]
        .as_array()
        .expect("scopes_supported should be array");
    assert!(
        scopes.contains(&Value::String("boards:read".to_string())),
        "Should support boards:read scope"
    );
    assert!(
        scopes.contains(&Value::String("boards:write".to_string())),
        "Should support boards:write scope"
    );

    // Verify introspection endpoint (optional field)
    assert!(
        metadata.get("introspection_endpoint").is_some(),
        "Should include introspection_endpoint for Miro token validation"
    );
}

/// Test that metadata does NOT include RFC 8414 Authorization Server fields
#[tokio::test]
async fn test_metadata_does_not_include_rfc8414_fields() {
    let config = Arc::new(get_test_config());
    let token_validator = Arc::new(TokenValidator::new(
        config.base_url.clone().unwrap_or_else(|| "https://test.example.com".to_string())
    ));
    let app = create_app_adr002(token_validator, config);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/.well-known/oauth-protected-resource")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let metadata: Value = serde_json::from_slice(&body).unwrap();

    // These are RFC 8414 fields that should NOT be in RFC 9728 metadata
    assert!(
        metadata.get("issuer").is_none(),
        "Should NOT include 'issuer' (that's RFC 8414 Authorization Server metadata)"
    );
    assert!(
        metadata.get("authorization_endpoint").is_none(),
        "Should NOT include 'authorization_endpoint' (that's RFC 8414)"
    );
    assert!(
        metadata.get("token_endpoint").is_none(),
        "Should NOT include 'token_endpoint' (that's RFC 8414)"
    );
}

/// Test WWW-Authenticate header is returned for unauthorized requests
#[tokio::test]
async fn test_www_authenticate_header_returned() {
    use axum::http::header::WWW_AUTHENTICATE;

    let config = Arc::new(get_test_config());
    let token_validator = Arc::new(TokenValidator::new(
        config.base_url.clone().unwrap_or_else(|| "https://test.example.com".to_string())
    ));
    let app = create_app_adr002(token_validator, config);

    // Make request without auth token
    let response = app
        .oneshot(
            Request::builder()
                .uri("/mcp")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Check WWW-Authenticate header per RFC 6750
    let www_auth = response
        .headers()
        .get(WWW_AUTHENTICATE)
        .expect("Should have WWW-Authenticate header")
        .to_str()
        .unwrap();

    // Should include Bearer realm per RFC 6750
    assert!(
        www_auth.starts_with("Bearer"),
        "WWW-Authenticate should start with 'Bearer', got: {}",
        www_auth
    );
    assert!(
        www_auth.contains("realm=\"miro-mcp-server\""),
        "WWW-Authenticate should include realm, got: {}",
        www_auth
    );
}
