use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),

    #[error("Token is invalid or expired")]
    TokenInvalid,

    #[error("Invalid token format")]
    InvalidTokenFormat,

    #[error("Token expired")]
    TokenExpired,

    #[error("No token available")]
    NoToken,

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
