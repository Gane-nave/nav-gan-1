//! Adversarial test: eviction items_evicted accuracy
//! BUG: perform_eviction recorded the *requested* items_to_evict in the event,
//! even when fewer items actually existed.
//! Fix: `let actual_items_evicted = items_to_evict.min(alloc.item_count);`

use aurora_resilience::storage::*;

#[test]
fn eviction_items_evicted_capped_at_actual_count() {
    let mut budget = StorageBudget::new(1_000_000);
    budget.set_allocation(StorageCategory::Telemetry, 100_000, true, 0.8);

    // Record 2 items worth of usage
    budget.record_usage(StorageCategory::Telemetry, 50_000);
    budget.record_usage(StorageCategory::Telemetry, 30_000);

    let alloc = budget.allocation(StorageCategory::Telemetry).unwrap();
    assert_eq!(alloc.item_count, 2, "Should have exactly 2 items");

    // Request eviction of 100 items — but only 2 exist
    budget.perform_eviction(StorageCategory::Telemetry, 80_000, 100);

    let history = budget.eviction_history();
    assert_eq!(history.len(), 1, "Should have 1 eviction event");

    assert_eq!(
        history[0].items_evicted, 2,
        "BUG FIX: items_evicted capped at actual count (2), not requested (100). Old code would report 100."
    );

    // Verify item_count is now 0 (not underflowed)
    let alloc = budget.allocation(StorageCategory::Telemetry).unwrap();
    assert_eq!(alloc.item_count, 0, "All items evicted");
}
