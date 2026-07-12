//! Hook system — extension points where plugins can inject behaviour.

use std::collections::HashMap;

/// Hook execution priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookPriority {
    /// Runs first.
    Highest,
    /// Runs before normal.
    High,
    /// Default priority.
    Normal,
    /// Runs after normal.
    Low,
    /// Runs last.
    Lowest,
}

/// A registered hook handler.
#[derive(Debug, Clone)]
pub struct HookHandler {
    /// Handler identifier.
    pub id: String,
    /// Plugin that registered this handler.
    pub plugin_id: String,
    /// Priority for ordering.
    pub priority: HookPriority,
    /// Whether the handler is enabled.
    pub enabled: bool,
}

/// Hook point definition.
#[derive(Debug, Clone)]
pub struct HookPoint {
    /// Hook name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Number of times this hook has been invoked.
    pub invocation_count: u64,
}

/// Hook manager — manages extension points and handler registration.
pub struct HookManager {
    hooks: HashMap<String, HookPoint>,
    handlers: HashMap<String, Vec<HookHandler>>,
}

impl HookManager {
    /// Create a new hook manager.
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    /// Define a new hook point.
    pub fn define_hook(&mut self, name: &str, description: &str) -> Result<(), String> {
        if self.hooks.contains_key(name) {
            return Err(format!("Hook '{name}' already defined"));
        }
        self.hooks.insert(
            name.to_string(),
            HookPoint {
                name: name.to_string(),
                description: description.to_string(),
                invocation_count: 0,
            },
        );
        self.handlers.insert(name.to_string(), Vec::new());
        Ok(())
    }

    /// Register a handler on a hook.
    pub fn register_handler(
        &mut self,
        hook_name: &str,
        handler: HookHandler,
    ) -> Result<(), String> {
        let handlers = self
            .handlers
            .get_mut(hook_name)
            .ok_or_else(|| format!("Hook '{hook_name}' not defined"))?;

        // Check for duplicate handler ID on this hook
        if handlers.iter().any(|h| h.id == handler.id) {
            return Err(format!(
                "Handler '{}' already registered on hook '{hook_name}'",
                handler.id
            ));
        }

        handlers.push(handler);
        // Sort by priority (Highest first)
        handlers.sort_by_key(|a| a.priority);
        Ok(())
    }

    /// Unregister a handler.
    pub fn unregister_handler(&mut self, hook_name: &str, handler_id: &str) -> Result<(), String> {
        let handlers = self
            .handlers
            .get_mut(hook_name)
            .ok_or_else(|| format!("Hook '{hook_name}' not defined"))?;
        let before = handlers.len();
        handlers.retain(|h| h.id != handler_id);
        if handlers.len() == before {
            return Err(format!(
                "Handler '{handler_id}' not found on hook '{hook_name}'"
            ));
        }
        Ok(())
    }

    /// Invoke a hook — returns ordered list of enabled handlers.
    pub fn invoke(&mut self, hook_name: &str) -> Result<Vec<&HookHandler>, String> {
        let hook = self
            .hooks
            .get_mut(hook_name)
            .ok_or_else(|| format!("Hook '{hook_name}' not defined"))?;
        hook.invocation_count += 1;

        let handlers = self.handlers.get(hook_name).unwrap();
        Ok(handlers.iter().filter(|h| h.enabled).collect())
    }

    /// Get handler count for a hook.
    pub fn handler_count(&self, hook_name: &str) -> usize {
        self.handlers.get(hook_name).map(|h| h.len()).unwrap_or(0)
    }

    /// Get invocation count for a hook.
    pub fn invocation_count(&self, hook_name: &str) -> u64 {
        self.hooks
            .get(hook_name)
            .map(|h| h.invocation_count)
            .unwrap_or(0)
    }

    /// Get all hook names.
    pub fn hook_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.hooks.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    /// Get hook count.
    pub fn hook_count(&self) -> usize {
        self.hooks.len()
    }

    /// Remove a hook and all its handlers.
    pub fn remove_hook(&mut self, name: &str) -> Result<(), String> {
        if self.hooks.remove(name).is_none() {
            return Err(format!("Hook '{name}' not defined"));
        }
        self.handlers.remove(name);
        Ok(())
    }

    /// Unregister all handlers for a given plugin.
    pub fn unregister_plugin(&mut self, plugin_id: &str) {
        for handlers in self.handlers.values_mut() {
            handlers.retain(|h| h.plugin_id != plugin_id);
        }
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_hook() {
        let mut mgr = HookManager::new();
        mgr.define_hook("pre_route", "Before routing").unwrap();
        assert_eq!(mgr.hook_count(), 1);
        assert_eq!(mgr.hook_names(), vec!["pre_route"]);
    }

    #[test]
    fn test_define_duplicate_hook() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "d1").unwrap();
        let err = mgr.define_hook("h1", "d2").unwrap_err();
        assert!(err.contains("already defined"));
    }

    #[test]
    fn test_register_handler() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "handler1".to_string(),
                plugin_id: "p1".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        assert_eq!(mgr.handler_count("h1"), 1);
    }

    #[test]
    fn test_duplicate_handler_rejected() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        let handler = HookHandler {
            id: "handler1".to_string(),
            plugin_id: "p1".to_string(),
            priority: HookPriority::Normal,
            enabled: true,
        };
        mgr.register_handler("h1", handler.clone()).unwrap();
        let err = mgr.register_handler("h1", handler).unwrap_err();
        assert!(err.contains("already registered"));
    }

    #[test]
    fn test_handler_on_undefined_hook() {
        let mut mgr = HookManager::new();
        let err = mgr
            .register_handler(
                "missing",
                HookHandler {
                    id: "h".to_string(),
                    plugin_id: "p".to_string(),
                    priority: HookPriority::Normal,
                    enabled: true,
                },
            )
            .unwrap_err();
        assert!(err.contains("not defined"));
    }

    #[test]
    fn test_invoke_orders_by_priority() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "low".to_string(),
                plugin_id: "p1".to_string(),
                priority: HookPriority::Low,
                enabled: true,
            },
        )
        .unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "high".to_string(),
                plugin_id: "p2".to_string(),
                priority: HookPriority::Highest,
                enabled: true,
            },
        )
        .unwrap();
        let handlers = mgr.invoke("h1").unwrap();
        assert_eq!(handlers[0].id, "high");
        assert_eq!(handlers[1].id, "low");
        assert_eq!(mgr.invocation_count("h1"), 1);
    }

    #[test]
    fn test_invoke_skips_disabled() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "enabled".to_string(),
                plugin_id: "p1".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "disabled".to_string(),
                plugin_id: "p2".to_string(),
                priority: HookPriority::Normal,
                enabled: false,
            },
        )
        .unwrap();
        let handlers = mgr.invoke("h1").unwrap();
        assert_eq!(handlers.len(), 1);
        assert_eq!(handlers[0].id, "enabled");
    }

    #[test]
    fn test_unregister_handler() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "h".to_string(),
                plugin_id: "p".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        mgr.unregister_handler("h1", "h").unwrap();
        assert_eq!(mgr.handler_count("h1"), 0);
    }

    #[test]
    fn test_unregister_plugin() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.define_hook("h2", "desc2").unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "a".to_string(),
                plugin_id: "p1".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        mgr.register_handler(
            "h2",
            HookHandler {
                id: "b".to_string(),
                plugin_id: "p1".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        mgr.register_handler(
            "h1",
            HookHandler {
                id: "c".to_string(),
                plugin_id: "p2".to_string(),
                priority: HookPriority::Normal,
                enabled: true,
            },
        )
        .unwrap();
        mgr.unregister_plugin("p1");
        assert_eq!(mgr.handler_count("h1"), 1); // only p2's handler remains
        assert_eq!(mgr.handler_count("h2"), 0);
    }

    #[test]
    fn test_remove_hook() {
        let mut mgr = HookManager::new();
        mgr.define_hook("h1", "desc").unwrap();
        mgr.remove_hook("h1").unwrap();
        assert_eq!(mgr.hook_count(), 0);
        let err = mgr.remove_hook("h1").unwrap_err();
        assert!(err.contains("not defined"));
    }
}
