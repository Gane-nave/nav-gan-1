//! Extension system — lightweight extension points for customising
//! SDK behaviour without the full plugin lifecycle.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::debug;

use crate::client::SdkRequest;

// ---------------------------------------------------------------------------
// Extension trait
// ---------------------------------------------------------------------------

/// A lightweight extension that can modify requests or add capabilities.
///
/// Unlike plugins, extensions are stateless and do not have lifecycle hooks.
/// They are applied in registration order on every request.
pub trait Extension: Send {
    /// Unique name of this extension.
    fn name(&self) -> &str;

    /// Extension category.
    fn category(&self) -> ExtensionCategory;

    /// Transform a request. Default is pass-through.
    fn transform_request(&self, req: SdkRequest) -> SdkRequest {
        req
    }

    /// Priority (higher = runs first). Default is 0.
    fn priority(&self) -> i32 {
        0
    }
}

/// Extension category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionCategory {
    /// Adds or transforms headers/metadata.
    Middleware,
    /// Provides data transformation.
    Transform,
    /// Adds monitoring or observability.
    Observability,
    /// Provides security features.
    Security,
    /// Custom / uncategorised.
    Custom,
}

/// Extension info snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub category: ExtensionCategory,
    pub priority: i32,
    pub registered_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Extension registry
// ---------------------------------------------------------------------------

/// Registry that manages extensions and applies them to requests.
pub struct ExtensionRegistry {
    extensions: Vec<(Box<dyn Extension>, DateTime<Utc>)>,
}

impl fmt::Debug for ExtensionRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtensionRegistry")
            .field("count", &self.extensions.len())
            .finish()
    }
}

impl ExtensionRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    /// Register an extension.
    pub fn register(&mut self, ext: Box<dyn Extension>) {
        debug!(name = ext.name(), category = ?ext.category(), "extension registered");
        self.extensions.push((ext, Utc::now()));
        // Sort by priority (descending — higher priority runs first).
        self.extensions
            .sort_by(|(a, _), (b, _)| b.priority().cmp(&a.priority()));
    }

    /// Apply all extensions to a request (in priority order).
    pub fn apply_request_extensions(&self, mut req: SdkRequest) -> SdkRequest {
        for (ext, _) in &self.extensions {
            req = ext.transform_request(req);
        }
        req
    }

    /// Number of registered extensions.
    pub fn count(&self) -> usize {
        self.extensions.len()
    }

    /// List all registered extensions.
    pub fn list(&self) -> Vec<ExtensionInfo> {
        self.extensions
            .iter()
            .map(|(ext, registered_at)| ExtensionInfo {
                name: ext.name().to_string(),
                category: ext.category(),
                priority: ext.priority(),
                registered_at: *registered_at,
            })
            .collect()
    }

    /// Find extensions by category.
    pub fn by_category(&self, category: ExtensionCategory) -> Vec<ExtensionInfo> {
        self.list()
            .into_iter()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Remove an extension by name. Returns true if removed.
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.extensions.len();
        self.extensions.retain(|(ext, _)| ext.name() != name);
        let removed = self.extensions.len() < before;
        if removed {
            debug!(name = name, "extension removed");
        }
        removed
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Built-in extensions
// ---------------------------------------------------------------------------

/// Extension that adds a correlation ID to every request.
pub struct CorrelationIdExtension;

impl Extension for CorrelationIdExtension {
    fn name(&self) -> &str {
        "correlation-id"
    }

    fn category(&self) -> ExtensionCategory {
        ExtensionCategory::Observability
    }

    fn transform_request(&self, mut req: SdkRequest) -> SdkRequest {
        if let Some(obj) = req.params.as_object_mut() {
            obj.insert(
                "correlation_id".to_string(),
                serde_json::json!(EntityId::new().to_string()),
            );
        }
        req
    }

    fn priority(&self) -> i32 {
        100 // High priority — should run first.
    }
}

/// Extension that adds a timestamp to every request.
pub struct TimestampExtension;

impl Extension for TimestampExtension {
    fn name(&self) -> &str {
        "timestamp"
    }

    fn category(&self) -> ExtensionCategory {
        ExtensionCategory::Middleware
    }

    fn transform_request(&self, mut req: SdkRequest) -> SdkRequest {
        if let Some(obj) = req.params.as_object_mut() {
            obj.insert(
                "sdk_timestamp".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
        }
        req
    }

    fn priority(&self) -> i32 {
        50
    }
}

/// Extension that tags requests with a custom label.
pub struct TaggingExtension {
    tag_key: String,
    tag_value: String,
}

impl TaggingExtension {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            tag_key: key.into(),
            tag_value: value.into(),
        }
    }
}

impl Extension for TaggingExtension {
    fn name(&self) -> &str {
        "tagging"
    }

    fn category(&self) -> ExtensionCategory {
        ExtensionCategory::Middleware
    }

    fn transform_request(&self, mut req: SdkRequest) -> SdkRequest {
        if let Some(obj) = req.params.as_object_mut() {
            obj.insert(self.tag_key.clone(), serde_json::json!(self.tag_value));
        }
        req
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct HighPriorityExt;
    impl Extension for HighPriorityExt {
        fn name(&self) -> &str {
            "high"
        }
        fn category(&self) -> ExtensionCategory {
            ExtensionCategory::Custom
        }
        fn priority(&self) -> i32 {
            10
        }
        fn transform_request(&self, mut req: SdkRequest) -> SdkRequest {
            if let Some(obj) = req.params.as_object_mut() {
                // Append to processing order.
                let order = obj.entry("order").or_insert_with(|| serde_json::json!(""));
                let prev = order.as_str().unwrap_or("").to_string();
                *order = serde_json::json!(format!("{}high,", prev));
            }
            req
        }
    }

    struct LowPriorityExt;
    impl Extension for LowPriorityExt {
        fn name(&self) -> &str {
            "low"
        }
        fn category(&self) -> ExtensionCategory {
            ExtensionCategory::Custom
        }
        fn priority(&self) -> i32 {
            1
        }
        fn transform_request(&self, mut req: SdkRequest) -> SdkRequest {
            if let Some(obj) = req.params.as_object_mut() {
                let order = obj.entry("order").or_insert_with(|| serde_json::json!(""));
                let prev = order.as_str().unwrap_or("").to_string();
                *order = serde_json::json!(format!("{}low,", prev));
            }
            req
        }
    }

    #[test]
    fn register_and_count() {
        let mut reg = ExtensionRegistry::new();
        assert_eq!(reg.count(), 0);

        reg.register(Box::new(CorrelationIdExtension));
        assert_eq!(reg.count(), 1);

        reg.register(Box::new(TimestampExtension));
        assert_eq!(reg.count(), 2);
    }

    #[test]
    fn extensions_sorted_by_priority() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(LowPriorityExt));
        reg.register(Box::new(HighPriorityExt));

        let list = reg.list();
        assert_eq!(list[0].name, "high"); // priority 10 first
        assert_eq!(list[1].name, "low"); // priority 1 second
    }

    #[test]
    fn extensions_applied_in_priority_order() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(LowPriorityExt));
        reg.register(Box::new(HighPriorityExt));

        let req = SdkRequest::new("test", serde_json::json!({}));
        let processed = reg.apply_request_extensions(req);

        let order = processed.params["order"].as_str().unwrap();
        assert_eq!(order, "high,low,", "high priority should run first");
    }

    #[test]
    fn correlation_id_extension_adds_id() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(CorrelationIdExtension));

        let req = SdkRequest::new("test", serde_json::json!({}));
        let processed = reg.apply_request_extensions(req);
        assert!(processed.params.get("correlation_id").is_some());
    }

    #[test]
    fn timestamp_extension_adds_timestamp() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(TimestampExtension));

        let req = SdkRequest::new("test", serde_json::json!({}));
        let processed = reg.apply_request_extensions(req);
        assert!(processed.params.get("sdk_timestamp").is_some());
    }

    #[test]
    fn tagging_extension_adds_tag() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(TaggingExtension::new("env", "production")));

        let req = SdkRequest::new("test", serde_json::json!({}));
        let processed = reg.apply_request_extensions(req);
        assert_eq!(processed.params["env"], "production");
    }

    #[test]
    fn remove_extension() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(CorrelationIdExtension));
        reg.register(Box::new(TimestampExtension));
        assert_eq!(reg.count(), 2);

        assert!(reg.remove("correlation-id"));
        assert_eq!(reg.count(), 1);

        // Remove non-existent.
        assert!(!reg.remove("nonexistent"));
    }

    #[test]
    fn by_category_filters() {
        let mut reg = ExtensionRegistry::new();
        reg.register(Box::new(CorrelationIdExtension));
        reg.register(Box::new(TimestampExtension));
        reg.register(Box::new(TaggingExtension::new("k", "v")));

        let observability = reg.by_category(ExtensionCategory::Observability);
        assert_eq!(observability.len(), 1);
        assert_eq!(observability[0].name, "correlation-id");

        let middleware = reg.by_category(ExtensionCategory::Middleware);
        assert_eq!(middleware.len(), 2);
    }
}
