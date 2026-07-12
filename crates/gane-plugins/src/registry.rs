//! Plugin registry — central registration, discovery, and lookup of plugins.

use std::collections::HashMap;

/// Plugin status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginStatus {
    /// Plugin is registered but not loaded.
    Registered,
    /// Plugin is loaded and ready.
    Active,
    /// Plugin is temporarily disabled.
    Disabled,
    /// Plugin failed to load or crashed.
    Failed,
    /// Plugin is being unloaded.
    Unloading,
}

/// Plugin metadata.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Unique plugin identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version string.
    pub version: String,
    /// Plugin author.
    pub author: String,
    /// Plugin description.
    pub description: String,
    /// Required capabilities.
    pub capabilities: Vec<String>,
    /// Current status.
    pub status: PluginStatus,
    /// Priority (higher = loaded first).
    pub priority: u32,
}

/// Plugin registry — manages plugin lifecycle and discovery.
pub struct PluginRegistry {
    plugins: HashMap<String, PluginInfo>,
    load_order: Vec<String>,
    max_plugins: usize,
}

impl PluginRegistry {
    /// Create a new registry with a maximum plugin count.
    pub fn new(max_plugins: usize) -> Self {
        Self {
            plugins: HashMap::new(),
            load_order: Vec::new(),
            max_plugins,
        }
    }

    /// Register a new plugin.
    pub fn register(&mut self, info: PluginInfo) -> Result<(), String> {
        if self.plugins.len() >= self.max_plugins {
            return Err(format!("Registry full: max {} plugins", self.max_plugins));
        }
        if self.plugins.contains_key(&info.id) {
            return Err(format!("Plugin '{}' already registered", info.id));
        }
        let id = info.id.clone();
        self.plugins.insert(id.clone(), info);
        self.load_order.push(id);
        Ok(())
    }

    /// Unregister a plugin.
    pub fn unregister(&mut self, id: &str) -> Result<PluginInfo, String> {
        let info = self
            .plugins
            .remove(id)
            .ok_or_else(|| format!("Plugin '{id}' not found"))?;
        self.load_order.retain(|x| x != id);
        Ok(info)
    }

    /// Get a plugin by ID.
    pub fn get(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id)
    }

    /// Get a mutable reference to a plugin.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut PluginInfo> {
        self.plugins.get_mut(id)
    }

    /// Set plugin status.
    pub fn set_status(&mut self, id: &str, status: PluginStatus) -> Result<(), String> {
        let plugin = self
            .plugins
            .get_mut(id)
            .ok_or_else(|| format!("Plugin '{id}' not found"))?;
        plugin.status = status;
        Ok(())
    }

    /// Get all plugins with a given status.
    pub fn by_status(&self, status: PluginStatus) -> Vec<&PluginInfo> {
        self.plugins
            .values()
            .filter(|p| p.status == status)
            .collect()
    }

    /// Get all active plugins sorted by priority (descending).
    pub fn active_by_priority(&self) -> Vec<&PluginInfo> {
        let mut active: Vec<&PluginInfo> = self
            .plugins
            .values()
            .filter(|p| p.status == PluginStatus::Active)
            .collect();
        active.sort_by_key(|r| std::cmp::Reverse(r.priority));
        active
    }

    /// Search plugins by capability.
    pub fn with_capability(&self, capability: &str) -> Vec<&PluginInfo> {
        self.plugins
            .values()
            .filter(|p| p.capabilities.iter().any(|c| c == capability))
            .collect()
    }

    /// Get total plugin count.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Get remaining capacity.
    pub fn remaining_capacity(&self) -> usize {
        self.max_plugins.saturating_sub(self.plugins.len())
    }

    /// Get load order.
    pub fn load_order(&self) -> &[String] {
        &self.load_order
    }

    /// Check if a plugin exists.
    pub fn contains(&self, id: &str) -> bool {
        self.plugins.contains_key(id)
    }

    /// List all plugin IDs.
    pub fn plugin_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.plugins.keys().map(|s| s.as_str()).collect();
        ids.sort();
        ids
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_plugin(id: &str, priority: u32) -> PluginInfo {
        PluginInfo {
            id: id.to_string(),
            name: format!("Plugin {id}"),
            version: "1.0.0".to_string(),
            author: "test".to_string(),
            description: "test plugin".to_string(),
            capabilities: vec!["routing".to_string()],
            status: PluginStatus::Registered,
            priority,
        }
    }

    #[test]
    fn test_register_and_lookup() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        assert!(reg.contains("p1"));
        assert!(!reg.contains("p2"));
        assert_eq!(reg.count(), 1);
        let p = reg.get("p1").unwrap();
        assert_eq!(p.name, "Plugin p1");
    }

    #[test]
    fn test_register_duplicate_rejected() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        let err = reg.register(make_plugin("p1", 2)).unwrap_err();
        assert!(err.contains("already registered"));
    }

    #[test]
    fn test_registry_full() {
        let mut reg = PluginRegistry::new(2);
        reg.register(make_plugin("p1", 1)).unwrap();
        reg.register(make_plugin("p2", 2)).unwrap();
        let err = reg.register(make_plugin("p3", 3)).unwrap_err();
        assert!(err.contains("Registry full"));
        assert_eq!(reg.remaining_capacity(), 0);
    }

    #[test]
    fn test_unregister() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        let info = reg.unregister("p1").unwrap();
        assert_eq!(info.id, "p1");
        assert!(!reg.contains("p1"));
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn test_unregister_not_found() {
        let mut reg = PluginRegistry::new(10);
        let err = reg.unregister("missing").unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_set_status() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        reg.set_status("p1", PluginStatus::Active).unwrap();
        assert_eq!(reg.get("p1").unwrap().status, PluginStatus::Active);
    }

    #[test]
    fn test_by_status() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        reg.register(make_plugin("p2", 2)).unwrap();
        reg.set_status("p1", PluginStatus::Active).unwrap();
        assert_eq!(reg.by_status(PluginStatus::Active).len(), 1);
        assert_eq!(reg.by_status(PluginStatus::Registered).len(), 1);
    }

    #[test]
    fn test_active_by_priority() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("p1", 1)).unwrap();
        reg.register(make_plugin("p2", 5)).unwrap();
        reg.register(make_plugin("p3", 3)).unwrap();
        reg.set_status("p1", PluginStatus::Active).unwrap();
        reg.set_status("p2", PluginStatus::Active).unwrap();
        reg.set_status("p3", PluginStatus::Active).unwrap();
        let active = reg.active_by_priority();
        assert_eq!(active[0].id, "p2"); // priority 5
        assert_eq!(active[1].id, "p3"); // priority 3
        assert_eq!(active[2].id, "p1"); // priority 1
    }

    #[test]
    fn test_with_capability() {
        let mut reg = PluginRegistry::new(10);
        let mut p1 = make_plugin("p1", 1);
        p1.capabilities = vec!["routing".to_string(), "display".to_string()];
        let mut p2 = make_plugin("p2", 2);
        p2.capabilities = vec!["telemetry".to_string()];
        reg.register(p1).unwrap();
        reg.register(p2).unwrap();
        assert_eq!(reg.with_capability("routing").len(), 1);
        assert_eq!(reg.with_capability("display").len(), 1);
        assert_eq!(reg.with_capability("telemetry").len(), 1);
        assert_eq!(reg.with_capability("unknown").len(), 0);
    }

    #[test]
    fn test_load_order() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("b", 1)).unwrap();
        reg.register(make_plugin("a", 2)).unwrap();
        let order = reg.load_order();
        assert_eq!(order, &["b", "a"]); // insertion order
    }

    #[test]
    fn test_plugin_ids_sorted() {
        let mut reg = PluginRegistry::new(10);
        reg.register(make_plugin("c", 1)).unwrap();
        reg.register(make_plugin("a", 2)).unwrap();
        reg.register(make_plugin("b", 3)).unwrap();
        assert_eq!(reg.plugin_ids(), vec!["a", "b", "c"]);
    }
}
