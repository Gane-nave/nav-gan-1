//! Security headers for HTTP responses.
//!
//! Provides a configurable set of security headers that should be applied
//! to all HTTP responses to harden the API against common web attacks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Security headers config
// ---------------------------------------------------------------------------

/// Configuration for security headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersConfig {
    /// Enable Strict-Transport-Security header.
    pub hsts_enabled: bool,
    /// HSTS max-age in seconds.
    pub hsts_max_age: u64,
    /// Include subdomains in HSTS.
    pub hsts_include_subdomains: bool,
    /// Content-Security-Policy value.
    pub content_security_policy: Option<String>,
    /// X-Content-Type-Options value.
    pub x_content_type_options: String,
    /// X-Frame-Options value.
    pub x_frame_options: String,
    /// X-XSS-Protection value.
    pub x_xss_protection: String,
    /// Referrer-Policy value.
    pub referrer_policy: String,
    /// Permissions-Policy value.
    pub permissions_policy: Option<String>,
    /// Cache-Control for API responses.
    pub cache_control: String,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            hsts_enabled: true,
            hsts_max_age: 31_536_000, // 1 year
            hsts_include_subdomains: true,
            content_security_policy: Some("default-src 'self'".to_string()),
            x_content_type_options: "nosniff".to_string(),
            x_frame_options: "DENY".to_string(),
            x_xss_protection: "1; mode=block".to_string(),
            referrer_policy: "strict-origin-when-cross-origin".to_string(),
            permissions_policy: Some("geolocation=(self), camera=(), microphone=()".to_string()),
            cache_control: "no-store, no-cache, must-revalidate".to_string(),
        }
    }
}

impl SecurityHeadersConfig {
    /// Build a map of header name → header value.
    pub fn to_header_map(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        if self.hsts_enabled {
            let mut hsts = format!("max-age={}", self.hsts_max_age);
            if self.hsts_include_subdomains {
                hsts.push_str("; includeSubDomains");
            }
            headers.insert("Strict-Transport-Security".to_string(), hsts);
        }

        if let Some(csp) = &self.content_security_policy {
            headers.insert("Content-Security-Policy".to_string(), csp.clone());
        }

        headers.insert(
            "X-Content-Type-Options".to_string(),
            self.x_content_type_options.clone(),
        );
        headers.insert("X-Frame-Options".to_string(), self.x_frame_options.clone());
        headers.insert(
            "X-XSS-Protection".to_string(),
            self.x_xss_protection.clone(),
        );
        headers.insert("Referrer-Policy".to_string(), self.referrer_policy.clone());

        if let Some(pp) = &self.permissions_policy {
            headers.insert("Permissions-Policy".to_string(), pp.clone());
        }

        headers.insert("Cache-Control".to_string(), self.cache_control.clone());

        headers
    }

    /// Number of headers that will be set.
    pub fn header_count(&self) -> usize {
        let mut count = 5; // always present: X-Content-Type-Options, X-Frame-Options, X-XSS-Protection, Referrer-Policy, Cache-Control
        if self.hsts_enabled {
            count += 1;
        }
        if self.content_security_policy.is_some() {
            count += 1;
        }
        if self.permissions_policy.is_some() {
            count += 1;
        }
        count
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_headers_are_secure() {
        let config = SecurityHeadersConfig::default();
        let headers = config.to_header_map();

        assert_eq!(headers["X-Content-Type-Options"], "nosniff");
        assert_eq!(headers["X-Frame-Options"], "DENY");
        assert_eq!(headers["X-XSS-Protection"], "1; mode=block");
        assert!(headers.contains_key("Strict-Transport-Security"));
        assert!(headers.contains_key("Content-Security-Policy"));
        assert!(headers.contains_key("Referrer-Policy"));
        assert!(headers.contains_key("Cache-Control"));
    }

    #[test]
    fn hsts_includes_subdomains() {
        let config = SecurityHeadersConfig::default();
        let headers = config.to_header_map();
        let hsts = &headers["Strict-Transport-Security"];
        assert!(hsts.contains("includeSubDomains"));
        assert!(hsts.contains("max-age=31536000"));
    }

    #[test]
    fn hsts_disabled() {
        let config = SecurityHeadersConfig {
            hsts_enabled: false,
            ..Default::default()
        };
        let headers = config.to_header_map();
        assert!(!headers.contains_key("Strict-Transport-Security"));
    }

    #[test]
    fn csp_optional() {
        let config = SecurityHeadersConfig {
            content_security_policy: None,
            ..Default::default()
        };
        let headers = config.to_header_map();
        assert!(!headers.contains_key("Content-Security-Policy"));
    }

    #[test]
    fn header_count_matches() {
        let config = SecurityHeadersConfig::default();
        assert_eq!(config.header_count(), config.to_header_map().len());
    }

    #[test]
    fn header_count_without_optionals() {
        let config = SecurityHeadersConfig {
            hsts_enabled: false,
            content_security_policy: None,
            permissions_policy: None,
            ..Default::default()
        };
        assert_eq!(config.header_count(), 5);
        assert_eq!(config.header_count(), config.to_header_map().len());
    }

    #[test]
    fn custom_cache_control() {
        let config = SecurityHeadersConfig {
            cache_control: "public, max-age=3600".to_string(),
            ..Default::default()
        };
        let headers = config.to_header_map();
        assert_eq!(headers["Cache-Control"], "public, max-age=3600");
    }
}
