//! Slab allocator implementation for index-based allocation.

/// A slab allocator that manages fixed slots with index-based access.
///
/// Provides O(1) insert, remove, and get operations. Reuses freed
/// slots through a free list.
#[derive(Debug)]
pub struct Slab<T> {
    /// Storage entries. `None` means the slot is free.
    entries: Vec<Option<T>>,
    /// Free list: indices of available slots.
    free: Vec<usize>,
    /// Number of occupied slots.
    count: usize,
    /// Total insertions.
    insertions: u64,
    /// Total removals.
    removals: u64,
}

impl<T> Slab<T> {
    /// Create a new empty slab.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
            count: 0,
            insertions: 0,
            removals: 0,
        }
    }

    /// Create a slab with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            free: Vec::new(),
            count: 0,
            insertions: 0,
            removals: 0,
        }
    }

    /// Insert a value and return its index (key).
    pub fn insert(&mut self, value: T) -> usize {
        self.insertions = self.insertions.saturating_add(1);
        self.count = self.count.saturating_add(1);

        if let Some(idx) = self.free.pop() {
            self.entries[idx] = Some(value);
            idx
        } else {
            let idx = self.entries.len();
            self.entries.push(Some(value));
            idx
        }
    }

    /// Remove the value at the given index. Returns the value if present.
    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if idx >= self.entries.len() {
            return None;
        }
        let entry = self.entries[idx].take()?;
        self.free.push(idx);
        self.count = self.count.saturating_sub(1);
        self.removals = self.removals.saturating_add(1);
        Some(entry)
    }

    /// Get a reference to the value at the given index.
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.entries.get(idx)?.as_ref()
    }

    /// Get a mutable reference to the value at the given index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.entries.get_mut(idx)?.as_mut()
    }

    /// Check if an index is occupied.
    pub fn contains(&self, idx: usize) -> bool {
        idx < self.entries.len() && self.entries[idx].is_some()
    }

    /// Number of occupied slots.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Total allocated capacity (including free slots).
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// Number of free slots available for reuse.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }

    /// Total insertions.
    pub fn total_insertions(&self) -> u64 {
        self.insertions
    }

    /// Total removals.
    pub fn total_removals(&self) -> u64 {
        self.removals
    }

    /// Iterate over all occupied entries as (index, &value).
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.as_ref().map(|v| (i, v)))
    }

    /// Iterate over all occupied entries as (index, &mut value).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.entries
            .iter_mut()
            .enumerate()
            .filter_map(|(i, e)| e.as_mut().map(|v| (i, v)))
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.free.clear();
        self.count = 0;
    }

    /// Compact the slab by removing trailing free slots.
    /// Returns the number of slots reclaimed.
    pub fn compact(&mut self) -> usize {
        let before = self.entries.len();
        while self.entries.last().is_some_and(|e| e.is_none()) {
            self.entries.pop();
        }
        let after = self.entries.len();
        // Remove freed indices that are now out of bounds
        self.free.retain(|&idx| idx < after);
        before - after
    }

    /// Get all occupied indices.
    pub fn keys(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e.is_some() { Some(i) } else { None })
            .collect()
    }

    /// Get all values as a vector.
    pub fn values(&self) -> Vec<&T> {
        self.entries.iter().filter_map(|e| e.as_ref()).collect()
    }
}

impl<T> Default for Slab<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let slab: Slab<u32> = Slab::new();
        assert!(slab.is_empty());
        assert_eq!(slab.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut slab = Slab::new();
        let idx = slab.insert(42u32);
        assert_eq!(idx, 0);
        assert_eq!(slab.len(), 1);
        assert!(slab.contains(idx));
    }

    #[test]
    fn test_get() {
        let mut slab = Slab::new();
        let idx = slab.insert("hello");
        assert_eq!(slab.get(idx), Some(&"hello"));
        assert_eq!(slab.get(99), None);
    }

    #[test]
    fn test_get_mut() {
        let mut slab = Slab::new();
        let idx = slab.insert(10u32);
        *slab.get_mut(idx).unwrap() = 20;
        assert_eq!(slab.get(idx), Some(&20));
    }

    #[test]
    fn test_remove() {
        let mut slab = Slab::new();
        let idx = slab.insert(42u32);
        assert_eq!(slab.remove(idx), Some(42));
        assert!(!slab.contains(idx));
        assert!(slab.is_empty());
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut slab: Slab<u32> = Slab::new();
        assert_eq!(slab.remove(0), None);
    }

    #[test]
    fn test_reuse_slot() {
        let mut slab = Slab::new();
        let idx0 = slab.insert("a");
        let _idx1 = slab.insert("b");
        slab.remove(idx0);
        let idx2 = slab.insert("c");
        assert_eq!(idx2, idx0); // reused slot
        assert_eq!(slab.get(idx2), Some(&"c"));
        assert_eq!(slab.len(), 2);
    }

    #[test]
    fn test_sequential_indices() {
        let mut slab = Slab::new();
        assert_eq!(slab.insert(1u32), 0);
        assert_eq!(slab.insert(2), 1);
        assert_eq!(slab.insert(3), 2);
    }

    #[test]
    fn test_with_capacity() {
        let slab: Slab<u32> = Slab::with_capacity(100);
        assert!(slab.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut slab = Slab::new();
        slab.insert(10u32);
        slab.insert(20);
        slab.insert(30);
        slab.remove(1);
        let items: Vec<(usize, &u32)> = slab.iter().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], (0, &10));
        assert_eq!(items[1], (2, &30));
    }

    #[test]
    fn test_iter_mut() {
        let mut slab = Slab::new();
        slab.insert(1u32);
        slab.insert(2);
        for (_, val) in slab.iter_mut() {
            *val *= 10;
        }
        assert_eq!(slab.get(0), Some(&10));
        assert_eq!(slab.get(1), Some(&20));
    }

    #[test]
    fn test_clear() {
        let mut slab = Slab::new();
        slab.insert(1u32);
        slab.insert(2);
        slab.clear();
        assert!(slab.is_empty());
        assert_eq!(slab.capacity(), 0);
    }

    #[test]
    fn test_compact() {
        let mut slab = Slab::new();
        slab.insert(1u32);
        slab.insert(2);
        slab.insert(3);
        slab.remove(2); // last slot freed
        let reclaimed = slab.compact();
        assert_eq!(reclaimed, 1);
        assert_eq!(slab.capacity(), 2);
    }

    #[test]
    fn test_keys() {
        let mut slab = Slab::new();
        slab.insert(10u32);
        slab.insert(20);
        slab.insert(30);
        slab.remove(1);
        assert_eq!(slab.keys(), vec![0, 2]);
    }

    #[test]
    fn test_values() {
        let mut slab = Slab::new();
        slab.insert(10u32);
        slab.insert(20);
        let vals = slab.values();
        assert_eq!(vals, vec![&10, &20]);
    }

    #[test]
    fn test_free_count() {
        let mut slab = Slab::new();
        slab.insert(1u32);
        slab.insert(2);
        slab.remove(0);
        assert_eq!(slab.free_count(), 1);
    }

    #[test]
    fn test_stats() {
        let mut slab = Slab::new();
        slab.insert(1u32);
        slab.insert(2);
        slab.remove(0);
        assert_eq!(slab.total_insertions(), 2);
        assert_eq!(slab.total_removals(), 1);
    }

    #[test]
    fn test_default() {
        let slab: Slab<u32> = Slab::default();
        assert!(slab.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut slab = Slab::new();
        let idx = slab.insert(42u32);
        assert!(slab.contains(idx));
        assert!(!slab.contains(99));
    }
}
