//! TLS configuration and certificate management.
//!
//! Provides TLS configuration for the AURORA NAV API server,
//! supporting configurable cipher suites, protocol versions,
//! and certificate paths.

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum TlsError {
    #[error("certificate file not found: {0}")]
    CertNotFound(String),
    #[error("private key file not found: {0}")]
    KeyNotFound(String),
    #[error("invalid TLS version: {0}")]
    InvalidVersion(String),
    #[error("no cipher suites configured")]
    NoCipherSuites,
}

// ---------------------------------------------------------------------------
// TLS version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsVersion {
    Tls12,
    Tls13,
}

impl TlsVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            TlsVersion::Tls12 => "TLSv1.2",
            TlsVersion::Tls13 => "TLSv1.3",
        }
    }

    pub fn parse_version(s: &str) -> Result<Self, TlsError> {
        match s {
            "1.2" | "TLSv1.2" => Ok(TlsVersion::Tls12),
            "1.3" | "TLSv1.3" => Ok(TlsVersion::Tls13),
            _ => Err(TlsError::InvalidVersion(s.to_string())),
        }
    }
}

// ---------------------------------------------------------------------------
// TLS config
// ---------------------------------------------------------------------------

/// TLS configuration for the API server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Whether TLS is enabled.
    pub enabled: bool,
    /// Path to the certificate chain PEM file.
    pub cert_path: Option<String>,
    /// Path to the private key PEM file.
    pub key_path: Option<String>,
    /// Minimum TLS version.
    pub min_version: TlsVersion,
    /// Allowed cipher suites.
    pub cipher_suites: Vec<String>,
    /// Enable OCSP stapling.
    pub ocsp_stapling: bool,
    /// Enable client certificate authentication.
    pub client_auth: bool,
    /// Path to CA certificate for client auth.
    pub client_ca_path: Option<String>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: None,
            key_path: None,
            min_version: TlsVersion::Tls12,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
            ocsp_stapling: false,
            client_auth: false,
            client_ca_path: None,
        }
    }
}

impl TlsConfig {
    /// Create a production-ready TLS config.
    pub fn production(cert_path: &str, key_path: &str) -> Self {
        Self {
            enabled: true,
            cert_path: Some(cert_path.to_string()),
            key_path: Some(key_path.to_string()),
            min_version: TlsVersion::Tls13,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
            ocsp_stapling: true,
            client_auth: false,
            client_ca_path: None,
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), TlsError> {
        if !self.enabled {
            return Ok(());
        }

        if self.cert_path.is_none() {
            return Err(TlsError::CertNotFound(
                "cert_path is required when TLS is enabled".to_string(),
            ));
        }

        if self.key_path.is_none() {
            return Err(TlsError::KeyNotFound(
                "key_path is required when TLS is enabled".to_string(),
            ));
        }

        if self.cipher_suites.is_empty() {
            return Err(TlsError::NoCipherSuites);
        }

        if self.client_auth && self.client_ca_path.is_none() {
            return Err(TlsError::CertNotFound(
                "client_ca_path is required when client_auth is enabled".to_string(),
            ));
        }

        Ok(())
    }

    /// Summary string for logging.
    pub fn summary(&self) -> String {
        if !self.enabled {
            return "TLS disabled".to_string();
        }
        format!(
            "TLS enabled, min_version={}, ciphers={}, ocsp={}, client_auth={}",
            self.min_version.as_str(),
            self.cipher_suites.len(),
            self.ocsp_stapling,
            self.client_auth,
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tls_disabled() {
        let config = TlsConfig::default();
        assert!(!config.enabled);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn production_config_valid() {
        let config = TlsConfig::production("/path/cert.pem", "/path/key.pem");
        assert!(config.enabled);
        assert_eq!(config.min_version, TlsVersion::Tls13);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn enabled_without_cert_fails() {
        let config = TlsConfig {
            enabled: true,
            cert_path: None,
            key_path: Some("/path/key.pem".to_string()),
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(TlsError::CertNotFound(_))));
    }

    #[test]
    fn enabled_without_key_fails() {
        let config = TlsConfig {
            enabled: true,
            cert_path: Some("/path/cert.pem".to_string()),
            key_path: None,
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(TlsError::KeyNotFound(_))));
    }

    #[test]
    fn empty_cipher_suites_fails() {
        let config = TlsConfig {
            enabled: true,
            cert_path: Some("/path/cert.pem".to_string()),
            key_path: Some("/path/key.pem".to_string()),
            cipher_suites: vec![],
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(TlsError::NoCipherSuites)));
    }

    #[test]
    fn client_auth_without_ca_fails() {
        let config = TlsConfig {
            enabled: true,
            cert_path: Some("/path/cert.pem".to_string()),
            key_path: Some("/path/key.pem".to_string()),
            client_auth: true,
            client_ca_path: None,
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(TlsError::CertNotFound(_))));
    }

    #[test]
    fn client_auth_with_ca_ok() {
        let config = TlsConfig {
            enabled: true,
            cert_path: Some("/path/cert.pem".to_string()),
            key_path: Some("/path/key.pem".to_string()),
            client_auth: true,
            client_ca_path: Some("/path/ca.pem".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn tls_version_roundtrip() {
        assert_eq!(TlsVersion::parse_version("1.2").unwrap(), TlsVersion::Tls12);
        assert_eq!(TlsVersion::parse_version("1.3").unwrap(), TlsVersion::Tls13);
        assert_eq!(
            TlsVersion::parse_version("TLSv1.3").unwrap(),
            TlsVersion::Tls13
        );
    }

    #[test]
    fn invalid_tls_version() {
        assert!(matches!(
            TlsVersion::parse_version("1.1"),
            Err(TlsError::InvalidVersion(_))
        ));
    }

    #[test]
    fn summary_disabled() {
        let config = TlsConfig::default();
        assert_eq!(config.summary(), "TLS disabled");
    }

    #[test]
    fn summary_enabled() {
        let config = TlsConfig::production("/cert", "/key");
        let summary = config.summary();
        assert!(summary.contains("TLS enabled"));
        assert!(summary.contains("TLSv1.3"));
    }
}
