use axum::http::HeaderMap;

use crate::auth::AuthError;

const BEARER_PREFIX: &str = "Bearer ";

/// Extract Bearer token from Authorization header
///
/// Parses the standard `Authorization: Bearer <token>` header format.
///
/// # Arguments
///
/// * `headers` - HTTP headers from the request
///
/// # Returns
///
/// * `Ok(String)` - The extracted token
/// * `Err(AuthError::NoToken)` - If Authorization header is missing
/// * `Err(AuthError::InvalidTokenFormat)` - If header format is invalid or token is empty
///
/// # Examples
///
/// ```ignore
/// let mut headers = HeaderMap::new();
/// headers.insert(
///     "authorization",
///     "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".parse().unwrap(),
/// );
/// let token = extract_bearer_token(&headers)?;
/// assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");
/// ```
pub fn extract_bearer_token(headers: &HeaderMap) -> Result<String, AuthError> {
    // Get the Authorization header value
    let auth_header = headers
        .get("authorization")
        .ok_or(AuthError::NoToken)?
        .to_str()
        .map_err(|_| AuthError::InvalidTokenFormat)?;

    // Check if header starts with "Bearer " (case-sensitive per RFC 7235)
    if !auth_header.starts_with(BEARER_PREFIX) {
        return Err(AuthError::InvalidTokenFormat);
    }

    // Extract token after "Bearer " prefix
    let token = &auth_header[BEARER_PREFIX.len()..];

    // Ensure token is not empty
    if token.is_empty() {
        return Err(AuthError::InvalidTokenFormat);
    }

    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_valid_bearer_token() {
        let mut headers = HeaderMap::new();
        let token_value =
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.TJVA95OrM7E";
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token_value);
    }

    #[test]
    fn test_missing_authorization_header() {
        let headers = HeaderMap::new();
        let result = extract_bearer_token(&headers);

        assert!(result.is_err());
        match result {
            Err(AuthError::NoToken) => {}
            _ => panic!("Expected NoToken error"),
        }
    }

    #[test]
    fn test_wrong_auth_scheme() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str("Basic dXNlcjpwYXNz").unwrap(),
        );

        let result = extract_bearer_token(&headers);

        assert!(result.is_err());
        match result {
            Err(AuthError::InvalidTokenFormat) => {}
            _ => panic!("Expected InvalidTokenFormat error"),
        }
    }

    #[test]
    fn test_empty_token() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str("Bearer ").unwrap());

        let result = extract_bearer_token(&headers);

        assert!(result.is_err());
        match result {
            Err(AuthError::InvalidTokenFormat) => {}
            _ => panic!("Expected InvalidTokenFormat error"),
        }
    }

    #[test]
    fn test_multiple_spaces_in_bearer_prefix() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str("Bearer  token123").unwrap(),
        );

        let result = extract_bearer_token(&headers);

        // Multiple spaces result in a token with leading space (not ideal but valid per RFC)
        // The token would be " token123" (with leading space)
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), " token123");
    }

    #[test]
    fn test_lowercase_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str("bearer token123").unwrap(),
        );

        let result = extract_bearer_token(&headers);

        // Should fail because "Bearer" must be capitalized (RFC 7235)
        assert!(result.is_err());
        match result {
            Err(AuthError::InvalidTokenFormat) => {}
            _ => panic!("Expected InvalidTokenFormat error"),
        }
    }

    #[test]
    fn test_token_with_special_characters() {
        let mut headers = HeaderMap::new();
        let token_value = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIn0";
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token_value);
    }

    #[test]
    fn test_token_with_dots_and_hyphens() {
        let mut headers = HeaderMap::new();
        let token_value = "aB.cD-eF_gH.iJ-kL_mN.oP-qR";
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token_value);
    }

    #[test]
    fn test_short_token() {
        let mut headers = HeaderMap::new();
        let token_value = "x";
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token_value);
    }

    #[test]
    fn test_long_token() {
        let mut headers = HeaderMap::new();
        let token_value = "a".repeat(2000);
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", token_value)).unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), token_value);
    }

    #[test]
    fn test_token_with_trailing_whitespace() {
        let mut headers = HeaderMap::new();
        // Note: HeaderValue should trim, but we test the behavior
        let header_str = "Bearer token123 ";
        headers.insert("authorization", HeaderValue::from_str(header_str).unwrap());

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        // Token includes trailing space as-is
        assert_eq!(result.unwrap(), "token123 ");
    }

    #[test]
    fn test_no_space_after_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str("Bearertoken123").unwrap(),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_err());
    }
}
