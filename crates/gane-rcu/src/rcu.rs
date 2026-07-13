//! Read-Copy-Update implementation.

use std::sync::Arc;

/// A cell that supports lock-free reads via Read-Copy-Update semantics.
/// Writers create a new version; readers see a consistent snapshot.
pub struct RcuCell<T> {
    current: Arc<T>,
    versions: Vec<Arc<T>>,
    max_versions: usize,
}

impl<T: Clone> RcuCell<T> {
    /// Create a new RCU cell with the given initial value.
    pub fn new(value: T) -> Self {
        let arc = Arc::new(value);
        Self {
            current: arc.clone(),
            versions: vec![arc],
            max_versions: 8,
        }
    }

    /// Create with a specific max version history size.
    pub fn with_max_versions(value: T, max_versions: usize) -> Self {
        let arc = Arc::new(value);
        Self {
            current: arc.clone(),
            versions: vec![arc],
            max_versions: max_versions.max(1),
        }
    }

    /// Read the current value (lock-free).
    pub fn read(&self) -> &T {
        &self.current
    }

    /// Get a shared reference to the current value.
    pub fn read_arc(&self) -> Arc<T> {
        self.current.clone()
    }

    /// Update the value by applying a function to the current value.
    /// Creates a new version (copy-on-write).
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&T) -> T,
    {
        let new_value = f(&self.current);
        let new_arc = Arc::new(new_value);
        self.current = new_arc.clone();
        self.versions.push(new_arc);
        if self.versions.len() > self.max_versions {
            self.versions.remove(0);
        }
    }

    /// Replace the value entirely.
    pub fn replace(&mut self, value: T) {
        let new_arc = Arc::new(value);
        self.current = new_arc.clone();
        self.versions.push(new_arc);
        if self.versions.len() > self.max_versions {
            self.versions.remove(0);
        }
    }

    /// Number of retained versions.
    pub fn version_count(&self) -> usize {
        self.versions.len()
    }

    /// Maximum number of retained versions.
    pub fn max_versions(&self) -> usize {
        self.max_versions
    }

    /// Reclaim old versions, keeping only the current one.
    pub fn reclaim(&mut self) {
        self.versions.clear();
        self.versions.push(self.current.clone());
    }

    /// Get a snapshot of all retained version values.
    pub fn version_history(&self) -> Vec<Arc<T>> {
        self.versions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_read() {
        let cell = RcuCell::new(42);
        assert_eq!(*cell.read(), 42);
    }

    #[test]
    fn test_update() {
        let mut cell = RcuCell::new(10);
        cell.update(|v| v + 5);
        assert_eq!(*cell.read(), 15);
    }

    #[test]
    fn test_replace() {
        let mut cell = RcuCell::new("hello");
        cell.replace("world");
        assert_eq!(*cell.read(), "world");
    }

    #[test]
    fn test_version_count() {
        let mut cell = RcuCell::new(0);
        assert_eq!(cell.version_count(), 1);
        cell.update(|v| v + 1);
        assert_eq!(cell.version_count(), 2);
    }

    #[test]
    fn test_max_versions() {
        let mut cell = RcuCell::with_max_versions(0, 3);
        cell.update(|v| v + 1);
        cell.update(|v| v + 1);
        cell.update(|v| v + 1);
        cell.update(|v| v + 1);
        assert!(cell.version_count() <= 3);
    }

    #[test]
    fn test_read_arc() {
        let cell = RcuCell::new(42);
        let arc = cell.read_arc();
        assert_eq!(*arc, 42);
    }

    #[test]
    fn test_reclaim() {
        let mut cell = RcuCell::new(0);
        cell.update(|v| v + 1);
        cell.update(|v| v + 1);
        cell.reclaim();
        assert_eq!(cell.version_count(), 1);
        assert_eq!(*cell.read(), 2);
    }

    #[test]
    fn test_version_history() {
        let mut cell = RcuCell::new(0);
        cell.update(|v| v + 1);
        cell.update(|v| v + 1);
        let history = cell.version_history();
        assert_eq!(history.len(), 3);
        assert_eq!(*history[0], 0);
        assert_eq!(*history[1], 1);
        assert_eq!(*history[2], 2);
    }

    #[test]
    fn test_multiple_updates() {
        let mut cell = RcuCell::new(0);
        for _ in 0..100 {
            cell.update(|v| v + 1);
        }
        assert_eq!(*cell.read(), 100);
    }

    #[test]
    fn test_with_string() {
        let mut cell = RcuCell::new(String::from("hello"));
        cell.update(|s| format!("{} world", s));
        assert_eq!(cell.read().as_str(), "hello world");
    }
}
