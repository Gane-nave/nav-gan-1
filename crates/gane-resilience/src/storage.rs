//! Storage budget — manages device storage allocation across offline data categories.
//!
//! Tracks storage usage per category, enforces budgets, and triggers
//! eviction when thresholds are exceeded.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Category of offline data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageCategory {
    /// Map tiles and routing graphs.
    Maps,
    /// Navigation evidence (photos, reports).
    Evidence,
    /// Telemetry and audit logs.
    Telemetry,
    /// Cached route plans.
    Routes,
    /// Offline sync queue.
    SyncQueue,
    /// Sensor recordings (IMU, GNSS raw).
    SensorData,
    /// User preferences and settings.
    UserData,
}

/// A storage allocation for a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAllocation {
    pub category: StorageCategory,
    /// Maximum bytes allocated to this category.
    pub budget_bytes: u64,
    /// Current usage in bytes.
    pub used_bytes: u64,
    /// Number of items stored.
    pub item_count: u64,
    /// Whether eviction is enabled for this category.
    pub eviction_enabled: bool,
    /// Eviction threshold (0.0–1.0): start evicting when usage exceeds this fraction.
    pub eviction_threshold: f64,
}

/// An eviction event record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvictionEvent {
    pub id: EntityId,
    pub category: StorageCategory,
    pub bytes_freed: u64,
    pub items_evicted: u64,
    pub triggered_at: DateTime<Utc>,
    pub reason: String,
}

/// Storage budget manager.
pub struct StorageBudget {
    allocations: HashMap<StorageCategory, StorageAllocation>,
    total_budget_bytes: u64,
    eviction_history: Vec<EvictionEvent>,
    total_bytes_evicted: u64,
}

impl StorageBudget {
    pub fn new(total_budget_bytes: u64) -> Self {
        Self {
            allocations: HashMap::new(),
            total_budget_bytes,
            eviction_history: Vec::new(),
            total_bytes_evicted: 0,
        }
    }

    /// Set the budget for a category.
    pub fn set_allocation(
        &mut self,
        category: StorageCategory,
        budget_bytes: u64,
        eviction_enabled: bool,
        eviction_threshold: f64,
    ) {
        let alloc = StorageAllocation {
            category,
            budget_bytes,
            used_bytes: 0,
            item_count: 0,
            eviction_enabled,
            eviction_threshold: eviction_threshold.clamp(0.0, 1.0),
        };
        debug!(
            category = ?category,
            budget = budget_bytes,
            "storage allocation set"
        );
        self.allocations.insert(category, alloc);
    }

    /// Record storage usage for a category. Returns true if within budget.
    pub fn record_usage(&mut self, category: StorageCategory, bytes: u64) -> bool {
        let Some(alloc) = self.allocations.get_mut(&category) else {
            return false;
        };

        alloc.used_bytes += bytes;
        alloc.item_count += 1;

        let within_budget = alloc.used_bytes <= alloc.budget_bytes;
        if !within_budget {
            warn!(
                category = ?category,
                used = alloc.used_bytes,
                budget = alloc.budget_bytes,
                "storage budget exceeded"
            );
        }
        within_budget
    }

    /// Release storage for a category.
    pub fn release_usage(&mut self, category: StorageCategory, bytes: u64) {
        if let Some(alloc) = self.allocations.get_mut(&category) {
            alloc.used_bytes = alloc.used_bytes.saturating_sub(bytes);
            if alloc.item_count > 0 {
                alloc.item_count -= 1;
            }
        }
    }

    /// Check if eviction is needed for a category.
    pub fn needs_eviction(&self, category: StorageCategory) -> bool {
        let Some(alloc) = self.allocations.get(&category) else {
            return false;
        };
        if !alloc.eviction_enabled || alloc.budget_bytes == 0 {
            return false;
        }
        let usage_ratio = alloc.used_bytes as f64 / alloc.budget_bytes as f64;
        usage_ratio > alloc.eviction_threshold
    }

    /// Perform eviction for a category. Returns bytes freed.
    /// In production, this would delete actual data; here we simulate.
    pub fn perform_eviction(
        &mut self,
        category: StorageCategory,
        bytes_to_free: u64,
        items_to_evict: u64,
    ) -> u64 {
        let Some(alloc) = self.allocations.get_mut(&category) else {
            return 0;
        };

        let freed = bytes_to_free.min(alloc.used_bytes);
        let actual_items_evicted = items_to_evict.min(alloc.item_count);
        alloc.used_bytes = alloc.used_bytes.saturating_sub(freed);
        alloc.item_count = alloc.item_count.saturating_sub(actual_items_evicted);

        let event = EvictionEvent {
            id: EntityId::new(),
            category,
            bytes_freed: freed,
            items_evicted: actual_items_evicted,
            triggered_at: Utc::now(),
            reason: format!(
                "usage exceeded threshold ({:.0}%)",
                alloc.eviction_threshold * 100.0
            ),
        };

        info!(
            category = ?category,
            freed = freed,
            items = items_to_evict,
            "eviction performed"
        );

        self.eviction_history.push(event);
        self.total_bytes_evicted += freed;
        freed
    }

    /// Get usage percentage for a category.
    pub fn usage_pct(&self, category: StorageCategory) -> f64 {
        let Some(alloc) = self.allocations.get(&category) else {
            return 0.0;
        };
        if alloc.budget_bytes == 0 {
            return 0.0;
        }
        alloc.used_bytes as f64 / alloc.budget_bytes as f64 * 100.0
    }

    /// Get total storage used across all categories.
    pub fn total_used(&self) -> u64 {
        self.allocations.values().map(|a| a.used_bytes).sum()
    }

    /// Get total budget.
    pub fn total_budget(&self) -> u64 {
        self.total_budget_bytes
    }

    /// Get allocation for a category.
    pub fn allocation(&self, category: StorageCategory) -> Option<&StorageAllocation> {
        self.allocations.get(&category)
    }

    /// Get eviction history.
    pub fn eviction_history(&self) -> &[EvictionEvent] {
        &self.eviction_history
    }

    /// Total bytes evicted.
    pub fn total_bytes_evicted(&self) -> u64 {
        self.total_bytes_evicted
    }

    /// Number of categories configured.
    pub fn category_count(&self) -> usize {
        self.allocations.len()
    }

    /// Get categories that are over budget.
    pub fn over_budget_categories(&self) -> Vec<StorageCategory> {
        self.allocations
            .values()
            .filter(|a| a.used_bytes > a.budget_bytes)
            .map(|a| a.category)
            .collect()
    }
}

impl Default for StorageBudget {
    fn default() -> Self {
        let mut budget = Self::new(1_000_000_000); // 1 GB default

        budget.set_allocation(StorageCategory::Maps, 500_000_000, true, 0.9);
        budget.set_allocation(StorageCategory::Evidence, 100_000_000, true, 0.85);
        budget.set_allocation(StorageCategory::Telemetry, 50_000_000, true, 0.8);
        budget.set_allocation(StorageCategory::Routes, 50_000_000, true, 0.9);
        budget.set_allocation(StorageCategory::SyncQueue, 100_000_000, false, 1.0);
        budget.set_allocation(StorageCategory::SensorData, 150_000_000, true, 0.85);
        budget.set_allocation(StorageCategory::UserData, 50_000_000, false, 1.0);

        budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_budget_has_7_categories() {
        let budget = StorageBudget::default();
        assert_eq!(budget.category_count(), 7);
        assert_eq!(budget.total_budget(), 1_000_000_000);
    }

    #[test]
    fn record_usage_within_budget() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 500_000, true, 0.9);

        assert!(budget.record_usage(StorageCategory::Maps, 100_000));
        assert_eq!(budget.usage_pct(StorageCategory::Maps), 20.0);
    }

    #[test]
    fn record_usage_exceeds_budget() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 100, true, 0.9);

        assert!(budget.record_usage(StorageCategory::Maps, 50));
        assert!(!budget.record_usage(StorageCategory::Maps, 200));
    }

    #[test]
    fn release_usage() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Telemetry, 100_000, true, 0.9);

        budget.record_usage(StorageCategory::Telemetry, 50_000);
        budget.release_usage(StorageCategory::Telemetry, 30_000);

        let alloc = budget.allocation(StorageCategory::Telemetry).unwrap();
        assert_eq!(alloc.used_bytes, 20_000);
    }

    #[test]
    fn needs_eviction_above_threshold() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 1000, true, 0.8);

        budget.record_usage(StorageCategory::Maps, 900);
        assert!(budget.needs_eviction(StorageCategory::Maps));
    }

    #[test]
    fn needs_eviction_below_threshold() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 1000, true, 0.8);

        budget.record_usage(StorageCategory::Maps, 500);
        assert!(!budget.needs_eviction(StorageCategory::Maps));
    }

    #[test]
    fn eviction_disabled_never_triggers() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::SyncQueue, 100, false, 0.5);

        budget.record_usage(StorageCategory::SyncQueue, 99);
        assert!(!budget.needs_eviction(StorageCategory::SyncQueue));
    }

    #[test]
    fn perform_eviction_frees_space() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Telemetry, 100_000, true, 0.8);
        budget.record_usage(StorageCategory::Telemetry, 95_000);

        let freed = budget.perform_eviction(StorageCategory::Telemetry, 30_000, 5);
        assert_eq!(freed, 30_000);
        assert_eq!(budget.total_bytes_evicted(), 30_000);
        assert_eq!(budget.eviction_history().len(), 1);

        let alloc = budget.allocation(StorageCategory::Telemetry).unwrap();
        assert_eq!(alloc.used_bytes, 65_000);
    }

    #[test]
    fn eviction_cannot_free_more_than_used() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Routes, 100_000, true, 0.8);
        budget.record_usage(StorageCategory::Routes, 10_000);

        let freed = budget.perform_eviction(StorageCategory::Routes, 50_000, 100);
        assert_eq!(freed, 10_000); // Capped at used_bytes.
    }

    #[test]
    fn total_used_across_categories() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 500_000, true, 0.9);
        budget.set_allocation(StorageCategory::Routes, 200_000, true, 0.9);

        budget.record_usage(StorageCategory::Maps, 100_000);
        budget.record_usage(StorageCategory::Routes, 50_000);

        assert_eq!(budget.total_used(), 150_000);
    }

    #[test]
    fn over_budget_categories_listed() {
        let mut budget = StorageBudget::new(1_000_000);
        budget.set_allocation(StorageCategory::Maps, 100, true, 0.9);
        budget.set_allocation(StorageCategory::Routes, 100_000, true, 0.9);

        budget.record_usage(StorageCategory::Maps, 200);
        budget.record_usage(StorageCategory::Routes, 50);

        let over = budget.over_budget_categories();
        assert_eq!(over.len(), 1);
        assert_eq!(over[0], StorageCategory::Maps);
    }
}
