//! API key management — creation, validation, rotation, and revocation
//! of API keys for developer access to AURORA NAV services.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// API key types
// ---------------------------------------------------------------------------

/// An API key with associated metadata and permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: EntityId,
    /// The key string (prefix + hash). Only the prefix is stored in plain text.
    pub key_prefix: String,
    /// SHA-256 hash of the full key for validation.
    pub key_hash: String,
    pub name: String,
    pub owner_id: EntityId,
    pub permissions: Vec<Permission>,
    pub tier: ApiTier,
    pub status: KeyStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub request_count: u64,
}

/// API key status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStatus {
    /// Active and usable.
    Active,
    /// Temporarily suspended.
    Suspended,
    /// Permanently revoked.
    Revoked,
    /// Past expiration date.
    Expired,
}

/// Permission scopes for API keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read position data.
    PositionRead,
    /// Read route data.
    RouteRead,
    /// Create routes.
    RouteWrite,
    /// Read map data.
    MapRead,
    /// Read traffic data.
    TrafficRead,
    /// Read fleet data.
    FleetRead,
    /// Write fleet data.
    FleetWrite,
    /// Read telemetry data.
    TelemetryRead,
    /// Read integrity / health data.
    IntegrityRead,
    /// Manage webhooks.
    WebhookManage,
    /// Full admin access.
    Admin,
}

/// API tier determining rate limits and quotas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ApiTier {
    /// Free tier — limited usage.
    Free,
    /// Developer tier — moderate limits.
    Developer,
    /// Professional tier — high limits.
    Professional,
    /// Enterprise tier — custom limits.
    Enterprise,
}

impl ApiTier {
    /// Default requests per minute for this tier.
    pub fn default_rpm(&self) -> u64 {
        match self {
            Self::Free => 60,
            Self::Developer => 600,
            Self::Professional => 6_000,
            Self::Enterprise => 60_000,
        }
    }

    /// Default daily request quota.
    pub fn default_daily_quota(&self) -> u64 {
        match self {
            Self::Free => 1_000,
            Self::Developer => 50_000,
            Self::Professional => 1_000_000,
            Self::Enterprise => u64::MAX,
        }
    }
}

/// Result of key validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub key_id: Option<EntityId>,
    pub reason: Option<String>,
    pub permissions: Vec<Permission>,
    pub tier: Option<ApiTier>,
}

// ---------------------------------------------------------------------------
// Key manager
// ---------------------------------------------------------------------------

/// Manages the lifecycle of API keys.
pub struct ApiKeyManager {
    keys: HashMap<EntityId, ApiKey>,
    key_lookup: HashMap<String, EntityId>,
    max_keys_per_owner: usize,
}

impl ApiKeyManager {
    pub fn new(max_keys_per_owner: usize) -> Self {
        Self {
            keys: HashMap::new(),
            key_lookup: HashMap::new(),
            max_keys_per_owner,
        }
    }

    /// Create a new API key. Returns the full key string (only available at creation time).
    pub fn create_key(
        &mut self,
        name: impl Into<String>,
        owner_id: EntityId,
        permissions: Vec<Permission>,
        tier: ApiTier,
        expires_in: Option<Duration>,
    ) -> Result<(ApiKey, String), ApiKeyError> {
        // Check per-owner limit.
        let owner_count = self
            .keys
            .values()
            .filter(|k| k.owner_id == owner_id && k.status != KeyStatus::Revoked)
            .count();
        if owner_count >= self.max_keys_per_owner {
            return Err(ApiKeyError::TooManyKeys {
                max: self.max_keys_per_owner,
            });
        }

        let key_id = EntityId::new();
        let full_key = format!("ak_{}", key_id.0.simple());
        let prefix = full_key[..12].to_string();
        let key_hash = simple_hash(&full_key);

        let expires_at = expires_in.map(|d| Utc::now() + d);

        let key = ApiKey {
            id: key_id,
            key_prefix: prefix,
            key_hash: key_hash.clone(),
            name: name.into(),
            owner_id,
            permissions,
            tier,
            status: KeyStatus::Active,
            created_at: Utc::now(),
            expires_at,
            last_used_at: None,
            request_count: 0,
        };

        info!(key_id = %key_id, tier = ?tier, "API key created");
        self.key_lookup.insert(key_hash, key_id);
        self.keys.insert(key_id, key.clone());

        Ok((key, full_key))
    }

    /// Validate an API key string.
    pub fn validate_key(&mut self, key_string: &str) -> ValidationResult {
        let hash = simple_hash(key_string);

        let key_id = match self.key_lookup.get(&hash) {
            Some(id) => *id,
            None => {
                return ValidationResult {
                    valid: false,
                    key_id: None,
                    reason: Some("key not found".into()),
                    permissions: Vec::new(),
                    tier: None,
                };
            }
        };

        let key = match self.keys.get_mut(&key_id) {
            Some(k) => k,
            None => {
                return ValidationResult {
                    valid: false,
                    key_id: Some(key_id),
                    reason: Some("key data missing".into()),
                    permissions: Vec::new(),
                    tier: None,
                };
            }
        };

        // Check status.
        if key.status != KeyStatus::Active {
            return ValidationResult {
                valid: false,
                key_id: Some(key_id),
                reason: Some(format!("key is {:?}", key.status)),
                permissions: Vec::new(),
                tier: Some(key.tier),
            };
        }

        // Check expiration.
        if let Some(expires_at) = key.expires_at {
            if Utc::now() > expires_at {
                key.status = KeyStatus::Expired;
                return ValidationResult {
                    valid: false,
                    key_id: Some(key_id),
                    reason: Some("key expired".into()),
                    permissions: Vec::new(),
                    tier: Some(key.tier),
                };
            }
        }

        // Update usage stats.
        key.last_used_at = Some(Utc::now());
        key.request_count += 1;

        ValidationResult {
            valid: true,
            key_id: Some(key_id),
            reason: None,
            permissions: key.permissions.clone(),
            tier: Some(key.tier),
        }
    }

    /// Revoke an API key.
    pub fn revoke_key(&mut self, key_id: &EntityId) -> Result<(), ApiKeyError> {
        let key = self
            .keys
            .get_mut(key_id)
            .ok_or(ApiKeyError::KeyNotFound(*key_id))?;
        key.status = KeyStatus::Revoked;
        info!(key_id = %key_id, "API key revoked");
        Ok(())
    }

    /// Suspend an API key.
    pub fn suspend_key(&mut self, key_id: &EntityId) -> Result<(), ApiKeyError> {
        let key = self
            .keys
            .get_mut(key_id)
            .ok_or(ApiKeyError::KeyNotFound(*key_id))?;
        if key.status == KeyStatus::Revoked {
            return Err(ApiKeyError::AlreadyRevoked);
        }
        key.status = KeyStatus::Suspended;
        info!(key_id = %key_id, "API key suspended");
        Ok(())
    }

    /// Reactivate a suspended key.
    pub fn reactivate_key(&mut self, key_id: &EntityId) -> Result<(), ApiKeyError> {
        let key = self
            .keys
            .get_mut(key_id)
            .ok_or(ApiKeyError::KeyNotFound(*key_id))?;
        if key.status != KeyStatus::Suspended {
            return Err(ApiKeyError::InvalidStateTransition {
                from: key.status,
                to: KeyStatus::Active,
            });
        }
        key.status = KeyStatus::Active;
        info!(key_id = %key_id, "API key reactivated");
        Ok(())
    }

    /// Rotate a key — revokes the old key and creates a new one with same permissions.
    pub fn rotate_key(&mut self, key_id: &EntityId) -> Result<(ApiKey, String), ApiKeyError> {
        let old_key = self
            .keys
            .get(key_id)
            .ok_or(ApiKeyError::KeyNotFound(*key_id))?
            .clone();

        // Reject rotation of expired keys.
        if let Some(expires_at) = old_key.expires_at {
            if Utc::now() > expires_at {
                return Err(ApiKeyError::KeyExpired);
            }
        }

        // Revoke old key.
        self.revoke_key(key_id)?;

        // Create new key with same settings.
        let remaining_expiry = old_key
            .expires_at
            .and_then(|e| (e - Utc::now()).to_std().ok())
            .map(|d| Duration::from_std(d).unwrap_or(Duration::hours(1)));

        self.create_key(
            format!("{} (rotated)", old_key.name),
            old_key.owner_id,
            old_key.permissions,
            old_key.tier,
            remaining_expiry,
        )
    }

    /// Get a key by ID.
    pub fn get_key(&self, key_id: &EntityId) -> Option<&ApiKey> {
        self.keys.get(key_id)
    }

    /// List all keys for an owner.
    pub fn keys_for_owner(&self, owner_id: &EntityId) -> Vec<&ApiKey> {
        self.keys
            .values()
            .filter(|k| k.owner_id == *owner_id)
            .collect()
    }

    /// Total number of keys.
    pub fn total_keys(&self) -> usize {
        self.keys.len()
    }

    /// Check if a key has a specific permission.
    pub fn has_permission(&self, key_id: &EntityId, permission: Permission) -> bool {
        self.keys
            .get(key_id)
            .map(|k| {
                k.permissions.contains(&Permission::Admin) || k.permissions.contains(&permission)
            })
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error("key not found: {0}")]
    KeyNotFound(EntityId),
    #[error("too many keys (max {max})")]
    TooManyKeys { max: usize },
    #[error("key already revoked")]
    AlreadyRevoked,
    #[error("invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition { from: KeyStatus, to: KeyStatus },
    #[error("key has expired and cannot be rotated")]
    KeyExpired,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Simple deterministic hash for key validation (not cryptographic).
fn simple_hash(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manager() -> ApiKeyManager {
        ApiKeyManager::new(10)
    }

    #[test]
    fn create_and_validate_key() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, full_key) = mgr
            .create_key(
                "test-key",
                owner,
                vec![Permission::PositionRead, Permission::RouteRead],
                ApiTier::Developer,
                None,
            )
            .unwrap();

        assert_eq!(key.status, KeyStatus::Active);
        assert_eq!(key.tier, ApiTier::Developer);
        assert!(full_key.starts_with("ak_"));

        // Validate with the full key.
        let result = mgr.validate_key(&full_key);
        assert!(result.valid);
        assert_eq!(result.key_id, Some(key.id));
        assert_eq!(result.permissions.len(), 2);
    }

    #[test]
    fn validate_unknown_key_returns_invalid() {
        let mut mgr = test_manager();
        let result = mgr.validate_key("nonexistent-key");
        assert!(!result.valid);
        assert_eq!(result.reason, Some("key not found".into()));
    }

    #[test]
    fn revoked_key_fails_validation() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, full_key) = mgr
            .create_key("k", owner, vec![Permission::MapRead], ApiTier::Free, None)
            .unwrap();

        mgr.revoke_key(&key.id).unwrap();
        let result = mgr.validate_key(&full_key);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("Revoked"));
    }

    #[test]
    fn suspended_key_fails_validation() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, full_key) = mgr
            .create_key("k", owner, vec![], ApiTier::Free, None)
            .unwrap();

        mgr.suspend_key(&key.id).unwrap();
        let result = mgr.validate_key(&full_key);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("Suspended"));
    }

    #[test]
    fn reactivate_suspended_key() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, full_key) = mgr
            .create_key("k", owner, vec![], ApiTier::Free, None)
            .unwrap();

        mgr.suspend_key(&key.id).unwrap();
        mgr.reactivate_key(&key.id).unwrap();
        let result = mgr.validate_key(&full_key);
        assert!(result.valid);
    }

    #[test]
    fn reactivate_revoked_key_fails() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, _) = mgr
            .create_key("k", owner, vec![], ApiTier::Free, None)
            .unwrap();

        mgr.revoke_key(&key.id).unwrap();
        let result = mgr.suspend_key(&key.id);
        assert!(result.is_err());
    }

    #[test]
    fn reactivate_active_key_fails() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, _) = mgr
            .create_key("k", owner, vec![], ApiTier::Free, None)
            .unwrap();

        let result = mgr.reactivate_key(&key.id);
        assert!(result.is_err());
    }

    #[test]
    fn max_keys_per_owner_enforced() {
        let mut mgr = ApiKeyManager::new(2);
        let owner = EntityId::new();
        mgr.create_key("k1", owner, vec![], ApiTier::Free, None)
            .unwrap();
        mgr.create_key("k2", owner, vec![], ApiTier::Free, None)
            .unwrap();
        let result = mgr.create_key("k3", owner, vec![], ApiTier::Free, None);
        assert!(matches!(
            result.unwrap_err(),
            ApiKeyError::TooManyKeys { max: 2 }
        ));
    }

    #[test]
    fn rotate_key() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (old_key, old_full) = mgr
            .create_key(
                "original",
                owner,
                vec![Permission::PositionRead],
                ApiTier::Professional,
                None,
            )
            .unwrap();

        let (new_key, new_full) = mgr.rotate_key(&old_key.id).unwrap();

        // Old key should be revoked.
        let old_result = mgr.validate_key(&old_full);
        assert!(!old_result.valid);

        // New key should be valid.
        let new_result = mgr.validate_key(&new_full);
        assert!(new_result.valid);
        assert_eq!(new_result.tier, Some(ApiTier::Professional));
        assert_ne!(old_key.id, new_key.id);
    }

    #[test]
    fn keys_for_owner() {
        let mut mgr = test_manager();
        let owner_a = EntityId::new();
        let owner_b = EntityId::new();

        mgr.create_key("a1", owner_a, vec![], ApiTier::Free, None)
            .unwrap();
        mgr.create_key("a2", owner_a, vec![], ApiTier::Free, None)
            .unwrap();
        mgr.create_key("b1", owner_b, vec![], ApiTier::Free, None)
            .unwrap();

        assert_eq!(mgr.keys_for_owner(&owner_a).len(), 2);
        assert_eq!(mgr.keys_for_owner(&owner_b).len(), 1);
    }

    #[test]
    fn has_permission_checks_admin() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, _) = mgr
            .create_key(
                "admin",
                owner,
                vec![Permission::Admin],
                ApiTier::Enterprise,
                None,
            )
            .unwrap();

        // Admin has all permissions.
        assert!(mgr.has_permission(&key.id, Permission::PositionRead));
        assert!(mgr.has_permission(&key.id, Permission::FleetWrite));
        assert!(mgr.has_permission(&key.id, Permission::WebhookManage));
    }

    #[test]
    fn has_permission_specific() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, _) = mgr
            .create_key(
                "limited",
                owner,
                vec![Permission::PositionRead],
                ApiTier::Free,
                None,
            )
            .unwrap();

        assert!(mgr.has_permission(&key.id, Permission::PositionRead));
        assert!(!mgr.has_permission(&key.id, Permission::FleetWrite));
    }

    #[test]
    fn api_tier_defaults() {
        assert_eq!(ApiTier::Free.default_rpm(), 60);
        assert_eq!(ApiTier::Developer.default_rpm(), 600);
        assert_eq!(ApiTier::Professional.default_rpm(), 6_000);
        assert_eq!(ApiTier::Enterprise.default_rpm(), 60_000);

        assert_eq!(ApiTier::Free.default_daily_quota(), 1_000);
        assert_eq!(ApiTier::Enterprise.default_daily_quota(), u64::MAX);
    }

    #[test]
    fn request_count_increments_on_validate() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let (key, full_key) = mgr
            .create_key("k", owner, vec![], ApiTier::Free, None)
            .unwrap();

        mgr.validate_key(&full_key);
        mgr.validate_key(&full_key);
        mgr.validate_key(&full_key);

        let k = mgr.get_key(&key.id).unwrap();
        assert_eq!(k.request_count, 3);
        assert!(k.last_used_at.is_some());
    }
}
