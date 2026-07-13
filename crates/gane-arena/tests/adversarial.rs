//! Adversarial tests for gane-arena

use gane_arena::pool::ArenaPool;

#[test]
fn adversarial_alloc_free_reuse_stale_handle() {
    let mut pool: ArenaPool<String> = ArenaPool::new(3);

    // Allocate all 3 slots
    let h1 = pool.alloc("alpha".to_string(), 1000).unwrap();
    let h2 = pool.alloc("beta".to_string(), 2000).unwrap();
    let h3 = pool.alloc("gamma".to_string(), 3000).unwrap();
    assert_eq!(pool.allocated(), 3);
    assert_eq!(pool.available(), 0);
    assert!((pool.fill_ratio() - 1.0).abs() < f64::EPSILON);

    // Pool exhausted — should return None
    assert!(pool.alloc("delta".to_string(), 4000).is_none());

    // Verify handles work
    assert_eq!(pool.get(h1), Some(&"alpha".to_string()));
    assert_eq!(pool.get(h2), Some(&"beta".to_string()));
    assert_eq!(pool.get(h3), Some(&"gamma".to_string()));

    // Free h2, reuse the slot
    let freed = pool.free(h2, 5000);
    assert_eq!(freed, Some("beta".to_string()));
    assert_eq!(pool.allocated(), 2);
    assert_eq!(pool.available(), 1);

    // Allocate into freed slot
    let h4 = pool.alloc("delta".to_string(), 6000).unwrap();
    assert_eq!(pool.get(h4), Some(&"delta".to_string()));
    assert_eq!(pool.total_reuses(), 1);

    // CRITICAL: Stale handle h2 must NOT access the new data
    assert_eq!(pool.get(h2), None, "Stale handle must return None");
    assert_eq!(
        pool.free(h2, 7000),
        None,
        "Stale handle free must return None"
    );

    // Mutate through handle
    if let Some(val) = pool.get_mut(h4) {
        *val = "DELTA".to_string();
    }
    assert_eq!(pool.get(h4), Some(&"DELTA".to_string()));

    // Stats
    assert_eq!(pool.total_allocs(), 4); // 3 initial + 1 after free (ignore failed one)
    assert_eq!(pool.total_frees(), 1);
    assert_eq!(pool.peak_allocated(), 3);

    // Clear and verify
    pool.clear();
    assert_eq!(pool.allocated(), 0);
    assert_eq!(pool.available(), 3);
    // All handles invalid after clear
    assert_eq!(pool.get(h1), None);
    assert_eq!(pool.get(h3), None);
    assert_eq!(pool.get(h4), None);
}
