//! Role-Based Access Control (RBAC).
//!
//! Defines roles, permissions, and resource-level access policies for the
//! AURORA NAV system. Supports hierarchical roles where higher-privilege
//! roles inherit permissions from lower ones.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum RbacError {
    #[error("role not found: {0}")]
    RoleNotFound(String),
    #[error("permission denied: {role} cannot {action} on {resource}")]
    PermissionDenied {
        role: String,
        action: String,
        resource: String,
    },
    #[error("duplicate role: {0}")]
    DuplicateRole(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A named permission (action on a resource).
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    /// The resource being accessed (e.g., "position", "fleet", "config").
    pub resource: String,
    /// The action being performed (e.g., "read", "write", "admin").
    pub action: String,
}

impl Permission {
    pub fn new(resource: &str, action: &str) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}

/// Built-in system roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemRole {
    /// Read-only access to public endpoints.
    Viewer,
    /// Read/write access to navigation data.
    Operator,
    /// Fleet management capabilities.
    FleetManager,
    /// Full system administration.
    Admin,
    /// Machine-to-machine service account.
    Service,
}

impl SystemRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemRole::Viewer => "viewer",
            SystemRole::Operator => "operator",
            SystemRole::FleetManager => "fleet_manager",
            SystemRole::Admin => "admin",
            SystemRole::Service => "service",
        }
    }

    pub fn parse_role(s: &str) -> Option<Self> {
        match s {
            "viewer" => Some(SystemRole::Viewer),
            "operator" => Some(SystemRole::Operator),
            "fleet_manager" => Some(SystemRole::FleetManager),
            "admin" => Some(SystemRole::Admin),
            "service" => Some(SystemRole::Service),
            _ => None,
        }
    }

    /// Privilege level (higher = more permissions).
    pub fn privilege_level(&self) -> u8 {
        match self {
            SystemRole::Viewer => 1,
            SystemRole::Operator => 2,
            SystemRole::FleetManager => 3,
            SystemRole::Service => 3,
            SystemRole::Admin => 4,
        }
    }
}

/// Role definition with associated permissions.
#[derive(Debug, Clone)]
pub struct RoleDefinition {
    pub name: String,
    pub permissions: HashSet<Permission>,
    pub privilege_level: u8,
}

// ---------------------------------------------------------------------------
// Policy engine
// ---------------------------------------------------------------------------

/// RBAC policy engine that manages roles and checks permissions.
pub struct PolicyEngine {
    roles: HashMap<String, RoleDefinition>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            roles: HashMap::new(),
        };
        engine.register_default_roles();
        engine
    }

    /// Register the built-in AURORA NAV roles.
    fn register_default_roles(&mut self) {
        // Viewer: read-only
        let viewer_perms: HashSet<Permission> = [
            Permission::new("health", "read"),
            Permission::new("position", "read"),
            Permission::new("integrity", "read"),
            Permission::new("status", "read"),
            Permission::new("metrics", "read"),
        ]
        .into_iter()
        .collect();

        self.roles.insert(
            "viewer".to_string(),
            RoleDefinition {
                name: "viewer".to_string(),
                permissions: viewer_perms,
                privilege_level: 1,
            },
        );

        // Operator: viewer + write nav data
        let mut operator_perms = self.roles["viewer"].permissions.clone();
        operator_perms.insert(Permission::new("position", "write"));
        operator_perms.insert(Permission::new("routing", "read"));
        operator_perms.insert(Permission::new("routing", "write"));
        operator_perms.insert(Permission::new("telemetry", "read"));
        operator_perms.insert(Permission::new("constellation", "read"));

        self.roles.insert(
            "operator".to_string(),
            RoleDefinition {
                name: "operator".to_string(),
                permissions: operator_perms,
                privilege_level: 2,
            },
        );

        // Fleet manager: operator + fleet
        let mut fleet_perms = self.roles["operator"].permissions.clone();
        fleet_perms.insert(Permission::new("fleet", "read"));
        fleet_perms.insert(Permission::new("fleet", "write"));
        fleet_perms.insert(Permission::new("fleet", "admin"));
        fleet_perms.insert(Permission::new("emergency", "read"));
        fleet_perms.insert(Permission::new("emergency", "write"));

        self.roles.insert(
            "fleet_manager".to_string(),
            RoleDefinition {
                name: "fleet_manager".to_string(),
                permissions: fleet_perms,
                privilege_level: 3,
            },
        );

        // Service: operator + metrics + probes
        let mut service_perms = self.roles["operator"].permissions.clone();
        service_perms.insert(Permission::new("metrics", "read"));
        service_perms.insert(Permission::new("probes", "read"));
        service_perms.insert(Permission::new("openapi", "read"));

        self.roles.insert(
            "service".to_string(),
            RoleDefinition {
                name: "service".to_string(),
                permissions: service_perms,
                privilege_level: 3,
            },
        );

        // Admin: everything
        let mut admin_perms = self.roles["fleet_manager"].permissions.clone();
        admin_perms.extend(self.roles["service"].permissions.clone());
        admin_perms.insert(Permission::new("config", "read"));
        admin_perms.insert(Permission::new("config", "write"));
        admin_perms.insert(Permission::new("auth", "admin"));
        admin_perms.insert(Permission::new("system", "admin"));

        self.roles.insert(
            "admin".to_string(),
            RoleDefinition {
                name: "admin".to_string(),
                permissions: admin_perms,
                privilege_level: 4,
            },
        );
    }

    /// Register a custom role.
    pub fn register_role(
        &mut self,
        name: &str,
        permissions: HashSet<Permission>,
        privilege_level: u8,
    ) -> Result<(), RbacError> {
        if self.roles.contains_key(name) {
            return Err(RbacError::DuplicateRole(name.to_string()));
        }
        self.roles.insert(
            name.to_string(),
            RoleDefinition {
                name: name.to_string(),
                permissions,
                privilege_level,
            },
        );
        Ok(())
    }

    /// Check if a role has a specific permission.
    pub fn check_permission(
        &self,
        role: &str,
        resource: &str,
        action: &str,
    ) -> Result<(), RbacError> {
        let role_def = self
            .roles
            .get(role)
            .ok_or_else(|| RbacError::RoleNotFound(role.to_string()))?;

        let perm = Permission::new(resource, action);
        if role_def.permissions.contains(&perm) {
            Ok(())
        } else {
            Err(RbacError::PermissionDenied {
                role: role.to_string(),
                action: action.to_string(),
                resource: resource.to_string(),
            })
        }
    }

    /// Get all permissions for a role.
    pub fn role_permissions(&self, role: &str) -> Result<&HashSet<Permission>, RbacError> {
        let role_def = self
            .roles
            .get(role)
            .ok_or_else(|| RbacError::RoleNotFound(role.to_string()))?;
        Ok(&role_def.permissions)
    }

    /// Check if role_a has higher or equal privilege than role_b.
    pub fn has_higher_privilege(&self, role_a: &str, role_b: &str) -> Result<bool, RbacError> {
        let a = self
            .roles
            .get(role_a)
            .ok_or_else(|| RbacError::RoleNotFound(role_a.to_string()))?;
        let b = self
            .roles
            .get(role_b)
            .ok_or_else(|| RbacError::RoleNotFound(role_b.to_string()))?;
        Ok(a.privilege_level >= b.privilege_level)
    }

    /// List all registered role names.
    pub fn role_names(&self) -> Vec<String> {
        self.roles.keys().cloned().collect()
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_can_read_health() {
        let engine = PolicyEngine::new();
        assert!(engine.check_permission("viewer", "health", "read").is_ok());
    }

    #[test]
    fn viewer_cannot_write_position() {
        let engine = PolicyEngine::new();
        assert!(matches!(
            engine.check_permission("viewer", "position", "write"),
            Err(RbacError::PermissionDenied { .. })
        ));
    }

    #[test]
    fn operator_inherits_viewer_perms() {
        let engine = PolicyEngine::new();
        assert!(engine
            .check_permission("operator", "health", "read")
            .is_ok());
        assert!(engine
            .check_permission("operator", "position", "write")
            .is_ok());
    }

    #[test]
    fn fleet_manager_can_manage_fleet() {
        let engine = PolicyEngine::new();
        assert!(engine
            .check_permission("fleet_manager", "fleet", "admin")
            .is_ok());
    }

    #[test]
    fn admin_can_do_everything() {
        let engine = PolicyEngine::new();
        assert!(engine.check_permission("admin", "health", "read").is_ok());
        assert!(engine.check_permission("admin", "config", "write").is_ok());
        assert!(engine.check_permission("admin", "auth", "admin").is_ok());
        assert!(engine.check_permission("admin", "fleet", "admin").is_ok());
    }

    #[test]
    fn unknown_role_rejected() {
        let engine = PolicyEngine::new();
        assert!(matches!(
            engine.check_permission("hacker", "system", "admin"),
            Err(RbacError::RoleNotFound(_))
        ));
    }

    #[test]
    fn privilege_hierarchy() {
        let engine = PolicyEngine::new();
        assert!(engine.has_higher_privilege("admin", "viewer").unwrap());
        assert!(engine.has_higher_privilege("admin", "operator").unwrap());
        assert!(!engine.has_higher_privilege("viewer", "admin").unwrap());
        assert!(engine.has_higher_privilege("admin", "admin").unwrap());
    }

    #[test]
    fn custom_role_registration() {
        let mut engine = PolicyEngine::new();
        let perms: HashSet<Permission> = [Permission::new("custom", "read")].into_iter().collect();
        engine.register_role("custom_role", perms, 1).unwrap();
        assert!(engine
            .check_permission("custom_role", "custom", "read")
            .is_ok());
    }

    #[test]
    fn duplicate_role_rejected() {
        let mut engine = PolicyEngine::new();
        let perms: HashSet<Permission> = HashSet::new();
        assert!(matches!(
            engine.register_role("viewer", perms, 1),
            Err(RbacError::DuplicateRole(_))
        ));
    }

    #[test]
    fn system_role_string_roundtrip() {
        for role in &[
            SystemRole::Viewer,
            SystemRole::Operator,
            SystemRole::FleetManager,
            SystemRole::Admin,
            SystemRole::Service,
        ] {
            let s = role.as_str();
            let parsed = SystemRole::parse_role(s).unwrap();
            assert_eq!(*role, parsed);
        }
    }

    #[test]
    fn role_names_includes_defaults() {
        let engine = PolicyEngine::new();
        let names = engine.role_names();
        assert!(names.contains(&"viewer".to_string()));
        assert!(names.contains(&"admin".to_string()));
        assert!(names.contains(&"operator".to_string()));
        assert_eq!(names.len(), 5);
    }

    #[test]
    fn role_permissions_returns_correct_set() {
        let engine = PolicyEngine::new();
        let perms = engine.role_permissions("viewer").unwrap();
        assert!(perms.contains(&Permission::new("health", "read")));
        assert!(!perms.contains(&Permission::new("config", "write")));
    }
}
