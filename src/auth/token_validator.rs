use crate::auth::types::AuthError;
use jsonwebtoken::{decode, decode_header, DecodingKey, TokenData, Validation};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

/// User information extracted from JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// User ID from token (sub claim)
    pub user_id: String,
    /// Team/tenant ID (if available)
    pub team_id: Option<String>,
    /// Scopes granted to the token
    pub scopes: Vec<String>,
    /// Timestamp when this cache entry was created
    #[serde(skip)]
    cached_at: u64,
}

impl UserInfo {
    /// Create new UserInfo with current timestamp
    pub fn new(user_id: String, team_id: Option<String>, scopes: Vec<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            user_id,
            team_id,
            scopes,
            cached_at: now,
        }
    }

    /// Check if this cache entry is expired (5 minute TTL)
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        const TTL_SECONDS: u64 = 5 * 60; // 5 minutes
        now - self.cached_at > TTL_SECONDS
    }
}

/// JWT Claims for Miro OAuth tokens
///
/// Standard JWT claims plus Miro-specific fields
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (user ID)
    sub: String,
    /// Audience (which resource this token is for)
    #[serde(default)]
    aud: Option<StringOrVec>,
    /// Expiration time (Unix timestamp)
    exp: u64,
    /// Issued at (Unix timestamp)
    #[serde(default)]
    iat: Option<u64>,
    /// Issuer (authorization server)
    #[serde(default)]
    iss: Option<String>,
    /// Scopes (space-separated or array)
    #[serde(default)]
    scope: Option<String>,
    /// Team ID (Miro-specific)
    #[serde(default, rename = "team_id")]
    team_id: Option<String>,
}

/// Helper to handle audience claim that can be string or array
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

impl StringOrVec {
    fn contains(&self, value: &str) -> bool {
        match self {
            StringOrVec::String(s) => s == value,
            StringOrVec::Vec(v) => v.iter().any(|s| s == value),
        }
    }
}

/// Token validator with LRU caching for Resource Server pattern
///
/// Validates JWT tokens from Claude.ai:
/// - Decodes JWT (without signature verification - trusts Claude's validation)
/// - Verifies audience claim matches our server
/// - Verifies token not expired
/// - Caches validation results for performance
///
/// For production, consider adding JWT signature verification using Miro's JWKS.
pub struct TokenValidator {
    /// LRU cache for validated tokens (capacity: 100)
    cache: Mutex<LruCache<String, UserInfo>>,
    /// Our server URL (expected in audience claim)
    resource_url: String,
}

impl TokenValidator {
    /// Create a new token validator
    ///
    /// # Arguments
    ///
    /// * `resource_url` - Our MCP server URL (e.g., "https://miro-mcp.fly-agile.com")
    ///                    Must match the audience claim in JWT
    pub fn new(resource_url: String) -> Self {
        Self {
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
            resource_url,
        }
    }

    /// Validate Bearer token (JWT from Claude.ai)
    ///
    /// # Arguments
    ///
    /// * `token` - JWT access token (without "Bearer " prefix)
    ///
    /// # Returns
    ///
    /// * `Ok(UserInfo)` - Token is valid, returns user info
    /// * `Err(AuthError)` - Token is invalid or expired
    ///
    /// # Performance
    ///
    /// Results are cached for 5 minutes to reduce validation overhead.
    pub async fn validate(&self, token: &str) -> Result<UserInfo, AuthError> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(user_info) = cache.get(token) {
                if !user_info.is_expired() {
                    debug!(
                        user_id = %user_info.user_id,
                        "Token validation cache hit"
                    );
                    return Ok(user_info.clone());
                } else {
                    debug!("Token validation cache expired");
                    cache.pop(token);
                }
            }
        }

        // Validate token
        debug!("Validating JWT token");
        let user_info = self.validate_jwt(token)?;

        // Cache result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(token.to_string(), user_info.clone());
        }

        info!(
            user_id = %user_info.user_id,
            scopes = ?user_info.scopes,
            "Token validated successfully"
        );

        Ok(user_info)
    }

    /// Validate JWT token
    ///
    /// Performs:
    /// 1. JWT decoding (without signature verification for now)
    /// 2. Expiry check
    /// 3. Audience verification
    ///
    /// For production: Add signature verification using Miro's JWKS endpoint
    fn validate_jwt(&self, token: &str) -> Result<UserInfo, AuthError> {
        // Decode JWT header to check algorithm
        let header = decode_header(token).map_err(|e| {
            warn!(error = %e, "Failed to decode JWT header");
            AuthError::InvalidTokenFormat
        })?;

        debug!(algorithm = ?header.alg, "JWT header decoded");

        // For MVP: Decode without signature verification
        // TODO: Fetch Miro's JWKS and verify signature in production
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        validation.validate_exp = true; // Still check expiry

        // Audience validation (if we have it)
        if !self.resource_url.is_empty() {
            validation.set_audience(&[&self.resource_url]);
        }

        let token_data: TokenData<Claims> =
            decode(token, &DecodingKey::from_secret(&[]), &validation).map_err(|e| {
                warn!(error = %e, "JWT validation failed");
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                        AuthError::TokenValidationFailed(format!(
                            "Invalid audience - expected {}",
                            self.resource_url
                        ))
                    }
                    _ => AuthError::TokenInvalid,
                }
            })?;

        let claims = token_data.claims;

        // Verify expiry manually (double-check)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if claims.exp <= now {
            warn!(
                expiry = claims.exp,
                now = now,
                "Token expired (manual check)"
            );
            return Err(AuthError::TokenExpired);
        }

        // Verify audience manually if not validated automatically
        if let Some(aud) = &claims.aud {
            if !aud.contains(&self.resource_url) {
                warn!(
                    expected = %self.resource_url,
                    "Token audience mismatch"
                );
                return Err(AuthError::TokenValidationFailed(format!(
                    "Token audience does not include {}",
                    self.resource_url
                )));
            }
        }

        // Extract scopes
        let scopes = claims
            .scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default();

        debug!(
            user_id = %claims.sub,
            team_id = ?claims.team_id,
            scopes = ?scopes,
            expiry = claims.exp,
            "JWT claims extracted"
        );

        Ok(UserInfo::new(claims.sub, claims.team_id, scopes))
    }

    /// Clear validation cache (useful for testing)
    #[cfg(test)]
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl Default for TokenValidator {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a test JWT (unsigned, for testing only)
    fn create_test_jwt(sub: &str, aud: &str, exp: u64, scope: Option<&str>) -> String {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

        let header = serde_json::json!({
            "alg": "HS256",
            "typ": "JWT"
        });

        let mut claims = serde_json::json!({
            "sub": sub,
            "aud": aud,
            "exp": exp,
            "iat": exp.saturating_sub(3600)
        });

        if let Some(s) = scope {
            claims["scope"] = serde_json::json!(s);
        }

        let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());
        let claims_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&claims).unwrap());

        format!("{}.{}.fake_signature", header_b64, claims_b64)
    }

    #[tokio::test]
    async fn test_validate_valid_token() {
        let validator = TokenValidator::new("https://test.example.com".to_string());
        let future_exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        let token = create_test_jwt(
            "user123",
            "https://test.example.com",
            future_exp,
            Some("boards:read"),
        );

        let result = validator.validate(&token).await;
        assert!(result.is_ok());

        let user_info = result.unwrap();
        assert_eq!(user_info.user_id, "user123");
        assert_eq!(user_info.scopes, vec!["boards:read"]);
    }

    #[tokio::test]
    async fn test_validate_expired_token() {
        let validator = TokenValidator::new("https://test.example.com".to_string());
        let past_exp = 1000; // Way in the past

        let token = create_test_jwt("user123", "https://test.example.com", past_exp, None);

        let result = validator.validate(&token).await;
        assert!(result.is_err());
        matches!(result.unwrap_err(), AuthError::TokenExpired);
    }

    #[tokio::test]
    async fn test_validate_wrong_audience() {
        let validator = TokenValidator::new("https://test.example.com".to_string());
        let future_exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        let token = create_test_jwt("user123", "https://wrong.example.com", future_exp, None);

        let result = validator.validate(&token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cache_hit() {
        let validator = TokenValidator::new("https://test.example.com".to_string());
        let future_exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        let token = create_test_jwt("user123", "https://test.example.com", future_exp, None);

        // First validation
        let result1 = validator.validate(&token).await;
        assert!(result1.is_ok());

        // Second validation should hit cache
        let result2 = validator.validate(&token).await;
        assert!(result2.is_ok());
        assert_eq!(result1.unwrap().user_id, result2.unwrap().user_id);
    }

    #[test]
    fn test_user_info_expiry() {
        let user_info = UserInfo {
            user_id: "test".to_string(),
            team_id: None,
            scopes: vec![],
            cached_at: 1000,
        };

        // Should be expired if cached_at is way in the past
        assert!(user_info.is_expired());
    }
}
