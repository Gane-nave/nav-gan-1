//! Adversarial tests for aurora-pool.

use aurora_pool::ObjectPool;

#[test]
fn adversarial_exhaust_capacity() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(3);
    let a = pool.checkout(|| 1);
    let b = pool.checkout(|| 2);
    let c = pool.checkout(|| 3);
    assert!(a.is_some());
    assert!(b.is_some());
    assert!(c.is_some());
    // Pool at capacity, all checked out
    let d = pool.checkout(|| 4);
    assert!(d.is_none(), "should not create beyond capacity");
    assert_eq!(pool.checked_out(), 3);
}

#[test]
fn adversarial_return_full_pool() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(2);
    pool.prefill(2, || 0);
    // Pool is full, return should fail
    assert!(!pool.return_obj(99));
    assert_eq!(pool.available(), 2);
}

#[test]
fn adversarial_rapid_checkout_return_cycle() {
    let mut pool: ObjectPool<u64> = ObjectPool::new(5);
    pool.prefill(5, || 0);
    for i in 0..10_000u64 {
        let obj = pool.checkout(|| i).expect("should always have available");
        pool.return_obj(obj);
    }
    assert_eq!(pool.total_checkouts(), 10_000);
    assert_eq!(pool.total_returns(), 10_000);
    assert_eq!(pool.available(), 5);
}

#[test]
fn adversarial_shrink_below_available() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(100);
    pool.prefill(50, || 42);
    pool.shrink_to(5);
    assert_eq!(pool.capacity(), 5);
    assert_eq!(pool.available(), 5);
}

#[test]
fn adversarial_drain_and_reuse() {
    let mut pool: ObjectPool<String> = ObjectPool::new(10);
    pool.prefill(5, || "old".to_string());
    let drained = pool.drain();
    assert_eq!(drained.len(), 5);
    assert!(pool.is_empty());
    // Can still checkout (creates new)
    let obj = pool.checkout(|| "new".to_string());
    assert_eq!(obj, Some("new".to_string()));
}

#[test]
fn adversarial_zero_capacity() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(0);
    let result = pool.checkout(|| 1);
    assert!(result.is_none(), "zero capacity should never produce objects");
}

#[test]
fn adversarial_prefill_twice() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(5);
    pool.prefill(3, || 1);
    pool.prefill(10, || 2); // tries 10 but only 2 more fit
    assert_eq!(pool.available(), 5);
    assert_eq!(pool.created(), 5);
}

#[test]
fn adversarial_shrink_to_zero() {
    let mut pool: ObjectPool<u32> = ObjectPool::new(10);
    pool.prefill(5, || 0);
    pool.shrink_to(0);
    assert_eq!(pool.capacity(), 0);
    assert_eq!(pool.available(), 0);
}
