use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::auth::ProtectedResourceMetadata;
use crate::config::Config;

/// Handle Protected Resource Metadata endpoint (ADR-005 Resource Server pattern)
/// Returns metadata pointing to Miro as the authorization server
/// This tells Claude.ai to use Miro directly for OAuth, and send us JWT tokens
///
/// Per RFC 9728 - OAuth 2.0 Protected Resource Metadata
///
/// # Arguments
/// * `config` - Server configuration containing BASE_URL
///
/// # ADR-005 Pattern
/// - authorization_servers: Points to Miro (not our server)
/// - Claude handles OAuth flow with Miro directly
/// - We validate JWT tokens from Claude
pub async fn protected_resource_metadata(
    State(config): State<Arc<Config>>,
) -> impl IntoResponse {
    let base_url = config
        .base_url
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("http://localhost:3000");

    let metadata = ProtectedResourceMetadata::new_for_miro(base_url.to_string());

    Json(metadata)
}

// Keep oauth_metadata as alias for backward compatibility during transition
pub use protected_resource_metadata as oauth_metadata;
