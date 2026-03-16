//! Memory management — pool allocator, budget tracking, pressure detection.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory pressure level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
    Emergency,
}

/// Memory budget for a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudget {
    pub name: String,
    pub budget_bytes: usize,
    pub used_bytes: usize,
    pub peak_bytes: usize,
    pub allocation_count: u64,
}

impl MemoryBudget {
    /// Create a new memory budget.
    pub fn new(name: &str, budget_bytes: usize) -> Self {
        Self {
            name: name.to_string(),
            budget_bytes,
            used_bytes: 0,
            peak_bytes: 0,
            allocation_count: 0,
        }
    }

    /// Try to allocate memory within the budget.
    pub fn allocate(&mut self, bytes: usize) -> bool {
        if self.used_bytes + bytes > self.budget_bytes {
            return false;
        }
        self.used_bytes += bytes;
        if self.used_bytes > self.peak_bytes {
            self.peak_bytes = self.used_bytes;
        }
        self.allocation_count += 1;
        true
    }

    /// Free allocated memory.
    pub fn free(&mut self, bytes: usize) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }

    /// Usage ratio (0.0 to 1.0).
    pub fn usage_ratio(&self) -> f64 {
        if self.budget_bytes == 0 {
            return 0.0;
        }
        self.used_bytes as f64 / self.budget_bytes as f64
    }

    /// Remaining bytes.
    pub fn remaining(&self) -> usize {
        self.budget_bytes.saturating_sub(self.used_bytes)
    }

    /// Current pressure level.
    pub fn pressure(&self) -> MemoryPressure {
        let ratio = self.usage_ratio();
        if ratio >= 0.95 {
            MemoryPressure::Emergency
        } else if ratio >= 0.85 {
            MemoryPressure::Critical
        } else if ratio >= 0.70 {
            MemoryPressure::Warning
        } else {
            MemoryPressure::Normal
        }
    }
}

/// Object pool — reuses pre-allocated objects to reduce allocation pressure.
pub struct ObjectPool<T> {
    pool: RwLock<Vec<T>>,
    capacity: usize,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    checkout_count: RwLock<u64>,
    return_count: RwLock<u64>,
}

impl<T> ObjectPool<T> {
    /// Create a new object pool with a factory function.
    pub fn new(capacity: usize, factory: impl Fn() -> T + Send + Sync + 'static) -> Self {
        let mut pool = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            pool.push(factory());
        }
        Self {
            pool: RwLock::new(pool),
            capacity,
            factory: Box::new(factory),
            checkout_count: RwLock::new(0),
            return_count: RwLock::new(0),
        }
    }

    /// Check out an object from the pool.
    pub fn checkout(&self) -> T {
        *self.checkout_count.write() += 1;
        self.pool.write().pop().unwrap_or_else(|| (self.factory)())
    }

    /// Return an object to the pool.
    pub fn checkin(&self, obj: T) {
        *self.return_count.write() += 1;
        let mut pool = self.pool.write();
        if pool.len() < self.capacity {
            pool.push(obj);
        }
        // Otherwise drop the object (pool is full)
    }

    /// Current pool size (available objects).
    pub fn available(&self) -> usize {
        self.pool.read().len()
    }

    /// Total checkouts.
    pub fn total_checkouts(&self) -> u64 {
        *self.checkout_count.read()
    }

    /// Total returns.
    pub fn total_returns(&self) -> u64 {
        *self.return_count.read()
    }
}

/// Memory manager — tracks budgets across subsystems.
pub struct MemoryManager {
    budgets: RwLock<HashMap<String, MemoryBudget>>,
    global_budget: usize,
}

impl MemoryManager {
    /// Create a new memory manager with a global budget.
    pub fn new(global_budget: usize) -> Self {
        Self {
            budgets: RwLock::new(HashMap::new()),
            global_budget,
        }
    }

    /// Register a subsystem budget.
    pub fn register_budget(&self, name: &str, budget_bytes: usize) {
        self.budgets
            .write()
            .insert(name.to_string(), MemoryBudget::new(name, budget_bytes));
    }

    /// Allocate memory for a subsystem.
    /// Both global and subsystem budget checks happen under a single write lock
    /// to prevent TOCTOU races.
    pub fn allocate(&self, subsystem: &str, bytes: usize) -> bool {
        let mut budgets = self.budgets.write();

        // Check global budget under the same lock
        let total_used: usize = budgets.values().map(|b| b.used_bytes).sum();
        if total_used + bytes > self.global_budget {
            return false;
        }

        if let Some(budget) = budgets.get_mut(subsystem) {
            budget.allocate(bytes)
        } else {
            false
        }
    }

    /// Free memory for a subsystem.
    pub fn free(&self, subsystem: &str, bytes: usize) {
        let mut budgets = self.budgets.write();
        if let Some(budget) = budgets.get_mut(subsystem) {
            budget.free(bytes);
        }
    }

    /// Total memory used across all subsystems.
    pub fn total_used(&self) -> usize {
        self.budgets.read().values().map(|b| b.used_bytes).sum()
    }

    /// Global memory remaining.
    pub fn global_remaining(&self) -> usize {
        self.global_budget.saturating_sub(self.total_used())
    }

    /// Get the highest pressure level across all subsystems.
    pub fn overall_pressure(&self) -> MemoryPressure {
        self.budgets
            .read()
            .values()
            .map(|b| b.pressure())
            .max()
            .unwrap_or(MemoryPressure::Normal)
    }

    /// Get budget info for a subsystem.
    pub fn budget_info(&self, subsystem: &str) -> Option<MemoryBudget> {
        self.budgets.read().get(subsystem).cloned()
    }

    /// Get all budget summaries.
    pub fn all_budgets(&self) -> Vec<MemoryBudget> {
        self.budgets.read().values().cloned().collect()
    }

    /// Number of registered subsystems.
    pub fn subsystem_count(&self) -> usize {
        self.budgets.read().len()
    }
}

/// Memory snapshot for diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub total_budget: usize,
    pub total_used: usize,
    pub total_remaining: usize,
    pub pressure: MemoryPressure,
    pub subsystems: Vec<MemoryBudget>,
}

impl MemoryManager {
    /// Take a snapshot of current memory state.
    pub fn snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            total_budget: self.global_budget,
            total_used: self.total_used(),
            total_remaining: self.global_remaining(),
            pressure: self.overall_pressure(),
            subsystems: self.all_budgets(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_budget_basic() {
        let mut budget = MemoryBudget::new("tiles", 1024);
        assert!(budget.allocate(512));
        assert_eq!(budget.used_bytes, 512);
        assert_eq!(budget.remaining(), 512);
    }

    #[test]
    fn test_memory_budget_exceed() {
        let mut budget = MemoryBudget::new("tiles", 100);
        assert!(budget.allocate(50));
        assert!(!budget.allocate(60)); // Would exceed
        assert_eq!(budget.used_bytes, 50);
    }

    #[test]
    fn test_memory_budget_free() {
        let mut budget = MemoryBudget::new("tiles", 1024);
        budget.allocate(512);
        budget.free(256);
        assert_eq!(budget.used_bytes, 256);
    }

    #[test]
    fn test_memory_budget_peak() {
        let mut budget = MemoryBudget::new("tiles", 1024);
        budget.allocate(800);
        budget.free(400);
        budget.allocate(200);
        assert_eq!(budget.peak_bytes, 800);
    }

    #[test]
    fn test_memory_pressure_levels() {
        let mut budget = MemoryBudget::new("test", 100);
        budget.allocate(50);
        assert_eq!(budget.pressure(), MemoryPressure::Normal);

        budget.allocate(21);
        assert_eq!(budget.pressure(), MemoryPressure::Warning);

        budget.allocate(15);
        assert_eq!(budget.pressure(), MemoryPressure::Critical);

        budget.allocate(10);
        assert_eq!(budget.pressure(), MemoryPressure::Emergency);
    }

    #[test]
    fn test_object_pool_checkout_checkin() {
        let pool: ObjectPool<Vec<u8>> = ObjectPool::new(3, Vec::new);
        assert_eq!(pool.available(), 3);

        let obj = pool.checkout();
        assert_eq!(pool.available(), 2);

        pool.checkin(obj);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_object_pool_factory_when_empty() {
        let pool: ObjectPool<Vec<u8>> = ObjectPool::new(1, Vec::new);
        let _a = pool.checkout();
        let _b = pool.checkout(); // Pool empty, uses factory
        assert_eq!(pool.available(), 0);
        assert_eq!(pool.total_checkouts(), 2);
    }

    #[test]
    fn test_object_pool_capacity_limit() {
        let pool: ObjectPool<u32> = ObjectPool::new(2, || 0);
        let _a = pool.checkout();
        let _b = pool.checkout();
        pool.checkin(1);
        pool.checkin(2);
        pool.checkin(3); // Exceeds capacity, should be dropped
        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn test_memory_manager_basic() {
        let mgr = MemoryManager::new(10000);
        mgr.register_budget("tiles", 5000);
        mgr.register_budget("routes", 3000);

        assert!(mgr.allocate("tiles", 2000));
        assert!(mgr.allocate("routes", 1000));
        assert_eq!(mgr.total_used(), 3000);
    }

    #[test]
    fn test_memory_manager_global_limit() {
        let mgr = MemoryManager::new(100);
        mgr.register_budget("a", 80);
        mgr.register_budget("b", 80);

        assert!(mgr.allocate("a", 60));
        // Global budget only has 40 left, but "b" wants 60
        assert!(!mgr.allocate("b", 60));
    }

    #[test]
    fn test_memory_manager_unknown_subsystem() {
        let mgr = MemoryManager::new(1000);
        assert!(!mgr.allocate("unknown", 100));
    }

    #[test]
    fn test_memory_manager_free() {
        let mgr = MemoryManager::new(1000);
        mgr.register_budget("tiles", 500);
        mgr.allocate("tiles", 300);
        mgr.free("tiles", 100);
        assert_eq!(mgr.total_used(), 200);
    }

    #[test]
    fn test_memory_manager_pressure() {
        let mgr = MemoryManager::new(10000);
        mgr.register_budget("tiles", 100);
        mgr.allocate("tiles", 96);
        assert_eq!(mgr.overall_pressure(), MemoryPressure::Emergency);
    }

    #[test]
    fn test_memory_snapshot() {
        let mgr = MemoryManager::new(1000);
        mgr.register_budget("tiles", 500);
        mgr.allocate("tiles", 200);

        let snap = mgr.snapshot();
        assert_eq!(snap.total_budget, 1000);
        assert_eq!(snap.total_used, 200);
        assert_eq!(snap.total_remaining, 800);
        assert_eq!(snap.subsystems.len(), 1);
    }

    #[test]
    fn test_usage_ratio_zero_budget() {
        let budget = MemoryBudget::new("empty", 0);
        assert!((budget.usage_ratio() - 0.0).abs() < f64::EPSILON);
    }
}
