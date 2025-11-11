pub mod bearer;
pub mod token_validator;
pub mod types;

#[cfg(feature = "stdio-mcp")]
pub mod token_store;

pub use bearer::extract_bearer_token;
pub use token_validator::{TokenValidator, UserInfo};
pub use types::{AuthError, TokenSet};

#[cfg(feature = "stdio-mcp")]
pub use token_store::TokenStore;

// Type alias for backward compatibility with stdio-mcp code
// MiroOAuthClient has been replaced by MiroOAuthProvider in the oauth module
#[cfg(feature = "stdio-mcp")]
pub use crate::oauth::proxy_provider::MiroOAuthProvider as MiroOAuthClient;
