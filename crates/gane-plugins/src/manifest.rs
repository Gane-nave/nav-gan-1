//! Plugin manifest — declarative plugin configuration and dependency resolution.

use std::collections::HashMap;

/// Version constraint for dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Exact version match.
    Exact(String),
    /// Minimum version (inclusive).
    AtLeast(String),
    /// Version range [min, max).
    Range(String, String),
    /// Any version.
    Any,
}

/// Plugin dependency declaration.
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Plugin ID this depends on.
    pub plugin_id: String,
    /// Version constraint.
    pub version: VersionConstraint,
    /// Whether this dependency is optional.
    pub optional: bool,
}

/// Plugin manifest — describes a plugin's metadata and requirements.
#[derive(Debug, Clone)]
pub struct PluginManifest {
    /// Plugin identifier.
    pub id: String,
    /// Plugin version.
    pub version: String,
    /// Minimum Aurora system version required.
    pub min_system_version: String,
    /// Plugin dependencies.
    pub dependencies: Vec<Dependency>,
    /// Provided capabilities.
    pub provides: Vec<String>,
    /// Required system permissions.
    pub permissions: Vec<String>,
    /// Configuration schema (key → type description).
    pub config_schema: HashMap<String, String>,
}

/// Dependency resolver — checks if all dependencies can be satisfied.
pub struct DependencyResolver {
    available: HashMap<String, String>, // plugin_id → version
}

impl DependencyResolver {
    /// Create a new resolver.
    pub fn new() -> Self {
        Self {
            available: HashMap::new(),
        }
    }

    /// Register an available plugin version.
    pub fn register_available(&mut self, plugin_id: &str, version: &str) {
        self.available
            .insert(plugin_id.to_string(), version.to_string());
    }

    /// Check if a dependency is satisfied.
    pub fn is_satisfied(&self, dep: &Dependency) -> bool {
        if dep.optional {
            return true; // optional dependencies are always "satisfied"
        }
        let Some(available_version) = self.available.get(&dep.plugin_id) else {
            return false;
        };
        match &dep.version {
            VersionConstraint::Any => true,
            VersionConstraint::Exact(v) => available_version == v,
            VersionConstraint::AtLeast(min) => available_version.as_str() >= min.as_str(),
            VersionConstraint::Range(min, max) => {
                available_version.as_str() >= min.as_str()
                    && available_version.as_str() < max.as_str()
            }
        }
    }

    /// Resolve all dependencies for a manifest.
    pub fn resolve(&self, manifest: &PluginManifest) -> ResolutionResult {
        let mut unsatisfied = Vec::new();
        let mut satisfied = Vec::new();
        for dep in &manifest.dependencies {
            if self.is_satisfied(dep) {
                satisfied.push(dep.plugin_id.clone());
            } else {
                unsatisfied.push(dep.plugin_id.clone());
            }
        }
        ResolutionResult {
            plugin_id: manifest.id.clone(),
            all_satisfied: unsatisfied.is_empty(),
            satisfied,
            unsatisfied,
        }
    }

    /// Compute install order for multiple manifests (topological sort).
    pub fn install_order(&self, manifests: &[PluginManifest]) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

        for m in manifests {
            in_degree.entry(m.id.clone()).or_insert(0);
            for dep in &m.dependencies {
                if !dep.optional {
                    *in_degree.entry(m.id.clone()).or_insert(0) += 1;
                    dependents
                        .entry(dep.plugin_id.clone())
                        .or_default()
                        .push(m.id.clone());
                }
            }
        }

        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(id, _)| id.clone())
            .collect();
        queue.sort(); // deterministic order

        let mut order = Vec::new();
        while let Some(id) = queue.pop() {
            order.push(id.clone());
            if let Some(deps) = dependents.get(&id) {
                for dep_id in deps {
                    if let Some(deg) = in_degree.get_mut(dep_id) {
                        *deg = deg.saturating_sub(1);
                        if *deg == 0 {
                            queue.push(dep_id.clone());
                            queue.sort();
                        }
                    }
                }
            }
        }

        if order.len() < manifests.len() {
            return Err("Circular dependency detected".to_string());
        }

        Ok(order)
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of dependency resolution.
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    /// Plugin being resolved.
    pub plugin_id: String,
    /// Whether all dependencies are satisfied.
    pub all_satisfied: bool,
    /// Satisfied dependency IDs.
    pub satisfied: Vec<String>,
    /// Unsatisfied dependency IDs.
    pub unsatisfied: Vec<String>,
}

/// Validate a plugin manifest.
pub fn validate_manifest(manifest: &PluginManifest) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.id.is_empty() {
        errors.push("Plugin ID cannot be empty".to_string());
    }
    if manifest.version.is_empty() {
        errors.push("Plugin version cannot be empty".to_string());
    }
    if manifest.id.contains(' ') {
        errors.push("Plugin ID cannot contain spaces".to_string());
    }
    // Check for self-dependency
    for dep in &manifest.dependencies {
        if dep.plugin_id == manifest.id {
            errors.push("Plugin cannot depend on itself".to_string());
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manifest(id: &str, deps: Vec<Dependency>) -> PluginManifest {
        PluginManifest {
            id: id.to_string(),
            version: "1.0.0".to_string(),
            min_system_version: "0.1.0".to_string(),
            dependencies: deps,
            provides: vec![],
            permissions: vec![],
            config_schema: HashMap::new(),
        }
    }

    #[test]
    fn test_resolve_all_satisfied() {
        let mut resolver = DependencyResolver::new();
        resolver.register_available("dep1", "1.0.0");
        resolver.register_available("dep2", "2.0.0");

        let manifest = make_manifest(
            "my-plugin",
            vec![
                Dependency {
                    plugin_id: "dep1".to_string(),
                    version: VersionConstraint::Any,
                    optional: false,
                },
                Dependency {
                    plugin_id: "dep2".to_string(),
                    version: VersionConstraint::Exact("2.0.0".to_string()),
                    optional: false,
                },
            ],
        );
        let result = resolver.resolve(&manifest);
        assert!(result.all_satisfied);
        assert_eq!(result.satisfied.len(), 2);
        assert!(result.unsatisfied.is_empty());
    }

    #[test]
    fn test_resolve_missing_dependency() {
        let resolver = DependencyResolver::new();
        let manifest = make_manifest(
            "my-plugin",
            vec![Dependency {
                plugin_id: "missing".to_string(),
                version: VersionConstraint::Any,
                optional: false,
            }],
        );
        let result = resolver.resolve(&manifest);
        assert!(!result.all_satisfied);
        assert_eq!(result.unsatisfied, vec!["missing"]);
    }

    #[test]
    fn test_optional_dependency_always_satisfied() {
        let resolver = DependencyResolver::new();
        let manifest = make_manifest(
            "my-plugin",
            vec![Dependency {
                plugin_id: "optional-dep".to_string(),
                version: VersionConstraint::Any,
                optional: true,
            }],
        );
        let result = resolver.resolve(&manifest);
        assert!(result.all_satisfied);
    }

    #[test]
    fn test_version_constraint_exact() {
        let mut resolver = DependencyResolver::new();
        resolver.register_available("dep", "1.0.0");
        let dep_match = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::Exact("1.0.0".to_string()),
            optional: false,
        };
        let dep_no_match = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::Exact("2.0.0".to_string()),
            optional: false,
        };
        assert!(resolver.is_satisfied(&dep_match));
        assert!(!resolver.is_satisfied(&dep_no_match));
    }

    #[test]
    fn test_version_constraint_at_least() {
        let mut resolver = DependencyResolver::new();
        resolver.register_available("dep", "2.0.0");
        let ok = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::AtLeast("1.0.0".to_string()),
            optional: false,
        };
        let too_high = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::AtLeast("3.0.0".to_string()),
            optional: false,
        };
        assert!(resolver.is_satisfied(&ok));
        assert!(!resolver.is_satisfied(&too_high));
    }

    #[test]
    fn test_version_constraint_range() {
        let mut resolver = DependencyResolver::new();
        resolver.register_available("dep", "1.5.0");
        let in_range = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::Range("1.0.0".to_string(), "2.0.0".to_string()),
            optional: false,
        };
        let out_of_range = Dependency {
            plugin_id: "dep".to_string(),
            version: VersionConstraint::Range("2.0.0".to_string(), "3.0.0".to_string()),
            optional: false,
        };
        assert!(resolver.is_satisfied(&in_range));
        assert!(!resolver.is_satisfied(&out_of_range));
    }

    #[test]
    fn test_install_order() {
        let resolver = DependencyResolver::new();
        let m1 = make_manifest("core", vec![]);
        let m2 = make_manifest(
            "routing",
            vec![Dependency {
                plugin_id: "core".to_string(),
                version: VersionConstraint::Any,
                optional: false,
            }],
        );
        let m3 = make_manifest(
            "ui",
            vec![Dependency {
                plugin_id: "routing".to_string(),
                version: VersionConstraint::Any,
                optional: false,
            }],
        );
        let order = resolver.install_order(&[m1, m2, m3]).unwrap();
        let core_idx = order.iter().position(|x| x == "core").unwrap();
        let routing_idx = order.iter().position(|x| x == "routing").unwrap();
        let ui_idx = order.iter().position(|x| x == "ui").unwrap();
        assert!(core_idx < routing_idx);
        assert!(routing_idx < ui_idx);
    }

    #[test]
    fn test_validate_manifest_valid() {
        let m = make_manifest("my-plugin", vec![]);
        assert!(validate_manifest(&m).is_empty());
    }

    #[test]
    fn test_validate_manifest_empty_id() {
        let m = make_manifest("", vec![]);
        let errs = validate_manifest(&m);
        assert!(errs.iter().any(|e| e.contains("ID cannot be empty")));
    }

    #[test]
    fn test_validate_manifest_self_dependency() {
        let m = make_manifest(
            "self-dep",
            vec![Dependency {
                plugin_id: "self-dep".to_string(),
                version: VersionConstraint::Any,
                optional: false,
            }],
        );
        let errs = validate_manifest(&m);
        assert!(errs.iter().any(|e| e.contains("cannot depend on itself")));
    }

    #[test]
    fn test_validate_manifest_spaces_in_id() {
        let m = make_manifest("my plugin", vec![]);
        let errs = validate_manifest(&m);
        assert!(errs.iter().any(|e| e.contains("cannot contain spaces")));
    }
}
