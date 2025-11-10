use axum::http::{Request, StatusCode};
use axum::Router;
use miro_mcp_server::{
    create_app, Config, CookieStateManager, CookieTokenManager, MiroOAuthClient, TokenValidator,
};
use std::sync::Arc;
use tower::ServiceExt;

fn get_test_config() -> Config {
    Config {
        client_id: "test_client_id".to_string(),
        client_secret: "test_client_secret".to_string(),
        redirect_uri: "http://localhost:3010/oauth/callback".to_string(),
        encryption_key: [0u8; 32],
        port: 3010,
    }
}

fn create_test_app() -> Router {
    let config = get_test_config();
    let oauth_client = Arc::new(MiroOAuthClient::new(&config).unwrap());
    let cookie_state_manager = CookieStateManager::from_config(config.encryption_key);
    let cookie_token_manager = CookieTokenManager::from_config(config.encryption_key);
    let token_validator = Arc::new(TokenValidator::new());

    create_app(
        oauth_client,
        cookie_state_manager,
        cookie_token_manager,
        token_validator,
    )
}

#[tokio::test]
async fn test_list_boards_requires_auth() {
    let app = create_test_app();

    // Request without Bearer token should be rejected with 401
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/list_boards")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "list_boards should require Bearer token"
    );
}

#[tokio::test]
async fn test_list_boards_with_invalid_token() {
    let app = create_test_app();

    // Request with invalid Bearer token should be rejected with 401
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/list_boards")
                .header("Authorization", "Bearer invalid_token_xyz")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Invalid Bearer token should be rejected"
    );
}

#[tokio::test]
async fn test_get_board_requires_auth() {
    let app = create_test_app();

    // Request without Bearer token should be rejected with 401
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/get_board/board-123")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "get_board should require Bearer token"
    );
}

#[tokio::test]
async fn test_get_board_with_invalid_token() {
    let app = create_test_app();

    // Request with invalid Bearer token should be rejected with 401
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/get_board/board-123")
                .header("Authorization", "Bearer invalid_token_xyz")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Invalid Bearer token should be rejected"
    );
}

#[tokio::test]
async fn test_mcp_tools_endpoints_exist() {
    let app = create_test_app();

    // Verify list_boards endpoint exists (will fail auth but not 404)
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/list_boards")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be 401 (auth required), not 404 (endpoint doesn't exist)
    assert_ne!(
        response1.status(),
        StatusCode::NOT_FOUND,
        "list_boards endpoint should exist"
    );

    // Verify get_board endpoint exists
    let response2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp/get_board/test-board")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be 401 (auth required), not 404 (endpoint doesn't exist)
    assert_ne!(
        response2.status(),
        StatusCode::NOT_FOUND,
        "get_board endpoint should exist"
    );
}
