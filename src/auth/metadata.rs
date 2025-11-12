use serde::{Deserialize, Serialize};

/// Protected Resource Metadata per RFC 9728
///
/// This metadata tells OAuth clients (like Claude.ai) where to find the authorization server
/// for this protected resource. In the Resource Server pattern, we delegate OAuth to Miro
/// and only validate tokens.
///
/// See: https://datatracker.ietf.org/doc/html/rfc9728
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedResourceMetadata {
    /// The protected resource identifier (our MCP server URL)
    pub resource: String,

    /// List of authorization servers that issue tokens for this resource
    /// For Miro integration, this points to Miro's OAuth server
    pub authorization_servers: Vec<String>,

    /// Optional: Scopes required for accessing this resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,

    /// Optional: Token introspection endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_endpoint: Option<String>,

    /// Optional: Token revocation endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_endpoint: Option<String>,
}

impl ProtectedResourceMetadata {
    /// Create metadata for Miro Resource Server pattern
    ///
    /// # Arguments
    ///
    /// * `base_url` - Our MCP server URL (e.g., "https://miro-mcp.fly-agile.com")
    ///
    /// # Returns
    ///
    /// Metadata indicating Miro as the authorization server
    pub fn new_for_miro(base_url: String) -> Self {
        Self {
            resource: base_url,
            // Miro's OAuth authorization server
            authorization_servers: vec!["https://miro.com".to_string()],
            // Miro scopes for board access
            scopes_supported: Some(vec!["boards:read".to_string(), "boards:write".to_string()]),
            // Miro's introspection endpoint
            introspection_endpoint: Some(
                "https://api.miro.com/v2/oauth/token/introspect".to_string(),
            ),
            revocation_endpoint: None, // Miro doesn't provide public revocation endpoint
        }
    }

    /// Validate metadata completeness
    pub fn validate(&self) -> Result<(), String> {
        if self.resource.is_empty() {
            return Err("Resource URL cannot be empty".to_string());
        }

        if self.authorization_servers.is_empty() {
            return Err("At least one authorization server must be specified".to_string());
        }

        // Validate resource URL format
        if !self.resource.starts_with("https://") && !self.resource.starts_with("http://") {
            return Err("Resource URL must be a valid HTTP(S) URL".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_for_miro() {
        let base_url = "https://miro-mcp.fly-agile.com".to_string();
        let metadata = ProtectedResourceMetadata::new_for_miro(base_url.clone());

        assert_eq!(metadata.resource, base_url);
        assert_eq!(metadata.authorization_servers, vec!["https://miro.com"]);
        assert!(metadata.scopes_supported.is_some());
        assert!(metadata.introspection_endpoint.is_some());
    }

    #[test]
    fn test_validate_success() {
        let metadata =
            ProtectedResourceMetadata::new_for_miro("https://miro-mcp.fly-agile.com".to_string());
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_resource() {
        let mut metadata =
            ProtectedResourceMetadata::new_for_miro("https://miro-mcp.fly-agile.com".to_string());
        metadata.resource = "".to_string();
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_validate_no_auth_servers() {
        let mut metadata =
            ProtectedResourceMetadata::new_for_miro("https://miro-mcp.fly-agile.com".to_string());
        metadata.authorization_servers.clear();
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_url() {
        let mut metadata =
            ProtectedResourceMetadata::new_for_miro("https://miro-mcp.fly-agile.com".to_string());
        metadata.resource = "not-a-url".to_string();
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let metadata =
            ProtectedResourceMetadata::new_for_miro("https://miro-mcp.fly-agile.com".to_string());
        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("resource"));
        assert!(json.contains("authorization_servers"));
        assert!(json.contains("https://miro.com"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "resource": "https://miro-mcp.fly-agile.com",
            "authorization_servers": ["https://miro.com"],
            "scopes_supported": ["boards:read", "boards:write"]
        }"#;
        let metadata: ProtectedResourceMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.resource, "https://miro-mcp.fly-agile.com");
        assert_eq!(metadata.authorization_servers.len(), 1);
    }
}
