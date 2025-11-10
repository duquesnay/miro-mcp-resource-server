use crate::auth::{
    extract_bearer_token, CookieStateManager, CookieTokenManager, MiroOAuthClient,
    OAuthCookieState, OAuthTokenCookie, TokenValidator,
};
use crate::mcp::oauth_metadata;
use axum::{
    extract::{Query, State},
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use oauth2::PkceCodeVerifier;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info, warn};

/// OAuth callback query parameters
#[derive(Debug, Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    oauth_client: Arc<MiroOAuthClient>,
    cookie_state_manager: CookieStateManager,
    cookie_token_manager: CookieTokenManager,
    token_validator: Arc<TokenValidator>,
}

/// Handle OAuth callback from Miro
async fn oauth_callback(
    Query(params): Query<OAuthCallback>,
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    info!("Received OAuth callback with code");

    match handle_oauth_exchange(params, state, headers).await {
        Ok(token_cookie) => {
            let mut response = Html(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Authorization Successful</title>
                <style>
                    body {
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                        margin: 0;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    }
                    .container {
                        background: white;
                        padding: 3rem;
                        border-radius: 12px;
                        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                        text-align: center;
                        max-width: 500px;
                    }
                    h1 { color: #2d3748; margin-bottom: 1rem; }
                    p { color: #4a5568; line-height: 1.6; }
                    .success { color: #48bb78; font-size: 3rem; margin-bottom: 1rem; }
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="success">✓</div>
                    <h1>Authorization Successful!</h1>
                    <p>Your Miro account has been connected.</p>
                    <p>You can now close this window and return to Claude.</p>
                </div>
            </body>
            </html>
            "#
            )
            .into_response();

            // Set token cookie
            let cookie_header = format!("{}={}", token_cookie.name(), token_cookie.value());
            response.headers_mut().insert(
                header::SET_COOKIE,
                cookie_header.parse().unwrap(),
            );

            response
        }
        Err(e) => {
            error!("OAuth exchange failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title>Authorization Failed</title>
                        <style>
                            body {{
                                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                                display: flex;
                                justify-content: center;
                                align-items: center;
                                height: 100vh;
                                margin: 0;
                                background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
                            }}
                            .container {{
                                background: white;
                                padding: 3rem;
                                border-radius: 12px;
                                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                                text-align: center;
                                max-width: 500px;
                            }}
                            h1 {{ color: #2d3748; margin-bottom: 1rem; }}
                            p {{ color: #4a5568; line-height: 1.6; }}
                            .error {{ color: #f56565; font-size: 3rem; margin-bottom: 1rem; }}
                            code {{ background: #f7fafc; padding: 0.2rem 0.4rem; border-radius: 3px; }}
                        </style>
                    </head>
                    <body>
                        <div class="container">
                            <div class="error">✗</div>
                            <h1>Authorization Failed</h1>
                            <p>Error: <code>{}</code></p>
                            <p>Please try again or contact support.</p>
                        </div>
                    </body>
                    </html>
                    "#,
                    e
                )),
            )
                .into_response()
        }
    }
}

/// Exchange authorization code for access token
async fn handle_oauth_exchange(
    params: OAuthCallback,
    app_state: AppState,
    headers: axum::http::HeaderMap,
) -> Result<cookie::Cookie<'static>, Box<dyn std::error::Error>> {
    // Extract cookie from request headers
    let cookie_value = headers
        .get(header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            // Parse cookies and find miro_oauth_state
            cookies
                .split(';')
                .map(|c| c.trim())
                .find(|c| c.starts_with("miro_oauth_state="))
                .map(|c| c.strip_prefix("miro_oauth_state=").unwrap().to_string())
        })
        .ok_or("OAuth state cookie not found")?;

    // Retrieve and validate OAuth state from cookie
    let oauth_state = app_state
        .cookie_state_manager
        .retrieve_and_validate(&cookie_value, &params.state)
        .map_err(|e| format!("Cookie validation failed: {}", e))?;

    // Extract PKCE verifier
    let pkce_verifier = PkceCodeVerifier::new(oauth_state.pkce_verifier);

    // Exchange code for tokens
    let tokens = app_state
        .oauth_client
        .exchange_code(params.code, pkce_verifier)
        .await?;

    // Calculate expires_in from expires_at
    let expires_in = tokens.expires_in() as u64;

    // Create token cookie
    let token_cookie_data = OAuthTokenCookie::new(
        tokens.access_token.clone(),
        tokens.refresh_token.clone().unwrap_or_default(),
        expires_in,
    );

    let token_cookie = app_state
        .cookie_token_manager
        .create_cookie(token_cookie_data)?;

    info!("OAuth tokens stored in encrypted cookie");
    Ok(token_cookie)
}

/// Initiate OAuth flow - creates cookie and redirects to Miro
async fn oauth_authorize(State(state): State<AppState>) -> Response {
    match handle_oauth_authorize(state).await {
        Ok(response) => response,
        Err(e) => {
            error!("OAuth authorization failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Authorization failed: {}", e),
            )
                .into_response()
        }
    }
}

/// Generate authorization URL, create cookie, and redirect
async fn handle_oauth_authorize(
    app_state: AppState,
) -> Result<Response, Box<dyn std::error::Error>> {
    // Generate authorization URL with CSRF and PKCE
    let (auth_url, csrf_token, pkce_verifier) = app_state
        .oauth_client
        .get_authorization_url()
        .map_err(|e| format!("Failed to generate auth URL: {}", e))?;

    // Create OAuth state for cookie
    let oauth_state = OAuthCookieState::new(csrf_token, pkce_verifier);

    // Create encrypted cookie
    let cookie = app_state
        .cookie_state_manager
        .create_cookie(oauth_state)
        .map_err(|e| format!("Failed to create cookie: {}", e))?;

    // Build redirect response with cookie
    let response = axum::response::Redirect::to(&auth_url);
    let mut response = response.into_response();

    // Set cookie header
    let cookie_header = format!("{}={}", cookie.name(), cookie.value());
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie_header.parse().unwrap(),
    );

    info!("Redirecting to Miro authorization URL with encrypted state cookie");
    Ok(response)
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Bearer token validation middleware
///
/// Validates Bearer tokens on all protected routes.
/// Extracts token from Authorization header, validates with TokenValidator,
/// and returns 401 if missing or invalid.
async fn bearer_auth_middleware(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Bearer token from Authorization header
    let token = match extract_bearer_token(request.headers()) {
        Ok(token) => token,
        Err(e) => {
            warn!("Bearer token extraction failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Validate token with Miro API (with caching)
    let user_info = match state.token_validator.validate_token(&token).await {
        Ok(user_info) => user_info,
        Err(e) => {
            warn!("Token validation failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Store both token and user_info in request extensions for handlers to access
    request.extensions_mut().insert(std::sync::Arc::new(token));
    request.extensions_mut().insert(std::sync::Arc::new(user_info));

    // Continue to handler
    Ok(next.run(request).await)
}

/// Create and configure the HTTP server
pub fn create_app(
    oauth_client: Arc<MiroOAuthClient>,
    cookie_state_manager: CookieStateManager,
    cookie_token_manager: CookieTokenManager,
    token_validator: Arc<TokenValidator>,
) -> Router {
    let state = AppState {
        oauth_client,
        cookie_state_manager,
        cookie_token_manager,
        token_validator,
    };

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/.well-known/oauth-protected-resource", get(oauth_metadata))
        .route("/oauth/authorize", get(oauth_authorize))
        .route("/oauth/callback", get(oauth_callback));

    // Protected routes (require Bearer token validation)
    let protected_routes = Router::new()
        .route("/mcp/list_boards", axum::routing::post(crate::mcp::tools::list_boards))
        .route("/mcp/get_board/:board_id", axum::routing::post(crate::mcp::tools::get_board))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            bearer_auth_middleware,
        ));

    // Combine routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}

/// Run the HTTP server
pub async fn run_server(
    port: u16,
    oauth_client: Arc<MiroOAuthClient>,
    cookie_state_manager: CookieStateManager,
    cookie_token_manager: CookieTokenManager,
    token_validator: Arc<TokenValidator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(
        oauth_client,
        cookie_state_manager,
        cookie_token_manager,
        token_validator,
    );
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("OAuth HTTP server listening on {}", addr);
    info!("OAuth callback URL: http://127.0.0.1:{}/oauth/callback", port);

    axum::serve(listener, app).await?;
    Ok(())
}

//
// ============================================================================
// ADR-002 Resource Server Implementation (OAuth client removed)
// ============================================================================
//

/// Simplified application state for ADR-002 Resource Server
/// No OAuth client, no cookie managers - only token validation
#[derive(Clone)]
pub struct AppStateADR002 {
    token_validator: Arc<TokenValidator>,
}

/// Bearer token validation middleware for ADR-002
/// Simplified version without OAuth client dependencies
async fn bearer_auth_middleware_adr002(
    State(state): State<AppStateADR002>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Bearer token from Authorization header
    let token = match extract_bearer_token(request.headers()) {
        Ok(token) => token,
        Err(e) => {
            warn!("Bearer token extraction failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Validate token with Miro API (with caching)
    let user_info = match state.token_validator.validate_token(&token).await {
        Ok(user_info) => user_info,
        Err(e) => {
            warn!("Token validation failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    info!("Request authenticated for user: {}", user_info.user_id);

    // Store both token and user_info in request extensions for handlers
    request.extensions_mut().insert(Arc::new(token));
    request.extensions_mut().insert(Arc::new(user_info));

    Ok(next.run(request).await)
}

/// Create HTTP server for ADR-002 Resource Server pattern
/// Only includes:
/// - OAuth metadata endpoint (AUTH6)
/// - Bearer token authentication (AUTH7+AUTH8+AUTH9)
/// - MCP tools (list_boards, get_board)
pub fn create_app_adr002(token_validator: Arc<TokenValidator>) -> Router {
    let state = AppStateADR002 { token_validator };

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/.well-known/oauth-protected-resource", get(oauth_metadata));

    // Protected routes (Bearer token required)
    let protected_routes = Router::new()
        .route("/mcp/list_boards", axum::routing::post(crate::mcp::tools::list_boards))
        .route("/mcp/get_board/:board_id", axum::routing::post(crate::mcp::tools::get_board))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            bearer_auth_middleware_adr002,
        ));

    // Merge routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}

/// Run HTTP server with ADR-002 Resource Server pattern
/// No OAuth client code - only Bearer token validation
pub async fn run_server_adr002(
    port: u16,
    token_validator: Arc<TokenValidator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app_adr002(token_validator);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("ADR-002 Resource Server listening on {}", addr);
    info!("OAuth metadata endpoint: http://{}/.well-known/oauth-protected-resource", addr);
    info!("Protected endpoints require Bearer token validation");

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
    use crate::config::Config;

    fn get_test_config() -> Config {
        Config {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            redirect_uri: "http://localhost:3010/oauth/callback".to_string(),
            encryption_key: [0u8; 32],
            port: 3010,
        }
    }

    #[test]
    fn test_create_app() {
        let config = get_test_config();
        let oauth_client = Arc::new(MiroOAuthClient::new(&config).unwrap());
        let cookie_state_manager = CookieStateManager::from_config(config.encryption_key);
        let cookie_token_manager = CookieTokenManager::from_config(config.encryption_key);
        let token_validator = Arc::new(TokenValidator::new());

        let app = create_app(
            oauth_client,
            cookie_state_manager,
            cookie_token_manager,
            token_validator,
        );
        assert!(std::mem::size_of_val(&app) > 0);
    }
}
