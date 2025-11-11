//! HTTP server binary for ADR-005 Resource Server pattern
//!
//! This binary starts the HTTP server with:
//! - Protected Resource Metadata endpoint (RFC 9728)
//! - Bearer token authentication middleware with JWT validation
//! - MCP tools (list_boards, get_board)
//!
//! OAuth is handled by Claude.ai - we only validate JWT tokens

use miro_mcp_server::{Config, TokenValidator};
use std::sync::Arc;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing with configurable format
    // LOG_FORMAT=json for production (Scaleway Cockpit)
    // LOG_FORMAT=pretty (or unset) for development
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "miro_mcp_server=info".into());

    match log_format.as_str() {
        "json" => {
            // JSON format for production (structured logs for Scaleway Cockpit)
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        _ => {
            // Pretty format for development (human-readable)
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer())
                .init();
        }
    }

    info!(
        log_format = %log_format,
        "Starting ADR-002 Resource Server + ADR-004 Proxy OAuth (HTTP only)"
    );

    // Load configuration from environment variables
    let config = Arc::new(Config::from_env_or_file()?);
    info!("Configuration loaded from environment");

    // Create token validator for JWT validation (ADR-005)
    let resource_url = config.base_url.clone().unwrap_or_else(|| {
        warn!("base_url not configured - using fallback");
        "https://miro-mcp.example.com".to_string()
    });
    let token_validator = Arc::new(TokenValidator::new(resource_url.clone()));
    info!(resource_url = %resource_url, "Token validator initialized with JWT validation");

    // Get port from environment or use config default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(config.port);

    info!("Starting ADR-005 Resource Server on 0.0.0.0:{}", port);
    info!("OAuth handled by Claude.ai - we validate JWT tokens");

    // Start HTTP server (ADR-005 Resource Server pattern)
    miro_mcp_server::run_server_adr002(port, token_validator, config).await?;

    Ok(())
}
