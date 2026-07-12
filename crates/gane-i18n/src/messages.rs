//! Message catalog — translation loading, key lookup with fallback, and interpolation.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A translation catalog for a single locale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCatalog {
    pub locale: String,
    messages: HashMap<String, String>,
}

impl MessageCatalog {
    /// Create a new empty catalog for a locale.
    pub fn new(locale: &str) -> Self {
        Self {
            locale: locale.to_string(),
            messages: HashMap::new(),
        }
    }

    /// Add a message to the catalog.
    pub fn insert(&mut self, key: &str, value: &str) {
        self.messages.insert(key.to_string(), value.to_string());
    }

    /// Get a message by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|s| s.as_str())
    }

    /// Number of messages in the catalog.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if catalog is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Load messages from a JSON string.
    pub fn load_json(&mut self, json: &str) -> Result<usize, serde_json::Error> {
        let map: HashMap<String, String> = serde_json::from_str(json)?;
        let count = map.len();
        self.messages.extend(map);
        Ok(count)
    }

    /// Merge another catalog into this one (other takes precedence).
    pub fn merge(&mut self, other: &MessageCatalog) {
        for (k, v) in &other.messages {
            self.messages.insert(k.clone(), v.clone());
        }
    }
}

/// Interpolation context for template variables.
pub type InterpolationContext = HashMap<String, String>;

/// Interpolate variables into a message template.
/// Variables are referenced as `{name}` in the template.
pub fn interpolate(template: &str, ctx: &InterpolationContext) -> String {
    let mut result = template.to_string();
    for (key, value) in ctx {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

/// Translation manager — holds catalogs for multiple locales, supports fallback lookup.
pub struct TranslationManager {
    catalogs: RwLock<HashMap<String, MessageCatalog>>,
    fallback_chain: RwLock<Vec<String>>,
}

impl TranslationManager {
    /// Create a new translation manager.
    pub fn new() -> Self {
        Self {
            catalogs: RwLock::new(HashMap::new()),
            fallback_chain: RwLock::new(vec!["en".to_string()]),
        }
    }

    /// Register a catalog for a locale.
    pub fn register_catalog(&self, catalog: MessageCatalog) {
        self.catalogs
            .write()
            .insert(catalog.locale.clone(), catalog);
    }

    /// Set the fallback chain (ordered list of locale tags to try).
    pub fn set_fallback_chain(&self, chain: Vec<String>) {
        *self.fallback_chain.write() = chain;
    }

    /// Translate a key using the fallback chain.
    pub fn translate(&self, key: &str) -> Option<String> {
        let catalogs = self.catalogs.read();
        let chain = self.fallback_chain.read();
        for locale in chain.iter() {
            if let Some(catalog) = catalogs.get(locale) {
                if let Some(msg) = catalog.get(key) {
                    return Some(msg.to_string());
                }
            }
        }
        None
    }

    /// Translate a key with interpolation.
    pub fn translate_with(&self, key: &str, ctx: &InterpolationContext) -> Option<String> {
        self.translate(key).map(|msg| interpolate(&msg, ctx))
    }

    /// Get all keys in a catalog.
    pub fn keys(&self, locale: &str) -> Vec<String> {
        self.catalogs
            .read()
            .get(locale)
            .map(|c| c.messages.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Check if a key exists in any catalog in the fallback chain.
    pub fn has_key(&self, key: &str) -> bool {
        let catalogs = self.catalogs.read();
        let chain = self.fallback_chain.read();
        for locale in chain.iter() {
            if let Some(catalog) = catalogs.get(locale) {
                if catalog.get(key).is_some() {
                    return true;
                }
            }
        }
        false
    }

    /// Get statistics about loaded catalogs.
    pub fn stats(&self) -> HashMap<String, usize> {
        self.catalogs
            .read()
            .iter()
            .map(|(locale, catalog)| (locale.clone(), catalog.len()))
            .collect()
    }
}

impl Default for TranslationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_insert_get() {
        let mut catalog = MessageCatalog::new("en");
        catalog.insert("nav.start", "Start Navigation");
        catalog.insert("nav.stop", "Stop Navigation");
        assert_eq!(catalog.get("nav.start"), Some("Start Navigation"));
        assert_eq!(catalog.len(), 2);
    }

    #[test]
    fn test_catalog_load_json() {
        let mut catalog = MessageCatalog::new("en");
        let json = r#"{"greeting": "Hello", "farewell": "Goodbye"}"#;
        let count = catalog.load_json(json).unwrap();
        assert_eq!(count, 2);
        assert_eq!(catalog.get("greeting"), Some("Hello"));
    }

    #[test]
    fn test_catalog_merge() {
        let mut base = MessageCatalog::new("en");
        base.insert("a", "base_a");
        base.insert("b", "base_b");

        let mut overlay = MessageCatalog::new("en");
        overlay.insert("b", "overlay_b");
        overlay.insert("c", "overlay_c");

        base.merge(&overlay);
        assert_eq!(base.get("a"), Some("base_a"));
        assert_eq!(base.get("b"), Some("overlay_b"));
        assert_eq!(base.get("c"), Some("overlay_c"));
    }

    #[test]
    fn test_interpolation() {
        let mut ctx = InterpolationContext::new();
        ctx.insert("distance".to_string(), "500m".to_string());
        ctx.insert("street".to_string(), "Main St".to_string());

        let result = interpolate("Turn left in {distance} onto {street}", &ctx);
        assert_eq!(result, "Turn left in 500m onto Main St");
    }

    #[test]
    fn test_interpolation_missing_var() {
        let ctx = InterpolationContext::new();
        let result = interpolate("Hello {name}", &ctx);
        assert_eq!(result, "Hello {name}");
    }

    #[test]
    fn test_translation_manager_fallback() {
        let mgr = TranslationManager::new();

        let mut en = MessageCatalog::new("en");
        en.insert("nav.start", "Start");
        en.insert("nav.stop", "Stop");

        let mut de = MessageCatalog::new("de");
        de.insert("nav.start", "Starten");

        mgr.register_catalog(en);
        mgr.register_catalog(de);
        mgr.set_fallback_chain(vec!["de".to_string(), "en".to_string()]);

        // "nav.start" exists in de
        assert_eq!(mgr.translate("nav.start"), Some("Starten".to_string()));
        // "nav.stop" falls back to en
        assert_eq!(mgr.translate("nav.stop"), Some("Stop".to_string()));
        // "nav.xyz" not in any
        assert_eq!(mgr.translate("nav.xyz"), None);
    }

    #[test]
    fn test_translate_with_interpolation() {
        let mgr = TranslationManager::new();
        let mut en = MessageCatalog::new("en");
        en.insert("eta", "Arriving in {minutes} minutes");
        mgr.register_catalog(en);
        mgr.set_fallback_chain(vec!["en".to_string()]);

        let mut ctx = InterpolationContext::new();
        ctx.insert("minutes".to_string(), "5".to_string());

        let result = mgr.translate_with("eta", &ctx);
        assert_eq!(result, Some("Arriving in 5 minutes".to_string()));
    }

    #[test]
    fn test_has_key() {
        let mgr = TranslationManager::new();
        let mut en = MessageCatalog::new("en");
        en.insert("exists", "yes");
        mgr.register_catalog(en);
        mgr.set_fallback_chain(vec!["en".to_string()]);

        assert!(mgr.has_key("exists"));
        assert!(!mgr.has_key("missing"));
    }

    #[test]
    fn test_stats() {
        let mgr = TranslationManager::new();
        let mut en = MessageCatalog::new("en");
        en.insert("a", "1");
        en.insert("b", "2");
        let mut de = MessageCatalog::new("de");
        de.insert("a", "1");
        mgr.register_catalog(en);
        mgr.register_catalog(de);

        let stats = mgr.stats();
        assert_eq!(stats["en"], 2);
        assert_eq!(stats["de"], 1);
    }
}
