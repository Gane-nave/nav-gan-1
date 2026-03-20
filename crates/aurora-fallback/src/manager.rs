use serde::{Deserialize, Serialize};

/// Core engine for Fallback hierarchy — GNSS to IMU to Cell to WiFi to trajectory.
pub struct FallbackManager {
    entries: Vec<Entry>,
    active: bool,
    counter: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub value: f64,
    pub timestamp_ms: u64,
    pub label: String,
}

impl FallbackManager {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            active: true,
            counter: 0,
        }
    }
    pub fn add(&mut self, value: f64, label: &str) -> u64 {
        self.counter += 1;
        self.entries.push(Entry {
            id: self.counter,
            value,
            timestamp_ms: 0,
            label: label.into(),
        });
        self.counter
    }
    pub fn get(&self, id: u64) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, v: bool) {
        self.active = v;
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn all(&self) -> &[Entry] {
        &self.entries
    }
    pub fn remove(&mut self, id: u64) -> bool {
        let len = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < len
    }
    pub fn total_value(&self) -> f64 {
        self.entries.iter().map(|e| e.value).sum()
    }
}

impl Default for FallbackManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_empty() {
        assert_eq!(FallbackManager::new().count(), 0);
    }
    #[test]
    fn default_impl() {
        assert!(FallbackManager::default().is_active());
    }
    #[test]
    fn add_get() {
        let mut e = FallbackManager::new();
        let id = e.add(1.0, "t");
        assert!(e.get(id).is_some());
    }
    #[test]
    fn count_inc() {
        let mut e = FallbackManager::new();
        e.add(1.0, "a");
        e.add(2.0, "b");
        assert_eq!(e.count(), 2);
    }
    #[test]
    fn remove_e() {
        let mut e = FallbackManager::new();
        let id = e.add(1.0, "x");
        assert!(e.remove(id));
        assert_eq!(e.count(), 0);
    }
    #[test]
    fn clear_e() {
        let mut e = FallbackManager::new();
        e.add(1.0, "a");
        e.clear();
        assert_eq!(e.count(), 0);
    }
    #[test]
    fn total() {
        let mut e = FallbackManager::new();
        e.add(10.0, "a");
        e.add(20.0, "b");
        assert_eq!(e.total_value(), 30.0);
    }
    #[test]
    fn active() {
        let mut e = FallbackManager::new();
        e.set_active(false);
        assert!(!e.is_active());
    }
}
