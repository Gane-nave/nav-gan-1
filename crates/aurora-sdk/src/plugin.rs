//! Plugin system — extensible hook-based architecture for adding custom
//! behaviour to the AURORA NAV SDK pipeline.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::{debug, info, warn};

use crate::client::{ClientError, SdkRequest, SdkResponse};

// ---------------------------------------------------------------------------
// Plugin trait
// ---------------------------------------------------------------------------

/// A plugin that can hook into the SDK request/response pipeline.
pub trait Plugin: Send {
    /// Unique name of this plugin.
    fn name(&self) -> &str;

    /// Plugin version string.
    fn version(&self) -> &str;

    /// Plugin metadata.
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: self.name().to_string(),
            version: self.version().to_string(),
            description: String::new(),
            author: String::new(),
            capabilities: Vec::new(),
        }
    }

    /// Called once when the client connects. Return Err to abort.
    fn on_initialise(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Called before each request. Can modify the request.
    fn on_pre_request(&self, req: SdkRequest) -> Result<SdkRequest, String> {
        Ok(req)
    }

    /// Called after each response. Can modify the response.
    fn on_post_response(&self, resp: SdkResponse) -> Result<SdkResponse, String> {
        Ok(resp)
    }

    /// Called when the client disconnects.
    fn on_shutdown(&mut self) {}
}

// ---------------------------------------------------------------------------
// Plugin metadata
// ---------------------------------------------------------------------------

/// Descriptive metadata for a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<PluginCapability>,
}

/// Capability a plugin can declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginCapability {
    /// Can transform requests.
    RequestTransform,
    /// Can transform responses.
    ResponseTransform,
    /// Provides telemetry / logging.
    Telemetry,
    /// Provides caching.
    Caching,
    /// Provides authentication augmentation.
    Authentication,
    /// Custom data source.
    DataSource,
}

/// Runtime state of a registered plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    /// Registered but not yet initialised.
    Registered,
    /// Initialised and active.
    Active,
    /// Initialisation failed.
    Failed,
    /// Disabled by user or error.
    Disabled,
}

/// Snapshot of a plugin's runtime info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub state: PluginState,
    pub registered_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Plugin manager
// ---------------------------------------------------------------------------

/// Manages the lifecycle of plugins in the SDK.
pub struct PluginManager {
    plugins: Vec<(Box<dyn Plugin>, PluginState, DateTime<Utc>)>,
    max_plugins: usize,
}

impl fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PluginManager")
            .field("count", &self.plugins.len())
            .field("max_plugins", &self.max_plugins)
            .finish()
    }
}

impl PluginManager {
    /// Create a new plugin manager with a max plugin limit.
    pub fn new(max_plugins: usize) -> Self {
        Self {
            plugins: Vec::new(),
            max_plugins,
        }
    }

    /// Register a plugin.
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), ClientError> {
        if self.plugins.len() >= self.max_plugins {
            return Err(ClientError::TooManyPlugins(self.max_plugins));
        }
        info!(
            plugin = plugin.name(),
            version = plugin.version(),
            "plugin registered"
        );
        self.plugins
            .push((plugin, PluginState::Registered, Utc::now()));
        Ok(())
    }

    /// Initialise all registered plugins.
    pub fn initialise_all(&mut self) -> Result<(), String> {
        for (plugin, state, _) in &mut self.plugins {
            if *state == PluginState::Registered {
                match plugin.on_initialise() {
                    Ok(()) => {
                        *state = PluginState::Active;
                        debug!(plugin = plugin.name(), "plugin initialised");
                    }
                    Err(e) => {
                        *state = PluginState::Failed;
                        warn!(plugin = plugin.name(), error = %e, "plugin init failed");
                        return Err(format!("plugin '{}' init failed: {}", plugin.name(), e));
                    }
                }
            }
        }
        Ok(())
    }

    /// Run pre-request hooks on all active plugins.
    pub fn pre_request(&self, mut req: SdkRequest) -> Result<SdkRequest, String> {
        for (plugin, state, _) in &self.plugins {
            if *state == PluginState::Active {
                req = plugin.on_pre_request(req)?;
            }
        }
        Ok(req)
    }

    /// Run post-response hooks on all active plugins.
    pub fn post_response(&self, mut resp: SdkResponse) -> Result<SdkResponse, String> {
        for (plugin, state, _) in &self.plugins {
            if *state == PluginState::Active {
                resp = plugin.on_post_response(resp)?;
            }
        }
        Ok(resp)
    }

    /// Shutdown all active plugins.
    pub fn shutdown_all(&mut self) {
        for (plugin, state, _) in &mut self.plugins {
            if *state == PluginState::Active {
                plugin.on_shutdown();
                *state = PluginState::Disabled;
                debug!(plugin = plugin.name(), "plugin shutdown");
            }
        }
    }

    /// Number of registered plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Number of active plugins.
    pub fn active_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|(_, s, _)| *s == PluginState::Active)
            .count()
    }

    /// Get info about all registered plugins.
    pub fn list(&self) -> Vec<PluginInfo> {
        self.plugins
            .iter()
            .map(|(p, state, registered_at)| PluginInfo {
                name: p.name().to_string(),
                version: p.version().to_string(),
                state: *state,
                registered_at: *registered_at,
            })
            .collect()
    }

    /// Disable a plugin by name.
    pub fn disable(&mut self, name: &str) -> bool {
        for (plugin, state, _) in &mut self.plugins {
            if plugin.name() == name && *state == PluginState::Active {
                plugin.on_shutdown();
                *state = PluginState::Disabled;
                info!(plugin = name, "plugin disabled");
                return true;
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        name: String,
        init_count: std::cell::Cell<u32>,
        should_fail: bool,
    }

    impl TestPlugin {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                init_count: std::cell::Cell::new(0),
                should_fail: false,
            }
        }

        fn failing(name: &str) -> Self {
            Self {
                name: name.to_string(),
                init_count: std::cell::Cell::new(0),
                should_fail: true,
            }
        }
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn on_initialise(&mut self) -> Result<(), String> {
            self.init_count.set(self.init_count.get() + 1);
            if self.should_fail {
                Err("intentional failure".into())
            } else {
                Ok(())
            }
        }

        fn on_pre_request(&self, mut req: SdkRequest) -> Result<SdkRequest, String> {
            // Stamp plugin name into params.
            if let Some(obj) = req.params.as_object_mut() {
                obj.insert(format!("plugin_{}", self.name), serde_json::json!(true));
            }
            Ok(req)
        }
    }

    #[test]
    fn register_and_initialise_plugins() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::new("alpha"))).unwrap();
        pm.register(Box::new(TestPlugin::new("beta"))).unwrap();
        assert_eq!(pm.count(), 2);
        assert_eq!(pm.active_count(), 0);

        pm.initialise_all().unwrap();
        assert_eq!(pm.active_count(), 2);
    }

    #[test]
    fn max_plugins_enforced() {
        let mut pm = PluginManager::new(1);
        pm.register(Box::new(TestPlugin::new("a"))).unwrap();
        let result = pm.register(Box::new(TestPlugin::new("b")));
        assert!(result.is_err());
    }

    #[test]
    fn failing_plugin_stops_init() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::failing("bad"))).unwrap();
        let result = pm.initialise_all();
        assert!(result.is_err());
        assert_eq!(pm.active_count(), 0);

        let info = pm.list();
        assert_eq!(info[0].state, PluginState::Failed);
    }

    #[test]
    fn pre_request_hooks_modify_request() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::new("stamper"))).unwrap();
        pm.initialise_all().unwrap();

        let req = SdkRequest::new("test", serde_json::json!({}));
        let processed = pm.pre_request(req).unwrap();
        assert_eq!(processed.params["plugin_stamper"], true);
    }

    #[test]
    fn disable_plugin() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::new("removable"))).unwrap();
        pm.initialise_all().unwrap();
        assert_eq!(pm.active_count(), 1);

        assert!(pm.disable("removable"));
        assert_eq!(pm.active_count(), 0);
        assert_eq!(pm.count(), 1); // still registered but disabled.

        // Disabling non-existent returns false.
        assert!(!pm.disable("nonexistent"));
    }

    #[test]
    fn shutdown_all_deactivates() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::new("a"))).unwrap();
        pm.register(Box::new(TestPlugin::new("b"))).unwrap();
        pm.initialise_all().unwrap();
        assert_eq!(pm.active_count(), 2);

        pm.shutdown_all();
        assert_eq!(pm.active_count(), 0);
    }

    #[test]
    fn plugin_list_returns_info() {
        let mut pm = PluginManager::new(10);
        pm.register(Box::new(TestPlugin::new("info-test"))).unwrap();
        let list = pm.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "info-test");
        assert_eq!(list[0].version, "1.0.0");
        assert_eq!(list[0].state, PluginState::Registered);
    }
}
