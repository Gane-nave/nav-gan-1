//! Adversarial tests for aurora-lru

use aurora_lru::cache::LruCache;

#[test]
fn adversarial_eviction_ttl_hit_rate() {
    let mut c: LruCache<u32> = LruCache::new(3, Some(5000));

    // Fill cache: a at t=1000, b at t=2000, c at t=3000
    c.put("a", 1, 1000);
    c.put("b", 2, 2000);
    c.put("c", 3, 3000);
    assert_eq!(c.len(), 3);

    // Access "a" at t=3500 to promote it to MRU
    assert_eq!(c.get("a", 3500), Some(1));
    // Order is now: [b, c, a] (LRU to MRU)

    // Put "d" at t=4000 — should evict "b" (LRU), NOT "a"
    let evicted = c.put("d", 4, 4000);
    assert_eq!(
        evicted,
        Some("b".to_string()),
        "LRU eviction should remove 'b', not 'a'"
    );

    // "b" is gone, "a" is still present
    assert_eq!(c.get("b", 4500), None);
    assert_eq!(c.get("a", 4500), Some(1));

    // At t=6000, "a" has expired (created at 1000, TTL 5000 → expires at 6000)
    assert_eq!(
        c.get("a", 6000),
        None,
        "a should be expired at t=6000 (created 1000 + TTL 5000)"
    );

    // "c" also expired (created 3000 + 5000 = 8000, so at t=6000 not yet)
    assert_eq!(c.get("c", 6000), Some(3)); // still valid

    // At t=8000, "c" expired too
    assert_eq!(c.get("c", 8000), None);

    // "d" still valid (created 4000 + 5000 = 9000)
    assert_eq!(c.get("d", 8000), Some(4));

    // Verify stats
    assert!(c.hits() > 0);
    assert!(c.misses() > 0);
    assert!(c.evictions() > 0);
    assert!(c.hit_rate() > 0.0);
    assert!(c.hit_rate() < 1.0);
}
