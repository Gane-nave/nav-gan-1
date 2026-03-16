//! API key generation, storage, and validation.
//!
//! Supports creating named API keys with optional expiration, scoping,
//! and revocation. Keys are stored in-memory with thread-safe access.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ApiKeyError {
    #[error("API key not found")]
    NotFound,
    #[error("API key has been revoked")]
    Revoked,
    #[error("API key has expired")]
    Expired,
    #[error("API key name already exists: {0}")]
    DuplicateName(String),
    #[error("insufficient scope: required {required}, have {have}")]
    InsufficientScope { required: String, have: String },
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Metadata for a stored API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    /// Unique identifier.
    pub id: Uuid,
    /// Human-readable name.
    pub name: String,
    /// SHA-256 hash of the key (we never store the raw key).
    pub key_hash: String,
    /// Key prefix for display (first 8 chars).
    pub prefix: String,
    /// Granted scopes.
    pub scopes: Vec<String>,
    /// When the key was created.
    pub created_at: DateTime<Utc>,
    /// Optional expiration.
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether the key has been revoked.
    pub revoked: bool,
    /// When the key was last used.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Total usage count.
    pub usage_count: u64,
}

/// Result of creating a new API key (includes the raw key, shown only once).
#[derive(Debug)]
pub struct CreateKeyResult {
    pub record: ApiKeyRecord,
    pub raw_key: String,
}

// ---------------------------------------------------------------------------
// Key store
// ---------------------------------------------------------------------------

/// Thread-safe in-memory API key store.
pub struct ApiKeyStore {
    /// Maps key_hash → record.
    keys: RwLock<HashMap<String, ApiKeyRecord>>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new API key with the given name and scopes.
    pub fn create_key(
        &self,
        name: &str,
        scopes: Vec<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<CreateKeyResult, ApiKeyError> {
        let mut keys = self.keys.write();

        // Check for duplicate name
        if keys.values().any(|r| r.name == name && !r.revoked) {
            return Err(ApiKeyError::DuplicateName(name.to_string()));
        }

        // Generate raw key
        let raw_key = format!("aknav_{}", Uuid::new_v4().as_simple());
        let key_hash = hash_key(&raw_key);
        let prefix = raw_key[..8].to_string();

        let record = ApiKeyRecord {
            id: Uuid::new_v4(),
            name: name.to_string(),
            key_hash: key_hash.clone(),
            prefix,
            scopes,
            created_at: Utc::now(),
            expires_at,
            revoked: false,
            last_used_at: None,
            usage_count: 0,
        };

        keys.insert(key_hash, record.clone());
        Ok(CreateKeyResult { record, raw_key })
    }

    /// Validate an API key and return its record. Updates last_used_at.
    pub fn validate_key(&self, raw_key: &str) -> Result<ApiKeyRecord, ApiKeyError> {
        let key_hash = hash_key(raw_key);
        let mut keys = self.keys.write();
        let record = keys.get_mut(&key_hash).ok_or(ApiKeyError::NotFound)?;

        if record.revoked {
            return Err(ApiKeyError::Revoked);
        }

        if let Some(exp) = record.expires_at {
            if Utc::now() > exp {
                return Err(ApiKeyError::Expired);
            }
        }

        record.last_used_at = Some(Utc::now());
        record.usage_count += 1;

        Ok(record.clone())
    }

    /// Check that a key has the required scope.
    pub fn validate_key_with_scope(
        &self,
        raw_key: &str,
        required_scope: &str,
    ) -> Result<ApiKeyRecord, ApiKeyError> {
        let record = self.validate_key(raw_key)?;

        // Wildcard scope grants everything
        if record.scopes.iter().any(|s| s == "*") {
            return Ok(record);
        }

        if !record.scopes.iter().any(|s| s == required_scope) {
            return Err(ApiKeyError::InsufficientScope {
                required: required_scope.to_string(),
                have: record.scopes.join(","),
            });
        }

        Ok(record)
    }

    /// Revoke an API key by name.
    pub fn revoke_key(&self, name: &str) -> Result<(), ApiKeyError> {
        let mut keys = self.keys.write();
        let record = keys
            .values_mut()
            .find(|r| r.name == name && !r.revoked)
            .ok_or(ApiKeyError::NotFound)?;
        record.revoked = true;
        Ok(())
    }

    /// List all API key records (without raw keys).
    pub fn list_keys(&self) -> Vec<ApiKeyRecord> {
        self.keys.read().values().cloned().collect()
    }

    /// Count active (non-revoked, non-expired) keys.
    pub fn active_key_count(&self) -> usize {
        let now = Utc::now();
        self.keys
            .read()
            .values()
            .filter(|r| !r.revoked && r.expires_at.is_none_or(|exp| exp > now))
            .count()
    }
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash a raw API key with SHA-256.
fn hash_key(raw_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    format!("{:x}", hasher.finalize())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_validate_key() {
        let store = ApiKeyStore::new();
        let result = store
            .create_key("test-key", vec!["read".into()], None)
            .unwrap();
        assert!(result.raw_key.starts_with("aknav_"));
        assert_eq!(result.record.name, "test-key");

        let validated = store.validate_key(&result.raw_key).unwrap();
        assert_eq!(validated.name, "test-key");
        assert_eq!(validated.usage_count, 1);
    }

    #[test]
    fn validate_nonexistent_key() {
        let store = ApiKeyStore::new();
        assert!(matches!(
            store.validate_key("aknav_bogus"),
            Err(ApiKeyError::NotFound)
        ));
    }

    #[test]
    fn revoke_and_reject() {
        let store = ApiKeyStore::new();
        let result = store.create_key("k1", vec![], None).unwrap();
        store.revoke_key("k1").unwrap();
        assert!(matches!(
            store.validate_key(&result.raw_key),
            Err(ApiKeyError::Revoked)
        ));
    }

    #[test]
    fn expired_key_rejected() {
        let store = ApiKeyStore::new();
        let past = Utc::now() - chrono::Duration::hours(1);
        let result = store.create_key("expired", vec![], Some(past)).unwrap();
        assert!(matches!(
            store.validate_key(&result.raw_key),
            Err(ApiKeyError::Expired)
        ));
    }

    #[test]
    fn duplicate_name_rejected() {
        let store = ApiKeyStore::new();
        store.create_key("unique", vec![], None).unwrap();
        assert!(matches!(
            store.create_key("unique", vec![], None),
            Err(ApiKeyError::DuplicateName(_))
        ));
    }

    #[test]
    fn duplicate_name_allowed_after_revocation() {
        let store = ApiKeyStore::new();
        store.create_key("reusable", vec![], None).unwrap();
        store.revoke_key("reusable").unwrap();
        assert!(store.create_key("reusable", vec![], None).is_ok());
    }

    #[test]
    fn scope_validation() {
        let store = ApiKeyStore::new();
        let result = store
            .create_key("scoped", vec!["read".into(), "write".into()], None)
            .unwrap();

        assert!(store
            .validate_key_with_scope(&result.raw_key, "read")
            .is_ok());
        assert!(matches!(
            store.validate_key_with_scope(&result.raw_key, "admin"),
            Err(ApiKeyError::InsufficientScope { .. })
        ));
    }

    #[test]
    fn wildcard_scope_grants_everything() {
        let store = ApiKeyStore::new();
        let result = store
            .create_key("wildcard", vec!["*".into()], None)
            .unwrap();
        assert!(store
            .validate_key_with_scope(&result.raw_key, "anything")
            .is_ok());
    }

    #[test]
    fn list_and_count_keys() {
        let store = ApiKeyStore::new();
        store.create_key("a", vec![], None).unwrap();
        store.create_key("b", vec![], None).unwrap();
        store.create_key("c", vec![], None).unwrap();
        assert_eq!(store.list_keys().len(), 3);
        assert_eq!(store.active_key_count(), 3);

        store.revoke_key("b").unwrap();
        assert_eq!(store.list_keys().len(), 3); // still listed
        assert_eq!(store.active_key_count(), 2); // but not active
    }

    #[test]
    fn usage_count_increments() {
        let store = ApiKeyStore::new();
        let result = store.create_key("counter", vec![], None).unwrap();
        store.validate_key(&result.raw_key).unwrap();
        store.validate_key(&result.raw_key).unwrap();
        let r = store.validate_key(&result.raw_key).unwrap();
        assert_eq!(r.usage_count, 3);
    }

    #[test]
    fn key_prefix_is_first_8_chars() {
        let store = ApiKeyStore::new();
        let result = store.create_key("pf", vec![], None).unwrap();
        assert_eq!(result.record.prefix, &result.raw_key[..8]);
    }
}
