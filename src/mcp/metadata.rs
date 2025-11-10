use axum::{response::IntoResponse, Json};
use serde::Serialize;

/// OAuth 2.0 Authorization Server Metadata per RFC 8414
/// This is what Claude.ai expects for OAuth auto-discovery in Resource Server pattern
#[derive(Serialize, Debug)]
pub struct OAuthAuthorizationServerMetadata {
    /// OAuth 2.0 issuer identifier (usually the provider domain)
    pub issuer: String,
    /// OAuth 2.0 authorization endpoint URL
    pub authorization_endpoint: String,
    /// OAuth 2.0 token endpoint URL
    pub token_endpoint: String,
    /// Grant types supported (authorization_code for standard OAuth flow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    /// Response types supported (code for authorization code flow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_types_supported: Option<Vec<String>>,
}

/// Handle OAuth protected resource metadata endpoint
/// Returns OAuth Authorization Server metadata for Miro
/// This tells Claude.ai where to redirect users for OAuth authorization
pub async fn oauth_metadata() -> impl IntoResponse {
    Json(OAuthAuthorizationServerMetadata {
        issuer: "https://miro.com".to_string(),
        authorization_endpoint: "https://miro.com/oauth/authorize".to_string(),
        token_endpoint: "https://api.miro.com/v1/oauth/token".to_string(),
        grant_types_supported: Some(vec!["authorization_code".to_string()]),
        response_types_supported: Some(vec!["code".to_string()]),
    })
}
