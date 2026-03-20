//! CORS (Cross-Origin Resource Sharing) policy configuration.
//!
//! Provides configurable CORS policies for the AURORA NAV API,
//! supporting both permissive development and restrictive production modes.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// CORS policy
// ---------------------------------------------------------------------------

/// HTTP methods allowed in CORS requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorsMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

impl CorsMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            CorsMethod::Get => "GET",
            CorsMethod::Post => "POST",
            CorsMethod::Put => "PUT",
            CorsMethod::Delete => "DELETE",
            CorsMethod::Patch => "PATCH",
            CorsMethod::Options => "OPTIONS",
            CorsMethod::Head => "HEAD",
        }
    }
}

/// CORS policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsPolicy {
    /// Allowed origins (use `*` for any, or specific domains).
    pub allowed_origins: Vec<String>,
    /// Allowed HTTP methods.
    pub allowed_methods: Vec<CorsMethod>,
    /// Allowed request headers.
    pub allowed_headers: Vec<String>,
    /// Headers exposed to the client.
    pub exposed_headers: Vec<String>,
    /// Allow credentials (cookies, auth headers).
    pub allow_credentials: bool,
    /// Preflight cache duration in seconds.
    pub max_age_seconds: u64,
}

impl CorsPolicy {
    /// Permissive CORS for development.
    pub fn permissive() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                CorsMethod::Get,
                CorsMethod::Post,
                CorsMethod::Put,
                CorsMethod::Delete,
                CorsMethod::Patch,
                CorsMethod::Options,
                CorsMethod::Head,
            ],
            allowed_headers: vec!["*".to_string()],
            exposed_headers: vec![],
            allow_credentials: false,
            max_age_seconds: 3600,
        }
    }

    /// Restrictive CORS for production.
    pub fn restrictive(origins: Vec<String>) -> Self {
        Self {
            allowed_origins: origins,
            allowed_methods: vec![CorsMethod::Get, CorsMethod::Post, CorsMethod::Options],
            allowed_headers: vec![
                "Authorization".to_string(),
                "Content-Type".to_string(),
                "Accept".to_string(),
                "X-Request-Id".to_string(),
            ],
            exposed_headers: vec!["X-Request-Id".to_string()],
            allow_credentials: true,
            max_age_seconds: 86400,
        }
    }

    /// Check if an origin is allowed by this policy.
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.iter().any(|o| o == "*" || o == origin)
    }

    /// Check if a method is allowed.
    pub fn is_method_allowed(&self, method: &str) -> bool {
        self.allowed_methods
            .iter()
            .any(|m| m.as_str().eq_ignore_ascii_case(method))
    }

    /// Check if a header is allowed.
    pub fn is_header_allowed(&self, header: &str) -> bool {
        self.allowed_headers
            .iter()
            .any(|h| h == "*" || h.eq_ignore_ascii_case(header))
    }

    /// Build Access-Control-Allow-Methods header value.
    pub fn methods_header_value(&self) -> String {
        self.allowed_methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Build Access-Control-Allow-Headers header value.
    pub fn headers_header_value(&self) -> String {
        self.allowed_headers.join(", ")
    }
}

impl Default for CorsPolicy {
    fn default() -> Self {
        Self::permissive()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissive_allows_any_origin() {
        let policy = CorsPolicy::permissive();
        assert!(policy.is_origin_allowed("http://localhost:3000"));
        assert!(policy.is_origin_allowed("https://example.com"));
    }

    #[test]
    fn restrictive_only_allows_listed_origins() {
        let policy = CorsPolicy::restrictive(vec!["https://aurora-nav.com".to_string()]);
        assert!(policy.is_origin_allowed("https://aurora-nav.com"));
        assert!(!policy.is_origin_allowed("https://evil.com"));
    }

    #[test]
    fn permissive_allows_all_methods() {
        let policy = CorsPolicy::permissive();
        assert!(policy.is_method_allowed("GET"));
        assert!(policy.is_method_allowed("DELETE"));
        assert!(policy.is_method_allowed("PATCH"));
    }

    #[test]
    fn restrictive_limits_methods() {
        let policy = CorsPolicy::restrictive(vec!["https://example.com".to_string()]);
        assert!(policy.is_method_allowed("GET"));
        assert!(policy.is_method_allowed("POST"));
        assert!(!policy.is_method_allowed("DELETE"));
        assert!(!policy.is_method_allowed("PUT"));
    }

    #[test]
    fn permissive_allows_all_headers() {
        let policy = CorsPolicy::permissive();
        assert!(policy.is_header_allowed("Authorization"));
        assert!(policy.is_header_allowed("X-Custom-Header"));
    }

    #[test]
    fn restrictive_limits_headers() {
        let policy = CorsPolicy::restrictive(vec!["https://example.com".to_string()]);
        assert!(policy.is_header_allowed("Authorization"));
        assert!(policy.is_header_allowed("Content-Type"));
        assert!(!policy.is_header_allowed("X-Custom-Header"));
    }

    #[test]
    fn methods_header_value() {
        let policy = CorsPolicy::restrictive(vec!["https://example.com".to_string()]);
        let value = policy.methods_header_value();
        assert!(value.contains("GET"));
        assert!(value.contains("POST"));
        assert!(value.contains("OPTIONS"));
    }

    #[test]
    fn permissive_no_credentials() {
        let policy = CorsPolicy::permissive();
        assert!(!policy.allow_credentials);
    }

    #[test]
    fn restrictive_allows_credentials() {
        let policy = CorsPolicy::restrictive(vec!["https://example.com".to_string()]);
        assert!(policy.allow_credentials);
    }

    #[test]
    fn default_is_permissive() {
        let policy = CorsPolicy::default();
        assert!(policy.is_origin_allowed("anything"));
    }
}
