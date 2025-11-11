use axum::{response::IntoResponse, Json};
use serde::Serialize;

/// OAuth 2.0 Protected Resource Metadata per RFC 9728
/// This tells Claude.ai which resource this server protects and where to get OAuth tokens
#[derive(Serialize, Debug)]
pub struct OAuthProtectedResourceMetadata {
    /// The protected resource identifier (the API this server is protecting access to)
    pub resource: String,
    /// Authorization servers that can issue tokens for this resource
    pub authorization_servers: Vec<String>,
    /// OAuth 2.0 scopes supported by this protected resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    /// Bearer token methods supported (per RFC 6750)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_methods_supported: Option<Vec<String>>,
}

/// Handle OAuth protected resource metadata endpoint
/// Returns RFC 9728 Protected Resource metadata for Miro API
/// This tells Claude.ai:
/// - Which resource we protect (Miro API)
/// - Where to get authorization (Miro OAuth server)
/// - What scopes are needed
pub async fn oauth_metadata() -> impl IntoResponse {
    Json(OAuthProtectedResourceMetadata {
        resource: "https://api.miro.com".to_string(),
        authorization_servers: vec!["https://miro.com/oauth".to_string()],
        scopes_supported: Some(vec![
            "boards:read".to_string(),
            "boards:write".to_string(),
        ]),
        bearer_methods_supported: Some(vec!["header".to_string()]),
    })
}
