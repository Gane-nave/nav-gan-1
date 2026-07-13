//! Installation management — tracking installed plugins per user,
//! version updates, enable/disable, and uninstallation.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Installation types
// ---------------------------------------------------------------------------

/// An installed plugin instance for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installation {
    pub id: EntityId,
    pub listing_id: EntityId,
    pub user_id: EntityId,
    pub installed_version: String,
    pub status: InstallationStatus,
    pub auto_update: bool,
    pub installed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Installation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallationStatus {
    /// Actively running.
    Active,
    /// Installed but disabled by user.
    Disabled,
    /// Update available.
    UpdateAvailable,
    /// Currently updating.
    Updating,
    /// Installation failed.
    Failed,
    /// Uninstalled (kept for history).
    Uninstalled,
}

/// Update check result.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub listing_id: EntityId,
    pub current_version: String,
    pub available_version: String,
    pub changelog: String,
}

// ---------------------------------------------------------------------------
// Installation manager
// ---------------------------------------------------------------------------

/// Manages plugin installations per user.
pub struct InstallationManager {
    installations: HashMap<EntityId, Installation>,
    /// (user_id, listing_id) → installation_id for deduplication.
    user_installs: HashMap<(EntityId, EntityId), EntityId>,
    max_per_user: usize,
}

impl InstallationManager {
    pub fn new(max_per_user: usize) -> Self {
        Self {
            installations: HashMap::new(),
            user_installs: HashMap::new(),
            max_per_user,
        }
    }

    /// Install a plugin for a user.
    pub fn install(
        &mut self,
        listing_id: EntityId,
        user_id: EntityId,
        version: impl Into<String>,
    ) -> Result<Installation, InstallationError> {
        let key = (user_id, listing_id);
        if self.user_installs.contains_key(&key) {
            return Err(InstallationError::AlreadyInstalled {
                user_id,
                listing_id,
            });
        }

        // Count active installations (not Uninstalled) for the user.
        let active_count = self
            .installations
            .values()
            .filter(|i| i.user_id == user_id && i.status != InstallationStatus::Uninstalled)
            .count();

        if active_count >= self.max_per_user {
            return Err(InstallationError::TooManyInstallations {
                max: self.max_per_user,
            });
        }

        let now = Utc::now();
        let installation = Installation {
            id: EntityId::new(),
            listing_id,
            user_id,
            installed_version: version.into(),
            status: InstallationStatus::Active,
            auto_update: true,
            installed_at: now,
            updated_at: now,
        };

        info!(install_id = %installation.id, listing = %listing_id, user = %user_id, "plugin installed");
        self.user_installs.insert(key, installation.id);
        let result = installation.clone();
        self.installations.insert(installation.id, installation);
        Ok(result)
    }

    /// Uninstall a plugin.
    pub fn uninstall(&mut self, install_id: &EntityId) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;

        if installation.status == InstallationStatus::Uninstalled {
            return Err(InstallationError::AlreadyUninstalled);
        }

        installation.status = InstallationStatus::Uninstalled;
        installation.updated_at = Utc::now();

        // Remove from user_installs so re-install is possible.
        let key = (installation.user_id, installation.listing_id);
        self.user_installs.remove(&key);

        info!(install_id = %install_id, "plugin uninstalled");
        Ok(())
    }

    /// Enable a disabled installation.
    pub fn enable(&mut self, install_id: &EntityId) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;

        if installation.status != InstallationStatus::Disabled {
            return Err(InstallationError::InvalidTransition {
                from: installation.status,
                to: InstallationStatus::Active,
            });
        }

        installation.status = InstallationStatus::Active;
        installation.updated_at = Utc::now();
        Ok(())
    }

    /// Disable an active installation.
    pub fn disable(&mut self, install_id: &EntityId) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;

        if installation.status != InstallationStatus::Active
            && installation.status != InstallationStatus::UpdateAvailable
        {
            return Err(InstallationError::InvalidTransition {
                from: installation.status,
                to: InstallationStatus::Disabled,
            });
        }

        installation.status = InstallationStatus::Disabled;
        installation.updated_at = Utc::now();
        Ok(())
    }

    /// Mark an update as available for an installation.
    pub fn mark_update_available(
        &mut self,
        install_id: &EntityId,
    ) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;

        if installation.status == InstallationStatus::Uninstalled {
            return Err(InstallationError::AlreadyUninstalled);
        }

        installation.status = InstallationStatus::UpdateAvailable;
        installation.updated_at = Utc::now();
        Ok(())
    }

    /// Apply an update to an installation.
    pub fn apply_update(
        &mut self,
        install_id: &EntityId,
        new_version: impl Into<String>,
    ) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;

        installation.installed_version = new_version.into();
        installation.status = InstallationStatus::Active;
        installation.updated_at = Utc::now();
        Ok(())
    }

    /// Toggle auto-update.
    pub fn set_auto_update(
        &mut self,
        install_id: &EntityId,
        enabled: bool,
    ) -> Result<(), InstallationError> {
        let installation = self
            .installations
            .get_mut(install_id)
            .ok_or(InstallationError::NotFound(*install_id))?;
        installation.auto_update = enabled;
        Ok(())
    }

    /// Get all active installations for a user.
    pub fn for_user(&self, user_id: &EntityId) -> Vec<&Installation> {
        self.installations
            .values()
            .filter(|i| i.user_id == *user_id && i.status != InstallationStatus::Uninstalled)
            .collect()
    }

    /// Get an installation by ID.
    pub fn get(&self, install_id: &EntityId) -> Option<&Installation> {
        self.installations.get(install_id)
    }

    /// Total active installations (all users).
    pub fn total_active(&self) -> usize {
        self.installations
            .values()
            .filter(|i| i.status == InstallationStatus::Active)
            .count()
    }

    /// Installations needing update (all users).
    pub fn pending_updates(&self) -> Vec<&Installation> {
        self.installations
            .values()
            .filter(|i| i.status == InstallationStatus::UpdateAvailable && i.auto_update)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum InstallationError {
    #[error("installation not found: {0}")]
    NotFound(EntityId),
    #[error("already installed for user {user_id} listing {listing_id}")]
    AlreadyInstalled {
        user_id: EntityId,
        listing_id: EntityId,
    },
    #[error("already uninstalled")]
    AlreadyUninstalled,
    #[error("too many installations (max {max})")]
    TooManyInstallations { max: usize },
    #[error("invalid transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: InstallationStatus,
        to: InstallationStatus,
    },
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mgr() -> InstallationManager {
        InstallationManager::new(10)
    }

    #[test]
    fn install_and_retrieve() {
        let mut mgr = test_mgr();
        let user_id = EntityId::new();
        let listing_id = EntityId::new();
        let inst = mgr.install(listing_id, user_id, "1.0.0").unwrap();
        assert_eq!(inst.status, InstallationStatus::Active);
        assert!(inst.auto_update);
        assert_eq!(mgr.for_user(&user_id).len(), 1);
    }

    #[test]
    fn duplicate_install_rejected() {
        let mut mgr = test_mgr();
        let user_id = EntityId::new();
        let listing_id = EntityId::new();
        mgr.install(listing_id, user_id, "1.0.0").unwrap();
        let result = mgr.install(listing_id, user_id, "1.0.0");
        assert!(matches!(
            result.unwrap_err(),
            InstallationError::AlreadyInstalled { .. }
        ));
    }

    #[test]
    fn max_installations_enforced() {
        let mut mgr = InstallationManager::new(2);
        let user_id = EntityId::new();
        mgr.install(EntityId::new(), user_id, "1.0").unwrap();
        mgr.install(EntityId::new(), user_id, "1.0").unwrap();
        let result = mgr.install(EntityId::new(), user_id, "1.0");
        assert!(matches!(
            result.unwrap_err(),
            InstallationError::TooManyInstallations { max: 2 }
        ));
    }

    #[test]
    fn uninstall_frees_slot() {
        let mut mgr = InstallationManager::new(2);
        let user_id = EntityId::new();
        let inst1 = mgr.install(EntityId::new(), user_id, "1.0").unwrap();
        mgr.install(EntityId::new(), user_id, "1.0").unwrap();
        mgr.uninstall(&inst1.id).unwrap();
        // Now we can install again because slot was freed.
        mgr.install(EntityId::new(), user_id, "1.0").unwrap();
        assert_eq!(mgr.for_user(&user_id).len(), 2);
    }

    #[test]
    fn enable_disable_cycle() {
        let mut mgr = test_mgr();
        let inst = mgr
            .install(EntityId::new(), EntityId::new(), "1.0")
            .unwrap();
        mgr.disable(&inst.id).unwrap();
        assert_eq!(
            mgr.get(&inst.id).unwrap().status,
            InstallationStatus::Disabled
        );
        mgr.enable(&inst.id).unwrap();
        assert_eq!(
            mgr.get(&inst.id).unwrap().status,
            InstallationStatus::Active
        );
    }

    #[test]
    fn enable_non_disabled_fails() {
        let mut mgr = test_mgr();
        let inst = mgr
            .install(EntityId::new(), EntityId::new(), "1.0")
            .unwrap();
        let result = mgr.enable(&inst.id);
        assert!(result.is_err());
    }

    #[test]
    fn update_flow() {
        let mut mgr = test_mgr();
        let inst = mgr
            .install(EntityId::new(), EntityId::new(), "1.0.0")
            .unwrap();
        mgr.mark_update_available(&inst.id).unwrap();
        assert_eq!(mgr.pending_updates().len(), 1);
        mgr.apply_update(&inst.id, "2.0.0").unwrap();
        assert_eq!(mgr.get(&inst.id).unwrap().installed_version, "2.0.0");
        assert_eq!(
            mgr.get(&inst.id).unwrap().status,
            InstallationStatus::Active
        );
    }

    #[test]
    fn auto_update_toggle() {
        let mut mgr = test_mgr();
        let inst = mgr
            .install(EntityId::new(), EntityId::new(), "1.0")
            .unwrap();
        mgr.mark_update_available(&inst.id).unwrap();
        assert_eq!(mgr.pending_updates().len(), 1);
        mgr.set_auto_update(&inst.id, false).unwrap();
        assert_eq!(mgr.pending_updates().len(), 0);
    }

    #[test]
    fn reinstall_after_uninstall() {
        let mut mgr = test_mgr();
        let user_id = EntityId::new();
        let listing_id = EntityId::new();
        let inst = mgr.install(listing_id, user_id, "1.0").unwrap();
        mgr.uninstall(&inst.id).unwrap();
        // Re-install same listing.
        let inst2 = mgr.install(listing_id, user_id, "2.0").unwrap();
        assert_eq!(inst2.installed_version, "2.0");
    }

    #[test]
    fn total_active() {
        let mut mgr = test_mgr();
        let i1 = mgr
            .install(EntityId::new(), EntityId::new(), "1.0")
            .unwrap();
        mgr.install(EntityId::new(), EntityId::new(), "1.0")
            .unwrap();
        mgr.disable(&i1.id).unwrap();
        assert_eq!(mgr.total_active(), 1);
    }
}
