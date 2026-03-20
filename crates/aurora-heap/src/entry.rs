//! Heap entry — a prioritized item in the heap.

/// A heap entry with a priority and value.
#[derive(Debug, Clone)]
pub struct HeapEntry<T: Clone> {
    /// Priority (lower = higher priority in min-heap).
    priority: i64,
    /// Insertion sequence for stable ordering.
    sequence: u64,
    /// The stored value.
    value: T,
}

impl<T: Clone> HeapEntry<T> {
    /// Create a new heap entry.
    pub fn new(priority: i64, sequence: u64, value: T) -> Self {
        Self {
            priority,
            sequence,
            value,
        }
    }

    /// Get the priority.
    pub fn priority(&self) -> i64 {
        self.priority
    }

    /// Get the sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Get a reference to the value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consume the entry and return the value.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Compare two entries for min-heap ordering.
    /// Returns true if self should come before other.
    pub fn is_higher_priority_min(&self, other: &Self) -> bool {
        if self.priority != other.priority {
            self.priority < other.priority
        } else {
            self.sequence < other.sequence
        }
    }

    /// Compare two entries for max-heap ordering.
    /// Returns true if self should come before other.
    pub fn is_higher_priority_max(&self, other: &Self) -> bool {
        if self.priority != other.priority {
            self.priority > other.priority
        } else {
            self.sequence < other.sequence
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_creation() {
        let e = HeapEntry::new(5, 1, "hello");
        assert_eq!(e.priority(), 5);
        assert_eq!(e.sequence(), 1);
        assert_eq!(*e.value(), "hello");
    }

    #[test]
    fn test_entry_min_ordering() {
        let a = HeapEntry::new(1, 1, "low");
        let b = HeapEntry::new(5, 2, "high");
        assert!(a.is_higher_priority_min(&b));
        assert!(!b.is_higher_priority_min(&a));
    }

    #[test]
    fn test_entry_max_ordering() {
        let a = HeapEntry::new(1, 1, "low");
        let b = HeapEntry::new(5, 2, "high");
        assert!(b.is_higher_priority_max(&a));
        assert!(!a.is_higher_priority_max(&b));
    }

    #[test]
    fn test_entry_tie_breaking() {
        let a = HeapEntry::new(5, 1, "first");
        let b = HeapEntry::new(5, 2, "second");
        // Same priority — earlier sequence wins
        assert!(a.is_higher_priority_min(&b));
        assert!(a.is_higher_priority_max(&b));
    }

    #[test]
    fn test_entry_into_value() {
        let e = HeapEntry::new(1, 1, "consumed".to_string());
        let val = e.into_value();
        assert_eq!(val, "consumed");
    }

    #[test]
    fn test_entry_clone() {
        let e = HeapEntry::new(3, 10, 42u32);
        let c = e.clone();
        assert_eq!(c.priority(), 3);
        assert_eq!(c.sequence(), 10);
        assert_eq!(*c.value(), 42);
    }

    #[test]
    fn test_entry_negative_priority() {
        let a = HeapEntry::new(-10, 1, "urgent");
        let b = HeapEntry::new(0, 2, "normal");
        assert!(a.is_higher_priority_min(&b));
    }

    #[test]
    fn test_entry_debug() {
        let e = HeapEntry::new(1, 1, "test");
        let debug = format!("{:?}", e);
        assert!(debug.contains("HeapEntry"));
    }
}
