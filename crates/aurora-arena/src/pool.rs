use crate::handle::{ArenaHandle, ArenaSlot, SlotStatus};

/// An arena/pool allocator for efficient object reuse.
///
/// Pre-allocates a fixed number of slots. Objects are allocated from
/// free slots and returned to a free list when freed, avoiding
/// heap allocation churn.
#[derive(Debug)]
pub struct ArenaPool<T: Clone> {
    slots: Vec<ArenaSlot<T>>,
    free_list: Vec<usize>,
    capacity: usize,
    allocated: usize,
    total_allocs: u64,
    total_frees: u64,
    total_reuses: u64,
    peak_allocated: usize,
}

impl<T: Clone> ArenaPool<T> {
    /// Create a new arena pool with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity);
        let mut free_list = Vec::with_capacity(capacity);
        for i in 0..capacity {
            slots.push(ArenaSlot::new());
            free_list.push(capacity - 1 - i); // reverse order so pop gives low indices first
        }
        Self {
            slots,
            free_list,
            capacity,
            allocated: 0,
            total_allocs: 0,
            total_frees: 0,
            total_reuses: 0,
            peak_allocated: 0,
        }
    }

    /// Allocate an object from the pool.
    /// Returns None if the pool is exhausted.
    pub fn alloc(&mut self, value: T, now_ms: u64) -> Option<ArenaHandle> {
        let index = self.free_list.pop()?;
        let mut handle = self.slots[index].allocate(value, now_ms);
        // Fix the handle index
        handle = ArenaHandle::new(index, handle.generation());

        self.allocated += 1;
        self.total_allocs = self.total_allocs.saturating_add(1);
        if self.allocated > self.peak_allocated {
            self.peak_allocated = self.allocated;
        }

        // Check if this slot was recycled (reused)
        if self.slots[index].generation() > 1 {
            self.total_reuses = self.total_reuses.saturating_add(1);
        }

        Some(handle)
    }

    /// Free an object by its handle. Returns the value if the handle is valid.
    pub fn free(&mut self, handle: ArenaHandle, now_ms: u64) -> Option<T> {
        let index = handle.index();
        if index >= self.capacity {
            return None;
        }
        let slot = &self.slots[index];
        if slot.generation() != handle.generation() {
            return None; // stale handle
        }
        if *slot.status() != SlotStatus::Occupied {
            return None;
        }
        let value = self.slots[index].free(now_ms);
        if value.is_some() {
            self.allocated -= 1;
            self.total_frees = self.total_frees.saturating_add(1);
            self.free_list.push(index);
        }
        value
    }

    /// Get a reference to the object by its handle.
    pub fn get(&self, handle: ArenaHandle) -> Option<&T> {
        let index = handle.index();
        if index >= self.capacity {
            return None;
        }
        let slot = &self.slots[index];
        if slot.generation() != handle.generation() {
            return None;
        }
        slot.get()
    }

    /// Get a mutable reference to the object by its handle.
    pub fn get_mut(&mut self, handle: ArenaHandle) -> Option<&mut T> {
        let index = handle.index();
        if index >= self.capacity {
            return None;
        }
        let gen = handle.generation();
        let slot = &mut self.slots[index];
        if slot.generation() != gen {
            return None;
        }
        slot.get_mut()
    }

    /// Get the number of currently allocated objects.
    pub fn allocated(&self) -> usize {
        self.allocated
    }

    /// Get the total capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of free slots.
    pub fn available(&self) -> usize {
        self.free_list.len()
    }

    /// Get the fill ratio (allocated / capacity).
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.allocated as f64 / self.capacity as f64
    }

    /// Get total allocations.
    pub fn total_allocs(&self) -> u64 {
        self.total_allocs
    }

    /// Get total frees.
    pub fn total_frees(&self) -> u64 {
        self.total_frees
    }

    /// Get total reuses (allocations into previously-used slots).
    pub fn total_reuses(&self) -> u64 {
        self.total_reuses
    }

    /// Get peak concurrent allocations.
    pub fn peak_allocated(&self) -> usize {
        self.peak_allocated
    }

    /// Get the reuse ratio (reuses / total allocs).
    pub fn reuse_ratio(&self) -> f64 {
        if self.total_allocs == 0 {
            return 0.0;
        }
        self.total_reuses as f64 / self.total_allocs as f64
    }

    /// Clear all allocations, returning all slots to the free list.
    pub fn clear(&mut self) {
        self.free_list.clear();
        for (i, slot) in self.slots.iter_mut().enumerate() {
            slot.mark_free();
            self.free_list.push(self.capacity - 1 - i);
        }
        self.allocated = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_and_get() {
        let mut pool: ArenaPool<String> = ArenaPool::new(4);
        let h = pool.alloc("hello".to_string(), 100).unwrap();
        assert_eq!(pool.get(h), Some(&"hello".to_string()));
        assert_eq!(pool.allocated(), 1);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_free_and_reuse() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(2);
        let h1 = pool.alloc(10, 100).unwrap();
        let h2 = pool.alloc(20, 200).unwrap();
        assert_eq!(pool.available(), 0);

        // Free h1
        let val = pool.free(h1, 300);
        assert_eq!(val, Some(10));
        assert_eq!(pool.available(), 1);

        // Reuse the slot
        let h3 = pool.alloc(30, 400).unwrap();
        assert_eq!(pool.get(h3), Some(&30));
        assert_eq!(pool.total_reuses(), 1);

        // Stale handle h1 should not work
        assert_eq!(pool.get(h1), None);
        // h2 still valid
        assert_eq!(pool.get(h2), Some(&20));
    }

    #[test]
    fn test_exhaustion() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(2);
        pool.alloc(1, 100).unwrap();
        pool.alloc(2, 200).unwrap();
        assert!(pool.alloc(3, 300).is_none());
    }

    #[test]
    fn test_stale_handle() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(2);
        let h1 = pool.alloc(10, 100).unwrap();
        pool.free(h1, 200);
        let _h2 = pool.alloc(20, 300).unwrap();
        // h1 is stale — generation mismatch
        assert_eq!(pool.get(h1), None);
        assert_eq!(pool.free(h1, 400), None);
    }

    #[test]
    fn test_fill_ratio() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(4);
        pool.alloc(1, 100);
        pool.alloc(2, 200);
        assert!((pool.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_peak_allocated() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(4);
        let h1 = pool.alloc(1, 100).unwrap();
        let h2 = pool.alloc(2, 200).unwrap();
        let _h3 = pool.alloc(3, 300).unwrap();
        assert_eq!(pool.peak_allocated(), 3);
        pool.free(h1, 400);
        pool.free(h2, 500);
        assert_eq!(pool.peak_allocated(), 3); // peak doesn't decrease
        assert_eq!(pool.allocated(), 1);
    }

    #[test]
    fn test_get_mut() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(2);
        let h = pool.alloc(10, 100).unwrap();
        if let Some(val) = pool.get_mut(h) {
            *val = 99;
        }
        assert_eq!(pool.get(h), Some(&99));
    }

    #[test]
    fn test_clear() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(3);
        pool.alloc(1, 100);
        pool.alloc(2, 200);
        pool.alloc(3, 300);
        assert_eq!(pool.allocated(), 3);
        pool.clear();
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_double_free() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(2);
        let h = pool.alloc(10, 100).unwrap();
        assert!(pool.free(h, 200).is_some());
        assert!(pool.free(h, 300).is_none()); // already freed
    }

    #[test]
    fn test_stats() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(4);
        let h1 = pool.alloc(1, 100).unwrap();
        pool.alloc(2, 200);
        pool.free(h1, 300);
        assert_eq!(pool.total_allocs(), 2);
        assert_eq!(pool.total_frees(), 1);
    }

    #[test]
    fn test_zero_capacity() {
        let mut pool: ArenaPool<u32> = ArenaPool::new(0);
        assert!(pool.alloc(1, 100).is_none());
        assert_eq!(pool.fill_ratio(), 0.0);
    }
}
