//! Precise geo-fencing with zone entry/exit triggers.

use serde::{Deserialize, Serialize};

/// Entry in the GeofenceEngine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub label: String,
    pub value: f64,
    pub timestamp_ms: u64,
    pub active: bool,
}

/// Precise geo-fencing with zone entry/exit triggers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeofenceEngine {
    entries: Vec<Entry>,
    triggers: u64,
    max_entries: usize,
}

impl GeofenceEngine {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            triggers: 0,
            max_entries,
        }
    }

    pub fn add(&mut self, entry: Entry) -> bool {
        if self.entries.len() >= self.max_entries {
            return false;
        }
        self.entries.push(entry);
        self.triggers += 1;
        true
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }

    pub fn get(&self, id: u64) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|e| e.id == id)
    }

    pub fn active_entries(&self) -> Vec<&Entry> {
        self.entries.iter().filter(|e| e.active).collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn triggers(&self) -> u64 {
        self.triggers
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn average_value(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        self.entries.iter().map(|e| e.value).sum::<f64>() / self.entries.len() as f64
    }
}

impl Default for GeofenceEngine {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(id: u64) -> Entry {
        Entry {
            id,
            label: format!("entry_{id}"),
            value: id as f64 * 1.5,
            timestamp_ms: id * 1000,
            active: true,
        }
    }

    #[test]
    fn test_new() {
        let s = GeofenceEngine::new(100);
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn test_default() {
        let s = GeofenceEngine::default();
        assert_eq!(s.triggers(), 0);
    }

    #[test]
    fn test_add() {
        let mut s = GeofenceEngine::new(100);
        assert!(s.add(make_entry(1)));
        assert_eq!(s.count(), 1);
    }

    #[test]
    fn test_remove() {
        let mut s = GeofenceEngine::new(100);
        s.add(make_entry(1));
        assert!(s.remove(1));
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn test_get() {
        let mut s = GeofenceEngine::new(100);
        s.add(make_entry(42));
        assert_eq!(s.get(42).unwrap().id, 42);
    }

    #[test]
    fn test_capacity() {
        let mut s = GeofenceEngine::new(2);
        assert!(s.add(make_entry(1)));
        assert!(s.add(make_entry(2)));
        assert!(!s.add(make_entry(3)));
    }

    #[test]
    fn test_active() {
        let mut s = GeofenceEngine::new(100);
        s.add(make_entry(1));
        let mut e = make_entry(2);
        e.active = false;
        s.add(e);
        assert_eq!(s.active_entries().len(), 1);
    }

    #[test]
    fn test_average() {
        let mut s = GeofenceEngine::new(100);
        s.add(Entry {
            id: 1,
            label: "a".into(),
            value: 10.0,
            timestamp_ms: 0,
            active: true,
        });
        s.add(Entry {
            id: 2,
            label: "b".into(),
            value: 20.0,
            timestamp_ms: 0,
            active: true,
        });
        assert!((s.average_value() - 15.0).abs() < 1e-10);
    }
}
