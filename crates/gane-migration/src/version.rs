//! Schema version tracking and comparison.

use std::fmt;

/// A semantic version for schema migrations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SchemaVersion {
    /// Create a new schema version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Whether this version is compatible with another (same major).
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major
    }

    /// Whether this version is newer than another.
    pub fn is_newer_than(&self, other: &Self) -> bool {
        self > other
    }

    /// Bump major version (resets minor and patch).
    pub fn bump_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }

    /// Bump minor version (resets patch).
    pub fn bump_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }

    /// Bump patch version.
    pub fn bump_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Tracks the current schema version and history.
pub struct VersionTracker {
    current: SchemaVersion,
    history: Vec<SchemaVersion>,
}

impl VersionTracker {
    /// Create a new version tracker starting at the given version.
    pub fn new(initial: SchemaVersion) -> Self {
        let history = vec![initial.clone()];
        Self {
            current: initial,
            history,
        }
    }

    /// Get the current version.
    pub fn current(&self) -> &SchemaVersion {
        &self.current
    }

    /// Advance to a new version.
    pub fn advance(&mut self, version: SchemaVersion) -> bool {
        if version.is_newer_than(&self.current) {
            self.history.push(version.clone());
            self.current = version;
            true
        } else {
            false
        }
    }

    /// Roll back to a previous version by index.
    pub fn rollback_to(&mut self, index: usize) -> bool {
        if index < self.history.len() {
            self.current = self.history[index].clone();
            self.history.truncate(index + 1);
            true
        } else {
            false
        }
    }

    /// Number of versions in history.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Get version at a specific history index.
    pub fn version_at(&self, index: usize) -> Option<&SchemaVersion> {
        self.history.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_creation() {
        let v = SchemaVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_ordering() {
        let v1 = SchemaVersion::new(1, 0, 0);
        let v2 = SchemaVersion::new(1, 1, 0);
        let v3 = SchemaVersion::new(2, 0, 0);
        assert!(v2.is_newer_than(&v1));
        assert!(v3.is_newer_than(&v2));
        assert!(!v1.is_newer_than(&v2));
    }

    #[test]
    fn test_compatibility() {
        let v1 = SchemaVersion::new(1, 0, 0);
        let v2 = SchemaVersion::new(1, 5, 0);
        let v3 = SchemaVersion::new(2, 0, 0);
        assert!(v1.is_compatible_with(&v2));
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_bumps() {
        let v = SchemaVersion::new(1, 2, 3);
        assert_eq!(v.bump_major(), SchemaVersion::new(2, 0, 0));
        assert_eq!(v.bump_minor(), SchemaVersion::new(1, 3, 0));
        assert_eq!(v.bump_patch(), SchemaVersion::new(1, 2, 4));
    }

    #[test]
    fn test_tracker_advance() {
        let mut tracker = VersionTracker::new(SchemaVersion::new(1, 0, 0));
        assert!(tracker.advance(SchemaVersion::new(1, 1, 0)));
        assert_eq!(tracker.current().to_string(), "1.1.0");
        assert_eq!(tracker.history_len(), 2);
    }

    #[test]
    fn test_tracker_reject_older() {
        let mut tracker = VersionTracker::new(SchemaVersion::new(2, 0, 0));
        assert!(!tracker.advance(SchemaVersion::new(1, 0, 0)));
        assert_eq!(tracker.current().to_string(), "2.0.0");
    }

    #[test]
    fn test_tracker_rollback() {
        let mut tracker = VersionTracker::new(SchemaVersion::new(1, 0, 0));
        tracker.advance(SchemaVersion::new(1, 1, 0));
        tracker.advance(SchemaVersion::new(1, 2, 0));
        assert_eq!(tracker.history_len(), 3);

        assert!(tracker.rollback_to(1));
        assert_eq!(tracker.current().to_string(), "1.1.0");
        assert_eq!(tracker.history_len(), 2);
    }

    #[test]
    fn test_tracker_rollback_invalid() {
        let mut tracker = VersionTracker::new(SchemaVersion::new(1, 0, 0));
        assert!(!tracker.rollback_to(5));
    }

    #[test]
    fn test_version_at() {
        let mut tracker = VersionTracker::new(SchemaVersion::new(1, 0, 0));
        tracker.advance(SchemaVersion::new(2, 0, 0));
        assert_eq!(tracker.version_at(0), Some(&SchemaVersion::new(1, 0, 0)));
        assert_eq!(tracker.version_at(1), Some(&SchemaVersion::new(2, 0, 0)));
        assert_eq!(tracker.version_at(2), None);
    }
}
