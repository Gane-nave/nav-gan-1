//! Key-value store — provides typed, namespaced persistent storage with TTL support.

use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A stored value with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredValue {
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub version: u64,
}

/// Namespace-scoped key-value store with TTL and versioning.
pub struct KvStore {
    namespaces: RwLock<HashMap<String, HashMap<String, StoredValue>>>,
}

impl KvStore {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self {
            namespaces: RwLock::new(HashMap::new()),
        }
    }

    /// Put a value into a namespace. Returns the version number.
    pub fn put(&self, namespace: &str, key: &str, value: serde_json::Value) -> u64 {
        self.put_with_ttl(namespace, key, value, None)
    }

    /// Put a value with an optional TTL in seconds.
    pub fn put_with_ttl(
        &self,
        namespace: &str,
        key: &str,
        value: serde_json::Value,
        ttl_secs: Option<i64>,
    ) -> u64 {
        let now = Utc::now();
        let mut store = self.namespaces.write();
        let ns = store.entry(namespace.to_string()).or_default();
        let version = ns.get(key).map(|v| v.version + 1).unwrap_or(1);
        let expires_at = ttl_secs.map(|s| now + Duration::seconds(s));
        ns.insert(
            key.to_string(),
            StoredValue {
                data: value,
                created_at: ns.get(key).map(|v| v.created_at).unwrap_or(now),
                updated_at: now,
                expires_at,
                version,
            },
        );
        version
    }

    /// Get a value from a namespace. Returns None if expired or missing.
    pub fn get(&self, namespace: &str, key: &str) -> Option<StoredValue> {
        let store = self.namespaces.read();
        let ns = store.get(namespace)?;
        let val = ns.get(key)?;
        if let Some(exp) = val.expires_at {
            if Utc::now() > exp {
                return None;
            }
        }
        Some(val.clone())
    }

    /// Delete a key. Returns true if it existed.
    pub fn delete(&self, namespace: &str, key: &str) -> bool {
        let mut store = self.namespaces.write();
        if let Some(ns) = store.get_mut(namespace) {
            return ns.remove(key).is_some();
        }
        false
    }

    /// List all keys in a namespace (excluding expired).
    pub fn keys(&self, namespace: &str) -> Vec<String> {
        let store = self.namespaces.read();
        let now = Utc::now();
        store
            .get(namespace)
            .map(|ns| {
                ns.iter()
                    .filter(|(_, v)| v.expires_at.is_none_or(|exp| now <= exp))
                    .map(|(k, _)| k.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Count keys in a namespace (excluding expired).
    pub fn count(&self, namespace: &str) -> usize {
        self.keys(namespace).len()
    }

    /// Purge expired entries across all namespaces. Returns count purged.
    pub fn purge_expired(&self) -> usize {
        let now = Utc::now();
        let mut store = self.namespaces.write();
        let mut purged = 0;
        for ns in store.values_mut() {
            let before = ns.len();
            ns.retain(|_, v| v.expires_at.is_none_or(|exp| now <= exp));
            purged += before - ns.len();
        }
        purged
    }

    /// List all namespaces.
    pub fn namespaces(&self) -> Vec<String> {
        self.namespaces.read().keys().cloned().collect()
    }

    /// Drop an entire namespace.
    pub fn drop_namespace(&self, namespace: &str) -> bool {
        self.namespaces.write().remove(namespace).is_some()
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let store = KvStore::new();
        let v = store.put(
            "nav",
            "last_route",
            serde_json::json!({"from": "A", "to": "B"}),
        );
        assert_eq!(v, 1);
        let got = store.get("nav", "last_route").unwrap();
        assert_eq!(got.data["from"], "A");
        assert_eq!(got.version, 1);
    }

    #[test]
    fn test_versioning() {
        let store = KvStore::new();
        store.put("ns", "k", serde_json::json!(1));
        store.put("ns", "k", serde_json::json!(2));
        let v = store.put("ns", "k", serde_json::json!(3));
        assert_eq!(v, 3);
        let got = store.get("ns", "k").unwrap();
        assert_eq!(got.data, serde_json::json!(3));
        assert_eq!(got.version, 3);
    }

    #[test]
    fn test_ttl_expiration() {
        let store = KvStore::new();
        store.put_with_ttl("ns", "temp", serde_json::json!("hi"), Some(-1));
        assert!(
            store.get("ns", "temp").is_none(),
            "expired key should not be returned"
        );
    }

    #[test]
    fn test_ttl_not_expired() {
        let store = KvStore::new();
        store.put_with_ttl("ns", "long", serde_json::json!("alive"), Some(3600));
        assert!(store.get("ns", "long").is_some());
    }

    #[test]
    fn test_delete() {
        let store = KvStore::new();
        store.put("ns", "k", serde_json::json!(1));
        assert!(store.delete("ns", "k"));
        assert!(store.get("ns", "k").is_none());
        assert!(!store.delete("ns", "k"));
    }

    #[test]
    fn test_keys_excludes_expired() {
        let store = KvStore::new();
        store.put("ns", "alive", serde_json::json!(1));
        store.put_with_ttl("ns", "dead", serde_json::json!(2), Some(-1));
        let keys = store.keys("ns");
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "alive");
    }

    #[test]
    fn test_purge_expired() {
        let store = KvStore::new();
        store.put("ns", "alive", serde_json::json!(1));
        store.put_with_ttl("ns", "dead1", serde_json::json!(2), Some(-1));
        store.put_with_ttl("ns", "dead2", serde_json::json!(3), Some(-1));
        let purged = store.purge_expired();
        assert_eq!(purged, 2);
        assert_eq!(store.count("ns"), 1);
    }

    #[test]
    fn test_namespaces_and_drop() {
        let store = KvStore::new();
        store.put("nav", "k1", serde_json::json!(1));
        store.put("fleet", "k2", serde_json::json!(2));
        let mut ns = store.namespaces();
        ns.sort();
        assert_eq!(ns, vec!["fleet", "nav"]);
        assert!(store.drop_namespace("fleet"));
        assert_eq!(store.namespaces().len(), 1);
    }

    #[test]
    fn test_missing_namespace() {
        let store = KvStore::new();
        assert!(store.get("nonexistent", "k").is_none());
        assert!(store.keys("nonexistent").is_empty());
        assert_eq!(store.count("nonexistent"), 0);
    }

    #[test]
    fn test_created_at_preserved() {
        let store = KvStore::new();
        store.put("ns", "k", serde_json::json!(1));
        let created = store.get("ns", "k").unwrap().created_at;
        store.put("ns", "k", serde_json::json!(2));
        let after = store.get("ns", "k").unwrap();
        assert_eq!(after.created_at, created);
        assert!(after.updated_at >= created);
    }
}
