//! JWT (JSON Web Token) generation and validation.
//!
//! Implements HS256-signed JWTs for API authentication with configurable
//! expiration, issuer, and audience claims.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use thiserror::Error;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("token has expired")]
    Expired,
    #[error("invalid token format: {0}")]
    InvalidFormat(String),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid base64: {0}")]
    Base64Error(String),
    #[error("invalid JSON: {0}")]
    JsonError(String),
    #[error("missing required claim: {0}")]
    MissingClaim(String),
    #[error("audience mismatch")]
    AudienceMismatch,
    #[error("issuer mismatch")]
    IssuerMismatch,
}

// ---------------------------------------------------------------------------
// Claims
// ---------------------------------------------------------------------------

/// Standard JWT claims with custom extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID or service ID).
    pub sub: String,
    /// Issuer.
    pub iss: String,
    /// Audience.
    pub aud: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// JWT ID (unique identifier).
    pub jti: String,
    /// User role.
    pub role: String,
    /// Additional scopes (comma-separated).
    #[serde(default)]
    pub scopes: String,
}

// ---------------------------------------------------------------------------
// Token config
// ---------------------------------------------------------------------------

/// Configuration for JWT token generation.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// HMAC secret key.
    secret: Vec<u8>,
    /// Token issuer.
    pub issuer: String,
    /// Token audience.
    pub audience: String,
    /// Token lifetime.
    pub token_lifetime: Duration,
}

impl JwtConfig {
    pub fn new(secret: &[u8], issuer: &str, audience: &str, lifetime_secs: i64) -> Self {
        Self {
            secret: secret.to_vec(),
            issuer: issuer.to_string(),
            audience: audience.to_string(),
            token_lifetime: Duration::seconds(lifetime_secs),
        }
    }

    /// Create a config with default AURORA NAV settings.
    pub fn default_aurora() -> Self {
        Self::new(
            b"aurora-nav-default-secret-change-me",
            "aurora-nav",
            "aurora-api",
            3600, // 1 hour
        )
    }
}

// ---------------------------------------------------------------------------
// Token manager
// ---------------------------------------------------------------------------

/// Manages JWT token creation and validation.
pub struct JwtManager {
    config: JwtConfig,
}

impl JwtManager {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// Access the configuration.
    pub fn config(&self) -> &JwtConfig {
        &self.config
    }

    /// Generate a signed JWT token for the given subject and role.
    pub fn generate_token(&self, subject: &str, role: &str, scopes: &str) -> String {
        let now = Utc::now();
        let claims = Claims {
            sub: subject.to_string(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            exp: (now + self.config.token_lifetime).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            role: role.to_string(),
            scopes: scopes.to_string(),
        };

        self.encode_and_sign(&claims)
    }

    /// Validate a JWT token and return its claims.
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::InvalidFormat(
                "expected 3 dot-separated parts".to_string(),
            ));
        }

        // Verify signature
        let message = format!("{}.{}", parts[0], parts[1]);
        let provided_sig = URL_SAFE_NO_PAD
            .decode(parts[2])
            .map_err(|e| JwtError::Base64Error(e.to_string()))?;
        let expected_sig = self.compute_signature(message.as_bytes());

        if provided_sig != expected_sig {
            return Err(JwtError::InvalidSignature);
        }

        // Decode claims
        let claims_json = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| JwtError::Base64Error(e.to_string()))?;
        let claims: Claims =
            serde_json::from_slice(&claims_json).map_err(|e| JwtError::JsonError(e.to_string()))?;

        // Validate expiration
        let now = Utc::now().timestamp();
        if claims.exp < now {
            return Err(JwtError::Expired);
        }

        // Validate issuer
        if claims.iss != self.config.issuer {
            return Err(JwtError::IssuerMismatch);
        }

        // Validate audience
        if claims.aud != self.config.audience {
            return Err(JwtError::AudienceMismatch);
        }

        Ok(claims)
    }

    fn encode_and_sign(&self, claims: &Claims) -> String {
        // Header
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let header_b64 = URL_SAFE_NO_PAD.encode(header.as_bytes());

        // Payload
        let payload = serde_json::to_string(claims).expect("claims serialization");
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());

        // Signature
        let message = format!("{header_b64}.{payload_b64}");
        let sig = self.compute_signature(message.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(&sig);

        format!("{header_b64}.{payload_b64}.{sig_b64}")
    }

    fn compute_signature(&self, message: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.config.secret).expect("HMAC key init");
        mac.update(message);
        mac.finalize().into_bytes().to_vec()
    }
}

impl Default for JwtManager {
    fn default() -> Self {
        Self::new(JwtConfig::default_aurora())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manager() -> JwtManager {
        JwtManager::new(JwtConfig::default_aurora())
    }

    #[test]
    fn generate_and_validate_token() {
        let mgr = test_manager();
        let token = mgr.generate_token("user-123", "admin", "read,write");
        let claims = mgr.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.scopes, "read,write");
        assert_eq!(claims.iss, "aurora-nav");
        assert_eq!(claims.aud, "aurora-api");
    }

    #[test]
    fn validate_rejects_tampered_payload() {
        let mgr = test_manager();
        let token = mgr.generate_token("user-1", "viewer", "");
        let parts: Vec<&str> = token.split('.').collect();
        // Tamper with the payload
        let tampered = format!("{}.{}.{}", parts[0], "dGFtcGVyZWQ", parts[2]);
        assert!(matches!(
            mgr.validate_token(&tampered),
            Err(JwtError::InvalidSignature)
        ));
    }

    #[test]
    fn validate_rejects_tampered_signature() {
        let mgr = test_manager();
        let token = mgr.generate_token("user-1", "viewer", "");
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.{}.{}", parts[0], parts[1], "badsig");
        assert!(matches!(
            mgr.validate_token(&tampered),
            Err(JwtError::InvalidSignature)
        ));
    }

    #[test]
    fn validate_rejects_expired_token() {
        let config = JwtConfig::new(b"secret", "iss", "aud", -10); // negative lifetime
        let mgr = JwtManager::new(config);
        let token = mgr.generate_token("user-1", "viewer", "");
        assert!(matches!(mgr.validate_token(&token), Err(JwtError::Expired)));
    }

    #[test]
    fn validate_rejects_wrong_issuer() {
        let config_a = JwtConfig::new(b"shared-secret", "issuer-a", "aud", 3600);
        let config_b = JwtConfig::new(b"shared-secret", "issuer-b", "aud", 3600);
        let mgr_a = JwtManager::new(config_a);
        let mgr_b = JwtManager::new(config_b);
        let token = mgr_a.generate_token("user-1", "admin", "");
        assert!(matches!(
            mgr_b.validate_token(&token),
            Err(JwtError::IssuerMismatch)
        ));
    }

    #[test]
    fn validate_rejects_wrong_audience() {
        let config_a = JwtConfig::new(b"shared-secret", "iss", "aud-a", 3600);
        let config_b = JwtConfig::new(b"shared-secret", "iss", "aud-b", 3600);
        let mgr_a = JwtManager::new(config_a);
        let mgr_b = JwtManager::new(config_b);
        let token = mgr_a.generate_token("user-1", "admin", "");
        assert!(matches!(
            mgr_b.validate_token(&token),
            Err(JwtError::AudienceMismatch)
        ));
    }

    #[test]
    fn validate_rejects_wrong_secret() {
        let config_a = JwtConfig::new(b"secret-a", "iss", "aud", 3600);
        let config_b = JwtConfig::new(b"secret-b", "iss", "aud", 3600);
        let mgr_a = JwtManager::new(config_a);
        let mgr_b = JwtManager::new(config_b);
        let token = mgr_a.generate_token("user-1", "admin", "");
        assert!(matches!(
            mgr_b.validate_token(&token),
            Err(JwtError::InvalidSignature)
        ));
    }

    #[test]
    fn validate_rejects_malformed_token() {
        let mgr = test_manager();
        assert!(matches!(
            mgr.validate_token("not.a.valid.token"),
            Err(JwtError::InvalidFormat(_))
        ));
        assert!(matches!(
            mgr.validate_token("only-one-part"),
            Err(JwtError::InvalidFormat(_))
        ));
    }

    #[test]
    fn each_token_gets_unique_jti() {
        let mgr = test_manager();
        let t1 = mgr.generate_token("u", "r", "");
        let t2 = mgr.generate_token("u", "r", "");
        let c1 = mgr.validate_token(&t1).unwrap();
        let c2 = mgr.validate_token(&t2).unwrap();
        assert_ne!(c1.jti, c2.jti);
    }

    #[test]
    fn token_contains_three_base64_parts() {
        let mgr = test_manager();
        let token = mgr.generate_token("user", "admin", "scope");
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
        // Each part should be valid base64
        for part in &parts {
            assert!(URL_SAFE_NO_PAD.decode(part).is_ok());
        }
    }
}
