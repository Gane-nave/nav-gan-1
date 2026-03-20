//! Object pool implementation.

use std::collections::VecDeque;

/// A generic object pool that recycles pre-allocated objects.
///
/// Objects are checked out from the pool and returned when done.
/// If the pool is empty, new objects are created via the factory.
#[derive(Debug)]
pub struct ObjectPool<T> {
    /// Available objects ready for reuse.
    available: VecDeque<T>,
    /// Maximum pool capacity.
    capacity: usize,
    /// Total objects created (including those currently checked out).
    created: usize,
    /// Total checkouts performed.
    checkouts: u64,
    /// Total returns performed.
    returns: u64,
}

impl<T> ObjectPool<T> {
    /// Create a new empty pool with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            available: VecDeque::with_capacity(capacity),
            capacity,
            created: 0,
            checkouts: 0,
            returns: 0,
        }
    }

    /// Pre-fill the pool with objects created by the factory.
    pub fn prefill<F>(&mut self, count: usize, mut factory: F)
    where
        F: FnMut() -> T,
    {
        let to_create = count.min(self.capacity.saturating_sub(self.available.len()));
        for _ in 0..to_create {
            self.available.push_back(factory());
            self.created = self.created.saturating_add(1);
        }
    }

    /// Check out an object from the pool.
    /// If the pool is empty, creates a new one via the factory.
    /// Returns `None` if the pool is at capacity and all objects are checked out.
    pub fn checkout<F>(&mut self, factory: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        self.checkouts = self.checkouts.saturating_add(1);
        if let Some(obj) = self.available.pop_front() {
            return Some(obj);
        }
        // Pool is empty — create new if under capacity.
        if self.created < self.capacity {
            self.created = self.created.saturating_add(1);
            Some(factory())
        } else {
            None
        }
    }

    /// Return an object to the pool for reuse.
    /// Returns `false` if the pool is full (object is dropped).
    pub fn return_obj(&mut self, obj: T) -> bool {
        self.returns = self.returns.saturating_add(1);
        if self.available.len() < self.capacity {
            self.available.push_back(obj);
            true
        } else {
            false
        }
    }

    /// Number of objects currently available in the pool.
    pub fn available(&self) -> usize {
        self.available.len()
    }

    /// Maximum capacity of the pool.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Total objects created by this pool.
    pub fn created(&self) -> usize {
        self.created
    }

    /// Number of objects currently checked out.
    pub fn checked_out(&self) -> usize {
        self.created.saturating_sub(self.available.len())
    }

    /// Total checkout operations.
    pub fn total_checkouts(&self) -> u64 {
        self.checkouts
    }

    /// Total return operations.
    pub fn total_returns(&self) -> u64 {
        self.returns
    }

    /// Clear all available objects from the pool.
    pub fn clear(&mut self) {
        self.available.clear();
    }

    /// Check if the pool is empty (no available objects).
    pub fn is_empty(&self) -> bool {
        self.available.is_empty()
    }

    /// Drain all available objects from the pool.
    pub fn drain(&mut self) -> Vec<T> {
        self.available.drain(..).collect()
    }

    /// Shrink the pool capacity.
    pub fn shrink_to(&mut self, new_capacity: usize) {
        self.capacity = new_capacity;
        while self.available.len() > new_capacity {
            self.available.pop_back();
        }
    }
}

impl<T> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pool() {
        let pool: ObjectPool<Vec<u8>> = ObjectPool::new(10);
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.available(), 0);
        assert_eq!(pool.created(), 0);
        assert!(pool.is_empty());
    }

    #[test]
    fn test_prefill() {
        let mut pool = ObjectPool::new(5);
        pool.prefill(3, || vec![0u8; 1024]);
        assert_eq!(pool.available(), 3);
        assert_eq!(pool.created(), 3);
    }

    #[test]
    fn test_prefill_capped_at_capacity() {
        let mut pool = ObjectPool::new(3);
        pool.prefill(10, || vec![0u8; 1024]);
        assert_eq!(pool.available(), 3);
        assert_eq!(pool.created(), 3);
    }

    #[test]
    fn test_checkout_from_prefilled() {
        let mut pool = ObjectPool::new(5);
        pool.prefill(3, || vec![0u8; 4]);
        let obj = pool.checkout(|| vec![0u8; 4]);
        assert!(obj.is_some());
        assert_eq!(pool.available(), 2);
        assert_eq!(pool.total_checkouts(), 1);
    }

    #[test]
    fn test_checkout_creates_new() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        let obj = pool.checkout(|| 42);
        assert_eq!(obj, Some(42));
        assert_eq!(pool.created(), 1);
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_checkout_at_capacity_returns_none() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(2);
        let _a = pool.checkout(|| 1);
        let _b = pool.checkout(|| 2);
        // Both checked out, pool at capacity
        let c = pool.checkout(|| 3);
        assert!(c.is_none());
    }

    #[test]
    fn test_return_obj() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        let obj = pool.checkout(|| 42).unwrap();
        assert!(pool.return_obj(obj));
        assert_eq!(pool.available(), 1);
        assert_eq!(pool.total_returns(), 1);
    }

    #[test]
    fn test_checkout_return_cycle() {
        let mut pool: ObjectPool<String> = ObjectPool::new(2);
        let a = pool.checkout(|| "hello".to_string()).unwrap();
        let b = pool.checkout(|| "world".to_string()).unwrap();
        pool.return_obj(a);
        pool.return_obj(b);
        assert_eq!(pool.available(), 2);
        // Reuse
        let c = pool.checkout(|| "new".to_string()).unwrap();
        assert_eq!(c, "hello"); // FIFO order
    }

    #[test]
    fn test_checked_out_count() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        pool.prefill(3, || 0);
        assert_eq!(pool.checked_out(), 0);
        let _a = pool.checkout(|| 0);
        assert_eq!(pool.checked_out(), 1);
        let _b = pool.checkout(|| 0);
        assert_eq!(pool.checked_out(), 2);
    }

    #[test]
    fn test_clear() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        pool.prefill(3, || 0);
        pool.clear();
        assert!(pool.is_empty());
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_drain() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        pool.prefill(3, || 99);
        let drained = pool.drain();
        assert_eq!(drained.len(), 3);
        assert!(drained.iter().all(|&v| v == 99));
        assert!(pool.is_empty());
    }

    #[test]
    fn test_shrink_to() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(10);
        pool.prefill(8, || 0);
        pool.shrink_to(3);
        assert_eq!(pool.capacity(), 3);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_default() {
        let pool: ObjectPool<u32> = ObjectPool::default();
        assert_eq!(pool.capacity(), 64);
    }

    #[test]
    fn test_return_full_pool() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(1);
        pool.prefill(1, || 0);
        // Pool is full, return should fail
        assert!(!pool.return_obj(99));
    }

    #[test]
    fn test_stats() {
        let mut pool: ObjectPool<u32> = ObjectPool::new(5);
        for i in 0..3 {
            let obj = pool.checkout(|| i).unwrap();
            pool.return_obj(obj);
        }
        assert_eq!(pool.total_checkouts(), 3);
        assert_eq!(pool.total_returns(), 3);
    }
}
