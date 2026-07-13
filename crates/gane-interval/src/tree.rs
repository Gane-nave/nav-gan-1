//! Interval tree implementation using a sorted list with augmented max values.

use crate::interval::Interval;

/// An entry in the interval tree with augmented max value.
#[derive(Debug, Clone)]
struct TreeEntry {
    interval: Interval,
    /// Maximum high value in this entry's subtree (for pruning).
    subtree_max: i64,
}

/// An interval tree supporting efficient overlap and point queries.
///
/// Uses a sorted vector with augmented max values for efficient queries.
/// For the expected workload sizes in navigation, this provides good
/// cache locality and simplicity over a balanced BST approach.
#[derive(Debug)]
pub struct IntervalTree {
    entries: Vec<TreeEntry>,
    /// Whether the tree needs rebuilding (after insert/remove).
    dirty: bool,
    /// Total queries performed.
    queries: u64,
}

impl IntervalTree {
    /// Create a new empty interval tree.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            dirty: false,
            queries: 0,
        }
    }

    /// Insert an interval into the tree.
    pub fn insert(&mut self, interval: Interval) {
        self.entries.push(TreeEntry {
            subtree_max: interval.high(),
            interval,
        });
        self.dirty = true;
    }

    /// Remove all intervals with the given label.
    /// Returns the number of intervals removed.
    pub fn remove_by_label(&mut self, label: &str) -> usize {
        let before = self.entries.len();
        self.entries.retain(|e| e.interval.label() != label);
        let removed = before - self.entries.len();
        if removed > 0 {
            self.dirty = true;
        }
        removed
    }

    /// Rebuild the augmented structure after modifications.
    fn rebuild(&mut self) {
        if !self.dirty {
            return;
        }
        // Sort by low endpoint
        self.entries.sort_by_key(|e| e.interval.low());
        // Recompute subtree_max from right to left
        let mut running_max = i64::MIN;
        for entry in self.entries.iter_mut().rev() {
            running_max = running_max.max(entry.interval.high());
            entry.subtree_max = running_max;
        }
        self.dirty = false;
    }

    /// Find all intervals that contain the given point.
    pub fn query_point(&mut self, point: i64) -> Vec<&Interval> {
        self.rebuild();
        self.queries = self.queries.saturating_add(1);
        let mut results = Vec::new();
        for entry in &self.entries {
            // Prune: if the minimum low in remaining entries > point, stop
            if entry.interval.low() > point {
                break;
            }
            if entry.interval.contains_point(point) {
                results.push(&entry.interval);
            }
        }
        results
    }

    /// Find all intervals that overlap with the given interval.
    pub fn query_overlap(&mut self, low: i64, high: i64) -> Vec<&Interval> {
        self.rebuild();
        self.queries = self.queries.saturating_add(1);
        let mut results = Vec::new();
        let query = Interval::new(low, high, "");
        for entry in &self.entries {
            // Prune: if entry.low > high, no more overlaps possible
            if entry.interval.low() > high {
                break;
            }
            if entry.interval.overlaps(&query) {
                results.push(&entry.interval);
            }
        }
        results
    }

    /// Find all intervals that fully contain the given interval.
    pub fn query_containing(&mut self, low: i64, high: i64) -> Vec<&Interval> {
        self.rebuild();
        self.queries = self.queries.saturating_add(1);
        let mut results = Vec::new();
        for entry in &self.entries {
            if entry.interval.low() > low {
                break;
            }
            if entry
                .interval
                .contains_interval(&Interval::new(low, high, ""))
            {
                results.push(&entry.interval);
            }
        }
        results
    }

    /// Get the number of intervals in the tree.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the total number of queries performed.
    pub fn queries(&self) -> u64 {
        self.queries
    }

    /// Get all intervals sorted by low endpoint.
    pub fn intervals(&mut self) -> Vec<&Interval> {
        self.rebuild();
        self.entries.iter().map(|e| &e.interval).collect()
    }

    /// Get the span of all intervals (min low, max high).
    /// Returns None if the tree is empty.
    pub fn span(&mut self) -> Option<(i64, i64)> {
        self.rebuild();
        if self.entries.is_empty() {
            return None;
        }
        let min_low = self.entries.first().map(|e| e.interval.low()).unwrap();
        let max_high = self.entries.first().map(|e| e.subtree_max).unwrap();
        Some((min_low, max_high))
    }

    /// Clear all intervals.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.dirty = false;
    }

    /// Check if any interval contains the given point.
    pub fn contains_point(&mut self, point: i64) -> bool {
        !self.query_point(point).is_empty()
    }

    /// Count how many intervals overlap with the given range.
    pub fn count_overlaps(&mut self, low: i64, high: i64) -> usize {
        self.query_overlap(low, high).len()
    }
}

impl Default for IntervalTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = IntervalTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_insert_and_len() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        assert_eq!(tree.len(), 2);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_point_query() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        tree.insert(Interval::new(20, 30, "c"));

        let at_7 = tree.query_point(7);
        assert_eq!(at_7.len(), 2);
        let labels: Vec<&str> = at_7.iter().map(|i| i.label()).collect();
        assert!(labels.contains(&"a"));
        assert!(labels.contains(&"b"));

        let at_25 = tree.query_point(25);
        assert_eq!(at_25.len(), 1);
        assert_eq!(at_25[0].label(), "c");

        let at_16 = tree.query_point(16);
        assert!(at_16.is_empty());
    }

    #[test]
    fn test_overlap_query() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        tree.insert(Interval::new(20, 30, "c"));

        let overlaps = tree.query_overlap(8, 12);
        assert_eq!(overlaps.len(), 2);

        let overlaps = tree.query_overlap(25, 35);
        assert_eq!(overlaps.len(), 1);

        let overlaps = tree.query_overlap(16, 19);
        assert!(overlaps.is_empty());
    }

    #[test]
    fn test_containing_query() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 100, "wide"));
        tree.insert(Interval::new(10, 20, "narrow"));
        tree.insert(Interval::new(5, 50, "medium"));

        let containers = tree.query_containing(10, 20);
        assert_eq!(containers.len(), 3); // wide, medium, and narrow (exact match)
        let labels: Vec<&str> = containers.iter().map(|i| i.label()).collect();
        assert!(labels.contains(&"wide"));
        assert!(labels.contains(&"medium"));
        assert!(labels.contains(&"narrow"));
    }

    #[test]
    fn test_remove_by_label() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        tree.insert(Interval::new(20, 30, "a"));
        assert_eq!(tree.remove_by_label("a"), 2);
        assert_eq!(tree.len(), 1);
        let all = tree.intervals();
        assert_eq!(all[0].label(), "b");
    }

    #[test]
    fn test_span() {
        let mut tree = IntervalTree::new();
        assert!(tree.span().is_none());
        tree.insert(Interval::new(10, 20, "a"));
        tree.insert(Interval::new(5, 30, "b"));
        tree.insert(Interval::new(15, 25, "c"));
        assert_eq!(tree.span(), Some((5, 30)));
    }

    #[test]
    fn test_clear() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        tree.clear();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_contains_point() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(10, 20, "a"));
        assert!(tree.contains_point(15));
        assert!(!tree.contains_point(5));
    }

    #[test]
    fn test_count_overlaps() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(5, 15, "b"));
        tree.insert(Interval::new(20, 30, "c"));
        assert_eq!(tree.count_overlaps(8, 12), 2);
        assert_eq!(tree.count_overlaps(16, 19), 0);
    }

    #[test]
    fn test_query_counter() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(0, 10, "a"));
        assert_eq!(tree.queries(), 0);
        tree.query_point(5);
        tree.query_overlap(0, 5);
        assert_eq!(tree.queries(), 2);
    }

    #[test]
    fn test_default() {
        let tree = IntervalTree::default();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_intervals_sorted() {
        let mut tree = IntervalTree::new();
        tree.insert(Interval::new(20, 30, "c"));
        tree.insert(Interval::new(0, 10, "a"));
        tree.insert(Interval::new(10, 20, "b"));
        let intervals = tree.intervals();
        assert_eq!(intervals[0].low(), 0);
        assert_eq!(intervals[1].low(), 10);
        assert_eq!(intervals[2].low(), 20);
    }
}
